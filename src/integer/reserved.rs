// SPDX-License-Identifier: AGPL-3.0-or-later
//! Integer parsing, addition, multiplication, exact division and greatest
//! common divisor that ask a [`Reservation`] for every limb block before
//! requesting it (FR-396).
//!
//! Every block is a `Vec<u32>` requested once with its exact capacity, and the
//! amount asked is that capacity in bytes. Operands are read as limb slices, so
//! reading, comparing and testing signs allocate nothing. The unmetered
//! `Integer` operations run these with [`Unlimited`].
//!
//! The hot loops read two 32-bit limbs at a time as one 64-bit digit and
//! accumulate in 128 bits. A block that holds 64-bit digits has an even limb
//! count, with a zero pad limb on top when the value has an odd count.

// The digit loops walk limb pairs with `chunks_exact` so the odd remainder limb
// stays reachable beside the pairs and the in-place `rev()` shifts read alike.
#![allow(clippy::chunks_exact_to_as_chunks)]

use alloc::alloc::{handle_alloc_error, Layout};
use alloc::vec::Vec;
use core::cmp::Ordering;

use num_bigint::Sign;

use super::{Integer, IntegerRepr, NonCanonicalInteger, NonZeroInteger};
use crate::reservation::{Reservation, ReservationDenied, Unlimited};

const LIMB_BYTES: usize = core::mem::size_of::<u32>();
/// Decimal digits folded into one 64-bit multiply-add when parsing (10^19 < 2^64).
const DIGIT_CHUNK: usize = 19;
/// The shorter factor length, in limbs, from which multiplication splits
/// (Karatsuba). Below it the schoolbook rows are faster.
const KARATSUBA_LIMBS: usize = 64;

/// A reserved integer operation could not obtain its storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum IntegerReserveError {
    /// The caller's reservation refused a limb block; nothing was allocated for it.
    #[error("integer limb reservation denied")]
    Denied,
    /// The allocator refused a block the reservation had admitted.
    #[error("integer limb allocation of {requested} bytes failed")]
    Allocation {
        /// Bytes of the failed block, `usize::MAX` if the size overflows `usize`.
        requested: usize,
    },
}

impl From<ReservationDenied> for IntegerReserveError {
    fn from(_: ReservationDenied) -> Self {
        Self::Denied
    }
}

/// A reserved parse refused its text or its storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum IntegerParseError {
    /// The text is not the canonical spelling `^(0|-?[1-9][0-9]*)$`.
    #[error(transparent)]
    NonCanonical(#[from] NonCanonicalInteger),
    /// The text is canonical but its limb storage was refused.
    #[error(transparent)]
    Reserve(#[from] IntegerReserveError),
}

/// Request one zeroed block of `len` limbs, asking `reservation` first.
fn limb_block(
    len: usize,
    reservation: &mut (impl Reservation + ?Sized),
) -> Result<Vec<u32>, IntegerReserveError> {
    let mut block = Vec::new();
    if len == 0 {
        return Ok(block);
    }
    let Some(bytes) = len.checked_mul(LIMB_BYTES) else {
        return Err(IntegerReserveError::Allocation {
            requested: usize::MAX,
        });
    };
    reservation.reserve(bytes)?;
    block
        .try_reserve_exact(len)
        .map_err(|_| IntegerReserveError::Allocation { requested: bytes })?;
    block.resize(len, 0);
    Ok(block)
}

/// `len` rounded up to a whole number of 64-bit digits, in limbs.
fn even(len: usize) -> usize {
    len.saturating_add(len & 1)
}

/// Unwrap the result of an operation run with [`Unlimited`]: it cannot be
/// denied, and an allocator failure aborts as an infallible allocation does.
fn infallible<T>(result: Result<T, IntegerReserveError>) -> T {
    match result {
        Ok(value) => value,
        Err(IntegerReserveError::Denied) => unreachable!("the unlimited reservation never denies"),
        Err(IntegerReserveError::Allocation { requested }) => handle_alloc_error(
            Layout::from_size_align(requested, LIMB_BYTES).unwrap_or(Layout::new::<u32>()),
        ),
    }
}

/// Compare magnitudes without allocating.
pub(super) fn compare_magnitudes(left: &[u32], right: &[u32]) -> Ordering {
    left.len()
        .cmp(&right.len())
        .then_with(|| left.iter().rev().cmp(right.iter().rev()))
}

/// Drop high zero limbs, keeping the block's capacity.
fn trim(limbs: &mut Vec<u32>) {
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
}

fn trimmed(mut limbs: &[u32]) -> &[u32] {
    while let [rest @ .., 0] = limbs {
        limbs = rest;
    }
    limbs
}

/// The 64-bit digit at digit index `at` of a limb slice.
fn digit(limbs: &[u32], at: usize) -> u64 {
    u64::from(limbs[2 * at]) | (u64::from(limbs[2 * at + 1]) << 32)
}

fn set_digit(limbs: &mut [u32], at: usize, value: u64) {
    limbs[2 * at] = value as u32;
    limbs[2 * at + 1] = (value >> 32) as u32;
}

fn pair(limbs: &[u32]) -> u64 {
    u64::from(limbs[0]) | (u64::from(limbs[1]) << 32)
}

/// Add `carry` into `limbs`, propagating; the sum must fit.
fn add_carry(limbs: &mut [u32], carry: u64) {
    let mut carry = u128::from(carry);
    for limb in limbs {
        if carry == 0 {
            return;
        }
        let sum = u128::from(*limb) + carry;
        *limb = sum as u32;
        carry = sum >> 32;
    }
}

/// `dst += src` where `src.len() <= dst.len()`; the sum must fit `dst`.
fn add_assign(dst: &mut [u32], src: &[u32]) {
    let (head, tail) = dst.split_at_mut(src.len());
    let mut carry = 0_u64;
    for (d, s) in head.iter_mut().zip(src) {
        let sum = u64::from(*d) + u64::from(*s) + carry;
        *d = sum as u32;
        carry = sum >> 32;
    }
    add_carry(tail, carry);
}

/// `dst -= src` where `src.len() <= dst.len()` and `dst >= src`.
fn sub_assign(dst: &mut [u32], src: &[u32]) {
    let (head, tail) = dst.split_at_mut(src.len());
    let mut borrow = 0_u64;
    for (d, s) in head.iter_mut().zip(src) {
        let (difference, first) = d.overflowing_sub(*s);
        let (difference, second) = difference.overflowing_sub(borrow as u32);
        *d = difference;
        borrow = u64::from(first | second);
    }
    for limb in tail {
        if borrow == 0 {
            return;
        }
        let (difference, under) = limb.overflowing_sub(1);
        *limb = difference;
        borrow = u64::from(under);
    }
}

/// `out += x * b` for a row whose limbs from `b.len()` up are still zero (the
/// schoolbook rows climb by two limbs, so the previous row stopped short of
/// them): the final carry is stored, not added. `out` holds at least
/// `b.len() + 1` limbs, and `b.len() + 2` when `x` needs 33 or more bits.
fn add_mul_row(out: &mut [u32], x: u64, b: &[u32]) {
    let pairs = b.len() / 2;
    let (head, tail) = out.split_at_mut(2 * pairs);
    let mut carry = 0_u64;
    for (o, y) in head.chunks_exact_mut(2).zip(b.chunks_exact(2)) {
        let t = u128::from(x) * u128::from(pair(y)) + u128::from(pair(o)) + u128::from(carry);
        o[0] = t as u32;
        o[1] = (t >> 32) as u32;
        carry = (t >> 64) as u64;
    }
    let spill = if let [last] = b.chunks_exact(2).remainder() {
        let t = u128::from(x) * u128::from(*last) + u128::from(pair(tail)) + u128::from(carry);
        tail[0] = t as u32;
        tail[1] = (t >> 32) as u32;
        (t >> 64) as u64
    } else {
        carry
    };
    let from = tail.len().min(2) * usize::from(b.len() % 2 == 1);
    if let Some(slot) = tail.get_mut(from) {
        *slot = spill as u32;
    }
    if let Some(slot) = tail.get_mut(from + 1) {
        *slot = (spill >> 32) as u32;
    }
}

/// Schoolbook `out = a * b` for `out.len() == a.len() + b.len()`.
fn mul_school(out: &mut [u32], a: &[u32], b: &[u32]) {
    out.fill(0);
    for (row, x) in a.chunks_exact(2).enumerate() {
        add_mul_row(&mut out[2 * row..], pair(x), b);
    }
    if let [last] = a.chunks_exact(2).remainder() {
        add_mul_row(&mut out[a.len() - 1..], u64::from(*last), b);
    }
}

/// Limbs of scratch `mul_limbs` needs for an `na` by `nb` product, `na >= nb`.
fn mul_scratch(na: usize, nb: usize) -> usize {
    if nb < KARATSUBA_LIMBS {
        return 0;
    }
    let half = na.div_ceil(2);
    if nb <= half {
        let remainder = na % nb;
        let mut deepest = mul_scratch(nb, nb);
        if remainder != 0 {
            deepest = deepest.max(mul_scratch(nb, remainder));
        }
        return 2 * nb + deepest;
    }
    let deepest = mul_scratch(half + 1, half + 1)
        .max(mul_scratch(half, half))
        .max(mul_scratch(na - half, nb - half));
    2 * (half + 1) + 2 * half + 2 + deepest
}

/// `out = a * b` for `a.len() >= b.len()` and `out.len() == a.len() + b.len()`,
/// using Karatsuba above [`KARATSUBA_LIMBS`] with `scratch` of at least
/// `mul_scratch(a.len(), b.len())` limbs.
fn mul_limbs(out: &mut [u32], a: &[u32], b: &[u32], scratch: &mut [u32]) {
    let (na, nb) = (a.len(), b.len());
    if nb < KARATSUBA_LIMBS {
        mul_school(out, a, b);
        return;
    }
    let half = na.div_ceil(2);
    if nb <= half {
        // Unbalanced: multiply chunks of `a` the size of `b` and accumulate.
        let (product, rest) = scratch.split_at_mut(2 * nb);
        out.fill(0);
        for (at, chunk) in a.chunks(nb).enumerate() {
            let product = &mut product[..chunk.len() + nb];
            mul_limbs(product, b, chunk, rest);
            add_assign(&mut out[at * nb..], product);
        }
        return;
    }
    let (a0, a1) = a.split_at(half);
    let (b0, b1) = b.split_at(half);
    let (sum_a, rest) = scratch.split_at_mut(half + 1);
    let (sum_b, rest) = rest.split_at_mut(half + 1);
    let (middle, rest) = rest.split_at_mut(2 * half + 2);
    let (low, high) = out.split_at_mut(2 * half);
    mul_limbs(low, a0, b0, rest);
    mul_limbs(high, a1, b1, rest);
    sum_a[..half].copy_from_slice(a0);
    sum_a[half] = 0;
    add_assign(sum_a, a1);
    sum_b[..half].copy_from_slice(b0);
    sum_b[half] = 0;
    add_assign(sum_b, b1);
    mul_limbs(middle, sum_a, sum_b, rest);
    sub_assign(middle, low);
    sub_assign(middle, high);
    add_assign(&mut out[half..], trimmed(middle));
}

/// Divide the value in `quotient` (a dividend padded to whole digits) by a
/// one-digit divisor in place.
fn divide_short(quotient: &mut [u32], divisor: u64) {
    let mut remainder = 0_u128;
    for at in (0..quotient.len() / 2).rev() {
        let current = (remainder << 64) | u128::from(digit(quotient, at));
        set_digit(quotient, at, (current / u128::from(divisor)) as u64);
        remainder = current % u128::from(divisor);
    }
}

/// Shift a block of whole digits left by `bits < 64`, in place.
fn shift_left_digits(limbs: &mut [u32], bits: u32) {
    if bits == 0 {
        return;
    }
    for at in (1..limbs.len() / 2).rev() {
        let shifted = (digit(limbs, at) << bits) | (digit(limbs, at - 1) >> (64 - bits));
        set_digit(limbs, at, shifted);
    }
    let first = digit(limbs, 0) << bits;
    set_digit(limbs, 0, first);
}

/// Knuth's algorithm D in base 2^64: `quotient = u / v` for a normalized `v` of
/// at least two digits (top bit set) and a dividend work copy `u` one digit
/// longer than the dividend. The remainder is left in `u`.
fn divide_normalized(u: &mut [u32], v: &[u32], quotient: &mut [u32]) {
    let n = v.len() / 2;
    let top = u128::from(digit(v, n - 1));
    let next = u128::from(digit(v, n - 2));
    for j in (0..quotient.len() / 2).rev() {
        let numerator = (u128::from(digit(u, j + n)) << 64) | u128::from(digit(u, j + n - 1));
        let mut estimate = numerator / top;
        let mut remainder = numerator % top;
        while estimate >> 64 != 0
            || estimate * next > ((remainder << 64) | u128::from(digit(u, j + n - 2)))
        {
            estimate -= 1;
            remainder += top;
            if remainder >> 64 != 0 {
                break;
            }
        }
        let mut estimate = estimate as u64;
        let window = &mut u[2 * j..2 * (j + n) + 2];
        let (body, head) = window.split_at_mut(2 * n);
        let mut carry = 0_u64;
        let mut borrow = 0_u64;
        for (u_digit, v_digit) in body.chunks_exact_mut(2).zip(v.chunks_exact(2)) {
            let product = u128::from(estimate) * u128::from(pair(v_digit)) + u128::from(carry);
            carry = (product >> 64) as u64;
            let (difference, first) = pair(u_digit).overflowing_sub(product as u64);
            let (difference, second) = difference.overflowing_sub(borrow);
            borrow = u64::from(first | second);
            u_digit[0] = difference as u32;
            u_digit[1] = (difference >> 32) as u32;
        }
        let (difference, first) = pair(head).overflowing_sub(carry);
        let (difference, second) = difference.overflowing_sub(borrow);
        head[0] = difference as u32;
        head[1] = (difference >> 32) as u32;
        if first | second {
            estimate -= 1;
            let mut carry = 0_u64;
            for (u_digit, v_digit) in body.chunks_exact_mut(2).zip(v.chunks_exact(2)) {
                let sum = u128::from(pair(u_digit)) + u128::from(pair(v_digit)) + u128::from(carry);
                u_digit[0] = sum as u32;
                u_digit[1] = (sum >> 32) as u32;
                carry = (sum >> 64) as u64;
            }
            let top_digit = pair(head).wrapping_add(carry);
            head[0] = top_digit as u32;
            head[1] = (top_digit >> 32) as u32;
        }
        set_digit(quotient, j, estimate);
    }
}

fn trailing_zero_bits(limbs: &[u32]) -> usize {
    let zero_limbs = limbs.iter().take_while(|limb| **limb == 0).count();
    limbs
        .get(zero_limbs)
        .map_or(0, |limb| zero_limbs * 32 + limb.trailing_zeros() as usize)
}

/// Keep a block of whole digits no longer than its significant limbs need.
fn trim_to_digits(limbs: &mut Vec<u32>) {
    let significant = trimmed(limbs).len();
    limbs.truncate(even(significant));
}

/// `x -= y` for blocks of whole digits with `x >= y`.
fn sub_digits(x: &mut [u32], y: &[u32]) {
    let (head, tail) = x.split_at_mut(y.len());
    let mut borrow = 0_u64;
    for (xd, yd) in head.chunks_exact_mut(2).zip(y.chunks_exact(2)) {
        let (d, first) = pair(xd).overflowing_sub(pair(yd));
        let (d, second) = d.overflowing_sub(borrow);
        borrow = u64::from(first | second);
        xd[0] = d as u32;
        xd[1] = (d >> 32) as u32;
    }
    for xd in tail.chunks_exact_mut(2) {
        if borrow == 0 {
            return;
        }
        let (d, under) = pair(xd).overflowing_sub(borrow);
        borrow = u64::from(under);
        xd[0] = d as u32;
        xd[1] = (d >> 32) as u32;
    }
}

/// `x >>= bits` for a block of whole digits, leaving a trimmed block of whole
/// digits.
fn shift_right_digits(x: &mut Vec<u32>, bits: usize) {
    x.drain(..(2 * (bits / 64)).min(x.len()));
    let bits = (bits % 64) as u32;
    if bits != 0 {
        let mut carry = 0_u64;
        for xd in x.chunks_exact_mut(2).rev() {
            let cur = pair(xd);
            let next = cur << (64 - bits);
            let shifted = (cur >> bits) | carry;
            carry = next;
            xd[0] = shifted as u32;
            xd[1] = (shifted >> 32) as u32;
        }
    }
    trim_to_digits(x);
}

/// Shift left by `bits`; the result is no larger than the block's capacity
/// because the caller shifts back a value no larger than the first operand.
fn shift_left_in_capacity(limbs: &mut Vec<u32>, bits: usize) {
    let whole = bits / 32;
    let shifted_len = limbs.len() + whole;
    limbs.resize(shifted_len, 0);
    limbs.rotate_right(whole);
    let bits = (bits % 32) as u32;
    if bits != 0 {
        let mut carry = 0_u32;
        for limb in limbs.iter_mut().skip(whole) {
            let next = *limb >> (32 - bits);
            *limb = (*limb << bits) | carry;
            carry = next;
        }
        if carry != 0 {
            limbs.push(carry);
        }
    }
}

fn binary_gcd_u128(mut left: u128, mut right: u128) -> u128 {
    if left == 0 || right == 0 {
        return left | right;
    }
    let common = (left | right).trailing_zeros();
    left >>= left.trailing_zeros();
    while right != 0 {
        right >>= right.trailing_zeros();
        if left > right {
            core::mem::swap(&mut left, &mut right);
        }
        right -= left;
    }
    left << common
}

impl Integer {
    /// A value from a sign and a limb block, inline when it fits `i128`.
    fn from_limbs(negative: bool, mut limbs: Vec<u32>) -> Self {
        trim(&mut limbs);
        if limbs.len() <= 4 {
            let magnitude = limbs
                .iter()
                .rev()
                .fold(0_u128, |acc, limb| (acc << 32) | u128::from(*limb));
            let inline = if negative {
                0_i128.checked_sub_unsigned(magnitude)
            } else {
                i128::try_from(magnitude).ok()
            };
            if let Some(value) = inline {
                return Self::small128(value);
            }
        }
        Self(IntegerRepr::LargeDigits {
            sign: if negative { Sign::Minus } else { Sign::Plus },
            limbs,
            cache: once_cell::race::OnceBox::new(),
        })
    }

    /// `self + other`, asking `reservation` for the full size of the one limb
    /// block the sum needs before requesting it.
    ///
    /// Operands that both fit `i128` and a sum that fits request nothing.
    ///
    /// # Errors
    ///
    /// [`IntegerReserveError`] if the reservation denies the block (no value is
    /// returned) or the allocator refuses it.
    pub fn add_reserved(
        &self,
        other: &Self,
        reservation: &mut (impl Reservation + ?Sized),
    ) -> Result<Self, IntegerReserveError> {
        if let (Some(left), Some(right)) = (self.native(), other.native()) {
            if let Some(sum) = left.checked_add(right) {
                return Ok(Self::small128(sum));
            }
        }
        let (left_negative, right_negative) = (self.is_negative(), other.is_negative());
        let (left, right) = (self.magnitude(), other.magnitude());
        let (left, right) = (left.as_slice(), right.as_slice());
        if left_negative == right_negative {
            let (longer, shorter) = if left.len() >= right.len() {
                (left, right)
            } else {
                (right, left)
            };
            let mut out = limb_block(longer.len().saturating_add(1), reservation)?;
            out[..longer.len()].copy_from_slice(longer);
            add_assign(&mut out, shorter);
            return Ok(Self::from_limbs(left_negative, out));
        }
        match compare_magnitudes(left, right) {
            Ordering::Equal => Ok(Self::small128(0)),
            Ordering::Greater => {
                let mut out = limb_block(left.len(), reservation)?;
                out.copy_from_slice(left);
                sub_assign(&mut out, right);
                Ok(Self::from_limbs(left_negative, out))
            }
            Ordering::Less => {
                let mut out = limb_block(right.len(), reservation)?;
                out.copy_from_slice(right);
                sub_assign(&mut out, left);
                Ok(Self::from_limbs(right_negative, out))
            }
        }
    }

    /// `self × other`, asking `reservation` for the full size of the product
    /// block and then of the scratch block large products need, before
    /// requesting each.
    ///
    /// # Errors
    ///
    /// [`IntegerReserveError`] if the reservation denies a block (no value is
    /// returned) or the allocator refuses it.
    pub fn mul_reserved(
        &self,
        other: &Self,
        reservation: &mut (impl Reservation + ?Sized),
    ) -> Result<Self, IntegerReserveError> {
        if let (Some(left), Some(right)) = (self.native(), other.native()) {
            if let Some(product) = left.checked_mul(right) {
                return Ok(Self::small128(product));
            }
        }
        if self.is_zero() || other.is_zero() {
            return Ok(Self::small128(0));
        }
        let negative = self.is_negative() != other.is_negative();
        let (left, right) = (self.magnitude(), other.magnitude());
        let (left, right) = (left.as_slice(), right.as_slice());
        let (longer, shorter) = if left.len() >= right.len() {
            (left, right)
        } else {
            (right, left)
        };
        let mut out = limb_block(longer.len().saturating_add(shorter.len()), reservation)?;
        let mut scratch = limb_block(mul_scratch(longer.len(), shorter.len()), reservation)?;
        mul_limbs(&mut out, longer, shorter, &mut scratch);
        Ok(Self::from_limbs(negative, out))
    }

    /// The quotient rounded toward zero, asking `reservation` for the full size
    /// of each limb block (divisor work copy, dividend work copy and quotient,
    /// each a whole number of 64-bit digits) before requesting it. Divisibility
    /// is not checked.
    ///
    /// # Errors
    ///
    /// [`IntegerReserveError`] if the reservation denies a block (no value is
    /// returned) or the allocator refuses it.
    pub fn exact_div_reserved(
        &self,
        divisor: NonZeroInteger<'_>,
        reservation: &mut (impl Reservation + ?Sized),
    ) -> Result<Self, IntegerReserveError> {
        let divisor = divisor.get();
        if let (Some(dividend), Some(by)) = (self.native(), divisor.native()) {
            if let Some(quotient) = dividend.checked_div(by) {
                return Ok(Self::small128(quotient));
            }
        }
        let (dividend, by) = (self.magnitude(), divisor.magnitude());
        let (dividend, by) = (dividend.as_slice(), by.as_slice());
        if compare_magnitudes(dividend, by) == Ordering::Less {
            return Ok(Self::small128(0));
        }
        let negative = self.is_negative() != divisor.is_negative();
        let (dividend_digits, divisor_digits) = (dividend.len().div_ceil(2), by.len().div_ceil(2));
        if divisor_digits == 1 {
            let mut quotient = limb_block(2 * dividend_digits, reservation)?;
            quotient[..dividend.len()].copy_from_slice(dividend);
            let mut padded = [0_u32; 2];
            padded[..by.len()].copy_from_slice(by);
            divide_short(&mut quotient, pair(&padded));
            return Ok(Self::from_limbs(negative, quotient));
        }
        let mut v = limb_block(2 * divisor_digits, reservation)?;
        let mut u = limb_block(2 * dividend_digits + 2, reservation)?;
        let mut quotient = limb_block(2 * (dividend_digits - divisor_digits + 1), reservation)?;
        v[..by.len()].copy_from_slice(by);
        u[..dividend.len()].copy_from_slice(dividend);
        let shift = digit(&v, divisor_digits - 1).leading_zeros();
        shift_left_digits(&mut v, shift);
        shift_left_digits(&mut u, shift);
        divide_normalized(&mut u, &v, &mut quotient);
        Ok(Self::from_limbs(negative, quotient))
    }

    /// The non-negative greatest common divisor, asking `reservation` for the
    /// full size of each work block (one copy of each operand) before
    /// requesting it. The result keeps the first operand's block.
    ///
    /// # Errors
    ///
    /// [`IntegerReserveError`] if the reservation denies a block (no value is
    /// returned) or the allocator refuses it.
    pub fn gcd_reserved(
        &self,
        other: &Self,
        reservation: &mut (impl Reservation + ?Sized),
    ) -> Result<Self, IntegerReserveError> {
        if let (Some(left), Some(right)) = (self.native(), other.native()) {
            let divisor = binary_gcd_u128(left.unsigned_abs(), right.unsigned_abs());
            if let Ok(divisor) = i128::try_from(divisor) {
                return Ok(Self::small128(divisor));
            }
        }
        let (left, right) = (self.magnitude(), other.magnitude());
        let (left, right) = (left.as_slice(), right.as_slice());
        if left.is_empty() || right.is_empty() {
            let kept = if left.is_empty() { right } else { left };
            let mut out = limb_block(kept.len(), reservation)?;
            out.copy_from_slice(kept);
            return Ok(Self::from_limbs(false, out));
        }
        let mut x = limb_block(even(left.len()), reservation)?;
        let mut y = limb_block(even(right.len()), reservation)?;
        x[..left.len()].copy_from_slice(left);
        y[..right.len()].copy_from_slice(right);
        let (x_twos, y_twos) = (trailing_zero_bits(&x), trailing_zero_bits(&y));
        let common = x_twos.min(y_twos);
        shift_right_digits(&mut x, x_twos);
        shift_right_digits(&mut y, y_twos);
        loop {
            match compare_magnitudes(trimmed(&x), trimmed(&y)) {
                Ordering::Equal => break,
                Ordering::Greater => {
                    sub_digits(&mut x, &y);
                    let twos = trailing_zero_bits(&x);
                    shift_right_digits(&mut x, twos);
                }
                Ordering::Less => {
                    sub_digits(&mut y, &x);
                    let twos = trailing_zero_bits(&y);
                    shift_right_digits(&mut y, twos);
                }
            }
        }
        drop(y);
        trim(&mut x);
        shift_left_in_capacity(&mut x, common);
        Ok(Self::from_limbs(false, x))
    }

    /// Parse the canonical spelling `^(0|-?[1-9][0-9]*)$`, asking `reservation`
    /// for the full size of the one limb block (a whole number of 64-bit
    /// digits) a value outside `i128` needs before requesting it. A spelling
    /// that fits `i128` requests nothing.
    ///
    /// # Errors
    ///
    /// [`IntegerParseError::NonCanonical`] for any other spelling, before any
    /// request; [`IntegerParseError::Reserve`] if the block is denied (no value
    /// is returned) or refused by the allocator.
    pub fn parse_reserved(
        spelling: &str,
        reservation: &mut (impl Reservation + ?Sized),
    ) -> Result<Self, IntegerParseError> {
        let digits = spelling.strip_prefix('-').unwrap_or(spelling);
        let canonical = match digits.as_bytes() {
            [] => false,
            [b'0'] => digits.len() == spelling.len(),
            [first, rest @ ..] => {
                (b'1'..=b'9').contains(first) && rest.iter().all(u8::is_ascii_digit)
            }
        };
        if !canonical {
            return Err(NonCanonicalInteger.into());
        }
        if let Ok(value) = spelling.parse::<i128>() {
            return Ok(Self::small128(value));
        }
        // 10^n < 2^(10n/3), so ceil(10n/3) bits hold n digits.
        let Some(bits) = digits
            .len()
            .checked_mul(10)
            .map(|scaled| scaled.div_ceil(3))
        else {
            return Err(IntegerReserveError::Allocation {
                requested: usize::MAX,
            }
            .into());
        };
        let mut limbs = limb_block(even(bits.div_ceil(32)), reservation)?;
        let mut used = 0_usize;
        for chunk in digits.as_bytes().chunks(DIGIT_CHUNK) {
            let (mut carry, scale) = chunk.iter().fold((0_u64, 1_u64), |(value, scale), digit| {
                (value * 10 + u64::from(digit - b'0'), scale * 10)
            });
            let span = even(used);
            for limb in limbs[..span].chunks_exact_mut(2) {
                let cell = u128::from(pair(limb)) * u128::from(scale) + u128::from(carry);
                limb[0] = cell as u32;
                limb[1] = (cell >> 32) as u32;
                carry = (cell >> 64) as u64;
            }
            used = span;
            if carry != 0 {
                limbs[used] = carry as u32;
                limbs[used + 1] = (carry >> 32) as u32;
                used += 2;
            }
            while used > 0 && limbs[used - 1] == 0 {
                used -= 1;
            }
        }
        Ok(Self::from_limbs(digits.len() != spelling.len(), limbs))
    }
}

impl Integer {
    pub(super) fn add_unlimited(&self, other: &Self) -> Self {
        infallible(self.add_reserved(other, &mut Unlimited))
    }

    pub(super) fn mul_unlimited(&self, other: &Self) -> Self {
        infallible(self.mul_reserved(other, &mut Unlimited))
    }

    pub(super) fn exact_div_unlimited(&self, divisor: NonZeroInteger<'_>) -> Self {
        infallible(self.exact_div_reserved(divisor, &mut Unlimited))
    }

    pub(super) fn gcd_unlimited(&self, other: &Self) -> Self {
        infallible(self.gcd_reserved(other, &mut Unlimited))
    }

    pub(super) fn parse_unlimited(spelling: &str) -> Result<Self, NonCanonicalInteger> {
        match Self::parse_reserved(spelling, &mut Unlimited) {
            Ok(value) => Ok(value),
            Err(IntegerParseError::NonCanonical(error)) => Err(error),
            Err(IntegerParseError::Reserve(error)) => infallible(Err(error)),
        }
    }
}
