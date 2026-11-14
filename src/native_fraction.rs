// SPDX-License-Identifier: AGPL-3.0-or-later
//! The fixed-width canonical-fraction helpers shared by the native Decimal
//! (FR-403), carried-quantity (FR-398) and Rational (FR-408) carriers: the
//! canonical-pair rule, the 128-step coprimality test, the 256-step gcd and the
//! width-tiered reduction.
//!
//! Everything here is fixed-width native arithmetic: no loop, iterator adapter
//! or recursion, no allocation, and one body for host and `cfg(kani)` builds.

/// Whether `numerator / denominator` is a canonical pair: positive
/// denominator, lowest terms, zero only as `0/1`. A non-positive denominator
/// is never canonical.
pub(crate) fn is_canonical_pair(numerator: i64, denominator: i64) -> bool {
    if denominator <= 0 {
        false
    } else if numerator == 0 {
        denominator == 1
    } else {
        coprime(numerator.unsigned_abs(), denominator.unsigned_abs())
    }
}

/// Whether two nonzero magnitudes share no factor above one, by the binary
/// greatest-common-divisor algorithm unrolled to its worst case (128 steps)
/// so that it holds no loop. Every step at least halves the product of the
/// two odd parts, which starts below `2^126`.
pub(crate) fn coprime(left: u64, right: u64) -> bool {
    if (left | right) & 1 == 0 {
        return false;
    }
    let (divisor, _) = steps_128((
        left >> left.trailing_zeros(),
        right >> right.trailing_zeros(),
    ));
    divisor == 1
}

/// The gcd of two magnitudes, not both zero, by the same 128 unrolled steps
/// on the odd parts, with the common power of two restored.
pub(crate) fn gcd64(left: u64, right: u64) -> u64 {
    if left == 0 {
        return right;
    }
    if right == 0 {
        return left;
    }
    let shift = (left | right).trailing_zeros();
    let (divisor, _) = steps_128((
        left >> left.trailing_zeros(),
        right >> right.trailing_zeros(),
    ));
    divisor << shift
}

/// Reduce `numerator / denominator` for a positive `denominator` to lowest
/// terms with zero as `0/1`, in the width the magnitudes need: `u64` gcd and
/// division when both fit `u64`, else the 256-step `u128` gcd.
pub(crate) fn reduce(numerator: i128, denominator: i128) -> (i128, i128) {
    let (n, d) = (numerator.unsigned_abs(), denominator.unsigned_abs());
    let (n, d) = match (u64::try_from(n), u64::try_from(d)) {
        (Ok(n), Ok(d)) => {
            let gcd = gcd64(n, d);
            (u128::from(n / gcd), u128::from(d / gcd))
        }
        _ => {
            let gcd = gcd256(n, d);
            (n / gcd, d / gcd)
        }
    };
    // `n` is at most `2^127`, so the negation wraps only for `i128::MIN`,
    // which is the intended value.
    let signed = n as i128;
    let numerator = if numerator < 0 {
        signed.wrapping_neg()
    } else {
        signed
    };
    (numerator, d as i128)
}

/// One step on a pair `(small, other)` of odd values or `(value, 0)`: replace
/// the larger by the odd part of the difference. The divisor ends in the first
/// slot with the second zero.
fn step((left, right): (u64, u64)) -> (u64, u64) {
    if right == 0 {
        return (left, right);
    }
    let (small, large) = if left > right {
        (right, left)
    } else {
        (left, right)
    };
    let difference = large - small;
    if difference == 0 {
        (small, 0)
    } else {
        (small, difference >> difference.trailing_zeros())
    }
}

fn steps_2(state: (u64, u64)) -> (u64, u64) {
    step(step(state))
}
fn steps_4(state: (u64, u64)) -> (u64, u64) {
    steps_2(steps_2(state))
}
fn steps_8(state: (u64, u64)) -> (u64, u64) {
    steps_4(steps_4(state))
}
fn steps_16(state: (u64, u64)) -> (u64, u64) {
    steps_8(steps_8(state))
}
fn steps_32(state: (u64, u64)) -> (u64, u64) {
    steps_16(steps_16(state))
}
fn steps_64(state: (u64, u64)) -> (u64, u64) {
    steps_32(steps_32(state))
}
fn steps_128(state: (u64, u64)) -> (u64, u64) {
    steps_64(steps_64(state))
}

pub(crate) type GcdState = (u128, u128, u128);

/// One step of the binary gcd on `(u, v, g)` (FR-403 Behavior 16).
fn gcd_step(state: GcdState) -> GcdState {
    let (u, v, g) = state;
    let (u_even, v_even) = (u & 1 == 0 && u != 0, v & 1 == 0 && v != 0);
    if u_even && v_even {
        (u >> 1, v >> 1, g << 1)
    } else if u_even && v != 0 {
        (u >> 1, v, g)
    } else if v_even && u != 0 {
        (u, v >> 1, g)
    } else if u & 1 == 1 && v & 1 == 1 {
        if u >= v {
            ((u - v) >> 1, v, g)
        } else {
            (u, (v - u) >> 1, g)
        }
    } else {
        state
    }
}

fn gcd_steps_2(state: GcdState) -> GcdState {
    gcd_step(gcd_step(state))
}
fn gcd_steps_4(state: GcdState) -> GcdState {
    gcd_steps_2(gcd_steps_2(state))
}
fn gcd_steps_8(state: GcdState) -> GcdState {
    gcd_steps_4(gcd_steps_4(state))
}
fn gcd_steps_16(state: GcdState) -> GcdState {
    gcd_steps_8(gcd_steps_8(state))
}
fn gcd_steps_32(state: GcdState) -> GcdState {
    gcd_steps_16(gcd_steps_16(state))
}
fn gcd_steps_64(state: GcdState) -> GcdState {
    gcd_steps_32(gcd_steps_32(state))
}
pub(crate) fn gcd_steps_128(state: GcdState) -> GcdState {
    gcd_steps_64(gcd_steps_64(state))
}
fn gcd_steps_256(state: GcdState) -> GcdState {
    gcd_steps_128(gcd_steps_128(state))
}

/// The gcd of two magnitudes, not both zero, by exactly 256 fixed steps. A
/// zero operand leaves the state unchanged, so `gcd(0, m)` is `m`.
pub(crate) fn gcd256(u: u128, v: u128) -> u128 {
    let (u, v, g) = gcd_steps_256((u, v, 1));
    g * (u + v)
}
