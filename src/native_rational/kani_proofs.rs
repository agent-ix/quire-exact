// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani harnesses of FR-408 Behavior 18. Each draws symbolic `i64` parts,
//! builds carriers with `from_i64_parts` as the representation constraint
//! (it admits exactly the canonical pairs), and compares the shipped
//! operation with the independent expected side of `reference.rs`: `i128`
//! `checked_*` arithmetic, Euclid's algorithm unrolled, and the literal
//! charge sequence. Meter limits are concrete per harness except where a
//! limit is derived from the drawn parts: a symbolic limit left the outcome
//! variant undecided and CBMC unwound drop glue without end. Each outcome is
//! held in `ManuallyDrop` and matched by reference, as in the enum
//! carrier's harnesses. No verdict is recorded here.
use super::reference::{self, Op};
use super::{
    native_rational_add, native_rational_equal, native_rational_multiply, native_rational_order,
    native_rational_subtract, NativeRational, NativeRationalRefusal as Refusal,
};
use crate::accounting::{ChargePoint, LimitKind, Meter, ScalarLimits};
use crate::numeric::OrderingOperator;
use crate::outcome::Outcome;
use core::mem::ManuallyDrop;

fn limits(
    integer_bits: u64,
    value_occurrences: u64,
    work_units: u64,
    result_units: u64,
) -> ScalarLimits {
    ScalarLimits {
        integer_bits,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences,
        work_units,
        result_units,
    }
}

fn sufficient() -> Meter {
    Meter::new(limits(u64::MAX, u64::MAX, u64::MAX, u64::MAX))
}

/// A canonical carrier with numerator in `[-9, 9]` and denominator in `[1, 9]`.
fn cg_carrier() -> NativeRational {
    let numerator: i64 = kani::any();
    let denominator: i64 = kani::any();
    kani::assume((-9..=9).contains(&numerator) && (1..=9).contains(&denominator));
    let built = NativeRational::from_i64_parts(numerator, denominator);
    kani::assume(built.is_ok());
    match built {
        Ok(carrier) => carrier,
        Err(_) => panic!("assumed canonical"),
    }
}

/// The same carriers as [`cg_carrier`], built by field and constrained
/// canonical by the reference Euclid instead of by `from_i64_parts`, whose
/// 128-step test is proved against the reference separately and would
/// otherwise be unrolled twice more into every arithmetic proof.
fn cg_reference_carrier() -> NativeRational {
    let numerator: i64 = kani::any();
    let denominator: i64 = kani::any();
    kani::assume((-9..=9).contains(&numerator) && (1..=9).contains(&denominator));
    kani::assume(reference::canonical_small(numerator, denominator));
    NativeRational {
        numerator: i128::from(numerator),
        denominator: i128::from(denominator),
    }
}

/// A canonical carrier over the full `i64` domain.
fn full_carrier() -> NativeRational {
    let numerator: i64 = kani::any();
    let denominator: i64 = kani::any();
    let built = NativeRational::from_i64_parts(numerator, denominator);
    kani::assume(built.is_ok());
    match built {
        Ok(carrier) => carrier,
        Err(_) => panic!("assumed canonical"),
    }
}

fn operands(left: &NativeRational, right: &NativeRational) -> (i128, i128, i128, i128) {
    (
        left.numerator(),
        left.denominator(),
        right.numerator(),
        right.denominator(),
    )
}

fn operate(
    op: Op,
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> ManuallyDrop<Result<Outcome<NativeRational>, Refusal>> {
    ManuallyDrop::new(match op {
        Op::Add => native_rational_add(left, right, meter),
        Op::Subtract => native_rational_subtract(left, right, meter),
        Op::Multiply => native_rational_multiply(left, right, meter),
    })
}

/// The success path of an arithmetic operation: the value is the reduced
/// unreduced fraction, and the meter shows the literal four-charge sequence.
fn arithmetic_completes(op: Op, left: NativeRational, right: NativeRational, wide: bool) {
    let parts = operands(&left, &right);
    let unreduced = match reference::formed(op, parts.0, parts.1, parts.2, parts.3) {
        Ok(unreduced) => unreduced,
        Err(_) => panic!("no capacity refusal inside signed 64 bits"),
    };
    let expected = reference::reduce(unreduced.0, unreduced.1, wide);
    let sizes = reference::arithmetic_sizes(op, parts, unreduced);
    let mut meter = sufficient();
    let outcome = operate(op, &left, &right, &mut meter);
    match &*outcome {
        Ok(Outcome::Completed(value)) => {
            kani::cover!(value.numerator() != unreduced.0, "reduced result");
            kani::cover!(value.numerator() < 0, "negative result");
            kani::cover!(value.numerator() == 0, "zero result");
            assert_eq!((value.numerator(), value.denominator()), expected);
        }
        _ => panic!("a sufficient meter completes"),
    }
    assert_eq!(
        meter.consumed(LimitKind::IntegerBits),
        sizes.operands.max(sizes.arithmetic).max(sizes.normalize)
    );
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 4);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    assert_eq!(meter.admission_count(), 4);
}

#[kani::proof]
fn native_rational_from_i64_parts_matches_reference() {
    let numerator: i64 = kani::any();
    let denominator: i64 = kani::any();
    let actual = NativeRational::from_i64_parts(numerator, denominator)
        .map(|carrier| (carrier.numerator(), carrier.denominator()));
    kani::cover!(actual.is_ok(), "success witness");
    kani::cover!(actual == Err(Refusal::ZeroDenominator), "zero denominator");
    kani::cover!(actual == Err(Refusal::NonCanonical), "non-canonical");
    assert_eq!(actual, reference::from_parts(numerator, denominator));
}

#[kani::proof]
fn native_rational_add_cg_domain_matches_reference() {
    arithmetic_completes(
        Op::Add,
        cg_reference_carrier(),
        cg_reference_carrier(),
        false,
    );
}

#[kani::proof]
fn native_rational_subtract_cg_domain_matches_reference() {
    arithmetic_completes(
        Op::Subtract,
        cg_reference_carrier(),
        cg_reference_carrier(),
        false,
    );
}

#[kani::proof]
fn native_rational_multiply_cg_domain_matches_reference() {
    arithmetic_completes(
        Op::Multiply,
        cg_reference_carrier(),
        cg_reference_carrier(),
        false,
    );
}

#[kani::proof]
fn native_rational_add_full_i64_matches_reference() {
    arithmetic_completes(Op::Add, full_carrier(), full_carrier(), true);
}

#[kani::proof]
fn native_rational_multiply_full_i64_matches_reference() {
    arithmetic_completes(Op::Multiply, full_carrier(), full_carrier(), true);
}

fn symbolic_ordering() -> OrderingOperator {
    match kani::any::<u8>() % 4 {
        0 => OrderingOperator::Less,
        1 => OrderingOperator::LessOrEqual,
        2 => OrderingOperator::Greater,
        _ => OrderingOperator::GreaterOrEqual,
    }
}

fn expected_order(operator: OrderingOperator, first: i128, second: i128) -> bool {
    match operator {
        OrderingOperator::Less => first < second,
        OrderingOperator::LessOrEqual => first <= second,
        OrderingOperator::Greater => first > second,
        OrderingOperator::GreaterOrEqual => first >= second,
    }
}

fn ordering_completes(left: NativeRational, right: NativeRational) {
    let operator = symbolic_ordering();
    let (a, p, c, q) = operands(&left, &right);
    let sizes = reference::ordering_sizes((a, p, c, q));
    let mut meter = sufficient();
    let outcome = ManuallyDrop::new(native_rational_order(operator, &left, &right, &mut meter));
    match &*outcome {
        Ok(Outcome::Completed(holds)) => {
            kani::cover!(*holds, "true witness");
            kani::cover!(!*holds, "false witness");
            assert_eq!(*holds, expected_order(operator, a * q, c * p));
        }
        _ => panic!("a sufficient meter completes"),
    }
    assert_eq!(meter.consumed(LimitKind::IntegerBits), sizes.0.max(sizes.1));
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    assert_eq!(meter.admission_count(), 3);
}

#[kani::proof]
fn native_rational_order_cg_domain_matches_reference() {
    ordering_completes(cg_carrier(), cg_carrier());
}

#[kani::proof]
fn native_rational_order_full_i64_matches_reference() {
    ordering_completes(full_carrier(), full_carrier());
}

fn equality_matches(left: NativeRational, right: NativeRational) {
    let (a, p, c, q) = operands(&left, &right);
    let mut meter = sufficient();
    let outcome = ManuallyDrop::new(native_rational_equal(&left, &right, &mut meter));
    match &*outcome {
        Outcome::Completed(equal) => {
            kani::cover!(*equal, "equal witness");
            kani::cover!(!*equal, "unequal witness");
            // Canonical fractions are equal exactly when they cross-multiply
            // equal.
            assert_eq!(*equal, a * q == c * p);
        }
        _ => panic!("a sufficient meter completes"),
    }
    assert_eq!(meter.admission_count(), 4);
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 1);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 5);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
}

#[kani::proof]
fn native_rational_equal_cg_domain_matches_reference() {
    equality_matches(cg_carrier(), cg_carrier());
}

#[kani::proof]
fn native_rational_equal_full_i64_matches_reference() {
    equality_matches(full_carrier(), full_carrier());
}

/// One denied charge of an addition over the CG domain: the record names the
/// charge point and counter, and the meter holds the admitted prefix.
fn denied_add(limits: ScalarLimits, point: ChargePoint, kind: LimitKind, prefix: u64) {
    let (left, right) = (cg_carrier(), cg_carrier());
    let mut meter = Meter::new(limits);
    let outcome = ManuallyDrop::new(native_rational_add(&left, &right, &mut meter));
    match &*outcome {
        Ok(Outcome::Incomplete(record)) => {
            kani::cover!(true, "denied path");
            assert_eq!(record.charge_point, point);
            assert_eq!(record.limit_kind, kind);
        }
        _ => panic!("the limit denies the charge"),
    }
    assert_eq!(meter.admission_count(), prefix);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}

#[kani::proof]
fn native_rational_add_denies_the_operands_occurrences() {
    denied_add(
        limits(u64::MAX, 1, u64::MAX, u64::MAX),
        ChargePoint::RationalArithmeticOperands,
        LimitKind::ValueOccurrences,
        0,
    );
}

#[kani::proof]
fn native_rational_add_denies_the_arithmetic_bits_one_short() {
    let (left, right) = (cg_carrier(), cg_carrier());
    let (a, p, c, q) = operands(&left, &right);
    let sizes = reference::arithmetic_sizes(Op::Add, (a, p, c, q), (a * q + c * p, p * q));
    let mut meter = Meter::new(limits(sizes.arithmetic - 1, u64::MAX, u64::MAX, u64::MAX));
    let outcome = ManuallyDrop::new(native_rational_add(&left, &right, &mut meter));
    match &*outcome {
        Ok(Outcome::Incomplete(record)) => {
            kani::cover!(true, "denied path");
            assert_eq!(
                record.charge_point,
                ChargePoint::RationalArithmeticArithmetic
            );
            assert_eq!(record.limit_kind, LimitKind::IntegerBits);
            assert_eq!(record.consumed, sizes.operands);
        }
        _ => panic!("one bit short denies the arithmetic charge"),
    }
    assert_eq!(meter.admission_count(), 1);
}

#[kani::proof]
fn native_rational_add_denies_the_fourth_work_unit() {
    denied_add(
        limits(u64::MAX, u64::MAX, 3, u64::MAX),
        ChargePoint::RationalArithmeticResultRetain,
        LimitKind::WorkUnits,
        3,
    );
}

#[kani::proof]
fn native_rational_add_denies_the_result_unit() {
    denied_add(
        limits(u64::MAX, u64::MAX, u64::MAX, 0),
        ChargePoint::RationalArithmeticResultRetain,
        LimitKind::ResultUnits,
        3,
    );
}

/// The composed predicate `add(a, b) == add(b, a)` over the CG domain.
#[kani::proof]
fn native_rational_add_is_commutative_over_cg_domain() {
    let (left, right) = (cg_reference_carrier(), cg_reference_carrier());
    let mut meter = sufficient();
    let forward = ManuallyDrop::new(native_rational_add(&left, &right, &mut meter));
    let backward = ManuallyDrop::new(native_rational_add(&right, &left, &mut meter));
    let (Ok(Outcome::Completed(forward)), Ok(Outcome::Completed(backward))) =
        (&*forward, &*backward)
    else {
        panic!("a sufficient meter completes");
    };
    let equal = ManuallyDrop::new(native_rational_equal(forward, backward, &mut meter));
    match &*equal {
        Outcome::Completed(holds) => {
            kani::cover!(*holds, "predicate holds");
            assert!(*holds);
        }
        _ => panic!("a sufficient meter completes"),
    }
}
