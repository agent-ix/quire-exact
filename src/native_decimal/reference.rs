// SPDX-License-Identifier: AGPL-3.0-or-later
//! Independent bounded mathematics for the native Decimal carrier. It shares
//! no code with the constructor or the normalized view: powers of ten are
//! written as literals, magnitudes are `u128`, and normalization is stated
//! as a postcondition, never computed. Host tests bind it to the owner route.

use core::cmp::Ordering;

/// `10^0` through `10^38`, written out.
pub(super) const TEN_POWERS: [u128; 39] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
    100_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000_000_000,
    10_000_000_000_000_000_000_000_000_000_000_000_000,
    100_000_000_000_000_000_000_000_000_000_000_000_000,
];

/// A sign and a `u128` magnitude; zero is never negative.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SignMagnitude {
    negative: bool,
    magnitude: u128,
}

fn split(value: i128) -> SignMagnitude {
    let bits = u128::from_le_bytes(value.to_le_bytes());
    SignMagnitude {
        negative: value < 0,
        magnitude: if value < 0 {
            (!bits).wrapping_add(1)
        } else {
            bits
        },
    }
}

fn order(a: SignMagnitude, b: SignMagnitude) -> Ordering {
    match (a.negative, b.negative) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (false, false) => a.magnitude.cmp(&b.magnitude),
        (true, true) => b.magnitude.cmp(&a.magnitude),
    }
}

/// Whether `(c2, s2)` is the least-scale pair equal in value to `(c, s)`:
/// the scale never rises, `c = c2 x 10^(s - s2)`, and the coefficient is not
/// divisible by ten unless the scale is zero.
pub(super) fn is_least_scale_pair(c: i128, s: u32, c2: i128, s2: u32) -> bool {
    if s2 > s {
        return false;
    }
    let drop = s - s2;
    let value_equal = if drop > 38 {
        c == 0 && c2 == 0
    } else {
        let a = split(c);
        let b = split(c2);
        match b.magnitude.checked_mul(TEN_POWERS[drop as usize]) {
            Some(magnitude) => {
                a.magnitude == magnitude && (a.negative == b.negative || magnitude == 0)
            }
            None => false,
        }
    };
    let least = s2 == 0 || c2 % 10 != 0;
    value_equal && least
}

/// Whether the normalized pair `(c2, s2)` is a member of
/// `Decimal[lower, upper; min_scale, max_scale]` by the QSpec FR-140 rule.
pub(super) fn is_member(
    lower: i128,
    upper: i128,
    min_scale: u32,
    max_scale: u32,
    c2: i128,
    s2: u32,
) -> bool {
    let lifted_scale = s2.max(min_scale);
    if lifted_scale > max_scale {
        return false;
    }
    let shift = lifted_scale - s2;
    let base = split(c2);
    let lifted = if base.magnitude == 0 {
        Some(base)
    } else if shift > 38 {
        None
    } else {
        base.magnitude
            .checked_mul(TEN_POWERS[shift as usize])
            .map(|magnitude| SignMagnitude {
                negative: base.negative,
                magnitude,
            })
    };
    match lifted {
        // A magnitude beyond u128 exceeds both native bounds in its own sign.
        None => false,
        Some(value) => {
            order(value, split(lower)) != Ordering::Less
                && order(value, split(upper)) != Ordering::Greater
        }
    }
}
