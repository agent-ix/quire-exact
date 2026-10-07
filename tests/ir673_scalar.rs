// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public scalar boundary and direct atom evidence for IR-673.

#![cfg(feature = "test-support")]

use quire_exact::{
    evaluate_integer_arithmetic as integer_op, evaluate_rational_arithmetic as rational_op,
    order_numbers, ChargePoint as P, InjectedDenial, Integer as I, IntegerArithmetic as IA,
    IntegerInterval, LimitKind as K, Meter, OrderedOperands, OrderingOperator as OO, Outcome,
    Rational as R, RationalArithmetic as RA, RationalDomain, Refusal, ScalarLimits, Undefined,
};
use std::num::NonZeroU64;

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
fn n(v: i64) -> I {
    I::from(v)
}
fn r(a: i64, b: i64) -> R {
    R::new(n(a), n(b)).unwrap()
}
fn interval(a: i64, b: i64) -> IntegerInterval {
    IntegerInterval::new(n(a), n(b)).unwrap()
}
fn domain(a: i64, b: i64, c: i64, d: i64) -> RationalDomain {
    RationalDomain::new(interval(a, b), interval(c, d)).unwrap()
}
#[cfg(feature = "test-support")]
fn denied<T>(
    outcome: Outcome<T>,
    meter: &Meter,
    point: P,
    prior: &[P],
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
        (point, K::IntegerBits, bits, consumed, I::from(next))
    );
    assert_eq!(meter.admitted_charges(), prior);
    assert_eq!(meter.consumed(K::IntegerBits), consumed);
    assert_eq!(meter.consumed(K::ResultUnits), 0);
}

/// Trace: TC-910, FR-362-AC-10
#[cfg(feature = "test-support")]
#[test]
fn scalar_bit_limits_admit_exactly_and_deny_first_excess() {
    let (a, b) = (n(1000), n(999));
    let integers = [
        (
            IA::Add(&a, &b),
            n(1999),
            11,
            10,
            P::IntegerArithmeticArithmetic,
            11,
        ),
        (
            IA::Subtract(&a, &b),
            n(1),
            11,
            10,
            P::IntegerArithmeticArithmetic,
            11,
        ),
        (
            IA::Multiply(&n(255), &n(255)),
            n(65025),
            16,
            8,
            P::IntegerArithmeticArithmetic,
            16,
        ),
        (
            IA::Negate(&n(-5)),
            n(5),
            3,
            0,
            P::IntegerArithmeticOperands,
            3,
        ),
    ];
    for (op, expected, bits, consumed, point, request) in integers {
        let mut ok = limits();
        ok.integer_bits = bits;
        let mut m = Meter::new(ok);
        assert_eq!(integer_op(op, None, &mut m).completed(), Some(expected));
        assert_eq!(m.consumed(K::IntegerBits), bits);
        let mut low = ok;
        low.integer_bits -= 1;
        let mut m = Meter::new(low);
        let outcome = integer_op(op, None, &mut m);
        let prefix = if point == P::IntegerArithmeticOperands {
            vec![]
        } else {
            vec![P::IntegerArithmeticOperands]
        };
        denied(outcome, &m, point, &prefix, bits - 1, consumed, request);
    }
    let (x, y) = (r(2, 3), r(3, 2));
    let (u, v) = (r(3, 4), r(5, 7));
    let (w, z) = (r(5, 7), r(4, 7));
    let neg = r(-5, 8);
    let rationals = [
        (RA::Add(&w, &z), r(9, 7), 7, 3, 7),
        (RA::Subtract(&w, &z), r(1, 7), 7, 3, 7),
        (RA::Multiply(&x, &y), r(1, 1), 4, 2, 4),
        (RA::Divide(&u, &v), r(21, 20), 6, 3, 6),
        (RA::Negate(&neg), r(5, 8), 4, 0, 4),
    ];
    for (op, expected, bits, consumed, request) in rationals {
        let mut ok = limits();
        ok.integer_bits = bits;
        let mut m = Meter::new(ok);
        assert_eq!(rational_op(op, None, &mut m).completed(), Some(expected));
        assert_eq!(m.consumed(K::IntegerBits), bits);
        let mut low = ok;
        low.integer_bits -= 1;
        let mut m = Meter::new(low);
        let outcome = rational_op(op, None, &mut m);
        let (point, prefix) = if matches!(op, RA::Negate(_)) {
            (P::RationalArithmeticOperands, vec![])
        } else {
            (
                P::RationalArithmeticArithmetic,
                vec![P::RationalArithmeticOperands],
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
            order_numbers(OO::Less, operands, &mut m).completed(),
            Some(expected)
        );
        assert_eq!(m.consumed(K::IntegerBits), bits);
        let mut low = ok;
        low.integer_bits -= 1;
        let mut m = Meter::new(low);
        let outcome = order_numbers(OO::Less, operands, &mut m);
        let prefix = if bits == 2 {
            vec![]
        } else {
            vec![P::OrderingOperands]
        };
        let point = if bits == 2 {
            P::OrderingOperands
        } else {
            P::OrderingArithmetic
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
#[cfg(feature = "test-support")]
#[test]
fn scalar_outcomes_keep_only_admitted_prefixes() {
    let mut m = meter();
    let false_result = order_numbers(OO::Less, OrderedOperands::Integers(&n(2), &n(1)), &mut m);
    assert_eq!(false_result.clone(), Outcome::Completed(false));
    assert_eq!(false_result.completed(), Some(false));
    let mut m = meter();
    let undefined = rational_op(RA::Divide(&r(1, 2), &r(0, 1)), None, &mut m);
    assert_eq!(
        undefined.clone(),
        Outcome::Undefined(Undefined::DivisionByZero)
    );
    assert_eq!(undefined.completed(), None);
    assert_eq!(m.admitted_charges(), [P::RationalArithmeticOperands]);
    assert_eq!(
        (m.consumed(K::WorkUnits), m.consumed(K::ResultUnits)),
        (1, 0)
    );
    let mut m = meter();
    let out = integer_op(IA::Add(&n(2), &n(2)), Some(&interval(0, 3)), &mut m);
    assert!(matches!(
        out,
        Outcome::Refused(Refusal::IntegerOutOfDomain { .. })
    ));
    assert_eq!(
        m.admitted_charges(),
        [P::IntegerArithmeticOperands, P::IntegerArithmeticArithmetic]
    );
    assert_eq!(
        (
            m.consumed(K::IntegerBits),
            m.consumed(K::WorkUnits),
            m.consumed(K::ResultUnits)
        ),
        (3, 2, 0)
    );
    let mut m = meter();
    let out = rational_op(
        RA::Divide(&r(1, 1), &r(3, 1)),
        Some(&domain(0, 1, 1, 2)),
        &mut m,
    );
    assert!(matches!(
        out,
        Outcome::Refused(Refusal::RationalOutOfDomain { .. })
    ));
    assert_eq!(
        m.admitted_charges(),
        [
            P::RationalArithmeticOperands,
            P::RationalArithmeticArithmetic,
            P::RationalArithmeticNormalize
        ]
    );
    assert_eq!(
        (
            m.consumed(K::IntegerBits),
            m.consumed(K::WorkUnits),
            m.consumed(K::ResultUnits)
        ),
        (3, 3, 0)
    );
    let mut m = meter().with_injected_denial(InjectedDenial {
        point: P::IntegerArithmeticOperands,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    assert!(matches!(
        integer_op(IA::Add(&n(2), &n(2)), None, &mut m),
        Outcome::Incomplete(_)
    ));
    assert_eq!(m.admission_count(), 0);
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
#[cfg(feature = "test-support")]
#[test]
fn rational_domain_refuses_denominator_outside_interval() {
    let target = domain(0, 1, 1, 2);
    assert!(!target.contains(&r(1, 3)));
    let mut m = meter();
    assert_eq!(
        rational_op(RA::Divide(&r(1, 1), &r(3, 1)), Some(&target), &mut m),
        Outcome::Refused(Refusal::RationalOutOfDomain {
            target: Box::new(target.clone())
        })
    );
    assert_eq!(
        m.admitted_charges(),
        [
            P::RationalArithmeticOperands,
            P::RationalArithmeticArithmetic,
            P::RationalArithmeticNormalize
        ]
    );
    assert_eq!(m.consumed(K::ResultUnits), 0);
    assert_eq!(
        rational_op(RA::Divide(&r(1, 1), &r(3, 1)), None, &mut meter()).completed(),
        Some(r(1, 3))
    );
}

/// Trace: TC-910, FR-362-AC-14
#[cfg(feature = "test-support")]
#[test]
fn direct_countdown_atoms_stop_at_fifth_result() {
    fn run(m: &mut Meter) -> Vec<Outcome<I>> {
        let mut values = Vec::new();
        assert_eq!(
            order_numbers(OO::Greater, OrderedOperands::Integers(&n(2), &n(0)), m).completed(),
            Some(true)
        );
        values.push(integer_op(IA::Subtract(&n(2), &n(1)), None, m));
        assert_eq!(
            order_numbers(OO::Greater, OrderedOperands::Integers(&n(1), &n(0)), m).completed(),
            Some(true)
        );
        values.push(integer_op(IA::Subtract(&n(1), &n(1)), None, m));
        values
    }
    let mut m = meter();
    assert_eq!(
        run(&mut m),
        [Outcome::Completed(n(1)), Outcome::Completed(n(0))]
    );
    assert_eq!(
        order_numbers(OO::Greater, OrderedOperands::Integers(&n(0), &n(0)), &mut m).completed(),
        Some(false)
    );
    assert_eq!(
        (m.consumed(K::WorkUnits), m.consumed(K::ResultUnits)),
        (15, 5)
    );
    let mut configured = limits();
    configured.work_units = 14;
    let mut m = Meter::new(configured);
    assert_eq!(
        run(&mut m),
        [Outcome::Completed(n(1)), Outcome::Completed(n(0))]
    );
    let Outcome::Incomplete(stop) =
        order_numbers(OO::Greater, OrderedOperands::Integers(&n(0), &n(0)), &mut m)
    else {
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
        (P::OrderingResultRetain, K::WorkUnits, 14, 14, I::one())
    );
    assert_eq!(m.consumed(K::ResultUnits), 4);
    let ordering = [
        P::OrderingOperands,
        P::OrderingArithmetic,
        P::OrderingResultRetain,
    ];
    let arithmetic = [
        P::IntegerArithmeticOperands,
        P::IntegerArithmeticArithmetic,
        P::IntegerArithmeticResultRetain,
    ];
    let expected = [ordering, arithmetic, ordering, arithmetic];
    let mut prefix: Vec<_> = expected.into_iter().flatten().collect();
    prefix.extend_from_slice(&ordering[..2]);
    assert_eq!(m.admitted_charges(), prefix);
}

/// Trace: TC-910, FR-362-AC-15
#[cfg(feature = "test-support")]
#[test]
fn direct_rational_division_charges_four_points() {
    let (a, b) = (r(3, 1), r(2, 1));
    let mut m = meter();
    assert_eq!(
        rational_op(RA::Divide(&a, &b), None, &mut m).completed(),
        Some(r(3, 2))
    );
    assert_eq!(
        (
            m.consumed(K::IntegerBits),
            m.consumed(K::ValueOccurrences),
            m.consumed(K::WorkUnits),
            m.consumed(K::ResultUnits)
        ),
        (3, 2, 4, 1)
    );
    assert_eq!(
        m.admitted_charges(),
        [
            P::RationalArithmeticOperands,
            P::RationalArithmeticArithmetic,
            P::RationalArithmeticNormalize,
            P::RationalArithmeticResultRetain
        ]
    );
    let mut configured = limits();
    configured.work_units = 3;
    let mut m = Meter::new(configured);
    let Outcome::Incomplete(stop) = rational_op(RA::Divide(&a, &b), None, &mut m) else {
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
            P::RationalArithmeticResultRetain,
            K::WorkUnits,
            3,
            3,
            I::one()
        )
    );
    assert_eq!(m.consumed(K::ResultUnits), 0);
    assert_eq!(m.admitted_charges().len(), 3);
}

/// Trace: TC-910, FR-362-AC-16
#[cfg(feature = "test-support")]
#[test]
fn direct_integer_additions_share_meter_without_fold() {
    let mut m = meter();
    assert_eq!(
        integer_op(IA::Add(&n(0), &n(1)), None, &mut m).completed(),
        Some(n(1))
    );
    assert_eq!(
        integer_op(IA::Add(&n(1), &n(2)), None, &mut m).completed(),
        Some(n(3))
    );
    assert_eq!(
        m.admitted_charges(),
        [
            P::IntegerArithmeticOperands,
            P::IntegerArithmeticArithmetic,
            P::IntegerArithmeticResultRetain,
            P::IntegerArithmeticOperands,
            P::IntegerArithmeticArithmetic,
            P::IntegerArithmeticResultRetain
        ]
    );
    assert_eq!(
        (
            m.consumed(K::IntegerBits),
            m.consumed(K::ValueOccurrences),
            m.consumed(K::WorkUnits),
            m.consumed(K::ResultUnits)
        ),
        (3, 2, 6, 2)
    );
}

/// Trace: TC-910, FR-362-AC-17
#[cfg(feature = "test-support")]
#[test]
fn atom_work_boundaries_stop_on_named_points() {
    for (work, point, prefix) in [(2, Some(P::IntegerArithmeticResultRetain), 2), (3, None, 3)] {
        let mut configured = limits();
        configured.work_units = work;
        let mut m = Meter::new(configured);
        let outcome = integer_op(IA::Add(&n(5), &n(7)), None, &mut m);
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
                (point, K::WorkUnits, work, work, I::one())
            );
            assert_eq!(m.consumed(K::ResultUnits), 0);
        } else {
            assert_eq!(outcome.completed(), Some(n(12)));
            assert_eq!(m.consumed(K::ResultUnits), 1);
        }
        let expected = [
            P::IntegerArithmeticOperands,
            P::IntegerArithmeticArithmetic,
            P::IntegerArithmeticResultRetain,
        ];
        assert_eq!(m.admitted_charges(), &expected[..prefix as usize]);
    }
    let (a, b) = (r(2, 3), r(3, 2));
    for (work, point) in [
        (2, Some(P::RationalArithmeticNormalize)),
        (3, Some(P::RationalArithmeticResultRetain)),
        (4, None),
    ] {
        let mut configured = limits();
        configured.work_units = work;
        let mut m = Meter::new(configured);
        let outcome = rational_op(RA::Multiply(&a, &b), None, &mut m);
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
                (point, K::WorkUnits, work, work, I::one())
            );
            assert_eq!(m.consumed(K::ResultUnits), 0);
        } else {
            assert_eq!(outcome.completed(), Some(r(1, 1)));
            assert_eq!(m.consumed(K::ResultUnits), 1);
        }
        let expected = [
            P::RationalArithmeticOperands,
            P::RationalArithmeticArithmetic,
            P::RationalArithmeticNormalize,
            P::RationalArithmeticResultRetain,
        ];
        assert_eq!(m.admitted_charges(), &expected[..work as usize]);
    }
}

/// Trace: TC-910, FR-362-AC-18
#[test]
fn powers_of_two_charge_operand_derived_bounds() {
    for k in [63, 64, 65, 127, 128, 200, 511] {
        let power = I::one().shifted_left(k);
        let below = power.sub(&I::one());
        let above = power.add(&I::one());
        let reference_power = num_bigint::BigInt::from(1_u8) << k;
        let reference_square = &reference_power * &reference_power;
        for (operation, expected, bits) in [
            (
                IA::Multiply(&below, &above),
                I::from_big(&reference_square - 1_u8),
                2 * k + 1,
            ),
            (
                IA::Multiply(&power, &power),
                I::from_big(reference_square),
                2 * k + 2,
            ),
            (
                IA::Add(&below, &I::one()),
                I::from_big(reference_power),
                k + 1,
            ),
            (IA::Subtract(&above, &power), I::one(), k + 2),
            (IA::Subtract(&power, &power), I::zero(), k + 2),
        ] {
            let mut m = meter();
            assert_eq!(
                integer_op(operation, None, &mut m).completed(),
                Some(expected)
            );
            assert_eq!(m.consumed(K::IntegerBits), bits, "k={k}");
        }
    }
}
