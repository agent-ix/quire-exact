// SPDX-License-Identifier: AGPL-3.0-or-later
//! `quire.value.accounting/v1`: `u64` limits, named ordered charge points and
//! the exact incomplete record.
//!
//! Semantic-size counters record a high-water maximum; `work_units` and
//! `result_units` are cumulative. Every charge is atomic: the first
//! unavailable counter, in the order written in its definition row, returns
//! [`Incomplete`] and nothing is consumed.
//!
//! Every type here references only [`crate::integer::Integer`], which is itself
//! a kernel type. `ScalarLimits` does not derive `serde::Serialize`/`Deserialize`:
//! the kernel takes no wire dependency, and its fields are `pub`, so a wire to
//! `ScalarLimits` conversion is the caller's job.

use crate::cancel::Cancel;
use crate::integer::Integer;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::num::NonZeroU64;

#[cfg(kani)]
mod kani_proofs;

#[cfg(feature = "test-support")]
const CHARGE_LOG_CAPACITY: usize = 4096;

/// A `usize` length or count as an accounting amount.
///
/// Every supported target has pointers of at most 64 bits, so an in-memory
/// length always fits `u64`; a wider target would violate that invariant.
///
/// # Panics
///
/// Only on a target whose pointers are wider than 64 bits.
pub fn length_amount(length: usize) -> u64 {
    u64::try_from(length)
        .expect("in-memory lengths fit u64 on targets with pointers of at most 64 bits")
}

/// `ScalarLimitsV1`. Every member is required; zero is a real limit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScalarLimits {
    /// Maximum magnitude bit length.
    pub integer_bits: u64,
    /// Maximum base-ten digit count.
    pub decimal_digits: u64,
    /// Maximum decimal-place shift in one operation.
    pub scale_expansion: u64,
    /// Maximum UTF-8 input bytes.
    pub text_input_bytes: u64,
    /// Maximum decoded Unicode scalars.
    pub text_scalars: u64,
    /// Maximum normalized output scalars.
    pub normalized_scalars: u64,
    /// Maximum unit graph edges.
    pub unit_edges: u64,
    /// Maximum typed value occurrences.
    pub value_occurrences: u64,
    /// Maximum cumulative work units.
    pub work_units: u64,
    /// Maximum cumulative retained result units.
    pub result_units: u64,
}

/// One counter of [`ScalarLimits`].
///
/// quire:canonical
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LimitKind {
    /// `integer_bits`.
    IntegerBits,
    /// `decimal_digits`.
    DecimalDigits,
    /// `scale_expansion`.
    ScaleExpansion,
    /// `text_input_bytes`.
    TextInputBytes,
    /// `text_scalars`.
    TextScalars,
    /// `normalized_scalars`.
    NormalizedScalars,
    /// `unit_edges`.
    UnitEdges,
    /// `value_occurrences`.
    ValueOccurrences,
    /// `work_units`.
    WorkUnits,
    /// `result_units`.
    ResultUnits,
}

impl LimitKind {
    /// Every counter in `ScalarLimitsV1` member order.
    pub const ALL: [Self; 10] = [
        Self::IntegerBits,
        Self::DecimalDigits,
        Self::ScaleExpansion,
        Self::TextInputBytes,
        Self::TextScalars,
        Self::NormalizedScalars,
        Self::UnitEdges,
        Self::ValueOccurrences,
        Self::WorkUnits,
        Self::ResultUnits,
    ];

    /// Normative member name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IntegerBits => "integer_bits",
            Self::DecimalDigits => "decimal_digits",
            Self::ScaleExpansion => "scale_expansion",
            Self::TextInputBytes => "text_input_bytes",
            Self::TextScalars => "text_scalars",
            Self::NormalizedScalars => "normalized_scalars",
            Self::UnitEdges => "unit_edges",
            Self::ValueOccurrences => "value_occurrences",
            Self::WorkUnits => "work_units",
            Self::ResultUnits => "result_units",
        }
    }

    /// Whether this counter is cumulative rather than a high-water size.
    pub fn is_cumulative(self) -> bool {
        matches!(self, Self::WorkUnits | Self::ResultUnits)
    }

    /// The number of counters; every per-counter array is sized from it.
    const COUNT: usize = Self::ALL.len();

    fn index(self) -> usize {
        match self {
            Self::IntegerBits => 0,
            Self::DecimalDigits => 1,
            Self::ScaleExpansion => 2,
            Self::TextInputBytes => 3,
            Self::TextScalars => 4,
            Self::NormalizedScalars => 5,
            Self::UnitEdges => 6,
            Self::ValueOccurrences => 7,
            Self::WorkUnits => 8,
            Self::ResultUnits => 9,
        }
    }

    fn limit(self, limits: &ScalarLimits) -> u64 {
        match self {
            Self::IntegerBits => limits.integer_bits,
            Self::DecimalDigits => limits.decimal_digits,
            Self::ScaleExpansion => limits.scale_expansion,
            Self::TextInputBytes => limits.text_input_bytes,
            Self::TextScalars => limits.text_scalars,
            Self::NormalizedScalars => limits.normalized_scalars,
            Self::UnitEdges => limits.unit_edges,
            Self::ValueOccurrences => limits.value_occurrences,
            Self::WorkUnits => limits.work_units,
            Self::ResultUnits => limits.result_units,
        }
    }
}

/// A normative named charge point.
///
/// quire:canonical
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChargePoint {
    /// `decimal.operands`.
    DecimalOperands,
    /// `decimal.scale-expansion`.
    DecimalScaleExpansion,
    /// `decimal.arithmetic`.
    DecimalArithmetic,
    /// `decimal.rounding`.
    DecimalRounding,
    /// `decimal.result-retain`.
    DecimalResultRetain,
    /// `text.input-bytes`.
    TextInputBytes,
    /// `text.decode-scalars`.
    TextDecodeScalars,
    /// `text.normalize-input`.
    TextNormalizeInput,
    /// `text.normalize-output`.
    TextNormalizeOutput,
    /// `text.result-retain`.
    TextResultRetain,
    /// `enum.identity-read`.
    EnumIdentityRead,
    /// `enum.result-retain`.
    EnumResultRetain,
    /// `unit.identity-read`.
    UnitIdentityRead,
    /// `unit.edge`.
    UnitEdge,
    /// `unit.rational-arithmetic`.
    UnitRationalArithmetic,
    /// `unit.target-domain`.
    UnitTargetDomain,
    /// `unit.result-retain`.
    UnitResultRetain,
    /// `integer-division.operands`.
    IntegerDivisionOperands,
    /// `integer-division.arithmetic`.
    IntegerDivisionArithmetic,
    /// `integer-division.domain`.
    IntegerDivisionDomain,
    /// `integer-division.result-retain`.
    IntegerDivisionResultRetain,
    /// `integer-modulus.operands`.
    IntegerModulusOperands,
    /// `integer-modulus.arithmetic`.
    IntegerModulusArithmetic,
    /// `integer-modulus.domain`.
    IntegerModulusDomain,
    /// `integer-modulus.result-retain`.
    IntegerModulusResultRetain,
    /// `ieee.operands`.
    IeeeOperands,
    /// `ieee.exact-intermediate`.
    IeeeExactIntermediate,
    /// `ieee.round`.
    IeeeRound,
    /// `ieee.result-retain`.
    IeeeResultRetain,
    /// `equality.plan-form`.
    EqualityPlanForm,
    /// `equality.plan`.
    EqualityPlan,
    /// `equality.pair`.
    EqualityPair,
    /// `equality.result-retain`.
    EqualityResultRetain,
    /// `function.call`: one checked function call.
    FunctionCall,
    /// `collection.element`.
    CollectionElement,
    /// `collection.visit`.
    CollectionVisit,
    /// `collection.member-walk`.
    CollectionMemberWalk,
    /// `collection.member-test`.
    CollectionMemberTest,
    /// `collection.bound`.
    CollectionBound,
    /// `collection.result-retain`.
    CollectionResultRetain,
    /// `composite.result-retain`.
    CompositeResultRetain,
    /// `integer-arithmetic.operands`.
    IntegerArithmeticOperands,
    /// `integer-arithmetic.arithmetic`.
    IntegerArithmeticArithmetic,
    /// `integer-arithmetic.result-retain`.
    IntegerArithmeticResultRetain,
    /// `rational-arithmetic.operands`.
    RationalArithmeticOperands,
    /// `rational-arithmetic.arithmetic`.
    RationalArithmeticArithmetic,
    /// `rational-arithmetic.normalize`.
    RationalArithmeticNormalize,
    /// `rational-arithmetic.result-retain`.
    RationalArithmeticResultRetain,
    /// `ordering.operands`.
    OrderingOperands,
    /// `ordering.arithmetic`.
    OrderingArithmetic,
    /// `ordering.result-retain`.
    OrderingResultRetain,
    /// `boolean.result-retain`.
    BooleanResultRetain,
    /// `lookup.key`.
    LookupKey,
    /// `lookup.result-retain`.
    LookupResultRetain,
    /// `population.visit`.
    PopulationVisit,
    /// `dispatch.select`: one dispatched `receiver.member(args)` call.
    DispatchSelect,
    /// `declaration.check`: one checked-family declaration's own checking
    /// work -- distinct from
    /// `FunctionCall`, which charges one *evaluated* call, not one
    /// *checked* declaration. Sized by the declaration's own preimage
    /// field-write count (a caller's identity-preimage metrics); denied into a `Limit`
    /// outcome naming the work-budget stage limit, never `Incomplete`
    /// (`check` never returns `Incomplete`).
    DeclarationCheck,
    /// `graph.expand`: one node enqueued by a `reaches`
    /// walk, its source included.
    GraphExpand,
    /// `graph.edge`: one edge target visited by a `reaches` walk.
    GraphEdge,
    /// `graph.result-retain`: a `reaches` walk's Boolean result.
    GraphResultRetain,
    /// `model.deref`: one `deref(...)` of a model reference.
    ModelDeref,
    /// `model.navigate`: one model attribute field read
    /// (`self.f`/`deref(r).f`).
    ModelNavigate,
    /// `temporal.visit`: one caller-owned temporal operator visit.
    TemporalVisit,
}

impl ChargePoint {
    /// Every named point, grouped by family in normative order.
    pub const ALL: [Self; 63] = [
        Self::DecimalOperands,
        Self::DecimalScaleExpansion,
        Self::DecimalArithmetic,
        Self::DecimalRounding,
        Self::DecimalResultRetain,
        Self::TextInputBytes,
        Self::TextDecodeScalars,
        Self::TextNormalizeInput,
        Self::TextNormalizeOutput,
        Self::TextResultRetain,
        Self::EnumIdentityRead,
        Self::EnumResultRetain,
        Self::UnitIdentityRead,
        Self::UnitEdge,
        Self::UnitRationalArithmetic,
        Self::UnitTargetDomain,
        Self::UnitResultRetain,
        Self::IntegerDivisionOperands,
        Self::IntegerDivisionArithmetic,
        Self::IntegerDivisionDomain,
        Self::IntegerDivisionResultRetain,
        Self::IntegerModulusOperands,
        Self::IntegerModulusArithmetic,
        Self::IntegerModulusDomain,
        Self::IntegerModulusResultRetain,
        Self::IeeeOperands,
        Self::IeeeExactIntermediate,
        Self::IeeeRound,
        Self::IeeeResultRetain,
        Self::EqualityPlanForm,
        Self::EqualityPlan,
        Self::EqualityPair,
        Self::EqualityResultRetain,
        Self::FunctionCall,
        Self::CollectionElement,
        Self::CollectionVisit,
        Self::CollectionMemberWalk,
        Self::CollectionMemberTest,
        Self::CollectionBound,
        Self::CollectionResultRetain,
        Self::CompositeResultRetain,
        Self::IntegerArithmeticOperands,
        Self::IntegerArithmeticArithmetic,
        Self::IntegerArithmeticResultRetain,
        Self::RationalArithmeticOperands,
        Self::RationalArithmeticArithmetic,
        Self::RationalArithmeticNormalize,
        Self::RationalArithmeticResultRetain,
        Self::OrderingOperands,
        Self::OrderingArithmetic,
        Self::OrderingResultRetain,
        Self::BooleanResultRetain,
        Self::LookupKey,
        Self::LookupResultRetain,
        Self::PopulationVisit,
        Self::DispatchSelect,
        Self::DeclarationCheck,
        Self::GraphExpand,
        Self::GraphEdge,
        Self::GraphResultRetain,
        Self::ModelDeref,
        Self::ModelNavigate,
        Self::TemporalVisit,
    ];

    /// Normative identifier.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DecimalOperands => "decimal.operands",
            Self::DecimalScaleExpansion => "decimal.scale-expansion",
            Self::DecimalArithmetic => "decimal.arithmetic",
            Self::DecimalRounding => "decimal.rounding",
            Self::DecimalResultRetain => "decimal.result-retain",
            Self::TextInputBytes => "text.input-bytes",
            Self::TextDecodeScalars => "text.decode-scalars",
            Self::TextNormalizeInput => "text.normalize-input",
            Self::TextNormalizeOutput => "text.normalize-output",
            Self::TextResultRetain => "text.result-retain",
            Self::EnumIdentityRead => "enum.identity-read",
            Self::EnumResultRetain => "enum.result-retain",
            Self::UnitIdentityRead => "unit.identity-read",
            Self::UnitEdge => "unit.edge",
            Self::UnitRationalArithmetic => "unit.rational-arithmetic",
            Self::UnitTargetDomain => "unit.target-domain",
            Self::UnitResultRetain => "unit.result-retain",
            Self::IntegerDivisionOperands => "integer-division.operands",
            Self::IntegerDivisionArithmetic => "integer-division.arithmetic",
            Self::IntegerDivisionDomain => "integer-division.domain",
            Self::IntegerDivisionResultRetain => "integer-division.result-retain",
            Self::IntegerModulusOperands => "integer-modulus.operands",
            Self::IntegerModulusArithmetic => "integer-modulus.arithmetic",
            Self::IntegerModulusDomain => "integer-modulus.domain",
            Self::IntegerModulusResultRetain => "integer-modulus.result-retain",
            Self::IeeeOperands => "ieee.operands",
            Self::IeeeExactIntermediate => "ieee.exact-intermediate",
            Self::IeeeRound => "ieee.round",
            Self::IeeeResultRetain => "ieee.result-retain",
            Self::EqualityPlanForm => "equality.plan-form",
            Self::EqualityPlan => "equality.plan",
            Self::EqualityPair => "equality.pair",
            Self::EqualityResultRetain => "equality.result-retain",
            Self::FunctionCall => "function.call",
            Self::CollectionElement => "collection.element",
            Self::CollectionVisit => "collection.visit",
            Self::CollectionMemberWalk => "collection.member-walk",
            Self::CollectionMemberTest => "collection.member-test",
            Self::CollectionBound => "collection.bound",
            Self::CollectionResultRetain => "collection.result-retain",
            Self::CompositeResultRetain => "composite.result-retain",
            Self::IntegerArithmeticOperands => "integer-arithmetic.operands",
            Self::IntegerArithmeticArithmetic => "integer-arithmetic.arithmetic",
            Self::IntegerArithmeticResultRetain => "integer-arithmetic.result-retain",
            Self::RationalArithmeticOperands => "rational-arithmetic.operands",
            Self::RationalArithmeticArithmetic => "rational-arithmetic.arithmetic",
            Self::RationalArithmeticNormalize => "rational-arithmetic.normalize",
            Self::RationalArithmeticResultRetain => "rational-arithmetic.result-retain",
            Self::OrderingOperands => "ordering.operands",
            Self::OrderingArithmetic => "ordering.arithmetic",
            Self::OrderingResultRetain => "ordering.result-retain",
            Self::BooleanResultRetain => "boolean.result-retain",
            Self::LookupKey => "lookup.key",
            Self::LookupResultRetain => "lookup.result-retain",
            Self::PopulationVisit => "population.visit",
            Self::DispatchSelect => "dispatch.select",
            Self::DeclarationCheck => "declaration.check",
            Self::GraphExpand => "graph.expand",
            Self::GraphEdge => "graph.edge",
            Self::GraphResultRetain => "graph.result-retain",
            Self::ModelDeref => "model.deref",
            Self::ModelNavigate => "model.navigate",
            Self::TemporalVisit => "temporal.visit",
        }
    }

    /// Resolve a normative identifier.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|point| point.as_str() == code)
    }
}

/// The exact incomplete record: `incomplete { limit_kind, limit, consumed,
/// next_charge, charge_point }`. It never carries a partial value or any other
/// member.
///
/// quire:canonical
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Incomplete {
    /// First unavailable counter in `ScalarLimitsV1` field order.
    pub limit_kind: LimitKind,
    /// The configured counter limit for an ordinary shortage. For an injected
    /// denial or cancellation, this is the work consumed before the charge,
    /// equal to [`Self::consumed`] rather than `ScalarLimits::work_units`.
    pub limit: u64,
    /// That counter's consumed value before the denied charge.
    pub consumed: u64,
    /// The denied amount (a size for high-water counters, an addition for
    /// cumulative counters). It is a mathematical integer because a unit
    /// integer power can require more than `u64::MAX`.
    pub next_charge: Integer,
    /// The named point whose charge was denied.
    pub charge_point: ChargePoint,
}

/// A fault-injection request: deny the `occurrence`th (1-based) charge
/// at `point`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InjectedDenial {
    /// The named point to deny.
    pub point: ChargePoint,
    /// Which occurrence of that point to deny, starting at one.
    pub occurrence: NonZeroU64,
}

/// One exact `{ counter: amount }` charge vector.
///
/// **`pub`** so a caller outside this crate can read it, together with
/// [`Meter::charge`]/[`Meter::charge_plan`] and [`length_amount`].
#[derive(Clone)]
pub struct Charge {
    point: ChargePoint,
    storage: ChargeStorage,
}

#[derive(Clone)]
enum ChargeStorage {
    Common(CommonCharge),
    General(GeneralCharge),
}

// The common admission payload owns no exact integer, allocation or drop glue.
#[derive(Clone, Copy, Eq, PartialEq)]
struct CommonCharge {
    sizes: [Option<(LimitKind, u64)>; 2],
    work_units: u64,
    result_units: u64,
}

impl CommonCharge {
    fn new() -> Self {
        Self {
            sizes: [None; 2],
            work_units: 1,
            result_units: 0,
        }
    }

    fn into_general(self) -> GeneralCharge {
        let mut sizes = InlineSizes::new();
        for (kind, amount) in self.sizes.into_iter().flatten() {
            sizes.push(kind, ChargeAmount::Units(amount));
        }
        GeneralCharge {
            sizes,
            work_units: ChargeAmount::Units(self.work_units),
            result_units: ChargeAmount::Units(self.result_units),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct GeneralCharge {
    sizes: InlineSizes,
    work_units: ChargeAmount,
    result_units: ChargeAmount,
}

impl PartialEq for Charge {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
            && match (&self.storage, &other.storage) {
                (ChargeStorage::Common(left), ChargeStorage::Common(right)) => left == right,
                (ChargeStorage::Common(left), ChargeStorage::General(right)) => {
                    &left.into_general() == right
                }
                (ChargeStorage::General(left), ChargeStorage::Common(right)) => {
                    left == &right.into_general()
                }
                (ChargeStorage::General(left), ChargeStorage::General(right)) => left == right,
            }
    }
}
impl Eq for Charge {}

impl fmt::Debug for Charge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = |value: &GeneralCharge, formatter: &mut fmt::Formatter<'_>| {
            formatter
                .debug_struct("Charge")
                .field("point", &self.point)
                .field("sizes", &value.sizes)
                .field("work_units", &value.work_units)
                .field("result_units", &value.result_units)
                .finish()
        };
        match &self.storage {
            ChargeStorage::Common(value) => fields(&value.into_general(), formatter),
            ChargeStorage::General(value) => fields(value, formatter),
        }
    }
}

// Admission is shared, but its common instantiation receives only u64 values.
trait MeterAmount: Copy {
    fn units(self) -> Option<u64>;
    fn integer(self) -> Integer;
}

impl MeterAmount for u64 {
    fn units(self) -> Option<u64> {
        Some(self)
    }
    fn integer(self) -> Integer {
        Integer::from(self)
    }
}

impl MeterAmount for &ChargeAmount {
    fn units(self) -> Option<u64> {
        ChargeAmount::units(self)
    }
    fn integer(self) -> Integer {
        ChargeAmount::integer(self)
    }
}

/// Amounts admissible under a `u64` limit stay in machine width. An exact
/// amount outside that domain is retained out of line only for refusal.
#[derive(Clone, Eq, PartialEq)]
enum ChargeAmount {
    Units(u64),
    Exact(Box<Integer>),
}

impl ChargeAmount {
    fn exact(amount: Integer) -> Self {
        match amount.to_u64() {
            Some(units) => Self::Units(units),
            None => Self::Exact(Box::new(amount)),
        }
    }

    fn units(&self) -> Option<u64> {
        match self {
            Self::Units(units) => Some(*units),
            Self::Exact(_) => None,
        }
    }

    fn integer(&self) -> Integer {
        match self {
            Self::Units(units) => Integer::from(*units),
            Self::Exact(exact) => exact.as_ref().clone(),
        }
    }
}

impl fmt::Debug for ChargeAmount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.integer(), formatter)
    }
}

/// The two common requests have named inline slots. Additional requests spill
/// without a count limit and retain insertion order.
#[derive(Clone)]
struct InlineSizes {
    first: Option<(LimitKind, ChargeAmount)>,
    second: Option<(LimitKind, ChargeAmount)>,
    overflow: Vec<(LimitKind, ChargeAmount)>,
}

impl InlineSizes {
    fn new() -> Self {
        Self {
            first: None,
            second: None,
            overflow: Vec::new(),
        }
    }

    fn push(&mut self, kind: LimitKind, amount: ChargeAmount) {
        if self.first.is_none() {
            self.first = Some((kind, amount));
        } else if self.second.is_none() {
            self.second = Some((kind, amount));
        } else {
            self.overflow.push((kind, amount));
        }
    }

    fn iter(&self) -> impl Iterator<Item = &(LimitKind, ChargeAmount)> + Clone {
        self.first
            .iter()
            .chain(self.second.iter())
            .chain(self.overflow.iter())
    }
}

impl fmt::Debug for InlineSizes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl PartialEq for InlineSizes {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
impl Eq for InlineSizes {}

impl Charge {
    /// A charge at `point` with no size yet, a default one work unit, and
    /// no result units.
    pub fn new(point: ChargePoint) -> Self {
        Self {
            point,
            storage: ChargeStorage::Common(CommonCharge::new()),
        }
    }

    fn push_units(mut self, kind: LimitKind, amount: u64) -> Self {
        self.storage = match self.storage {
            ChargeStorage::Common(mut common) => {
                if let Some(slot) = common.sizes.iter_mut().find(|slot| slot.is_none()) {
                    *slot = Some((kind, amount));
                    ChargeStorage::Common(common)
                } else {
                    let mut general = common.into_general();
                    general.sizes.push(kind, ChargeAmount::Units(amount));
                    ChargeStorage::General(general)
                }
            }
            ChargeStorage::General(mut general) => {
                general.sizes.push(kind, ChargeAmount::Units(amount));
                ChargeStorage::General(general)
            }
        };
        self
    }

    fn into_general(self) -> GeneralCharge {
        match self.storage {
            ChargeStorage::Common(common) => common.into_general(),
            ChargeStorage::General(general) => general,
        }
    }

    fn set_units(mut self, amount: u64, result: bool) -> Self {
        match &mut self.storage {
            ChargeStorage::Common(common) => {
                if result {
                    common.result_units = amount;
                } else {
                    common.work_units = amount;
                }
            }
            ChargeStorage::General(general) => {
                if result {
                    general.result_units = ChargeAmount::Units(amount);
                } else {
                    general.work_units = ChargeAmount::Units(amount);
                }
            }
        }
        self
    }

    fn set_exact(self, amount: Integer, result: bool) -> Self {
        let point = self.point;
        let mut general = self.into_general();
        let exact = ChargeAmount::Exact(Box::new(amount));
        if result {
            general.result_units = exact;
        } else {
            general.work_units = exact;
        }
        Self {
            point,
            storage: ChargeStorage::General(general),
        }
    }

    /// A semantic-size charge against `kind`.
    pub fn size(self, kind: LimitKind, amount: u64) -> Self {
        self.push_units(kind, amount)
    }

    /// A semantic-size amount that may exceed `u64::MAX`.
    pub fn exact_size(self, kind: LimitKind, amount: Integer) -> Self {
        match amount.to_u64() {
            Some(units) => self.push_units(kind, units),
            None => {
                let point = self.point;
                let mut general = self.into_general();
                general
                    .sizes
                    .push(kind, ChargeAmount::Exact(Box::new(amount)));
                Self {
                    point,
                    storage: ChargeStorage::General(general),
                }
            }
        }
    }

    /// A `work_units` addition other than the default one.
    pub fn work(self, amount: Integer) -> Self {
        match amount.to_u64() {
            Some(units) => self.set_units(units, false),
            None => self.set_exact(amount, false),
        }
    }

    /// A `result_units` addition.
    pub fn results(self, amount: u64) -> Self {
        self.set_units(amount, true)
    }

    /// A `result_units` addition that may exceed `u64::MAX`.
    pub fn exact_results(self, amount: Integer) -> Self {
        match amount.to_u64() {
            Some(units) => self.set_units(units, true),
            None => self.set_exact(amount, true),
        }
    }
}

/// One admitted charge's point and actual per-admission semantic-size maxima.
///
/// Each absent field means the charge made no request for that size kind;
/// `Some(0)` records an explicit zero request. Repeated requests coalesce
/// within this admission, independently of the meter's high-water counters.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmittedCharge {
    /// The admitted charge point.
    pub point: ChargePoint,
    /// Maximum actual `integer_bits` request in this admission, if present.
    pub integer_bits: Option<u64>,
    /// Maximum actual `decimal_digits` request in this admission, if present.
    pub decimal_digits: Option<u64>,
    /// Maximum actual `scale_expansion` request in this admission, if present.
    pub scale_expansion: Option<u64>,
    /// Maximum actual `text_input_bytes` request in this admission, if present.
    pub text_input_bytes: Option<u64>,
    /// Maximum actual `text_scalars` request in this admission, if present.
    pub text_scalars: Option<u64>,
    /// Maximum actual `normalized_scalars` request in this admission, if present.
    pub normalized_scalars: Option<u64>,
    /// Maximum actual `unit_edges` request in this admission, if present.
    pub unit_edges: Option<u64>,
    /// Maximum actual `value_occurrences` request in this admission, if present.
    pub value_occurrences: Option<u64>,
}

#[cfg(feature = "test-support")]
impl AdmittedCharge {
    fn new(point: ChargePoint) -> Self {
        Self {
            point,
            integer_bits: None,
            decimal_digits: None,
            scale_expansion: None,
            text_input_bytes: None,
            text_scalars: None,
            normalized_scalars: None,
            unit_edges: None,
            value_occurrences: None,
        }
    }

    fn observe(&mut self, kind: LimitKind, amount: u64) {
        let slot = match kind {
            LimitKind::IntegerBits => &mut self.integer_bits,
            LimitKind::DecimalDigits => &mut self.decimal_digits,
            LimitKind::ScaleExpansion => &mut self.scale_expansion,
            LimitKind::TextInputBytes => &mut self.text_input_bytes,
            LimitKind::TextScalars => &mut self.text_scalars,
            LimitKind::NormalizedScalars => &mut self.normalized_scalars,
            LimitKind::UnitEdges => &mut self.unit_edges,
            LimitKind::ValueOccurrences => &mut self.value_occurrences,
            LimitKind::WorkUnits | LimitKind::ResultUnits => return,
        };
        *slot = Some(slot.map_or(amount, |previous| previous.max(amount)));
    }
}

// No admission payload owns a request collection or any other heap storage.
#[cfg(feature = "test-support")]
const _: () = assert!(!core::mem::needs_drop::<AdmittedCharge>());

/// A per-request scalar meter.
///
/// **A count, not a log.** A production meter holds only fixed-size
/// counters: the ten consumed values, the limits, the number of admitted
/// charges and, for an injected denial, the number of admissions at the
/// denied point. The counters own no heap memory, so the meter that bounds an
/// evaluation's work does not itself grow with that work; the const
/// assertion below this type holds that in every production build. The meter
/// also shares the caller's [`Cancel`] handle, a reference count that is no
/// per-charge state. The ordered charge log
/// ([`Meter::admitted_charges`]) exists only under the `test-support`
/// feature, which only a dev-dependency may enable.
///
/// quire:canonical
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meter {
    counters: Counters,
    /// The handle every charge polls, when the caller gave one.
    cancel: Option<Cancel>,
    #[cfg(feature = "test-support")]
    admitted: Vec<AdmittedCharge>,
    #[cfg(feature = "test-support")]
    charge_log_truncated: bool,
}

/// The fixed-size state a [`Meter`] charges against.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Counters {
    limits: ScalarLimits,
    consumed: [u64; LimitKind::COUNT],
    denial: Option<InjectedDenial>,
    /// Admissions so far at `denial`'s point; zero when no denial is set.
    denied_point_admissions: u64,
    admissions: u64,
}

// The counters own no heap memory: a type with no drop glue holds no `Vec`,
// `Box` or `String`, so a heap-owning field added to it fails the build
// rather than a test.
const _: () = assert!(!core::mem::needs_drop::<Counters>());

impl Meter {
    /// A fresh meter with nothing consumed.
    pub fn new(limits: ScalarLimits) -> Self {
        Self {
            counters: Counters {
                limits,
                consumed: [0; LimitKind::COUNT],
                denial: None,
                denied_point_admissions: 0,
                admissions: 0,
            },
            cancel: None,
            #[cfg(feature = "test-support")]
            admitted: Vec::new(),
            #[cfg(feature = "test-support")]
            charge_log_truncated: false,
        }
    }

    /// This meter, polling `cancel` at every charge. A cancelled
    /// handle denies the charge as an exhausted `work_units` budget does.
    #[must_use]
    pub fn with_cancel(mut self, cancel: Cancel) -> Self {
        self.cancel = Some(cancel);
        self
    }

    /// The installed cancellation handle, borrowed from this meter. Clone it
    /// to share its state while mutably charging this meter or another meter.
    pub fn cancel(&self) -> Option<&Cancel> {
        self.cancel.as_ref()
    }

    /// Add the qualification seam that denies one exact named charge. The
    /// occurrence counts admissions at the denied point from this call on.
    pub fn with_injected_denial(mut self, denial: InjectedDenial) -> Self {
        self.counters.denial = Some(denial);
        self.counters.denied_point_admissions = 0;
        self
    }

    /// The configured limits.
    pub fn limits(&self) -> &ScalarLimits {
        &self.counters.limits
    }

    /// Consumed value of one counter.
    pub fn consumed(&self, kind: LimitKind) -> u64 {
        self.counters.consumed[kind.index()]
    }

    /// How many charges this meter has admitted.
    pub fn admission_count(&self) -> u64 {
        self.counters.admissions
    }

    /// The first 4096 admitted charge records in admission order. Test-only: a
    /// production meter keeps [`Self::admission_count`], not a log.
    #[cfg(feature = "test-support")]
    pub fn admitted_charges(&self) -> &[AdmittedCharge] {
        &self.admitted
    }

    /// Whether an admitted charge fell beyond the diagnostic log's first
    /// 4096 entries.
    #[cfg(feature = "test-support")]
    pub fn charge_log_truncated(&self) -> bool {
        self.charge_log_truncated
    }

    fn incomplete(&self, kind: LimitKind, next: Integer, point: ChargePoint) -> Incomplete {
        Incomplete {
            limit_kind: kind,
            limit: kind.limit(&self.counters.limits),
            consumed: self.consumed(kind),
            next_charge: next,
            charge_point: point,
        }
    }

    /// The denials that do not come from a limit, checked first at every
    /// charge, each recorded as if the `work_units` limit were the work
    /// already consumed, so `limit = consumed = w`: a cancelled handle,
    /// which the operation holding it reports as its
    /// cancellation, and the qualification seam's one exact named charge.
    fn check_injected<A: MeterAmount>(
        &mut self,
        point: ChargePoint,
        work_units: A,
    ) -> Result<(), Incomplete> {
        if self.cancel.as_ref().is_some_and(Cancel::poll) {
            let consumed = self.consumed(LimitKind::WorkUnits);
            return Err(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: consumed,
                consumed,
                next_charge: work_units.integer(),
                charge_point: point,
            });
        }
        match self.counters.denial {
            Some(denial)
                if denial.point == point
                    && self.counters.denied_point_admissions.checked_add(1)
                        == Some(denial.occurrence.get()) =>
            {
                let consumed = self.consumed(LimitKind::WorkUnits);
                self.counters.denial = None;
                Err(Incomplete {
                    limit_kind: LimitKind::WorkUnits,
                    limit: consumed,
                    consumed,
                    next_charge: work_units.integer(),
                    charge_point: point,
                })
            }
            _ => Ok(()),
        }
    }

    /// Atomically admit `charge` or return the first unavailable counter in
    /// `ScalarLimitsV1` field order.
    ///
    /// **`pub`**, so a caller outside this crate can charge its own `meter`
    /// parameter through this method, its one real
    /// cross-crate caller for that purpose.
    pub fn charge(&mut self, charge: Charge) -> Result<(), Incomplete> {
        match charge.storage {
            ChargeStorage::Common(common) => self.charge_fields(
                charge.point,
                common.sizes.into_iter().flatten(),
                common.work_units,
                common.result_units,
            ),
            ChargeStorage::General(general) => self.charge_fields(
                charge.point,
                general.sizes.iter().map(|(kind, amount)| (*kind, amount)),
                &general.work_units,
                &general.result_units,
            ),
        }
    }

    fn charge_fields<A: MeterAmount>(
        &mut self,
        point: ChargePoint,
        requests: impl Iterator<Item = (LimitKind, A)> + Clone,
        work_units: A,
        result_units: A,
    ) -> Result<(), Incomplete> {
        self.check_injected(point, work_units)?;
        // Keep validation and per-admission maxima local until every counter
        // is known to fit. An earlier field wins even when requested later;
        // an equal field keeps its first failing request.
        let mut sizes = [None; LimitKind::COUNT];
        let mut first_fail: Option<(LimitKind, A)> = None;
        #[cfg(feature = "test-support")]
        let mut record = AdmittedCharge::new(point);
        for (kind, amount) in requests {
            let valid = amount
                .units()
                .filter(|amount| *amount <= kind.limit(&self.counters.limits));
            if let Some(amount) = valid {
                let slot = &mut sizes[kind.index()];
                *slot = Some(slot.map_or(amount, |previous: u64| previous.max(amount)));
                #[cfg(feature = "test-support")]
                record.observe(kind, amount);
            } else if first_fail
                .as_ref()
                .is_none_or(|(first_kind, _)| kind.index() < first_kind.index())
            {
                first_fail = Some((kind, amount));
            }
        }
        if let Some((kind, amount)) = first_fail {
            return Err(self.incomplete(kind, amount.integer(), point));
        }
        let cumulative = |kind: LimitKind, amount: A| {
            amount
                .units()
                .and_then(|addition| self.consumed(kind).checked_add(addition))
                .filter(|total| *total <= kind.limit(&self.counters.limits))
                .ok_or_else(|| self.incomplete(kind, amount.integer(), point))
        };
        let work = cumulative(LimitKind::WorkUnits, work_units)?;
        let results = cumulative(LimitKind::ResultUnits, result_units)?;
        // Straight-line writes have no trip count independent of requests. A
        // size request naming a cumulative counter is validated above and
        // then superseded by that counter's own total below.
        // Eight high-water kinds, named rather than numbered. The assertion
        // below fails to compile if a counter is added without a write here.
        const _: () = assert!(LimitKind::COUNT == 8 + 2);
        self.observe_size(
            LimitKind::IntegerBits,
            sizes[LimitKind::IntegerBits.index()],
        );
        self.observe_size(
            LimitKind::DecimalDigits,
            sizes[LimitKind::DecimalDigits.index()],
        );
        self.observe_size(
            LimitKind::ScaleExpansion,
            sizes[LimitKind::ScaleExpansion.index()],
        );
        self.observe_size(
            LimitKind::TextInputBytes,
            sizes[LimitKind::TextInputBytes.index()],
        );
        self.observe_size(
            LimitKind::TextScalars,
            sizes[LimitKind::TextScalars.index()],
        );
        self.observe_size(
            LimitKind::NormalizedScalars,
            sizes[LimitKind::NormalizedScalars.index()],
        );
        self.observe_size(LimitKind::UnitEdges, sizes[LimitKind::UnitEdges.index()]);
        self.observe_size(
            LimitKind::ValueOccurrences,
            sizes[LimitKind::ValueOccurrences.index()],
        );
        self.counters.consumed[LimitKind::WorkUnits.index()] = work;
        self.counters.consumed[LimitKind::ResultUnits.index()] = results;
        self.admit(
            point,
            #[cfg(feature = "test-support")]
            record,
        );
        Ok(())
    }

    fn observe_size(&mut self, kind: LimitKind, amount: Option<u64>) {
        if let Some(amount) = amount {
            let slot = &mut self.counters.consumed[kind.index()];
            *slot = (*slot).max(amount);
        }
    }

    fn admit(
        &mut self,
        point: ChargePoint,
        #[cfg(feature = "test-support")] record: AdmittedCharge,
    ) {
        if self
            .counters
            .denial
            .is_some_and(|denial| denial.point == point)
        {
            self.counters.denied_point_admissions =
                self.counters.denied_point_admissions.saturating_add(1);
        }
        self.counters.admissions = self.counters.admissions.saturating_add(1);
        #[cfg(feature = "test-support")]
        if self.admitted.len() < CHARGE_LOG_CAPACITY {
            self.admitted.push(record);
        } else {
            self.charge_log_truncated = true;
        }
    }

    /// The QSpec-FR-149 `equality.plan` charge: size `value_occurrences` is the
    /// planned pair count; without changing any consumed counter it requires
    /// `pairs + 2` remaining work units and one remaining result unit, then
    /// consumes the plan's own work unit.
    ///
    /// **`pub` (an export gap).** Was `pub(crate)`, same
    /// reason as [`Self::charge`].
    pub fn charge_plan(&mut self, pairs: &Integer) -> Result<(), Incomplete> {
        let point = ChargePoint::EqualityPlan;
        let reservation = pairs.add(&Integer::from(2_u64));
        self.check_injected(point, &ChargeAmount::exact(reservation.clone()))?;
        let kind = LimitKind::ValueOccurrences;
        let size = pairs
            .to_u64()
            .filter(|size| *size <= kind.limit(&self.counters.limits))
            .ok_or_else(|| self.incomplete(kind, pairs.clone(), point))?;
        let remaining = |kind: LimitKind| {
            Integer::from(kind.limit(&self.counters.limits))
                .sub(&Integer::from(self.consumed(kind)))
        };
        if reservation > remaining(LimitKind::WorkUnits) {
            return Err(self.incomplete(LimitKind::WorkUnits, reservation, point));
        }
        if remaining(LimitKind::ResultUnits) < Integer::one() {
            return Err(self.incomplete(LimitKind::ResultUnits, Integer::one(), point));
        }
        let slot = &mut self.counters.consumed[kind.index()];
        *slot = (*slot).max(size);
        let work = &mut self.counters.consumed[LimitKind::WorkUnits.index()];
        *work = work.saturating_add(1);
        self.admit(
            point,
            #[cfg(feature = "test-support")]
            AdmittedCharge {
                value_occurrences: Some(size),
                ..AdmittedCharge::new(point)
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::*;

    fn assert_unchanged(before: &Meter, after: &Meter) {
        assert_eq!(after.admission_count(), before.admission_count());
        for kind in LimitKind::ALL {
            assert_eq!(after.consumed(kind), before.consumed(kind), "{kind:?}");
        }
        #[cfg(feature = "test-support")]
        assert_eq!(after.admitted_charges(), before.admitted_charges());
    }

    /// Trace: FR-359-AC-1
    #[test]
    fn cumulative_work_reaches_max_then_refuses_atomically() {
        let mut meter = Meter::new(unlimited());
        let point = ChargePoint::FunctionCall;
        meter
            .charge(Charge::new(point).work(Integer::from(u64::MAX - 1)))
            .unwrap();
        meter.charge(Charge::new(point)).unwrap();
        assert_eq!(meter.consumed(LimitKind::WorkUnits), u64::MAX);
        let before = meter.clone();
        let denial = meter.charge(Charge::new(point)).unwrap_err();
        assert_eq!(
            (
                denial.limit_kind,
                denial.limit,
                denial.consumed,
                denial.next_charge,
                denial.charge_point
            ),
            (
                LimitKind::WorkUnits,
                u64::MAX,
                u64::MAX,
                Integer::one(),
                point
            )
        );
        assert_unchanged(&before, &meter);
    }

    /// Trace: FR-359-AC-2
    #[test]
    fn cumulative_results_reach_max_then_refuse_atomically() {
        let mut meter = Meter::new(unlimited());
        let point = ChargePoint::FunctionCall;
        meter.charge(Charge::new(point).results(u64::MAX)).unwrap();
        let before = meter.clone();
        let denial = meter.charge(Charge::new(point).results(1)).unwrap_err();
        assert_eq!(
            (
                denial.limit_kind,
                denial.limit,
                denial.consumed,
                denial.next_charge,
                denial.charge_point
            ),
            (
                LimitKind::ResultUnits,
                u64::MAX,
                u64::MAX,
                Integer::one(),
                point
            )
        );
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
        assert_unchanged(&before, &meter);
    }

    /// Trace: FR-359-AC-3, FR-359-AC-4
    #[test]
    fn exact_cumulative_amount_above_max_refuses_without_changes() {
        let excess = Integer::from(u64::MAX).add(&Integer::one());
        let point = ChargePoint::FunctionCall;
        for (charge, kind) in [
            (
                Charge::new(point).work(excess.clone()),
                LimitKind::WorkUnits,
            ),
            (
                Charge::new(point).exact_results(excess.clone()),
                LimitKind::ResultUnits,
            ),
        ] {
            let mut meter = Meter::new(unlimited());
            let before = meter.clone();
            let denial = meter.charge(charge).unwrap_err();
            assert_eq!(
                (
                    denial.limit_kind,
                    denial.limit,
                    denial.consumed,
                    denial.next_charge,
                    denial.charge_point
                ),
                (kind, u64::MAX, 0, excess.clone(), point)
            );
            assert_unchanged(&before, &meter);
        }
    }

    /// Trace: FR-359-AC-5
    #[test]
    fn result_refusal_preserves_prior_size_high_water() {
        let mut limits = unlimited();
        limits.result_units = 1;
        let mut meter = Meter::new(limits);
        let point = ChargePoint::FunctionCall;
        meter
            .charge(
                Charge::new(point)
                    .size(LimitKind::IntegerBits, 8)
                    .results(1),
            )
            .unwrap();
        let before = meter.clone();
        let denial = meter
            .charge(
                Charge::new(point)
                    .size(LimitKind::IntegerBits, 16)
                    .results(1),
            )
            .unwrap_err();
        assert_eq!(
            (
                denial.limit_kind,
                denial.limit,
                denial.consumed,
                denial.next_charge,
                denial.charge_point
            ),
            (LimitKind::ResultUnits, 1, 1, Integer::one(), point)
        );
        assert_eq!(meter.consumed(LimitKind::IntegerBits), 8);
        assert_unchanged(&before, &meter);
    }

    fn tight_limits() -> ScalarLimits {
        ScalarLimits {
            integer_bits: 8,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        }
    }

    /// a charge within every limit is admitted, and the high-water
    /// counter it names records the charged size. Also proves a
    /// cumulative counter (`work_units`) accumulates across charges rather
    /// than only ever recording one charge's amount, every other counter assertion
    /// in this crate is single-charge high-water.
    #[test]
    fn charge_within_limits_is_admitted() {
        let mut meter = Meter::new(tight_limits());
        meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticOperands).size(LimitKind::IntegerBits, 4),
            )
            .expect("4 bits is within the 8-bit limit");
        assert_eq!(meter.consumed(LimitKind::IntegerBits), 4);

        meter
            .charge(Charge::new(ChargePoint::IntegerArithmeticOperands))
            .expect("work_units has room");
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 2);
    }

    #[trace("FR-359-AC-8")]
    #[test]
    fn charge_size_requests_keep_two_inline_and_overflow_in_order() {
        fn require_copy<T: Copy>() {}
        require_copy::<CommonCharge>();
        assert!(!core::mem::needs_drop::<CommonCharge>());
        let empty = Charge::new(ChargePoint::FunctionCall);
        assert!(matches!(empty.storage, ChargeStorage::Common(_)));
        let first = empty.size(LimitKind::IntegerBits, 1);
        let ChargeStorage::Common(common) = first.storage else {
            panic!("first request must stay native");
        };
        assert_eq!(common.sizes, [Some((LimitKind::IntegerBits, 1)), None]);
        let two = first.clone().size(LimitKind::DecimalDigits, 2);
        let ChargeStorage::Common(common) = two.storage else {
            panic!("second request must stay native");
        };
        assert_eq!(
            common.sizes,
            [
                Some((LimitKind::IntegerBits, 1)),
                Some((LimitKind::DecimalDigits, 2))
            ]
        );
        let three = two.size(LimitKind::ScaleExpansion, 3);
        let ChargeStorage::General(general) = &three.storage else {
            panic!("third request must promote");
        };
        assert_eq!(general.sizes.overflow.len(), 1);
        let four = three.size(LimitKind::TextInputBytes, 4);
        let ChargeStorage::General(general) = four.storage else {
            panic!("third request must promote");
        };
        assert_eq!(general.sizes.overflow.len(), 2);
        assert_eq!(
            general
                .sizes
                .iter()
                .map(|(kind, _)| *kind)
                .collect::<Vec<_>>(),
            [
                LimitKind::IntegerBits,
                LimitKind::DecimalDigits,
                LimitKind::ScaleExpansion,
                LimitKind::TextInputBytes,
            ]
        );
    }

    #[trace("FR-359-AC-8", "FR-358-AC-13")]
    #[test]
    fn common_and_promoted_builder_histories_have_identical_observations() {
        let point = ChargePoint::FunctionCall;
        let native = Charge::new(point)
            .size(LimitKind::IntegerBits, 7)
            .size(LimitKind::IntegerBits, 0)
            .results(2)
            .work(Integer::from(3_u64));
        let fitted = Charge::new(point)
            .exact_size(LimitKind::IntegerBits, Integer::from(7_u64))
            .exact_size(LimitKind::IntegerBits, Integer::zero())
            .exact_results(Integer::from(2_u64))
            .work(Integer::from(3_u64));
        assert!(matches!(fitted.storage, ChargeStorage::Common(_)));
        for general in [
            native
                .clone()
                .work(Integer::from(-1_i64))
                .work(Integer::from(3_u64)),
            native
                .clone()
                .exact_results(Integer::from(u64::MAX).add(&Integer::one()))
                .results(2),
        ] {
            assert!(matches!(general.storage, ChargeStorage::General(_)));
            assert_eq!(native, general);
            assert_eq!(general, native);
            assert_eq!(alloc::format!("{native:?}"), alloc::format!("{general:?}"));
            assert_eq!(native.clone(), fitted);
            let mut a = Meter::new(unlimited());
            let mut b = Meter::new(unlimited());
            assert_eq!(a.charge(native.clone()), b.charge(general.clone()));
            assert_eq!(a, b);
            let mut limits = unlimited();
            limits.integer_bits = 6;
            let mut a = Meter::new(limits);
            let mut b = Meter::new(limits);
            let before = a.clone();
            let denial = a.charge(native.clone()).unwrap_err();
            assert_eq!(denial.next_charge, Integer::from(7_u64));
            assert_eq!(denial.limit_kind, LimitKind::IntegerBits);
            assert_eq!(b.charge(general), Err(denial));
            assert_eq!(a, b);
            assert_eq!(a, before);
        }
    }

    #[ix_trace_rs::trace("FR-359-AC-6", "FR-359-AC-8")]
    #[test]
    fn exceptional_exact_sizes_preserve_the_first_failure_and_state() {
        let point = ChargePoint::FunctionCall;
        let over_u64 = Integer::from(u64::MAX).add(&Integer::one());
        let over_i128 = Integer::from(i128::MAX).add(&Integer::one());
        for amount in [Integer::from(-1_i64), over_u64, over_i128] {
            let mut meter = Meter::new(unlimited());
            meter
                .charge(Charge::new(point).size(LimitKind::IntegerBits, 8))
                .unwrap();
            let before = meter.clone();
            let refusal = meter
                .charge(
                    Charge::new(point)
                        .exact_size(LimitKind::ValueOccurrences, amount.clone())
                        .exact_size(LimitKind::IntegerBits, amount.clone())
                        .exact_size(LimitKind::IntegerBits, Integer::from(-2_i64)),
                )
                .unwrap_err();
            assert_eq!(refusal.limit_kind, LimitKind::IntegerBits);
            assert_eq!(refusal.limit, u64::MAX);
            assert_eq!(refusal.consumed, 8);
            assert_eq!(refusal.next_charge, amount);
            assert_eq!(refusal.charge_point, point);
            assert_unchanged(&before, &meter);
        }
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn duplicate_size_requests_preserve_first_refusal_and_are_atomic() {
        let mut meter = Meter::new(tight_limits());
        let before = meter.clone();
        let refusal = meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticOperands)
                    .size(LimitKind::IntegerBits, 9)
                    .size(LimitKind::IntegerBits, 7)
                    .size(LimitKind::DecimalDigits, 2),
            )
            .expect_err("the first duplicate request exceeds the integer limit");

        assert_eq!(refusal.limit_kind, LimitKind::IntegerBits);
        assert_eq!(refusal.next_charge, Integer::from(9_u64));
        assert_unchanged(&before, &meter);
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn duplicate_sizes_coalesce_by_max_and_retain_explicit_zero() {
        let mut meter = Meter::new(unlimited());
        meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticOperands)
                    .size(LimitKind::IntegerBits, 0)
                    .size(LimitKind::IntegerBits, 5)
                    .size(LimitKind::IntegerBits, 3)
                    .size(LimitKind::DecimalDigits, 0),
            )
            .expect("all size requests fit");

        assert_eq!(meter.consumed(LimitKind::IntegerBits), 5);
        assert_eq!(meter.consumed(LimitKind::DecimalDigits), 0);
        #[cfg(feature = "test-support")]
        {
            let admitted = meter.admitted_charges();
            assert_eq!(admitted[0].integer_bits, Some(5));
            assert_eq!(admitted[0].decimal_digits, Some(0));
        }
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn first_inserted_failing_size_precedes_later_same_kind_maximum() {
        for (first, second) in [(7, 9), (9, 7)] {
            let mut limits = unlimited();
            limits.integer_bits = 5;
            let mut meter = Meter::new(limits);
            let before = meter.clone();
            let denied = meter
                .charge(
                    Charge::new(ChargePoint::FunctionCall)
                        .size(LimitKind::IntegerBits, first)
                        .size(LimitKind::IntegerBits, second),
                )
                .expect_err("first inserted failing amount must be reported");
            assert_eq!(
                denied,
                Incomplete {
                    limit_kind: LimitKind::IntegerBits,
                    limit: 5,
                    consumed: 0,
                    next_charge: Integer::from(first),
                    charge_point: ChargePoint::FunctionCall,
                }
            );
            assert_unchanged(&before, &meter);
            #[cfg(feature = "test-support")]
            assert_eq!(meter.charge_log_truncated(), before.charge_log_truncated());
        }
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn size_shortages_use_field_order_across_attachment_orders() {
        for reverse in [false, true] {
            let mut limits = unlimited();
            limits.integer_bits = 5;
            limits.value_occurrences = 1;
            let mut charge = Charge::new(ChargePoint::FunctionCall);
            if reverse {
                charge = charge
                    .size(LimitKind::IntegerBits, 7)
                    .size(LimitKind::ValueOccurrences, 2);
            } else {
                charge = charge
                    .size(LimitKind::ValueOccurrences, 2)
                    .size(LimitKind::IntegerBits, 7);
            }
            let mut meter = Meter::new(limits);
            let before = meter.clone();
            let denied = meter.charge(charge).expect_err("both kinds are short");
            assert_eq!(denied.limit_kind, LimitKind::IntegerBits);
            assert_eq!(denied.next_charge, Integer::from(7_u64));
            assert_unchanged(&before, &meter);
        }
    }

    /// a charge past a counter's limit is denied atomically: the
    /// counter is unchanged and the exact limit/consumed/next-charge triple
    /// is reported.
    #[test]
    fn charge_past_limit_is_denied_atomically() {
        let mut meter = Meter::new(tight_limits());
        let denial = meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticOperands).size(LimitKind::IntegerBits, 9),
            )
            .expect_err("9 bits exceeds the 8-bit limit");
        assert_eq!(denial.limit_kind, LimitKind::IntegerBits);
        assert_eq!(denial.limit, 8);
        assert_eq!(denial.consumed, 0);
        assert_eq!(meter.consumed(LimitKind::IntegerBits), 0);
    }

    fn unlimited() -> ScalarLimits {
        ScalarLimits {
            integer_bits: u64::MAX,
            ..tight_limits()
        }
    }

    /// Trace: FR-358-AC-6
    /// A production meter's counters own no heap memory, however many
    /// charges it admits; the admission count still grows with every charge.
    /// Built only without `test-support` (`cargo test -p quire-exact`, which
    /// `make ci` runs): with it, the meter keeps its ordered charge log,
    /// which allocates.
    #[cfg(not(feature = "test-support"))]
    #[test]
    fn production_meter_size_is_constant_as_charges_grow() {
        assert!(
            !core::mem::needs_drop::<Counters>(),
            "a production Meter's counters must hold no field that grows with its charges"
        );
        let mut meter = Meter::new(unlimited());
        let mut admitted = 0_u64;
        for target in [1_u64, 1_000, 100_000] {
            while admitted < target {
                meter
                    .charge(Charge::new(ChargePoint::FunctionCall))
                    .expect("an unlimited meter admits every charge");
                admitted += 1;
            }
            assert_eq!(meter.admission_count(), target);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), target);
        }
    }

    /// Trace: FR-359-AC-8
    ///
    /// A cancelled handle and a pending injected denial both precede size
    /// validation: an oversized request is reported as the cancellation
    /// (`WorkUnits`, `limit = consumed`), never as its own size refusal, and
    /// the refused charge changes nothing.
    #[test]
    fn cancellation_and_injection_precede_size_refusal() {
        let cancel = Cancel::new();
        let mut cancelled = Meter::new(tight_limits()).with_cancel(cancel.clone());
        let before = cancelled.clone();
        cancel.cancel(crate::CancelCause::Deadline);
        let denial = cancelled
            .charge(Charge::new(ChargePoint::FunctionCall).size(LimitKind::IntegerBits, u64::MAX))
            .expect_err("a cancelled handle denies before size validation");
        assert_eq!(denial.limit_kind, LimitKind::WorkUnits);
        assert_eq!(denial.limit, denial.consumed);
        assert_eq!(cancelled.consumed(LimitKind::IntegerBits), 0);
        assert_unchanged(&before, &cancelled);

        let mut injected = Meter::new(tight_limits()).with_injected_denial(InjectedDenial {
            point: ChargePoint::FunctionCall,
            occurrence: NonZeroU64::new(1).expect("one is nonzero"),
        });
        let before = injected.clone();
        let denial = injected
            .charge(Charge::new(ChargePoint::FunctionCall).size(LimitKind::IntegerBits, u64::MAX))
            .expect_err("an injected denial precedes size validation");
        assert_eq!(denial.limit_kind, LimitKind::WorkUnits);
        assert_eq!(denial.limit, denial.consumed);
        assert_unchanged(&before, &injected);
    }

    /// A meter polls its handle at every charge. A cancelled handle
    /// denies the next charge without changing any counter, and the handle
    /// records the trip.
    #[test]
    fn a_cancelled_handle_denies_the_next_charge_and_records_the_trip() {
        let cancel = Cancel::new();
        let mut meter = Meter::new(unlimited()).with_cancel(cancel.clone());
        meter
            .charge(Charge::new(ChargePoint::FunctionCall))
            .expect("a live handle admits the charge");
        cancel.cancel(crate::CancelCause::Deadline);
        assert_eq!(cancel.tripped(), None);
        let denial = meter
            .charge(Charge::new(ChargePoint::FunctionCall))
            .expect_err("a cancelled handle denies the charge");
        assert_eq!(denial.limit_kind, LimitKind::WorkUnits);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
        assert_eq!(cancel.tripped(), Some(crate::CancelCause::Deadline));
    }

    /// The injected denial counts only the denied point's
    /// admissions, without the per-point occurrence table it replaced. The
    /// third `function.call` is denied, whatever other points were admitted
    /// in between; a denial set after earlier charges counts from then on.
    #[test]
    fn injected_denial_counts_only_its_own_point() {
        let denial = InjectedDenial {
            point: ChargePoint::FunctionCall,
            occurrence: NonZeroU64::new(3).expect("three is nonzero"),
        };
        let mut meter = Meter::new(unlimited()).with_injected_denial(denial);
        for _ in 0..2 {
            meter
                .charge(Charge::new(ChargePoint::FunctionCall))
                .expect("occurrences 1 and 2 are admitted");
            meter
                .charge(Charge::new(ChargePoint::CollectionVisit))
                .expect("another point never counts toward the denial");
        }
        let denied = meter
            .charge(Charge::new(ChargePoint::FunctionCall))
            .expect_err("occurrence 3 is denied");
        assert_eq!(denied.charge_point, ChargePoint::FunctionCall);
        assert_eq!(meter.admission_count(), 4);

        let mut late = Meter::new(unlimited());
        late.charge(Charge::new(ChargePoint::FunctionCall))
            .expect("no denial is set yet");
        let mut late = late.with_injected_denial(InjectedDenial {
            point: ChargePoint::FunctionCall,
            occurrence: NonZeroU64::new(1).expect("one is nonzero"),
        });
        late.charge(Charge::new(ChargePoint::FunctionCall))
            .expect_err("the first occurrence after the denial is set is denied");
    }

    /// Trace: FR-358-AC-1
    #[test]
    fn injected_denial_spends_only_on_its_named_admitted_position() {
        let mut meter = Meter::new(tight_limits()).with_injected_denial(InjectedDenial {
            point: ChargePoint::FunctionCall,
            occurrence: NonZeroU64::new(2).expect("two is nonzero"),
        });
        let before_ordinary_refusal = meter.clone();
        let ordinary = meter
            .charge(Charge::new(ChargePoint::FunctionCall).size(LimitKind::IntegerBits, 9))
            .expect_err("nine bits exceeds the ordinary limit");
        assert_eq!(ordinary.limit_kind, LimitKind::IntegerBits);
        assert_eq!(meter, before_ordinary_refusal);

        meter
            .charge(Charge::new(ChargePoint::FunctionCall).size(LimitKind::IntegerBits, 4))
            .expect("first admitted named occurrence");
        meter
            .charge(Charge::new(ChargePoint::CollectionVisit))
            .expect("an unrelated admission does not advance the named occurrence");
        let before_injected_refusal = meter.clone();
        let injected = meter
            .charge(Charge::new(ChargePoint::FunctionCall))
            .expect_err("second named occurrence is injected denial");
        assert_eq!(injected.charge_point, ChargePoint::FunctionCall);
        assert_eq!(injected.limit_kind, LimitKind::WorkUnits);
        assert_eq!(
            meter.admission_count(),
            before_injected_refusal.admission_count()
        );
        for kind in LimitKind::ALL {
            assert_eq!(meter.consumed(kind), before_injected_refusal.consumed(kind));
        }
        #[cfg(feature = "test-support")]
        assert_eq!(
            meter.admitted_charges(),
            before_injected_refusal.admitted_charges()
        );

        meter
            .charge(Charge::new(ChargePoint::FunctionCall))
            .expect("the injected denial is spent after one refusal");
        assert_eq!(meter.admission_count(), 3);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
        assert_eq!(meter.consumed(LimitKind::IntegerBits), 4);
        #[cfg(feature = "test-support")]
        assert_eq!(
            meter
                .admitted_charges()
                .iter()
                .map(|charge| charge.point)
                .collect::<alloc::vec::Vec<_>>(),
            [
                ChargePoint::FunctionCall,
                ChargePoint::CollectionVisit,
                ChargePoint::FunctionCall,
            ]
        );
    }

    /// Trace: FR-358-AC-2
    #[test]
    fn injected_plan_denial_spends_after_one_refusal() {
        let mut meter = Meter::new(unlimited()).with_injected_denial(InjectedDenial {
            point: ChargePoint::EqualityPlan,
            occurrence: NonZeroU64::new(1).expect("one is nonzero"),
        });
        let before = meter.clone();
        let denied = meter
            .charge_plan(&Integer::from(2_u64))
            .expect_err("first plan charge is injected denial");
        assert_eq!(denied.charge_point, ChargePoint::EqualityPlan);
        assert_eq!(denied.limit_kind, LimitKind::WorkUnits);
        assert_eq!(meter.admission_count(), before.admission_count());
        for kind in LimitKind::ALL {
            assert_eq!(meter.consumed(kind), before.consumed(kind));
        }
        #[cfg(feature = "test-support")]
        assert_eq!(meter.admitted_charges(), before.admitted_charges());

        meter
            .charge_plan(&Integer::from(2_u64))
            .expect("the next plan charge is admitted");
        assert_eq!(meter.admission_count(), 1);
        assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
        #[cfg(feature = "test-support")]
        assert_eq!(
            meter
                .admitted_charges()
                .iter()
                .map(|charge| charge.point)
                .collect::<alloc::vec::Vec<_>>(),
            [ChargePoint::EqualityPlan]
        );
    }

    /// Trace: FR-358-AC-12
    #[test]
    fn injected_record_uses_precharge_work_and_plan_reservation() {
        for work_limit in [10, 100] {
            let limits = ScalarLimits {
                work_units: work_limit,
                ..unlimited()
            };
            let mut meter = Meter::new(limits);
            meter
                .charge(Charge::new(ChargePoint::CollectionVisit).work(Integer::from(2_u64)))
                .expect("two work units are admitted before injection");
            let mut meter = meter.with_injected_denial(InjectedDenial {
                point: ChargePoint::FunctionCall,
                occurrence: NonZeroU64::new(1).expect("one is nonzero"),
            });
            let before = meter.clone();
            assert_eq!(
                meter
                    .charge(Charge::new(ChargePoint::FunctionCall).work(Integer::from(3_u64)))
                    .expect_err("selected charge is denied"),
                Incomplete {
                    limit_kind: LimitKind::WorkUnits,
                    limit: 2,
                    consumed: 2,
                    next_charge: Integer::from(3_u64),
                    charge_point: ChargePoint::FunctionCall,
                }
            );
            assert_unchanged(&before, &meter);

            let mut plan_meter = Meter::new(limits);
            plan_meter
                .charge(Charge::new(ChargePoint::CollectionVisit).work(Integer::from(2_u64)))
                .expect("two work units are admitted before the plan");
            let mut plan_meter = plan_meter.with_injected_denial(InjectedDenial {
                point: ChargePoint::EqualityPlan,
                occurrence: NonZeroU64::new(1).expect("one is nonzero"),
            });
            let before = plan_meter.clone();
            assert_eq!(
                plan_meter
                    .charge_plan(&Integer::from(2_u64))
                    .expect_err("selected plan is denied"),
                Incomplete {
                    limit_kind: LimitKind::WorkUnits,
                    limit: 2,
                    consumed: 2,
                    next_charge: Integer::from(4_u64),
                    charge_point: ChargePoint::EqualityPlan,
                }
            );
            assert_unchanged(&before, &plan_meter);
            plan_meter
                .charge_plan(&Integer::from(2_u64))
                .expect("the spent injection permits a valid plan");
            assert_eq!(plan_meter.consumed(LimitKind::WorkUnits), 3);
            assert_eq!(plan_meter.admission_count(), 2);
        }
    }

    /// Trace: FR-358-AC-13, FR-359-AC-8
    #[test]
    fn injected_denial_precedes_ordinary_size_and_plan_shortages() {
        let size_limits = ScalarLimits {
            integer_bits: 8,
            work_units: 10,
            ..unlimited()
        };
        let size_charge = || {
            Charge::new(ChargePoint::FunctionCall)
                .size(LimitKind::IntegerBits, 9)
                .work(Integer::from(3_u64))
        };
        let mut ordinary = Meter::new(size_limits);
        assert_eq!(
            ordinary
                .charge(size_charge())
                .expect_err("size exceeds eight"),
            Incomplete {
                limit_kind: LimitKind::IntegerBits,
                limit: 8,
                consumed: 0,
                next_charge: Integer::from(9_u64),
                charge_point: ChargePoint::FunctionCall,
            }
        );
        let mut injected = Meter::new(size_limits).with_injected_denial(InjectedDenial {
            point: ChargePoint::FunctionCall,
            occurrence: NonZeroU64::new(1).expect("one is nonzero"),
        });
        let before = injected.clone();
        assert_eq!(
            injected
                .charge(size_charge())
                .expect_err("injection precedes size"),
            Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: 0,
                consumed: 0,
                next_charge: Integer::from(3_u64),
                charge_point: ChargePoint::FunctionCall,
            }
        );
        assert_unchanged(&before, &injected);

        let plan_limits = ScalarLimits {
            work_units: 3,
            ..unlimited()
        };
        let mut plan_meter = Meter::new(plan_limits);
        plan_meter
            .charge(Charge::new(ChargePoint::CollectionVisit).work(Integer::from(2_u64)))
            .expect("two work units are admitted");
        let mut ordinary = plan_meter.clone();
        assert_eq!(
            ordinary
                .charge_plan(&Integer::from(2_u64))
                .expect_err("four-unit reservation exceeds one remaining unit"),
            Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: 3,
                consumed: 2,
                next_charge: Integer::from(4_u64),
                charge_point: ChargePoint::EqualityPlan,
            }
        );
        let mut injected = plan_meter.with_injected_denial(InjectedDenial {
            point: ChargePoint::EqualityPlan,
            occurrence: NonZeroU64::new(1).expect("one is nonzero"),
        });
        let before = injected.clone();
        assert_eq!(
            injected
                .charge_plan(&Integer::from(2_u64))
                .expect_err("injection precedes the reservation shortage"),
            Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: 2,
                consumed: 2,
                next_charge: Integer::from(4_u64),
                charge_point: ChargePoint::EqualityPlan,
            }
        );
        assert_unchanged(&before, &injected);

        let cancel = Cancel::new();
        let mut cancelled = Meter::new(size_limits)
            .with_cancel(cancel.clone())
            .with_injected_denial(InjectedDenial {
                point: ChargePoint::FunctionCall,
                occurrence: NonZeroU64::new(1).expect("one is nonzero"),
            });
        cancel.cancel(crate::CancelCause::Deadline);
        let before = cancelled.clone();
        assert_eq!(
            cancelled
                .charge(size_charge())
                .expect_err("cancellation precedes injection"),
            Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: 0,
                consumed: 0,
                next_charge: Integer::from(3_u64),
                charge_point: ChargePoint::FunctionCall,
            }
        );
        assert_eq!(cancel.tripped(), Some(crate::CancelCause::Deadline));
        assert_unchanged(&before, &cancelled);

        let mut resumed = cancelled.with_cancel(Cancel::new());
        let before_injection = resumed.clone();
        assert_eq!(
            resumed
                .charge(size_charge())
                .expect_err("cancellation left the selected injection pending"),
            Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: 0,
                consumed: 0,
                next_charge: Integer::from(3_u64),
                charge_point: ChargePoint::FunctionCall,
            }
        );
        assert_unchanged(&before_injection, &resumed);
        resumed
            .charge(Charge::new(ChargePoint::FunctionCall))
            .expect("the spent injection allows an ordinary valid charge");
        assert_eq!(resumed.admission_count(), 1);
        assert_eq!(resumed.consumed(LimitKind::WorkUnits), 1);
    }

    /// Trace: FR-358-AC-3
    #[test]
    fn injected_denial_occurrence_requires_nonzero() {
        assert_eq!(NonZeroU64::new(0), None);
        let denial = InjectedDenial {
            point: ChargePoint::FunctionCall,
            occurrence: NonZeroU64::new(1).expect("one is nonzero"),
        };
        let occurrence: NonZeroU64 = denial.occurrence;
        assert_eq!(occurrence.get(), 1);
    }

    /// Trace: FR-358-AC-4, FR-358-AC-5
    #[cfg(feature = "test-support")]
    #[test]
    fn charge_log_retains_only_first_4096_admissions() {
        let mut meter = Meter::new(unlimited());
        let expected: Vec<_> = (0..CHARGE_LOG_CAPACITY)
            .map(|index| {
                if index % 2 == 0 {
                    ChargePoint::FunctionCall
                } else {
                    ChargePoint::CollectionVisit
                }
            })
            .collect();
        for point in &expected {
            meter
                .charge(Charge::new(*point))
                .expect("unlimited meter admits each charge");
        }
        assert_eq!(
            meter
                .admitted_charges()
                .iter()
                .map(|charge| charge.point)
                .collect::<alloc::vec::Vec<_>>(),
            expected
        );
        assert!(!meter.charge_log_truncated());
        assert_eq!(meter.admission_count(), 4096);

        meter
            .charge(Charge::new(ChargePoint::EqualityPlan))
            .expect("the charge beyond the log cap is still admitted");
        assert_eq!(
            meter
                .admitted_charges()
                .iter()
                .map(|charge| charge.point)
                .collect::<alloc::vec::Vec<_>>(),
            expected
        );
        assert!(meter.charge_log_truncated());
        assert_eq!(meter.admission_count(), 4097);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 4097);
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn size_request_naming_a_cumulative_counter_is_validated_in_field_order() {
        let mut limits = unlimited();
        limits.work_units = 4;
        limits.result_units = 4;
        let mut meter = Meter::new(limits);
        let before = meter.clone();
        let refused = meter
            .charge(
                Charge::new(ChargePoint::FunctionCall)
                    .results(9)
                    .size(LimitKind::ResultUnits, 6)
                    .size(LimitKind::WorkUnits, 5),
            )
            .expect_err("the work-units size request exceeds its limit");
        assert_eq!(refused.limit_kind, LimitKind::WorkUnits);
        assert_eq!(refused.next_charge, Integer::from(5_u64));
        assert_unchanged(&before, &meter);
        meter
            .charge(
                Charge::new(ChargePoint::FunctionCall)
                    .size(LimitKind::WorkUnits, 3)
                    .size(LimitKind::ResultUnits, 3),
            )
            .expect("both size requests are within their limits");
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn request_counts_from_zero_to_many_gain_no_denial() {
        let mut meter = Meter::new(unlimited());
        let mut expected_bits = 0_u64;
        for (admitted, count) in [0_u64, 1, 2, 3, 64, 1000].into_iter().enumerate() {
            let mut charge = Charge::new(ChargePoint::FunctionCall);
            for amount in 1..=count {
                charge = charge.size(LimitKind::IntegerBits, amount);
            }
            meter
                .charge(charge)
                .expect("no request count is itself a denial");
            expected_bits = expected_bits.max(count);
            assert_eq!(meter.consumed(LimitKind::IntegerBits), expected_bits);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), admitted as u64 + 1);
            assert_eq!(meter.admission_count(), admitted as u64 + 1);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
            assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 0);
        }
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn reordered_and_repeated_requests_keep_the_high_water_maximum() {
        for amounts in [[3_u64, 9, 5], [9, 3, 5], [5, 9, 3], [0, 0, 0]] {
            let mut meter = Meter::new(unlimited());
            let charge = amounts.into_iter().fold(
                Charge::new(ChargePoint::FunctionCall).results(2),
                |charge, amount| {
                    charge
                        .size(LimitKind::IntegerBits, amount)
                        .size(LimitKind::TextScalars, 4)
                },
            );
            meter.charge(charge).expect("every size is within limits");
            let expected = amounts.into_iter().max().expect("three amounts");
            assert_eq!(meter.consumed(LimitKind::IntegerBits), expected);
            assert_eq!(meter.consumed(LimitKind::TextScalars), 4);
            assert_eq!(meter.consumed(LimitKind::DecimalDigits), 0);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 2);
            meter
                .charge(Charge::new(ChargePoint::FunctionCall).size(LimitKind::IntegerBits, 0))
                .expect("a zero later request is admitted");
            assert_eq!(meter.consumed(LimitKind::IntegerBits), expected);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), 2);
        }
    }

    /// Trace: FR-359-AC-8
    #[test]
    fn ordinary_refusal_leaves_a_pending_injected_occurrence_unspent() {
        let mut limits = unlimited();
        limits.integer_bits = 5;
        let point = ChargePoint::FunctionCall;
        let mut meter = Meter::new(limits).with_injected_denial(InjectedDenial {
            point,
            occurrence: NonZeroU64::new(3).expect("three is nonzero"),
        });
        meter
            .charge(Charge::new(point))
            .expect("occurrence 1 is admitted");
        let before = meter.clone();
        let refused = meter
            .charge(
                Charge::new(point)
                    .size(LimitKind::IntegerBits, 7)
                    .size(LimitKind::IntegerBits, 9),
            )
            .expect_err("seven bits exceeds five");
        assert_eq!(refused.limit_kind, LimitKind::IntegerBits);
        assert_eq!(refused.next_charge, Integer::from(7_u64));
        assert_unchanged(&before, &meter);
        #[cfg(feature = "test-support")]
        assert_eq!(meter.charge_log_truncated(), before.charge_log_truncated());
        meter
            .charge(Charge::new(point))
            .expect("the refusal did not advance the occurrence count");
        let injected = meter
            .charge(Charge::new(point))
            .expect_err("the third admission position is denied");
        assert_eq!(injected.limit_kind, LimitKind::WorkUnits);
        assert_eq!(injected.limit, injected.consumed);
        assert_eq!(meter.admission_count(), 2);
    }

    /// The independent expectation for one admission: sizes as
    /// `(kind, amount)` requests in attachment order, folded to the per-kind
    /// maximum by plain comparison.
    #[cfg(feature = "test-support")]
    fn expected_record(point: ChargePoint, requests: &[(LimitKind, u64)]) -> AdmittedCharge {
        let maximum = |kind: LimitKind| {
            let mut found: Option<u64> = None;
            for (requested, amount) in requests {
                if *requested == kind {
                    found = Some(match found {
                        Some(previous) if previous >= *amount => previous,
                        _ => *amount,
                    });
                }
            }
            found
        };
        AdmittedCharge {
            point,
            integer_bits: maximum(LimitKind::IntegerBits),
            decimal_digits: maximum(LimitKind::DecimalDigits),
            scale_expansion: maximum(LimitKind::ScaleExpansion),
            text_input_bytes: maximum(LimitKind::TextInputBytes),
            text_scalars: maximum(LimitKind::TextScalars),
            normalized_scalars: maximum(LimitKind::NormalizedScalars),
            unit_edges: maximum(LimitKind::UnitEdges),
            value_occurrences: maximum(LimitKind::ValueOccurrences),
        }
    }

    /// A charge point, its size requests and its result units.
    #[cfg(feature = "test-support")]
    type ScheduledCharge = (ChargePoint, &'static [(LimitKind, u64)], u64);

    #[cfg(feature = "test-support")]
    fn charge_with(point: ChargePoint, requests: &[(LimitKind, u64)]) -> Charge {
        requests
            .iter()
            .fold(Charge::new(point), |charge, (kind, amount)| {
                charge.size(*kind, *amount)
            })
    }

    /// Trace: FR-359-AC-12
    #[cfg(feature = "test-support")]
    #[test]
    fn tc_931_admitted_log_matches_model_ordered_prefix() {
        let schedule: [ScheduledCharge; 3] = [
            (
                ChargePoint::IntegerArithmeticOperands,
                &[
                    (LimitKind::IntegerBits, 6),
                    (LimitKind::ValueOccurrences, 2),
                ],
                0,
            ),
            (
                ChargePoint::IntegerArithmeticArithmetic,
                &[(LimitKind::IntegerBits, 7)],
                0,
            ),
            (ChargePoint::IntegerArithmeticResultRetain, &[], 1),
        ];
        // A result limit of zero refuses the third charge; one admits it.
        for (result_limit, admitted) in [(0_u64, 2_usize), (1, 3)] {
            let mut meter = Meter::new(ScalarLimits {
                result_units: result_limit,
                ..unlimited()
            });
            for (position, (point, requests, results)) in schedule.iter().enumerate() {
                let result = meter.charge(charge_with(*point, requests).results(*results));
                assert_eq!(result.is_ok(), position < admitted);
            }
            let expected: Vec<AdmittedCharge> = schedule
                .iter()
                .take(admitted)
                .map(|(point, requests, _)| expected_record(*point, requests))
                .collect();
            assert_eq!(meter.admitted_charges(), expected.as_slice());
            assert_eq!(meter.admission_count(), admitted as u64);
        }
        let mut meter = Meter::new(ScalarLimits {
            integer_bits: 6,
            ..unlimited()
        });
        meter
            .charge(charge_with(schedule[0].0, schedule[0].1))
            .expect("six bits fits");
        let log_before = meter.admitted_charges().to_vec();
        meter
            .charge(charge_with(schedule[1].0, schedule[1].1))
            .expect_err("seven bits exceeds six");
        assert_eq!(meter.admitted_charges(), log_before.as_slice());
        assert!(!meter.charge_log_truncated());
    }

    /// Trace: FR-359-AC-12
    #[cfg(feature = "test-support")]
    #[test]
    fn tc_931_admitted_log_matches_model_same_kind() {
        let point = ChargePoint::FunctionCall;
        let cases: [&[(LimitKind, u64)]; 5] = [
            &[(LimitKind::IntegerBits, 3), (LimitKind::IntegerBits, 9)],
            &[(LimitKind::IntegerBits, 9), (LimitKind::IntegerBits, 3)],
            &[(LimitKind::TextScalars, 0), (LimitKind::TextScalars, 0)],
            &[
                (LimitKind::UnitEdges, 4),
                (LimitKind::ValueOccurrences, 0),
                (LimitKind::UnitEdges, 2),
                (LimitKind::ValueOccurrences, 8),
            ],
            &[(LimitKind::DecimalDigits, 5)],
        ];
        let mut meter = Meter::new(ScalarLimits {
            integer_bits: 20,
            ..unlimited()
        });
        let mut expected = Vec::new();
        for requests in cases {
            meter
                .charge(charge_with(point, requests))
                .expect("every request fits");
            expected.push(expected_record(point, requests));
            assert_eq!(meter.admitted_charges(), expected.as_slice());
        }
        assert_eq!(expected[0].integer_bits, Some(9));
        assert_eq!(expected[2].text_scalars, Some(0));
        assert_eq!(expected[2].integer_bits, None);

        let mut tight = Meter::new(ScalarLimits {
            integer_bits: 5,
            ..unlimited()
        });
        let refused = tight
            .charge(charge_with(
                point,
                &[(LimitKind::IntegerBits, 7), (LimitKind::IntegerBits, 9)],
            ))
            .expect_err("both requests exceed five");
        assert_eq!(refused.next_charge, Integer::from(7_u64));
        assert!(tight.admitted_charges().is_empty());
    }
}
