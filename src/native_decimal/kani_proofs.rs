// SPDX-License-Identifier: AGPL-3.0-or-later
//! Universal obligations for the native Decimal carrier (FR-402-AC-10). The
//! proofs carry no unwind attribute: the constructor and the normalized view
//! contain no loop. Not executed under the repository's standing instruction.
use super::operations::{
    order_values, power_of_ten_bits, LOG2_10_FRACTION, LOG2_10_LOWER, LOG2_10_UPPER,
};
use super::reference::{
    decide, is_least_scale_pair, is_member, native_decision, order_ref, ref_op, ref_type, Decision,
    TEN_POWERS,
};
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
                    "oversize power decided by sign"
                );
                kani::cover!(
                    c2 != 0
                        && lifted <= max_scale
                        && shift <= 38
                        && c2.unsigned_abs() > (u128::MAX >> 1) / TEN_POWERS[shift as usize],
                    "overflowing lift decided by sign"
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

// ---------------------------------------------------------------------------
// FR-403 Behavior 37: one proof per operation and one for the ordering, over
// all symbolic carried operands and all symbolic native types, with no unwind
// attribute (the proof-path bodies contain no loop). Each equates the
// admission decision, the outcome, the completed value and the loss record to
// the independent reference of `reference.rs`, which shares no code with the
// native path, and covers every reachable class. Not executed under the
// repository's standing instruction.
// ---------------------------------------------------------------------------

fn any_type() -> NativeDecimalType {
    let lower: i128 = kani::any();
    let upper: i128 = kani::any();
    let min_scale: u32 = kani::any();
    let max_scale: u32 = kani::any();
    kani::assume(lower <= upper);
    kani::assume(min_scale <= max_scale);
    let mode = match kani::any::<u8>() % 6 {
        0 => RoundingMode::Exact,
        1 => RoundingMode::TowardZero,
        2 => RoundingMode::TowardPositive,
        3 => RoundingMode::TowardNegative,
        4 => RoundingMode::NearestEven,
        _ => RoundingMode::NearestAway,
    };
    NativeDecimalType::new(
        lower,
        upper,
        u64::from(min_scale),
        u64::from(max_scale),
        mode,
    )
    .unwrap()
}

fn any_decimal() -> NativeDecimal {
    NativeDecimal::from_retained(kani::any(), kani::any())
}

/// The native decision equals the reference decision, with the covers.
fn refines(ty: &NativeDecimalType, op: NativeDecimalOperation) {
    let native = native_decision(ty, op);
    assert_eq!(native, decide(ref_type(ty), ref_op(op)));
    kani::cover!(
        matches!(native, Decision::Completed { loss: None, .. }),
        "completed path"
    );
    kani::cover!(
        matches!(native, Decision::Completed { loss: Some(_), .. }),
        "rounding step"
    );
    kani::cover!(matches!(native, Decision::Inexact), "exact refusal");
    kani::cover!(matches!(native, Decision::OutOfDomain), "domain refusal");
    kani::cover!(matches!(native, Decision::DivisionByZero), "zero divisor");
    kani::cover!(matches!(native, Decision::Capacity(_)), "capacity refusal");
}

#[kani::proof]
fn native_decimal_add_refines_owner() {
    let ty = any_type();
    refines(
        &ty,
        NativeDecimalOperation::Add(any_decimal(), any_decimal()),
    );
}

#[kani::proof]
fn native_decimal_subtract_refines_owner() {
    let ty = any_type();
    refines(
        &ty,
        NativeDecimalOperation::Subtract(any_decimal(), any_decimal()),
    );
}

#[kani::proof]
fn native_decimal_multiply_refines_owner() {
    let ty = any_type();
    refines(
        &ty,
        NativeDecimalOperation::Multiply(any_decimal(), any_decimal()),
    );
}

/// The gcd bound of Behavior 16 is covered here: the reference reduces by
/// Euclid's algorithm, so a native sequence that stops early disagrees.
#[kani::proof]
fn native_decimal_divide_refines_owner() {
    let ty = any_type();
    refines(
        &ty,
        NativeDecimalOperation::Divide(any_decimal(), any_decimal()),
    );
}

#[kani::proof]
fn native_decimal_negate_refines_owner() {
    let ty = any_type();
    refines(&ty, NativeDecimalOperation::Negate(any_decimal()));
}

#[kani::proof]
fn native_decimal_round_refines_owner() {
    let ty = any_type();
    refines(&ty, NativeDecimalOperation::Round(any_decimal()));
}

#[kani::proof]
fn native_decimal_ordering_refines_owner() {
    let (a, b) = (any_decimal(), any_decimal());
    let native = order_values(a, b);
    let reference = order_ref((a.coefficient(), a.scale()), (b.coefficient(), b.scale()));
    assert_eq!(native, reference);
    kani::cover!(native.is_lt(), "less");
    kani::cover!(native.is_eq(), "equal");
    kani::cover!(native.is_gt(), "greater");
    kani::cover!(
        a.scale().abs_diff(b.scale()) > 38 && a.coefficient() != 0 && b.coefficient() != 0,
        "shift beyond 38"
    );
}

/// FR-403-AC-9: the lower and upper approximations of `log2(10)` give one
/// floor for every `u32` shift, and the product fits 128 bits.
#[kani::proof]
fn native_decimal_log2_ten_approximations_agree() {
    let shift: u32 = kani::any();
    let k = u128::from(shift);
    let lower = (k * LOG2_10_LOWER) >> LOG2_10_FRACTION;
    let upper = (k * LOG2_10_UPPER) >> LOG2_10_FRACTION;
    assert_eq!(lower, upper);
    assert_eq!(u128::from(power_of_ten_bits(shift)), lower + 1);
}
