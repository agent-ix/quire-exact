// SPDX-License-Identifier: AGPL-3.0-or-later
//! Integer representation refinement observations (FR-371).

use core::num::{NonZeroU32, NonZeroU64};
use ix_trace_rs::trace;
use num_bigint::BigInt;
use std::{str::FromStr, string::ToString};

#[cfg(not(feature = "test-support"))]
use quire_exact::Charge;
use quire_exact::{
    evaluate_integer_arithmetic, order_numbers, ChargePoint, InjectedDenial, Integer,
    IntegerArithmetic, IntegerInterval, LimitKind, Meter, OrderedOperands, OrderingOperator,
    Outcome, ScalarLimits,
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

/// Trace: FR-371-AC-1
#[test]
fn construction_boundaries_and_sampled_arithmetic_match_independent_oracles() {
    for (value, spelling) in [
        (i128::MIN, "-170141183460469231731687303715884105728"),
        (i128::MAX, "170141183460469231731687303715884105727"),
        (-1, "-1"),
        (0, "0"),
        (1, "1"),
    ] {
        let primitive = Integer::from(value);
        let parsed = Integer::from_str(spelling).unwrap();
        let from_big = Integer::from_big(BigInt::from(value));
        assert_eq!(primitive.to_string(), spelling);
        assert_eq!(parsed, primitive);
        assert_eq!(from_big, primitive);
    }

    for noncanonical in ["", "-", "+1", "01", "-0", " 1", "1 "] {
        assert!(Integer::from_str(noncanonical).is_err(), "{noncanonical:?}");
    }

    let min = Integer::from(i128::MIN);
    let max = Integer::from(i128::MAX);
    assert!(min.is_negative());
    assert_eq!(min.magnitude_bits(), 128);
    assert_eq!(max.magnitude_bits(), 127);
    assert_eq!(Integer::zero().to_u64(), Some(0));
    assert_eq!(Integer::one().to_u64(), Some(1));
    assert_eq!(min.to_u64(), None);
    let min_squared = Integer::from(i64::MIN).mul(&Integer::from(i64::MIN));
    assert_eq!(
        min_squared.to_string(),
        "85070591730234615865843651857942052864"
    );
    assert_eq!(
        max.add(&Integer::one()).to_string(),
        "170141183460469231731687303715884105728"
    );
    assert_eq!(
        min.sub(&Integer::one()).to_string(),
        "-170141183460469231731687303715884105729"
    );
    assert_eq!(
        min.neg().to_string(),
        "170141183460469231731687303715884105728"
    );

    let two_to_200 = Integer::from_big(BigInt::from(1_u8) << 200_u32);
    let left = two_to_200.add(&Integer::from(123_i64));
    let right = two_to_200.add(&Integer::from(42_i64));
    assert_eq!(left.sub(&right).to_string(), "81");

    // Xorshift32 is deterministic and keeps every reference operation in i64,
    // making checked i128 arithmetic an independent oracle for fitting cases.
    let mut sample = 0x6d2b_79f5_u32;
    for _ in 0..128 {
        sample ^= sample << 13;
        sample ^= sample >> 17;
        sample ^= sample << 5;
        let left = i64::from(sample as i32);
        sample ^= sample << 13;
        sample ^= sample >> 17;
        sample ^= sample << 5;
        let right = i64::from(sample as i32);
        let left_integer = Integer::from(left as i128);
        let right_integer = Integer::from(right as i128);

        for (actual, expected) in [
            (
                left_integer.add(&right_integer),
                i128::from(left).checked_add(i128::from(right)),
            ),
            (
                left_integer.sub(&right_integer),
                i128::from(left).checked_sub(i128::from(right)),
            ),
            (
                left_integer.mul(&right_integer),
                i128::from(left).checked_mul(i128::from(right)),
            ),
        ] {
            if let Some(expected) = expected {
                assert_eq!(actual, Integer::from(expected));
            }
        }
        if let Some(expected) = i128::from(left).checked_neg() {
            assert_eq!(left_integer.neg(), Integer::from(expected));
        }

        let spelling = left.to_string();
        assert_eq!(Integer::from_str(&spelling).unwrap(), left_integer);
        assert_eq!(Integer::from_big(BigInt::from(left)), left_integer);
    }
}

#[test]
#[trace("FR-371-AC-2")]
fn structural_comparisons_match_independent_bigint_order_across_all_storage_pairs() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut cases = Vec::new();
    for value in [i64::MIN, -1, 0, 1, i64::MAX] {
        cases.push((Integer::from(value), BigInt::from(value)));
        cases.push((Integer::from(i128::from(value)), BigInt::from(value)));
    }
    for value in [i128::MIN, i128::MAX] {
        cases.push((Integer::from(value), BigInt::from(value)));
    }
    for expected in [
        BigInt::from(i128::MIN) - 1_u8,
        BigInt::from(i128::MAX) + 1_u8,
        -(BigInt::from(1_u8) << 200_u32),
        BigInt::from(1_u8) << 200_u32,
    ] {
        cases.push((Integer::from_big(expected.clone()), expected));
    }
    for materialized in [false, true] {
        if materialized {
            for (actual, expected) in &cases {
                assert_eq!(actual.as_big(), expected);
            }
        }
        for (left, left_reference) in &cases {
            assert_eq!(left.cmp(left), core::cmp::Ordering::Equal);
            for (right, right_reference) in &cases {
                let expected = left_reference.cmp(right_reference);
                assert_eq!(left.cmp(right), expected);
                assert_eq!(left.partial_cmp(right), Some(expected));
                assert_eq!(left == right, left_reference == right_reference);
                assert_eq!(right.cmp(left), expected.reverse());
            }
        }
        for (actual, expected) in &cases {
            let cloned = actual.clone();
            assert_eq!(&cloned, actual);
            assert_eq!(cloned.as_big(), expected);
            let mut actual_hash = DefaultHasher::new();
            let mut reference_hash = DefaultHasher::new();
            actual.hash(&mut actual_hash);
            expected.hash(&mut reference_hash);
            assert_eq!(actual_hash.finish(), reference_hash.finish());
            assert_eq!(actual.to_string(), expected.to_string());
            assert_eq!(format!("{actual:?}"), format!("Integer({expected:?})"));
        }
    }
}

// Resource diagnostic for mixed comparisons, not an assertion that AC-4
// already promises allocation-free comparisons for every Large input.
#[test]
#[trace("FR-371-AC-2")]
fn mixed_large_inline_comparisons_do_not_allocate_inline_bigint_images() {
    for reference in [
        BigInt::from(i128::MIN) - 1_u8,
        BigInt::from(i128::MAX) + 1_u8,
        -(BigInt::from(1_u8) << 200_u32),
        BigInt::from(1_u8) << 200_u32,
    ] {
        let large = Integer::from_big(reference.clone());
        for (inline, inline_reference) in [
            (Integer::from(i64::MIN), BigInt::from(i64::MIN)),
            (Integer::from(i64::MAX), BigInt::from(i64::MAX)),
            (Integer::from(i128::MIN), BigInt::from(i128::MIN)),
            (Integer::from(i128::MAX), BigInt::from(i128::MAX)),
        ] {
            let expected = inline_reference.cmp(&reference);
            let mut observed = None;
            let allocations = allocation_counter::measure(|| {
                observed = Some((
                    inline == large,
                    large == inline,
                    inline.cmp(&large),
                    large.cmp(&inline),
                ));
            });
            assert_eq!(allocations.count_total, 0);
            assert_eq!(allocations.bytes_total, 0);
            let (left_equal, right_equal, left_order, right_order) = observed.unwrap();
            assert!(!left_equal);
            assert!(!right_equal);
            assert_eq!(left_order, expected);
            assert_eq!(right_order, expected.reverse());
        }
    }
}

#[test]
#[trace("FR-371-AC-1", "FR-371-AC-2")]
fn mixed_constructor_pairs_preserve_arithmetic_and_public_observations() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    for left in [i64::MIN, -1, 0, 1, i64::MAX] {
        let narrow = Integer::from(left);
        let wide = Integer::from(i128::from(left));
        let parsed = Integer::from_str(&left.to_string()).unwrap();
        let from_big = Integer::from_big(BigInt::from(left));
        for materialized in [false, true] {
            if materialized {
                assert_eq!(narrow.as_big(), &BigInt::from(left));
                assert_eq!(wide.as_big(), &BigInt::from(left));
            }
            for a in [&narrow, &wide, &parsed, &from_big] {
                assert_eq!(a, &narrow);
                assert_eq!(a.cmp(&wide), core::cmp::Ordering::Equal);
                assert_eq!(a.to_string(), left.to_string());
                assert_eq!(format!("{a:?}"), format!("{narrow:?}"));
                let mut actual = DefaultHasher::new();
                let mut reference = DefaultHasher::new();
                a.hash(&mut actual);
                BigInt::from(left).hash(&mut reference);
                assert_eq!(actual.finish(), reference.finish());
                let cloned = a.clone();
                assert_eq!(cloned.as_big(), &BigInt::from(left));
                for right in [i64::MIN, -1, 0, 1, i64::MAX] {
                    for b in [Integer::from(right), Integer::from(i128::from(right))] {
                        let (left, right) = (i128::from(left), i128::from(right));
                        for (result, expected) in [
                            (a.add(&b), left.checked_add(right)),
                            (a.sub(&b), left.checked_sub(right)),
                            (a.mul(&b), left.checked_mul(right)),
                            (a.neg(), left.checked_neg()),
                        ] {
                            assert_eq!(result, Integer::from(expected.unwrap()));
                        }
                    }
                }
            }
        }
    }
}

#[test]
#[trace("FR-371-AC-4")]
fn mixed_inline_operations_and_materialized_source_clones_allocate_nothing() {
    for left in [Integer::from(i64::MIN), Integer::from(i128::from(i64::MIN))] {
        for right in [Integer::from(i64::MAX), Integer::from(i128::from(i64::MAX))] {
            let _ = left.as_big();
            let _ = right.as_big();
            let mut observed = None;
            let allocations = allocation_counter::measure(|| {
                observed = Some((
                    left.add(&right),
                    left.sub(&right),
                    left.mul(&right),
                    left.neg(),
                    left.clone(),
                    right.clone(),
                ));
            });
            assert_eq!(allocations.count_total, 0);
            assert_eq!(allocations.bytes_total, 0);
            let (add, sub, mul, neg, left_clone, right_clone) = observed.unwrap();
            let (a, b) = (i128::from(i64::MIN), i128::from(i64::MAX));
            assert_eq!(add, Integer::from(a.checked_add(b).unwrap()));
            assert_eq!(sub, Integer::from(a.checked_sub(b).unwrap()));
            assert_eq!(mul, Integer::from(a.checked_mul(b).unwrap()));
            assert_eq!(neg, Integer::from(a.checked_neg().unwrap()));
            assert_eq!(left_clone, left);
            assert_eq!(right_clone, right);
        }
    }
}

#[test]
#[trace("FR-371-AC-1")]
fn signed_i64_extremes_refine_all_four_widened_operations() {
    let values = [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX];
    for left in values {
        let left_integer = Integer::from(left);
        let left_reference = i128::from(left);
        assert_eq!(
            left_integer.neg(),
            Integer::from(left_reference.checked_neg().unwrap())
        );
        for right in values {
            let right_integer = Integer::from(right);
            let right_reference = i128::from(right);
            for (actual, expected) in [
                (
                    left_integer.add(&right_integer),
                    left_reference.checked_add(right_reference),
                ),
                (
                    left_integer.sub(&right_integer),
                    left_reference.checked_sub(right_reference),
                ),
                (
                    left_integer.mul(&right_integer),
                    left_reference.checked_mul(right_reference),
                ),
            ] {
                assert_eq!(
                    actual,
                    Integer::from(expected.unwrap()),
                    "left={left}, right={right}"
                );
            }
        }
    }
    let min = Integer::from(i64::MIN);
    assert_eq!(
        min.mul(&min).to_string(),
        "85070591730234615865843651857942052864"
    );
    assert_eq!(min.neg().to_string(), "9223372036854775808");
}

#[test]
#[trace("FR-371-AC-1")]
fn failed_width_guards_retain_full_i128_and_arbitrary_precision_results() {
    let values = [
        i128::from(i64::MIN) - 1,
        i128::from(i64::MIN),
        -1,
        0,
        1,
        i128::from(i64::MAX),
        i128::from(i64::MAX) + 1,
        i128::MIN,
        i128::MAX,
    ];
    for left in values {
        let left_integer = Integer::from(left);
        let left_reference = BigInt::from(left);
        assert_eq!(left_integer.neg().as_big(), &(-&left_reference));
        for right in values {
            let right_integer = Integer::from(right);
            let right_reference = BigInt::from(right);
            for (actual, expected) in [
                (
                    left_integer.add(&right_integer),
                    &left_reference + &right_reference,
                ),
                (
                    left_integer.sub(&right_integer),
                    &left_reference - &right_reference,
                ),
                (
                    left_integer.mul(&right_integer),
                    &left_reference * &right_reference,
                ),
            ] {
                assert_eq!(actual.as_big(), &expected, "left={left}, right={right}");
            }
        }
    }
    let large = Integer::from_big(BigInt::from(1_u8) << 200_u32);
    let small = Integer::from(i64::MIN);
    let reference = (BigInt::from(1_u8) << 200_u32) + BigInt::from(i64::MIN);
    assert_eq!(large.add(&small).as_big(), &reference);
    assert_eq!(small.add(&large).as_big(), &reference);
    assert_eq!(large.add(&small).sub(&large), small);
}

#[test]
#[trace("FR-371-AC-4")]
fn widened_i64_extreme_operations_request_no_allocations() {
    for left in [i64::MIN, -1, 0, 1, i64::MAX] {
        let left_integer = Integer::from(left);
        for right in [i64::MIN, -1, 0, 1, i64::MAX] {
            let right_integer = Integer::from(right);
            let mut observed = None;
            let allocations = allocation_counter::measure(|| {
                observed = Some((
                    left_integer.add(&right_integer),
                    left_integer.sub(&right_integer),
                    left_integer.mul(&right_integer),
                    left_integer.neg(),
                ));
            });
            assert_eq!(allocations.count_total, 0, "left={left}, right={right}");
            assert_eq!(allocations.bytes_total, 0, "left={left}, right={right}");
            let (add, sub, mul, neg) = observed.unwrap();
            let (left, right) = (i128::from(left), i128::from(right));
            assert_eq!(add, Integer::from(left.checked_add(right).unwrap()));
            assert_eq!(sub, Integer::from(left.checked_sub(right).unwrap()));
            assert_eq!(mul, Integer::from(left.checked_mul(right).unwrap()));
            assert_eq!(neg, Integer::from(left.checked_neg().unwrap()));
        }
    }
}

#[test]
#[trace("FR-371-AC-1")]
fn canonical_parsing_preserves_native_boundaries_and_arbitrary_precision() {
    for (spelling, expected) in [
        ("-170141183460469231731687303715884105728", i128::MIN),
        ("170141183460469231731687303715884105727", i128::MAX),
        ("-9223372036854775808", i128::from(i64::MIN)),
        ("9223372036854775807", i128::from(i64::MAX)),
        ("-1", -1),
        ("0", 0),
        ("1", 1),
    ] {
        let parsed = Integer::from_str(spelling).unwrap();
        assert_eq!(parsed, Integer::from(expected));
        assert_eq!(parsed.as_big(), &BigInt::from(expected));
        assert_eq!(parsed.to_string(), spelling);
    }

    let positive_overflow = Integer::from_str("170141183460469231731687303715884105728").unwrap();
    let negative_overflow = Integer::from_str("-170141183460469231731687303715884105729").unwrap();
    assert_eq!(
        positive_overflow.sub(&Integer::one()),
        Integer::from(i128::MAX)
    );
    assert_eq!(
        negative_overflow.add(&Integer::one()),
        Integer::from(i128::MIN)
    );

    for (spelling, expected) in [
        (
            "170141183460469231731687303715884105728",
            BigInt::from(1_u8) << 127_u32,
        ),
        (
            "-170141183460469231731687303715884105729",
            -(BigInt::from(1_u8) << 127_u32) - 1_u8,
        ),
        (
            "1267650600228229401496703205376",
            BigInt::from(1_u8) << 100_u32,
        ),
        (
            "1606938044258990275541962092341162602522202993782792835301376",
            BigInt::from(1_u8) << 200_u32,
        ),
        (
            "-1606938044258990275541962092341162602522202993782792835301376",
            -(BigInt::from(1_u8) << 200_u32),
        ),
    ] {
        let parsed = Integer::from_str(spelling).unwrap();
        assert_eq!(parsed.as_big(), &expected);
        assert_eq!(parsed.to_string(), spelling);
    }

    for spelling in [
        "",
        "-",
        "+1",
        "+0",
        "00",
        "01",
        "-0",
        "-01",
        " 1",
        "1 ",
        "1\n",
        "1_000",
        "1.0",
        "1e3",
        "--1",
        "١",
        "１",
        "−1",
        "+170141183460469231731687303715884105728",
        "01606938044258990275541962092341162602522202993782792835301376",
    ] {
        assert_eq!(
            Integer::from_str(spelling),
            Err(quire_exact::NonCanonicalInteger),
            "{spelling:?}"
        );
    }
}

// Resource diagnostic for canonical construction, beyond AC-1's value contract;
// AC-4's existing allocation promise covers primitive/arithmetic paths.
#[test]
#[trace("FR-371-AC-1")]
fn fitting_canonical_parsing_requests_no_allocations() {
    for (spelling, expected) in [
        ("0", 0),
        ("-1", -1),
        ("1", 1),
        ("-9223372036854775808", i128::from(i64::MIN)),
        ("9223372036854775807", i128::from(i64::MAX)),
        ("-170141183460469231731687303715884105728", i128::MIN),
        ("170141183460469231731687303715884105727", i128::MAX),
    ] {
        let mut parsed = None;
        let allocations = allocation_counter::measure(|| {
            parsed = Some(Integer::from_str(spelling));
        });
        assert_eq!(allocations.count_total, 0, "{spelling}");
        assert_eq!(allocations.bytes_total, 0, "{spelling}");
        assert_eq!(parsed.unwrap().unwrap(), Integer::from(expected));
    }
}

#[test]
#[trace("FR-371-AC-2")]
fn parsed_values_keep_observations_and_borrows_across_materialization_and_clone() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    for value in [i128::MIN, -1, 0, 1, i128::MAX] {
        let spelling = value.to_string();
        let parsed = Integer::from_str(&spelling).unwrap();
        let routes = [
            Integer::from(value),
            Integer::from_big(BigInt::from(value)),
            Integer::from(value).add(&Integer::zero()),
        ];
        let before_clone = parsed.clone();
        // Display and native equality/order do not require BigInt observation.
        assert_eq!(parsed.to_string(), spelling);
        for other in &routes {
            assert_eq!(&parsed, other);
            assert_eq!(parsed.cmp(other), core::cmp::Ordering::Equal);
        }
        let borrowed = parsed.as_big();
        assert_eq!(borrowed, &BigInt::from(value));
        assert!(core::ptr::eq(borrowed, parsed.as_big()));
        let after_clone = parsed.clone();
        for other in routes.iter().chain([&before_clone, &after_clone]) {
            let mut parsed_hash = DefaultHasher::new();
            let mut other_hash = DefaultHasher::new();
            parsed.hash(&mut parsed_hash);
            other.hash(&mut other_hash);
            assert_eq!(parsed_hash.finish(), other_hash.finish());
            assert_eq!(format!("{parsed:?}"), format!("{other:?}"));
            assert_eq!(other.to_string(), spelling);
            assert_eq!(other.as_big(), borrowed);
        }
        drop(after_clone);
        drop(before_clone);
        assert_eq!(borrowed, &BigInt::from(value));
        assert!(core::ptr::eq(borrowed, parsed.as_big()));
    }
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

/// Trace: FR-371-AC-3
#[test]
fn metered_operation_matrix_preserves_fr362_amounts_work_and_denial_prefixes() {
    let i64_min = Integer::from(i64::MIN);
    let i64_max = Integer::from(i64::MAX);
    let wide_i64_min = Integer::from(i128::from(i64::MIN));
    let wide_i64_max = Integer::from(i128::from(i64::MAX));
    let small_pos = Integer::from(8_i128);
    let small_pos_rhs = Integer::from(3_i128);
    let small_sub_rhs = Integer::from(7_i128);
    let small_neg = Integer::from(-8_i128);
    let native_max = Integer::from(i128::MAX);
    let native_min = Integer::from(i128::MIN);
    let one = Integer::one();
    let two = Integer::from(2_i128);
    let huge = Integer::from_big(BigInt::from(1_u8) << 200_u32);
    let huge_plus_123 = huge.add(&Integer::from(123_i64));
    let huge_plus_42 = huge.add(&Integer::from(42_i64));
    let huge_negative = huge.neg();

    struct Case<'a> {
        operation: IntegerArithmetic<'a>,
        expected: &'static str,
        operand_bits: u64,
        arithmetic_bits: u64,
        occurrences: u64,
    }
    // FR-362 independently gives binary operands max(B(a), B(c)), add/sub
    // max(B(a), B(c)) + 1, multiply B(a) + B(c), and negate B(a).
    let cases = [
        Case {
            operation: IntegerArithmetic::Add(&i64_min, &wide_i64_min),
            expected: "-18446744073709551616",
            operand_bits: 64,
            arithmetic_bits: 65,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Subtract(&wide_i64_max, &i64_min),
            expected: "18446744073709551615",
            operand_bits: 64,
            arithmetic_bits: 65,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Multiply(&wide_i64_min, &wide_i64_min),
            expected: "85070591730234615865843651857942052864",
            operand_bits: 64,
            arithmetic_bits: 128,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Negate(&wide_i64_min),
            expected: "9223372036854775808",
            operand_bits: 64,
            arithmetic_bits: 64,
            occurrences: 1,
        },
        Case {
            operation: IntegerArithmetic::Add(&i64_min, &i64_min),
            expected: "-18446744073709551616",
            operand_bits: 64,
            arithmetic_bits: 65,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Subtract(&i64_max, &i64_min),
            expected: "18446744073709551615",
            operand_bits: 64,
            arithmetic_bits: 65,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Multiply(&i64_min, &i64_min),
            expected: "85070591730234615865843651857942052864",
            operand_bits: 64,
            arithmetic_bits: 128,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Negate(&i64_min),
            expected: "9223372036854775808",
            operand_bits: 64,
            arithmetic_bits: 64,
            occurrences: 1,
        },
        Case {
            operation: IntegerArithmetic::Add(&small_pos, &small_pos_rhs),
            expected: "11",
            operand_bits: 4,
            arithmetic_bits: 5,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Add(&native_max, &one),
            expected: "170141183460469231731687303715884105728",
            operand_bits: 127,
            arithmetic_bits: 128,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Add(&huge, &one),
            expected: "1606938044258990275541962092341162602522202993782792835301377",
            operand_bits: 201,
            arithmetic_bits: 202,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Subtract(&small_pos, &small_sub_rhs),
            expected: "1",
            operand_bits: 4,
            arithmetic_bits: 5,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Subtract(&native_min, &one),
            expected: "-170141183460469231731687303715884105729",
            operand_bits: 128,
            arithmetic_bits: 129,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Subtract(&huge_plus_123, &huge_plus_42),
            expected: "81",
            operand_bits: 201,
            arithmetic_bits: 202,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Multiply(&small_pos, &small_pos_rhs),
            expected: "24",
            operand_bits: 4,
            arithmetic_bits: 6,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Multiply(&native_max, &two),
            expected: "340282366920938463463374607431768211454",
            operand_bits: 127,
            arithmetic_bits: 129,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Multiply(&huge, &one),
            expected: "1606938044258990275541962092341162602522202993782792835301376",
            operand_bits: 201,
            arithmetic_bits: 202,
            occurrences: 2,
        },
        Case {
            operation: IntegerArithmetic::Negate(&small_neg),
            expected: "8",
            operand_bits: 4,
            arithmetic_bits: 4,
            occurrences: 1,
        },
        Case {
            operation: IntegerArithmetic::Negate(&native_min),
            expected: "170141183460469231731687303715884105728",
            operand_bits: 128,
            arithmetic_bits: 128,
            occurrences: 1,
        },
        Case {
            operation: IntegerArithmetic::Negate(&huge_negative),
            expected: "1606938044258990275541962092341162602522202993782792835301376",
            operand_bits: 201,
            arithmetic_bits: 201,
            occurrences: 1,
        },
    ];
    let points = [
        ChargePoint::IntegerArithmeticOperands,
        ChargePoint::IntegerArithmeticArithmetic,
        ChargePoint::IntegerArithmeticResultRetain,
    ];

    for case in cases {
        let mut exact_bits = limits();
        exact_bits.integer_bits = case.arithmetic_bits;
        exact_bits.work_units = 3;
        let mut meter = Meter::new(exact_bits);
        let outcome = evaluate_integer_arithmetic(case.operation, None, &mut meter);
        assert_eq!(
            outcome
                .completed()
                .map(|value| value.to_string())
                .as_deref(),
            Some(case.expected)
        );
        assert_eq!(meter.consumed(LimitKind::IntegerBits), case.arithmetic_bits);
        assert_eq!(
            meter.consumed(LimitKind::ValueOccurrences),
            case.occurrences
        );
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
        assert_metered_counters(&meter, case.arithmetic_bits, case.occurrences, 3, 1);
        assert_points(&meter, &points);

        let mut insufficient_bits = limits();
        insufficient_bits.integer_bits = case.arithmetic_bits - 1;
        let mut meter = Meter::new(insufficient_bits);
        let outcome = evaluate_integer_arithmetic(case.operation, None, &mut meter);
        let Outcome::Incomplete(incomplete) = outcome else {
            panic!("expected integer bit refusal, got {outcome:?}");
        };
        let operand_would_refuse = case.operand_bits > case.arithmetic_bits - 1;
        assert_eq!(
            incomplete.charge_point,
            if operand_would_refuse {
                points[0]
            } else {
                points[1]
            }
        );
        assert_eq!(incomplete.limit_kind, LimitKind::IntegerBits);
        assert_eq!(incomplete.limit, case.arithmetic_bits - 1);
        assert_eq!(
            incomplete.consumed,
            if operand_would_refuse {
                0
            } else {
                case.operand_bits
            }
        );
        assert_eq!(
            incomplete.next_charge,
            Integer::from(if operand_would_refuse {
                case.operand_bits
            } else {
                case.arithmetic_bits
            })
        );
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        let prior_work = if operand_would_refuse { 0 } else { 1 };
        assert_eq!(meter.consumed(LimitKind::WorkUnits), prior_work);
        assert_metered_counters(
            &meter,
            if operand_would_refuse {
                0
            } else {
                case.operand_bits
            },
            if operand_would_refuse {
                0
            } else {
                case.occurrences
            },
            prior_work,
            0,
        );
        assert_points(
            &meter,
            if operand_would_refuse {
                &[] as &[ChargePoint]
            } else {
                &points[..1]
            },
        );

        let mut insufficient_work = limits();
        insufficient_work.work_units = 2;
        let mut meter = Meter::new(insufficient_work);
        let outcome = evaluate_integer_arithmetic(case.operation, None, &mut meter);
        let Outcome::Incomplete(incomplete) = outcome else {
            panic!("expected work refusal, got {outcome:?}");
        };
        assert_eq!(incomplete.charge_point, points[2]);
        assert_eq!(incomplete.limit_kind, LimitKind::WorkUnits);
        assert_eq!(incomplete.limit, 2);
        assert_eq!(incomplete.consumed, 2);
        assert_eq!(incomplete.next_charge, Integer::one());
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 2);
        assert_metered_counters(&meter, case.arithmetic_bits, case.occurrences, 2, 0);
        assert_points(&meter, &points[..2]);

        for (index, denied_point) in points.into_iter().enumerate() {
            let mut meter = Meter::new(limits()).with_injected_denial(InjectedDenial {
                point: denied_point,
                occurrence: NonZeroU64::new(1).unwrap(),
            });
            let outcome = evaluate_integer_arithmetic(case.operation, None, &mut meter);
            let Outcome::Incomplete(incomplete) = outcome else {
                panic!("expected injected denial at {denied_point:?}, got {outcome:?}");
            };
            assert_eq!(incomplete.charge_point, denied_point);
            assert_eq!(incomplete.limit_kind, LimitKind::WorkUnits);
            assert_eq!(incomplete.limit, index as u64);
            assert_eq!(incomplete.consumed, index as u64);
            assert_eq!(incomplete.next_charge, Integer::one());
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), index as u64);
            let admitted_bits = match index {
                0 => 0,
                1 => case.operand_bits,
                _ => case.arithmetic_bits,
            };
            assert_metered_counters(
                &meter,
                admitted_bits,
                if index == 0 { 0 } else { case.occurrences },
                index as u64,
                0,
            );
            assert_points(&meter, &points[..index]);
        }
    }
}

fn assert_metered_counters(
    meter: &Meter,
    integer_bits: u64,
    occurrences: u64,
    work_units: u64,
    result_units: u64,
) {
    for kind in LimitKind::ALL {
        let expected = match kind {
            LimitKind::IntegerBits => integer_bits,
            LimitKind::ValueOccurrences => occurrences,
            LimitKind::WorkUnits => work_units,
            LimitKind::ResultUnits => result_units,
            _ => 0,
        };
        assert_eq!(meter.consumed(kind), expected, "{kind:?}");
    }
}

#[cfg(feature = "test-support")]
fn assert_points(meter: &Meter, expected: &[ChargePoint]) {
    assert_eq!(
        meter
            .admitted_charges()
            .iter()
            .map(|charge| charge.point)
            .collect::<Vec<_>>(),
        expected
    );
}

#[cfg(not(feature = "test-support"))]
fn assert_points(_meter: &Meter, _expected: &[ChargePoint]) {}

#[trace("FR-371-AC-4")]
#[cfg(not(feature = "test-support"))]
#[test]
fn native_public_metered_call_matches_accounting_only_allocation_observation() {
    let left = Integer::from(8_i128);
    let right = Integer::from(3_i128);
    let mut public_meter = Meter::new(limits());
    let mut public_outcome = None;
    let public_allocations = allocation_counter::measure(|| {
        public_outcome = Some(evaluate_integer_arithmetic(
            IntegerArithmetic::Add(&left, &right),
            None,
            &mut public_meter,
        ));
    });
    assert_eq!(
        public_outcome.unwrap().completed(),
        Some(Integer::from(11_i128))
    );

    let mut baseline_meter = Meter::new(limits());
    let baseline_allocations = allocation_counter::measure(|| {
        baseline_meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticOperands)
                    .size(LimitKind::IntegerBits, 4)
                    .size(LimitKind::ValueOccurrences, 2),
            )
            .unwrap();
        baseline_meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticArithmetic)
                    .size(LimitKind::IntegerBits, 5),
            )
            .unwrap();
        baseline_meter
            .charge(Charge::new(ChargePoint::IntegerArithmeticResultRetain).results(1))
            .unwrap();
    });

    assert_eq!(
        baseline_allocations.bytes_total, 0,
        "the two-request arithmetic charge schedule stays inline"
    );
    assert_eq!(
        public_allocations.bytes_total,
        baseline_allocations.bytes_total
    );

    let mut refusal_limits = limits();
    refusal_limits.work_units = 2;
    let mut refused_meter = Meter::new(refusal_limits);
    let mut refused_outcome = None;
    let refused_allocations = allocation_counter::measure(|| {
        refused_outcome = Some(evaluate_integer_arithmetic(
            IntegerArithmetic::Add(&left, &right),
            None,
            &mut refused_meter,
        ));
    });
    assert!(matches!(refused_outcome, Some(Outcome::Incomplete(_))));

    let mut refusal_baseline_limits = limits();
    refusal_baseline_limits.work_units = 2;
    let mut refusal_baseline_meter = Meter::new(refusal_baseline_limits);
    let refusal_baseline_allocations = allocation_counter::measure(|| {
        refusal_baseline_meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticOperands)
                    .size(LimitKind::IntegerBits, 4)
                    .size(LimitKind::ValueOccurrences, 2),
            )
            .unwrap();
        refusal_baseline_meter
            .charge(
                Charge::new(ChargePoint::IntegerArithmeticArithmetic)
                    .size(LimitKind::IntegerBits, 5),
            )
            .unwrap();
        assert_eq!(
            refusal_baseline_meter
                .charge(Charge::new(ChargePoint::IntegerArithmeticResultRetain).results(1))
                .unwrap_err()
                .charge_point,
            ChargePoint::IntegerArithmeticResultRetain
        );
    });
    assert_eq!(
        refused_allocations.bytes_total,
        refusal_baseline_allocations.bytes_total
    );
}

#[trace("FR-362-AC-4", "FR-362-AC-5", "FR-362-AC-6", "FR-362-AC-8")]
#[test]
fn integer_ordering_preserves_values_sizes_and_denied_prefixes() {
    let values = [
        Integer::from(i64::MIN),
        Integer::from(-1_i64),
        Integer::zero(),
        Integer::from(1_i64),
        Integer::from(i64::MAX),
        Integer::from(i128::MIN),
        Integer::from(-1_i128),
        Integer::from(1_i128),
        Integer::from(i128::MAX),
        Integer::from_str("-340282366920938463463374607431768211456").unwrap(),
        Integer::from_str("340282366920938463463374607431768211456").unwrap(),
    ];
    let points = [
        ChargePoint::OrderingOperands,
        ChargePoint::OrderingArithmetic,
        ChargePoint::OrderingResultRetain,
    ];
    for left in &values {
        for right in &values {
            // The maintained arbitrary-precision implementation supplies both
            // the numeric oracle and an independent magnitude measurement.
            let a = BigInt::from_str(&left.to_string()).unwrap();
            let b = BigInt::from_str(&right.to_string()).unwrap();
            let bits = a.bits().max(b.bits()).max(1);
            for (operator, expected) in [
                (OrderingOperator::Less, a < b),
                (OrderingOperator::LessOrEqual, a <= b),
                (OrderingOperator::Greater, a > b),
                (OrderingOperator::GreaterOrEqual, a >= b),
            ] {
                let mut exact = limits();
                exact.integer_bits = bits;
                exact.work_units = 3;
                let mut meter = Meter::new(exact);
                assert_eq!(
                    order_numbers(operator, OrderedOperands::Integers(left, right), &mut meter),
                    Outcome::Completed(expected)
                );
                assert_metered_counters(&meter, bits, 2, 3, 1);
                assert_points(&meter, &points);
                #[cfg(feature = "test-support")]
                {
                    let charges = meter.admitted_charges();
                    assert_eq!(charges[0].integer_bits, Some(bits));
                    assert_eq!(charges[1].integer_bits, Some(bits));
                    assert_eq!(charges[2].integer_bits, None);
                    assert_eq!(charges[0].value_occurrences, Some(2));
                    assert_eq!(charges[1].value_occurrences, None);
                }
                let mut short = limits();
                short.integer_bits = bits - 1;
                let mut meter = Meter::new(short);
                let outcome =
                    order_numbers(operator, OrderedOperands::Integers(left, right), &mut meter);
                let Outcome::Incomplete(denial) = outcome else {
                    panic!("expected bit denial: {outcome:?}");
                };
                assert_eq!(denial.charge_point, points[0]);
                assert_eq!(denial.limit_kind, LimitKind::IntegerBits);
                assert_eq!(denial.limit, bits - 1);
                assert_eq!(denial.consumed, 0);
                assert_eq!(denial.next_charge, Integer::from(bits));
                assert_metered_counters(&meter, 0, 0, 0, 0);
                assert_points(&meter, &[]);
                for (index, point) in points.into_iter().enumerate() {
                    let mut meter = Meter::new(limits()).with_injected_denial(InjectedDenial {
                        point,
                        occurrence: NonZeroU64::new(1).unwrap(),
                    });
                    let outcome =
                        order_numbers(operator, OrderedOperands::Integers(left, right), &mut meter);
                    let Outcome::Incomplete(denial) = outcome else {
                        panic!("expected injected denial: {outcome:?}");
                    };
                    assert_eq!(denial.charge_point, point);
                    assert_eq!(denial.limit_kind, LimitKind::WorkUnits);
                    assert_eq!(denial.limit, index as u64);
                    assert_eq!(denial.consumed, index as u64);
                    assert_eq!(denial.next_charge, Integer::one());
                    assert_metered_counters(
                        &meter,
                        if index == 0 { 0 } else { bits },
                        if index == 0 { 0 } else { 2 },
                        index as u64,
                        0,
                    );
                    assert_points(&meter, &points[..index]);
                    // The injected stop is one-shot. Reuse the actual meter:
                    // the next comparison retains its independent full schedule.
                    assert_eq!(
                        order_numbers(operator, OrderedOperands::Integers(left, right), &mut meter),
                        Outcome::Completed(expected)
                    );
                    assert_metered_counters(&meter, bits, 2, index as u64 + 3, 1);
                    let expected_points = points[..index]
                        .iter()
                        .chain(points.iter())
                        .copied()
                        .collect::<Vec<_>>();
                    assert_points(&meter, &expected_points);
                }
            }
        }
    }
}
