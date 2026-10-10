// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exact kernel `Value`/`ValueType` and composite construction.
//!
//! Every kernel-crossing payload is trimmed to the bare id(s) it needs. The
//! declaration *registry* (duplicate-key checks, the recursion rule,
//! generalization, `Reference<T>` target lookup) is the caller's, not the
//! kernel's.
//!
//! - `ValueType::Enum(EnumShape)`: the shape carries its admitted variants
//!   inline, as a canonically ranked list of opaque `VariantId` digests, so
//!   `ValueType::admits` needs no declaration lookup at all -- membership and
//!   rank agreement are checked against the shape the type itself carries.
//!   `Value::Enum(EnumMember)` carries one variant's identity plus its
//!   zero-based canonical rank, so ordering (`crate::key::compare_keys`) needs no
//!   declaration lookup either.
//! - `ValueType::Reference(EffectiveId)`; the value payload
//!   `Value::Reference(ObjectReference)` carries the triple (`EffectiveId`,
//!   `UniverseId`, `ObjectId`) from [`crate::reference`].
//! - `ValueType::Quantity(UnitId)`; `Value::Quantity` carries
//!   [`crate::quantity::Quantity`], a bare `(magnitude, UnitId)` pair with no
//!   unit-graph declaration.
//! - `Value::Population(PopulationId)`: the kernel carries the opaque identity
//!   alone, never the population binding a caller's model owns.
//!   `ValueType::Population(Option<u64>)` holds the declared maximum.
//!   The declared-maximum comparison is a caller-layer check: the
//!   caller resolves a `PopulationId` to its binding and compares the binding's
//!   own declared maximum there, since this leaf crate has no way to resolve a
//!   `PopulationId` to anything. Kernel `ValueType::admits` refuses every
//!   population pair outright (FR-089-AC-6): every
//!   `(ValueType::Population(_), Value::Population(_))` pair falls through to
//!   `admits`'s catch-all and returns `false`.
//! - `record`/`tuple`/`evaluate_record`/`evaluate_tuple` are free functions taking
//!   the declared shape directly (`&[FieldDeclaration]` or `&[ValueType]`)
//!   rather than looking it up in a registry. The kernel trusts the shape its
//!   caller hands it.
//! - [`UnionMember::from_admitted`] retains the checking stage's verified
//!   (union declaration, FR-441 member key, identifier) binding. A union
//!   value shares that binding with its positional payload; `Composite`
//!   remains the type shape, and SV owns the member list. Identifier ASCII
//!   bytes, not a digest or enum rank, supply its FR-144 canonical key.
//! - [`from_admitted_slots`] is a trusted, unchecked composite constructor, `pub`
//!   for a caller that has independently checked a value against its own
//!   registry, mirroring [`OptionValue::from_admitted`]'s identical role.

use alloc::sync::Arc;
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::fmt;

use crate::accounting::{Charge, ChargePoint, LimitKind, Meter};
use crate::collection::{CollectionType, CollectionValue};
use crate::decimal::{Decimal, DecimalType};
use crate::identity::{EffectiveId, MemberId, PopulationId, UnitId, VariantId};
use crate::ieee::{FloatType, IeeeValue};
use crate::integer::{Integer, IntegerInterval};
use crate::node::{Identifier, NodeKey};
use crate::outcome::{CheckedInvariantCause, Outcome, Refusal, Stop};
use crate::quantity::Quantity;
use crate::rational::{Rational, RationalDomain};
use crate::reference::ObjectReference;
use crate::text::{Text, TextType};

mod native;
mod value_type;
pub use native::{Timestamp, Uuid};

/// The admitted identity of one union member, retaining its FR-144 ASCII
/// identifier key separately from its opaque FR-441 identity. This is a
/// member handle, not a member registry or a declaration-position rank.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnionMember {
    declaration: NodeKey,
    variant: VariantId,
    identifier: Identifier,
}

impl UnionMember {
    /// Retain a member binding already admitted by the checking/package
    /// identity stage. `member_key` must be the FR-441 key of exactly
    /// (`declaration`, `identifier`), in the evaluating package, and must
    /// name a union member, never an enum member. That stage owns the
    /// canonical preimage and digest verification; the kernel hashes nothing.
    /// Do not call this constructor on an unvalidated runtime label/key pair.
    pub fn from_admitted(
        declaration: NodeKey,
        member_key: NodeKey,
        identifier: Identifier,
    ) -> Self {
        Self {
            declaration,
            variant: VariantId::from_digest(*member_key.as_bytes()),
            identifier,
        }
    }

    /// The union declaration in the admitted package.
    pub fn declaration(&self) -> NodeKey {
        self.declaration
    }

    /// The FR-441 member key bytes, retyped without fresh computation.
    pub fn variant(&self) -> VariantId {
        self.variant
    }

    /// The admitted member identifier, whose ASCII bytes define key order.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

/// One completed union member with positional payload values. Cloning shares
/// immutable children, just as [`CompositeValue`] does; dropping uses the
/// same iterative worklist. Member lists and position types stay in SV.
#[derive(Clone)]
pub struct UnionValue {
    member: UnionMember,
    payload: Box<[Value]>,
    occ: Integer,
}

impl UnionValue {
    /// Materialize payload values already admitted against this member's
    /// declared positions by the caller's type environment. The caller owns
    /// membership, arity, reference validation and any construction charge.
    pub fn from_admitted(member: UnionMember, payload: Vec<Value>) -> Value {
        let occ = payload
            .iter()
            .fold(Integer::one(), |occ, value| occ.add(&value.occ()));
        Value::Union(Arc::new(Self {
            member,
            payload: payload.into_boxed_slice(),
            occ,
        }))
    }

    /// The union declaration key.
    pub fn declaration(&self) -> NodeKey {
        self.member.declaration()
    }

    /// The active member's identity and admitted identifier key.
    pub fn member(&self) -> &UnionMember {
        &self.member
    }

    /// The active member's opaque identity.
    pub fn variant(&self) -> VariantId {
        self.member.variant()
    }

    /// Payload values in declared position order.
    pub fn payload(&self) -> &[Value] {
        &self.payload
    }
}

impl Drop for UnionValue {
    fn drop(&mut self) {
        drop_nested(core::mem::take(&mut self.payload).into_vec());
    }
}

impl fmt::Debug for UnionValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        DebugWriter::new(formatter)
            .write(Step::Union(self))
            .map(|_peak| ())
    }
}

/// Construct an admitted member using its declared position types, looked
/// up by the caller. This checks arity and kernel position admission, not
/// registry membership or reference conformance.
pub fn union(
    shape: &[ValueType],
    member: UnionMember,
    payload: Vec<Value>,
) -> Result<Value, ConstructionRefusal> {
    check_union_arity(shape, payload.len())?;
    if let Some(position) = shape
        .iter()
        .zip(&payload)
        .position(|(value_type, value)| !value_type.admits(value))
    {
        return refuse(
            Component::Position(position),
            ConstructionCause::TypeMismatch,
        );
    }
    Ok(UnionValue::from_admitted(member, payload))
}

/// Evaluate a checked union member in position order, stopping at the first
/// unavailable argument, then charge `composite.result-retain`. Arity is
/// checked before evaluating any argument. The member and shape must have
/// been resolved together by the caller's type environment.
pub fn evaluate_union(
    shape: &[ValueType],
    member: UnionMember,
    payload: Vec<Deferred<'_>>,
    meter: &mut Meter,
) -> Result<Outcome<Value>, ConstructionRefusal> {
    check_union_arity(shape, payload.len())?;
    let mut values = Vec::with_capacity(shape.len());
    for (value_type, expression) in shape.iter().zip(payload) {
        match admitted(value_type, expression(meter)) {
            Ok(value) => values.push(value),
            Err(stop) => return Ok(Outcome::from_stop(Err(stop))),
        }
    }
    Ok(retain_composite(
        UnionValue::from_admitted(member, values),
        meter,
    ))
}

fn check_union_arity(shape: &[ValueType], supplied: usize) -> Result<(), ConstructionRefusal> {
    if shape.len() != supplied {
        return refuse(
            Component::Value,
            ConstructionCause::WrongArity {
                declared: shape.len(),
                supplied,
            },
        );
    }
    Ok(())
}

/// The inline, *ranked* set of an enum type's admitted variants. The kernel `ValueType::Enum` carries its variant set
/// directly, as opaque [`VariantId`] digests, so `admits` is a pure
/// set-membership test needing no declaration lookup. The variants are held as a canonically
/// ordered list, not a digest-ordered set: QSpec-FR-144's enumeration key row (QSpec-FR-144-AC-9) fixes
/// canonical order as declaration position for an `ordered enum` and
/// case-identifier byte order for an unordered one, never the `VariantId`
/// digest. `EnumShape` cannot compute that order itself -- a kernel leaf
/// holds no case-name strings -- so the caller
/// supplies `variants` already in that canonical order; each variant's
/// zero-based index in the list is its *rank* ([`Self::rank`]), which
/// [`EnumMember`] carries next to its `VariantId` (an enum value
/// carries its `VariantId` and its rank). Two shapes are the same shape
/// exactly when they admit the same variants in the same canonical order and
/// agree on [`Self::is_ordered`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnumShape {
    ordered: bool,
    variants: Vec<VariantId>,
}

impl EnumShape {
    /// The enum shape admitting exactly `variants`, supplied already in QSpec-FR-141
    /// canonical order (declaration order when `ordered`, case-identifier byte
    /// order otherwise): the caller's responsibility, since this leaf type has
    /// no case-name strings to sort by itself.
    pub fn new(ordered: bool, variants: impl IntoIterator<Item = VariantId>) -> Self {
        Self {
            ordered,
            variants: variants.into_iter().collect(),
        }
    }

    /// Whether the declaration this shape describes selects `ordered enum`
    /// semantics (QSpec-FR-141-AC-5): only an ordered enum admits an ordering
    /// operator.
    pub fn is_ordered(&self) -> bool {
        self.ordered
    }

    /// Whether `variant` is one of this shape's admitted variants.
    pub fn contains(&self, variant: VariantId) -> bool {
        self.rank(variant).is_some()
    }

    /// `variant`'s zero-based index in this shape's canonical member list, or
    /// `None` when `variant` is not one of this shape's admitted variants.
    pub fn rank(&self, variant: VariantId) -> Option<u32> {
        self.variants
            .iter()
            .position(|candidate| *candidate == variant)
            .map(|position| {
                u32::try_from(position)
                    .expect("an admitted enum has far fewer than u32::MAX variants")
            })
    }

    /// The admitted variants, in canonical (rank) order.
    pub fn variants(&self) -> impl Iterator<Item = VariantId> + '_ {
        self.variants.iter().copied()
    }
}

/// A completed enum value: its variant identity and its canonical rank next
/// to it (an enum value carries its `VariantId`
/// and its rank, the variant's zero-based index in the QSpec-FR-141 canonical
/// member list). Identity and equality use `variant` alone
/// (the kernel equality leaf match, "Identity and equality use the
/// `VariantId` only"); `rank` exists purely so the kernel's own canonical key
/// (`compare_keys`, QSpec-FR-144) can order same-enum values
/// without any declaration lookup. [`ValueType::admits`] refuses a value
/// whose claimed rank disagrees with the shape's own ranked list, so a
/// well-formed `EnumMember` always has `shape.rank(member.variant()) ==
/// Some(member.rank())` for the `EnumShape` it was admitted against.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnumMember {
    variant: VariantId,
    rank: u32,
}

impl EnumMember {
    /// Pair a variant identity with its claimed canonical rank. This
    /// constructor performs no validation of its own -- [`ValueType::admits`]
    /// is where a mismatched pair is caught.
    pub fn new(variant: VariantId, rank: u32) -> Self {
        Self { variant, rank }
    }

    /// The variant identity: the only component identity and equality use.
    pub fn variant(&self) -> VariantId {
        self.variant
    }

    /// This member's zero-based index in its enum's QSpec-FR-141 canonical member
    /// list.
    pub fn rank(&self) -> u32 {
        self.rank
    }
}

/// A declared kernel value type. Two types are the same type exactly when
/// they are equal, collection bounds included.
///
/// `Clone`, `PartialEq`, `Hash`, `Debug` and `Drop` are hand-written so that
/// no walk over a type uses the host stack in proportion to its nesting
/// depth: a type of any depth clones, compares, hashes, formats and drops on
/// a small fixed stack, such as a no_std target's. `Debug` prints what
/// `#[derive(Debug)]` printed, except that in alternate mode a leaf gets
/// plain `{:#?}`, so the caller's format flags reach compact-mode leaves
/// only.
///
/// A nested type is shared behind an `Arc`, so cloning a type, or declaring
/// a value's payload type from a type it already holds, costs one reference
/// count however deep the type runs: a value `N` levels deep holds `O(N)`
/// type nodes, not `O(N^2)`.
///
/// quire:canonical
#[derive(Eq)]
pub enum ValueType {
    /// `Boolean`.
    Boolean,
    /// The unbounded mathematical `Integer`.
    Integer,
    /// A bounded `Int[lo, hi]`.
    Int(IntegerInterval),
    /// A `Rational[n1, n2; d1, d2]` domain.
    Rational(RationalDomain),
    /// A `Decimal[lo, hi; smin, smax; mode]`.
    Decimal(DecimalType),
    /// A `Float32[mode]` or `Float64[mode]`.
    Float(FloatType),
    /// A quantity in exactly this unit.
    Quantity(UnitId),
    /// A `Text[min, max; profile]`.
    Text(TextType),
    /// A native opaque UUID.
    Uuid,
    /// A native signed POSIX-epoch nanosecond Timestamp.
    Timestamp,
    /// An `Enum` type admitting exactly this inline variant set.
    Enum(EnumShape),
    /// `Option<T>`: `none` or a present `T`.
    Option(Arc<ValueType>),
    /// The record, tuple or union declaration with this node key.
    Composite(NodeKey),
    /// A collection type `K<T>[min, max]`, or the unbounded `K<T>`.
    Collection(Arc<CollectionType>),
    /// `Reference<T>` to an object of the object type with this effective
    /// identity.
    Reference(EffectiveId),
    /// The `Population<T>[N]` parameter type's declared maximum `N`, or
    /// `None` for a population with no declared maximum, which is unbounded
    /// (QSpec FR-153-AC-9).
    Population(Option<u64>),
}

impl ValueType {
    /// `K<element>[bound]`.
    pub fn collection(collection_type: CollectionType) -> Self {
        Self::Collection(Arc::new(collection_type))
    }

    /// `Option<payload>`.
    pub fn option(payload: Self) -> Self {
        Self::Option(Arc::new(payload))
    }

    /// Whether `value` is a member of this declared type. Composite, option
    /// and collection values carry their declared type, which must be this
    /// type; their contents were admitted at construction. An `Enum` shape
    /// admits a `Value::Enum` exactly when the shape's own ranked list agrees
    /// with the member's claimed rank for its variant: a
    /// pure lookup against the shape, no declaration lookup needed.
    pub fn admits(&self, value: &Value) -> bool {
        match (self, value) {
            (Self::Boolean, Value::Boolean(_)) | (Self::Integer, Value::Integer(_)) => true,
            (Self::Int(interval), Value::Integer(integer)) => interval.contains(integer),
            (Self::Rational(domain), Value::Rational(rational)) => domain.contains(rational),
            (Self::Decimal(declared), Value::Decimal(decimal)) => declared.contains(decimal),
            (Self::Float(declared), Value::Float(float)) => float.width() == declared.width(),
            (Self::Quantity(unit), Value::Quantity(quantity)) => quantity.unit() == *unit,
            (Self::Text(declared), Value::Text(text)) => text.text_type() == declared,
            (Self::Uuid, Value::Uuid(_)) | (Self::Timestamp, Value::Timestamp(_)) => true,
            (Self::Enum(shape), Value::Enum(member)) => {
                shape.rank(member.variant()) == Some(member.rank())
            }
            (Self::Option(payload), Value::Option(option)) => option.payload_type() == &**payload,
            (Self::Composite(declaration), Value::Composite(composite)) => {
                composite.declaration() == *declaration
            }
            (Self::Composite(declaration), Value::Union(union)) => {
                union.declaration() == *declaration
            }
            (Self::Collection(declared), Value::Collection(collection)) => {
                collection.collection_type() == &**declared
            }
            (Self::Reference(object_type), Value::Reference(reference)) => {
                reference.object_type() == *object_type
            }
            // `ValueType::Population` does not pair with `Value::Population`
            // here (see this module's doc comment): the
            // declared-maximum comparison is a caller-layer check, performed by
            // the caller once it resolves the binding. Kernel
            // `admits` refuses population pairs outright; this pair falls
            // through to the catch-all below and returns `false`.
            (
                Self::Boolean
                | Self::Integer
                | Self::Int(_)
                | Self::Rational(_)
                | Self::Decimal(_)
                | Self::Float(_)
                | Self::Quantity(_)
                | Self::Text(_)
                | Self::Uuid
                | Self::Timestamp
                | Self::Enum(_)
                | Self::Option(_)
                | Self::Composite(_)
                | Self::Collection(_)
                | Self::Reference(_)
                | Self::Population(_),
                Value::Boolean(_)
                | Value::Integer(_)
                | Value::Rational(_)
                | Value::Decimal(_)
                | Value::Float(_)
                | Value::Quantity(_)
                | Value::Text(_)
                | Value::Uuid(_)
                | Value::Timestamp(_)
                | Value::Enum(_)
                | Value::Population(_)
                | Value::Option(_)
                | Value::Composite(_)
                | Value::Union(_)
                | Value::Collection(_)
                | Value::Reference(_),
            ) => false,
        }
    }
}

/// A completed kernel value. It deliberately has no structural `PartialEq`:
/// equality is the relation [`crate::plan_equality`]/[`crate::planned_equality`]
/// plans and evaluates.
///
/// No walk over a value uses the host stack in proportion to its nesting
/// depth, so a value of any depth can be dropped, formatted and compared on
/// a small fixed stack, such as a no_std target's, where an overflow is not
/// a clean panic:
///
/// - `Debug`, and the `Drop` of the four node structs a nested value lives
///   in, are hand-written over an explicit worklist. `Value` itself has no
///   `Drop`, so a caller can still move a payload out of it by pattern.
/// - `Clone` is shallow: a nested value is one `Arc`.
/// - Equality and the canonical key ([`crate::plan_equality`],
///   [`crate::compare_keys`]) are worklist walks.
/// - [`ValueType::admits`] reads only the outermost value, since a nested
///   value carries a type that was checked when it was built.
///
/// quire:canonical
#[derive(Clone)]
pub enum Value {
    /// A Boolean.
    Boolean(bool),
    /// A mathematical integer of `Integer` or `Int[..]`.
    Integer(Integer),
    /// An exact reduced rational.
    Rational(Rational),
    /// An exact decimal.
    Decimal(Decimal),
    /// An IEEE bit pattern.
    Float(IeeeValue),
    /// A quantity in its unit.
    Quantity(Quantity),
    /// A text value of its declared type.
    Text(Text),
    /// An opaque native UUID.
    Uuid(Uuid),
    /// A signed POSIX-epoch nanosecond Timestamp.
    Timestamp(Timestamp),
    /// A bare enum member identity and its canonical rank.
    Enum(EnumMember),
    /// An opaque population admission identity: never the population binding itself, which
    /// stays a caller's model type.
    Population(PopulationId),
    /// An option value.
    Option(Arc<OptionValue>),
    /// A record or tuple value.
    Composite(Arc<CompositeValue>),
    /// One admitted union member and its payload in declared position order.
    Union(Arc<UnionValue>),
    /// A collection value.
    Collection(Arc<CollectionValue>),
    /// A terminal object reference.
    Reference(ObjectReference),
}

impl Value {
    /// `occ(v)` of `quire.value.accounting/v1`: one for the value itself plus
    /// every nested occurrence, with `absent` and `null` slots counting zero
    /// and a bag occurrence counted once per multiplicity.
    pub fn occ(&self) -> Integer {
        match self {
            Self::Option(option) => option.occ.clone(),
            Self::Composite(composite) => composite.occ.clone(),
            Self::Union(union) => union.occ.clone(),
            Self::Collection(collection) => collection.occ().clone(),
            Self::Boolean(_)
            | Self::Integer(_)
            | Self::Rational(_)
            | Self::Decimal(_)
            | Self::Float(_)
            | Self::Quantity(_)
            | Self::Text(_)
            | Self::Uuid(_)
            | Self::Timestamp(_)
            | Self::Enum(_)
            | Self::Population(_)
            | Self::Reference(_) => Integer::one(),
        }
    }
}

/// Drop `children`, and everything nested in them, from a worklist instead
/// of the compiler's recursive drop glue: the `Drop` of each node struct
/// ([`OptionValue`], [`CompositeValue`], `CollectionValue`) hands its
/// children here. Before a node popped off the worklist drops, its own
/// children are moved onto the worklist, so its `Drop` finds nothing left
/// and the stack stays a fixed few frames deep at any value depth.
///
/// A node shared through another `Arc` handle is only decremented, never
/// taken apart, since the other handle still reaches its contents.
/// `Arc::into_inner` decides that atomically: exactly one dropping handle
/// receives the node.
pub(crate) fn drop_nested(children: impl IntoIterator<Item = Value>) {
    let mut pending = Vec::new();
    children
        .into_iter()
        .for_each(|child| defer_node(&mut pending, child));
    while let Some(node) = pending.pop() {
        release_children(node, &mut pending);
    }
}

/// Queue `value` if it is a node with children of its own; drop a scalar
/// at once, which recurses nowhere.
fn defer_node(pending: &mut Vec<Value>, value: Value) {
    match value {
        Value::Option(_) | Value::Composite(_) | Value::Union(_) | Value::Collection(_) => {
            pending.push(value)
        }
        Value::Boolean(_)
        | Value::Integer(_)
        | Value::Rational(_)
        | Value::Decimal(_)
        | Value::Float(_)
        | Value::Quantity(_)
        | Value::Text(_)
        | Value::Uuid(_)
        | Value::Timestamp(_)
        | Value::Enum(_)
        | Value::Population(_)
        | Value::Reference(_) => {}
    }
}

/// Drop `node`. When this was the last handle to its node, the node's
/// children move onto `pending` first, so the node itself drops empty.
fn release_children(node: Value, pending: &mut Vec<Value>) {
    match node {
        Value::Option(option) => {
            if let Some(mut option) = Arc::into_inner(option) {
                option
                    .payload
                    .take()
                    .into_iter()
                    .for_each(|payload| defer_node(pending, payload));
            }
        }
        Value::Composite(composite) => {
            if let Some(mut composite) = Arc::into_inner(composite) {
                take_present(&mut composite.slots).for_each(|slot| defer_node(pending, slot));
            }
        }
        Value::Collection(collection) => {
            if let Some(mut collection) = Arc::into_inner(collection) {
                collection
                    .take_elements()
                    .into_vec()
                    .into_iter()
                    .for_each(|element| defer_node(pending, element));
            }
        }
        Value::Union(union) => {
            if let Some(mut union) = Arc::into_inner(union) {
                core::mem::take(&mut union.payload)
                    .into_vec()
                    .into_iter()
                    .for_each(|payload| defer_node(pending, payload));
            }
        }
        Value::Boolean(_)
        | Value::Integer(_)
        | Value::Rational(_)
        | Value::Decimal(_)
        | Value::Float(_)
        | Value::Quantity(_)
        | Value::Text(_)
        | Value::Uuid(_)
        | Value::Timestamp(_)
        | Value::Enum(_)
        | Value::Population(_)
        | Value::Reference(_) => {}
    }
}

/// Move the present values out of `slots`, leaving none.
fn take_present(slots: &mut Box<[FieldValue]>) -> impl Iterator<Item = Value> {
    core::mem::take(slots)
        .into_vec()
        .into_iter()
        .filter_map(|slot| match slot {
            FieldValue::Present(value) => Some(value),
            FieldValue::Absent | FieldValue::Null => None,
        })
}

impl fmt::Debug for Value {
    /// Prints what `#[derive(Debug)]` printed, in compact and alternate
    /// mode, from a worklist instead of recursion. Leaves (scalars, types,
    /// counts) print through their own `Debug`. In alternate mode a leaf
    /// gets plain `{:#?}`, so the width, fill, precision and hex flags of
    /// the caller's format spec reach compact-mode leaves only.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        DebugWriter::new(formatter)
            .write(Step::Value(self))
            .map(|_peak| ())
    }
}

/// A bracket pair of the derived `Debug` layout: `Name(..)`, `Name { .. }`
/// or `[..]`.
#[derive(Clone, Copy)]
enum Bracket {
    Paren,
    Brace,
    Square,
}

/// The rest of a list still to print.
#[derive(Clone, Copy)]
enum Items<'a> {
    Slots(&'a [FieldValue]),
    Elements(&'a [Value]),
}

impl<'a> Items<'a> {
    fn is_empty(self) -> bool {
        match self {
            Self::Slots(slots) => slots.is_empty(),
            Self::Elements(elements) => elements.is_empty(),
        }
    }

    /// The first item's step and the items after it.
    fn split_first(self) -> Option<(Step<'a>, Self)> {
        match self {
            Self::Slots(slots) => slots
                .split_first()
                .map(|(first, rest)| (Step::Slot(first), Self::Slots(rest))),
            Self::Elements(elements) => elements
                .split_first()
                .map(|(first, rest)| (Step::Value(first), Self::Elements(rest))),
        }
    }
}

/// One pending piece of `Value`'s or `ValueType`'s `Debug` output. A list
/// is expanded one item at a time, so the worklist grows with nesting depth,
/// not width.
#[derive(Clone, Copy)]
enum Step<'a> {
    Value(&'a Value),
    Union(&'a UnionValue),
    Type(&'a ValueType),
    CollectionType(&'a CollectionType),
    Slot(&'a FieldValue),
    Payload(Option<&'a Value>),
    /// A whole list, brackets included.
    List(Items<'a>),
    /// The not-yet-printed items of an open list.
    Items(Items<'a>),
    Leaf(&'a dyn fmt::Debug),
    Text(&'static str),
    Open(Bracket),
    Next,
    Close(Bracket),
}

/// Writes `Value`'s or `ValueType`'s `Debug` output straight to the
/// formatter, tracking the open bracket depth that alternate mode indents
/// to.
struct DebugWriter<'f, 'g> {
    formatter: &'f mut fmt::Formatter<'g>,
    alternate: bool,
    depth: usize,
}

impl<'f, 'g> DebugWriter<'f, 'g> {
    fn new(formatter: &'f mut fmt::Formatter<'g>) -> Self {
        let alternate = formatter.alternate();
        Self {
            formatter,
            alternate,
            depth: 0,
        }
    }

    /// Write `root`, returning the most steps the worklist held at once.
    fn write(mut self, root: Step<'_>) -> Result<usize, fmt::Error> {
        let mut steps = vec![root];
        let mut peak = steps.len();
        while let Some(step) = steps.pop() {
            match step {
                Step::Value(value) => schedule_value(&mut steps, value),
                Step::Union(union) => schedule_union(&mut steps, union),
                Step::Type(value_type) => value_type::schedule_type(&mut steps, value_type),
                Step::CollectionType(collection_type) => {
                    value_type::schedule_collection_type(&mut steps, collection_type)
                }
                Step::Slot(FieldValue::Present(value)) => {
                    schedule(&mut steps, tuple_steps("Present", Step::Value(value)))
                }
                Step::Slot(FieldValue::Absent) => self.formatter.write_str("Absent")?,
                Step::Slot(FieldValue::Null) => self.formatter.write_str("Null")?,
                Step::Payload(None) => self.formatter.write_str("None")?,
                Step::Payload(Some(value)) => {
                    schedule(&mut steps, tuple_steps("Some", Step::Value(value)))
                }
                Step::List(items) if items.is_empty() => self.formatter.write_str("[]")?,
                Step::List(items) => schedule(
                    &mut steps,
                    [
                        Step::Open(Bracket::Square),
                        Step::Items(items),
                        Step::Close(Bracket::Square),
                    ],
                ),
                Step::Items(items) => {
                    if let Some((first, rest)) = items.split_first() {
                        if rest.is_empty() {
                            steps.push(first);
                        } else {
                            schedule(&mut steps, [first, Step::Next, Step::Items(rest)]);
                        }
                    }
                }
                Step::Leaf(leaf) => self.leaf(leaf)?,
                Step::Text(text) => self.formatter.write_str(text)?,
                Step::Open(bracket) => self.open(bracket)?,
                Step::Next => self.next()?,
                Step::Close(bracket) => self.close(bracket)?,
            }
            peak = peak.max(steps.len());
        }
        Ok(peak)
    }

    fn open(&mut self, bracket: Bracket) -> fmt::Result {
        self.formatter.write_str(match bracket {
            Bracket::Paren => "(",
            Bracket::Brace => " {",
            Bracket::Square => "[",
        })?;
        self.depth += 1;
        if self.alternate {
            self.newline()
        } else if matches!(bracket, Bracket::Brace) {
            self.formatter.write_str(" ")
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> fmt::Result {
        if self.alternate {
            self.formatter.write_str(",")?;
            self.newline()
        } else {
            self.formatter.write_str(", ")
        }
    }

    fn close(&mut self, bracket: Bracket) -> fmt::Result {
        self.depth = self.depth.saturating_sub(1);
        if self.alternate {
            self.formatter.write_str(",")?;
            self.newline()?;
        } else if matches!(bracket, Bracket::Brace) {
            self.formatter.write_str(" ")?;
        }
        self.formatter.write_str(match bracket {
            Bracket::Paren => ")",
            Bracket::Brace => "}",
            Bracket::Square => "]",
        })
    }

    /// A line break and the current depth's indent.
    fn newline(&mut self) -> fmt::Result {
        self.formatter.write_str("\n")?;
        indent(self.formatter, self.depth)
    }

    /// A leaf through its own `Debug`. Compact mode hands it the caller's
    /// formatter; alternate mode indents each line of its `{:#?}` output to
    /// the current depth.
    fn leaf(&mut self, leaf: &dyn fmt::Debug) -> fmt::Result {
        if self.alternate {
            let mut indented = Indented {
                out: &mut *self.formatter,
                depth: self.depth,
            };
            fmt::write(&mut indented, format_args!("{leaf:#?}"))
        } else {
            leaf.fmt(self.formatter)
        }
    }
}

/// Push `steps` so they pop in the order given.
fn schedule<'a, const N: usize>(stack: &mut Vec<Step<'a>>, steps: [Step<'a>; N]) {
    stack.extend(steps.into_iter().rev());
}

/// Push the steps that print one value, outermost name first.
fn schedule_value<'a>(stack: &mut Vec<Step<'a>>, value: &'a Value) {
    let (name, leaf): (&'static str, &'a dyn fmt::Debug) = match value {
        Value::Boolean(leaf) => ("Boolean", leaf),
        Value::Integer(leaf) => ("Integer", leaf),
        Value::Rational(leaf) => ("Rational", leaf),
        Value::Decimal(leaf) => ("Decimal", leaf),
        Value::Float(leaf) => ("Float", leaf),
        Value::Quantity(leaf) => ("Quantity", leaf),
        Value::Text(leaf) => ("Text", leaf),
        Value::Uuid(leaf) => ("Uuid", leaf),
        Value::Timestamp(leaf) => ("Timestamp", leaf),
        Value::Enum(leaf) => ("Enum", leaf),
        Value::Population(leaf) => ("Population", leaf),
        Value::Reference(leaf) => ("Reference", leaf),
        Value::Option(option) => {
            // Destructured whole, so a new field cannot be left out of the
            // output without a compile error, as the derive guaranteed.
            let OptionValue {
                payload_type,
                payload,
                occ,
            } = &**option;
            return schedule_node(
                stack,
                ["Option", "OptionValue", "payload_type: ", "payload: "],
                Step::Type(payload_type),
                Step::Payload(payload.as_ref()),
                occ,
            );
        }
        Value::Composite(composite) => {
            let CompositeValue {
                declaration,
                slots,
                occ,
            } = &**composite;
            return schedule_node(
                stack,
                ["Composite", "CompositeValue", "declaration: ", "slots: "],
                Step::Leaf(declaration),
                Step::List(Items::Slots(slots)),
                occ,
            );
        }
        Value::Collection(collection) => {
            let (collection_type, elements, occ) = collection.debug_fields();
            return schedule_node(
                stack,
                [
                    "Collection",
                    "CollectionValue",
                    "collection_type: ",
                    "elements: ",
                ],
                Step::CollectionType(collection_type),
                Step::List(Items::Elements(elements)),
                occ,
            );
        }
        Value::Union(union) => {
            return schedule(stack, tuple_steps("Union", Step::Union(union)));
        }
    };
    schedule(stack, tuple_steps(name, Step::Leaf(leaf)));
}

fn schedule_union<'a>(stack: &mut Vec<Step<'a>>, union: &'a UnionValue) {
    let UnionValue {
        member,
        payload,
        occ,
    } = union;
    schedule(
        stack,
        [
            Step::Text("UnionValue"),
            Step::Open(Bracket::Brace),
            Step::Text("member: "),
            Step::Leaf(member),
            Step::Next,
            Step::Text("payload: "),
            Step::List(Items::Elements(payload)),
            Step::Next,
            Step::Text("occ: "),
            Step::Leaf(occ),
            Step::Close(Bracket::Brace),
        ],
    );
}

/// The steps of a one-field tuple shape, `name(inner)`.
fn tuple_steps<'a>(name: &'static str, inner: Step<'a>) -> [Step<'a>; 4] {
    [
        Step::Text(name),
        Step::Open(Bracket::Paren),
        inner,
        Step::Close(Bracket::Paren),
    ]
}

/// Push `Variant(Node { first: .., second: .., occ: .. })`, the shape every
/// nesting variant shares. The four names are the variant, the node struct
/// and the labels of its first two fields.
fn schedule_node<'a>(
    stack: &mut Vec<Step<'a>>,
    [variant, node, first_label, second_label]: [&'static str; 4],
    first: Step<'a>,
    second: Step<'a>,
    occ: &'a Integer,
) {
    schedule(
        stack,
        [
            Step::Text(variant),
            Step::Open(Bracket::Paren),
            Step::Text(node),
            Step::Open(Bracket::Brace),
            Step::Text(first_label),
            first,
            Step::Next,
            Step::Text(second_label),
            second,
            Step::Next,
            Step::Text("occ: "),
            Step::Leaf(occ),
            Step::Close(Bracket::Brace),
            Step::Close(Bracket::Paren),
        ],
    );
}

/// Write `depth` levels of the derived `Debug` alternate-mode indent.
fn indent(out: &mut dyn fmt::Write, depth: usize) -> fmt::Result {
    (0..depth).try_for_each(|_| out.write_str("    "))
}

/// A `fmt::Write` adaptor that follows every line break written through it
/// with `depth` levels of indent, as the derive's own nested formatter
/// does, without buffering the text.
struct Indented<'w> {
    out: &'w mut dyn fmt::Write,
    depth: usize,
}

impl fmt::Write for Indented<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let mut lines = text.split('\n');
        if let Some(first) = lines.next() {
            self.out.write_str(first)?;
        }
        lines.try_for_each(|line| {
            self.out.write_str("\n")?;
            indent(self.out, self.depth)?;
            self.out.write_str(line)
        })
    }
}

/// `none` or a present value of one declared payload type.
#[derive(Clone, Debug)]
pub struct OptionValue {
    payload_type: ValueType,
    payload: Option<Value>,
    occ: Integer,
}

impl OptionValue {
    /// `none` of `Option<payload_type>`.
    pub fn none(payload_type: ValueType) -> Value {
        Value::Option(Arc::new(Self {
            payload_type,
            payload: None,
            occ: Integer::one(),
        }))
    }

    /// A present `payload` of `Option<payload_type>`.
    pub fn present(payload_type: ValueType, payload: Value) -> Result<Value, ConstructionRefusal> {
        if !payload_type.admits(&payload) {
            return Err(ConstructionRefusal {
                component: Component::Payload,
                cause: ConstructionCause::TypeMismatch,
            });
        }
        let occ = Integer::one().add(&payload.occ());
        Ok(Value::Option(Arc::new(Self {
            payload_type,
            payload: Some(payload),
            occ,
        })))
    }

    /// Materialize an already-admitted `payload` as `Option<payload_type>`,
    /// checking no structural `admits()` match. `pub`, not `pub(crate)`,
    /// since the caller checking admission (e.g. an upcast-aware
    /// lookup) is a separate crate from this one.
    pub fn from_admitted(payload_type: ValueType, payload: Option<Value>) -> Value {
        let occ = match &payload {
            Some(payload) => Integer::one().add(&payload.occ()),
            None => Integer::one(),
        };
        Value::Option(Arc::new(Self {
            payload_type,
            payload,
            occ,
        }))
    }

    /// The declared payload type.
    pub fn payload_type(&self) -> &ValueType {
        &self.payload_type
    }

    /// The present payload, or `None` for `none`.
    pub fn payload(&self) -> Option<&Value> {
        self.payload.as_ref()
    }
}

impl Drop for OptionValue {
    /// Frees the payload from a worklist, never by recursion (see
    /// `drop_nested`), so the stack depth stays fixed at any value depth.
    fn drop(&mut self) {
        drop_nested(self.payload.take());
    }
}

/// Whether a declared field admits only a present value (`f: T`) or also
/// `absent` and explicit `null` (`f: T?`).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Presence {
    /// A field declared without `?`.
    Required,
    /// A field declared with `?`.
    Optional,
}

/// The state of one field slot.
#[derive(Clone, Debug)]
pub enum FieldValue {
    /// A present value.
    Present(Value),
    /// `absent`: the `?` field was omitted.
    Absent,
    /// Explicit `null`, distinct from `absent`.
    Null,
}

/// A declaration-owned named field. The kernel carries a member only as an
/// opaque [`MemberId`] digest: `name` is retained solely as a
/// human-readable label for `Debug`/diagnostics, and no public semantic
/// dispatch here keys on it -- every
/// lookup, refusal component and slot match below keys on `member`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldDeclaration {
    member: MemberId,
    name: String,
    value_type: ValueType,
    presence: Presence,
}

impl FieldDeclaration {
    /// The field `member` (`name: value_type` or `name: value_type?`, where
    /// `name` is a display label only -- see the struct doc comment).
    pub fn new(
        member: MemberId,
        name: impl Into<String>,
        value_type: ValueType,
        presence: Presence,
    ) -> Self {
        Self {
            member,
            name: name.into(),
            value_type,
            presence,
        }
    }

    /// The opaque member identity dispatch keys on.
    pub fn member(&self) -> MemberId {
        self.member
    }

    /// The field's human-readable label. Not used for dispatch.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The declared value type.
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }

    /// Whether the field was declared with `?`.
    pub fn presence(&self) -> Presence {
        self.presence
    }
}

/// Construct a record from its declared `shape` (looked up by the caller's
/// own declaration registry -- not the kernel).
/// An omitted `?` field is `absent`.
pub fn record(
    shape: &[FieldDeclaration],
    declaration: NodeKey,
    fields: Vec<(MemberId, FieldValue)>,
) -> Result<Value, ConstructionRefusal> {
    let slots = fill_slots(shape, fields)?;
    Ok(composite(declaration, slots))
}

/// Construct a tuple of exactly `shape`'s declared arity.
pub fn tuple(
    shape: &[ValueType],
    declaration: NodeKey,
    positions: Vec<Value>,
) -> Result<Value, ConstructionRefusal> {
    if shape.len() != positions.len() {
        return refuse(
            Component::Value,
            ConstructionCause::WrongArity {
                declared: shape.len(),
                supplied: positions.len(),
            },
        );
    }
    if let Some(position) = shape
        .iter()
        .zip(&positions)
        .position(|(value_type, value)| !value_type.admits(value))
    {
        return refuse(
            Component::Position(position),
            ConstructionCause::TypeMismatch,
        );
    }
    let slots = positions.into_iter().map(FieldValue::Present).collect();
    Ok(composite(declaration, slots))
}

/// Evaluate a record value expression against its declared `shape`. Every
/// construction refusal is decided before any field expression runs. Field
/// expressions then run in declaration order, whatever the source order; the
/// first one that does not complete becomes the outcome and no later one
/// runs. A completed record charges `composite.result-retain`.
pub fn evaluate_record(
    shape: &[FieldDeclaration],
    declaration: NodeKey,
    fields: Vec<(MemberId, FieldExpression<'_>)>,
    meter: &mut Meter,
) -> Result<Outcome<Value>, ConstructionRefusal> {
    let mut supplied = match_members(shape, fields)?;
    let mut plan = Vec::with_capacity(shape.len());
    for field in shape {
        let expression = supplied.remove(&field.member);
        let component = || Component::Field(field.member);
        match (&expression, field.presence) {
            (None, Presence::Required) => {
                return refuse(component(), ConstructionCause::MissingField)
            }
            (Some(FieldExpression::Null), Presence::Required) => {
                return refuse(component(), ConstructionCause::NullForRequiredField)
            }
            (None | Some(FieldExpression::Null | FieldExpression::Evaluate(_)), _) => {}
        }
        plan.push((field, expression));
    }
    let mut slots = Vec::with_capacity(plan.len());
    for (field, expression) in plan {
        let slot = match expression {
            None => FieldValue::Absent,
            Some(FieldExpression::Null) => FieldValue::Null,
            Some(FieldExpression::Evaluate(expression)) => {
                match admitted(&field.value_type, expression(meter)) {
                    Ok(value) => FieldValue::Present(value),
                    Err(stop) => return Ok(Outcome::from_stop(Err(stop))),
                }
            }
        };
        slots.push(slot);
    }
    Ok(retain_composite(
        composite(declaration, slots.into_boxed_slice()),
        meter,
    ))
}

/// Evaluate a tuple call `T(e, ...)` against its declared `shape`: the arity
/// is checked first, then the arguments run in position order under the
/// first-stopped rule, then a completed tuple charges
/// `composite.result-retain`.
pub fn evaluate_tuple(
    shape: &[ValueType],
    declaration: NodeKey,
    positions: Vec<Deferred<'_>>,
    meter: &mut Meter,
) -> Result<Outcome<Value>, ConstructionRefusal> {
    if shape.len() != positions.len() {
        return refuse(
            Component::Value,
            ConstructionCause::WrongArity {
                declared: shape.len(),
                supplied: positions.len(),
            },
        );
    }
    let mut slots = Vec::with_capacity(shape.len());
    for (value_type, expression) in shape.iter().zip(positions) {
        match admitted(value_type, expression(meter)) {
            Ok(value) => slots.push(FieldValue::Present(value)),
            Err(stop) => return Ok(Outcome::from_stop(Err(stop))),
        }
    }
    Ok(retain_composite(
        composite(declaration, slots.into_boxed_slice()),
        meter,
    ))
}

/// A deferred expression: it runs only when construction reaches it.
pub type Deferred<'a> = Box<dyn FnOnce(&mut Meter) -> Outcome<Value> + 'a>;

/// A record field in a record value expression. Omitting a `?` field
/// constructs `absent`.
pub enum FieldExpression<'a> {
    /// `f: e`.
    Evaluate(Deferred<'a>),
    /// `f: null`.
    Null,
}

impl core::fmt::Debug for FieldExpression<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Evaluate(_) => formatter.write_str("Evaluate(..)"),
            Self::Null => formatter.write_str("Null"),
        }
    }
}

/// The completed value of a deferred expression, which a checked program
/// guarantees is a member of `value_type`.
fn admitted(value_type: &ValueType, outcome: Outcome<Value>) -> Result<Value, Stop> {
    let value = outcome.into_stop()?;
    if value_type.admits(&value) {
        Ok(value)
    } else {
        Err(Stop::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::DeferredResultNotAdmitted,
        }))
    }
}

/// Charge `composite.result-retain` with `occ(result)`, then expose it.
/// `pub`, not `pub(crate)`: a caller's name-keyed record/tuple evaluation
/// and expression evaluator build composites outside this crate and charge the same point
/// through this one function.
pub fn retain_composite(value: Value, meter: &mut Meter) -> Outcome<Value> {
    Outcome::from_stop(retain_composite_stop(value, meter))
}

fn retain_composite_stop(value: Value, meter: &mut Meter) -> Result<Value, Stop> {
    let occ = value.occ();
    meter.charge(
        Charge::new(ChargePoint::CompositeResultRetain)
            .exact_size(LimitKind::ValueOccurrences, occ.clone())
            .exact_results(occ),
    )?;
    Ok(value)
}

/// Where a construction refusal originates.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Component {
    /// The composite value as a whole (its declaration or arity).
    Value,
    /// A named record field or object attribute, by its opaque member
    /// identity. No public semantic dispatch depends on
    /// display text.
    Field(MemberId),
    /// A zero-based tuple position.
    Position(usize),
    /// A zero-based collection occurrence in source order.
    Element(usize),
    /// An option payload.
    Payload,
}

/// Why a construction is refused.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConstructionCause {
    /// A required field is omitted.
    MissingField,
    /// A supplied field is not declared.
    UndeclaredField,
    /// A field is supplied twice.
    DuplicateField,
    /// `null` is supplied for a required field.
    NullForRequiredField,
    /// A tuple call has another argument count than its declared arity.
    WrongArity {
        /// Declared arity.
        declared: usize,
        /// Supplied arguments.
        supplied: usize,
    },
    /// A value that is not a member of the declared type.
    TypeMismatch,
    /// UUID text is not lowercase ASCII `8-4-4-4-12` hexadecimal.
    UuidNoncanonical,
    /// Timestamp text is not canonical signed ASCII decimal.
    TimestampNoncanonical,
    /// Canonical Timestamp text is outside the signed `i128` range.
    TimestampOutOfDomain,
}

/// A typed construction refusal at its originating component.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[error("construction refused at {component:?}: {cause:?}")]
pub struct ConstructionRefusal {
    /// The originating component.
    pub component: Component,
    /// The typed cause.
    pub cause: ConstructionCause,
}

fn refuse<T>(component: Component, cause: ConstructionCause) -> Result<T, ConstructionRefusal> {
    Err(ConstructionRefusal { component, cause })
}

/// Index supplied entries by declared member, refusing an undeclared or
/// repeated member.
fn match_members<T>(
    declared: &[FieldDeclaration],
    supplied: Vec<(MemberId, T)>,
) -> Result<alloc::collections::BTreeMap<MemberId, T>, ConstructionRefusal> {
    let mut by_member = alloc::collections::BTreeMap::new();
    for (member, entry) in supplied {
        let component = || Component::Field(member);
        if !declared.iter().any(|field| field.member == member) {
            return refuse(component(), ConstructionCause::UndeclaredField);
        }
        if by_member.insert(member, entry).is_some() {
            return refuse(component(), ConstructionCause::DuplicateField);
        }
    }
    Ok(by_member)
}

/// Declaration-ordered slots of a record from supplied fields.
pub fn fill_slots(
    declared: &[FieldDeclaration],
    supplied: Vec<(MemberId, FieldValue)>,
) -> Result<Box<[FieldValue]>, ConstructionRefusal> {
    let mut by_member = match_members(declared, supplied)?;
    let mut slots = Vec::with_capacity(declared.len());
    for field in declared {
        let component = || Component::Field(field.member);
        let slot = by_member
            .remove(&field.member)
            .unwrap_or(FieldValue::Absent);
        match (&slot, field.presence) {
            (FieldValue::Absent, Presence::Required) => {
                return refuse(component(), ConstructionCause::MissingField)
            }
            (FieldValue::Null, Presence::Required) => {
                return refuse(component(), ConstructionCause::NullForRequiredField)
            }
            (FieldValue::Present(value), _) if !field.value_type.admits(value) => {
                return refuse(component(), ConstructionCause::TypeMismatch)
            }
            (FieldValue::Present(_) | FieldValue::Absent | FieldValue::Null, _) => {}
        }
        slots.push(slot);
    }
    Ok(slots.into_boxed_slice())
}

/// `1 + occ` of every present slot.
pub(crate) fn slots_occ(slots: &[FieldValue]) -> Integer {
    slots.iter().fold(Integer::one(), |occ, slot| match slot {
        FieldValue::Present(value) => occ.add(&value.occ()),
        FieldValue::Absent | FieldValue::Null => occ,
    })
}

fn composite(declaration: NodeKey, slots: Box<[FieldValue]>) -> Value {
    let occ = slots_occ(&slots);
    Value::Composite(Arc::new(CompositeValue {
        declaration,
        slots,
        occ,
    }))
}

/// Materialize already-admitted slots as a composite value, checking
/// nothing: the caller (whose declaration registry is not kernel) has already checked every slot against its own declared
/// shape. Mirrors [`OptionValue::from_admitted`]'s identical bypass role.
pub fn from_admitted_slots(declaration: NodeKey, slots: Box<[FieldValue]>) -> Value {
    composite(declaration, slots)
}

/// A record or tuple value.
#[derive(Clone, Debug)]
pub struct CompositeValue {
    declaration: NodeKey,
    slots: Box<[FieldValue]>,
    occ: Integer,
}

impl CompositeValue {
    /// The declaration node key.
    pub fn declaration(&self) -> NodeKey {
        self.declaration
    }

    /// Field or position states in declaration order.
    pub fn slots(&self) -> &[FieldValue] {
        &self.slots
    }
}

impl Drop for CompositeValue {
    /// Frees the present slots from a worklist, never by recursion (see
    /// `drop_nested`), so the stack depth stays fixed at any value depth.
    fn drop(&mut self) {
        drop_nested(take_present(&mut self.slots));
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::*;
    use crate::accounting::ScalarLimits;

    fn uuid(text: &str) -> Value {
        Value::Uuid(Uuid::from_canonical_text(text).expect("canonical UUID"))
    }

    fn timestamp(text: &str) -> Value {
        Value::Timestamp(Timestamp::from_canonical_text(text).expect("canonical Timestamp"))
    }

    /// Native types admit their own payload and reject existing and crossed kinds.
    #[trace("TC-918", "FR-370-AC-1", "FR-370-AC-5")]
    #[test]
    fn native_type_admission_has_no_alias() {
        let uuid = uuid("00112233-4455-6677-8899-aabbccddeeff");
        let timestamp = timestamp("1");
        assert!(ValueType::Uuid.admits(&uuid));
        assert!(ValueType::Timestamp.admits(&timestamp));
        assert!(!ValueType::Uuid.admits(&timestamp));
        assert!(!ValueType::Timestamp.admits(&uuid));
        assert!(!ValueType::Integer.admits(&timestamp));
        assert!(!ValueType::Uuid.admits(&Value::Integer(Integer::one())));
        assert!(!ValueType::Timestamp.admits(&Value::Integer(Integer::one())));
        let text_type =
            TextType::new(0, 10, crate::text::TextProfile::UnicodeScalars).expect("bounds");
        let text = crate::text::admit_text(
            &crate::text::TextPayload::from_utf8(b"hi").expect("UTF-8"),
            &text_type,
            &mut generous_meter(),
        )
        .completed()
        .expect("admitted text");
        let text = Value::Text(text);
        assert!(!ValueType::Uuid.admits(&text));
        assert!(!ValueType::Timestamp.admits(&text));
        assert!(ValueType::Text(text_type).admits(&text));
        let object_type = EffectiveId::from_digest(digest(6));
        let reference = Value::Reference(ObjectReference::new(
            crate::identity::UniverseId::from_digest(digest(5)),
            object_type,
            crate::identity::ObjectId::new("o-1").expect("non-empty"),
        ));
        assert!(!ValueType::Uuid.admits(&reference));
        assert!(!ValueType::Timestamp.admits(&reference));
        assert!(ValueType::Reference(object_type).admits(&reference));
    }

    /// Every existing value kind remains distinct from both new native kinds.
    #[trace("TC-918", "FR-370-AC-3", "FR-370-AC-5")]
    #[test]
    fn native_mixed_kind_plans_refuse_in_both_orders() {
        use crate::outcome::CheckedInvariantCause;

        let key = NodeKey::from_digest(digest(9));
        let half = Rational::new(Integer::one(), Integer::from(2_i64)).expect("nonzero");
        let text_type =
            TextType::new(0, 10, crate::text::TextProfile::UnicodeScalars).expect("bounds");
        let text = crate::text::admit_text(
            &crate::text::TextPayload::from_utf8(b"hi").expect("UTF-8"),
            &text_type,
            &mut generous_meter(),
        )
        .completed()
        .expect("admitted text");
        let others = vec![
            Value::Boolean(true),
            Value::Integer(Integer::one()),
            Value::Rational(half.clone()),
            Value::Decimal(Decimal::new(Integer::one(), 0)),
            Value::Float(IeeeValue::binary64(0x3ff0_0000_0000_0000)),
            Value::Quantity(Quantity::new(half, UnitId::declared(key))),
            Value::Text(text),
            Value::Enum(EnumMember::new(VariantId::from_digest(digest(3)), 0)),
            Value::Population(PopulationId::from_digest(digest(4))),
            OptionValue::none(ValueType::option(ValueType::Boolean)),
            from_admitted_slots(key, vec![FieldValue::Absent].into_boxed_slice()),
            crate::collection::from_admitted(
                CollectionType::new(
                    crate::collection::CollectionKind::Set,
                    ValueType::Boolean,
                    None,
                ),
                vec![],
            ),
            Value::Reference(ObjectReference::new(
                crate::identity::UniverseId::from_digest(digest(5)),
                EffectiveId::from_digest(digest(6)),
                crate::identity::ObjectId::new("o-1").expect("non-empty"),
            )),
        ];
        let native = [uuid("00112233-4455-6677-8899-aabbccddeeff"), timestamp("1")];
        for left in &native {
            for right in native.iter().chain(others.iter()) {
                if core::mem::discriminant(left) == core::mem::discriminant(right) {
                    continue;
                }
                for (first, second) in [(left, right), (right, left)] {
                    assert_eq!(
                        crate::equality::plan_equality(first, second),
                        Err(Refusal::CheckedInvariant {
                            cause: CheckedInvariantCause::ValueKindMismatch,
                        })
                    );
                    assert_eq!(crate::key::compare_keys(first, second), None);
                }
            }
        }
    }

    /// Both native leaves use the same one-pair plan and charges as Boolean.
    #[trace("TC-918", "FR-370-AC-2", "FR-370-AC-5")]
    #[test]
    fn native_equality_uses_boolean_leaf_schedule() {
        let pairs = [
            (Value::Boolean(true), Value::Boolean(false)),
            (
                uuid("00000000-0000-0000-0000-000000000000"),
                uuid("ffffffff-ffff-ffff-ffff-ffffffffffff"),
            ),
            (timestamp("-1"), timestamp("1")),
        ];
        let mut baseline = None;
        for (left, right) in pairs {
            let plan = crate::equality::plan_equality(&left, &right).expect("same kind");
            assert_eq!(plan.pair_events(), &Integer::one());
            let mut meter = generous_meter();
            assert!(
                !crate::equality::planned_equality(&left, &right, &mut meter)
                    .completed()
                    .expect("charged comparison")
            );
            let charges = equality_counters(&meter);
            if let Some(expected) = baseline {
                assert_eq!(charges, expected);
            } else {
                baseline = Some(charges);
            }
        }
        let mut boolean_meter = generous_meter();
        assert!(crate::equality::planned_equality(
            &Value::Boolean(true),
            &Value::Boolean(true),
            &mut boolean_meter,
        )
        .completed()
        .expect("charged Boolean control"));
        let equal_boolean_charges = equality_counters(&boolean_meter);
        assert_eq!(equal_boolean_charges, (4, 1, 5, 1));
        for value in [uuid("00000000-0000-0000-0000-000000000000"), timestamp("0")] {
            let plan = crate::equality::plan_equality(&value, &value).expect("same kind");
            assert_eq!(plan.pair_events(), &Integer::one());
            let mut meter = generous_meter();
            assert!(
                crate::equality::planned_equality(&value, &value, &mut meter)
                    .completed()
                    .expect("charged comparison")
            );
            assert_eq!(equality_counters(&meter), equal_boolean_charges);
        }
    }

    /// Native keys order canonical ASCII payloads and set visiting order is stable.
    #[trace("TC-918", "FR-370-AC-4", "FR-370-AC-5")]
    #[test]
    fn native_keys_order_canonical_content_and_sets() {
        use core::cmp::Ordering;
        let pairs = [
            (timestamp("10"), timestamp("2")),
            (timestamp("-1"), timestamp("-2")),
            (
                uuid("00000000-0000-0000-0000-000000000000"),
                uuid("ffffffff-ffff-ffff-ffff-ffffffffffff"),
            ),
        ];
        for (left, right) in pairs {
            assert_eq!(
                crate::key::compare_keys(&left, &right),
                Some(Ordering::Less)
            );
            assert_eq!(
                crate::key::compare_keys(&right, &left),
                Some(Ordering::Greater)
            );
            assert_eq!(
                crate::key::compare_keys(&left, &left),
                Some(Ordering::Equal)
            );
        }
        let set_type = CollectionType::new(
            crate::collection::CollectionKind::Set,
            ValueType::Timestamp,
            None,
        );
        let first = crate::collection::form(
            &set_type,
            vec![timestamp("2"), timestamp("10")],
            &mut generous_meter(),
        )
        .completed()
        .expect("formed set");
        let reversed = crate::collection::form(
            &set_type,
            vec![timestamp("10"), timestamp("2")],
            &mut generous_meter(),
        )
        .completed()
        .expect("formed set");
        assert_eq!(
            crate::key::compare_keys(&first, &reversed),
            Some(Ordering::Equal)
        );
        let Value::Collection(set) = first else {
            panic!("set value")
        };
        let actual: Vec<_> = set
            .elements()
            .iter()
            .map(|value| match value {
                Value::Timestamp(value) => value.nanoseconds(),
                _ => panic!("Timestamp member"),
            })
            .collect();
        assert_eq!(actual, [10, 2]);
    }

    /// Existing admitted leaves and nested forms retain their equality and keys.
    #[trace("TC-918", "FR-370-AC-5")]
    #[test]
    fn existing_kind_controls_remain_admitted_and_keyed() {
        let text_type =
            TextType::new(0, 10, crate::text::TextProfile::UnicodeScalars).expect("bounds");
        let text = crate::text::admit_text(
            &crate::text::TextPayload::from_utf8(b"hi").expect("UTF-8"),
            &text_type,
            &mut generous_meter(),
        )
        .completed()
        .expect("admitted text");
        let object_type = EffectiveId::from_digest(digest(6));
        let reference = ObjectReference::new(
            crate::identity::UniverseId::from_digest(digest(5)),
            object_type,
            crate::identity::ObjectId::new("o-1").expect("non-empty"),
        );
        let option_type = ValueType::option(ValueType::Boolean);
        let set_type = CollectionType::new(
            crate::collection::CollectionKind::Set,
            ValueType::Boolean,
            None,
        );
        let controls = [
            (
                ValueType::Boolean,
                Value::Boolean(true),
                Integer::one(),
                (4, 1, 5, 1),
            ),
            (
                ValueType::Integer,
                Value::Integer(Integer::one()),
                Integer::one(),
                (4, 1, 5, 1),
            ),
            (
                ValueType::Text(text_type),
                Value::Text(text),
                Integer::one(),
                (4, 1, 5, 1),
            ),
            (
                ValueType::Reference(object_type),
                Value::Reference(reference),
                Integer::one(),
                (4, 1, 5, 1),
            ),
            (
                option_type.clone(),
                OptionValue::none(ValueType::Boolean),
                Integer::one(),
                (4, 1, 5, 1),
            ),
            (
                ValueType::collection(set_type.clone()),
                crate::collection::from_admitted(set_type, vec![Value::Boolean(true)]),
                Integer::from(2_i64),
                (5, 2, 8, 1),
            ),
        ];
        for (value_type, value, expected_occ, expected_charges) in controls {
            assert!(value_type.admits(&value));
            assert_eq!(value.occ(), expected_occ);
            assert_eq!(
                crate::key::compare_keys(&value, &value),
                Some(core::cmp::Ordering::Equal)
            );
            let mut meter = generous_meter();
            assert!(
                crate::equality::planned_equality(&value, &value, &mut meter)
                    .completed()
                    .expect("charged comparison")
            );
            assert_eq!(equality_counters(&meter), expected_charges);
        }
    }

    fn digest(byte: u8) -> [u8; 32] {
        let mut bytes = [0_u8; 32];
        bytes[31] = byte;
        bytes
    }

    fn generous_meter() -> Meter {
        Meter::new(ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        })
    }

    /// Admission count, maximum value occurrence size, work, and retained result units.
    fn equality_counters(meter: &Meter) -> (u64, u64, u64, u64) {
        (
            meter.admission_count(),
            meter.consumed(LimitKind::ValueOccurrences),
            meter.consumed(LimitKind::WorkUnits),
            meter.consumed(LimitKind::ResultUnits),
        )
    }

    /// Trace: FR-369-AC-2
    #[test]
    fn deferred_record_field_outside_its_type_reports_exact_cause() {
        let member = MemberId::from_digest(digest(1));
        let shape = [FieldDeclaration::new(
            member,
            "count",
            ValueType::Integer,
            Presence::Required,
        )];
        let fields = vec![(
            member,
            FieldExpression::Evaluate(Box::new(|_| Outcome::Completed(Value::Boolean(true)))),
        )];
        let outcome = evaluate_record(
            &shape,
            NodeKey::from_digest(digest(2)),
            fields,
            &mut generous_meter(),
        )
        .expect("field shape is valid before deferred evaluation");
        assert!(matches!(
            outcome,
            Outcome::Refused(Refusal::CheckedInvariant {
                cause: CheckedInvariantCause::DeferredResultNotAdmitted,
            })
        ));
    }

    /// Trace: FR-369-AC-2
    #[test]
    fn deferred_tuple_position_outside_its_type_reports_exact_cause() {
        let positions: Vec<Deferred<'_>> =
            vec![Box::new(|_| Outcome::Completed(Value::Boolean(true)))];
        let outcome = evaluate_tuple(
            &[ValueType::Integer],
            NodeKey::from_digest(digest(2)),
            positions,
            &mut generous_meter(),
        )
        .expect("arity is valid before deferred evaluation");
        assert!(matches!(
            outcome,
            Outcome::Refused(Refusal::CheckedInvariant {
                cause: CheckedInvariantCause::DeferredResultNotAdmitted,
            })
        ));
    }

    /// `ValueType::Boolean` admits only `Value::Boolean`, refusing
    /// a value of another kind.
    #[test]
    fn admits_checks_the_matching_variant_only() {
        assert!(ValueType::Boolean.admits(&Value::Boolean(true)));
        assert!(!ValueType::Boolean.admits(&Value::Integer(Integer::one())));
    }

    /// TC-297 (FR-089-AC-6): kernel `admits` refuses every population pair;
    /// the declared-maximum comparison is the caller layer's.
    #[trace("TC-297", "FR-089-AC-6")]
    #[test]
    fn admits_refuses_a_population_pair() {
        assert!(!ValueType::Population(Some(5))
            .admits(&Value::Population(PopulationId::from_digest(digest(1)))));
    }

    /// an `Enum` shape admits a `Value::Enum` of a variant it
    /// contains at the variant's own rank, and refuses one it does not
    /// contain.
    ///
    /// Also FR-088-AC-11: admission checks the whole `(VariantId, rank)` pair,
    /// not membership alone.
    #[trace("TC-409", "FR-088-AC-11")]
    #[test]
    fn enum_shape_admits_only_its_own_variants() {
        let in_shape = VariantId::from_digest(digest(1));
        let out_of_shape = VariantId::from_digest(digest(2));
        let shape = ValueType::Enum(EnumShape::new(false, [in_shape]));
        assert!(shape.admits(&Value::Enum(EnumMember::new(in_shape, 0))));
        assert!(!shape.admits(&Value::Enum(EnumMember::new(out_of_shape, 0))));
        assert!(!ValueType::Boolean.admits(&Value::Enum(EnumMember::new(in_shape, 0))));
    }

    /// A well-formed variant paired with the *wrong*
    /// rank is refused just as surely as an unknown variant -- admission
    /// checks the whole `(VariantId, rank)` pair against the shape's own
    /// ranked list, not membership alone.
    ///
    /// Also TC-409 step 3 (FR-088-AC-11): a value that pairs a known
    /// `VariantId` with the wrong rank is refused at admission.
    #[trace("TC-409", "FR-088-AC-11")]
    #[test]
    fn admits_refuses_a_known_variant_at_the_wrong_rank() {
        let first = VariantId::from_digest(digest(1));
        let second = VariantId::from_digest(digest(2));
        let shape = ValueType::Enum(EnumShape::new(true, [first, second]));
        assert!(shape.admits(&Value::Enum(EnumMember::new(first, 0))));
        assert!(shape.admits(&Value::Enum(EnumMember::new(second, 1))));
        assert!(!shape.admits(&Value::Enum(EnumMember::new(first, 1))));
        assert!(!shape.admits(&Value::Enum(EnumMember::new(second, 0))));
    }

    /// The shape's own rank lookup is exactly the variant's index in
    /// the canonical list the caller supplied, and an unranked (unknown)
    /// variant resolves to `None`, never a panic or a fabricated rank.
    ///
    /// Also TC-409 steps 1 and 2 (FR-088-AC-11): rank is the canonical-list
    /// position.
    #[trace("TC-409", "FR-088-AC-11")]
    #[test]
    fn enum_shape_rank_matches_canonical_position() {
        let a = VariantId::from_digest(digest(1));
        let b = VariantId::from_digest(digest(2));
        let c = VariantId::from_digest(digest(3));
        let shape = EnumShape::new(true, [a, b]);
        assert_eq!(shape.rank(a), Some(0));
        assert_eq!(shape.rank(b), Some(1));
        assert_eq!(shape.rank(c), None);
        assert!(shape.is_ordered());
        assert_eq!(shape.variants().collect::<Vec<_>>(), [a, b]);
    }

    /// Form the kernel set of `members` over `shape` and return the visited
    /// variants' ranks, in visiting order.
    fn set_visiting_ranks(shape: &EnumShape, members: &[VariantId]) -> Vec<u32> {
        use crate::accounting::ScalarLimits;
        use crate::collection::{form_collection, CollectionKind, CollectionType};

        let mut meter = Meter::new(ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        });
        let collection_type =
            CollectionType::new(CollectionKind::Set, ValueType::Enum(shape.clone()), None);
        let occurrences = members
            .iter()
            .map(|variant| {
                let rank = shape.rank(*variant).expect("a variant of the shape");
                Value::Enum(EnumMember::new(*variant, rank))
            })
            .collect();
        let value = form_collection(&collection_type, occurrences, &mut meter)
            .expect("every member is admitted")
            .completed()
            .expect("the set forms");
        let Value::Collection(set) = value else {
            panic!("a collection value");
        };
        set.elements()
            .iter()
            .map(|element| match element {
                Value::Enum(member) => member.rank(),
                other => panic!("an enum member, found {other:?}"),
            })
            .collect()
    }

    /// A set of an ordered enum's values visits them in declaration order,
    /// and a set of an unordered enum's values in its canonical (case
    /// identifier) order, whatever the digests of the variants and whatever
    /// order they were formed in. Two shapes whose variants differ in
    /// identity but not in rank and order give the same ranks and the same
    /// visiting order.
    #[trace("TC-409", "FR-088-AC-11")]
    #[test]
    fn a_set_visits_enum_values_in_canonical_rank_order() {
        // Declared order `b`, `a`, `c`; the digests order them `a`, `c`, `b`.
        let (a, b, c) = (
            VariantId::from_digest(digest(1)),
            VariantId::from_digest(digest(3)),
            VariantId::from_digest(digest(2)),
        );
        let ordered = EnumShape::new(true, [b, a, c]);
        assert_eq!(
            [ordered.rank(b), ordered.rank(a), ordered.rank(c)],
            [Some(0), Some(1), Some(2)]
        );
        assert_eq!(set_visiting_ranks(&ordered, &[c, a, b]), [0, 1, 2]);
        let visited: Vec<VariantId> = {
            let ranks = set_visiting_ranks(&ordered, &[c, a, b]);
            ranks
                .iter()
                .map(|rank| ordered.variants().nth(*rank as usize).unwrap())
                .collect()
        };
        assert_eq!(visited, [b, a, c]);

        // Unordered: canonical list `a`, `b`, `c`.
        let unordered = EnumShape::new(false, [a, b, c]);
        assert_eq!(set_visiting_ranks(&unordered, &[c, b, a]), [0, 1, 2]);
        let visited: Vec<VariantId> = set_visiting_ranks(&unordered, &[c, b, a])
            .iter()
            .map(|rank| unordered.variants().nth(*rank as usize).unwrap())
            .collect();
        assert_eq!(visited, [a, b, c]);

        // A second shape with other variant identities at the same positions.
        let (x, y, z) = (
            VariantId::from_digest(digest(9)),
            VariantId::from_digest(digest(8)),
            VariantId::from_digest(digest(7)),
        );
        let renamed = EnumShape::new(true, [x, y, z]);
        assert_eq!(
            [renamed.rank(x), renamed.rank(y), renamed.rank(z)],
            [Some(0), Some(1), Some(2)]
        );
        assert_eq!(set_visiting_ranks(&renamed, &[z, x, y]), [0, 1, 2]);
    }

    /// building a record with a missing required field is refused
    /// at that field, before any other field is inspected.
    #[trace("QSpec-TC-188", "QSpec-FR-143-AC-4")]
    #[test]
    fn record_refuses_a_missing_required_field() {
        let member = MemberId::from_digest(digest(2));
        let shape = vec![FieldDeclaration::new(
            member,
            "count",
            ValueType::Integer,
            Presence::Required,
        )];
        let err = record(&shape, NodeKey::from_digest(digest(1)), vec![]).unwrap_err();
        assert_eq!(err.component, Component::Field(member));
        assert_eq!(err.cause, ConstructionCause::MissingField);
    }

    /// a tuple built with the declared position count and types
    /// completes, and `occ` counts the tuple itself plus its one integer
    /// position.
    #[test]
    fn tuple_of_declared_arity_and_types_completes() {
        let shape = vec![ValueType::Integer];
        let value = tuple(
            &shape,
            NodeKey::from_digest(digest(1)),
            vec![Value::Integer(Integer::one())],
        )
        .unwrap();
        assert_eq!(value.occ(), Integer::one().add(&Integer::one()));
    }

    /// a tuple call with the wrong argument count is refused with
    /// the declared and supplied counts named.
    #[trace("QSpec-TC-188", "QSpec-FR-143-AC-4")]
    #[test]
    fn tuple_wrong_arity_names_both_counts() {
        let shape = vec![ValueType::Integer, ValueType::Boolean];
        let err = tuple(
            &shape,
            NodeKey::from_digest(digest(1)),
            vec![Value::Integer(Integer::one())],
        )
        .unwrap_err();
        assert_eq!(
            err.cause,
            ConstructionCause::WrongArity {
                declared: 2,
                supplied: 1,
            }
        );
    }

    /// `fill_slots` fills a present field and, for an
    /// omitted optional field, `Absent`, both in declaration order
    /// regardless of supplied order (`fill_slots` was previously only
    /// exercised indirectly, through `record`'s missing-required-field
    /// refusal path).
    #[test]
    fn fill_slots_fills_present_and_absent_in_declaration_order() {
        let count = MemberId::from_digest(digest(1));
        let label = MemberId::from_digest(digest(2));
        let shape = vec![
            FieldDeclaration::new(count, "count", ValueType::Integer, Presence::Required),
            FieldDeclaration::new(
                label,
                "label",
                ValueType::Text(
                    TextType::new(0, 10, crate::text::TextProfile::UnicodeScalars).unwrap(),
                ),
                Presence::Optional,
            ),
        ];
        let slots = fill_slots(
            &shape,
            vec![(count, FieldValue::Present(Value::Integer(Integer::one())))],
        )
        .unwrap();
        assert!(matches!(slots[0], FieldValue::Present(Value::Integer(_))));
        assert!(matches!(slots[1], FieldValue::Absent));
    }

    /// `evaluate_record` runs a deferred field expression
    /// and completes with `composite.result-retain` charged (`evaluate_record`
    /// had no test before this; `record`/`tuple`'s tests exercise only the
    /// non-deferred constructors).
    #[test]
    fn evaluate_record_completes_from_a_deferred_field() {
        let count = MemberId::from_digest(digest(1));
        let shape = vec![FieldDeclaration::new(
            count,
            "count",
            ValueType::Integer,
            Presence::Required,
        )];
        let mut meter = generous_meter();
        let fields: Vec<(MemberId, FieldExpression<'_>)> = vec![(
            count,
            FieldExpression::Evaluate(Box::new(|_meter| {
                Outcome::Completed(Value::Integer(Integer::one()))
            })),
        )];
        let outcome = evaluate_record(&shape, NodeKey::from_digest(digest(2)), fields, &mut meter)
            .expect("declared fields match");
        let value = outcome.completed().expect("charges available");
        assert_eq!(value.occ(), Integer::one().add(&Integer::one()));
    }

    /// `evaluate_tuple` runs deferred positional
    /// expressions in position order and completes (`evaluate_tuple` had no
    /// test before this).
    #[test]
    fn evaluate_tuple_completes_from_deferred_positions() {
        let shape = vec![ValueType::Integer];
        let mut meter = generous_meter();
        let positions: Vec<Deferred<'_>> = vec![Box::new(|_meter| {
            Outcome::Completed(Value::Integer(Integer::one()))
        })];
        let outcome = evaluate_tuple(
            &shape,
            NodeKey::from_digest(digest(1)),
            positions,
            &mut meter,
        )
        .expect("declared arity matches");
        let value = outcome.completed().expect("charges available");
        assert_eq!(value.occ(), Integer::one().add(&Integer::one()));
    }

    /// The `Debug` parity fixtures. The first is a composite root holding
    /// every slot state, every scalar variant, and every nesting variant,
    /// including a node inside a collection element inside an option
    /// payload. The second has an option root.
    fn debug_samples() -> [Value; 2] {
        let key = NodeKey::from_digest(digest(9));
        let boolean_sequence = CollectionType::new(
            crate::collection::CollectionKind::Sequence,
            ValueType::Boolean,
            None,
        );
        let collection = crate::collection::from_admitted(
            boolean_sequence.clone(),
            vec![Value::Boolean(true), Value::Boolean(false)],
        );
        let empty = crate::collection::from_admitted(boolean_sequence, vec![]);
        let present =
            OptionValue::from_admitted(ValueType::Integer, Some(Value::Integer(Integer::one())));
        let none = OptionValue::none(ValueType::option(ValueType::Boolean));
        let half = Rational::new(Integer::one(), Integer::from(2_i64)).expect("non-zero");
        let text_type =
            TextType::new(0, 10, crate::text::TextProfile::UnicodeScalars).expect("bounds");
        let text = crate::text::admit_text(
            &crate::text::TextPayload::from_utf8(b"hi").expect("utf-8"),
            &text_type,
            &mut generous_meter(),
        )
        .completed()
        .expect("admitted");
        let reference = ObjectReference::new(
            crate::identity::UniverseId::from_digest(digest(5)),
            EffectiveId::from_digest(digest(6)),
            crate::identity::ObjectId::new("o-1").expect("non-empty"),
        );
        let scalars = [
            Value::Rational(half.clone()),
            Value::Decimal(Decimal::new(Integer::from(125_i64), 2)),
            Value::Float(IeeeValue::binary64(0x3ff8_0000_0000_0000)),
            Value::Quantity(Quantity::new(half, UnitId::declared(key))),
            Value::Text(text),
            Value::Reference(reference),
            Value::Enum(EnumMember::new(VariantId::from_digest(digest(3)), 0)),
            Value::Population(PopulationId::from_digest(digest(4))),
        ];
        let node_sequence = CollectionType::new(
            crate::collection::CollectionKind::Sequence,
            ValueType::Composite(key),
            None,
        );
        let leaf_node = from_admitted_slots(
            key,
            vec![FieldValue::Present(Value::Boolean(true))].into_boxed_slice(),
        );
        let nodes = crate::collection::from_admitted(node_sequence.clone(), vec![leaf_node]);
        let nested = OptionValue::from_admitted(ValueType::collection(node_sequence), Some(nodes));
        let mut slots = vec![
            FieldValue::Present(present),
            FieldValue::Absent,
            FieldValue::Null,
            FieldValue::Present(collection),
            FieldValue::Present(empty),
            FieldValue::Present(none),
            FieldValue::Present(nested.clone()),
        ];
        slots.extend(scalars.into_iter().map(FieldValue::Present));
        [from_admitted_slots(key, slots.into_boxed_slice()), nested]
    }

    /// The derived `Debug` output for [`debug_samples`], captured from
    /// `#[derive(Debug)]` on `Value` before the derive was replaced by the
    /// iterative impl. The hand-written impl must reproduce it byte for byte.
    const DERIVED_COMPACT: [&str; 2] = [
        "Composite(CompositeValue { declaration: NodeKey(0000000000000000000000000000000000000000000000000000000000000009), slots: [Present(Option(OptionValue { payload_type: Integer, payload: Some(Integer(Integer(1))), occ: Integer(2) })), Absent, Null, Present(Collection(CollectionValue { collection_type: CollectionType { kind: Sequence, element: Boolean, bound: None }, elements: [Boolean(true), Boolean(false)], occ: Integer(3) })), Present(Collection(CollectionValue { collection_type: CollectionType { kind: Sequence, element: Boolean, bound: None }, elements: [], occ: Integer(1) })), Present(Option(OptionValue { payload_type: Option(Boolean), payload: None, occ: Integer(1) })), Present(Option(OptionValue { payload_type: Collection(CollectionType { kind: Sequence, element: Composite(NodeKey(0000000000000000000000000000000000000000000000000000000000000009)), bound: None }), payload: Some(Collection(CollectionValue { collection_type: CollectionType { kind: Sequence, element: Composite(NodeKey(0000000000000000000000000000000000000000000000000000000000000009)), bound: None }, elements: [Composite(CompositeValue { declaration: NodeKey(0000000000000000000000000000000000000000000000000000000000000009), slots: [Present(Boolean(true))], occ: Integer(2) })], occ: Integer(3) })), occ: Integer(4) })), Present(Rational(Rational { numerator: Integer(1), denominator: Integer(2) })), Present(Decimal(Decimal { representation: DecimalRepresentation { coefficient: Integer(125), scale: 2 }, normalized: DecimalRepresentation { coefficient: Integer(125), scale: 2 } })), Present(Float(IeeeValue { width: Binary64, bits: 4609434218613702656 })), Present(Quantity(Quantity { magnitude: Rational { numerator: Integer(1), denominator: Integer(2) }, unit: UnitId(quire.checked-semantic-node/v1, 0000000000000000000000000000000000000000000000000000000000000009) })), Present(Text(Text { text_type: TextType { min: 0, max: 10, profile: UnicodeScalars }, payload: TextPayload { text: \"hi\", provenance: Runtime }, retained: \"hi\" })), Present(Reference(ObjectReference { universe: UniverseId(0000000000000000000000000000000000000000000000000000000000000005), object_type: EffectiveId(0000000000000000000000000000000000000000000000000000000000000006), object: ObjectId(\"o-1\") })), Present(Enum(EnumMember { variant: VariantId(0000000000000000000000000000000000000000000000000000000000000003), rank: 0 })), Present(Population(PopulationId(0000000000000000000000000000000000000000000000000000000000000004)))], occ: Integer(20) })",
        "Option(OptionValue { payload_type: Collection(CollectionType { kind: Sequence, element: Composite(NodeKey(0000000000000000000000000000000000000000000000000000000000000009)), bound: None }), payload: Some(Collection(CollectionValue { collection_type: CollectionType { kind: Sequence, element: Composite(NodeKey(0000000000000000000000000000000000000000000000000000000000000009)), bound: None }, elements: [Composite(CompositeValue { declaration: NodeKey(0000000000000000000000000000000000000000000000000000000000000009), slots: [Present(Boolean(true))], occ: Integer(2) })], occ: Integer(3) })), occ: Integer(4) })",
    ];

    /// As [`DERIVED_COMPACT`], in alternate (`{:#?}`) mode.
    const DERIVED_ALTERNATE: [&str; 2] = [
        r#"Composite(
    CompositeValue {
        declaration: NodeKey(0000000000000000000000000000000000000000000000000000000000000009),
        slots: [
            Present(
                Option(
                    OptionValue {
                        payload_type: Integer,
                        payload: Some(
                            Integer(
                                Integer(
                                    1,
                                ),
                            ),
                        ),
                        occ: Integer(
                            2,
                        ),
                    },
                ),
            ),
            Absent,
            Null,
            Present(
                Collection(
                    CollectionValue {
                        collection_type: CollectionType {
                            kind: Sequence,
                            element: Boolean,
                            bound: None,
                        },
                        elements: [
                            Boolean(
                                true,
                            ),
                            Boolean(
                                false,
                            ),
                        ],
                        occ: Integer(
                            3,
                        ),
                    },
                ),
            ),
            Present(
                Collection(
                    CollectionValue {
                        collection_type: CollectionType {
                            kind: Sequence,
                            element: Boolean,
                            bound: None,
                        },
                        elements: [],
                        occ: Integer(
                            1,
                        ),
                    },
                ),
            ),
            Present(
                Option(
                    OptionValue {
                        payload_type: Option(
                            Boolean,
                        ),
                        payload: None,
                        occ: Integer(
                            1,
                        ),
                    },
                ),
            ),
            Present(
                Option(
                    OptionValue {
                        payload_type: Collection(
                            CollectionType {
                                kind: Sequence,
                                element: Composite(
                                    NodeKey(0000000000000000000000000000000000000000000000000000000000000009),
                                ),
                                bound: None,
                            },
                        ),
                        payload: Some(
                            Collection(
                                CollectionValue {
                                    collection_type: CollectionType {
                                        kind: Sequence,
                                        element: Composite(
                                            NodeKey(0000000000000000000000000000000000000000000000000000000000000009),
                                        ),
                                        bound: None,
                                    },
                                    elements: [
                                        Composite(
                                            CompositeValue {
                                                declaration: NodeKey(0000000000000000000000000000000000000000000000000000000000000009),
                                                slots: [
                                                    Present(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                ],
                                                occ: Integer(
                                                    2,
                                                ),
                                            },
                                        ),
                                    ],
                                    occ: Integer(
                                        3,
                                    ),
                                },
                            ),
                        ),
                        occ: Integer(
                            4,
                        ),
                    },
                ),
            ),
            Present(
                Rational(
                    Rational {
                        numerator: Integer(
                            1,
                        ),
                        denominator: Integer(
                            2,
                        ),
                    },
                ),
            ),
            Present(
                Decimal(
                    Decimal {
                        representation: DecimalRepresentation {
                            coefficient: Integer(
                                125,
                            ),
                            scale: 2,
                        },
                        normalized: DecimalRepresentation {
                            coefficient: Integer(
                                125,
                            ),
                            scale: 2,
                        },
                    },
                ),
            ),
            Present(
                Float(
                    IeeeValue {
                        width: Binary64,
                        bits: 4609434218613702656,
                    },
                ),
            ),
            Present(
                Quantity(
                    Quantity {
                        magnitude: Rational {
                            numerator: Integer(
                                1,
                            ),
                            denominator: Integer(
                                2,
                            ),
                        },
                        unit: UnitId(quire.checked-semantic-node/v1, 0000000000000000000000000000000000000000000000000000000000000009),
                    },
                ),
            ),
            Present(
                Text(
                    Text {
                        text_type: TextType {
                            min: 0,
                            max: 10,
                            profile: UnicodeScalars,
                        },
                        payload: TextPayload {
                            text: "hi",
                            provenance: Runtime,
                        },
                        retained: "hi",
                    },
                ),
            ),
            Present(
                Reference(
                    ObjectReference {
                        universe: UniverseId(0000000000000000000000000000000000000000000000000000000000000005),
                        object_type: EffectiveId(0000000000000000000000000000000000000000000000000000000000000006),
                        object: ObjectId(
                            "o-1",
                        ),
                    },
                ),
            ),
            Present(
                Enum(
                    EnumMember {
                        variant: VariantId(0000000000000000000000000000000000000000000000000000000000000003),
                        rank: 0,
                    },
                ),
            ),
            Present(
                Population(
                    PopulationId(0000000000000000000000000000000000000000000000000000000000000004),
                ),
            ),
        ],
        occ: Integer(
            20,
        ),
    },
)"#,
        r#"Option(
    OptionValue {
        payload_type: Collection(
            CollectionType {
                kind: Sequence,
                element: Composite(
                    NodeKey(0000000000000000000000000000000000000000000000000000000000000009),
                ),
                bound: None,
            },
        ),
        payload: Some(
            Collection(
                CollectionValue {
                    collection_type: CollectionType {
                        kind: Sequence,
                        element: Composite(
                            NodeKey(0000000000000000000000000000000000000000000000000000000000000009),
                        ),
                        bound: None,
                    },
                    elements: [
                        Composite(
                            CompositeValue {
                                declaration: NodeKey(0000000000000000000000000000000000000000000000000000000000000009),
                                slots: [
                                    Present(
                                        Boolean(
                                            true,
                                        ),
                                    ),
                                ],
                                occ: Integer(
                                    2,
                                ),
                            },
                        ),
                    ],
                    occ: Integer(
                        3,
                    ),
                },
            ),
        ),
        occ: Integer(
            4,
        ),
    },
)"#,
    ];

    /// `Value`'s `Debug` prints exactly what `#[derive(Debug)]` printed,
    /// in compact and in alternate mode, through every variant and slot
    /// state, from a composite root and from an option root.
    #[test]
    fn value_debug_matches_the_derived_format() {
        for ((value, compact), alternate) in debug_samples()
            .iter()
            .zip(DERIVED_COMPACT)
            .zip(DERIVED_ALTERNATE)
        {
            assert_eq!(format!("{value:?}"), compact);
            assert_eq!(format!("{value:#?}"), alternate);
        }
    }

    /// Levels of nesting in [`deep_value`]. A recursive walk spends at least
    /// one stack frame per level, so this many levels overflows the 512 KiB
    /// stack [`on_small_stack`] runs each walk on by a wide margin.
    const DEEP: usize = 100_000;

    /// A well-typed value `DEEP` levels deep that cycles through all three
    /// nesting variants: a record `Node { next: Option<Sequence<Node>> }`,
    /// whose declared types stay three levels deep however deep the value
    /// goes, so the depth is value depth and never type depth.
    fn deep_value() -> (ValueType, Value) {
        let node = NodeKey::from_digest(digest(7));
        let node_type = ValueType::Composite(node);
        let sequence_type = CollectionType::new(
            crate::collection::CollectionKind::Sequence,
            node_type.clone(),
            None,
        );
        let mut value = from_admitted_slots(node, vec![FieldValue::Absent].into_boxed_slice());
        let mut level = 1;
        while level < DEEP {
            let sequence = crate::collection::from_admitted(sequence_type.clone(), vec![value]);
            let next = OptionValue::from_admitted(
                ValueType::collection(sequence_type.clone()),
                Some(sequence),
            );
            value = from_admitted_slots(node, vec![FieldValue::Present(next)].into_boxed_slice());
            level += 3;
        }
        (node_type, value)
    }

    /// Run `walk` on a thread with a 512 KiB stack, so a recursive walk
    /// overflows whatever stack size the test harness itself was given.
    fn on_small_stack(walk: impl FnOnce() + Send + 'static) {
        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(walk)
            .expect("spawn the walk thread")
            .join()
            .expect("the walk completes without overflowing the stack");
    }

    /// A `fmt::Write` sink that counts opening and closing brackets, so
    /// formatting a deep value needs no buffer the size of its output.
    #[derive(Default)]
    struct BracketCount {
        open: usize,
        close: usize,
    }

    impl core::fmt::Write for BracketCount {
        fn write_str(&mut self, text: &str) -> core::fmt::Result {
            for byte in text.bytes() {
                match byte {
                    b'(' | b'{' | b'[' => self.open += 1,
                    b')' | b'}' | b']' => self.close += 1,
                    _ => {}
                }
            }
            Ok(())
        }
    }

    /// A value nested `DEEP` levels deep compares equal to its clone, and it
    /// and its clone drop, on a 512 KiB stack.
    #[trace("TC-735", "FR-262-AC-2")]
    #[test]
    fn a_deep_value_equals_its_clone_and_both_drop() {
        on_small_stack(|| {
            let (_, value) = deep_value();
            let clone = value.clone();
            assert_eq!(
                crate::key::compare_keys(&value, &clone),
                Some(core::cmp::Ordering::Equal)
            );
            drop(value);
            drop(clone);
        });
    }

    /// Dropping a value nested `DEEP` levels deep completes on a 512 KiB
    /// stack.
    #[trace("TC-735", "FR-262-AC-2")]
    #[test]
    fn a_deep_value_drops_on_a_small_stack() {
        on_small_stack(|| {
            let (_, value) = deep_value();
            drop(value);
        });
    }

    /// Dropping one handle to a shared deep value leaves the other handle's
    /// contents whole: the drop takes apart only nodes it owns alone.
    #[test]
    fn dropping_one_handle_leaves_a_shared_deep_value_whole() {
        on_small_stack(|| {
            let (_, value) = deep_value();
            let parent = from_admitted_slots(
                NodeKey::from_digest(digest(8)),
                vec![FieldValue::Present(value.clone())].into_boxed_slice(),
            );
            let occ = value.occ();
            drop(parent);
            assert_eq!(value.occ(), occ);
            let Value::Composite(root) = &value else {
                panic!("the deep value is a composite");
            };
            assert!(matches!(
                root.slots(),
                [FieldValue::Present(Value::Option(_))]
            ));
        });
    }

    /// Formatting a value nested `DEEP` levels deep with `Debug` completes
    /// on a 512 KiB stack, and the output closes every level it opens.
    #[test]
    fn a_deep_value_debug_formats_on_a_small_stack() {
        on_small_stack(|| {
            let (_, value) = deep_value();
            let mut sink = BracketCount::default();
            core::fmt::write(&mut sink, format_args!("{value:?}")).expect("format");
            assert!(sink.open > DEEP);
            assert_eq!(sink.open, sink.close);
        });
    }

    /// `admits` decides a value nested `DEEP` levels deep on a 512 KiB stack:
    /// it reads the type the composite carries and never descends into it.
    #[test]
    fn a_deep_value_is_admitted_on_a_small_stack() {
        on_small_stack(|| {
            let (node_type, value) = deep_value();
            assert!(node_type.admits(&value));
            assert!(!ValueType::Boolean.admits(&value));
        });
    }

    /// Equality planning and evaluation compare two separately built values
    /// nested `DEEP` levels deep on a 512 KiB stack, and find them equal.
    #[test]
    fn deep_values_compare_equal_on_a_small_stack() {
        on_small_stack(|| {
            let (_, left) = deep_value();
            let (_, right) = deep_value();
            assert!(crate::equality::plan_equality(&left, &right).is_ok());
            let outcome = crate::equality::planned_equality(&left, &right, &mut generous_meter());
            assert_eq!(outcome.completed(), Some(true));
        });
    }

    /// The canonical key orders two separately built values nested `DEEP`
    /// levels deep on a 512 KiB stack, and finds them equal.
    #[test]
    fn deep_values_compare_keys_on_a_small_stack() {
        on_small_stack(|| {
            let (_, left) = deep_value();
            let (_, right) = deep_value();
            assert_eq!(
                crate::key::compare_keys(&left, &right),
                Some(core::cmp::Ordering::Equal)
            );
        });
    }

    /// A well-typed value `levels` deep whose declared type is as deep as
    /// the value: levels alternate between `Option` and a `Sequence`, and
    /// each level declares its payload type by cloning the type of the level
    /// below it, as an admitting caller does.
    fn deep_typed_value(levels: usize) -> (ValueType, Value) {
        let mut value_type = ValueType::Integer;
        let mut value = Value::Integer(Integer::one());
        for level in 0..levels {
            if level % 2 == 0 {
                value = OptionValue::from_admitted(value_type.clone(), Some(value));
                value_type = ValueType::option(value_type);
            } else {
                let sequence_type = CollectionType::new(
                    crate::collection::CollectionKind::Sequence,
                    value_type,
                    None,
                );
                value = crate::collection::from_admitted(sequence_type.clone(), vec![value]);
                value_type = ValueType::collection(sequence_type);
            }
        }
        (value_type, value)
    }

    /// The number of distinct type nodes (`Option` and `Collection` links)
    /// the declared types inside `value` own, counted by allocation
    /// identity. A node already counted ends its chain's walk, so the count
    /// is linear in what is allocated.
    fn type_nodes(value: &Value) -> usize {
        let mut seen = std::collections::HashSet::new();
        let mut count_chain = |start: &ValueType| {
            let mut link = Some(start);
            while let Some(current) = link {
                link = match current {
                    ValueType::Option(payload) => seen
                        .insert(Arc::as_ptr(payload) as usize)
                        .then_some(&**payload),
                    ValueType::Collection(collection) => seen
                        .insert(Arc::as_ptr(collection) as usize)
                        .then(|| collection.element()),
                    _ => None,
                };
            }
        };
        let mut level = Some(value);
        while let Some(current) = level {
            level = match current {
                Value::Option(option) => {
                    count_chain(option.payload_type());
                    option.payload()
                }
                Value::Collection(collection) => {
                    count_chain(collection.collection_type().element());
                    collection.elements().first()
                }
                _ => None,
            };
        }
        seen.len()
    }

    /// A value `2N` levels deep, declared with types `2N` deep, holds about
    /// twice the type nodes of a value `N` deep: payload types are shared,
    /// not copied per level, so memory is linear in the value.
    #[trace("TC-735", "FR-262-AC-3")]
    #[test]
    fn type_nodes_grow_linearly_with_value_depth() {
        let small = type_nodes(&deep_typed_value(1_000).1);
        let large = type_nodes(&deep_typed_value(2_000).1);
        assert!(small >= 990, "{small}");
        assert!(large <= 2 * small + 2, "{large} nodes at 2N, {small} at N");
    }

    /// FR-262-AC-3: a recursive value `DEEP` levels deep, declared with
    /// types `DEEP` deep, is built, admitted, cloned, compared, hashed,
    /// formatted and dropped on a 512 KiB stack.
    #[trace("TC-735", "FR-262-AC-3")]
    #[test]
    fn a_deep_value_with_deep_types_is_admitted_in_linear_memory() {
        on_small_stack(|| {
            use core::hash::{Hash, Hasher};
            let (value_type, value) = deep_typed_value(DEEP);
            assert!(value_type.admits(&value));
            assert!(type_nodes(&value) <= 2 * DEEP);

            let clone = value_type.clone();
            assert!(clone == value_type);
            let mut left = std::collections::hash_map::DefaultHasher::new();
            let mut right = std::collections::hash_map::DefaultHasher::new();
            value_type.hash(&mut left);
            clone.hash(&mut right);
            assert_eq!(left.finish(), right.finish());

            let mut sink = BracketCount::default();
            core::fmt::write(&mut sink, format_args!("{value_type:?}")).expect("format");
            assert!(sink.open >= DEEP);
            assert_eq!(sink.open, sink.close);

            // `value` holds a share of every level, `value_type` and `clone`
            // share the whole chain: the last holder to drop must detach the
            // rest of it, so this order exercises a shared-in-the-middle drop.
            drop(value);
            drop(value_type);
            drop(clone);
        });
    }
}
