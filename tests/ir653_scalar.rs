// SPDX-License-Identifier: AGPL-3.0-or-later
//! Independent scalar value and charge evidence for FR-362.

use core::num::NonZeroU64;
use core::str::FromStr;

use quire_exact::{
    evaluate_boolean, evaluate_integer_arithmetic, evaluate_rational_arithmetic, order_numbers,
    retain_boolean, BooleanConnective, ChargePoint, InjectedDenial, Integer, IntegerArithmetic,
    LimitKind, Meter, OrderedOperands, OrderingOperator, Outcome, Rational, RationalArithmetic,
    ScalarLimits, Undefined,
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

fn rational(n: i128, d: i128) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}

fn bit_len(n: i128) -> u64 {
    let magnitude = n.unsigned_abs();
    u64::from(128 - magnitude.leading_zeros()).max(1)
}

fn samples() -> Vec<i128> {
    let mut values = vec![i128::MIN, -1, 0, 1, i128::MAX];
    let mut state = 0x91e1_0da5_c79e_7b1d_u64;
    for _ in 0..64 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        values.push(i128::from((state >> 16) as i32));
    }
    values
}

fn completed_integer(operation: IntegerArithmetic<'_>) -> Integer {
    evaluate_integer_arithmetic(operation, None, &mut meter())
        .completed()
        .expect("unbounded integer arithmetic completes")
}

/// Trace: FR-362-AC-1
#[test]
fn integer_arithmetic_matches_checked_i128_oracle() {
    let values = samples();
    for &a in &values {
        let left = Integer::from(a);
        if let Some(expected) = a.checked_neg() {
            assert_eq!(
                completed_integer(IntegerArithmetic::Negate(&left)),
                Integer::from(expected)
            );
        }
        for &b in &values {
            let right = Integer::from(b);
            for (expected, operation) in [
                (a.checked_add(b), IntegerArithmetic::Add(&left, &right)),
                (a.checked_sub(b), IntegerArithmetic::Subtract(&left, &right)),
                (a.checked_mul(b), IntegerArithmetic::Multiply(&left, &right)),
            ] {
                if let Some(expected) = expected {
                    assert_eq!(
                        completed_integer(operation),
                        Integer::from(expected),
                        "{a} {b}"
                    );
                }
            }
        }
    }
}

/// Trace: FR-362-AC-2
#[test]
fn integer_arithmetic_retains_results_beyond_i128() {
    let maximum = Integer::from(i128::MAX);
    let minimum = Integer::from(i128::MIN);
    assert_eq!(
        completed_integer(IntegerArithmetic::Multiply(&maximum, &Integer::from(2_i64))),
        Integer::from_str("340282366920938463463374607431768211454").unwrap()
    );
    assert_eq!(
        completed_integer(IntegerArithmetic::Negate(&minimum)),
        Integer::from_str("170141183460469231731687303715884105728").unwrap()
    );
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn reduced(n: i128, d: i128) -> (i128, i128) {
    let divisor = gcd(n, d);
    (n / divisor, d / divisor)
}

fn rational_parts(value: Rational) -> (Integer, Integer) {
    (value.numerator().clone(), value.denominator().clone())
}

/// Trace: FR-362-AC-3
#[test]
fn rational_arithmetic_matches_reduced_fraction_oracle() {
    let mut state = 0x7f4a_7c15_u64;
    let mut cases = vec![(0, 1, 1, 2), (-1, 2, 2, 3), (7, 5, -3, 4)];
    for _ in 0..64 {
        let mut next = || {
            state = state
                .wrapping_mul(2862933555777941757)
                .wrapping_add(3037000493);
            i128::from((state >> 32) as i16)
        };
        cases.push((
            next(),
            ((next().unsigned_abs() % 97) + 1) as i128,
            next(),
            ((next().unsigned_abs() % 97) + 1) as i128,
        ));
    }
    for (a, p, c, q) in cases {
        let left = rational(a, p);
        let right = rational(c, q);
        let checks = [
            (
                reduced(a * q + c * p, p * q),
                RationalArithmetic::Add(&left, &right),
            ),
            (
                reduced(a * q - c * p, p * q),
                RationalArithmetic::Subtract(&left, &right),
            ),
            (
                reduced(a * c, p * q),
                RationalArithmetic::Multiply(&left, &right),
            ),
        ];
        for ((n, d), operation) in checks {
            let actual = evaluate_rational_arithmetic(operation, None, &mut meter())
                .completed()
                .unwrap();
            assert_eq!(rational_parts(actual), (Integer::from(n), Integer::from(d)));
        }
        let negated =
            evaluate_rational_arithmetic(RationalArithmetic::Negate(&left), None, &mut meter())
                .completed()
                .unwrap();
        assert_eq!(rational_parts(negated), {
            let (n, d) = reduced(-a, p);
            (Integer::from(n), Integer::from(d))
        });
        if c != 0 {
            let (mut n, mut d) = reduced(a * q, p * c);
            if d < 0 {
                (n, d) = (-n, -d);
            }
            let actual = evaluate_rational_arithmetic(
                RationalArithmetic::Divide(&left, &right),
                None,
                &mut meter(),
            )
            .completed()
            .unwrap();
            assert_eq!(rational_parts(actual), (Integer::from(n), Integer::from(d)));
        }
    }
    let zero = rational(0, 1);
    let mut meter = meter();
    assert_eq!(
        evaluate_rational_arithmetic(
            RationalArithmetic::Divide(&rational(1, 2), &zero),
            None,
            &mut meter
        ),
        Outcome::Undefined(Undefined::DivisionByZero)
    );
    assert_eq!(meter.admission_count(), 1);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}

/// Trace: FR-362-AC-4
#[test]
fn integer_and_rational_ordering_match_checked_oracle() {
    let values = samples();
    for pair in values.windows(2).chain(values.chunks_exact(2)) {
        let (a, b) = (pair[0], pair[1]);
        let (left, right) = (Integer::from(a), Integer::from(b));
        let rational_left = rational(a, 97);
        let rational_right = rational(b, 89);
        assert_eq!(left == right, a == b);
        let mut checks = vec![(OrderedOperands::Integers(&left, &right), a.cmp(&b))];
        if let (Some(x), Some(y)) = (a.checked_mul(89), b.checked_mul(97)) {
            assert_eq!(rational_left == rational_right, x == y);
            checks.push((
                OrderedOperands::Rationals(&rational_left, &rational_right),
                x.cmp(&y),
            ));
        }
        for (operands, expected) in checks {
            for operator in [
                OrderingOperator::Less,
                OrderingOperator::LessOrEqual,
                OrderingOperator::Greater,
                OrderingOperator::GreaterOrEqual,
            ] {
                assert_eq!(
                    order_numbers(operator, operands, &mut meter()).completed(),
                    Some(operator.holds(expected))
                );
            }
        }
    }
}

#[cfg(feature = "test-support")]
fn assert_charges(meter: &Meter, points: &[ChargePoint], bits: u64, occurrences: u64) {
    assert_eq!(meter.admitted_charges(), points);
    assert_eq!(meter.consumed(LimitKind::IntegerBits), bits);
    for kind in [
        LimitKind::DecimalDigits,
        LimitKind::ScaleExpansion,
        LimitKind::TextInputBytes,
        LimitKind::TextScalars,
        LimitKind::NormalizedScalars,
        LimitKind::UnitEdges,
    ] {
        assert_eq!(meter.consumed(kind), 0, "{kind:?}");
    }
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), occurrences);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), points.len() as u64);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
}

/// Trace: FR-362-AC-5, FR-362-AC-6
#[cfg(feature = "test-support")]
#[test]
fn scalar_families_charge_order_and_independent_size_maxima() {
    let (a, b) = (Integer::from(5_i64), Integer::from(7_i64));
    let ints = [
        (IntegerArithmetic::Add(&a, &b), bit_len(7) + 1),
        (IntegerArithmetic::Subtract(&a, &b), bit_len(7) + 1),
        (IntegerArithmetic::Multiply(&a, &b), bit_len(5) + bit_len(7)),
        (IntegerArithmetic::Negate(&a), bit_len(5)),
    ];
    for (operation, expected_bits) in ints {
        let mut meter = meter();
        assert!(matches!(
            evaluate_integer_arithmetic(operation, None, &mut meter),
            Outcome::Completed(_)
        ));
        assert_charges(
            &meter,
            &[
                ChargePoint::IntegerArithmeticOperands,
                ChargePoint::IntegerArithmeticArithmetic,
                ChargePoint::IntegerArithmeticResultRetain,
            ],
            expected_bits,
            if matches!(operation, IntegerArithmetic::Negate(_)) {
                1
            } else {
                2
            },
        );
    }
    let (left, right) = (rational(1, 2), rational(2, 3));
    // The final columns are the independently formed, unreduced parts.
    let rationals = [
        (RationalArithmetic::Add(&left, &right), 5, 7, 6),
        (RationalArithmetic::Subtract(&left, &right), 5, -1, 6),
        (RationalArithmetic::Multiply(&left, &right), 4, 2, 6),
        (RationalArithmetic::Divide(&left, &right), 4, 3, 4),
        (RationalArithmetic::Negate(&left), 2, -1, 2),
    ];
    for (operation, arithmetic_bits, numerator, denominator) in rationals {
        let mut meter = meter();
        assert!(matches!(
            evaluate_rational_arithmetic(operation, None, &mut meter),
            Outcome::Completed(_)
        ));
        let expected_bits = arithmetic_bits
            .max(bit_len(numerator))
            .max(bit_len(denominator));
        assert_charges(
            &meter,
            &[
                ChargePoint::RationalArithmeticOperands,
                ChargePoint::RationalArithmeticArithmetic,
                ChargePoint::RationalArithmeticNormalize,
                ChargePoint::RationalArithmeticResultRetain,
            ],
            expected_bits,
            if matches!(operation, RationalArithmetic::Negate(_)) {
                1
            } else {
                2
            },
        );
    }
    for (operands, bits) in [
        (OrderedOperands::Integers(&a, &b), 3),
        (OrderedOperands::Rationals(&left, &right), 4),
    ] {
        let mut meter = meter();
        assert!(matches!(
            order_numbers(OrderingOperator::Less, operands, &mut meter),
            Outcome::Completed(_)
        ));
        assert_charges(
            &meter,
            &[
                ChargePoint::OrderingOperands,
                ChargePoint::OrderingArithmetic,
                ChargePoint::OrderingResultRetain,
            ],
            bits,
            2,
        );
    }
    let mut meter = meter();
    assert_eq!(retain_boolean(true, &mut meter), Ok(true));
    assert_charges(&meter, &[ChargePoint::BooleanResultRetain], 0, 0);
}

/// Trace: FR-362-AC-7
#[test]
fn three_direct_atoms_consume_seven_work_and_three_results() {
    let mut meter = meter();
    let (a, b) = (Integer::from(5_i64), Integer::from(7_i64));
    assert_eq!(
        evaluate_integer_arithmetic(IntegerArithmetic::Add(&a, &b), None, &mut meter).completed(),
        Some(Integer::from(12_i64))
    );
    assert_eq!(
        order_numbers(
            OrderingOperator::Less,
            OrderedOperands::Rationals(&rational(1, 2), &rational(2, 3)),
            &mut meter
        )
        .completed(),
        Some(true)
    );
    assert_eq!(retain_boolean(true, &mut meter), Ok(true));
    assert_eq!(meter.admission_count(), 7);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 7);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 3);
}

/// Trace: FR-362-AC-8
#[test]
fn each_scalar_charge_point_can_stop_before_result() {
    let (a, b) = (Integer::from(5_i64), Integer::from(7_i64));
    let (left, right) = (rational(1, 2), rational(2, 3));
    for (points, family) in [
        (
            &[
                ChargePoint::IntegerArithmeticOperands,
                ChargePoint::IntegerArithmeticArithmetic,
                ChargePoint::IntegerArithmeticResultRetain,
            ][..],
            0,
        ),
        (
            &[
                ChargePoint::RationalArithmeticOperands,
                ChargePoint::RationalArithmeticArithmetic,
                ChargePoint::RationalArithmeticNormalize,
                ChargePoint::RationalArithmeticResultRetain,
            ][..],
            1,
        ),
        (
            &[
                ChargePoint::OrderingOperands,
                ChargePoint::OrderingArithmetic,
                ChargePoint::OrderingResultRetain,
            ][..],
            2,
        ),
        (
            &[
                ChargePoint::OrderingOperands,
                ChargePoint::OrderingArithmetic,
                ChargePoint::OrderingResultRetain,
            ][..],
            3,
        ),
        (&[ChargePoint::BooleanResultRetain][..], 4),
    ] {
        for (index, &point) in points.iter().enumerate() {
            let mut meter = meter().with_injected_denial(InjectedDenial {
                point,
                occurrence: NonZeroU64::new(1).unwrap(),
            });
            let denial = match family {
                0 => evaluate_integer_arithmetic(IntegerArithmetic::Add(&a, &b), None, &mut meter)
                    .map_incomplete(),
                1 => evaluate_rational_arithmetic(
                    RationalArithmetic::Add(&left, &right),
                    None,
                    &mut meter,
                )
                .map_incomplete(),
                2 => order_numbers(
                    OrderingOperator::Less,
                    OrderedOperands::Integers(&a, &b),
                    &mut meter,
                )
                .map_incomplete(),
                3 => order_numbers(
                    OrderingOperator::Less,
                    OrderedOperands::Rationals(&left, &right),
                    &mut meter,
                )
                .map_incomplete(),
                4 => evaluate_boolean(BooleanConnective::Not(false), &mut meter).map_incomplete(),
                _ => unreachable!(),
            };
            assert_eq!(denial.charge_point, point);
            assert_eq!(denial.limit_kind, LimitKind::WorkUnits);
            assert_eq!(
                (denial.limit, denial.consumed, denial.next_charge),
                (index as u64, index as u64, Integer::one())
            );
            assert_eq!(meter.admission_count(), index as u64);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
            #[cfg(feature = "test-support")]
            assert_eq!(meter.admitted_charges(), &points[..index]);
        }
    }
}

trait IncompleteOutcome {
    fn map_incomplete(self) -> quire_exact::Incomplete;
}

impl<T> IncompleteOutcome for Outcome<T> {
    fn map_incomplete(self) -> quire_exact::Incomplete {
        match self {
            Outcome::Incomplete(record) => record,
            _ => panic!("expected an incomplete outcome"),
        }
    }
}

/// Trace: FR-362-AC-9
#[test]
fn already_decided_boolean_connectives_follow_truth_tables_and_one_charge() {
    for left in [false, true] {
        for right in [false, true] {
            for (connective, expected) in [
                (BooleanConnective::And(left, right), left && right),
                (BooleanConnective::Or(left, right), left || right),
                (BooleanConnective::Implies(left, right), !left || right),
            ] {
                let mut meter = meter();
                assert_eq!(
                    evaluate_boolean(connective, &mut meter).completed(),
                    Some(expected)
                );
                assert_eq!(
                    (
                        meter.consumed(LimitKind::WorkUnits),
                        meter.consumed(LimitKind::ResultUnits)
                    ),
                    (1, 1)
                );
                #[cfg(feature = "test-support")]
                assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
            }
        }
        let mut meter = meter();
        assert_eq!(
            evaluate_boolean(BooleanConnective::Not(left), &mut meter).completed(),
            Some(!left)
        );
        assert_eq!(
            (
                meter.consumed(LimitKind::WorkUnits),
                meter.consumed(LimitKind::ResultUnits)
            ),
            (1, 1)
        );
        #[cfg(feature = "test-support")]
        assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
    }
}
