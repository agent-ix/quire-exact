// SPDX-License-Identifier: AGPL-3.0-or-later
//! The proofs of FR-398 Behavior 9, one per carrier, with no unwind attribute.
//! Each draws the bindings and the native-width type bounds symbolically and
//! asserts that the shipped constructor equals the independent reference of
//! `reference.rs`, with a cover that the success path is reachable. The
//! `TypeNotRepresentable` and `CarrierMismatch` refusals need an owner type
//! with an oversize bound or another carrier; they are decided by the host
//! tests, not by these proofs.
use super::{
    construct, reference, NativeQuantity, NativeQuantityShape as Shape, NativeRange,
    QuantityBindings,
};
use crate::decimal::RoundingMode;
use crate::native_decimal::NativeDecimalType;

fn as_pair(outcome: Result<NativeQuantity, super::NativeQuantityRefusal>) -> reference::Outcome {
    outcome.map(|quantity| (quantity.numerator(), quantity.denominator()))
}

#[kani::proof]
fn native_quantity_int_matches_reference() {
    let lower: i128 = kani::any();
    let upper: i128 = kani::any();
    kani::assume(lower <= upper);
    let value: i64 = kani::any();
    let shape = Shape::Int(NativeRange { lower, upper });
    let actual = as_pair(construct(&shape, QuantityBindings::Int(value)));
    kani::cover!(actual.is_ok(), "success witness");
    assert_eq!(actual, reference::int(lower, upper, value));
}

#[kani::proof]
fn native_quantity_decimal_matches_reference() {
    let lower: i128 = kani::any();
    let upper: i128 = kani::any();
    let min_scale: u32 = kani::any();
    let max_scale: u32 = kani::any();
    kani::assume(lower <= upper && min_scale <= max_scale);
    let coefficient: i64 = kani::any();
    let scale: i64 = kani::any();
    let native = NativeDecimalType::new(
        lower,
        upper,
        u64::from(min_scale),
        u64::from(max_scale),
        RoundingMode::Exact,
    )
    .unwrap();
    let shape = Shape::Decimal(native);
    let actual = as_pair(construct(
        &shape,
        QuantityBindings::Decimal { coefficient, scale },
    ));
    kani::cover!(actual.is_ok(), "success witness");
    assert_eq!(
        actual,
        reference::decimal((lower, upper), (min_scale, max_scale), coefficient, scale)
    );
}

#[kani::proof]
fn native_quantity_rational_matches_reference() {
    let n_lower: i128 = kani::any();
    let n_upper: i128 = kani::any();
    let d_lower: i128 = kani::any();
    let d_upper: i128 = kani::any();
    kani::assume(n_lower <= n_upper && 1 <= d_lower && d_lower <= d_upper);
    let numerator: i64 = kani::any();
    let denominator: i64 = kani::any();
    let shape = Shape::Rational {
        numerator: NativeRange {
            lower: n_lower,
            upper: n_upper,
        },
        denominator: NativeRange {
            lower: d_lower,
            upper: d_upper,
        },
    };
    let actual = as_pair(construct(
        &shape,
        QuantityBindings::Rational {
            numerator,
            denominator,
        },
    ));
    kani::cover!(actual.is_ok(), "success witness");
    assert_eq!(
        actual,
        reference::rational(
            (n_lower, n_upper),
            (d_lower, d_upper),
            numerator,
            denominator
        )
    );
}
