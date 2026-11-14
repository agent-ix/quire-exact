// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native Rational carrier (FR-408, TC-956) against literals and the
//! owner route: `Rational::new`, `evaluate_rational_arithmetic`,
//! `order_numbers` and `planned_equality`.
use ix_trace_rs::trace;
use quire_exact::{
    evaluate_rational_arithmetic, native_rational_add, native_rational_equal,
    native_rational_multiply, native_rational_order, native_rational_subtract, order_numbers,
    planned_equality, ChargeAdmission, ChargeEvent, ChargePoint, Integer, LimitKind, Meter,
    NativeRational, NativeRationalCapacity as Capacity, NativeRationalRefusal as Refusal,
    OrderedOperands, OrderingOperator, Outcome, Rational, RationalArithmetic, ScalarLimits, Value,
};

const ORDERINGS: [OrderingOperator; 4] = [
    OrderingOperator::Less,
    OrderingOperator::LessOrEqual,
    OrderingOperator::Greater,
    OrderingOperator::GreaterOrEqual,
];

fn limits(value: u64) -> ScalarLimits {
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

fn limit_of(limits: &mut ScalarLimits, kind: LimitKind) -> &mut u64 {
    match kind {
        LimitKind::IntegerBits => &mut limits.integer_bits,
        LimitKind::ValueOccurrences => &mut limits.value_occurrences,
        LimitKind::WorkUnits => &mut limits.work_units,
        LimitKind::ResultUnits => &mut limits.result_units,
        other => panic!("{other:?} is not charged by the rational path"),
    }
}

const USED: [LimitKind; 4] = [
    LimitKind::IntegerBits,
    LimitKind::ValueOccurrences,
    LimitKind::WorkUnits,
    LimitKind::ResultUnits,
];

fn nat(numerator: i64, denominator: i64) -> NativeRational {
    NativeRational::from_i64_parts(numerator, denominator).unwrap()
}

fn big(value: i128) -> NativeRational {
    NativeRational::from_rational(&owner(value, 1)).unwrap()
}

fn owner(numerator: i128, denominator: i128) -> Rational {
    Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
}

fn parts(value: &NativeRational) -> (i128, i128) {
    (value.numerator(), value.denominator())
}

fn completed<T: std::fmt::Debug>(outcome: Result<Outcome<T>, Refusal>) -> T {
    outcome.unwrap().completed().expect("completed")
}

fn generous() -> Meter {
    Meter::new(limits(u64::MAX))
}

fn recording(limits: ScalarLimits) -> Meter {
    let mut meter = Meter::new(limits);
    meter.enable_charge_capture(64, 256);
    meter
}

fn events(meter: &mut Meter) -> Vec<ChargeEvent> {
    meter
        .disable_charge_capture()
        .unwrap()
        .unwrap()
        .events()
        .to_vec()
}

type Literal = (ChargePoint, u64, u64, Vec<(LimitKind, u64)>);

fn literal(events: &[ChargeEvent]) -> Vec<Literal> {
    let small = |value: &Integer| (0..=u64::from(u16::MAX)).find(|n| Integer::from(*n) == *value);
    events
        .iter()
        .map(|event| {
            assert_eq!(event.admission, ChargeAdmission::Admitted);
            (
                event.point,
                small(&event.work_units).unwrap(),
                small(&event.result_units).unwrap(),
                event
                    .sizes
                    .iter()
                    .map(|(kind, amount)| (*kind, small(amount).unwrap()))
                    .collect(),
            )
        })
        .collect()
}

fn arithmetic_literal(operands: u64, arithmetic: u64, normalize: u64) -> Vec<Literal> {
    use ChargePoint as P;
    use LimitKind as K;
    vec![
        (
            P::RationalArithmeticOperands,
            1,
            0,
            vec![(K::IntegerBits, operands), (K::ValueOccurrences, 2)],
        ),
        (
            P::RationalArithmeticArithmetic,
            1,
            0,
            vec![(K::IntegerBits, arithmetic)],
        ),
        (
            P::RationalArithmeticNormalize,
            1,
            0,
            vec![(K::IntegerBits, normalize)],
        ),
        (P::RationalArithmeticResultRetain, 1, 1, vec![]),
    ]
}

// ---- AC-1 ----

#[test]
#[trace("TC-956", "FR-408-AC-1")]
fn carrier_is_copy_without_drop_glue_and_allocation_free() {
    fn copy<T: Copy>() {}
    copy::<NativeRational>();
    copy::<Refusal>();
    copy::<Capacity>();
    assert!(!std::mem::needs_drop::<NativeRational>());
    assert_eq!(std::mem::size_of::<NativeRational>(), 32);
    let values = [nat(1, 2), nat(-1, 3), nat(i64::MIN, i64::MAX), nat(0, 1)];
    let info = allocation_counter::measure(|| {
        let mut meter = generous();
        for left in &values {
            for right in &values {
                std::hint::black_box(NativeRational::from_i64_parts(3, 7).ok());
                std::hint::black_box((left.numerator(), right.denominator()));
                std::hint::black_box(native_rational_add(left, right, &mut meter).ok());
                std::hint::black_box(native_rational_subtract(left, right, &mut meter).ok());
                std::hint::black_box(native_rational_multiply(left, right, &mut meter).ok());
                for operator in ORDERINGS {
                    std::hint::black_box(
                        native_rational_order(operator, left, right, &mut meter).ok(),
                    );
                }
                std::hint::black_box(native_rational_equal(left, right, &mut meter).completed());
            }
        }
    });
    assert_eq!(info.count_total, 0);
}

// ---- AC-2 ----

#[test]
#[trace("TC-956", "FR-408-AC-2")]
fn from_i64_parts_admits_canonical_pairs_and_refuses_the_rest_in_order() {
    for (n, d) in [
        (0, 1),
        (1, 1),
        (-1, 1),
        (1, 2),
        (-1, 2),
        (i64::MIN, 1),
        (1, i64::MAX),
        (i64::MIN, i64::MAX),
    ] {
        let built = NativeRational::from_i64_parts(n, d).unwrap();
        assert_eq!(parts(&built), (i128::from(n), i128::from(d)));
    }
    for (n, d) in [(1, 0), (0, 0), (2, 0), (-5, 0)] {
        assert_eq!(
            NativeRational::from_i64_parts(n, d),
            Err(Refusal::ZeroDenominator)
        );
    }
    for (n, d) in [
        (2, 4),
        (1, -2),
        (-1, -2),
        (0, 2),
        (0, -1),
        (i64::MIN, -1),
        (1, i64::MIN),
    ] {
        assert_eq!(
            NativeRational::from_i64_parts(n, d),
            Err(Refusal::NonCanonical),
            "({n}, {d})"
        );
    }
    let _: fn(i64, i64) -> Result<NativeRational, Refusal> = NativeRational::from_i64_parts;
}

// ---- AC-3 ----

fn cg_domain() -> Vec<NativeRational> {
    let mut all = Vec::new();
    for n in -9_i64..=9 {
        for d in 1_i64..=9 {
            if let Ok(value) = NativeRational::from_i64_parts(n, d) {
                all.push(value);
            }
        }
    }
    all
}

fn two_pow(exponent: u32) -> Integer {
    Integer::one().shifted_left(u64::from(exponent))
}

fn boundary() -> Vec<NativeRational> {
    let mut all = Vec::new();
    for (n, d) in [
        (0, 1),
        (1, 1),
        (-1, 1),
        (1, 2),
        (-1, 3),
        (2, 3),
        (9, 1),
        (-9, 1),
        (i64::MAX, 1),
        (i64::MIN, 1),
        (1, i64::MAX),
        (-1, i64::MAX),
        (i64::MIN, i64::MAX),
        (i64::MAX, 2),
    ] {
        all.push(nat(n, d));
    }
    let wide = [
        Rational::new(two_pow(126).sub(&Integer::one()), two_pow(126)).unwrap(),
        Rational::new(two_pow(100), Integer::one()).unwrap(),
        Rational::new(Integer::one(), two_pow(100)).unwrap(),
        owner(1, 3_i128.pow(80)),
        owner(i128::MAX, 1),
        owner(i128::MIN, 1),
        owner(-1, i128::MAX),
    ];
    for value in &wide {
        all.push(NativeRational::from_rational(value).unwrap());
    }
    all
}

#[test]
#[trace("TC-956", "FR-408-AC-3")]
fn owner_conversion_round_trips_and_refuses_oversize_parts() {
    assert_eq!(cg_domain().len(), 111);
    for value in cg_domain().into_iter().chain(boundary()) {
        let rational = value.to_rational();
        assert_eq!(
            rational,
            owner(value.numerator(), value.denominator()),
            "{value:?}"
        );
        assert_eq!(NativeRational::from_rational(&rational), Ok(value));
    }
    let too_big = Rational::new(two_pow(127), Integer::one()).unwrap();
    assert_eq!(
        NativeRational::from_rational(&too_big),
        Err(Refusal::Capacity(Capacity::Input))
    );
    let too_small_denominator = Rational::new(Integer::one(), two_pow(127)).unwrap();
    assert_eq!(
        NativeRational::from_rational(&too_small_denominator),
        Err(Refusal::Capacity(Capacity::Input))
    );
    assert_eq!(parts(&nat(0, 1)), (0, 1));
    assert_eq!(NativeRational::from_rational(&owner(0, 7)), Ok(nat(0, 1)));
}

// ---- AC-4 ----

#[test]
#[trace("TC-956", "FR-408-AC-4")]
fn add_subtract_multiply_complete_to_the_reduced_value() {
    type Case = (&'static str, (i64, i64), (i64, i64), (i128, i128));
    let cases: [Case; 10] = [
        ("add", (1, 2), (1, 3), (5, 6)),
        ("add", (1, 2), (1, 2), (1, 1)),
        ("sub", (1, 2), (1, 2), (0, 1)),
        ("sub", (-1, 2), (1, 3), (-5, 6)),
        ("mul", (2, 3), (3, 2), (1, 1)),
        ("mul", (0, 1), (7, 9), (0, 1)),
        ("add", (-9, 1), (-9, 1), (-18, 1)),
        ("add", (9, 1), (9, 1), (18, 1)),
        ("add", (i64::MAX, 1), (i64::MAX, 1), ((1_i128 << 64) - 2, 1)),
        ("mul", (i64::MIN, 1), (i64::MIN, 1), (1_i128 << 126, 1)),
    ];
    for (op, (a, p), (c, q), expected) in cases {
        let (left, right) = (nat(a, p), nat(c, q));
        let mut meter = generous();
        let value = match op {
            "add" => completed(native_rational_add(&left, &right, &mut meter)),
            "sub" => completed(native_rational_subtract(&left, &right, &mut meter)),
            _ => completed(native_rational_multiply(&left, &right, &mut meter)),
        };
        assert_eq!(parts(&value), expected, "{op} {a}/{p} {c}/{q}");
    }
    for left in boundary() {
        for right in boundary() {
            let mut meter = generous();
            for result in [
                native_rational_add(&left, &right, &mut meter),
                native_rational_subtract(&left, &right, &mut meter),
                native_rational_multiply(&left, &right, &mut meter),
            ] {
                if let Ok(Outcome::Completed(value)) = result {
                    assert!(value.denominator() > 0);
                    assert_eq!(
                        value,
                        NativeRational::from_rational(&value.to_rational()).unwrap()
                    );
                }
            }
        }
    }
}

// ---- AC-5 ----

#[test]
#[trace("TC-956", "FR-408-AC-5")]
fn ordering_and_equality_follow_the_numeric_order() {
    let order = |operator, left: NativeRational, right: NativeRational| {
        completed(native_rational_order(
            operator,
            &left,
            &right,
            &mut generous(),
        ))
    };
    use OrderingOperator as O;
    assert!(order(O::Less, nat(1, 3), nat(1, 2)));
    assert!(order(O::LessOrEqual, nat(1, 2), nat(1, 2)));
    assert!(order(O::Greater, nat(-1, 2), nat(-2, 3)));
    assert!(order(O::GreaterOrEqual, nat(7, 3), nat(5, 8)));
    assert!(order(O::GreaterOrEqual, nat(1, 2), nat(1, 2)));
    assert!(!order(O::Less, nat(1, 2), nat(1, 2)));
    assert!(!order(O::Greater, nat(1, 2), nat(1, 2)));
    let equal = |left: NativeRational, right: NativeRational| {
        native_rational_equal(&left, &right, &mut generous())
            .completed()
            .unwrap()
    };
    assert!(equal(nat(1, 2), nat(1, 2)));
    assert!(!equal(nat(1, 2), nat(2, 3)));
    assert!(!equal(nat(1, 2), nat(-1, 2)));
    assert!(!equal(nat(0, 1), nat(1, 1)));
}

// ---- AC-6 ----

fn capacity_of(result: Result<Outcome<impl std::fmt::Debug>, Refusal>) -> Capacity {
    match result {
        Err(Refusal::Capacity(cause)) => cause,
        other => panic!("expected a capacity refusal, got {other:?}"),
    }
}

fn assert_untouched(meter: &Meter) {
    assert_eq!(meter.admission_count(), 0);
    for kind in LimitKind::ALL {
        assert_eq!(meter.consumed(kind), 0);
    }
}

#[test]
#[trace("TC-956", "FR-408-AC-6")]
fn capacity_names_the_first_failing_quantity_and_charges_nothing() {
    let mut zero = Meter::new(limits(0));
    let frac = |n: i128, d: i128| NativeRational::from_rational(&owner(n, d)).unwrap();
    let (a, b) = (big(1 << 100), big(1 << 100));
    assert_eq!(
        capacity_of(native_rational_multiply(&a, &b, &mut zero)),
        Capacity::LeftProduct
    );
    let (a, b) = (frac(1, 1 << 100), frac(1, 3_i128.pow(80)));
    assert_eq!(
        capacity_of(native_rational_add(&a, &b, &mut zero)),
        Capacity::Denominator
    );
    let max = big(i128::MAX);
    assert_eq!(
        capacity_of(native_rational_add(&max, &max, &mut zero)),
        Capacity::Combination
    );
    assert_eq!(
        capacity_of(native_rational_subtract(&max, &big(-i128::MAX), &mut zero)),
        Capacity::Combination
    );
    let (a, b) = (frac(1, 1 << 100), big(1 << 100));
    assert_eq!(
        capacity_of(native_rational_add(&a, &b, &mut zero)),
        Capacity::RightProduct
    );
    assert_eq!(
        capacity_of(native_rational_order(
            OrderingOperator::Less,
            &a,
            &b,
            &mut zero
        )),
        Capacity::RightProduct
    );
    assert_eq!(
        capacity_of(native_rational_order(
            OrderingOperator::Less,
            &big(1 << 126),
            &frac(4, 5),
            &mut zero
        )),
        Capacity::LeftProduct
    );
    assert_untouched(&zero);
    // A representable difference is never refused: -1/1 - i128::MIN/1.
    let value = completed(native_rational_subtract(
        &nat(-1, 1),
        &big(i128::MIN),
        &mut generous(),
    ));
    assert_eq!(parts(&value), (i128::MAX, 1));
    // Equality never refuses.
    assert_eq!(
        native_rational_equal(&max, &big(i128::MIN), &mut generous()).completed(),
        Some(false)
    );
    // Inside signed 64 bits nothing refuses.
    let wide = [
        nat(i64::MIN, i64::MAX),
        nat(i64::MAX, 1),
        nat(1, i64::MAX),
        nat(i64::MIN, 1),
    ];
    for left in &wide {
        for right in &wide {
            let mut meter = generous();
            assert!(native_rational_add(left, right, &mut meter).is_ok());
            assert!(native_rational_subtract(left, right, &mut meter).is_ok());
            assert!(native_rational_multiply(left, right, &mut meter).is_ok());
            for operator in ORDERINGS {
                assert!(native_rational_order(operator, left, right, &mut meter).is_ok());
            }
        }
    }
}

// ---- AC-7 ----

#[test]
#[trace("TC-956", "FR-408-AC-7")]
fn charges_match_the_literal_schedule() {
    let run = |left: NativeRational, right: NativeRational| {
        let mut meter = recording(limits(u64::MAX));
        completed(native_rational_add(&left, &right, &mut meter));
        (literal(&events(&mut meter)), meter)
    };
    let (seen, meter) = run(nat(1, 2), nat(1, 3));
    assert_eq!(seen, arithmetic_literal(2, 4, 3));
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 4);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    // The normalize amount uses the unreduced 4/4, not the reduced 1/1.
    assert_eq!(run(nat(1, 2), nat(1, 2)).0, arithmetic_literal(2, 4, 3));
    // `N = max(4 + 1, 4 + 1) + 1 = 6`: a missing `+ 1` would give 5.
    assert_eq!(run(nat(9, 1), nat(9, 1)).0, arithmetic_literal(4, 6, 5));
    let mut meter = recording(limits(u64::MAX));
    completed(native_rational_multiply(&nat(2, 3), &nat(3, 2), &mut meter));
    assert_eq!(literal(&events(&mut meter)), arithmetic_literal(2, 4, 3));
    let mut meter = recording(limits(u64::MAX));
    completed(native_rational_subtract(&nat(1, 2), &nat(1, 3), &mut meter));
    assert_eq!(literal(&events(&mut meter)), arithmetic_literal(2, 4, 3));
    // Ordering: operands, arithmetic, result-retain.
    let mut meter = recording(limits(u64::MAX));
    assert!(completed(native_rational_order(
        OrderingOperator::Less,
        &nat(1, 3),
        &nat(1, 2),
        &mut meter
    )));
    assert_eq!(
        literal(&events(&mut meter)),
        vec![
            (
                ChargePoint::OrderingOperands,
                1,
                0,
                vec![
                    (LimitKind::IntegerBits, 2),
                    (LimitKind::ValueOccurrences, 2)
                ]
            ),
            (
                ChargePoint::OrderingArithmetic,
                1,
                0,
                vec![(LimitKind::IntegerBits, 3)]
            ),
            (ChargePoint::OrderingResultRetain, 1, 1, vec![]),
        ]
    );
}

fn with_limit(kind: LimitKind, value: u64) -> Meter {
    let mut all = limits(u64::MAX);
    *limit_of(&mut all, kind) = value;
    Meter::new(all)
}

#[test]
#[trace("TC-956", "FR-408-AC-7")]
fn denial_stops_at_the_named_charge_and_consumes_no_result_unit() {
    let (left, right) = (nat(1, 2), nat(1, 3));
    let denied = |kind, value| {
        let mut meter = with_limit(kind, value);
        let record = match native_rational_add(&left, &right, &mut meter) {
            Ok(Outcome::Incomplete(record)) => record,
            other => panic!("expected incomplete, got {other:?}"),
        };
        (record, meter)
    };
    // Exact limits complete.
    for (kind, exact) in [
        (LimitKind::IntegerBits, 4),
        (LimitKind::ValueOccurrences, 2),
        (LimitKind::WorkUnits, 4),
        (LimitKind::ResultUnits, 1),
    ] {
        let mut meter = with_limit(kind, exact);
        assert!(matches!(
            native_rational_add(&left, &right, &mut meter),
            Ok(Outcome::Completed(_))
        ));
    }
    let (record, meter) = denied(LimitKind::IntegerBits, 3);
    assert_eq!(
        record.charge_point,
        ChargePoint::RationalArithmeticArithmetic
    );
    assert_eq!((record.limit, record.consumed), (3, 2));
    assert_eq!(record.next_charge, Integer::from(4_u64));
    assert_eq!(meter.admission_count(), 1);
    let (record, meter) = denied(LimitKind::ValueOccurrences, 1);
    assert_eq!(record.charge_point, ChargePoint::RationalArithmeticOperands);
    assert_eq!(meter.admission_count(), 0);
    let (record, meter) = denied(LimitKind::WorkUnits, 3);
    assert_eq!(
        record.charge_point,
        ChargePoint::RationalArithmeticResultRetain
    );
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
    let (record, meter) = denied(LimitKind::ResultUnits, 0);
    assert_eq!(record.limit_kind, LimitKind::ResultUnits);
    assert_eq!(
        record.charge_point,
        ChargePoint::RationalArithmeticResultRetain
    );
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}

// ---- AC-8 ----

#[test]
#[trace("TC-956", "FR-408-AC-8")]
fn equality_charges_the_owner_schedule_and_a_shared_meter_accumulates() {
    let (a, b) = (nat(1, 2), nat(1, 3));
    let consumed = |meter: &Meter| USED.map(|kind| meter.consumed(kind));
    let mut first = recording(limits(u64::MAX));
    completed(native_rational_add(&a, &b, &mut first));
    let first_events = events(&mut first);
    let mut equality = recording(limits(u64::MAX));
    assert_eq!(
        native_rational_equal(&a, &b, &mut equality).completed(),
        Some(false)
    );
    let equality_events = events(&mut equality);
    let owner_value = |n: i128, d: i128| Value::Rational(owner(n, d));
    let mut owner_meter = recording(limits(u64::MAX));
    let _ = planned_equality(&owner_value(1, 2), &owner_value(1, 3), &mut owner_meter);
    assert_eq!(equality_events, events(&mut owner_meter));
    let points: Vec<_> = equality_events.iter().map(|event| event.point).collect();
    assert_eq!(
        points,
        [
            ChargePoint::EqualityPlanForm,
            ChargePoint::EqualityPlan,
            ChargePoint::EqualityPair,
            ChargePoint::EqualityResultRetain
        ]
    );
    // One meter across `a + b`, `b + a`, `=`.
    let mut shared = recording(limits(u64::MAX));
    completed(native_rational_add(&a, &b, &mut shared));
    completed(native_rational_add(&b, &a, &mut shared));
    assert_eq!(
        native_rational_equal(&a, &a, &mut shared).completed(),
        Some(true)
    );
    let total = consumed(&shared);
    let expected = [
        LimitKind::IntegerBits,
        LimitKind::ValueOccurrences,
        LimitKind::WorkUnits,
        LimitKind::ResultUnits,
    ]
    .map(|kind| {
        let single = |meter: &Meter| meter.consumed(kind);
        let mut one = generous();
        completed(native_rational_add(&a, &b, &mut one));
        let mut two = generous();
        native_rational_equal(&a, &a, &mut two).completed().unwrap();
        match kind {
            LimitKind::IntegerBits | LimitKind::ValueOccurrences => single(&one).max(single(&two)),
            _ => 2 * single(&one) + single(&two),
        }
    });
    assert_eq!(total, expected);
    let all = events(&mut shared);
    assert_eq!(all.len(), 4 + 4 + 4);
    assert_eq!(all[..4], first_events[..]);
    // A denial inside the second addition keeps the first addition's charges.
    let mut limited = limits(u64::MAX);
    limited.work_units = 4 + 2;
    let mut meter = recording(limited);
    completed(native_rational_add(&a, &b, &mut meter));
    let record = match native_rational_add(&b, &a, &mut meter) {
        Ok(Outcome::Incomplete(record)) => record,
        other => panic!("expected incomplete, got {other:?}"),
    };
    assert_eq!(
        record.charge_point,
        ChargePoint::RationalArithmeticNormalize
    );
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 6);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    let seen = events(&mut meter);
    assert_eq!(seen[..4], first_events[..]);
    assert_eq!(seen.len(), 4 + 3);
    assert_eq!(seen[6].admission, ChargeAdmission::Denied);
}

// ---- AC-9: host differential against the owner route ----

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Order(OrderingOperator),
    Equal,
}

const OPS: [Op; 8] = [
    Op::Add,
    Op::Subtract,
    Op::Multiply,
    Op::Order(OrderingOperator::Less),
    Op::Order(OrderingOperator::LessOrEqual),
    Op::Order(OrderingOperator::Greater),
    Op::Order(OrderingOperator::GreaterOrEqual),
    Op::Equal,
];

#[derive(Debug, PartialEq)]
enum Answer {
    Value(Rational),
    Flag(bool),
}

#[derive(Debug, PartialEq)]
struct Observed {
    outcome: Outcome<Answer>,
    events: Vec<ChargeEvent>,
    consumed: [u64; 10],
    admissions: u64,
}

fn observe(mut meter: Meter, outcome: Outcome<Answer>) -> Observed {
    Observed {
        outcome,
        consumed: LimitKind::ALL.map(|kind| meter.consumed(kind)),
        admissions: meter.admission_count(),
        events: events(&mut meter),
    }
}

fn map_outcome<T, U>(outcome: Outcome<T>, convert: impl FnOnce(T) -> U) -> Outcome<U> {
    match outcome {
        Outcome::Completed(value) => Outcome::Completed(convert(value)),
        Outcome::Undefined(reason) => Outcome::Undefined(reason),
        Outcome::Refused(reason) => Outcome::Refused(reason),
        Outcome::Incomplete(record) => Outcome::Incomplete(record),
    }
}

fn owner_side(op: Op, left: &NativeRational, right: &NativeRational, mut meter: Meter) -> Observed {
    let (l, r) = (left.to_rational(), right.to_rational());
    let outcome = match op {
        Op::Add => map_outcome(
            evaluate_rational_arithmetic(RationalArithmetic::Add(&l, &r), None, &mut meter),
            Answer::Value,
        ),
        Op::Subtract => map_outcome(
            evaluate_rational_arithmetic(RationalArithmetic::Subtract(&l, &r), None, &mut meter),
            Answer::Value,
        ),
        Op::Multiply => map_outcome(
            evaluate_rational_arithmetic(RationalArithmetic::Multiply(&l, &r), None, &mut meter),
            Answer::Value,
        ),
        Op::Order(operator) => map_outcome(
            order_numbers(operator, OrderedOperands::Rationals(&l, &r), &mut meter),
            Answer::Flag,
        ),
        Op::Equal => map_outcome(
            planned_equality(&Value::Rational(l), &Value::Rational(r), &mut meter),
            Answer::Flag,
        ),
    };
    observe(meter, outcome)
}

fn native_side(
    op: Op,
    left: &NativeRational,
    right: &NativeRational,
    mut meter: Meter,
) -> Result<Observed, Refusal> {
    let value = |result: Result<Outcome<NativeRational>, Refusal>| {
        result.map(|outcome| map_outcome(outcome, |v| Answer::Value(v.to_rational())))
    };
    let outcome = match op {
        Op::Add => value(native_rational_add(left, right, &mut meter)),
        Op::Subtract => value(native_rational_subtract(left, right, &mut meter)),
        Op::Multiply => value(native_rational_multiply(left, right, &mut meter)),
        Op::Order(operator) => native_rational_order(operator, left, right, &mut meter)
            .map(|outcome| map_outcome(outcome, Answer::Flag)),
        Op::Equal => Ok(map_outcome(
            native_rational_equal(left, right, &mut meter),
            Answer::Flag,
        )),
    }?;
    Ok(observe(meter, outcome))
}

/// The capacity cause the checked `i128` arithmetic names, written apart from
/// the carrier.
fn expected_capacity(op: Op, left: &NativeRational, right: &NativeRational) -> Option<Capacity> {
    let (a, p, c, q) = (
        left.numerator(),
        left.denominator(),
        right.numerator(),
        right.denominator(),
    );
    match op {
        Op::Equal => None,
        Op::Multiply => a.checked_mul(c).map_or(Some(Capacity::LeftProduct), |_| {
            p.checked_mul(q)
                .map_or(Some(Capacity::Denominator), |_| None)
        }),
        Op::Order(_) => a.checked_mul(q).map_or(Some(Capacity::LeftProduct), |_| {
            c.checked_mul(p)
                .map_or(Some(Capacity::RightProduct), |_| None)
        }),
        Op::Add | Op::Subtract => {
            let (Some(first), second) = (a.checked_mul(q), c.checked_mul(p)) else {
                return Some(Capacity::LeftProduct);
            };
            let Some(second) = second else {
                return Some(Capacity::RightProduct);
            };
            let combined = if matches!(op, Op::Add) {
                first.checked_add(second)
            } else {
                first.checked_sub(second)
            };
            if combined.is_none() {
                Some(Capacity::Combination)
            } else {
                p.checked_mul(q)
                    .map_or(Some(Capacity::Denominator), |_| None)
            }
        }
    }
}

/// The meters of one comparison: sufficient, then exact and one less for each
/// limit kind the charges use, taken from what the owner route consumed.
fn meters_for(op: Op, left: &NativeRational, right: &NativeRational) -> Vec<ScalarLimits> {
    let probe = owner_side(op, left, right, recording(limits(u64::MAX)));
    let mut all = vec![limits(u64::MAX)];
    for (index, kind) in LimitKind::ALL.into_iter().enumerate() {
        if !USED.contains(&kind) {
            assert_eq!(probe.consumed[index], 0, "{kind:?}");
            continue;
        }
        let used = probe.consumed[index];
        for limit in [Some(used), used.checked_sub(1)].into_iter().flatten() {
            let mut bounded = limits(u64::MAX);
            *limit_of(&mut bounded, kind) = limit;
            all.push(bounded);
        }
    }
    all
}

fn differ(op: Op, left: &NativeRational, right: &NativeRational) -> Option<Capacity> {
    if let Some(cause) = expected_capacity(op, left, right) {
        let mut zero = Meter::new(limits(0));
        let seen = match op {
            Op::Add => native_rational_add(left, right, &mut zero).err(),
            Op::Subtract => native_rational_subtract(left, right, &mut zero).err(),
            Op::Multiply => native_rational_multiply(left, right, &mut zero).err(),
            Op::Order(operator) => native_rational_order(operator, left, right, &mut zero).err(),
            Op::Equal => unreachable!("equality has no capacity case"),
        };
        assert_eq!(
            seen,
            Some(Refusal::Capacity(cause)),
            "{op:?} {left:?} {right:?}"
        );
        assert_untouched(&zero);
        return Some(cause);
    }
    for bounded in meters_for(op, left, right) {
        let expected = owner_side(op, left, right, recording(bounded));
        let actual = native_side(op, left, right, recording(bounded)).expect("no capacity refusal");
        assert_eq!(actual, expected, "{op:?} {left:?} {right:?}");
    }
    None
}

#[test]
#[trace("TC-956", "FR-408-AC-9")]
fn native_path_equals_the_owner_route_over_the_cg_domain() {
    let domain = cg_domain();
    let mut pairs = 0;
    for left in &domain {
        for right in &domain {
            pairs += 1;
            for op in OPS {
                assert_eq!(differ(op, left, right), None);
            }
        }
    }
    assert_eq!(pairs, 12_321);
}

#[test]
#[trace("TC-956", "FR-408-AC-9")]
fn native_path_equals_the_owner_route_over_the_boundary_set() {
    let set = boundary();
    let mut capacity_cases = 0;
    let mut completions = 0;
    for left in &set {
        for right in &set {
            for op in OPS {
                match differ(op, left, right) {
                    Some(_) => capacity_cases += 1,
                    None => completions += 1,
                }
            }
        }
    }
    // Both branches of the width-tiered reduction and the capacity set are
    // exercised.
    assert!(capacity_cases > 0 && completions > 0);
}

#[test]
#[trace("TC-956", "FR-408-AC-9")]
fn the_two_hundred_fifty_two_step_gcd_pair_reduces_like_the_owner() {
    let wide = NativeRational::from_rational(
        &Rational::new(two_pow(126).sub(&Integer::one()), two_pow(126)).unwrap(),
    )
    .unwrap();
    let one = nat(1, 1);
    assert_eq!(differ(Op::Multiply, &wide, &one), None);
    let product = completed(native_rational_multiply(&wide, &one, &mut generous()));
    assert_eq!(product, wide);
    // The unreduced product of two u64-sized fractions takes the narrow tier.
    let (left, right) = (nat(i64::MAX, 3), nat(3, i64::MAX));
    assert_eq!(
        parts(&completed(native_rational_multiply(
            &left,
            &right,
            &mut generous()
        ))),
        (1, 1)
    );
}
