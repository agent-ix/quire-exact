// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public admitted native operations against independent BigInt and real meters.
use ix_trace_rs::trace;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use quire_exact::{
    evaluate_integer_arithmetic, evaluate_native_integer_arithmetic as native,
    evaluate_native_integer_ordering, order_numbers, retain_boolean,
    AdmittedNativeIntegerOperation, Cancel, CancelCause, Charge, ChargePoint, InjectedDenial,
    Integer, IntegerArithmetic, IntegerInterval, LimitKind, Meter, NativeInteger as N,
    NativeIntegerAdmissionError as Error, NativeIntegerEndpoint, NativeIntegerInterval as Range,
    NativeIntegerOperandPosition, NativeIntegerOperands as Args, NativeIntegerOperation as Op,
    NativeIntegerOperationKind as Kind, NativeIntegerPlan as Plan, OrderedOperands,
    OrderingOperator, Outcome, Refusal, ScalarLimits,
};

const KINDS: [LimitKind; 10] = [
    LimitKind::IntegerBits,
    LimitKind::DecimalDigits,
    LimitKind::ScaleExpansion,
    LimitKind::TextInputBytes,
    LimitKind::TextScalars,
    LimitKind::NormalizedScalars,
    LimitKind::UnitEdges,
    LimitKind::ValueOccurrences,
    LimitKind::WorkUnits,
    LimitKind::ResultUnits,
];
fn limits() -> ScalarLimits {
    ScalarLimits {
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
    }
}
fn range(a: i128, b: i128) -> Range {
    Range::new(a, b).unwrap()
}
fn singleton(a: i128) -> Range {
    range(a, a)
}
fn operation(kind: Kind, left: Range, right: Range) -> Op {
    match kind {
        Kind::Add => Op::Add { left, right },
        Kind::Subtract => Op::Subtract { left, right },
        Kind::Multiply => Op::Multiply { left, right },
        Kind::Negate => Op::Negate { operand: left },
    }
}
fn args(kind: Kind, a: i128, b: i128) -> Args {
    if kind == Kind::Negate {
        Args::Unary { operand: N::new(a) }
    } else {
        Args::Binary {
            left: N::new(a),
            right: N::new(b),
        }
    }
}
fn admitted(kind: Kind, a: i128, b: i128, bound: Option<Range>) -> AdmittedNativeIntegerOperation {
    Plan::new(operation(kind, singleton(a), singleton(b)), bound)
        .unwrap()
        .admit(args(kind, a, b))
        .unwrap()
}
fn oracle(kind: Kind, a: &BigInt, b: &BigInt) -> BigInt {
    match kind {
        Kind::Add => a + b,
        Kind::Subtract => a - b,
        Kind::Multiply => a * b,
        Kind::Negate => -a,
    }
}
fn general(
    kind: Kind,
    a: i128,
    b: i128,
    bound: Option<Range>,
    meter: &mut Meter,
) -> Outcome<Integer> {
    let (a, b) = (Integer::from(a), Integer::from(b));
    let bound = bound
        .map(|r| IntegerInterval::new(Integer::from(r.lower()), Integer::from(r.upper())).unwrap());
    let op = match kind {
        Kind::Add => IntegerArithmetic::Add(&a, &b),
        Kind::Subtract => IntegerArithmetic::Subtract(&a, &b),
        Kind::Multiply => IntegerArithmetic::Multiply(&a, &b),
        Kind::Negate => IntegerArithmetic::Negate(&a),
    };
    evaluate_integer_arithmetic(op, bound.as_ref(), meter)
}
fn abstract_outcome(value: Outcome<N>) -> Outcome<Integer> {
    match value {
        Outcome::Completed(v) => Outcome::Completed(v.to_integer()),
        Outcome::Undefined(v) => Outcome::Undefined(v),
        Outcome::Refused(v) => Outcome::Refused(v),
        Outcome::Incomplete(v) => Outcome::Incomplete(v),
    }
}
fn same_meter(actual: &Meter, expected: &Meter) {
    assert_eq!(actual.admission_count(), expected.admission_count());
    for kind in KINDS {
        assert_eq!(actual.consumed(kind), expected.consumed(kind), "{kind:?}");
    }
    #[cfg(feature = "test-support")]
    {
        assert_eq!(actual.admitted_charges(), expected.admitted_charges());
        assert_eq!(
            actual.charge_log_truncated(),
            expected.charge_log_truncated()
        );
    }
}
fn set_limit(limits: &mut ScalarLimits, kind: LimitKind, value: u64) {
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

#[test]
#[trace("FR-384-AC-1")]
fn admission_distinguishes_shape_membership_capacity_and_general_ingress() {
    assert_eq!(
        Range::new(1, -1),
        Err(Error::ReversedInterval {
            lower: 1,
            upper: -1
        })
    );
    let above = Integer::from(i128::MAX).add(&Integer::one());
    let below = Integer::from(i128::MIN).sub(&Integer::one());
    for (interval, endpoint) in [
        (
            IntegerInterval::new(below, Integer::zero()).unwrap(),
            NativeIntegerEndpoint::Lower,
        ),
        (
            IntegerInterval::new(Integer::zero(), above).unwrap(),
            NativeIntegerEndpoint::Upper,
        ),
    ] {
        assert_eq!(
            Range::try_from(&interval),
            Err(Error::EndpointNotRepresentable { endpoint })
        );
    }
    let full = IntegerInterval::new(Integer::from(i128::MIN), Integer::from(i128::MAX)).unwrap();
    assert_eq!(Range::try_from(&full), Ok(range(i128::MIN, i128::MAX)));
    for (kind, a, b) in [
        (Kind::Add, i128::MAX, 1),
        (Kind::Subtract, i128::MIN, 1),
        (Kind::Multiply, i128::MIN, -1),
        (Kind::Negate, i128::MIN, 0),
    ] {
        assert_eq!(
            Plan::new(
                operation(kind, singleton(a), singleton(b)),
                Some(singleton(0))
            ),
            Err(Error::CapacityExceeded { operation: kind })
        );
    }
    let r = range(-3, 7);
    let plan = Plan::new(
        Op::Add {
            left: r,
            right: singleton(1),
        },
        None,
    )
    .unwrap();
    assert_eq!(plan.capacity(), range(-2, 8));
    assert_eq!(
        plan.admit(Args::Unary { operand: N::new(0) }),
        Err(Error::OperandShapeMismatch {
            operation: Kind::Add
        })
    );
    assert_eq!(
        plan.admit(args(Kind::Add, 8, 1)),
        Err(Error::OperandOutsideInterval {
            operand: NativeIntegerOperandPosition::Left,
            value: 8,
            interval: r
        })
    );
    assert_eq!(
        plan.admit(args(Kind::Add, 0, 2)),
        Err(Error::OperandOutsideInterval {
            operand: NativeIntegerOperandPosition::Right,
            value: 2,
            interval: singleton(1)
        })
    );
    assert_eq!(
        Plan::new(Op::Negate { operand: r }, None)
            .unwrap()
            .admit(args(Kind::Add, 0, 0)),
        Err(Error::OperandShapeMismatch {
            operation: Kind::Negate
        })
    );
}

#[test]
#[trace("FR-384-AC-1", "FR-384-AC-2")]
fn interval_images_and_results_match_bigint_at_extrema_and_reproducible_domains() {
    let mut domains = vec![
        range(i128::MIN, i128::MIN),
        range(i128::MAX, i128::MAX),
        range(i128::MIN, i128::MAX),
        range(-1, 1),
        range(0, 0),
        range(-3, 7),
        range(i64::MIN.into(), i64::MAX.into()),
        range(i32::MIN.into(), i32::MAX.into()),
        range(i32::MIN.into(), i32::MIN.into()),
        range(i32::MAX.into(), i32::MAX.into()),
        range(i128::from(i32::MIN) - 1, 0),
        range(0, i128::from(i32::MAX) + 1),
        range(-1, 0),
        range(0, 1),
    ];
    let mut seed = 0x4a39_b70d_f36c_1029_u128;
    for _ in 0..24 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let a = i128::from_le_bytes(seed.to_le_bytes());
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let b = i128::from_le_bytes(seed.to_le_bytes());
        domains.push(range(a.min(b), a.max(b)));
        let a = i128::from(i32::from_le_bytes(
            seed.to_le_bytes()[..4].try_into().unwrap(),
        ));
        let b = i128::from(i32::from_le_bytes(
            seed.to_le_bytes()[4..8].try_into().unwrap(),
        ));
        domains.push(range(a.min(b), a.max(b)));
    }
    for kind in [Kind::Add, Kind::Subtract, Kind::Multiply, Kind::Negate] {
        let mut successes = 0;
        for left in &domains {
            for right in &domains {
                let corners = [
                    oracle(
                        kind,
                        &BigInt::from(left.lower()),
                        &BigInt::from(right.lower()),
                    ),
                    oracle(
                        kind,
                        &BigInt::from(left.lower()),
                        &BigInt::from(right.upper()),
                    ),
                    oracle(
                        kind,
                        &BigInt::from(left.upper()),
                        &BigInt::from(right.lower()),
                    ),
                    oracle(
                        kind,
                        &BigInt::from(left.upper()),
                        &BigInt::from(right.upper()),
                    ),
                ];
                let expected_low = corners.iter().min().unwrap().to_i128();
                let expected_high = corners.iter().max().unwrap().to_i128();
                let actual = Plan::new(operation(kind, *left, *right), None);
                let supported = kind != Kind::Multiply
                    || [left.lower(), left.upper(), right.lower(), right.upper()]
                        .into_iter()
                        .all(|endpoint| (-2147483648..=2147483647).contains(&endpoint));
                if !supported {
                    assert_eq!(actual, Err(Error::CapacityExceeded { operation: kind }));
                    continue;
                }
                match (expected_low, expected_high) {
                    (Some(low), Some(high)) => {
                        let plan = actual.unwrap();
                        assert_eq!(plan.capacity(), range(low, high));
                        successes += 1;
                        for a in [
                            left.lower(),
                            left.upper(),
                            left.lower() / 2 + left.upper() / 2,
                        ] {
                            for b in [
                                right.lower(),
                                right.upper(),
                                right.lower() / 2 + right.upper() / 2,
                            ] {
                                if !left.contains(N::new(a)) || !right.contains(N::new(b)) {
                                    continue;
                                }
                                let expected = oracle(kind, &BigInt::from(a), &BigInt::from(b));
                                let got = native(
                                    plan.admit(args(kind, a, b)).unwrap(),
                                    &mut Meter::new(limits()),
                                )
                                .completed()
                                .unwrap();
                                assert_eq!(BigInt::from(got.value()), expected);
                            }
                        }
                    }
                    _ => assert_eq!(actual, Err(Error::CapacityExceeded { operation: kind })),
                }
            }
        }
        assert!(successes > 0, "nonvacuous {kind:?}");
    }
}

#[test]
#[trace("FR-384-AC-1", "FR-384-AC-2", "FR-384-AC-3")]
fn multiply_capacity_checks_whole_signed32_domains_independently_of_results() {
    let full = range(i32::MIN.into(), i32::MAX.into());
    let semantic = range(i128::MIN, i128::MAX);
    let plan = Plan::new(
        Op::Multiply {
            left: full,
            right: full,
        },
        Some(semantic),
    )
    .unwrap();
    assert_eq!(
        plan.capacity(),
        range(-4611686016279904256, 4611686018427387904)
    );
    for a in [i32::MIN, -1, 0, 1, i32::MAX] {
        for b in [i32::MIN, -1, 0, 1, i32::MAX] {
            let (a, b) = (i128::from(a), i128::from(b));
            let admitted = plan.admit(args(Kind::Multiply, a, b)).unwrap();
            let mut actual = Meter::new(limits());
            let mut expected = Meter::new(limits());
            let result = native(admitted, &mut actual);
            assert_eq!(
                abstract_outcome(result.clone()),
                general(Kind::Multiply, a, b, Some(semantic), &mut expected)
            );
            same_meter(&actual, &expected);
            let result = result.completed().unwrap();
            assert_eq!(
                BigInt::from(result.value()),
                BigInt::from(a) * BigInt::from(b)
            );
        }
    }
    for unsupported in [
        singleton(2147483648),
        singleton(-2147483649),
        range(-2147483649, 0),
        range(0, 2147483648),
        range(i128::MIN, 0),
        range(0, i128::MAX),
    ] {
        for (left, right) in [(unsupported, singleton(0)), (singleton(0), unsupported)] {
            for domain in [None, Some(singleton(0)), Some(semantic)] {
                let actual = Meter::new(limits());
                assert_eq!(
                    Plan::new(Op::Multiply { left, right }, domain),
                    Err(Error::CapacityExceeded {
                        operation: Kind::Multiply
                    })
                );
                same_meter(&actual, &Meter::new(limits()));
            }
        }
    }
    // A signed-i128 semantic domain is retained even when it excludes every product.
    let domain = Some(singleton(i128::MAX));
    let mut actual = Meter::new(limits());
    let mut expected = Meter::new(limits());
    assert_eq!(
        abstract_outcome(native(admitted(Kind::Multiply, -1, 1, domain), &mut actual)),
        general(Kind::Multiply, -1, 1, domain, &mut expected)
    );
    same_meter(&actual, &expected);
    assert_eq!(actual.admission_count(), 2);
}

fn formula_meter(
    kind: Kind,
    a: i128,
    b: i128,
    bound: Option<Range>,
    meter: &mut Meter,
) -> Outcome<Integer> {
    let a_big = BigInt::from(a);
    let b_big = BigInt::from(b);
    let abits = a_big.bits().max(1);
    let bbits = if kind == Kind::Negate {
        0
    } else {
        b_big.bits().max(1)
    };
    let result = oracle(kind, &a_big, &b_big);
    let schedule = [
        Charge::new(ChargePoint::IntegerArithmeticOperands)
            .size(LimitKind::IntegerBits, abits.max(bbits))
            .size(
                LimitKind::ValueOccurrences,
                if kind == Kind::Negate { 1 } else { 2 },
            ),
        Charge::new(ChargePoint::IntegerArithmeticArithmetic).size(
            LimitKind::IntegerBits,
            match kind {
                Kind::Add | Kind::Subtract => abits.max(bbits) + 1,
                Kind::Multiply => abits + bbits,
                Kind::Negate => abits,
            },
        ),
        Charge::new(ChargePoint::IntegerArithmeticResultRetain).results(1),
    ];
    for (index, charge) in schedule.into_iter().enumerate() {
        if index == 2 {
            if let Some(domain) = bound {
                if result < BigInt::from(domain.lower()) || result > BigInt::from(domain.upper()) {
                    return Outcome::Refused(Refusal::IntegerOutOfDomain {
                        target: Box::new(
                            IntegerInterval::new(
                                Integer::from(domain.lower()),
                                Integer::from(domain.upper()),
                            )
                            .unwrap(),
                        ),
                    });
                }
            }
        }
        if let Err(record) = meter.charge(charge) {
            return Outcome::Incomplete(record);
        }
    }
    Outcome::Completed(Integer::from_big(result))
}

#[test]
#[trace("FR-384-AC-3")]
fn fresh_and_cumulative_meters_match_formulas_and_general_exact_calls() {
    for kind in [Kind::Add, Kind::Subtract, Kind::Multiply, Kind::Negate] {
        let extremes = if kind == Kind::Multiply {
            [
                (i128::from(i32::MIN), i128::from(i32::MAX)),
                (i128::from(i32::MAX), 0),
            ]
        } else {
            [(i128::from(i64::MIN), i128::from(i64::MAX)), (i128::MAX, 0)]
        };
        for (a, b) in [(-23, 7), (0, 0), extremes[0], extremes[1]] {
            for bound in [None, Some(singleton(0)), Some(range(i128::MIN, i128::MAX))] {
                for limit_kind in KINDS {
                    for amount in [0, 1, 2, 3, 64, 65, 127, 128, 129, 256, u64::MAX] {
                        let mut budget = limits();
                        set_limit(&mut budget, limit_kind, amount);
                        let mut actual = Meter::new(budget);
                        let mut reference = Meter::new(budget);
                        let mut old = Meter::new(budget);
                        for _ in 0..2 {
                            let expected = formula_meter(kind, a, b, bound, &mut reference);
                            assert_eq!(general(kind, a, b, bound, &mut old), expected);
                            assert_eq!(
                                abstract_outcome(native(admitted(kind, a, b, bound), &mut actual)),
                                expected
                            );
                            same_meter(&actual, &reference);
                            same_meter(&actual, &old);
                        }
                    }
                }
            }
        }
    }
}

#[test]
#[trace("FR-384-AC-3")]
fn each_injected_point_and_cancellation_preserve_complete_denial_and_prefix() {
    for kind in [Kind::Add, Kind::Subtract, Kind::Multiply, Kind::Negate] {
        for point in [
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic,
            ChargePoint::IntegerArithmeticResultRetain,
        ] {
            for occurrence in [1, 2] {
                let denial = InjectedDenial {
                    point,
                    occurrence: core::num::NonZeroU64::new(occurrence).unwrap(),
                };
                let mut actual = Meter::new(limits()).with_injected_denial(denial);
                let mut reference = Meter::new(limits()).with_injected_denial(denial);
                for _ in 0..3 {
                    assert_eq!(
                        abstract_outcome(native(admitted(kind, -23, 7, None), &mut actual)),
                        general(kind, -23, 7, None, &mut reference)
                    );
                    same_meter(&actual, &reference);
                }
            }
        }
        let cancel = Cancel::new();
        let mut actual = Meter::new(limits()).with_cancel(cancel.clone());
        let mut reference = Meter::new(limits()).with_cancel(cancel.clone());
        assert_eq!(
            abstract_outcome(native(admitted(kind, -23, 7, None), &mut actual)),
            general(kind, -23, 7, None, &mut reference)
        );
        cancel.cancel(CancelCause::Deadline);
        assert_eq!(
            abstract_outcome(native(
                admitted(kind, -23, 7, Some(singleton(0))),
                &mut actual
            )),
            general(kind, -23, 7, Some(singleton(0)), &mut reference)
        );
        same_meter(&actual, &reference);
        assert_eq!(cancel.tripped(), Some(CancelCause::Deadline));
    }
}

#[test]
#[trace("FR-384-AC-3")]
fn ordering_boolean_and_log_truncation_use_the_real_shared_meter() {
    for op in [
        OrderingOperator::Less,
        OrderingOperator::LessOrEqual,
        OrderingOperator::Greater,
        OrderingOperator::GreaterOrEqual,
    ] {
        for (a, b) in [
            (i128::MIN, i128::MAX),
            (0, 0),
            (-1, 0),
            (i128::MAX, i128::MIN),
        ] {
            for limit_kind in KINDS {
                for amount in [0, 1, 2, 3, 127, 128, u64::MAX] {
                    let mut budget = limits();
                    set_limit(&mut budget, limit_kind, amount);
                    let mut actual = Meter::new(budget);
                    let mut reference = Meter::new(budget);
                    for _ in 0..2 {
                        assert_eq!(
                            evaluate_native_integer_ordering(op, N::new(a), N::new(b), &mut actual),
                            order_numbers(
                                op,
                                OrderedOperands::Integers(&Integer::from(a), &Integer::from(b)),
                                &mut reference
                            )
                        );
                        assert_eq!(
                            retain_boolean(true, &mut actual),
                            retain_boolean(true, &mut reference)
                        );
                        same_meter(&actual, &reference);
                    }
                }
            }
        }
        for point in [
            ChargePoint::OrderingOperands,
            ChargePoint::OrderingArithmetic,
            ChargePoint::OrderingResultRetain,
        ] {
            let denial = InjectedDenial {
                point,
                occurrence: core::num::NonZeroU64::new(1).unwrap(),
            };
            let mut actual = Meter::new(limits()).with_injected_denial(denial);
            let mut reference = Meter::new(limits()).with_injected_denial(denial);
            assert_eq!(
                evaluate_native_integer_ordering(op, N::new(-1), N::new(0), &mut actual),
                order_numbers(
                    op,
                    OrderedOperands::Integers(&Integer::from(-1_i128), &Integer::zero()),
                    &mut reference
                )
            );
            same_meter(&actual, &reference);
        }
    }
    let mut actual = Meter::new(limits());
    let mut reference = Meter::new(limits());
    for _ in 0..1400 {
        assert_eq!(
            abstract_outcome(native(admitted(Kind::Add, 1, 2, None), &mut actual)),
            general(Kind::Add, 1, 2, None, &mut reference)
        );
    }
    same_meter(&actual, &reference);
    #[cfg(feature = "test-support")]
    {
        assert!(actual.charge_log_truncated());
        assert_eq!(actual.admitted_charges().len(), 4096);
    }
}

#[test]
#[trace("FR-384-AC-4")]
fn native_admission_and_multistep_success_allocate_no_general_values() {
    fn assert_copy<T: Copy>() {}
    assert_copy::<N>();
    assert_copy::<Range>();
    assert_copy::<Plan>();
    assert_copy::<AdmittedNativeIntegerOperation>();
    let mut result = None;
    // The default production Meter has no diagnostic Vec log. With test-support
    // the log is intentional, so observe only carrier/plan/admission here.
    let allocations = allocation_counter::measure(|| {
        let input = N::new(999);
        let first = Plan::new(
            Op::Add {
                left: range(0, 1000),
                right: singleton(1),
            },
            None,
        )
        .unwrap();
        let admitted = first
            .admit(Args::Binary {
                left: input,
                right: N::new(1),
            })
            .unwrap();
        result = Some((first, admitted));
    });
    assert_eq!(allocations.count_total, 0);
    let (first, admitted) = result.unwrap();
    assert_eq!(first.capacity(), range(1, 1001));
    let mut meter = Meter::new(limits());
    let mut completed = None;
    let allocations = allocation_counter::measure(|| {
        let value = native(admitted, &mut meter).completed().unwrap();
        let second = Plan::new(
            Op::Multiply {
                left: first.capacity(),
                right: singleton(-1),
            },
            None,
        )
        .unwrap();
        let value = native(
            second
                .admit(Args::Binary {
                    left: value,
                    right: N::new(-1),
                })
                .unwrap(),
            &mut meter,
        )
        .completed()
        .unwrap();
        completed = Some((
            value,
            evaluate_native_integer_ordering(OrderingOperator::Less, value, N::new(0), &mut meter),
        ));
    });
    #[cfg(not(feature = "test-support"))]
    assert_eq!(allocations.count_total, 0);
    #[cfg(feature = "test-support")]
    let _ = allocations;
    assert_eq!(completed, Some((N::new(-1000), Outcome::Completed(true))));
    assert_eq!(N::new(i128::MIN).to_integer(), Integer::from(i128::MIN));
}

#[test]
#[trace("FR-384-AC-3")]
fn operation_derived_exact_and_one_less_limits_preserve_fresh_and_cumulative_prefixes() {
    for kind in [Kind::Add, Kind::Subtract, Kind::Multiply, Kind::Negate] {
        let (a, b) = (-23_i128, 7_i128);
        let left_bits = BigInt::from(a).bits().max(1);
        let right_bits = if kind == Kind::Negate {
            0
        } else {
            BigInt::from(b).bits().max(1)
        };
        let operand_bits = left_bits.max(right_bits);
        let arithmetic_bits = match kind {
            Kind::Add | Kind::Subtract => operand_bits + 1,
            Kind::Multiply => left_bits + right_bits,
            Kind::Negate => left_bits,
        };
        let occurrences = if kind == Kind::Negate { 1 } else { 2 };
        // Each semantic amount is derived from the independent BigInt size
        // formula. Work thresholds include every point of the SECOND call;
        // results include its exact cumulative retention boundary.
        let mut boundaries = vec![];
        for bits in [operand_bits, arithmetic_bits] {
            boundaries.extend([
                (LimitKind::IntegerBits, bits),
                (LimitKind::IntegerBits, bits - 1),
            ]);
        }
        boundaries.extend([
            (LimitKind::ValueOccurrences, occurrences),
            (LimitKind::ValueOccurrences, occurrences - 1),
        ]);
        for exact in [1_u64, 2, 3, 4, 5, 6] {
            boundaries.extend([
                (LimitKind::WorkUnits, exact),
                (LimitKind::WorkUnits, exact - 1),
            ]);
        }
        for exact in [1_u64, 2] {
            boundaries.extend([
                (LimitKind::ResultUnits, exact),
                (LimitKind::ResultUnits, exact - 1),
            ]);
        }
        for (limit_kind, amount) in boundaries {
            let mut budget = limits();
            set_limit(&mut budget, limit_kind, amount);
            let mut actual = Meter::new(budget);
            let mut old = Meter::new(budget);
            let mut expected = Meter::new(budget);
            for call in 1..=2 {
                let want = formula_meter(kind, a, b, None, &mut expected);
                assert_eq!(general(kind, a, b, None, &mut old), want);
                let got = abstract_outcome(native(admitted(kind, a, b, None), &mut actual));
                assert_eq!(got, want, "{kind:?}, {limit_kind:?}={amount}, call {call}");
                same_meter(&actual, &expected);
                same_meter(&actual, &old);
                if limit_kind == LimitKind::IntegerBits
                    && amount == operand_bits.max(arithmetic_bits)
                {
                    assert!(
                        matches!(got, Outcome::Completed(_)),
                        "exact semantic maximum completes"
                    );
                }
                if limit_kind == LimitKind::WorkUnits && amount >= 3 && call == 1 {
                    assert!(
                        matches!(got, Outcome::Completed(_)),
                        "first call establishes cumulative prefix"
                    );
                    assert_eq!(actual.consumed(LimitKind::WorkUnits), 3);
                    assert_eq!(actual.consumed(LimitKind::ResultUnits), 1);
                }
                if limit_kind == LimitKind::WorkUnits && amount == 5 && call == 2 {
                    let Outcome::Incomplete(ref record) = got else {
                        panic!("second retention requires sixth work unit")
                    };
                    assert_eq!(
                        *record,
                        quire_exact::Incomplete {
                            limit_kind: LimitKind::WorkUnits,
                            limit: 5,
                            consumed: 5,
                            next_charge: Integer::one(),
                            charge_point: ChargePoint::IntegerArithmeticResultRetain
                        }
                    );
                }
                if limit_kind == LimitKind::ResultUnits && amount == 1 && call == 2 {
                    let Outcome::Incomplete(ref record) = got else {
                        panic!("second result exceeds cumulative result limit")
                    };
                    assert_eq!(
                        *record,
                        quire_exact::Incomplete {
                            limit_kind: LimitKind::ResultUnits,
                            limit: 1,
                            consumed: 1,
                            next_charge: Integer::one(),
                            charge_point: ChargePoint::IntegerArithmeticResultRetain
                        }
                    );
                }
            }
        }
    }
}

#[test]
#[trace("FR-384-AC-3")]
fn ordering_cancellation_precedes_pending_injection_on_fresh_and_charged_meters() {
    for operator in [
        OrderingOperator::Less,
        OrderingOperator::LessOrEqual,
        OrderingOperator::Greater,
        OrderingOperator::GreaterOrEqual,
    ] {
        for charged in [false, true] {
            let cancel = Cancel::new();
            let mut actual = Meter::new(limits()).with_cancel(cancel.clone());
            let mut general_meter = Meter::new(limits()).with_cancel(cancel.clone());
            if charged {
                assert_eq!(
                    evaluate_native_integer_ordering(operator, N::new(-23), N::new(7), &mut actual),
                    order_numbers(
                        operator,
                        OrderedOperands::Integers(&Integer::from(-23_i128), &Integer::from(7_i128)),
                        &mut general_meter
                    )
                );
                assert_eq!(actual.admission_count(), 3);
            }
            same_meter(&actual, &general_meter);
            let prefix = actual.admission_count();
            let work = actual.consumed(LimitKind::WorkUnits);
            let denial = InjectedDenial {
                point: ChargePoint::OrderingOperands,
                occurrence: core::num::NonZeroU64::new(1).unwrap(),
            };
            actual = actual.with_injected_denial(denial);
            general_meter = general_meter.with_injected_denial(denial);
            cancel.cancel(CancelCause::Deadline);
            let expected = Outcome::Incomplete(quire_exact::Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: work,
                consumed: work,
                next_charge: Integer::one(),
                charge_point: ChargePoint::OrderingOperands,
            });
            let cancelled =
                evaluate_native_integer_ordering(operator, N::new(-23), N::new(7), &mut actual);
            assert_eq!(cancelled, expected);
            assert_eq!(
                cancelled,
                order_numbers(
                    operator,
                    OrderedOperands::Integers(&Integer::from(-23_i128), &Integer::from(7_i128)),
                    &mut general_meter
                )
            );
            assert_eq!(cancel.tripped(), Some(CancelCause::Deadline));
            assert_eq!(actual.admission_count(), prefix);
            same_meter(&actual, &general_meter);
            // Replacing the cancellation handle exposes the pending injection.
            // If cancellation consumed it, this call would complete instead.
            actual = actual.with_cancel(Cancel::new());
            general_meter = general_meter.with_cancel(Cancel::new());
            let injected =
                evaluate_native_integer_ordering(operator, N::new(-23), N::new(7), &mut actual);
            assert_eq!(injected, expected);
            assert_eq!(
                injected,
                order_numbers(
                    operator,
                    OrderedOperands::Integers(&Integer::from(-23_i128), &Integer::from(7_i128)),
                    &mut general_meter
                )
            );
            assert_eq!(actual.admission_count(), prefix);
            same_meter(&actual, &general_meter);
            let completed =
                evaluate_native_integer_ordering(operator, N::new(-23), N::new(7), &mut actual);
            assert!(matches!(completed, Outcome::Completed(_)));
            assert_eq!(
                completed,
                order_numbers(
                    operator,
                    OrderedOperands::Integers(&Integer::from(-23_i128), &Integer::from(7_i128)),
                    &mut general_meter
                )
            );
            assert_eq!(actual.admission_count(), prefix + 3);
            same_meter(&actual, &general_meter);
        }
    }
}
