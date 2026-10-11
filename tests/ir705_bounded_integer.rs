// SPDX-License-Identifier: AGPL-3.0-or-later
//! Integer representation refinement observations (FR-371).

use core::num::NonZeroU32;
use std::string::ToString;

use quire_exact::{
    evaluate_integer_arithmetic, Integer, IntegerArithmetic, IntegerInterval, Meter, Outcome,
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

/// Trace: FR-371-AC-4
#[test]
fn fitting_integer_operations_and_cached_source_clone_allocate_nothing() {
    let left = Integer::from(123_i128);
    let right = Integer::from(456_i128);
    let domain = IntegerInterval::signed_twos_complement(NonZeroU32::new(16).unwrap());
    let _ = left.as_big();

    let mut observed = None;
    let allocations = allocation_counter::measure(|| {
        let cloned = left.clone();
        observed = Some((
            left.add(&right),
            left.sub(&right),
            left.mul(&right),
            left.neg(),
            cloned,
            left.magnitude_bits(),
            Integer::from(7_i128).to_u64(),
            domain.contains(&left),
        ));
    });
    assert_eq!(allocations.bytes_total, 0);
    let observed = observed.expect("measurement completed");
    assert_eq!(observed.0, Integer::from(579_i128));
    assert_eq!(observed.1, Integer::from(-333_i128));
    assert_eq!(observed.2, Integer::from(56_088_i128));
    assert_eq!(observed.3, Integer::from(-123_i128));
    assert_eq!(observed.4, left);
    assert_eq!(observed.5, 7);
    assert_eq!(observed.6, Some(7));
    assert!(observed.7);
}

/// Trace: FR-371-AC-1, FR-371-AC-4
#[test]
fn inline_signed_bounds_refine_the_explicit_128_bit_domain() {
    let domain = IntegerInterval::signed_twos_complement(NonZeroU32::new(128).unwrap());
    let lower = Integer::from(i128::MIN);
    let upper = Integer::from(i128::MAX);
    assert!(domain.contains(&lower));
    assert!(domain.contains(&upper));
    assert_eq!(domain.admit(lower.clone()).unwrap().value(), &lower);
    assert_eq!(domain.admit(upper.clone()).unwrap().value(), &upper);

    let above = Integer::from_big(upper.as_big() + 1_u8);
    assert_eq!(domain.admit(above), Err(quire_exact::OutOfDomain));
}

/// Trace: FR-371-AC-3
#[test]
fn metered_native_and_overflow_operations_keep_the_existing_charge_schedule() {
    let native_left = Integer::from(8_i128);
    let native_right = Integer::from(3_i128);
    let overflow_left = Integer::from(i128::MAX);
    let overflow_right = Integer::one();

    for (operation, expected, arithmetic_bits, occurrences) in [
        (
            IntegerArithmetic::Add(&native_left, &native_right),
            "11",
            5,
            2,
        ),
        (
            IntegerArithmetic::Subtract(&native_left, &native_right),
            "5",
            5,
            2,
        ),
        (
            IntegerArithmetic::Multiply(&native_left, &native_right),
            "24",
            6,
            2,
        ),
        (IntegerArithmetic::Negate(&native_left), "-8", 4, 1),
        (
            IntegerArithmetic::Add(&overflow_left, &overflow_right),
            "170141183460469231731687303715884105728",
            128,
            2,
        ),
    ] {
        let mut meter = Meter::new(limits());
        let outcome = evaluate_integer_arithmetic(operation, None, &mut meter);
        assert_eq!(
            outcome
                .completed()
                .map(|value| value.to_string())
                .as_deref(),
            Some(expected)
        );
        assert_eq!(
            meter.consumed(quire_exact::LimitKind::IntegerBits),
            arithmetic_bits
        );
        assert_eq!(
            meter.consumed(quire_exact::LimitKind::ValueOccurrences),
            occurrences
        );
        assert_eq!(meter.consumed(quire_exact::LimitKind::WorkUnits), 3);
        assert_eq!(meter.consumed(quire_exact::LimitKind::ResultUnits), 1);
    }

    let excluded = IntegerInterval::new(Integer::zero(), Integer::from(i128::MAX)).unwrap();
    let mut meter = Meter::new(limits());
    assert!(matches!(
        evaluate_integer_arithmetic(
            IntegerArithmetic::Add(&overflow_left, &overflow_right),
            Some(&excluded),
            &mut meter,
        ),
        Outcome::Refused(_)
    ));
    assert_eq!(meter.consumed(quire_exact::LimitKind::ResultUnits), 0);
    assert_eq!(meter.consumed(quire_exact::LimitKind::WorkUnits), 2);

    let mut one_less = limits();
    one_less.integer_bits = 4;
    let mut meter = Meter::new(one_less);
    let incomplete = match evaluate_integer_arithmetic(
        IntegerArithmetic::Add(&native_left, &native_right),
        None,
        &mut meter,
    ) {
        Outcome::Incomplete(record) => record,
        other => panic!("expected arithmetic denial, got {other:?}"),
    };
    assert_eq!(
        incomplete.charge_point,
        quire_exact::ChargePoint::IntegerArithmeticArithmetic
    );
    assert_eq!(incomplete.limit, 4);
    assert_eq!(incomplete.consumed, 4);
    assert_eq!(incomplete.next_charge, Integer::from(5_u64));
    assert_eq!(meter.consumed(quire_exact::LimitKind::WorkUnits), 1);
    assert_eq!(meter.consumed(quire_exact::LimitKind::ResultUnits), 0);
}
