// SPDX-License-Identifier: AGPL-3.0-or-later
//! Independent bounded mathematics for the native Rational harnesses
//! (FR-408 Behavior 18). It shares no code with the carrier: the reduction is
//! Euclid's algorithm unrolled to a fixed step count, the capacity quantities
//! are `checked_*` arithmetic on `i128`, the bit lengths are counted by
//! comparison against powers of two, and the charge sequence is written out
//! as literals. Nothing here loops, so a proof over it needs no unwind bound.

use super::{NativeRationalCapacity as Capacity, NativeRationalRefusal as Refusal};

/// A reference operation over `a/p` and `c/q`.
#[derive(Clone, Copy)]
pub(super) enum Op {
    Add,
    Subtract,
    Multiply,
}

macro_rules! euclid_ladder {
    ($ty:ty, $one:ident, $two:ident, $four:ident, $eight:ident, $sixteen:ident) => {
        fn $one((a, b): ($ty, $ty)) -> ($ty, $ty) {
            if b == 0 {
                (a, b)
            } else {
                (b, a % b)
            }
        }
        fn $two(state: ($ty, $ty)) -> ($ty, $ty) {
            $one($one(state))
        }
        fn $four(state: ($ty, $ty)) -> ($ty, $ty) {
            $two($two(state))
        }
        fn $eight(state: ($ty, $ty)) -> ($ty, $ty) {
            $four($four(state))
        }
        fn $sixteen(state: ($ty, $ty)) -> ($ty, $ty) {
            $eight($eight(state))
        }
    };
}

euclid_ladder!(u64, small_1, small_2, small_4, small_8, small_16);
euclid_ladder!(u128, wide_1, wide_2, wide_4, wide_8, wide_16);

fn wide_64(state: (u128, u128)) -> (u128, u128) {
    wide_16(wide_16(wide_16(wide_16(state))))
}

fn wide_256(state: (u128, u128)) -> (u128, u128) {
    wide_64(wide_64(wide_64(wide_64(state))))
}

/// The canonical pair of `n / d` for a positive `d`. `wide` takes 256 Euclid
/// steps over `u128`, enough for any 128-bit magnitudes; otherwise the
/// magnitudes are below `2^63` and 16 steps over `u64` end the algorithm for
/// values below `F(17) = 1597`, which covers the CG domain.
pub(super) fn reduce(n: i128, d: i128, wide: bool) -> (i128, i128) {
    if wide {
        let g = wide_256((n.unsigned_abs(), d.unsigned_abs())).0 as i128;
        (n / g, d / g)
    } else {
        let g = small_16((n.unsigned_abs() as u64, d.unsigned_abs() as u64)).0 as i64;
        ((n as i64 / g).into(), (d as i64 / g).into())
    }
}

/// Whether `n / d` is canonical for `|n| <= 9`-sized magnitudes, where 16
/// Euclid steps end the algorithm.
pub(super) fn canonical_small(n: i64, d: i64) -> bool {
    d > 0
        && if n == 0 {
            d == 1
        } else {
            small_16((n.unsigned_abs(), d.unsigned_abs())).0 == 1
        }
}

/// The expected outcome of `from_i64_parts`: Euclid over `u64` ends within 92
/// steps, and 128 are taken.
pub(super) fn from_parts(n: i64, d: i64) -> Result<(i128, i128), Refusal> {
    if d == 0 {
        return Err(Refusal::ZeroDenominator);
    }
    let state = small_16(small_16(small_16(small_16((
        n.unsigned_abs(),
        d.unsigned_abs(),
    )))));
    let state = small_16(small_16(small_16(small_16(state))));
    let canonical = d > 0 && if n == 0 { d == 1 } else { state.0 == 1 };
    if canonical {
        Ok((i128::from(n), i128::from(d)))
    } else {
        Err(Refusal::NonCanonical)
    }
}

/// The unreduced result fraction, or the first failing quantity.
pub(super) fn formed(op: Op, a: i128, p: i128, c: i128, q: i128) -> Result<(i128, i128), Capacity> {
    let numerator = match op {
        Op::Multiply => a.checked_mul(c).ok_or(Capacity::LeftProduct)?,
        Op::Add => {
            let first = a.checked_mul(q).ok_or(Capacity::LeftProduct)?;
            let second = c.checked_mul(p).ok_or(Capacity::RightProduct)?;
            first.checked_add(second).ok_or(Capacity::Combination)?
        }
        Op::Subtract => {
            let first = a.checked_mul(q).ok_or(Capacity::LeftProduct)?;
            let second = c.checked_mul(p).ok_or(Capacity::RightProduct)?;
            first.checked_sub(second).ok_or(Capacity::Combination)?
        }
    };
    let denominator = p.checked_mul(q).ok_or(Capacity::Denominator)?;
    Ok((numerator, denominator))
}

/// `B(x)`: the bit length of `|x|` with `B(0) = 1`, by binary search with
/// comparisons against powers of two.
pub(super) fn bit_length(value: i128) -> u64 {
    let mut magnitude = value.unsigned_abs();
    let mut length = 1;
    let step = if magnitude >= 1 << 64 { 64 } else { 0 };
    magnitude >>= step;
    length += step;
    let step = if magnitude >= 1 << 32 { 32 } else { 0 };
    magnitude >>= step;
    length += step;
    let step = if magnitude >= 1 << 16 { 16 } else { 0 };
    magnitude >>= step;
    length += step;
    let step = if magnitude >= 1 << 8 { 8 } else { 0 };
    magnitude >>= step;
    length += step;
    let step = if magnitude >= 1 << 4 { 4 } else { 0 };
    magnitude >>= step;
    length += step;
    let step = if magnitude >= 1 << 2 { 2 } else { 0 };
    magnitude >>= step;
    length += step;
    let step = if magnitude >= 2 { 1 } else { 0 };
    length += step;
    length
}

/// The `integer_bits` amounts of the three sized charges of an arithmetic
/// success path: `operands`, `arithmetic` and `normalize`. The sequence is
/// operands (two occurrences), arithmetic, normalize, result-retain: four
/// work units and one result unit.
pub(super) struct ArithmeticSizes {
    pub(super) operands: u64,
    pub(super) arithmetic: u64,
    pub(super) normalize: u64,
}

pub(super) fn arithmetic_sizes(
    op: Op,
    (a, p, c, q): (i128, i128, i128, i128),
    unreduced: (i128, i128),
) -> ArithmeticSizes {
    let (ba, bp, bc, bq) = (bit_length(a), bit_length(p), bit_length(c), bit_length(q));
    let numerator = match op {
        Op::Multiply => ba + bc,
        Op::Add | Op::Subtract => (ba + bq).max(bc + bp) + 1,
    };
    ArithmeticSizes {
        operands: ba.max(bp).max(bc).max(bq),
        arithmetic: numerator.max(bp + bq),
        normalize: bit_length(unreduced.0).max(bit_length(unreduced.1)),
    }
}

/// The `integer_bits` amounts of the two sized charges of an ordering
/// success path (operands with two occurrences, then arithmetic), followed by
/// the result-retain: three work units and one result unit.
pub(super) fn ordering_sizes((a, p, c, q): (i128, i128, i128, i128)) -> (u64, u64) {
    let (ba, bp, bc, bq) = (bit_length(a), bit_length(p), bit_length(c), bit_length(q));
    (ba.max(bp).max(bc).max(bq), (ba + bq).max(bc + bp))
}
