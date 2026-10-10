// SPDX-License-Identifier: AGPL-3.0-or-later
//! `quire-exact`: the exact-value kernel, a `no_std` + `alloc` leaf that
//! depends on no other crate in the quire ecosystem.
//!
//! The crate holds checked identity ([`NodeKey`] and the seven opaque digest
//! identities, such as [`EffectiveId`]), provenance ([`Location`]), kernel
//! outcomes and refusals ([`Outcome`], [`Refusal`]), bounds and accounting
//! ([`Meter`], [`BoundedInteger`], [`CardinalityBound`]), and the exact
//! semantic value kernel ([`Value`]/[`ValueType`] and every value-family
//! module it composes: `numeric`, `rational`, `decimal`, `text`, `ieee`,
//! `division`, `comparison`, `equality`, `key`, `quantity`, `reference`).
//! Every submodule is private; the public surface is exactly the curated
//! `pub use` facade on this page, so the module names above are plain text,
//! not links.
//!
//! It depends on no wire format, hashing or canonicalization crate. Every
//! digest identity ([`NodeKey`], [`EffectiveId`], [`UniverseId`],
//! [`ObjectId`], [`UnitId`], [`VariantId`], [`MemberId`], [`PopulationId`])
//! is minted by wrapping an already-computed digest, through its one public
//! `from_digest` constructor (the two-domain [`UnitId`] has one constructor
//! per domain), and never by hashing internally.
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

mod accounting;
mod cancel;
mod collection;
mod comparison;
mod decimal;
mod division;
mod equality;
mod identity;
mod ieee;
mod integer;
mod key;
mod location;
mod node;
mod numeric;
mod outcome;
mod quantity;
mod rational;
mod reference;
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
pub use accounting::{
    length_amount, Charge, ChargePoint, Incomplete, InjectedDenial, LimitKind, Meter, ScalarLimits,
};
pub use cancel::{Cancel, CancelCause, ChargeCount};
pub use collection::{
    construct_collection, form, form_collection, form_grouped, from_admitted, member_equal,
    CardinalityBound, CollectionKind, CollectionType, CollectionValue, EmptyCardinalityBound,
};
pub use comparison::{ComparisonOperator, IllTyped, IllTypedCause};
pub use decimal::{
    compare_shifted, evaluate_decimal, power_of_ten_bits, sbits, sdigits, Admitted, Decimal,
    DecimalLoss, DecimalOperation, DecimalRepresentation, DecimalResult, DecimalType, Placed,
    Placement, RoundingMode,
};
pub use division::{divide, modulo, DivisionMember, DivisionProfile};
pub use equality::{plan_equality, planned_equality, EqualityPlan};
pub use identity::{
    EffectiveId, EmptyObjectIdentity, MemberId, ObjectId, PopulationId, UnitDomain, UnitId,
    UniverseId, VariantId, COMPOUND_UNIT_DOMAIN, EFFECTIVE_ID_DOMAIN, MEMBER_ID_DOMAIN,
    POPULATION_ID_DOMAIN, UNIVERSE_ID_DOMAIN, VARIANT_ID_DOMAIN,
};
pub use ieee::{
    compare_ieee, convert_ieee_width, evaluate_ieee, exact_to_ieee, ieee_intrinsic_identities,
    ieee_to_exact, ExactScalar, FloatType, IeeeComparison, IeeeExact, IeeeExactLoss,
    IeeeExactTarget, IeeeFlag, IeeeFlags, IeeeOperand, IeeeOperation, IeeeOperationKind,
    IeeeProvenance, IeeeResult, IeeeValue, IeeeWidth, IEEE_DEFINITION,
};
pub use integer::{
    BoundedInteger, EmptyInterval, Integer, IntegerDomain, IntegerInterval, NonCanonicalInteger,
    OutOfDomain,
};
pub use key::compare_keys;
pub use location::{Location, Origin, Role};
pub use node::{is_identifier, Identifier, InvalidIdentifier, NodeKey, NODE_KEY_DOMAIN};
pub use numeric::{
    evaluate_boolean, evaluate_integer_arithmetic, evaluate_rational_arithmetic, order_numbers,
    rational_arithmetic_bits, retain_boolean, ArithmeticOperator, BooleanConnective,
    IntegerArithmetic, OrderedOperands, OrderingOperator, RationalArithmetic,
};
pub use outcome::{
    BoundViolation, CheckedInvariantCause, InexactTarget, Outcome, Refusal, Undefined,
};
pub use quantity::{compare_quantity, evaluate_quantity_arithmetic, Quantity, QuantityArithmetic};
pub use rational::{NonPositiveDenominatorBound, Rational, RationalDomain, ZeroDenominator};
pub use reference::ObjectReference;
pub use text::{
    admit_text, compare_text, EmptyTextBounds, InvalidUtf8, NormalizationForm, Text, TextPayload,
    TextProfile, TextProvenance, TextType, UNICODE_TEXT_DEFINITION, UNICODE_VERSION,
};
pub use value::{
    evaluate_record, evaluate_tuple, evaluate_union, fill_slots, from_admitted_slots, record,
    retain_composite, tuple, union, Component, CompositeValue, ConstructionCause,
    ConstructionRefusal, Deferred, EnumMember, EnumShape, FieldDeclaration, FieldExpression,
    FieldValue, OptionValue, Presence, UnionMember, UnionValue, Value, ValueType,
};
