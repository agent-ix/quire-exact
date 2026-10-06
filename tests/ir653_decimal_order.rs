// SPDX-License-Identifier: AGPL-3.0-or-later
//! Retained decimal ordering evidence for FR-363.

use core::num::NonZeroU64;

use quire_exact::{
    order_numbers, ChargePoint, Decimal, InjectedDenial, Integer, LimitKind, Meter,
    OrderedOperands, OrderingOperator, Outcome, ScalarLimits,
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

fn decimal(coefficient: i64, scale: u32) -> Decimal {
    Decimal::new(Integer::from(coefficient), scale)
}

fn compare(
    operator: OrderingOperator,
    left: &Decimal,
    right: &Decimal,
    meter: &mut Meter,
) -> Outcome<bool> {
    order_numbers(operator, OrderedOperands::Decimals(left, right), meter)
}

/// Trace: FR-363-AC-1
#[test]
fn decimal_ordering_matches_independent_scaled_integer_oracle() {
    let cases = [
        ((100, 2), (1, 0)),
        ((0, 5), (0, 0)),
        ((120, 2), (2, 0)),
        ((-120, 2), (-2, 0)),
        ((-1, 3), (0, 0)),
        ((1, 3), (0, 0)),
        ((-100, 2), (-2, 1)),
    ];
    for ((a, sa), (b, sb)) in cases {
        let (left, right) = (decimal(a, sa), decimal(b, sb));
        let before = (
            left.representation().clone(),
            right.representation().clone(),
        );
        let common = sa.max(sb);
        let reference = (i128::from(a) * 10_i128.pow(common - sa))
            .cmp(&(i128::from(b) * 10_i128.pow(common - sb)));
        for operator in [
            OrderingOperator::Less,
            OrderingOperator::LessOrEqual,
            OrderingOperator::Greater,
            OrderingOperator::GreaterOrEqual,
        ] {
            assert_eq!(
                compare(operator, &left, &right, &mut Meter::new(limits())).completed(),
                Some(operator.holds(reference)),
                "{a}/{sa} against {b}/{sb}"
            );
            assert_eq!(left.representation(), &before.0);
            assert_eq!(right.representation(), &before.1);
        }
    }
}

/// Trace: FR-363-AC-2, FR-363-AC-3
#[test]
fn decimal_ordering_charges_retained_zero_and_equal_value_representations() {
    for (left, right, operator, expected_bits, expected_digits, expected_shift) in [
        (
            decimal(0, 5),
            decimal(0, 0),
            OrderingOperator::GreaterOrEqual,
            18,
            6,
            5,
        ),
        (
            decimal(100, 2),
            decimal(2, 0),
            OrderingOperator::LessOrEqual,
            9,
            3,
            2,
        ),
    ] {
        let mut meter = Meter::new(limits());
        assert_eq!(
            compare(operator, &left, &right, &mut meter).completed(),
            Some(true)
        );
        assert_eq!(meter.consumed(LimitKind::IntegerBits), expected_bits);
        assert_eq!(meter.consumed(LimitKind::DecimalDigits), expected_digits);
        assert_eq!(meter.consumed(LimitKind::ScaleExpansion), expected_shift);
        assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
        #[cfg(feature = "test-support")]
        assert_eq!(
            meter.admitted_charges(),
            [
                ChargePoint::OrderingOperands,
                ChargePoint::OrderingArithmetic,
                ChargePoint::OrderingResultRetain
            ]
        );
    }
}

/// Trace: FR-363-AC-4
#[test]
fn huge_retained_scale_refuses_analytically_before_comparison() {
    let (left, right) = (decimal(1, 0), decimal(1, u32::MAX));
    let mut configured = limits();
    configured.decimal_digits = 64;
    let mut meter = Meter::new(configured);
    let denial = match compare(OrderingOperator::Less, &left, &right, &mut meter) {
        Outcome::Incomplete(record) => record,
        _ => panic!("large alignment must refuse at arithmetic"),
    };
    assert_eq!(
        (
            denial.charge_point,
            denial.limit_kind,
            denial.limit,
            denial.consumed,
            denial.next_charge
        ),
        (
            ChargePoint::OrderingArithmetic,
            LimitKind::DecimalDigits,
            64,
            1,
            Integer::from(4_294_967_296_u64)
        )
    );
    assert_eq!(meter.admission_count(), 1);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    #[cfg(feature = "test-support")]
    assert_eq!(meter.admitted_charges(), [ChargePoint::OrderingOperands]);
}

/// Trace: FR-363-AC-5
#[test]
fn each_decimal_ordering_point_denies_its_suffix() {
    let (left, right) = (decimal(0, 5), decimal(0, 0));
    let points = [
        ChargePoint::OrderingOperands,
        ChargePoint::OrderingArithmetic,
        ChargePoint::OrderingResultRetain,
    ];
    for (prefix, &point) in points.iter().enumerate() {
        let mut meter = Meter::new(limits()).with_injected_denial(InjectedDenial {
            point,
            occurrence: NonZeroU64::new(1).unwrap(),
        });
        let denial = match compare(OrderingOperator::GreaterOrEqual, &left, &right, &mut meter) {
            Outcome::Incomplete(record) => record,
            _ => panic!("selected point must deny"),
        };
        assert_eq!(
            (
                denial.charge_point,
                denial.limit_kind,
                denial.limit,
                denial.consumed,
                denial.next_charge
            ),
            (
                point,
                LimitKind::WorkUnits,
                prefix as u64,
                prefix as u64,
                Integer::one()
            )
        );
        assert_eq!(meter.admission_count(), prefix as u64);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), prefix as u64);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        #[cfg(feature = "test-support")]
        assert_eq!(meter.admitted_charges(), &points[..prefix]);
    }
}
