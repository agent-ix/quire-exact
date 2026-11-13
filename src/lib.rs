// SPDX-License-Identifier: AGPL-3.0-or-later
//! `quire-exact`: the exact-value kernel, a `no_std` + `alloc` leaf that
//! depends on no other crate in the quire ecosystem.
//!
//! The crate holds checked structural identity ([`NodeKey`] and value
//! identities such as [`EffectiveId`]), provenance ([`Location`]), kernel
//! outcomes and refusals ([`Outcome`], [`Refusal`]), bounds and accounting
//! ([`Meter`], [`BoundedInteger`], [`CardinalityBound`]), and the exact
//! semantic value kernel ([`Value`]/[`ValueType`] and every value-family
//! module it composes: `numeric`, `rational`, `decimal`, `text`, `ieee`,
//! `division`, `comparison`, `equality`, `key`, `quantity`, `reference`).
//! Every submodule is private; the public surface is exactly the curated
//! `pub use` facade on this page, so the module names above are plain text,
//! not links.
//!
//! It depends on no wire format, hashing or canonicalization crate. Its
//! identity constructors carry caller-admitted structural components and
//! never derive identities from digests.
//!
//! [`Value`]/[`ValueType`]: `ValueType::admits` never pairs
//! `ValueType::Population(Option<u64>)` with `Value::Population(PopulationId)`
//! (FR-089-AC-6). A population identity resolves to its declared maximum only
//! in a layer that holds the correspondence, which this leaf does not, so
//! kernel `admits` refuses every population pair outright. The
//! `ValueType::Enum` shape, by contrast, carries its variant set inline, so it
//! needs no declaration lookup at all.
//!
//! Some capabilities are deliberately left out of the kernel and documented
//! where they occur:
//! - the `key` and `equality` modules: `Value::Population` has no key and
//!   compares under neither; a population binding is a direct operand of
//!   `allInstances`/`lookup` only, never an equality or key operand. A
//!   same-enum check is still available: `ValueType::Enum(EnumShape)`'s
//!   admission already guarantees both operands share one enum's variant set
//!   before either module runs.
//! - [`Quantity`]: no cross-unit arithmetic, comparison or equality; only
//!   same-unit operations.
//! - the `equality` module: the top-level text/enum/quantity schedule
//!   selection and the closed equality-conversion table are dropped along
//!   with the declaration registry and unit graph they need.
//!
//! Value-semantics requirements (decimals, text and enumerations, quantities
//! and units, integer division, IEEE profiles) are owned by
//! `agent-ix/quire-specification`. A test carries a `QSpec-` tagged AC only
//! where its own assertions, not merely its module doc's citation, tell the
//! AC's claim from a wrong implementation. The requirements this repository
//! owns are in `spec/`.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
// Unit tests use `std` (e.g. `HashSet` hash-consistency checks); the library
// itself links only `core` and `alloc`.
#[cfg(test)]
#[macro_use]
extern crate std;

/// The semantic epoch of the kernel's operations, bumped by hand.
///
/// `Cargo.toml`'s version is `0.1.0` on every commit of a `branch = main`
/// dependency, so it cannot tell a consumer that behaviour changed; this
/// constant can. A consumer that records the value it measured against treats
/// a record whose value differs from this one as stale.
///
/// Whoever changes the kernel bumps it by at least one, in the same change,
/// when the change can alter, for some input:
/// - the result of a public operation: a returned value, a flag, a refusal or
///   its cause, or an outcome kind;
/// - a charge: its point, order, limit kind or amount;
/// - the cost or size of the verification problem a public operation presents
///   to a Kani harness (wall time or peak memory for harnesses that run
///   `Value`, `Integer`, `Meter`, `build_typed_prestate` or the collection
///   types), including a semantics-preserving change of a carrier
///   representation.
///
/// No bump is needed for a rename, a comment, a new operation or type that no
/// existing operation uses, or a refactor with no effect on results or
/// verification cost. The value never decreases.
///
/// It is a plain integer literal, not a digest, so it depends on the source
/// only: no build time, git revision or feature set changes it. The golden
/// test in `tests/ir1532_kernel_version.rs` catches an unintended outcome or
/// charge change inside its grid; it does not catch a change outside the grid,
/// a change to verification cost, or re-recorded expectations without a bump,
/// which review has to catch.
pub const KERNEL_VERSION: u32 = 1;

mod accounting;
mod cancel;
mod collection;
mod comparison;
mod decimal;
mod decimal_membership;
mod division;
mod equality;
mod identity;
mod identity_copy;
mod ieee;
mod integer;
mod key;
mod location;
mod native_decimal;
mod native_enum;
#[cfg(kani)]
#[path = "native_enum/kani_proofs.rs"]
mod native_enum_kani_proofs;
mod native_integer;
mod native_text;
#[cfg(kani)]
#[path = "native_text/kani_proofs.rs"]
mod native_text_kani_proofs;
mod node;
mod numeric;
mod numeric_membership;
mod numeric_type_equality;
mod outcome;
mod quantity;
mod rational;
mod reference;
mod reservation;
mod text;
mod value;

// Every submodule above is private and its public surface is exposed only
// through this curated facade (private `mod`s behind selective `pub use`
// re-exports) rather than `pub mod` wholesale. `key::compare_keys` is `pub`
// so a caller that groups adjacent equal elements uses the kernel's own key,
// and grouping and canonical sort cannot drift apart. The decimal, rational,
// numeric and equality helpers exported below (`rational::
// divided_by_power_of_ten`/`divided_by_power_of_two`, `decimal::
// DecimalRepresentation::to_rational`, `decimal::compare_shifted`,
// `decimal::power_of_ten_bits`/`sbits`/`sdigits`, `decimal::DecimalType::
// placement` with `Placement`/`Placed`/`Admitted`,
// `numeric::rational_arithmetic_bits`, `decimal::DecimalLoss::exact`/
// `exact_denominator` and `equality::plan_equality`) are each unmetered exact
// arithmetic or comparison, the same discipline `Integer`'s own arithmetic
// carries, so a caller charges or bounds its inputs before calling any of
// them.
#[cfg(feature = "test-support")]
pub use accounting::AdmittedCharge;
pub use accounting::{
    length_amount, Charge, ChargeAccounting, ChargeAdmission, ChargeCapture, ChargeEvent,
    ChargePoint, Incomplete, InjectedDenial, LimitKind, Meter, ScalarLimits,
    TruncatedChargeCapture,
};
pub use cancel::{Cancel, CancelCause, ChargeCount};
pub use collection::{
    build_typed_prestate, construct_collection, form, form_collection, form_grouped, from_admitted,
    inspect_typed_poststate, member_equal, CardinalityBound, CollectionKind, CollectionType,
    CollectionValue, ElementAdmission, EmptyCardinalityBound, MapType, NestedMapKey,
};
pub use comparison::{ComparisonOperator, IllTyped, IllTypedCause};
pub use decimal::{
    compare_shifted, evaluate_decimal, power_of_ten_bits, sbits, sdigits, Admitted, Decimal,
    DecimalLoss, DecimalOperation, DecimalRepresentation, DecimalResult, DecimalType, Placed,
    Placement, RoundingMode,
};
pub use decimal_membership::DecimalMembershipFailure;
pub use division::{divide, modulo, DivisionMember, DivisionProfile};
pub use equality::{
    plan_equality, planned_equality, planned_equality_with_plan, EqualityEvaluation, EqualityPlan,
};
pub use identity::{
    EffectiveId, EmptyMemberName, EmptyObjectIdentity, EmptyVariantName, MemberId, MemberOwner,
    ObjectId, OriginalDeclarationKey, PopulationId, UnitDomain, UnitId, UniverseId, VariantId,
};
pub use identity_copy::IdentityCopyError;
pub use ieee::{
    compare_ieee, convert_ieee_width, evaluate_ieee, exact_to_ieee, ieee_intrinsic_identities,
    ieee_to_exact, ExactScalar, FloatType, IeeeComparison, IeeeExact, IeeeExactLoss, IeeeFlag,
    IeeeFlags, IeeeOperand, IeeeOperation, IeeeOperationKind, IeeeProvenance, IeeeResult,
    IeeeValue, IeeeWidth, IEEE_DEFINITION,
};
pub use integer::{
    BoundedInteger, EmptyInterval, Integer, IntegerCopyError, IntegerCopyFootprint,
    IntegerDecimalBudget, IntegerDecimalBudgetError, IntegerDecimalError, IntegerDomain,
    IntegerInterval, IntegerParseError, IntegerReserveError, NonCanonicalInteger, NonZeroInteger,
    OutOfDomain, ZeroDivisor,
};
pub use key::compare_keys;
pub use location::{Location, Origin, Role};
pub use native_decimal::{
    evaluate_native_decimal, order_native_decimals, AdmittedNativeDecimalOperation, NativeDecimal,
    NativeDecimalCapacityQuantity, NativeDecimalCapacityRefusal, NativeDecimalLoss,
    NativeDecimalOperation, NativeDecimalRefusal, NativeDecimalResult, NativeDecimalType,
};
pub use native_enum::{compare_native_enum, NativeEnum, NativeEnumRefusal};
pub use native_integer::{
    charge_native_integer_schedule, evaluate_native_boolean_equality,
    evaluate_native_integer_arithmetic, evaluate_native_integer_division,
    evaluate_native_integer_equality, evaluate_native_integer_modulo,
    evaluate_native_integer_multiply_i64, evaluate_native_integer_ordering,
    AdmittedNativeIntegerOperation, NativeInteger, NativeIntegerAdmissionError,
    NativeIntegerEndpoint, NativeIntegerInterval, NativeIntegerOperandPosition,
    NativeIntegerOperands, NativeIntegerOperation, NativeIntegerOperationKind, NativeIntegerPlan,
    NativeIntegerSchedule,
};
pub use native_text::{
    compare_native_text, normalize_text, unwind_bound, NativeText, NativeTextRefusal,
    NormalizeRefusal, NormalizedSize, NATIVE_TEXT_CAPACITY,
};
pub use node::{
    is_identifier, Identifier, InvalidIdentifier, InvalidNodeKey, NodeKey,
    NodeKeyConstructionError, NodeKeyView, NodePathIter, PathSegment, PathSegmentView,
    MAX_NODE_ORDINAL,
};
pub use numeric::{
    evaluate_boolean, evaluate_integer_arithmetic, evaluate_rational_arithmetic, order_numbers,
    rational_arithmetic_bits, retain_boolean, ArithmeticOperator, BooleanConnective,
    IntegerArithmetic, OrderedOperands, OrderingOperator, RationalArithmetic,
};
pub use numeric_membership::{NumericMembershipFailure, NumericMembershipOperand};
pub use numeric_type_equality::{NumericTypeEndpoint, NumericTypeEqualityFailure};
pub use outcome::{
    BoundViolation, CheckedInvariantCause, InexactTarget, Outcome, Refusal, Undefined,
};
pub use quantity::{
    admit_quantity_result, compare_quantity, evaluate_quantity_arithmetic, BoundedQuantityType,
    CarriedQuantityType, Quantity, QuantityArithmetic, QuantityCarrier, QuantityResultInterval,
};
pub use rational::{NonPositiveDenominatorBound, Rational, RationalDomain, ZeroDenominator};
pub use reference::ObjectReference;
pub use reservation::{Reservation, ReservationDenied};
pub use text::{
    admit_text, compare_text, EmptyTextBounds, InvalidUtf8, NormalizationForm, Text,
    TextNotAdmittedCause, TextPayload, TextProfile, TextProvenance, TextType,
    UNICODE_TEXT_DEFINITION, UNICODE_VERSION,
};
pub use value::{
    evaluate_record, evaluate_tuple, evaluate_union, fill_slots, from_admitted_slots, record,
    retain_composite, tuple, union, Component, CompositeValue, ConstructionCause,
    ConstructionRefusal, Deferred, EnumMember, EnumShape, FieldDeclaration, FieldExpression,
    FieldValue, OptionValue, Presence, Timestamp, UnionMember, UnionValue, Uuid, Value, ValueType,
};
