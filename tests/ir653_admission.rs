// SPDX-License-Identifier: AGPL-3.0-or-later
//! Denied size charges precede large exact allocation requests (FR-361).
//!
//! `allocation_counter` supplies the observer outside this unsafe-free crate.
//! Total requested bytes bound every single request, so this test asserts the
//! stronger total bound in each measurement window.

use core::num::NonZeroU64;

use quire_exact::{
    divide, evaluate_decimal, evaluate_integer_arithmetic, evaluate_rational_arithmetic, modulo,
    ChargePoint, Decimal, DecimalOperation, DecimalType, DivisionMember, DivisionProfile,
    InjectedDenial, Integer, IntegerArithmetic, IntegerDomain, LimitKind, Meter, Outcome, Rational,
    RationalArithmetic, RoundingMode, ScalarLimits,
};

const LARGE_BITS: u64 = 65_536;
const DECIMAL_SHIFT: u32 = 1 << 20;

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

fn denial<T>(outcome: Outcome<T>, point: ChargePoint, kind: LimitKind, amount: Integer) {
    match outcome {
        Outcome::Incomplete(record) => {
            assert_eq!(record.charge_point, point);
            assert_eq!(record.limit_kind, kind);
            assert_eq!(record.next_charge, amount);
        }
        _ => panic!("expected a denied charge"),
    }
}

fn measure<T>(call: impl FnOnce() -> T) -> (T, u64) {
    let mut outcome = None;
    let allocations = allocation_counter::measure(|| outcome = Some(call()));
    (
        outcome.expect("measurement returned"),
        allocations.bytes_total,
    )
}

fn assert_small(total_bytes: u64, ceiling: u64) {
    assert!(
        total_bytes < ceiling,
        "{total_bytes} requested bytes, ceiling {ceiling}"
    );
}

/// Trace: FR-361-AC-1
#[test]
fn integer_arithmetic_denial_precedes_large_intermediate_even_when_result_cancels() {
    let large = Integer::one().shifted_left(LARGE_BITS - 1);
    let one = Integer::one();
    let almost = large.sub(&one);
    let ceiling = (LARGE_BITS / 8) / 8;
    for (operation, charge_bits) in [
        (IntegerArithmetic::Multiply(&large, &large), LARGE_BITS * 2),
        (IntegerArithmetic::Subtract(&large, &almost), LARGE_BITS + 1),
    ] {
        let mut configured = limits();
        configured.integer_bits = charge_bits - 1;
        let mut meter = Meter::new(configured);
        let (outcome, requested) =
            measure(|| evaluate_integer_arithmetic(operation, None, &mut meter));
        denial(
            outcome,
            ChargePoint::IntegerArithmeticArithmetic,
            LimitKind::IntegerBits,
            Integer::from(charge_bits),
        );
        assert_eq!(meter.admission_count(), 1);
        assert_small(requested, ceiling);
    }
}

/// Trace: FR-361-AC-2
#[test]
fn rational_arithmetic_denial_precedes_large_unreduced_products() {
    let large = Integer::one().shifted_left(LARGE_BITS - 1);
    let one = Integer::one();
    let huge = Rational::new(large.clone(), one.clone()).unwrap();
    let reciprocal = Rational::new(one.clone(), large.clone()).unwrap();
    let other = Rational::new(Integer::from(3_i64), Integer::from(2_i64)).unwrap();
    let ceiling = (LARGE_BITS / 8) / 8;
    for (operation, charge_bits) in [
        (
            RationalArithmetic::Multiply(&huge, &reciprocal),
            LARGE_BITS + 1,
        ),
        (RationalArithmetic::Add(&huge, &other), LARGE_BITS + 3),
    ] {
        let mut configured = limits();
        configured.integer_bits = charge_bits - 1;
        let mut meter = Meter::new(configured);
        let (outcome, requested) =
            measure(|| evaluate_rational_arithmetic(operation, None, &mut meter));
        denial(
            outcome,
            ChargePoint::RationalArithmeticArithmetic,
            LimitKind::IntegerBits,
            Integer::from(charge_bits),
        );
        assert_eq!(meter.admission_count(), 1);
        assert_small(requested, ceiling);
    }
}

/// Trace: FR-361-AC-3
#[test]
fn division_and_modulus_denial_precedes_large_quotient() {
    let large = Integer::one().shifted_left(LARGE_BITS - 1);
    let three = Integer::from(3_i64);
    let ceiling = (LARGE_BITS / 8) / 8;
    for point in [
        ChargePoint::IntegerDivisionArithmetic,
        ChargePoint::IntegerModulusArithmetic,
    ] {
        let mut meter = Meter::new(limits()).with_injected_denial(InjectedDenial {
            point,
            occurrence: NonZeroU64::new(1).unwrap(),
        });
        let (outcome, requested) = measure(|| {
            if point == ChargePoint::IntegerDivisionArithmetic {
                divide(
                    DivisionProfile::Truncating,
                    DivisionMember::Quotient,
                    &large,
                    &three,
                    &IntegerDomain::Mathematical,
                    &mut meter,
                )
            } else {
                modulo(&large, &three, &IntegerDomain::Mathematical, &mut meter)
            }
        });
        denial(outcome, point, LimitKind::WorkUnits, Integer::one());
        assert_eq!(meter.admission_count(), 1);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        #[cfg(feature = "test-support")]
        assert_eq!(
            meter
                .admitted_charges()
                .iter()
                .map(|charge| charge.point)
                .collect::<Vec<_>>(),
            &[if point == ChargePoint::IntegerDivisionArithmetic {
                ChargePoint::IntegerDivisionOperands
            } else {
                ChargePoint::IntegerModulusOperands
            }]
        );
        assert_small(requested, ceiling);
    }
}

fn decimal(value: i64, scale: u32) -> Decimal {
    Decimal::new(Integer::from(value), scale)
}

fn target(scale: u32) -> DecimalType {
    DecimalType::new(
        Integer::from(-10_i64),
        Integer::from(10_i64),
        0,
        u64::from(scale),
        RoundingMode::Exact,
    )
    .unwrap()
}

/// Trace: FR-361-AC-4
#[test]
fn decimal_scale_denial_precedes_power_of_ten_allocation() {
    let (left, right) = (decimal(1, 0), decimal(1, DECIMAL_SHIFT));
    let mut configured = limits();
    configured.scale_expansion = u64::from(DECIMAL_SHIFT) - 1;
    let mut meter = Meter::new(configured);
    let target = target(DECIMAL_SHIFT);
    let (outcome, requested) =
        measure(|| evaluate_decimal(DecimalOperation::Add(&left, &right), &target, &mut meter));
    denial(
        outcome,
        ChargePoint::DecimalScaleExpansion,
        LimitKind::ScaleExpansion,
        Integer::from(u64::from(DECIMAL_SHIFT)),
    );
    assert_eq!(meter.admission_count(), 1);
    assert_small(requested, 4096);
}

/// Trace: FR-361-AC-5
#[test]
fn decimal_arithmetic_denial_precedes_aligned_intermediate() {
    let (left, right) = (decimal(1, 0), decimal(1, DECIMAL_SHIFT));
    // B(10^(2^20)) = 3,483,295, so scale expansion charges 3,483,296
    // and addition arithmetic charges one more bit.
    let scale_bits = 3_483_296_u64;
    let mut configured = limits();
    configured.integer_bits = scale_bits;
    let mut meter = Meter::new(configured);
    let target = target(DECIMAL_SHIFT);
    let (outcome, requested) =
        measure(|| evaluate_decimal(DecimalOperation::Add(&left, &right), &target, &mut meter));
    denial(
        outcome,
        ChargePoint::DecimalArithmetic,
        LimitKind::IntegerBits,
        Integer::from(scale_bits + 1),
    );
    assert_eq!(meter.admission_count(), 2);
    assert_small(requested, 4096);
}

/// Trace: FR-361-AC-6
#[test]
fn decimal_retain_denial_precedes_target_scale_expansion() {
    let input = decimal(1, 0);
    let target = target(DECIMAL_SHIFT);
    let mut configured = limits();
    configured.decimal_digits = u64::from(DECIMAL_SHIFT);
    let mut meter = Meter::new(configured);
    let (outcome, requested) =
        measure(|| evaluate_decimal(DecimalOperation::Round(&input), &target, &mut meter));
    denial(
        outcome,
        ChargePoint::DecimalResultRetain,
        LimitKind::DecimalDigits,
        Integer::from(u64::from(DECIMAL_SHIFT) + 1),
    );
    assert_eq!(meter.admission_count(), 3);
    assert_small(requested, 4096);
}

/// Trace: TC-909, FR-361-AC-7, FR-361-AC-8
#[test]
fn large_floor_quotient_completes_at_exact_operand_bit_limit_and_denies_below() {
    let dividend = Integer::one().shifted_left(LARGE_BITS - 1);
    let divisor = Integer::from(3_i64);
    let reference = Integer::from_big(dividend.as_big() / divisor.as_big());
    let mut configured = limits();
    configured.integer_bits = LARGE_BITS;
    let mut meter = Meter::new(configured);
    assert_eq!(
        divide(
            DivisionProfile::Floor,
            DivisionMember::Quotient,
            &dividend,
            &divisor,
            &IntegerDomain::Mathematical,
            &mut meter
        )
        .completed(),
        Some(reference)
    );
    assert_eq!(meter.consumed(LimitKind::IntegerBits), LARGE_BITS);
    configured.integer_bits = LARGE_BITS - 1;
    let mut meter = Meter::new(configured);
    let (outcome, bytes) = measure(|| {
        divide(
            DivisionProfile::Floor,
            DivisionMember::Quotient,
            &dividend,
            &divisor,
            &IntegerDomain::Mathematical,
            &mut meter,
        )
    });
    let Outcome::Incomplete(stop) = outcome else {
        panic!("operand charge must stop")
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
            ChargePoint::IntegerDivisionOperands,
            LimitKind::IntegerBits,
            LARGE_BITS - 1,
            0,
            Integer::from(LARGE_BITS)
        )
    );
    assert_eq!(meter.admission_count(), 0);
    assert_small(bytes, (LARGE_BITS / 8) / 8);
}

/// Trace: TC-909, FR-361-AC-9
#[test]
fn huge_decimal_target_denies_retain_before_power_allocation() {
    let (left, right) = (decimal(1, 0), decimal(1, 0));
    let target = DecimalType::new(
        Integer::from(0_i64),
        Integer::from(1_i64),
        0,
        u64::from(DECIMAL_SHIFT),
        RoundingMode::Exact,
    )
    .unwrap();
    let mut configured = limits();
    configured.decimal_digits = u64::from(DECIMAL_SHIFT);
    let mut meter = Meter::new(configured);
    let (outcome, bytes) = measure(|| {
        evaluate_decimal(
            DecimalOperation::Multiply(&left, &right),
            &target,
            &mut meter,
        )
    });
    let Outcome::Incomplete(stop) = outcome else {
        panic!("result retention must stop")
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
            ChargePoint::DecimalResultRetain,
            LimitKind::DecimalDigits,
            u64::from(DECIMAL_SHIFT),
            2,
            Integer::from(u64::from(DECIMAL_SHIFT) + 1)
        )
    );
    assert_eq!(meter.admission_count(), 3);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    #[cfg(feature = "test-support")]
    assert_eq!(
        meter
            .admitted_charges()
            .iter()
            .map(|charge| charge.point)
            .collect::<Vec<_>>(),
        [
            ChargePoint::DecimalOperands,
            ChargePoint::DecimalScaleExpansion,
            ChargePoint::DecimalArithmetic
        ]
    );
    assert_small(bytes, 4096);
}
