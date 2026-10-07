// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public scalar boundary and direct atom evidence for IR-673.

use std::num::NonZeroU64;

use quire_exact::{
    evaluate_integer_arithmetic as integer_op, evaluate_rational_arithmetic as rational_op,
    order_numbers, ChargePoint, InjectedDenial, Integer, IntegerArithmetic, IntegerInterval,
    LimitKind, Meter, OrderedOperands, OrderingOperator, Outcome, Rational, RationalArithmetic,
    RationalDomain, Refusal, ScalarLimits, Undefined,
};

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

fn meter() -> Meter {
    Meter::new(limits())
}

fn n(v: i64) -> Integer {
    Integer::from(v)
}

fn r(a: i64, b: i64) -> Rational {
    Rational::new(n(a), n(b)).unwrap()
}

fn interval(a: i64, b: i64) -> IntegerInterval {
    IntegerInterval::new(n(a), n(b)).unwrap()
}

fn domain(a: i64, b: i64, c: i64, d: i64) -> RationalDomain {
    RationalDomain::new(interval(a, b), interval(c, d)).unwrap()
}

fn denied<T>(
    outcome: Outcome<T>,
    meter: &Meter,
    point: ChargePoint,
    prior: &[ChargePoint],
    bits: u64,
    consumed: u64,
    next: u64,
) {
    let Outcome::Incomplete(incomplete) = outcome else {
        panic!("expected Incomplete")
    };
    assert_eq!(
        (
            incomplete.charge_point,
            incomplete.limit_kind,
            incomplete.limit,
            incomplete.consumed,
            incomplete.next_charge
        ),
        (
            point,
            LimitKind::IntegerBits,
            bits,
            consumed,
            Integer::from(next)
        )
    );
    #[cfg(feature = "test-support")]
    assert_eq!(meter.admitted_charges(), prior);
    #[cfg(not(feature = "test-support"))]
    let _ = prior;
    assert_eq!(meter.consumed(LimitKind::IntegerBits), consumed);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}

fn assert_exact_counters(meter: &Meter, expected: [u64; 10]) {
    for (kind, amount) in LimitKind::ALL.into_iter().zip(expected) {
        assert_eq!(meter.consumed(kind), amount, "{kind:?}");
    }
}

/// Trace: TC-910
#[test]
fn scalar_bit_limits_admit_exactly_and_deny_first_excess() {
    let (a, b) = (n(1000), n(999));
    let integers = [
        (
            IntegerArithmetic::Add(&a, &b),
            n(1999),
            11,
            10,
            ChargePoint::IntegerArithmeticArithmetic,
            11,
        ),
        (
            IntegerArithmetic::Subtract(&a, &b),
            n(1),
            11,
            10,
            ChargePoint::IntegerArithmeticArithmetic,
            11,
        ),
        (
            IntegerArithmetic::Multiply(&n(255), &n(255)),
            n(65025),
            16,
            8,
            ChargePoint::IntegerArithmeticArithmetic,
            16,
        ),
        (
            IntegerArithmetic::Negate(&n(-5)),
            n(5),
            3,
            0,
            ChargePoint::IntegerArithmeticOperands,
            3,
        ),
    ];
    for (op, expected, bits, consumed, point, request) in integers {
        let mut ok = limits();
        ok.integer_bits = bits;
        let mut m = Meter::new(ok);
        assert_eq!(integer_op(op, None, &mut m).completed(), Some(expected));
        assert_eq!(m.consumed(LimitKind::IntegerBits), bits);
        let mut low = ok;
        low.integer_bits -= 1;
        let mut m = Meter::new(low);
        let outcome = integer_op(op, None, &mut m);
        let prefix = if point == ChargePoint::IntegerArithmeticOperands {
            vec![]
        } else {
            vec![ChargePoint::IntegerArithmeticOperands]
        };
        denied(outcome, &m, point, &prefix, bits - 1, consumed, request);
    }
    let (x, y) = (r(2, 3), r(3, 2));
    let (u, v) = (r(3, 4), r(5, 7));
    let (w, z) = (r(5, 7), r(4, 7));
    let neg = r(-5, 8);
    let rationals = [
        (RationalArithmetic::Add(&w, &z), r(9, 7), 7, 3, 7),
        (RationalArithmetic::Subtract(&w, &z), r(1, 7), 7, 3, 7),
        (RationalArithmetic::Multiply(&x, &y), r(1, 1), 4, 2, 4),
        (RationalArithmetic::Divide(&u, &v), r(21, 20), 6, 3, 6),
        (RationalArithmetic::Negate(&neg), r(5, 8), 4, 0, 4),
    ];
    for (op, expected, bits, consumed, request) in rationals {
        let mut ok = limits();
        ok.integer_bits = bits;
        let mut m = Meter::new(ok);
        assert_eq!(rational_op(op, None, &mut m).completed(), Some(expected));
        assert_eq!(m.consumed(LimitKind::IntegerBits), bits);
        let mut low = ok;
        low.integer_bits -= 1;
        let mut m = Meter::new(low);
        let outcome = rational_op(op, None, &mut m);
        let (point, prefix) = if matches!(op, RationalArithmetic::Negate(_)) {
            (ChargePoint::RationalArithmeticOperands, vec![])
        } else {
            (
                ChargePoint::RationalArithmeticArithmetic,
                vec![ChargePoint::RationalArithmeticOperands],
            )
        };
        denied(outcome, &m, point, &prefix, bits - 1, consumed, request);
    }
    for (operands, expected, bits) in [
        (OrderedOperands::Integers(&n(2), &n(3)), true, 2),
        (OrderedOperands::Rationals(&r(7, 3), &r(5, 8)), false, 7),
    ] {
        let mut ok = limits();
        ok.integer_bits = bits;
        let mut m = Meter::new(ok);
        assert_eq!(
            order_numbers(OrderingOperator::Less, operands, &mut m).completed(),
            Some(expected)
        );
        assert_eq!(m.consumed(LimitKind::IntegerBits), bits);
        let mut low = ok;
        low.integer_bits -= 1;
        let mut m = Meter::new(low);
        let outcome = order_numbers(OrderingOperator::Less, operands, &mut m);
        let prefix = if bits == 2 {
            vec![]
        } else {
            vec![ChargePoint::OrderingOperands]
        };
        let point = if bits == 2 {
            ChargePoint::OrderingOperands
        } else {
            ChargePoint::OrderingArithmetic
        };
        denied(
            outcome,
            &m,
            point,
            &prefix,
            bits - 1,
            if bits == 2 { 0 } else { 4 },
            bits,
        );
    }
}

/// Trace: TC-910, FR-362-AC-11
#[test]
fn scalar_outcomes_keep_only_admitted_prefixes() {
    let mut m = meter();
    let false_result = order_numbers(
        OrderingOperator::Less,
        OrderedOperands::Integers(&n(2), &n(1)),
        &mut m,
    );
    assert_eq!(false_result.clone(), Outcome::Completed(false));
    assert_eq!(false_result.completed(), Some(false));
    let mut m = meter();
    let undefined = rational_op(RationalArithmetic::Divide(&r(1, 2), &r(0, 1)), None, &mut m);
    assert_eq!(
        undefined.clone(),
        Outcome::Undefined(Undefined::DivisionByZero)
    );
    assert_eq!(undefined.completed(), None);
    assert_eq!(m.consumed(LimitKind::IntegerBits), 2);
    assert_eq!(m.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(m.admission_count(), 1);
    assert_exact_counters(&m, [2, 0, 0, 0, 0, 0, 0, 2, 1, 0]);
    #[cfg(feature = "test-support")]
    assert_eq!(
        m.admitted_charges(),
        [ChargePoint::RationalArithmeticOperands]
    );
    assert_eq!(
        (
            m.consumed(LimitKind::WorkUnits),
            m.consumed(LimitKind::ResultUnits)
        ),
        (1, 0)
    );
    let mut m = meter();
    let out = integer_op(
        IntegerArithmetic::Add(&n(2), &n(2)),
        Some(&interval(0, 3)),
        &mut m,
    );
    assert!(out.clone().completed().is_none());
    assert!(matches!(
        out,
        Outcome::Refused(Refusal::IntegerOutOfDomain { .. })
    ));
    assert_eq!(m.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(m.admission_count(), 2);
    assert_exact_counters(&m, [3, 0, 0, 0, 0, 0, 0, 2, 2, 0]);
    #[cfg(feature = "test-support")]
    assert_eq!(
        m.admitted_charges(),
        [
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic
        ]
    );
    assert_eq!(
        (
            m.consumed(LimitKind::IntegerBits),
            m.consumed(LimitKind::WorkUnits),
            m.consumed(LimitKind::ResultUnits)
        ),
        (3, 2, 0)
    );
    let mut m = meter();
    let out = rational_op(
        RationalArithmetic::Divide(&r(1, 1), &r(3, 1)),
        Some(&domain(0, 1, 1, 2)),
        &mut m,
    );
    assert!(out.clone().completed().is_none());
    assert!(matches!(
        out,
        Outcome::Refused(Refusal::RationalOutOfDomain { .. })
    ));
    assert_eq!(m.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(m.admission_count(), 3);
    assert_exact_counters(&m, [3, 0, 0, 0, 0, 0, 0, 2, 3, 0]);
    #[cfg(feature = "test-support")]
    assert_eq!(
        m.admitted_charges(),
        [
            ChargePoint::RationalArithmeticOperands,
            ChargePoint::RationalArithmeticArithmetic,
            ChargePoint::RationalArithmeticNormalize
        ]
    );
    assert_eq!(
        (
            m.consumed(LimitKind::IntegerBits),
            m.consumed(LimitKind::WorkUnits),
            m.consumed(LimitKind::ResultUnits)
        ),
        (3, 3, 0)
    );
    let mut m = meter().with_injected_denial(InjectedDenial {
        point: ChargePoint::IntegerArithmeticOperands,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    let out = integer_op(IntegerArithmetic::Add(&n(2), &n(2)), None, &mut m);
    assert!(out.clone().completed().is_none());
    assert!(matches!(out, Outcome::Incomplete(_)));
    assert_eq!(m.admission_count(), 0);
    for kind in LimitKind::ALL {
        assert_eq!(m.consumed(kind), 0);
    }
    #[cfg(feature = "test-support")]
    assert_eq!(m.admitted_charges(), []);
}

/// Trace: TC-910, FR-362-AC-12
#[test]
fn rational_constructor_reduces_sign_and_zero() {
    for (a, b, c, d) in [
        (4, 8, 1, 2),
        (-4, -8, 1, 2),
        (1, -2, -1, 2),
        (0, 5, 0, 1),
        (0, -5, 0, 1),
    ] {
        let value = r(a, b);
        assert_eq!((value.numerator(), value.denominator()), (&n(c), &n(d)));
    }
}

/// Trace: TC-910, FR-362-AC-13
#[test]
fn rational_domain_refuses_denominator_outside_interval() {
    let target = domain(0, 1, 1, 2);
    assert!(!target.contains(&r(1, 3)));
    let mut m = meter();
    assert_eq!(
        rational_op(
            RationalArithmetic::Divide(&r(1, 1), &r(3, 1)),
            Some(&target),
            &mut m
        ),
        Outcome::Refused(Refusal::RationalOutOfDomain {
            target: Box::new(target.clone())
        })
    );
    #[cfg(feature = "test-support")]
    assert_eq!(
        m.admitted_charges(),
        [
            ChargePoint::RationalArithmeticOperands,
            ChargePoint::RationalArithmeticArithmetic,
            ChargePoint::RationalArithmeticNormalize
        ]
    );
    assert_eq!(m.consumed(LimitKind::ResultUnits), 0);
    assert_eq!(
        rational_op(
            RationalArithmetic::Divide(&r(1, 1), &r(3, 1)),
            None,
            &mut meter()
        )
        .completed(),
        Some(r(1, 3))
    );
}

/// Trace: TC-910, FR-362-AC-14
#[test]
fn direct_countdown_atoms_stop_at_fifth_result() {
    fn run(m: &mut Meter) -> Vec<Outcome<Integer>> {
        let mut values = Vec::new();
        assert_eq!(
            order_numbers(
                OrderingOperator::Greater,
                OrderedOperands::Integers(&n(2), &n(0)),
                m
            )
            .completed(),
            Some(true)
        );
        values.push(integer_op(
            IntegerArithmetic::Subtract(&n(2), &n(1)),
            None,
            m,
        ));
        assert_eq!(
            order_numbers(
                OrderingOperator::Greater,
                OrderedOperands::Integers(&n(1), &n(0)),
                m
            )
            .completed(),
            Some(true)
        );
        values.push(integer_op(
            IntegerArithmetic::Subtract(&n(1), &n(1)),
            None,
            m,
        ));
        values
    }
    let mut m = meter();
    assert_eq!(
        run(&mut m),
        [Outcome::Completed(n(1)), Outcome::Completed(n(0))]
    );
    assert_eq!(
        order_numbers(
            OrderingOperator::Greater,
            OrderedOperands::Integers(&n(0), &n(0)),
            &mut m
        )
        .completed(),
        Some(false)
    );
    assert_eq!(
        (
            m.consumed(LimitKind::WorkUnits),
            m.consumed(LimitKind::ResultUnits)
        ),
        (15, 5)
    );
    let mut configured = limits();
    configured.work_units = 14;
    let mut m = Meter::new(configured);
    assert_eq!(
        run(&mut m),
        [Outcome::Completed(n(1)), Outcome::Completed(n(0))]
    );
    let Outcome::Incomplete(stop) = order_numbers(
        OrderingOperator::Greater,
        OrderedOperands::Integers(&n(0), &n(0)),
        &mut m,
    ) else {
        panic!("fifth result must stop")
    };
    assert_eq!(
        (
            stop.charge_point,
            stop.limit_kind,
            stop.limit,
            stop.consumed,
            stop.next_charge
        ),
        (
            ChargePoint::OrderingResultRetain,
            LimitKind::WorkUnits,
            14,
            14,
            Integer::one()
        )
    );
    assert_eq!(m.consumed(LimitKind::ResultUnits), 4);
    #[cfg(feature = "test-support")]
    {
        let ordering = [
            ChargePoint::OrderingOperands,
            ChargePoint::OrderingArithmetic,
            ChargePoint::OrderingResultRetain,
        ];
        let arithmetic = [
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic,
            ChargePoint::IntegerArithmeticResultRetain,
        ];
        let expected = [ordering, arithmetic, ordering, arithmetic];
        let mut prefix: Vec<_> = expected.into_iter().flatten().collect();
        prefix.extend_from_slice(&ordering[..2]);
        assert_eq!(m.admitted_charges(), prefix);
    }
}

/// Trace: TC-910, FR-362-AC-15
#[test]
fn direct_rational_division_charges_four_points() {
    let (a, b) = (r(3, 1), r(2, 1));
    let mut m = meter();
    assert_eq!(
        rational_op(RationalArithmetic::Divide(&a, &b), None, &mut m).completed(),
        Some(r(3, 2))
    );
    assert_eq!(
        (
            m.consumed(LimitKind::IntegerBits),
            m.consumed(LimitKind::ValueOccurrences),
            m.consumed(LimitKind::WorkUnits),
            m.consumed(LimitKind::ResultUnits)
        ),
        (3, 2, 4, 1)
    );
    #[cfg(feature = "test-support")]
    assert_eq!(
        m.admitted_charges(),
        [
            ChargePoint::RationalArithmeticOperands,
            ChargePoint::RationalArithmeticArithmetic,
            ChargePoint::RationalArithmeticNormalize,
            ChargePoint::RationalArithmeticResultRetain
        ]
    );
    let mut configured = limits();
    configured.work_units = 3;
    let mut m = Meter::new(configured);
    let Outcome::Incomplete(stop) = rational_op(RationalArithmetic::Divide(&a, &b), None, &mut m)
    else {
        panic!("retain must stop")
    };
    assert_eq!(
        (
            stop.charge_point,
            stop.limit_kind,
            stop.limit,
            stop.consumed,
            stop.next_charge
        ),
        (
            ChargePoint::RationalArithmeticResultRetain,
            LimitKind::WorkUnits,
            3,
            3,
            Integer::one()
        )
    );
    assert_eq!(m.consumed(LimitKind::ResultUnits), 0);
    #[cfg(feature = "test-support")]
    assert_eq!(m.admitted_charges().len(), 3);
}

/// Trace: TC-910, FR-362-AC-16
#[test]
fn direct_integer_additions_share_meter_without_fold() {
    let mut m = meter();
    assert_eq!(
        integer_op(IntegerArithmetic::Add(&n(0), &n(1)), None, &mut m).completed(),
        Some(n(1))
    );
    assert_eq!(
        integer_op(IntegerArithmetic::Add(&n(1), &n(2)), None, &mut m).completed(),
        Some(n(3))
    );
    #[cfg(feature = "test-support")]
    assert_eq!(
        m.admitted_charges(),
        [
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic,
            ChargePoint::IntegerArithmeticResultRetain,
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic,
            ChargePoint::IntegerArithmeticResultRetain
        ]
    );
    assert_eq!(
        (
            m.consumed(LimitKind::IntegerBits),
            m.consumed(LimitKind::ValueOccurrences),
            m.consumed(LimitKind::WorkUnits),
            m.consumed(LimitKind::ResultUnits)
        ),
        (3, 2, 6, 2)
    );
}

/// Trace: TC-910, FR-362-AC-17
#[test]
fn atom_work_boundaries_stop_on_named_points() {
    for (work, point) in [
        (2, Some(ChargePoint::IntegerArithmeticResultRetain)),
        (3, None),
    ] {
        let mut configured = limits();
        configured.work_units = work;
        let mut m = Meter::new(configured);
        let outcome = integer_op(IntegerArithmetic::Add(&n(5), &n(7)), None, &mut m);
        if let Some(point) = point {
            let Outcome::Incomplete(stop) = outcome else {
                panic!("expected stop")
            };
            assert_eq!(
                (
                    stop.charge_point,
                    stop.limit_kind,
                    stop.limit,
                    stop.consumed,
                    stop.next_charge
                ),
                (point, LimitKind::WorkUnits, work, work, Integer::one())
            );
            assert_eq!(m.consumed(LimitKind::ResultUnits), 0);
        } else {
            assert_eq!(outcome.completed(), Some(n(12)));
            assert_eq!(m.consumed(LimitKind::ResultUnits), 1);
        }
        #[cfg(feature = "test-support")]
        let expected = [
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic,
            ChargePoint::IntegerArithmeticResultRetain,
        ];
        #[cfg(feature = "test-support")]
        assert_eq!(m.admitted_charges(), &expected[..work as usize]);
    }
    let (a, b) = (r(2, 3), r(3, 2));
    for (work, point) in [
        (2, Some(ChargePoint::RationalArithmeticNormalize)),
        (3, Some(ChargePoint::RationalArithmeticResultRetain)),
        (4, None),
    ] {
        let mut configured = limits();
        configured.work_units = work;
        let mut m = Meter::new(configured);
        let outcome = rational_op(RationalArithmetic::Multiply(&a, &b), None, &mut m);
        if let Some(point) = point {
            let Outcome::Incomplete(stop) = outcome else {
                panic!("expected stop")
            };
            assert_eq!(
                (
                    stop.charge_point,
                    stop.limit_kind,
                    stop.limit,
                    stop.consumed,
                    stop.next_charge
                ),
                (point, LimitKind::WorkUnits, work, work, Integer::one())
            );
            assert_eq!(m.consumed(LimitKind::ResultUnits), 0);
        } else {
            assert_eq!(outcome.completed(), Some(r(1, 1)));
            assert_eq!(m.consumed(LimitKind::ResultUnits), 1);
        }
        #[cfg(feature = "test-support")]
        let expected = [
            ChargePoint::RationalArithmeticOperands,
            ChargePoint::RationalArithmeticArithmetic,
            ChargePoint::RationalArithmeticNormalize,
            ChargePoint::RationalArithmeticResultRetain,
        ];
        #[cfg(feature = "test-support")]
        assert_eq!(m.admitted_charges(), &expected[..work as usize]);
    }
}

/// Trace: TC-910, FR-362-AC-18
#[test]
fn powers_of_two_charge_operand_derived_bounds() {
    for k in [63, 64, 65, 127, 128, 200, 511] {
        let power = Integer::one().shifted_left(k);
        let below = power.sub(&Integer::one());
        let above = power.add(&Integer::one());
        let reference_power = num_bigint::BigInt::from(1_u8) << k;
        let reference_square = &reference_power * &reference_power;
        for (operation, expected, bits) in [
            (
                IntegerArithmetic::Multiply(&below, &above),
                Integer::from_big(&reference_square - 1_u8),
                2 * k + 1,
            ),
            (
                IntegerArithmetic::Multiply(&power, &power),
                Integer::from_big(reference_square),
                2 * k + 2,
            ),
            (
                IntegerArithmetic::Add(&below, &Integer::one()),
                Integer::from_big(reference_power),
                k + 1,
            ),
            (
                IntegerArithmetic::Subtract(&above, &power),
                Integer::one(),
                k + 2,
            ),
            (
                IntegerArithmetic::Subtract(&power, &power),
                Integer::zero(),
                k + 2,
            ),
        ] {
            let mut m = meter();
            assert_eq!(
                integer_op(operation, None, &mut m).completed(),
                Some(expected)
            );
            assert_eq!(m.consumed(LimitKind::IntegerBits), bits, "k={k}");
        }
    }
}
