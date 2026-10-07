// SPDX-License-Identifier: AGPL-3.0-or-later
//! Retained decimal comparison and rounding evidence for IR-673.

use std::cmp::Ordering;

use quire_exact::{
    evaluate_decimal, order_numbers, Decimal, DecimalOperation, DecimalType, InexactTarget,
    Integer, LimitKind, Meter, OrderedOperands, OrderingOperator, Outcome, Refusal, RoundingMode,
    ScalarLimits,
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

fn decimal(c: i64, s: u32) -> Decimal {
    Decimal::new(Integer::from(c), s)
}

fn target(mode: RoundingMode) -> DecimalType {
    DecimalType::new(Integer::from(-10_i64), Integer::from(10_i64), 0, 0, mode).unwrap()
}

/// Trace: TC-911, FR-363-AC-6
#[test]
fn normalized_equal_decimals_keep_distinct_retained_representations() {
    let (left, right) = (decimal(110, 2), decimal(11, 1));
    assert_eq!(left.compare(&right), Ordering::Equal);
    assert!(left.numerically_equal(&right));
    assert_eq!(
        (left.normalized().coefficient(), left.normalized().scale()),
        (&Integer::from(11_i64), 1)
    );
    assert_eq!(
        (right.normalized().coefficient(), right.normalized().scale()),
        (&Integer::from(11_i64), 1)
    );
    assert_eq!(
        (
            left.representation().coefficient(),
            left.representation().scale()
        ),
        (&Integer::from(110_i64), 2)
    );
    assert_eq!(
        (
            right.representation().coefficient(),
            right.representation().scale()
        ),
        (&Integer::from(11_i64), 1)
    );
}

/// Trace: TC-911, FR-363-AC-7
#[test]
fn metered_ordering_uses_retained_decimal_sizes() {
    for (left, right, bits, digits, expansion) in [
        (decimal(110, 2), decimal(11, 1), 8, 3, 1),
        (decimal(11, 1), decimal(11, 1), 4, 2, 0),
    ] {
        let mut meter = Meter::new(limits());
        assert_eq!(
            order_numbers(
                OrderingOperator::Less,
                OrderedOperands::Decimals(&left, &right),
                &mut meter
            )
            .completed(),
            Some(false)
        );
        for (kind, expected) in [
            (LimitKind::IntegerBits, bits),
            (LimitKind::DecimalDigits, digits),
            (LimitKind::ScaleExpansion, expansion),
            (LimitKind::ValueOccurrences, 2),
            (LimitKind::WorkUnits, 3),
            (LimitKind::ResultUnits, 1),
        ] {
            assert_eq!(meter.consumed(kind), expected, "{kind:?}");
        }
        for kind in [
            LimitKind::TextInputBytes,
            LimitKind::TextScalars,
            LimitKind::NormalizedScalars,
            LimitKind::UnitEdges,
        ] {
            assert_eq!(meter.consumed(kind), 0);
        }
    }
}

/// Trace: TC-913, FR-365-AC-1
#[test]
fn decimal_half_ties_follow_all_six_rounding_modes() {
    for (mode, positive, negative) in [
        (RoundingMode::Exact, None, None),
        (RoundingMode::TowardZero, Some(0), Some(0)),
        (RoundingMode::TowardPositive, Some(1), Some(0)),
        (RoundingMode::TowardNegative, Some(0), Some(-1)),
        (RoundingMode::NearestEven, Some(0), Some(0)),
        (RoundingMode::NearestAway, Some(1), Some(-1)),
    ] {
        let target = target(mode);
        for (input, expected) in [(5, positive), (-5, negative)] {
            let mut meter = Meter::new(limits());
            let outcome = evaluate_decimal(
                DecimalOperation::Round(&decimal(input, 1)),
                &target,
                &mut meter,
            );
            match expected {
                Some(coefficient) => {
                    let value = outcome.completed().expect("rounding must complete");
                    assert_eq!(
                        (
                            value.value().representation().coefficient(),
                            value.value().representation().scale()
                        ),
                        (&Integer::from(coefficient as i64), 0)
                    );
                    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
                }
                None => {
                    assert!(matches!(outcome, Outcome::Refused(Refusal::InexactDecimal {
                        target: InexactTarget::Decimal(actual)
                    }) if *actual == target));
                    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
                }
            }
        }
    }
}

/// Trace: TC-913, FR-365-AC-2
#[test]
fn default_decimal_rounding_is_strict_exact() {
    assert_eq!(RoundingMode::default(), RoundingMode::Exact);
    let target = target(RoundingMode::default());
    assert_eq!(target.rounding(), RoundingMode::Exact);
    let mut meter = Meter::new(limits());
    assert!(
        matches!(evaluate_decimal(DecimalOperation::Round(&decimal(1, 1)), &target, &mut meter),
        Outcome::Refused(Refusal::InexactDecimal {
            target: InexactTarget::Decimal(actual)
        }) if *actual == target)
    );
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}
