// SPDX-License-Identifier: AGPL-3.0-or-later
//! Integer parsing, addition, multiplication, exact division and greatest
//! common divisor that ask a [`Reservation`] for every limb block before
//! requesting it (FR-396).
//!
//! Every block is a `Vec<u32>` requested once with its exact capacity, and the
//! amount asked is that capacity in bytes. Operands are read as limb streams,
//! so reading, comparing and testing signs allocate nothing. The unmetered
//! `Integer` operations run these with [`Unlimited`].

use alloc::alloc::{handle_alloc_error, Layout};
use alloc::vec::Vec;
use core::cmp::Ordering;

use num_bigint::Sign;

use super::{Integer, IntegerRepr, MagnitudeLimbs, NonCanonicalInteger, NonZeroInteger};
use crate::reservation::{Reservation, ReservationDenied, Unlimited};

const LIMB_BYTES: usize = core::mem::size_of::<u32>();
/// Decimal digits folded into one limb-wide multiply-add when parsing.
const DIGIT_CHUNK: usize = 9;

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

fn limb_count(magnitude: &MagnitudeLimbs<'_>) -> usize {
    magnitude.clone().count()
}

/// Compare magnitudes without allocating.
pub(super) fn compare_magnitudes(
    left: &MagnitudeLimbs<'_>,
    right: &MagnitudeLimbs<'_>,
) -> Ordering {
    let by_length = limb_count(left).cmp(&limb_count(right));
    if by_length != Ordering::Equal {
        return by_length;
    }
    let mut order = Ordering::Equal;
    for (l, r) in left.clone().zip(right.clone()) {
        let limb = l.cmp(&r);
        if limb != Ordering::Equal {
            order = limb;
        }
    }
    order
}

/// Drop high zero limbs, keeping the block's capacity.
fn trim(limbs: &mut Vec<u32>) {
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
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
        let (left, right) = (self.magnitude_limbs(), other.magnitude_limbs());
        let (left_len, right_len) = (limb_count(&left), limb_count(&right));
        if left_negative == right_negative {
            let mut out = limb_block(left_len.max(right_len).saturating_add(1), reservation)?;
            let mut carry = 0_u64;
            let (mut left, mut right) = (left, right);
            for slot in &mut out {
                let sum = u64::from(left.next().unwrap_or(0))
                    + u64::from(right.next().unwrap_or(0))
                    + carry;
                *slot = sum as u32;
                carry = sum >> 32;
            }
            return Ok(Self::from_limbs(left_negative, out));
        }
        match compare_magnitudes(&left, &right) {
            Ordering::Equal => Ok(Self::small128(0)),
            Ordering::Greater => {
                let out = subtract_magnitudes(left, right, left_len, reservation)?;
                Ok(Self::from_limbs(left_negative, out))
            }
            Ordering::Less => {
                let out = subtract_magnitudes(right, left, right_len, reservation)?;
                Ok(Self::from_limbs(right_negative, out))
            }
        }
    }

    /// `self × other`, asking `reservation` for the full size of the one limb
    /// block the product needs before requesting it.
    ///
    /// # Errors
    ///
    /// [`IntegerReserveError`] if the reservation denies the block (no value is
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
        let (left, right) = (self.magnitude_limbs(), other.magnitude_limbs());
        let right_len = limb_count(&right);
        let mut out = limb_block(limb_count(&left).saturating_add(right_len), reservation)?;
        for (row, x) in left.enumerate() {
            let mut carry = 0_u64;
            for (column, y) in right.clone().enumerate() {
                let at = row + column;
                let cell = u64::from(out[at]) + u64::from(x) * u64::from(y) + carry;
                out[at] = cell as u32;
                carry = cell >> 32;
            }
            out[row + right_len] = carry as u32;
        }
        Ok(Self::from_limbs(negative, out))
    }

    /// The quotient rounded toward zero, asking `reservation` for the full size
    /// of each limb block (dividend work copy, divisor work copy and quotient)
    /// before requesting it. Divisibility is not checked.
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
        let (dividend, by) = (self.magnitude_limbs(), divisor.magnitude_limbs());
        if compare_magnitudes(&dividend, &by) == Ordering::Less {
            return Ok(Self::small128(0));
        }
        let negative = self.is_negative() != divisor.is_negative();
        let (dividend_len, divisor_len) = (limb_count(&dividend), limb_count(&by));
        if divisor_len == 1 {
            let by = u64::from(by.clone().next().unwrap_or(1));
            let mut quotient = limb_block(dividend_len, reservation)?;
            for (slot, limb) in quotient.iter_mut().zip(dividend) {
                *slot = limb;
            }
            let mut remainder = 0_u64;
            for slot in quotient.iter_mut().rev() {
                let current = (remainder << 32) | u64::from(*slot);
                *slot = (current / by) as u32;
                remainder = current % by;
            }
            return Ok(Self::from_limbs(negative, quotient));
        }
        let mut v = limb_block(divisor_len, reservation)?;
        let mut u = limb_block(dividend_len + 1, reservation)?;
        let mut quotient = limb_block(dividend_len - divisor_len + 1, reservation)?;
        for (slot, limb) in v.iter_mut().zip(by) {
            *slot = limb;
        }
        for (slot, limb) in u.iter_mut().zip(dividend) {
            *slot = limb;
        }
        let shift = v[divisor_len - 1].leading_zeros();
        shift_left_small(&mut v, shift);
        shift_left_small(&mut u, shift);
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
        let (left, right) = (self.magnitude_limbs(), other.magnitude_limbs());
        let (left_len, right_len) = (limb_count(&left), limb_count(&right));
        if left_len == 0 || right_len == 0 {
            let (kept, kept_len) = if left_len == 0 {
                (right, right_len)
            } else {
                (left, left_len)
            };
            let mut out = limb_block(kept_len, reservation)?;
            for (slot, limb) in out.iter_mut().zip(kept) {
                *slot = limb;
            }
            return Ok(Self::from_limbs(false, out));
        }
        let mut x = limb_block(left_len, reservation)?;
        let mut y = limb_block(right_len, reservation)?;
        for (slot, limb) in x.iter_mut().zip(left) {
            *slot = limb;
        }
        for (slot, limb) in y.iter_mut().zip(right) {
            *slot = limb;
        }
        let (x_twos, y_twos) = (trailing_zero_bits(&x), trailing_zero_bits(&y));
        let common = x_twos.min(y_twos);
        shift_right(&mut x, x_twos);
        shift_right(&mut y, y_twos);
        loop {
            match compare_slices(&x, &y) {
                Ordering::Equal => break,
                Ordering::Greater => {
                    subtract_in_place(&mut x, &y);
                    let twos = trailing_zero_bits(&x);
                    shift_right(&mut x, twos);
                }
                Ordering::Less => {
                    subtract_in_place(&mut y, &x);
                    let twos = trailing_zero_bits(&y);
                    shift_right(&mut y, twos);
                }
            }
        }
        drop(y);
        shift_left_in_capacity(&mut x, common);
        Ok(Self::from_limbs(false, x))
    }

    /// Parse the canonical spelling `^(0|-?[1-9][0-9]*)$`, asking `reservation`
    /// for the full size of the one limb block a value outside `i128` needs
    /// before requesting it. A spelling that fits `i128` requests nothing.
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
        let mut limbs = limb_block(bits.div_ceil(32), reservation)?;
        let mut used = 0_usize;
        for chunk in digits.as_bytes().chunks(DIGIT_CHUNK) {
            let (mut carry, scale) = chunk.iter().fold((0_u64, 1_u64), |(value, scale), digit| {
                (value * 10 + u64::from(digit - b'0'), scale * 10)
            });
            for limb in &mut limbs[..used] {
                let cell = u64::from(*limb) * scale + carry;
                *limb = cell as u32;
                carry = cell >> 32;
            }
            if carry != 0 {
                limbs[used] = carry as u32;
                used += 1;
            }
        }
        Ok(Self::from_limbs(digits.len() != spelling.len(), limbs))
    }
}

/// `larger - smaller` into a fresh block of `larger_len` limbs.
fn subtract_magnitudes(
    mut larger: MagnitudeLimbs<'_>,
    mut smaller: MagnitudeLimbs<'_>,
    larger_len: usize,
    reservation: &mut (impl Reservation + ?Sized),
) -> Result<Vec<u32>, IntegerReserveError> {
    let mut out = limb_block(larger_len, reservation)?;
    let mut borrow = 0_i64;
    for slot in &mut out {
        let mut difference =
            i64::from(larger.next().unwrap_or(0)) - i64::from(smaller.next().unwrap_or(0)) - borrow;
        borrow = i64::from(difference < 0);
        if difference < 0 {
            difference += 1 << 32;
        }
        *slot = difference as u32;
    }
    Ok(out)
}

fn compare_slices(left: &[u32], right: &[u32]) -> Ordering {
    left.len()
        .cmp(&right.len())
        .then_with(|| left.iter().rev().cmp(right.iter().rev()))
}

fn subtract_in_place(minuend: &mut Vec<u32>, subtrahend: &[u32]) {
    let mut borrow = 0_i64;
    for (at, slot) in minuend.iter_mut().enumerate() {
        let mut difference =
            i64::from(*slot) - i64::from(subtrahend.get(at).copied().unwrap_or(0)) - borrow;
        borrow = i64::from(difference < 0);
        if difference < 0 {
            difference += 1 << 32;
        }
        *slot = difference as u32;
    }
    trim(minuend);
}

fn trailing_zero_bits(limbs: &[u32]) -> usize {
    let zero_limbs = limbs.iter().take_while(|limb| **limb == 0).count();
    limbs
        .get(zero_limbs)
        .map_or(0, |limb| zero_limbs * 32 + limb.trailing_zeros() as usize)
}

fn shift_right(limbs: &mut Vec<u32>, bits: usize) {
    limbs.drain(..(bits / 32).min(limbs.len()));
    let bits = (bits % 32) as u32;
    if bits != 0 {
        let mut carry = 0_u32;
        for limb in limbs.iter_mut().rev() {
            let next = *limb << (32 - bits);
            *limb = (*limb >> bits) | carry;
            carry = next;
        }
    }
    trim(limbs);
}

/// Shift left by `bits`; the result is no larger than the block's capacity
/// because the caller shifts back a value at most the first operand's size.
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

/// Shift a block left by fewer than 32 bits, in place.
fn shift_left_small(limbs: &mut [u32], bits: u32) {
    if bits == 0 {
        return;
    }
    for at in (1..limbs.len()).rev() {
        limbs[at] = (limbs[at] << bits) | (limbs[at - 1] >> (32 - bits));
    }
    if let Some(first) = limbs.first_mut() {
        *first <<= bits;
    }
}

/// Knuth's algorithm D: `quotient = u / v` for a normalized `v` of at least two
/// limbs whose top bit is set and a dividend work copy `u` one limb longer
/// than the dividend. The remainder is left in `u`.
fn divide_normalized(u: &mut [u32], v: &[u32], quotient: &mut [u32]) {
    const BASE: u64 = 1 << 32;
    let n = v.len();
    let top = u64::from(v[n - 1]);
    let next = u64::from(v[n - 2]);
    for j in (0..quotient.len()).rev() {
        let numerator = (u64::from(u[j + n]) << 32) | u64::from(u[j + n - 1]);
        let mut estimate = numerator / top;
        let mut remainder = numerator % top;
        while estimate >= BASE || estimate * next > ((remainder << 32) | u64::from(u[j + n - 2])) {
            estimate -= 1;
            remainder += top;
            if remainder >= BASE {
                break;
            }
        }
        let mut borrow = 0_i64;
        let mut carry = 0_u64;
        for i in 0..n {
            let product = estimate * u64::from(v[i]) + carry;
            carry = product >> 32;
            let difference = i64::from(u[i + j]) - borrow - i64::from(product as u32);
            u[i + j] = difference as u32;
            borrow = i64::from(difference < 0);
        }
        let difference = i64::from(u[j + n]) - borrow - carry as i64;
        u[j + n] = difference as u32;
        if difference < 0 {
            estimate -= 1;
            let mut carry = 0_u64;
            for i in 0..n {
                let sum = u64::from(u[i + j]) + u64::from(v[i]) + carry;
                u[i + j] = sum as u32;
                carry = sum >> 32;
            }
            u[j + n] = u[j + n].wrapping_add(carry as u32);
        }
        quotient[j] = estimate as u32;
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
