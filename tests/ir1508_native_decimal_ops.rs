// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native Decimal operations against independent literals and the owner
//! route (FR-403, TC-951).
use ix_trace_rs::trace;
use quire_exact::{
    evaluate_decimal, evaluate_native_decimal, order_native_decimals, order_numbers, ChargeEvent,
    ChargePoint, Decimal, DecimalOperation, DecimalResult, DecimalType, InexactTarget, Integer,
    IntegerInterval, LimitKind, Meter, NativeDecimal as D, NativeDecimalCapacityQuantity as Q,
    NativeDecimalCapacityRefusal, NativeDecimalOperation as Nop, NativeDecimalType as T,
    OrderedOperands, OrderingOperator as O, Outcome, Rational, Refusal, RoundingMode as M,
    ScalarLimits, Undefined,
};

const MAX: i128 = i128::MAX;
const MIN: i128 = i128::MIN;
const P125: i128 = 1 << 125;
const P126: i128 = 1 << 126;
const P63: i128 = 1 << 63;
const P64: i128 = 1 << 64;

type Pair = (i128, u32);

/// A `Decimal[lo, hi; smin, smax; mode]` written as literals.
#[derive(Clone, Copy, Debug)]
struct Ty {
    lo: i128,
    hi: i128,
    smin: u64,
    smax: u64,
    mode: M,
}

const fn ty(lo: i128, hi: i128, smin: u64, smax: u64, mode: M) -> Ty {
    Ty {
        lo,
        hi,
        smin,
        smax,
        mode,
    }
}

fn native_ty(t: Ty) -> T {
    T::new(t.lo, t.hi, t.smin, t.smax, t.mode).unwrap()
}

fn owner_ty(t: Ty) -> DecimalType {
    DecimalType::new(
        Integer::from(t.lo),
        Integer::from(t.hi),
        t.smin,
        t.smax,
        t.mode,
    )
    .unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add(Pair, Pair),
    Sub(Pair, Pair),
    Mul(Pair, Pair),
    Div(Pair, Pair),
    Neg(Pair),
    Round(Pair),
    Order(O, Pair, Pair),
}

fn d(p: Pair) -> D {
    D::from_retained(p.0, p.1)
}

fn owner_d(p: Pair) -> Decimal {
    Decimal::new(Integer::from(p.0), p.1)
}

fn native_op(op: Op) -> Option<Nop> {
    Some(match op {
        Op::Add(a, b) => Nop::Add(d(a), d(b)),
        Op::Sub(a, b) => Nop::Subtract(d(a), d(b)),
        Op::Mul(a, b) => Nop::Multiply(d(a), d(b)),
        Op::Div(a, b) => Nop::Divide(d(a), d(b)),
        Op::Neg(a) => Nop::Negate(d(a)),
        Op::Round(a) => Nop::Round(d(a)),
        Op::Order(..) => return None,
    })
}

#[derive(Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
enum Res {
    Arith(Outcome<DecimalResult>),
    Ord(Outcome<bool>),
}

fn limits_of(value: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: value,
        decimal_digits: value,
        scale_expansion: value,
        text_input_bytes: value,
        text_scalars: value,
        normalized_scalars: value,
        unit_edges: value,
        value_occurrences: value,
        work_units: value,
        result_units: value,
    }
}

/// Limits that keep the owner from forming a huge power of ten.
fn bounded() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 100_000,
        decimal_digits: 100_000,
        scale_expansion: 4096,
        ..limits_of(u64::MAX)
    }
}

fn get(limits: &ScalarLimits, kind: LimitKind) -> u64 {
    match kind {
        LimitKind::IntegerBits => limits.integer_bits,
        LimitKind::DecimalDigits => limits.decimal_digits,
        LimitKind::ScaleExpansion => limits.scale_expansion,
        LimitKind::TextInputBytes => limits.text_input_bytes,
        LimitKind::TextScalars => limits.text_scalars,
        LimitKind::NormalizedScalars => limits.normalized_scalars,
        LimitKind::UnitEdges => limits.unit_edges,
        LimitKind::ValueOccurrences => limits.value_occurrences,
        LimitKind::WorkUnits => limits.work_units,
        LimitKind::ResultUnits => limits.result_units,
    }
}

fn set(limits: &mut ScalarLimits, kind: LimitKind, value: u64) {
    match kind {
        LimitKind::IntegerBits => limits.integer_bits = value,
        LimitKind::DecimalDigits => limits.decimal_digits = value,
        LimitKind::ScaleExpansion => limits.scale_expansion = value,
        LimitKind::TextInputBytes => limits.text_input_bytes = value,
        LimitKind::TextScalars => limits.text_scalars = value,
        LimitKind::NormalizedScalars => limits.normalized_scalars = value,
        LimitKind::UnitEdges => limits.unit_edges = value,
        LimitKind::ValueOccurrences => limits.value_occurrences = value,
        LimitKind::WorkUnits => limits.work_units = value,
        LimitKind::ResultUnits => limits.result_units = value,
    }
}

fn recording(limits: ScalarLimits) -> Meter {
    let mut meter = Meter::new(limits);
    meter.enable_charge_capture(100_000, 1_000_000);
    meter
}

/// Everything a Meter shows after an operation.
#[derive(Debug, PartialEq)]
struct Seen {
    events: Vec<ChargeEvent>,
    consumed: Vec<u64>,
    admissions: u64,
}

fn seen(meter: &mut Meter) -> Seen {
    let consumed = LimitKind::ALL.map(|kind| meter.consumed(kind)).to_vec();
    let admissions = meter.admission_count();
    let capture = meter.take_charge_capture().unwrap().unwrap();
    Seen {
        events: capture.events().to_vec(),
        consumed,
        admissions,
    }
}

fn read_out(outcome: Outcome<quire_exact::NativeDecimalResult>) -> Outcome<DecimalResult> {
    match outcome {
        Outcome::Completed(result) => Outcome::Completed(result.to_decimal_result()),
        Outcome::Undefined(reason) => Outcome::Undefined(reason),
        Outcome::Refused(refusal) => Outcome::Refused(refusal),
        Outcome::Incomplete(record) => Outcome::Incomplete(record),
    }
}

/// The native route: admission without a Meter, then the evaluation.
fn native_on(t: Ty, op: Op, meter: &mut Meter) -> Result<Res, NativeDecimalCapacityRefusal> {
    let native = native_ty(t);
    if let Op::Order(operator, a, b) = op {
        return Ok(Res::Ord(order_native_decimals(operator, d(a), d(b), meter)));
    }
    let admitted = native.admit(native_op(op).unwrap())?;
    Ok(Res::Arith(read_out(evaluate_native_decimal(
        admitted, meter,
    ))))
}

fn owner_on(t: Ty, op: Op, meter: &mut Meter) -> Res {
    let target = owner_ty(t);
    let arith = |operation: DecimalOperation<'_>, meter: &mut Meter| {
        Res::Arith(evaluate_decimal(operation, &target, meter))
    };
    match op {
        Op::Add(a, b) => arith(DecimalOperation::Add(&owner_d(a), &owner_d(b)), meter),
        Op::Sub(a, b) => arith(DecimalOperation::Subtract(&owner_d(a), &owner_d(b)), meter),
        Op::Mul(a, b) => arith(DecimalOperation::Multiply(&owner_d(a), &owner_d(b)), meter),
        Op::Div(a, b) => arith(DecimalOperation::Divide(&owner_d(a), &owner_d(b)), meter),
        Op::Neg(a) => arith(DecimalOperation::Negate(&owner_d(a)), meter),
        Op::Round(a) => arith(DecimalOperation::Round(&owner_d(a)), meter),
        Op::Order(operator, a, b) => Res::Ord(order_numbers(
            operator,
            OrderedOperands::Decimals(&owner_d(a), &owner_d(b)),
            meter,
        )),
    }
}

fn run_native(
    t: Ty,
    op: Op,
    limits: ScalarLimits,
) -> Result<(Res, Seen), NativeDecimalCapacityRefusal> {
    let mut meter = recording(limits);
    let result = native_on(t, op, &mut meter)?;
    Ok((result, seen(&mut meter)))
}

fn run_owner(t: Ty, op: Op, limits: ScalarLimits) -> (Res, Seen) {
    let mut meter = recording(limits);
    let result = owner_on(t, op, &mut meter);
    (result, seen(&mut meter))
}

fn completed(t: Ty, op: Op) -> DecimalResult {
    match run_native(t, op, limits_of(u64::MAX)).unwrap().0 {
        Res::Arith(Outcome::Completed(result)) => result,
        other => panic!("{t:?} {op:?}: {other:?}"),
    }
}

fn pair_of(result: &DecimalResult) -> (Integer, u32) {
    (
        result.value().representation().coefficient().clone(),
        result.value().representation().scale(),
    )
}

fn assert_done(t: Ty, op: Op, coefficient: i128, scale: u32) {
    let result = completed(t, op);
    assert_eq!(
        pair_of(&result),
        (Integer::from(coefficient), scale),
        "{t:?} {op:?}"
    );
}

fn refusal_of(t: Ty, op: Op) -> Q {
    native_ty(t)
        .admit(native_op(op).unwrap())
        .unwrap_err()
        .quantity
}

fn admitted(t: Ty, op: Op) -> bool {
    native_ty(t).admit(native_op(op).unwrap()).is_ok()
}

fn stop_of(t: Ty, op: Op) -> Res {
    run_native(t, op, limits_of(u64::MAX)).unwrap().0
}

fn points(t: Ty, op: Op) -> Vec<ChargePoint> {
    run_native(t, op, limits_of(u64::MAX))
        .unwrap()
        .1
        .events
        .iter()
        .map(|event| event.point)
        .collect()
}

fn sizes_at(t: Ty, op: Op, point: ChargePoint) -> Vec<(LimitKind, u64)> {
    let seen = run_native(t, op, limits_of(u64::MAX)).unwrap().1;
    let event = seen.events.iter().find(|event| event.point == point);
    event
        .unwrap_or_else(|| panic!("{point:?} not charged for {op:?}"))
        .sizes
        .iter()
        .map(|(kind, amount)| (*kind, amount.to_u64().unwrap()))
        .collect()
}

fn result_units_at(t: Ty, op: Op, point: ChargePoint) -> u64 {
    let seen = run_native(t, op, limits_of(u64::MAX)).unwrap().1;
    let event = seen
        .events
        .iter()
        .find(|event| event.point == point)
        .unwrap();
    event.result_units.to_u64().unwrap()
}

const EXACT2: Ty = ty(0, 100, 0, 2, M::Exact);

use ChargePoint as P;
use LimitKind as K;

#[test]
#[trace("TC-951", "FR-403-AC-1")]
fn admission_edges_sums_products_and_shifts() {
    let wide = ty(-P126, P126, 0, 0, M::Exact);
    assert_done(wide, Op::Add((P125, 0), (P125, 0)), P126, 0);
    assert_eq!(refusal_of(wide, Op::Add((P126, 0), (P126, 0))), Q::Sum);
    assert_done(wide, Op::Mul((P63, 0), (P63, 0)), P126, 0);
    assert_eq!(refusal_of(wide, Op::Mul((P64, 0), (P64, 0))), Q::Product);

    let full = ty(MIN, MAX, 0, 38, M::Exact);
    let ten38 = 10_i128.pow(38);
    assert_done(full, Op::Add((1, 0), (1, 38)), ten38 + 1, 38);
    assert_eq!(
        refusal_of(full, Op::Add((2, 0), (1, 38))),
        Q::ShiftedOperand
    );
    assert_eq!(refusal_of(full, Op::Add((1, 0), (1, 39))), Q::Exponent);
    assert_eq!(refusal_of(full, Op::Add((0, 0), (0, 39))), Q::Exponent);
    assert_eq!(refusal_of(full, Op::Neg((MIN, 0))), Q::Negation);

    let nearest = ty(0, 1, 0, 0, M::NearestEven);
    assert_eq!(
        refusal_of(nearest, Op::Div((P126, 0), (P126 + 1, 0))),
        Q::DoubledRemainder
    );
    assert_done(nearest, Op::Div((P126 - 1, 0), (P126, 0)), 1, 0);
    assert_done(nearest, Op::Div((P126, 0), (MAX - 1, 0)), 1, 0);

    let toward = ty(MIN, MAX, 0, 0, M::TowardZero);
    assert_eq!(
        refusal_of(toward, Op::Div((MIN, 0), (-3, 0))),
        Q::CanonicalQuotient
    );
    assert_eq!(
        refusal_of(toward, Op::Div((1, 0), (MIN, 0))),
        Q::CanonicalQuotient
    );
    assert_done(toward, Op::Div((MIN, 0), (-2, 0)), P126, 0);
    assert_done(toward, Op::Div((MIN, 0), (1, 0)), MIN, 0);

    let lifted = ty(0, 1, 0, 100, M::Exact);
    assert_eq!(refusal_of(lifted, Op::Neg((0, 0))), Q::Exponent);
    assert_done(ty(0, 100, 0, 2, M::Exact), Op::Mul((0, 2), (7, 2)), 0, 2);
}

#[test]
#[trace("TC-951", "FR-403-AC-1")]
fn capacity_edges_of_the_boundary_set() {
    let numerator = ty(0, 10_i128.pow(38), 0, 38, M::Exact);
    assert_eq!(refusal_of(numerator, Op::Div((1, 0), (1, 38))), Q::Exponent);
    assert!(admitted(numerator, Op::Div((1, 0), (0, 1))));
    assert_eq!(
        stop_of(numerator, Op::Div((1, 0), (0, 1))),
        Res::Arith(Outcome::Undefined(Undefined::DivisionByZero))
    );
    assert_done(EXACT2, Op::Div((1, 0), (1, 0)), 100, 2);

    let nearest = ty(0, 1, 0, 0, M::NearestEven);
    assert_done(nearest, Op::Div((P126, 0), (MAX - 1, 0)), 1, 0);

    let toward = ty(MIN, MAX, 0, 0, M::TowardZero);
    assert!(admitted(toward, Op::Round((10_i128.pow(38), 1))));
    assert_done(toward, Op::Round((MAX, 38)), 1, 0);
    assert_eq!(refusal_of(toward, Op::Round((MAX, 39))), Q::PowerOfTen);

    let positive = ty(0, 100, 0, 2, M::TowardPositive);
    let max_scale = u32::MAX;
    assert_done(positive, Op::Mul((1, max_scale), (1, max_scale)), 1, 2);
    let loss = completed(positive, Op::Mul((1, max_scale), (1, max_scale)));
    let loss = loss.loss().unwrap();
    assert_eq!(*loss.exact_numerator(), Integer::one());
    assert_eq!(loss.rounded_coefficient(), &Integer::one());

    let sticky = ty(MIN, MAX, 38, 38, M::Exact);
    assert_done(sticky, Op::Neg((-1, 0)), 10_i128.pow(38), 38);
    assert_eq!(refusal_of(sticky, Op::Neg((-2, 0))), Q::RetainedCoefficient);

    let zero_lift = ty(0, 1, 0, 100, M::Exact);
    assert_eq!(refusal_of(zero_lift, Op::Neg((0, 0))), Q::Exponent);
    assert_eq!(
        refusal_of(ty(MIN, MAX, 0, 38, M::Exact), Op::Add((0, 0), (0, 39))),
        Q::Exponent
    );
    assert_done(ty(0, 100, 0, 2, M::Exact), Op::Mul((0, 2), (7, 2)), 0, 2);
    // A Meter presented to a refused admission is unchanged: admission takes
    // no Meter, so the refusal arrives before any Meter exists to touch.
    let mut meter = recording(limits_of(7));
    let before = seen(&mut meter);
    let refused = native_ty(toward).admit(Nop::Round(d((MAX, 39))));
    assert!(refused.is_err());
    assert_eq!(seen(&mut meter), before);
}

#[test]
#[trace("TC-951", "FR-403-AC-2")]
fn retained_representation_is_not_normalized() {
    assert_done(EXACT2, Op::Add((5, 1), (5, 1)), 100, 2);
    assert_done(EXACT2, Op::Sub((5, 1), (1, 1)), 40, 2);
    assert_done(EXACT2, Op::Sub((5, 1), (5, 1)), 0, 2);
    assert_done(EXACT2, Op::Mul((5, 1), (4, 1)), 20, 2);
    assert_done(EXACT2, Op::Neg((0, 0)), 0, 2);
    assert_done(EXACT2, Op::Round((5, 1)), 50, 2);
    assert_done(EXACT2, Op::Div((1, 0), (4, 0)), 25, 2);
    let zero = completed(EXACT2, Op::Div((0, 0), (12, 0)));
    assert_eq!(pair_of(&zero), (Integer::zero(), 2));
    assert!(zero.loss().is_none());
    assert_eq!(points(EXACT2, Op::Div((0, 0), (12, 0))).len(), 4);
}

#[test]
#[trace("TC-951", "FR-403-AC-3")]
fn typed_outcomes_and_their_charge_points() {
    let any = ty(0, 100, 0, 2, M::Exact);
    let division = Op::Div((1, 0), (0, 3));
    assert!(admitted(any, division));
    assert_eq!(
        stop_of(any, division),
        Res::Arith(Outcome::Undefined(Undefined::DivisionByZero))
    );
    assert_eq!(points(any, division), vec![P::DecimalOperands]);
    let (zero_budget, _) = run_native(
        any,
        division,
        ScalarLimits {
            work_units: 0,
            ..limits_of(u64::MAX)
        },
    )
    .unwrap();
    match zero_budget {
        Res::Arith(Outcome::Incomplete(record)) => {
            assert_eq!(record.charge_point, P::DecimalOperands);
            assert_eq!(record.limit_kind, K::WorkUnits);
        }
        other => panic!("{other:?}"),
    }
    let inexact = |t: Ty, op: Op| {
        let target = owner_ty(t);
        assert_eq!(
            stop_of(t, op),
            Res::Arith(Outcome::Refused(Refusal::InexactDecimal {
                target: InexactTarget::Decimal(Box::new(target))
            })),
            "{op:?}"
        );
        let charged = points(t, op);
        assert!(!charged.contains(&P::DecimalRounding), "{op:?}");
    };
    inexact(any, Op::Div((1, 0), (3, 0)));
    inexact(any, Op::Mul((1, 2), (1, 2)));
    inexact(any, Op::Add((1, 3), (0, 0)));
    assert_done(any, Op::Mul((0, 2), (7, 2)), 0, 2);
    assert!(!points(any, Op::Mul((0, 2), (7, 2))).contains(&P::DecimalRounding));

    let out = |t: Ty, op: Op| {
        assert_eq!(
            stop_of(t, op),
            Res::Arith(Outcome::Refused(Refusal::DecimalOutOfDomain {
                target: Box::new(owner_ty(t))
            })),
            "{op:?}"
        );
        assert!(!points(t, op).contains(&P::DecimalResultRetain), "{op:?}");
    };
    out(any, Op::Add((100, 0), (1, 0)));
    out(ty(0, 100, 0, 0, M::NearestAway), Op::Round((1005, 1)));
    assert_done(
        ty(0, 100, 0, 0, M::TowardZero),
        Op::Round((1005, 1)),
        100,
        0,
    );
    assert_done(
        ty(0, 100, 0, 0, M::NearestEven),
        Op::Round((1004, 1)),
        100,
        0,
    );
    // the rounding step is charged before the membership refusal
    assert!(
        points(ty(0, 100, 0, 0, M::NearestAway), Op::Round((1005, 1)))
            .contains(&P::DecimalRounding)
    );
}

const MODES: [M; 5] = [
    M::TowardZero,
    M::TowardPositive,
    M::TowardNegative,
    M::NearestEven,
    M::NearestAway,
];

#[test]
#[trace("TC-951", "FR-403-AC-4")]
fn rounding_results_in_all_modes() {
    let cases: [(Op, [i128; 5]); 6] = [
        (Op::Div((1, 0), (8, 0)), [12, 13, 12, 12, 13]),
        (Op::Div((-1, 0), (8, 0)), [-12, -12, -13, -12, -13]),
        (Op::Div((3, 0), (8, 0)), [37, 38, 37, 38, 38]),
        (Op::Round((125, 3)), [12, 13, 12, 12, 13]),
        (Op::Round((-125, 3)), [-12, -12, -13, -12, -13]),
        (Op::Round((135, 3)), [13, 14, 13, 14, 14]),
    ];
    for (op, expected) in cases {
        for (mode, want) in MODES.into_iter().zip(expected) {
            assert_done(ty(-100, 100, 0, 2, mode), op, want, 2);
        }
    }
    assert_done(
        ty(0, 100, 0, 2, M::TowardPositive),
        Op::Mul((1, 2), (1, 2)),
        1,
        2,
    );
    assert_done(
        ty(0, 100, 0, 2, M::TowardZero),
        Op::Mul((1, 2), (1, 2)),
        0,
        2,
    );
}

fn loss_of(t: Ty, op: Op) -> (i128, i128, u64, u64, (i128, u32), M) {
    let native = native_ty(t);
    let admitted = native.admit(native_op(op).unwrap()).unwrap();
    let mut meter = Meter::new(limits_of(u64::MAX));
    let result = evaluate_native_decimal(admitted, &mut meter)
        .completed()
        .unwrap();
    let loss = result.loss().expect("a rounding step");
    (
        loss.numerator(),
        loss.cofactor(),
        loss.twos(),
        loss.fives(),
        (loss.rounded().coefficient(), loss.rounded().scale()),
        loss.mode(),
    )
}

#[test]
#[trace("TC-951", "FR-403-AC-5")]
fn loss_records_are_the_owners_factored() {
    let ne = ty(-100, 100, 0, 2, M::NearestEven);
    assert_eq!(
        loss_of(ne, Op::Div((1, 0), (8, 0))),
        (1, 1, 3, 0, (12, 2), M::NearestEven)
    );
    assert_eq!(
        loss_of(ne, Op::Round((125, 3))),
        (1, 1, 3, 0, (12, 2), M::NearestEven)
    );
    assert_eq!(
        loss_of(ne, Op::Round((-125, 3))),
        (-1, 1, 3, 0, (-12, 2), M::NearestEven)
    );
    assert_eq!(
        loss_of(ne, Op::Div((1, 0), (3, 0))),
        (1, 3, 0, 0, (33, 2), M::NearestEven)
    );
    // reduced by the common factor 3 to the same quotient
    assert_eq!(
        loss_of(ne, Op::Div((3, 0), (9, 0))),
        (1, 3, 0, 0, (33, 2), M::NearestEven)
    );
    assert_eq!(
        loss_of(ne, Op::Div((3, 0), (8, 0))),
        (3, 1, 3, 0, (38, 2), M::NearestEven)
    );
    // 252 gcd steps
    let unit = ty(0, 1, 0, 0, M::NearestEven);
    assert_eq!(
        loss_of(unit, Op::Div((P126 - 1, 0), (P126, 0))),
        (P126 - 1, 1, 126, 0, (1, 0), M::NearestEven)
    );
    // the exact intermediate 1/10^4, not the proxy 1/10
    let tp = ty(0, 100, 0, 2, M::TowardPositive);
    assert_eq!(
        loss_of(tp, Op::Mul((1, 2), (1, 2))),
        (1, 1, 4, 4, (1, 2), M::TowardPositive)
    );
    assert_eq!(
        loss_of(tp, Op::Mul((1, u32::MAX), (1, u32::MAX))),
        (
            1,
            1,
            8_589_934_590,
            8_589_934_590,
            (1, 2),
            M::TowardPositive
        )
    );
    assert!(completed(EXACT2, Op::Div((1, 0), (4, 0))).loss().is_none());
}

#[test]
#[trace("TC-951", "FR-403-AC-6")]
fn charge_amounts_are_the_owners() {
    let add = Op::Add((5, 1), (5, 1));
    assert_eq!(
        points(EXACT2, add),
        vec![
            P::DecimalOperands,
            P::DecimalScaleExpansion,
            P::DecimalArithmetic,
            P::DecimalResultRetain
        ]
    );
    assert_eq!(
        sizes_at(EXACT2, add, P::DecimalOperands),
        vec![
            (K::IntegerBits, 3),
            (K::DecimalDigits, 1),
            (K::ValueOccurrences, 2)
        ]
    );
    assert_eq!(
        sizes_at(EXACT2, add, P::DecimalScaleExpansion),
        vec![(K::ScaleExpansion, 0)]
    );
    assert_eq!(
        sizes_at(EXACT2, add, P::DecimalArithmetic),
        vec![(K::IntegerBits, 4), (K::DecimalDigits, 2)]
    );
    assert_eq!(
        sizes_at(EXACT2, add, P::DecimalResultRetain),
        vec![
            (K::ScaleExpansion, 1),
            (K::IntegerBits, 8),
            (K::DecimalDigits, 3),
            (K::ValueOccurrences, 1)
        ]
    );
    assert_eq!(result_units_at(EXACT2, add, P::DecimalResultRetain), 1);

    let four = ty(0, 10_000, 0, 3, M::Exact);
    let shifted = Op::Add((1, 0), (1, 3));
    assert_eq!(
        sizes_at(four, shifted, P::DecimalScaleExpansion),
        vec![
            (K::ScaleExpansion, 3),
            (K::IntegerBits, 11),
            (K::DecimalDigits, 4)
        ]
    );
    assert_eq!(
        sizes_at(four, shifted, P::DecimalArithmetic),
        vec![(K::IntegerBits, 12), (K::DecimalDigits, 5)]
    );

    let ne = ty(-100, 100, 0, 2, M::NearestEven);
    let div = Op::Div((1, 0), (8, 0));
    assert_eq!(
        points(ne, div),
        vec![
            P::DecimalOperands,
            P::DecimalScaleExpansion,
            P::DecimalArithmetic,
            P::DecimalRounding,
            P::DecimalResultRetain
        ]
    );
    assert_eq!(
        sizes_at(ne, div, P::DecimalOperands),
        vec![
            (K::IntegerBits, 4),
            (K::DecimalDigits, 1),
            (K::ValueOccurrences, 2)
        ]
    );
    assert_eq!(
        sizes_at(ne, div, P::DecimalScaleExpansion),
        vec![
            (K::ScaleExpansion, 2),
            (K::IntegerBits, 8),
            (K::DecimalDigits, 3)
        ]
    );
    assert_eq!(
        sizes_at(ne, div, P::DecimalArithmetic),
        vec![(K::IntegerBits, 8), (K::DecimalDigits, 3)]
    );
    assert_eq!(
        sizes_at(ne, div, P::DecimalRounding),
        vec![(K::IntegerBits, 8), (K::DecimalDigits, 3)]
    );
    assert_eq!(
        sizes_at(ne, div, P::DecimalResultRetain),
        vec![
            (K::ScaleExpansion, 0),
            (K::IntegerBits, 4),
            (K::DecimalDigits, 2),
            (K::ValueOccurrences, 1)
        ]
    );
    // Multiply charges the sum of the operand sizes
    assert_eq!(
        sizes_at(EXACT2, Op::Mul((5, 1), (4, 1)), P::DecimalArithmetic),
        vec![(K::IntegerBits, 6), (K::DecimalDigits, 2)]
    );
    assert!(!points(EXACT2, add).contains(&P::DecimalRounding));
}

#[test]
#[trace("TC-951", "FR-403-AC-8")]
fn ordering_is_by_value_and_charges_as_the_owner() {
    let any = EXACT2;
    let run = |operator: O, a: Pair, b: Pair| match stop_of(any, Op::Order(operator, a, b)) {
        Res::Ord(Outcome::Completed(value)) => value,
        other => panic!("{other:?}"),
    };
    assert!(run(O::Less, (10, 1), (2, 0)));
    assert!(run(O::LessOrEqual, (10, 1), (1, 0)));
    assert!(!run(O::Greater, (10, 1), (1, 0)));
    assert!(run(O::GreaterOrEqual, (-1, 0), (-10, 1)));
    let lt = Op::Order(O::Less, (10, 1), (2, 0));
    assert_eq!(
        sizes_at(any, lt, P::OrderingOperands),
        vec![
            (K::IntegerBits, 4),
            (K::DecimalDigits, 2),
            (K::ValueOccurrences, 2)
        ]
    );
    assert_eq!(
        sizes_at(any, lt, P::OrderingArithmetic),
        vec![
            (K::ScaleExpansion, 1),
            (K::IntegerBits, 6),
            (K::DecimalDigits, 2)
        ]
    );
    assert_eq!(result_units_at(any, lt, P::OrderingResultRetain), 1);
    for (operator, want) in [
        (O::Less, false),
        (O::LessOrEqual, false),
        (O::Greater, true),
        (O::GreaterOrEqual, true),
    ] {
        assert_eq!(run(operator, (2, 0), (1, 38)), want);
    }
    let wide = Op::Order(O::Greater, (2, 0), (1, 38));
    assert_eq!(
        sizes_at(any, wide, P::OrderingArithmetic),
        vec![
            (K::ScaleExpansion, 38),
            (K::IntegerBits, 129),
            (K::DecimalDigits, 39)
        ]
    );
    let more = Op::Order(O::Greater, (1, 0), (1, 39));
    assert!(run(O::Greater, (1, 0), (1, 39)));
    assert_eq!(
        sizes_at(any, more, P::OrderingArithmetic),
        vec![
            (K::ScaleExpansion, 39),
            (K::IntegerBits, 131),
            (K::DecimalDigits, 40)
        ]
    );
    let huge = Op::Order(O::Greater, (1, 0), (1, 4_294_967_295));
    assert!(run(O::Greater, (1, 0), (1, 4_294_967_295)));
    assert_eq!(
        sizes_at(any, huge, P::OrderingArithmetic),
        vec![
            (K::ScaleExpansion, 4_294_967_295),
            (K::IntegerBits, 14_267_572_525),
            (K::DecimalDigits, 4_294_967_296)
        ]
    );
}

// ---------------------------------------------------------------------------
// The differential (AC-7, AC-11): the native route beside the owner route.
// ---------------------------------------------------------------------------

fn types() -> Vec<Ty> {
    let mut all = vec![
        ty(0, 100, 0, 2, M::Exact),
        ty(0, 10_000, 0, 3, M::Exact),
        ty(0, 1, 0, 0, M::NearestEven),
        ty(0, 1, 0, 100, M::Exact),
        ty(0, 100, 0, 0, M::NearestEven),
        ty(0, 100, 0, 0, M::NearestAway),
        ty(0, 100, 0, 0, M::TowardZero),
        ty(MIN, MAX, 0, 0, M::TowardZero),
        ty(MIN, MAX, 0, 0, M::NearestEven),
        ty(MIN, MAX, 0, 38, M::Exact),
        ty(MIN, MAX, 38, 38, M::Exact),
        ty(0, 10_i128.pow(38), 0, 38, M::Exact),
        ty(-P126, P126, 0, 0, M::Exact),
        ty(0, 100, 0, 2, M::TowardPositive),
    ];
    all.extend(MODES.map(|mode| ty(-100, 100, 0, 2, mode)));
    all
}

fn operands() -> Vec<Pair> {
    vec![
        (0, 0),
        (0, 3),
        (0, 39),
        (1, 0),
        (-1, 0),
        (2, 0),
        (3, 0),
        (-3, 0),
        (5, 1),
        (-5, 1),
        (10, 1),
        (-10, 1),
        (125, 3),
        (-125, 3),
        (135, 3),
        (1, 2),
        (7, 2),
        (8, 0),
        (9, 0),
        (12, 0),
        (100, 0),
        (1004, 1),
        (1005, 1),
        (1, 38),
        (1, 39),
        (MAX, 0),
        (MIN, 0),
        (MAX, 38),
        (MAX, 39),
        (P125, 0),
        (P126 - 1, 0),
        (P126, 0),
        (P126 + 1, 0),
        (MAX - 1, 0),
        (P63, 0),
        (P64, 0),
        (1, u32::MAX),
        (10_i128.pow(38), 1),
    ]
}

fn all_ops(a: Pair, b: Pair) -> Vec<Op> {
    let mut ops = vec![
        Op::Add(a, b),
        Op::Sub(a, b),
        Op::Mul(a, b),
        Op::Div(a, b),
        Op::Neg(a),
        Op::Round(a),
    ];
    ops.extend(
        [O::Less, O::LessOrEqual, O::Greater, O::GreaterOrEqual].map(|o| Op::Order(o, a, b)),
    );
    ops
}

fn pow10(exponent: u64) -> Integer {
    Integer::power_of_ten(exponent)
}

fn fits(value: &Integer) -> bool {
    IntegerInterval::new(Integer::from(MIN), Integer::from(MAX))
        .unwrap()
        .contains(value)
}

fn digits(value: &Integer) -> u64 {
    value.decimal_digits()
}

/// The first failing quantity of Behavior 3, derived with the owner's
/// arbitrary-precision `Integer` and `Rational` and the spec's grounds.
fn model(t: Ty, op: Op) -> Option<Q> {
    let target = t.smax;
    let big = |c: i128| Integer::from(c);
    let lift_ok = |working: u64| target - working.min(target) <= 38;
    let scaled = |value: Integer, working: u64| place_model(t, value, working);
    match op {
        Op::Add((ca, sa), (cb, sb)) | Op::Sub((ca, sa), (cb, sb)) => {
            let subtract = matches!(op, Op::Sub(..));
            let scale = sa.max(sb);
            let (ka, kb) = (u64::from(scale - sa), u64::from(scale - sb));
            if ka > 38 || kb > 38 || !lift_ok(u64::from(scale)) {
                return Some(Q::Exponent);
            }
            let (xa, xb) = (big(ca).mul(&pow10(ka)), big(cb).mul(&pow10(kb)));
            if !fits(&xa) || !fits(&xb) {
                return Some(Q::ShiftedOperand);
            }
            let value = if subtract { xa.sub(&xb) } else { xa.add(&xb) };
            if !fits(&value) {
                return Some(if subtract { Q::Difference } else { Q::Sum });
            }
            scaled(value, u64::from(scale))
        }
        Op::Mul((ca, sa), (cb, sb)) => {
            let working = u64::from(sa) + u64::from(sb);
            if !lift_ok(working) {
                return Some(Q::Exponent);
            }
            let value = big(ca).mul(&big(cb));
            if !fits(&value) {
                return Some(Q::Product);
            }
            scaled(value, working)
        }
        Op::Neg((c, s)) | Op::Round((c, s)) => {
            let working = u64::from(s);
            if !lift_ok(working) {
                return Some(Q::Exponent);
            }
            let value = if matches!(op, Op::Neg(..)) {
                big(c).neg()
            } else {
                big(c)
            };
            if !fits(&value) {
                return Some(Q::Negation);
            }
            scaled(value, working)
        }
        Op::Div((ca, sa), (cb, sb)) => {
            if cb == 0 {
                return None;
            }
            let up = target + u64::from(sb);
            let (kn, kd) = (
                up.saturating_sub(u64::from(sa)),
                u64::from(sa).saturating_sub(up),
            );
            if kn > 38 || kd > 38 {
                return Some(Q::Exponent);
            }
            let (n, dd) = (big(ca).mul(&pow10(kn)), big(cb).mul(&pow10(kd)));
            if !fits(&n) {
                return Some(Q::Numerator);
            }
            if !fits(&dd) {
                return Some(Q::Denominator);
            }
            let quotient = Rational::new(n, dd).unwrap();
            if !fits(quotient.numerator()) || !fits(quotient.denominator()) {
                return Some(Q::CanonicalQuotient);
            }
            if quotient.is_integer() {
                return None;
            }
            doubled_model(t, quotient.numerator(), quotient.denominator())
        }
        Op::Order(..) => None,
    }
}

fn doubled_model(t: Ty, numerator: &Integer, denominator: &Integer) -> Option<Q> {
    if !matches!(t.mode, M::NearestEven | M::NearestAway) {
        return None;
    }
    let (_, remainder) = numerator.div_mod_floor(denominator.try_into().unwrap());
    (!fits(&remainder.add(&remainder))).then_some(Q::DoubledRemainder)
}

fn place_model(t: Ty, value: Integer, working: u64) -> Option<Q> {
    let target = t.smax;
    if working <= target {
        let lifted = value.mul(&pow10(target - working));
        return (!fits(&lifted)).then_some(Q::RetainedCoefficient);
    }
    if value.is_zero() {
        return None;
    }
    let excess = working - target;
    if excess > digits(&value) {
        return None;
    }
    if excess > 38 {
        return Some(Q::PowerOfTen);
    }
    let units = Rational::new(value, pow10(excess)).unwrap();
    if units.is_integer() {
        return None;
    }
    doubled_model(t, units.numerator(), units.denominator())
}

/// Compare the native route with the owner route on one case: equal on
/// every observable except a capacity refusal, which is checked against the
/// independently modelled quantity.
fn agree(t: Ty, op: Op, limits: ScalarLimits) -> bool {
    let expected = model(t, op);
    match run_native(t, op, limits) {
        Err(refusal) => {
            assert_eq!(Some(refusal.quantity), expected, "{t:?} {op:?}");
            false
        }
        Ok((native, native_seen)) => {
            assert_eq!(expected, None, "{t:?} {op:?}");
            let (owner, owner_seen) = run_owner(t, op, limits);
            assert_eq!(native, owner, "{t:?} {op:?} {limits:?}");
            assert_eq!(native_seen, owner_seen, "{t:?} {op:?} {limits:?}");
            true
        }
    }
}

#[test]
#[trace("TC-951", "FR-403-AC-11")]
fn native_and_owner_agree_over_the_boundary_set() {
    let values = operands();
    let (mut compared, mut refused) = (0_u32, 0_u32);
    for t in types() {
        for &a in &values {
            for &b in &values {
                for op in all_ops(a, b) {
                    if agree(t, op, bounded()) {
                        compared += 1;
                    } else {
                        refused += 1;
                    }
                }
            }
        }
    }
    // Both sides of the single intended difference are exercised.
    assert!(compared > 10_000 && refused > 1_000, "{compared} {refused}");
}

/// The cases swept across limits: every literal case of the criteria.
fn swept() -> Vec<(Ty, Op)> {
    let ne = ty(-100, 100, 0, 2, M::NearestEven);
    let mut cases = vec![
        (EXACT2, Op::Add((5, 1), (5, 1))),
        (EXACT2, Op::Sub((5, 1), (1, 1))),
        (EXACT2, Op::Mul((5, 1), (4, 1))),
        (EXACT2, Op::Neg((0, 0))),
        (EXACT2, Op::Round((5, 1))),
        (EXACT2, Op::Div((1, 0), (4, 0))),
        (EXACT2, Op::Div((1, 0), (0, 3))),
        (EXACT2, Op::Div((1, 0), (3, 0))),
        (EXACT2, Op::Add((100, 0), (1, 0))),
        (ty(0, 10_000, 0, 3, M::Exact), Op::Add((1, 0), (1, 3))),
        (ne, Op::Div((1, 0), (8, 0))),
        (ne, Op::Div((3, 0), (9, 0))),
        (ne, Op::Round((-125, 3))),
        (ty(0, 100, 0, 0, M::NearestAway), Op::Round((1005, 1))),
        (ty(0, 100, 0, 2, M::TowardPositive), Op::Mul((1, 2), (1, 2))),
        (
            ty(0, 1, 0, 0, M::NearestEven),
            Op::Div((P126 - 1, 0), (P126, 0)),
        ),
        (
            ty(0, 100, 0, 2, M::TowardPositive),
            Op::Mul((1, 4000), (1, 4000)),
        ),
    ];
    for operator in [O::Less, O::LessOrEqual, O::Greater, O::GreaterOrEqual] {
        for (a, b) in [
            ((10, 1), (2, 0)),
            ((2, 0), (1, 38)),
            ((1, 0), (1, 39)),
            ((-1, 0), (-10, 1)),
        ] {
            cases.push((EXACT2, Op::Order(operator, a, b)));
        }
    }
    cases
}

/// Limits at sufficient, exact and one-less of each kind at each point reached.
fn limit_variants(t: Ty, op: Op) -> Vec<ScalarLimits> {
    let generous = limits_of(u64::MAX);
    let (_, observed) = run_owner(t, op, bounded());
    let mut variants = vec![generous];
    let total_work = observed.events.len() as u64;
    for work in 0..=total_work {
        variants.push(ScalarLimits {
            work_units: work,
            ..generous
        });
    }
    for event in &observed.events {
        for (kind, amount) in &event.sizes {
            let amount = amount.to_u64().unwrap();
            for limit in [amount, amount.saturating_sub(1)] {
                let mut limits = generous;
                set(&mut limits, *kind, limit);
                variants.push(limits);
            }
        }
        if event.result_units.to_u64().unwrap() > 0 {
            variants.push(ScalarLimits {
                result_units: 0,
                ..generous
            });
            variants.push(ScalarLimits {
                result_units: 1,
                ..generous
            });
        }
    }
    variants.retain(|limits| {
        get(limits, K::ScaleExpansion) <= 4096 || get(limits, K::ScaleExpansion) == u64::MAX
    });
    variants
        .into_iter()
        .map(|mut limits| {
            // Keep the owner from forming a huge power of ten: its expansion
            // is charged before it is formed.
            limits.scale_expansion = limits.scale_expansion.min(4096);
            limits
        })
        .collect()
}

#[test]
#[trace("TC-951", "FR-403-AC-7")]
fn charges_hold_at_every_limit_on_fresh_and_cumulative_meters() {
    for (t, op) in swept() {
        let variants = limit_variants(t, op);
        assert!(variants.len() > 3);
        let mut stopped = 0;
        for limits in &variants {
            if !agree(t, op, *limits) {
                continue;
            }
            let (result, _) = run_native(t, op, *limits).unwrap();
            if matches!(
                result,
                Res::Arith(Outcome::Incomplete(_)) | Res::Ord(Outcome::Incomplete(_))
            ) {
                stopped += 1;
            }
        }
        assert!(stopped > 0, "{t:?} {op:?}: no limit denied a charge");
    }
    // One cumulative Meter across the whole sequence, native beside owner.
    let sequence: Vec<(Ty, Op)> = swept();
    for work in [1_u64, 3, 7, 12, 20, 33, 50, 80, 1000] {
        let limits = ScalarLimits {
            work_units: work,
            ..bounded()
        };
        let (mut native_meter, mut owner_meter) = (recording(limits), recording(limits));
        for &(t, op) in &sequence {
            let Ok(native) = native_on(t, op, &mut native_meter) else {
                continue;
            };
            let owner = owner_on(t, op, &mut owner_meter);
            assert_eq!(native, owner, "{t:?} {op:?} {work}");
        }
        assert_eq!(seen(&mut native_meter), seen(&mut owner_meter));
    }
}

#[test]
#[trace("TC-951", "FR-403-AC-7")]
fn a_denied_charge_makes_no_later_charge() {
    // one less than the decimal.arithmetic amount (4 integer bits)
    let limits = ScalarLimits {
        integer_bits: 3,
        ..limits_of(u64::MAX)
    };
    let (result, observed) = run_native(EXACT2, Op::Add((5, 1), (5, 1)), limits).unwrap();
    match result {
        Res::Arith(Outcome::Incomplete(record)) => {
            assert_eq!(record.charge_point, P::DecimalArithmetic);
            assert_eq!(record.limit_kind, K::IntegerBits);
        }
        other => panic!("{other:?}"),
    }
    let last = observed.events.last().unwrap();
    assert_eq!(last.point, P::DecimalArithmetic);
    assert_eq!(observed.events.len(), 3);
}

#[test]
#[trace("TC-951", "FR-403-AC-11")]
fn a_capacity_refusal_is_never_another_outcome() {
    let wide = ty(-P126, P126, 0, 0, M::Exact);
    let refusal = native_ty(wide).admit(Nop::Add(d((P126, 0)), d((P126, 0))));
    assert_eq!(
        refusal.unwrap_err(),
        NativeDecimalCapacityRefusal { quantity: Q::Sum }
    );
    // the owner decides the same operation: a completed value outside the type
    let (owner, _) = run_owner(wide, Op::Add((P126, 0), (P126, 0)), limits_of(u64::MAX));
    assert!(matches!(
        owner,
        Res::Arith(Outcome::Refused(Refusal::DecimalOutOfDomain { .. }))
    ));
    // an input the owner refuses for a domain reason that admission admits is
    // refused natively with the same domain refusal
    let (native, _) = run_native(EXACT2, Op::Add((100, 0), (1, 0)), limits_of(u64::MAX)).unwrap();
    let (owner, _) = run_owner(EXACT2, Op::Add((100, 0), (1, 0)), limits_of(u64::MAX));
    assert_eq!(native, owner);
    assert!(matches!(
        native,
        Res::Arith(Outcome::Refused(Refusal::DecimalOutOfDomain { .. }))
    ));
}

#[test]
#[trace("TC-951", "FR-403-AC-10")]
fn results_and_losses_are_copy_and_admission_does_not_allocate() {
    fn is_copy<X: Copy>() {}
    is_copy::<quire_exact::NativeDecimalResult>();
    is_copy::<quire_exact::NativeDecimalLoss>();
    is_copy::<quire_exact::AdmittedNativeDecimalOperation>();
    is_copy::<Nop>();
    let cases: Vec<(T, Nop)> = types()
        .into_iter()
        .flat_map(|t| {
            operands().into_iter().flat_map(move |a| {
                [(1, 0), (0, 0), (-3, 0), (MAX, 0), (MIN, 0), (7, 2)]
                    .map(move |b| (native_ty(t), Nop::Divide(d(a), d(b))))
            })
        })
        .collect();
    let info = allocation_counter::measure(|| {
        for (native, op) in &cases {
            let _ = native.admit(*op);
        }
    });
    assert_eq!(info.count_total, 0);
    let more: Vec<(T, Nop)> = types()
        .into_iter()
        .flat_map(|t| {
            operands().into_iter().flat_map(move |a| {
                operands().into_iter().take(12).flat_map(move |b| {
                    [
                        Nop::Add(d(a), d(b)),
                        Nop::Subtract(d(a), d(b)),
                        Nop::Multiply(d(a), d(b)),
                        Nop::Negate(d(a)),
                        Nop::Round(d(a)),
                    ]
                    .map(move |op| (native_ty(t), op))
                })
            })
        })
        .collect();
    let info = allocation_counter::measure(|| {
        for (native, op) in &more {
            let _ = native.admit(*op);
        }
    });
    assert_eq!(info.count_total, 0);
}

// ---------------------------------------------------------------------------
// The shape of the proof-path bodies (AC-10, Behaviors 30 to 32).
// ---------------------------------------------------------------------------

/// The source with comment lines removed.
fn code_of(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        // a trait impl header is not a loop
        .map(|line| {
            if line.starts_with("impl") {
                line.replace(" for ", " as ")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

const ALLOWED_METHODS: &[&str] = &[
    "checked_add",
    "checked_mul",
    "checked_neg",
    "checked_sub",
    "cmp",
    "div_euclid",
    "leading_zeros",
    "max",
    "min",
    "ok_or",
    "rem_euclid",
    "results",
    "reverse",
    "saturating_sub",
    "signum",
    "size",
    "charge",
    "then_some",
    "trailing_zeros",
    "unsigned_abs",
    "wrapping_neg",
    "admits",
    "holds",
    "try_from",
    "ok",
    "then",
    "from",
    "is_some",
    "is_multiple_of",
    "is_none",
];

/// What in proof-path source breaks FR-403 Behaviors 30 to 32.
fn violations(source: &str) -> Vec<String> {
    let text = code_of(source);
    let mut found = Vec::new();
    for token in [
        "cfg(kani)",
        "cfg_attr",
        "Vec",
        "Box",
        "String",
        "alloc",
        "dyn ",
        "..",
    ] {
        if text.contains(token) {
            found.push(token.to_string());
        }
    }
    let words = [
        "for",
        "while",
        "loop",
        "Integer",
        "Decimal",
        "Rational",
        "iter",
        "into_iter",
        "map",
        "fold",
        "collect",
    ];
    for word in text.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
        if words.contains(&word) {
            found.push(word.to_string());
        }
    }
    for piece in text.split('.').skip(1) {
        let name: String = piece
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if piece[name.len()..].starts_with('(') && !ALLOWED_METHODS.contains(&name.as_str()) {
            found.push(format!(".{name}("));
        }
    }
    found.extend(recursion(&text));
    found
}

fn function_names(text: &str) -> Vec<String> {
    text.split("fn ")
        .skip(1)
        .map(|piece| {
            piece
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>()
        })
        .filter(|name| !name.is_empty())
        .collect()
}

fn body_of(text: &str, name: &str) -> String {
    let start = text
        .find(&format!("fn {name}("))
        .or_else(|| text.find(&format!("fn {name}<")))
        .unwrap_or_else(|| panic!("fn {name}"));
    let open = start + text[start..].find('{').unwrap();
    let mut depth = 0;
    for (offset, ch) in text[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return text[open..open + offset + 1].to_string();
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced {name}");
}

fn calls(text: &str, caller: &str, callee: &str) -> usize {
    let body = body_of(text, caller);
    body.match_indices(&format!("{callee}("))
        .filter(|(at, _)| {
            body[..*at]
                .chars()
                .next_back()
                .is_none_or(|c| !(c.is_alphanumeric() || c == '_'))
        })
        .count()
}

/// Any cycle in the call graph of the named functions is recursion.
fn recursion(text: &str) -> Vec<String> {
    let names = function_names(text);
    let mut found = Vec::new();
    for start in &names {
        let mut seen: Vec<&String> = Vec::new();
        let mut stack: Vec<&String> = names
            .iter()
            .filter(|callee| calls(text, start, callee) > 0)
            .collect();
        while let Some(next) = stack.pop() {
            if next == start {
                found.push(format!("recursion through {start}"));
                break;
            }
            if seen.contains(&next) {
                continue;
            }
            seen.push(next);
            stack.extend(names.iter().filter(|callee| calls(text, next, callee) > 0));
        }
    }
    found
}

const OPERATIONS: &str = include_str!("../src/native_decimal/operations.rs");

#[test]
#[trace("TC-951", "FR-403-AC-10")]
fn source_inspection_of_the_proof_path_bodies() {
    assert_eq!(violations(OPERATIONS), Vec::<String>::new());
    // the ordering and the gcd live in the scanned file
    let text = code_of(OPERATIONS);
    for name in ["gcd256", "order_values", "evaluate", "order", "settle"] {
        assert!(text.contains(&format!("fn {name}(")), "{name}");
    }
}

#[test]
#[trace("TC-951", "FR-403-AC-10")]
fn the_gcd_is_exactly_256_fixed_steps() {
    let text = code_of(OPERATIONS);
    let mut steps = std::collections::HashMap::new();
    steps.insert("gcd_step".to_string(), 1_usize);
    let ladder = [2_usize, 4, 8, 16, 32, 64, 128, 256];
    let mut previous = "gcd_step".to_string();
    for width in ladder {
        let name = format!("gcd_steps_{width}");
        assert_eq!(calls(&text, &name, &previous), 2, "{name}");
        let total = steps[&previous] * 2;
        steps.insert(name.clone(), total);
        previous = name;
    }
    assert_eq!(steps["gcd_steps_256"], 256);
    assert_eq!(calls(&text, "gcd256", "gcd_steps_256"), 1);
    // the step has no loop and the state is (u, v, g)
    assert!(body_of(&text, "gcd_step").contains("(u, v, g)"));
}

#[test]
#[trace("TC-951", "FR-403-AC-10")]
fn the_inspection_flags_every_forbidden_shape() {
    for (bad, flag) in [
        ("fn f() { for i in 0..3 {} }", "for"),
        ("fn f() { while true {} }", "while"),
        ("fn f() { loop {} }", "loop"),
        ("fn f(x: Vec<u8>) {}", "Vec"),
        ("fn f() { Box::new(1); }", "Box"),
        ("#[cfg(kani)]\nfn f() {}", "cfg(kani)"),
        ("fn f(x: Integer) {}", "Integer"),
        ("fn f(x: Decimal) {}", "Decimal"),
        ("fn f(x: Rational) {}", "Rational"),
        ("fn f(x: u8) { x.iter().sum(); }", ".iter("),
        ("fn f(x: u8) { x.fold(0, g); }", "fold"),
        ("fn f(x: u8) { x.zip(y); }", ".zip("),
        ("fn f() { f() }", "recursion through f"),
        ("fn f() { g() }\nfn g() { f() }", "recursion through f"),
    ] {
        let found = violations(bad);
        assert!(
            found.iter().any(|item| item.contains(flag)),
            "{bad} -> {found:?}"
        );
    }
    // and ordinary native arithmetic is not flagged
    assert!(
        violations("fn f(x: u128) -> u128 { x.checked_mul(2).unwrap_or(1) }")
            .iter()
            .any(|item| item.contains("unwrap_or"))
    );
    assert!(violations("fn f(x: u128) -> Option<u128> { x.checked_mul(2) }").is_empty());
}
