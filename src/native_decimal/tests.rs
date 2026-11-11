// SPDX-License-Identifier: AGPL-3.0-or-later
//! Host binding of the independent references to the owner route (FR-402
//! Behavior 9, TC-950 step 5).
use ix_trace_rs::trace;

use super::reference::{is_least_scale_pair, is_member};
use super::*;

const COEFFICIENTS: [i128; 17] = [
    0,
    1,
    -1,
    2,
    -2,
    9,
    10,
    -10,
    100,
    -1000,
    5000,
    9999,
    1_000_000_000_000_000_000,
    10_i128.pow(37),
    -(10_i128.pow(38)),
    i128::MAX,
    i128::MIN,
];
const SCALES: [u32; 11] = [0, 1, 2, 3, 5, 18, 19, 20, 38, 40, u32::MAX];
const BOUNDS: [(i128, i128); 7] = [
    (0, 100),
    (-1000, 1000),
    (0, 1),
    (i128::MIN, 0),
    (0, i128::MAX),
    (i128::MIN, i128::MAX),
    (0, 0),
];
const SCALE_RANGES: [(u32, u32); 8] = [
    (0, 0),
    (0, 2),
    (2, 3),
    (38, 38),
    (39, 39),
    (60, 70),
    (128, 130),
    (0, u32::MAX),
];

#[test]
#[trace("TC-950", "FR-402-AC-8", "FR-402-AC-10")]
fn references_agree_with_the_owner_route_over_the_grid() {
    for (lower, upper) in BOUNDS {
        for (min_scale, max_scale) in SCALE_RANGES {
            let owner = DecimalType::new(
                Integer::from(lower),
                Integer::from(upper),
                u64::from(min_scale),
                u64::from(max_scale),
                RoundingMode::Exact,
            )
            .unwrap();
            for coefficient in COEFFICIENTS {
                for scale in SCALES {
                    let decimal = Decimal::new(Integer::from(coefficient), scale);
                    let normalized = decimal.normalized();
                    let c2 = normalized.coefficient().native().unwrap();
                    let s2 = normalized.scale();
                    let native = NativeDecimal::from_retained(coefficient, scale);
                    assert_eq!(native.normalized(), (c2, s2));
                    assert!(is_least_scale_pair(coefficient, scale, c2, s2));
                    assert_eq!(
                        is_member(lower, upper, min_scale, max_scale, c2, s2),
                        owner.contains(&decimal),
                        "{lower} {upper} {min_scale} {max_scale} {coefficient} {scale}"
                    );
                    if let Ok(c) = i64::try_from(coefficient) {
                        let ty = NativeDecimalType::try_from(&owner).unwrap();
                        assert_eq!(
                            NativeDecimal::from_bindings(&ty, c, i64::from(scale)).is_ok(),
                            owner.contains(&decimal)
                        );
                    }
                }
            }
        }
    }
}

#[test]
#[trace("TC-950", "FR-402-AC-10")]
fn least_scale_reference_rejects_wrong_pairs() {
    assert!(is_least_scale_pair(1000, 3, 1, 0));
    assert!(!is_least_scale_pair(1000, 3, 10, 1));
    assert!(!is_least_scale_pair(1000, 3, 1000, 3));
    assert!(!is_least_scale_pair(0, 7, 0, 7));
    assert!(!is_least_scale_pair(-1000, 3, 1, 0));
    assert!(!is_least_scale_pair(1, 0, 1, 1));
}

#[test]
#[trace("TC-950", "FR-402-AC-2")]
fn membership_is_judged_on_the_normalized_pair_even_for_a_retained_pair() {
    // (100, 2) is the value 1: the owner admits it in Decimal[0,100; 0,1]
    // although its retained scale 2 exceeds smax. A rule applied to the
    // retained pair would refuse it.
    let ty = NativeDecimalType::new(0, 100, 0, 1, RoundingMode::Exact).unwrap();
    let retained = NativeDecimal::from_retained(100, 2);
    let owner = DecimalType::new(
        Integer::zero(),
        Integer::from(100_i64),
        0,
        1,
        RoundingMode::Exact,
    )
    .unwrap();
    assert!(owner.contains(&retained.to_decimal()));
    assert!(ty.admits(&retained));
    assert!(NativeDecimal::from_bindings(&ty, 100, 2).is_ok());
    assert!(!ty.admits(&NativeDecimal::from_retained(1, 2)));
}
