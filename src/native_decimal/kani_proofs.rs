// SPDX-License-Identifier: AGPL-3.0-or-later
//! Universal obligations for the native Decimal carrier (FR-402-AC-10). The
//! proofs carry no unwind attribute: the constructor and the normalized view
//! contain no loop. Not executed under the repository's standing instruction.
use super::reference::{is_least_scale_pair, is_member};
use super::*;

#[kani::proof]
fn native_decimal_from_bindings_refines_owner_membership() {
    let lower: i128 = kani::any();
    let upper: i128 = kani::any();
    let min_scale: u32 = kani::any();
    let max_scale: u32 = kani::any();
    kani::assume(lower <= upper);
    kani::assume(min_scale <= max_scale);
    let ty = NativeDecimalType::new(
        lower,
        upper,
        u64::from(min_scale),
        u64::from(max_scale),
        RoundingMode::Exact,
    )
    .unwrap();
    let coefficient: i64 = kani::any();
    let scale: i64 = kani::any();
    let outcome = NativeDecimal::from_bindings(&ty, coefficient, scale);
    match u32::try_from(scale) {
        Err(_) => {
            kani::cover!(true, "out-of-scale-range path");
            assert_eq!(outcome, Err(NativeDecimalRefusal::OutsideCarrierDomain));
        }
        Ok(scale) => {
            // The least-scale pair is unique, so choosing it symbolically and
            // assuming its postcondition fixes it without running the view.
            let c2: i128 = kani::any();
            let s2: u32 = kani::any();
            kani::assume(is_least_scale_pair(i128::from(coefficient), scale, c2, s2));
            let member = is_member(lower, upper, min_scale, max_scale, c2, s2);
            let lifted = s2.max(min_scale);
            let shift = lifted - s2.min(lifted);
            if member {
                kani::cover!(true, "member path");
                kani::cover!(
                    c2 != 0 && lifted > s2 && shift <= 38,
                    "in-range nonzero lift"
                );
                assert_eq!(
                    outcome,
                    Ok(NativeDecimal::from_retained(i128::from(coefficient), scale))
                );
            } else {
                kani::cover!(lifted <= max_scale, "non-member in range");
                kani::cover!(
                    c2 != 0 && lifted <= max_scale && shift > 38,
                    "oversize lift decided by sign"
                );
                assert_eq!(outcome, Err(NativeDecimalRefusal::OutsideCarrierDomain));
            }
        }
    }
}

#[kani::proof]
fn native_decimal_normalized_is_least_scale_pair() {
    let coefficient: i128 = kani::any();
    let scale: u32 = kani::any();
    let (c2, s2) = NativeDecimal::from_retained(coefficient, scale).normalized();
    assert!(is_least_scale_pair(coefficient, scale, c2, s2));
    kani::cover!(s2 < scale, "zeros removed");
    kani::cover!(s2 == scale && coefficient != 0, "nothing removed");
    kani::cover!(coefficient == 0 && scale > 0, "zero collapses");
}
