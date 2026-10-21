// SPDX-License-Identifier: AGPL-3.0-or-later
//! Opt-in compositional refinement. The primitive contract must be verified
//! before the separately named parent may rely on its verified summary.
use super::*;

#[kani::proof_for_contract(crate::native_integer::checked_product)]
fn checked_product_refines_mathematical_integer() {
    let left: i128 = kani::any();
    let right: i128 = kani::any();
    let result = checked_product(left, right);
    kani::cover!(result.is_some(), "representable product witness");
    kani::cover!(result.is_none(), "overflow product witness");
}

#[kani::proof]
#[kani::stub_verified(crate::native_integer::checked_product)]
fn native_multiply_compositional_refines_mathematical_integer() {
    refinement(NativeIntegerOperationKind::Multiply);
}
