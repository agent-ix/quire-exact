// SPDX-License-Identifier: AGPL-3.0-or-later
//! Mathematical integers and explicit inclusive integer domains (QSpec FR-147).
//!
//! `Integer` is unbounded. A finite consumer never narrows it: membership in an
//! [`IntegerInterval`] is an explicit admission that either returns a
//! [`BoundedInteger`] or refuses.
//!
//! Seventeen `Integer`/`IntegerInterval` methods are `pub` so a caller above this
//! crate can use them: [`abs`](Integer::abs), [`pow`](Integer::pow),
//! [`split_factor_two`](Integer::split_factor_two),
//! [`shifted_left`](Integer::shifted_left), [`add`](Integer::add),
//! [`sub`](Integer::sub), [`mul`](Integer::mul), [`neg`](Integer::neg),
//! [`gcd`](Integer::gcd), [`exact_div`](Integer::exact_div),
//! [`div_rem_truncating`](Integer::div_rem_truncating),
//! [`div_mod_floor`](Integer::div_mod_floor),
//! [`power_of_ten`](Integer::power_of_ten),
//! [`power_product_bits`](Integer::power_product_bits),
//! [`spanning`](IntegerInterval::spanning), [`from_big`](Integer::from_big)
//! and [`as_big`](Integer::as_big).

use alloc::{string::String, vec::Vec};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::num::NonZeroU32;
use core::str::FromStr;

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer as _;
use num_traits::{One, Signed, Zero};

use crate::accounting::length_amount;

/// An exact, arbitrary-precision mathematical integer.
///
/// Values that fit `i128` are stored inline, with separate private i64 and
/// i128 payloads. The first [`as_big`](Self::as_big)
/// call lazily initializes one retained immutable `BigInt` image; clones of an
/// inline value preserve its payload class and begin with an empty image cache. The safe
/// `once_cell::race::OnceBox` may construct multiple candidates concurrently,
/// retains one, and drops every losing candidate. Values outside `i128` retain
/// their `BigInt` directly.
///
/// quire:canonical
pub struct Integer(IntegerRepr);

/// Storage for an owned integer copy could not be reserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum IntegerCopyError {
    /// Minimum additional payload bytes of the failed reservation.
    #[error("integer copy allocation of {requested} bytes failed")]
    Allocation { requested: usize },
}

/// Refusal to render an integer as bounded canonical decimal text.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum IntegerDecimalError {
    /// Minimum additional payload bytes of the failed reservation.
    #[error("integer decimal allocation of {requested} bytes failed")]
    Allocation { requested: usize },
    /// The representable spelling exceeds the caller's byte limit.
    #[error("integer decimal output needs {required} bytes, limit {limit}")]
    ByteLimit { required: usize, limit: usize },
    /// The complete spelling length cannot fit `usize`.
    #[error("integer decimal output length overflows usize")]
    OutputSizeOverflow,
}

fn reservation_bytes<T>(current: usize, additional: usize) -> usize {
    current
        .checked_add(additional)
        .and_then(|_| additional.checked_mul(core::mem::size_of::<T>()))
        .unwrap_or(usize::MAX)
}

fn decimal_length(
    chunks: usize,
    highest_digits: usize,
    negative: bool,
    limit: usize,
) -> Result<usize, IntegerDecimalError> {
    let required = chunks
        .saturating_sub(1)
        .checked_mul(9)
        .and_then(|length| length.checked_add(highest_digits))
        .and_then(|length| length.checked_add(usize::from(negative)))
        .ok_or(IntegerDecimalError::OutputSizeOverflow)?;
    if required > limit {
        Err(IntegerDecimalError::ByteLimit { required, limit })
    } else {
        Ok(required)
    }
}

const DECIMAL_CHUNK_BASE: u64 = 1_000_000_000;
const BINARY_LIMB_BASE: u64 = 1_u64 << 32;

fn append_binary_limb(
    chunks: &mut Vec<u32>,
    limb: u32,
    before_reserve: &mut impl FnMut(usize) -> Result<(), IntegerDecimalError>,
) -> Result<(), IntegerDecimalError> {
    let mut carry = u64::from(limb);
    for chunk in chunks.iter_mut() {
        let value = u64::from(*chunk) * BINARY_LIMB_BASE + carry;
        *chunk = (value % DECIMAL_CHUNK_BASE) as u32;
        carry = value / DECIMAL_CHUNK_BASE;
    }
    while carry != 0 {
        if chunks.len() == chunks.capacity() {
            let requested = reservation_bytes::<u32>(chunks.len(), 1);
            if requested == usize::MAX {
                return Err(IntegerDecimalError::Allocation { requested });
            }
            before_reserve(requested)?;
            chunks
                .try_reserve(1)
                .map_err(|_| IntegerDecimalError::Allocation { requested })?;
        }
        chunks.push((carry % DECIMAL_CHUNK_BASE) as u32);
        carry /= DECIMAL_CHUNK_BASE;
    }
    Ok(())
}

fn write_decimal_chunk(output: &mut String, mut chunk: u32, width: usize) {
    let mut digits = [b'0'; 9];
    for digit in digits[9 - width..].iter_mut().rev() {
        *digit = b'0' + (chunk % 10) as u8;
        chunk /= 10;
    }
    for digit in &digits[9 - width..] {
        output.push(char::from(*digit));
    }
}

enum IntegerRepr {
    Small64 {
        value: i64,
        cache: once_cell::race::OnceBox<BigInt>,
    },
    Small128 {
        value: i128,
        cache: once_cell::race::OnceBox<BigInt>,
    },
    // Normalization admits this variant only outside the i128 range; cloning
    // preserves that invariant. Mixed inline/Large comparisons rely on it.
    Large(BigInt),
    // A fallible copy retains its reserved base-2^32 limbs. Materialization
    // occurs only through the pre-existing, infallible `as_big` API.
    LargeDigits {
        sign: Sign,
        limbs: Vec<u32>,
        cache: once_cell::race::OnceBox<BigInt>,
    },
}

impl Integer {
    fn small64(value: i64) -> Self {
        Self(IntegerRepr::Small64 {
            value,
            cache: once_cell::race::OnceBox::new(),
        })
    }

    fn small128(value: i128) -> Self {
        Self(IntegerRepr::Small128 {
            value,
            cache: once_cell::race::OnceBox::new(),
        })
    }

    fn from_big_normalized(value: BigInt) -> Self {
        match i128::try_from(&value) {
            Ok(value) => Self::small128(value),
            Err(_) => Self(IntegerRepr::Large(value)),
        }
    }

    /// Copy this integer into separately owned storage, refusing a failed reservation.
    pub fn try_clone(&self) -> Result<Self, IntegerCopyError> {
        self.try_clone_with(&mut |_| Ok(()))
    }

    fn try_clone_with(
        &self,
        before_reserve: &mut impl FnMut(usize) -> Result<(), IntegerCopyError>,
    ) -> Result<Self, IntegerCopyError> {
        match &self.0 {
            IntegerRepr::Small64 { value, .. } => Ok(Self::small64(*value)),
            IntegerRepr::Small128 { value, .. } => Ok(Self::small128(*value)),
            IntegerRepr::Large(value) => {
                let limbs = value.iter_u32_digits();
                let requested = reservation_bytes::<u32>(0, limbs.len());
                if requested == usize::MAX {
                    return Err(IntegerCopyError::Allocation { requested });
                }
                let mut copy = Vec::new();
                before_reserve(requested)?;
                copy.try_reserve_exact(limbs.len())
                    .map_err(|_| IntegerCopyError::Allocation { requested })?;
                copy.extend(limbs);
                Ok(Self(IntegerRepr::LargeDigits {
                    sign: value.sign(),
                    limbs: copy,
                    cache: once_cell::race::OnceBox::new(),
                }))
            }
            IntegerRepr::LargeDigits { sign, limbs, .. } => {
                let requested = reservation_bytes::<u32>(0, limbs.len());
                if requested == usize::MAX {
                    return Err(IntegerCopyError::Allocation { requested });
                }
                let mut copy = Vec::new();
                before_reserve(requested)?;
                copy.try_reserve_exact(limbs.len())
                    .map_err(|_| IntegerCopyError::Allocation { requested })?;
                copy.extend_from_slice(limbs);
                Ok(Self(IntegerRepr::LargeDigits {
                    sign: *sign,
                    limbs: copy,
                    cache: once_cell::race::OnceBox::new(),
                }))
            }
        }
    }

    /// Render canonical decimal text within `max_bytes`, with typed size and
    /// storage refusals. The source and its lazy image cache remain unchanged.
    ///
    /// ```
    /// use quire_exact::{Integer, IntegerCopyError, IntegerDecimalError};
    /// let original = Integer::from(42_i64);
    /// let copied = original.try_clone();
    /// match copied {
    ///     Ok(value) => assert_eq!(value, original),
    ///     Err(IntegerCopyError::Allocation { requested }) => assert!(requested > 0),
    /// }
    /// match original.try_to_decimal_string(2) {
    ///     Ok(text) => assert_eq!(text, "42"),
    ///     Err(IntegerDecimalError::Allocation { requested }) => assert!(requested > 0),
    ///     Err(IntegerDecimalError::ByteLimit { required, limit }) => assert!(required > limit),
    ///     Err(IntegerDecimalError::OutputSizeOverflow) => panic!("small text fits usize"),
    /// }
    /// ```
    pub fn try_to_decimal_string(&self, max_bytes: usize) -> Result<String, IntegerDecimalError> {
        self.try_to_decimal_string_with(max_bytes, &mut |_| Ok(()))
    }

    fn try_to_decimal_string_with(
        &self,
        max_bytes: usize,
        before_reserve: &mut impl FnMut(usize) -> Result<(), IntegerDecimalError>,
    ) -> Result<String, IntegerDecimalError> {
        let mut chunks = Vec::new();
        match &self.0 {
            IntegerRepr::Small64 { value, .. } => {
                let mut magnitude = value.unsigned_abs();
                let mut limbs = [0_u32; 2];
                let mut count = 0;
                while magnitude != 0 {
                    limbs[count] = magnitude as u32;
                    magnitude >>= 32;
                    count += 1;
                }
                for limb in limbs[..count].iter().rev() {
                    append_binary_limb(&mut chunks, *limb, before_reserve)?;
                }
            }
            IntegerRepr::Small128 { value, .. } => {
                let mut magnitude = value.unsigned_abs();
                let mut limbs = [0_u32; 4];
                let mut count = 0;
                while magnitude != 0 {
                    limbs[count] = magnitude as u32;
                    magnitude >>= 32;
                    count += 1;
                }
                for limb in limbs[..count].iter().rev() {
                    append_binary_limb(&mut chunks, *limb, before_reserve)?;
                }
            }
            IntegerRepr::Large(value) => {
                for limb in value.iter_u32_digits().rev() {
                    append_binary_limb(&mut chunks, limb, before_reserve)?;
                }
            }
            IntegerRepr::LargeDigits { limbs, .. } => {
                for limb in limbs.iter().rev() {
                    append_binary_limb(&mut chunks, *limb, before_reserve)?;
                }
            }
        }
        let negative = self.is_negative();
        let highest = chunks.last().copied().unwrap_or(0);
        let highest_digits = if highest == 0 {
            1
        } else {
            highest.ilog10() as usize + 1
        };
        let count = chunks.len().max(1);
        let required = decimal_length(count, highest_digits, negative, max_bytes)?;
        let mut output = String::new();
        before_reserve(required)?;
        output
            .try_reserve_exact(required)
            .map_err(|_| IntegerDecimalError::Allocation {
                requested: required,
            })?;
        if negative {
            output.push('-');
        }
        write_decimal_chunk(&mut output, highest, highest_digits);
        for chunk in chunks.iter().rev().skip(1) {
            write_decimal_chunk(&mut output, *chunk, 9);
        }
        Ok(output)
    }

    pub(crate) fn native(&self) -> Option<i128> {
        match &self.0 {
            IntegerRepr::Small64 { value, .. } => Some(i128::from(*value)),
            IntegerRepr::Small128 { value, .. } => Some(*value),
            IntegerRepr::Large(_) | IntegerRepr::LargeDigits { .. } => None,
        }
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self::small64(0)
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        match &self.0 {
            IntegerRepr::Small64 { value, .. } => Self::small64(*value),
            IntegerRepr::Small128 { value, .. } => Self::small128(*value),
            IntegerRepr::Large(value) => Self(IntegerRepr::Large(value.clone())),
            IntegerRepr::LargeDigits { sign, limbs, .. } => Self(IntegerRepr::LargeDigits {
                sign: *sign,
                limbs: limbs.clone(),
                cache: once_cell::race::OnceBox::new(),
            }),
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == core::cmp::Ordering::Equal
    }
}
impl Eq for Integer {}
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Integer {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (&self.0, &other.0) {
            (
                IntegerRepr::Small64 { value: left, .. },
                IntegerRepr::Small64 { value: right, .. },
            ) => left.cmp(right),
            (
                IntegerRepr::Small64 { value: left, .. },
                IntegerRepr::Small128 { value: right, .. },
            ) => i128::from(*left).cmp(right),
            (
                IntegerRepr::Small128 { value: left, .. },
                IntegerRepr::Small64 { value: right, .. },
            ) => left.cmp(&i128::from(*right)),
            (
                IntegerRepr::Small128 { value: left, .. },
                IntegerRepr::Small128 { value: right, .. },
            ) => left.cmp(right),
            (IntegerRepr::Large(left), IntegerRepr::Large(right)) => left.cmp(right),
            (
                IntegerRepr::Large(left),
                IntegerRepr::Small64 { .. } | IntegerRepr::Small128 { .. },
            ) => {
                if left.is_negative() {
                    core::cmp::Ordering::Less
                } else {
                    core::cmp::Ordering::Greater
                }
            }
            (
                IntegerRepr::Small64 { .. } | IntegerRepr::Small128 { .. },
                IntegerRepr::Large(right),
            ) => {
                if right.is_negative() {
                    core::cmp::Ordering::Greater
                } else {
                    core::cmp::Ordering::Less
                }
            }
            (IntegerRepr::LargeDigits { .. }, _) | (_, IntegerRepr::LargeDigits { .. }) => {
                self.as_big().cmp(other.as_big())
            }
        }
    }
}
impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_big().hash(state);
    }
}

pub(crate) struct MagnitudeLimbs<'a>(MagnitudeLimbsSource<'a>);

enum MagnitudeLimbsSource<'a> {
    Small { magnitude: u128, remaining: u8 },
    Large(num_bigint::U32Digits<'a>),
    Copied(core::slice::Iter<'a, u32>),
}

impl Iterator for MagnitudeLimbs<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            MagnitudeLimbsSource::Small {
                magnitude,
                remaining,
            } if *remaining > 0 => {
                let limb = *magnitude as u32;
                *magnitude >>= 32;
                *remaining -= 1;
                Some(limb)
            }
            MagnitudeLimbsSource::Small { .. } => None,
            MagnitudeLimbsSource::Large(digits) => digits.next(),
            MagnitudeLimbsSource::Copied(digits) => digits.next().copied(),
        }
    }
}
impl fmt::Debug for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Integer")
            .field(self.as_big())
            .finish()
    }
}

/// A shared borrow admitted as a nonzero mathematical integer (FR-374).
///
/// Trace: FR-374-AC-2
///
/// Copying the token preserves the borrow and never clones its integer.
/// External callers obtain it through `TryFrom<&Integer>` and can reuse one
/// admitted token across all three division helpers:
///
/// ```
/// use quire_exact::{Integer, NonZeroInteger};
/// let divisor = Integer::from(2_i64);
/// let token = NonZeroInteger::try_from(&divisor).unwrap();
/// assert_eq!(Integer::from(6_i64).exact_div(token), Integer::from(3_i64));
/// assert_eq!(Integer::from(7_i64).div_rem_truncating(token), (Integer::from(3_i64), Integer::from(1_i64)));
/// assert_eq!(Integer::from(-7_i64).div_mod_floor(token), (Integer::from(-4_i64), Integer::from(1_i64)));
/// ```
///
/// Plain integer borrows cannot be passed to any division helper:
///
/// ```compile_fail,E0308
/// use quire_exact::Integer;
/// let two = Integer::from(2_i64);
/// Integer::one().exact_div(&two);
/// ```
///
/// ```compile_fail,E0308
/// use quire_exact::Integer;
/// let two = Integer::from(2_i64);
/// Integer::one().div_rem_truncating(&two);
/// ```
///
/// ```compile_fail,E0308
/// use quire_exact::Integer;
/// let two = Integer::from(2_i64);
/// Integer::one().div_mod_floor(&two);
/// ```
///
/// The referent is private, immutable and cannot outlive its owner:
///
/// ```compile_fail,E0423
/// use quire_exact::{Integer, NonZeroInteger};
/// let zero = Integer::zero();
/// let token = NonZeroInteger(&zero);
/// ```
///
/// ```compile_fail,E0594
/// use quire_exact::{Integer, NonZeroInteger};
/// let two = Integer::from(2_i64);
/// let token = NonZeroInteger::try_from(&two).unwrap();
/// *token.get() = Integer::zero();
/// ```
///
/// ```compile_fail,E0597
/// use quire_exact::{Integer, NonZeroInteger};
/// let token = {
///     let two = Integer::from(2_i64);
///     NonZeroInteger::try_from(&two).unwrap()
/// };
/// let _ = token.get();
/// ```
///
/// The owner cannot be replaced while an admitted borrow remains in use:
///
/// ```compile_fail,E0506
/// use quire_exact::{Integer, NonZeroInteger};
/// let mut two = Integer::from(2_i64);
/// let token = NonZeroInteger::try_from(&two).unwrap();
/// two = Integer::zero();
/// let _ = token.get();
/// ```
///
/// Zero cannot be produced through a default or internal constructor:
///
/// ```compile_fail,E0277
/// use quire_exact::NonZeroInteger;
/// let token: NonZeroInteger<'_> = Default::default();
/// ```
///
/// ```compile_fail,E0624
/// use quire_exact::{Integer, NonZeroInteger};
/// let zero = Integer::zero();
/// let token = NonZeroInteger::from_proven_nonzero(&zero);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonZeroInteger<'a>(&'a Integer);

/// Admission of a zero integer as a divisor was requested.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("zero integer divisor")]
pub struct ZeroDivisor;

impl<'a> TryFrom<&'a Integer> for NonZeroInteger<'a> {
    type Error = ZeroDivisor;

    fn try_from(value: &'a Integer) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(ZeroDivisor)
        } else {
            Ok(Self(value))
        }
    }
}

impl<'a> NonZeroInteger<'a> {
    /// Borrow the original admitted integer without granting mutation.
    pub fn get(self) -> &'a Integer {
        self.0
    }

    /// Borrow an integer whose owning canonical path proves it nonzero.
    /// Every caller documents that mathematical premise. A violated premise
    /// is an implementation defect, rather than a substitute arithmetic value.
    pub(crate) fn from_proven_nonzero(value: &'a Integer) -> Self {
        assert!(!value.is_zero(), "canonical divisor must be nonzero");
        Self(value)
    }
}

impl Integer {
    /// The integer zero.
    pub fn zero() -> Self {
        Self::small64(0)
    }

    /// The integer one.
    pub fn one() -> Self {
        Self::small64(1)
    }

    /// Whether this integer is zero.
    pub fn is_zero(&self) -> bool {
        match &self.0 {
            IntegerRepr::Small64 { value, .. } => *value == 0,
            IntegerRepr::Small128 { value, .. } => *value == 0,
            IntegerRepr::Large(value) => value.is_zero(),
            IntegerRepr::LargeDigits { sign, .. } => *sign == Sign::NoSign,
        }
    }

    /// Whether this integer is strictly negative.
    pub fn is_negative(&self) -> bool {
        match &self.0 {
            IntegerRepr::Small64 { value, .. } => *value < 0,
            IntegerRepr::Small128 { value, .. } => *value < 0,
            IntegerRepr::Large(value) => value.is_negative(),
            IntegerRepr::LargeDigits { sign, .. } => *sign == Sign::Minus,
        }
    }

    /// `bits(x)` from `quire.value.accounting/v1`: the magnitude bit length,
    /// where zero has length one.
    pub fn magnitude_bits(&self) -> u64 {
        match self.native() {
            Some(value) => u64::from((128 - value.unsigned_abs().leading_zeros()).max(1)),
            None => self.as_big().bits().max(1),
        }
    }

    /// `digits(x)` from `quire.value.accounting/v1`: the base-ten magnitude
    /// digit count, where zero has one digit.
    pub fn decimal_digits(&self) -> u64 {
        length_amount(self.as_big().magnitude().to_str_radix(10).len())
    }

    /// The value as a `u64`, if it is one.
    pub fn to_u64(&self) -> Option<u64> {
        match self.native() {
            Some(value) => u64::try_from(value).ok(),
            None => u64::try_from(self.as_big()).ok(),
        }
    }

    pub(crate) fn magnitude_limbs(&self) -> MagnitudeLimbs<'_> {
        let magnitude = match &self.0 {
            IntegerRepr::Small64 { value, .. } => i128::from(*value).unsigned_abs(),
            IntegerRepr::Small128 { value, .. } => value.unsigned_abs(),
            IntegerRepr::Large(value) => {
                return MagnitudeLimbs(MagnitudeLimbsSource::Large(value.iter_u32_digits()));
            }
            IntegerRepr::LargeDigits { limbs, .. } => {
                return MagnitudeLimbs(MagnitudeLimbsSource::Copied(limbs.iter()));
            }
        };
        let bits = 128 - magnitude.leading_zeros();
        let remaining = u8::try_from(bits.div_ceil(32)).unwrap_or(0);
        MagnitudeLimbs(MagnitudeLimbsSource::Small {
            magnitude,
            remaining,
        })
    }

    /// The magnitude `|self|`.
    pub fn abs(&self) -> Self {
        match self.native() {
            Some(value) => value
                .checked_abs()
                .map(Self::small128)
                .unwrap_or_else(|| Self::from_big_normalized(self.as_big().abs())),
            None => Self::from_big_normalized(self.as_big().abs()),
        }
    }

    /// `self^|exponent|`. Callers bound the result size before calling.
    pub fn pow(&self, exponent: &Self) -> Self {
        Self::from_big_normalized(num_traits::Pow::pow(
            self.as_big(),
            exponent.as_big().magnitude(),
        ))
    }

    /// `(self / 2^k, k)` for the greatest `k <= limit` with `2^k | self`.
    /// Zero has no greatest such `k` and is returned with `k = 0`.
    pub fn split_factor_two(&self, limit: u64) -> (Self, u64) {
        match self.as_big().trailing_zeros() {
            None => (self.clone(), 0),
            Some(zeros) => {
                let shift = zeros.min(limit);
                (Self::from_big_normalized(self.as_big() >> shift), shift)
            }
        }
    }

    /// `self × 2^shift`. Callers bound the result size before calling.
    pub fn shifted_left(&self, shift: u64) -> Self {
        Self::from_big_normalized(self.as_big() << shift)
    }

    /// Whether this integer is even.
    pub fn is_even(&self) -> bool {
        self.native()
            .map(|value| value & 1 == 0)
            .unwrap_or_else(|| self.as_big().is_even())
    }

    /// `self + other`. Exact; unbounded `Integer` addition never overflows.
    pub fn add(&self, other: &Self) -> Self {
        if let (
            IntegerRepr::Small64 { value: left, .. },
            IntegerRepr::Small64 { value: right, .. },
        ) = (&self.0, &other.0)
        {
            return Self::small128(i128::from(*left) + i128::from(*right));
        }
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => {
                if let (Ok(left), Ok(right)) = (i64::try_from(left), i64::try_from(right)) {
                    return Self::small128(i128::from(left) + i128::from(right));
                }
                left.checked_add(right)
                    .map(Self::small128)
                    .unwrap_or_else(|| Self::from_big_normalized(self.as_big() + other.as_big()))
            }
            _ => Self::from_big_normalized(self.as_big() + other.as_big()),
        }
    }

    /// `self - other`. Exact; unbounded `Integer` subtraction never
    /// overflows.
    pub fn sub(&self, other: &Self) -> Self {
        if let (
            IntegerRepr::Small64 { value: left, .. },
            IntegerRepr::Small64 { value: right, .. },
        ) = (&self.0, &other.0)
        {
            return Self::small128(i128::from(*left) - i128::from(*right));
        }
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => {
                if let (Ok(left), Ok(right)) = (i64::try_from(left), i64::try_from(right)) {
                    return Self::small128(i128::from(left) - i128::from(right));
                }
                left.checked_sub(right)
                    .map(Self::small128)
                    .unwrap_or_else(|| Self::from_big_normalized(self.as_big() - other.as_big()))
            }
            _ => Self::from_big_normalized(self.as_big() - other.as_big()),
        }
    }

    /// `self × other`. Exact; unbounded `Integer` multiplication never
    /// overflows.
    pub fn mul(&self, other: &Self) -> Self {
        if let (
            IntegerRepr::Small64 { value: left, .. },
            IntegerRepr::Small64 { value: right, .. },
        ) = (&self.0, &other.0)
        {
            return Self::small128(i128::from(*left) * i128::from(*right));
        }
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => {
                if let (Ok(left), Ok(right)) = (i64::try_from(left), i64::try_from(right)) {
                    return Self::small128(i128::from(left) * i128::from(right));
                }
                left.checked_mul(right)
                    .map(Self::small128)
                    .unwrap_or_else(|| Self::from_big_normalized(self.as_big() * other.as_big()))
            }
            _ => Self::from_big_normalized(self.as_big() * other.as_big()),
        }
    }

    /// `-self`.
    pub fn neg(&self) -> Self {
        if let IntegerRepr::Small64 { value, .. } = &self.0 {
            return Self::small128(-i128::from(*value));
        }
        match self.native() {
            Some(value) => {
                if let Ok(value) = i64::try_from(value) {
                    return Self::small128(-i128::from(value));
                }
                value
                    .checked_neg()
                    .map(Self::small128)
                    .unwrap_or_else(|| Self::from_big_normalized(-self.as_big()))
            }
            None => Self::from_big_normalized(-self.as_big()),
        }
    }

    /// The greatest common divisor of `self` and `other`, non-negative.
    pub fn gcd(&self, other: &Self) -> Self {
        Self::from_big_normalized(self.as_big().gcd(other.as_big()))
    }

    /// Quotient rounded toward zero with an admitted nonzero divisor.
    /// Intended for division known to be exact; divisibility is not checked.
    pub fn exact_div(&self, divisor: NonZeroInteger<'_>) -> Self {
        Self::from_big_normalized(self.as_big() / divisor.get().as_big())
    }

    /// Truncating quotient/remainder; `divisor` is nonzero.
    pub fn div_rem_truncating(&self, divisor: NonZeroInteger<'_>) -> (Self, Self) {
        let (quotient, remainder) = self.as_big().div_rem(divisor.get().as_big());
        (
            Self::from_big_normalized(quotient),
            Self::from_big_normalized(remainder),
        )
    }

    /// Floor quotient/remainder; `divisor` is nonzero.
    pub fn div_mod_floor(&self, divisor: NonZeroInteger<'_>) -> (Self, Self) {
        let (quotient, remainder) = self.as_big().div_mod_floor(divisor.get().as_big());
        (
            Self::from_big_normalized(quotient),
            Self::from_big_normalized(remainder),
        )
    }

    /// Exact `10^exponent`.
    pub fn power_of_ten(exponent: u64) -> Self {
        let mut result = BigInt::one();
        let mut base = BigInt::from(10_u8);
        let mut remaining = exponent;
        while remaining > 0 {
            if remaining & 1 == 1 {
                result *= &base;
            }
            remaining >>= 1;
            if remaining > 0 {
                base = &base * &base;
            }
        }
        Self::from_big_normalized(result)
    }

    /// `bits(|factor| × |base|^exponent)` for a nonnegative `exponent`, derived
    /// without materializing the power.
    ///
    /// A power-of-two base is exact by shifting. Otherwise the power is bracketed
    /// by truncated lower and upper `P`-bit mantissas under a shared binary
    /// exponent, and `P` doubles until both brackets have one bit length. The
    /// product is then not a power of two, so a finite precision separates it
    /// from the nearest power of two and the loop terminates.
    pub fn power_product_bits(factor: &Self, base: &Self, exponent: &Self) -> Self {
        let factor = factor.as_big().magnitude();
        let base = base.as_big().magnitude();
        let exponent = exponent.as_big().magnitude();
        let factor_bits = BigUint::from(factor.bits().max(1));
        if factor.is_zero() || base.is_zero() && !exponent.is_zero() {
            return Self::one();
        }
        if exponent.is_zero() || base.is_one() {
            return Self::from_big_normalized(BigInt::from(factor_bits));
        }
        if base.count_ones() == 1 {
            let shift = BigUint::from(base.bits() - 1);
            return Self::from_big_normalized(BigInt::from(factor_bits + shift * exponent));
        }
        let mut precision = 64_u64;
        loop {
            let (low, high) = bracket_bits(factor, base, exponent, precision);
            if low == high {
                return Self::from_big_normalized(BigInt::from(low));
            }
            precision = precision.saturating_mul(2);
        }
    }

    /// `2^exponent`.
    fn power_of_two(exponent: u32) -> Self {
        Self::from_big_normalized(BigInt::one() << exponent)
    }
}

/// Bit lengths of a lower and upper bound of `factor × base^exponent`, each kept
/// to at most `precision` mantissa bits.
fn bracket_bits(
    factor: &BigUint,
    base: &BigUint,
    exponent: &BigUint,
    precision: u64,
) -> (BigUint, BigUint) {
    let (low, high, shift) = bracket(factor, base, exponent, precision);
    (
        BigUint::from(low.bits()) + &shift,
        BigUint::from(high.bits()) + shift,
    )
}

/// `(low, high, shift)` with `low × 2^shift <= factor × base^exponent <=
/// high × 2^shift`, the power's mantissas kept to at most `precision` bits.
/// With no truncation `low == high` is the exact value and `shift` is zero.
fn bracket(
    factor: &BigUint,
    base: &BigUint,
    exponent: &BigUint,
    precision: u64,
) -> (BigUint, BigUint, BigUint) {
    let mut low = BigUint::one();
    let mut high = BigUint::one();
    let mut shift = BigUint::zero();
    let truncate = |low: &mut BigUint, high: &mut BigUint, shift: &mut BigUint| {
        let excess = high.bits().saturating_sub(precision);
        if excess > 0 {
            *low >>= excess;
            *high = (&*high + ((BigUint::one() << excess) - 1_u8)) >> excess;
            *shift += excess;
        }
    };
    for bit in (0..exponent.bits()).rev() {
        low = &low * &low;
        high = &high * &high;
        shift = &shift << 1_u8;
        truncate(&mut low, &mut high, &mut shift);
        if exponent.bit(bit) {
            low *= base;
            high *= base;
            truncate(&mut low, &mut high, &mut shift);
        }
    }
    low *= factor;
    high *= factor;
    (low, high, shift)
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self::small64(value)
    }
}

impl From<i128> for Integer {
    fn from(value: i128) -> Self {
        Self::small128(value)
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        match i128::try_from(value) {
            Ok(value) => Self::small128(value),
            Err(_) => Self::from_big_normalized(BigInt::from(value)),
        }
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Self::small128(i128::from(value))
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.native() {
            Some(value) => fmt::Display::fmt(&value, formatter),
            None => fmt::Display::fmt(self.as_big(), formatter),
        }
    }
}

/// The canonical integer spelling `^(0|-?[1-9][0-9]*)$` was not supplied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("non-canonical integer spelling")]
pub struct NonCanonicalInteger;

impl FromStr for Integer {
    type Err = NonCanonicalInteger;

    /// Parse the canonical wire spelling used by complete-V1 schemas.
    fn from_str(spelling: &str) -> Result<Self, Self::Err> {
        let digits = spelling.strip_prefix('-').unwrap_or(spelling);
        let canonical = match digits.as_bytes() {
            [] => false,
            [b'0'] => digits.len() == spelling.len(),
            [first, rest @ ..] => {
                (b'1'..=b'9').contains(first) && rest.iter().all(u8::is_ascii_digit)
            }
        };
        if !canonical {
            return Err(NonCanonicalInteger);
        }
        if let Ok(value) = spelling.parse::<i128>() {
            return Ok(Self::small128(value));
        }
        BigInt::from_str(spelling)
            .map(Self::from_big_normalized)
            .map_err(|_| NonCanonicalInteger)
    }
}

/// A nonempty inclusive integer domain `[lower, upper]`.
///
/// This is the finite domain descriptor consumed by bounded backends; it is
/// never inferred from a host integer width.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntegerInterval {
    lower: Integer,
    upper: Integer,
}

/// An interval's lower bound exceeds its upper bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("empty integer interval")]
pub struct EmptyInterval;

impl IntegerInterval {
    /// Construct `[lower, upper]`, refusing an empty interval.
    pub fn new(lower: Integer, upper: Integer) -> Result<Self, EmptyInterval> {
        if lower > upper {
            return Err(EmptyInterval);
        }
        Ok(Self { lower, upper })
    }

    /// Retain the ordered endpoints certified by the native interval constructor.
    pub(crate) fn from_native_interval(
        interval: crate::native_integer::NativeIntegerInterval,
    ) -> Self {
        Self {
            lower: Integer::from(interval.lower()),
            upper: Integer::from(interval.upper()),
        }
    }

    /// The smallest interval containing both `a` and `b`, in either order.
    pub fn spanning(a: Integer, b: Integer) -> Self {
        if a <= b {
            Self { lower: a, upper: b }
        } else {
            Self { lower: b, upper: a }
        }
    }

    /// The two's-complement signed domain of `width` bits,
    /// `[-(2^(width-1)), 2^(width-1) - 1]`.
    pub fn signed_twos_complement(width: NonZeroU32) -> Self {
        let magnitude = Integer::power_of_two(width.get() - 1);
        Self {
            lower: magnitude.neg(),
            upper: magnitude.sub(&Integer::one()),
        }
    }

    /// Inclusive lower bound.
    pub fn lower(&self) -> &Integer {
        &self.lower
    }

    /// Inclusive upper bound.
    pub fn upper(&self) -> &Integer {
        &self.upper
    }

    /// Whether `value` is a member.
    pub fn contains(&self, value: &Integer) -> bool {
        &self.lower <= value && value <= &self.upper
    }

    /// Admit `value` into this domain without narrowing or saturation.
    pub fn admit(&self, value: Integer) -> Result<BoundedInteger, OutOfDomain> {
        if self.contains(&value) {
            Ok(BoundedInteger {
                value,
                domain: self.clone(),
            })
        } else {
            Err(OutOfDomain)
        }
    }
}

/// A value is outside its declared domain. The value is not retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("integer outside declared domain")]
pub struct OutOfDomain;

/// An integer admitted into an explicit inclusive domain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedInteger {
    value: Integer,
    domain: IntegerInterval,
}

impl BoundedInteger {
    /// The admitted mathematical value.
    pub fn value(&self) -> &Integer {
        &self.value
    }

    /// The domain that admitted it.
    pub fn domain(&self) -> &IntegerInterval {
        &self.domain
    }
}

/// The domain of an integer consumer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegerDomain {
    /// Unbounded mathematical integers; evaluation has no overflow.
    Mathematical,
    /// A finite consumer that must prove membership of every exposed result.
    Bounded(IntegerInterval),
}

impl IntegerDomain {
    /// Whether `value` belongs to this domain.
    pub fn contains(&self, value: &Integer) -> bool {
        match self {
            Self::Mathematical => true,
            Self::Bounded(interval) => interval.contains(value),
        }
    }
}

impl Integer {
    /// Wrap an arbitrary-precision integer (IEEE exact conversions).
    pub fn from_big(value: BigInt) -> Self {
        Self::from_big_normalized(value)
    }

    /// The arbitrary-precision integer (IEEE exact conversions).
    ///
    /// For an inline value, this initializes and retains its immutable `BigInt`
    /// image on first access. That may allocate; later borrows return the same
    /// object for the lifetime of `self`.
    pub fn as_big(&self) -> &BigInt {
        match &self.0 {
            IntegerRepr::Small64 { value, cache } => {
                cache.get_or_init(|| alloc::boxed::Box::new(BigInt::from(*value)))
            }
            IntegerRepr::Small128 { value, cache } => {
                cache.get_or_init(|| alloc::boxed::Box::new(BigInt::from(*value)))
            }
            IntegerRepr::Large(value) => value,
            IntegerRepr::LargeDigits { sign, limbs, cache } => {
                cache.get_or_init(|| alloc::boxed::Box::new(BigInt::new(*sign, limbs.clone())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use core::str::FromStr;

    use ix_trace_rs::trace;

    use super::*;

    fn assert_small128(value: &Integer) {
        assert!(matches!(&value.0, IntegerRepr::Small128 { .. }));
    }

    fn assert_empty_inline_cache(value: &Integer) {
        match &value.0 {
            IntegerRepr::Small64 { cache, .. } | IntegerRepr::Small128 { cache, .. } => {
                assert!(cache.get().is_none());
            }
            IntegerRepr::Large(_) => panic!("expected an inline value"),
        }
    }

    #[test]
    #[trace("FR-371-AC-7")]
    fn genuine_large_construction_and_results_stay_outside_i128() {
        let above = BigInt::from(i128::MAX) + 1_u8;
        let below = BigInt::from(i128::MIN) - 1_u8;
        let huge = BigInt::from(1_u8) << 200_u32;
        for expected in [above, below, huge.clone(), -huge] {
            for actual in [
                Integer::from_big(expected.clone()),
                Integer::from_str(&expected.to_string()).unwrap(),
            ] {
                assert!(matches!(&actual.0, IntegerRepr::Large(_)));
                assert!(i128::try_from(actual.as_big()).is_err());
                assert_eq!(actual.as_big(), &expected);
                let cloned = actual.clone();
                assert!(matches!(&cloned.0, IntegerRepr::Large(_)));
                assert_eq!(cloned.as_big(), &expected);
            }
        }
        let min = Integer::from(i128::MIN);
        let max = Integer::from(i128::MAX);
        for (actual, expected) in [
            (
                max.add(&Integer::one()),
                "170141183460469231731687303715884105728",
            ),
            (
                min.sub(&Integer::one()),
                "-170141183460469231731687303715884105729",
            ),
            (min.neg(), "170141183460469231731687303715884105728"),
            (
                min.mul(&Integer::from(-1_i64)),
                "170141183460469231731687303715884105728",
            ),
        ] {
            assert!(matches!(&actual.0, IntegerRepr::Large(_)));
            assert!(i128::try_from(actual.as_big()).is_err());
            assert_eq!(actual.to_string(), expected);
        }
        let large = Integer::from_big(BigInt::from(1_u8) << 200_u32);
        let fitting = large.add(&Integer::from(42_i64)).sub(&large);
        assert_small128(&fitting);
        assert_eq!(fitting.as_big(), &BigInt::from(42_i64));
    }

    #[test]
    #[trace("FR-384-AC-1", "FR-384-AC-4", "FR-371-AC-2")]
    fn native_interval_egress_preserves_exact_endpoints_and_stable_borrows() {
        let mut endpoints = std::vec![
            i128::MIN,
            i128::MIN + 1,
            i64::MIN.into(),
            -1,
            0,
            1,
            i64::MAX.into(),
            i128::MAX - 1,
            i128::MAX,
        ];
        let mut state = 0x7053_8400_0000_0001_u128;
        for _ in 0..64 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            endpoints.push(i128::from_le_bytes(state.to_le_bytes()));
        }
        for &lower in &endpoints {
            for &upper in &endpoints {
                let Ok(native) = crate::NativeIntegerInterval::new(lower, upper) else {
                    continue;
                };
                let actual = IntegerInterval::from_native_interval(native);
                assert_eq!(actual.lower().native(), Some(lower));
                assert_eq!(actual.upper().native(), Some(upper));
                for (value, expected) in [(actual.lower(), lower), (actual.upper(), upper)] {
                    assert_small128(value);
                    assert_empty_inline_cache(value);
                    let borrowed = value.as_big();
                    assert_eq!(borrowed, &BigInt::from(expected));
                    assert!(core::ptr::eq(borrowed, value.as_big()));
                    let cloned = value.clone();
                    assert_empty_inline_cache(&cloned);
                    assert_eq!(cloned.as_big(), borrowed);
                }
                assert_eq!(
                    actual,
                    IntegerInterval::spanning(Integer::from(lower), Integer::from(upper))
                );
            }
        }
    }

    #[test]
    #[trace("FR-371-AC-2", "FR-371-AC-7")]
    fn mixed_comparisons_preserve_empty_inline_caches_and_later_borrows() {
        let positive = Integer::from_big(BigInt::from(1_u8) << 200_u32);
        let negative = positive.neg();
        for inline in [
            Integer::from(i64::MIN),
            Integer::from(i64::MAX),
            Integer::from(i128::MIN),
            Integer::from(i128::MAX),
        ] {
            assert_empty_inline_cache(&inline);
            assert!(negative < inline);
            assert!(inline < positive);
            assert_ne!(inline, negative);
            assert_ne!(positive, inline);
            assert_empty_inline_cache(&inline);
            let borrowed = inline.as_big();
            let expected = borrowed.clone();
            assert!(negative < inline);
            assert!(inline < positive);
            assert_eq!(borrowed, &expected);
            assert!(core::ptr::eq(borrowed, inline.as_big()));
            let cloned = inline.clone();
            assert_empty_inline_cache(&cloned);
            assert_eq!(cloned.as_big(), borrowed);
        }
    }

    #[test]
    #[trace("FR-371-AC-7")]
    fn constructor_routes_preserve_the_selected_storage_class() {
        for value in [i64::MIN, -1, 0, 1, i64::MAX] {
            let integer = Integer::from(value);
            assert!(
                matches!(&integer.0, IntegerRepr::Small64 { value: stored, .. } if *stored == value)
            );
            assert_empty_inline_cache(&integer);
            assert_eq!(integer.as_big(), &BigInt::from(value));
        }
        for value in [
            i128::MIN,
            i128::from(i64::MIN) - 1,
            -1,
            0,
            1,
            i128::from(i64::MAX) + 1,
            i128::MAX,
        ] {
            for integer in [
                Integer::from(value),
                Integer::from_big(BigInt::from(value)),
                Integer::from_str(&value.to_string()).unwrap(),
            ] {
                assert_small128(&integer);
                assert_empty_inline_cache(&integer);
                assert_eq!(integer.as_big(), &BigInt::from(value));
            }
        }
        for value in [0_u64, 1, u64::MAX] {
            let integer = Integer::from(value);
            assert_small128(&integer);
            assert_empty_inline_cache(&integer);
            assert_eq!(integer.as_big(), &BigInt::from(value));
        }
        for value in [0_usize, 1, usize::MAX] {
            let integer = Integer::from(value);
            // The existing platform-sized conversion preserves non-fitting
            // continuation rather than assuming a pointer width.
            if i128::try_from(value).is_ok() {
                assert_small128(&integer);
                assert_empty_inline_cache(&integer);
            } else {
                assert!(matches!(&integer.0, IntegerRepr::Large(_)));
            }
            assert_eq!(integer.as_big(), &BigInt::from(value));
        }
        for (integer, expected) in [
            (Integer::zero(), 0),
            (Integer::one(), 1),
            (Integer::default(), 0),
        ] {
            assert!(matches!(&integer.0, IntegerRepr::Small64 { value, .. } if *value == expected));
            assert_empty_inline_cache(&integer);
        }
        for value in [
            BigInt::from(1_u8) << 200_u32,
            -(BigInt::from(1_u8) << 200_u32),
        ] {
            for integer in [
                Integer::from_big(value.clone()),
                Integer::from_str(&value.to_string()).unwrap(),
            ] {
                assert!(matches!(&integer.0, IntegerRepr::Large(_)));
                assert_eq!(integer.as_big(), &value);
                let cloned = integer.clone();
                assert!(matches!(&cloned.0, IntegerRepr::Large(_)));
                assert_eq!(cloned.as_big(), &value);
            }
        }
    }

    #[test]
    #[trace("FR-371-AC-7")]
    fn storage_pair_transitions_keep_fitting_results_wide() {
        for left in [i64::MIN, -1, 0, 1, i64::MAX] {
            for right in [i64::MIN, -1, 0, 1, i64::MAX] {
                for a in [Integer::from(left), Integer::from(i128::from(left))] {
                    for b in [Integer::from(right), Integer::from(i128::from(right))] {
                        for (actual, expected) in [
                            (a.add(&b), BigInt::from(left) + BigInt::from(right)),
                            (a.sub(&b), BigInt::from(left) - BigInt::from(right)),
                            (a.mul(&b), BigInt::from(left) * BigInt::from(right)),
                            (a.neg(), -BigInt::from(left)),
                        ] {
                            assert_small128(&actual);
                            assert_empty_inline_cache(&actual);
                            assert_eq!(actual.as_big(), &expected);
                        }
                    }
                }
            }
        }
        for outside in [i128::from(i64::MIN) - 1, i128::from(i64::MAX) + 1] {
            let wide = Integer::from(outside);
            let narrow = Integer::from(1_i64);
            for actual in [
                wide.add(&narrow),
                narrow.add(&wide),
                wide.sub(&narrow),
                narrow.sub(&wide),
                wide.mul(&narrow),
                narrow.mul(&wide),
                wide.neg(),
            ] {
                assert_small128(&actual);
            }
        }
        let zero = Integer::zero().add(&Integer::zero());
        assert_small128(&zero);
        assert_small128(&zero.add(&Integer::one()));
        assert_eq!(zero.add(&Integer::one()), Integer::one());
        let min = Integer::from(i128::MIN);
        let max = Integer::from(i128::MAX);
        for (actual, expected) in [
            (
                max.add(&Integer::one()),
                "170141183460469231731687303715884105728",
            ),
            (
                min.sub(&Integer::one()),
                "-170141183460469231731687303715884105729",
            ),
            (min.neg(), "170141183460469231731687303715884105728"),
            (
                min.mul(&Integer::from(-1_i64)),
                "170141183460469231731687303715884105728",
            ),
        ] {
            assert!(matches!(&actual.0, IntegerRepr::Large(_)));
            assert_eq!(actual.to_string(), expected);
        }
        let large = Integer::from_big(BigInt::from(1_u8) << 200_u32);
        let cancellation = large.add(&Integer::from(42_i64)).sub(&large);
        assert_small128(&cancellation);
        assert_eq!(cancellation, Integer::from(42_i64));
        let zero = large.sub(&large);
        assert_small128(&zero);
        assert!(zero.is_zero());
    }

    #[test]
    #[trace("FR-371-AC-2", "FR-371-AC-5", "FR-371-AC-7")]
    fn both_inline_classes_retain_borrows_and_reset_only_clone_caches() {
        for original in [Integer::from(42_i64), Integer::from(42_i128)] {
            assert_empty_inline_cache(&original);
            let start = std::sync::Barrier::new(4);
            std::thread::scope(|scope| {
                let workers = (0..4)
                    .map(|_| {
                        let original = &original;
                        let start = &start;
                        scope.spawn(move || {
                            start.wait();
                            let borrowed = original.as_big();
                            assert_eq!(borrowed, &BigInt::from(42_i64));
                            borrowed as *const BigInt as usize
                        })
                    })
                    .collect::<alloc::vec::Vec<_>>();
                let addresses = workers
                    .into_iter()
                    .map(|worker| worker.join().unwrap())
                    .collect::<alloc::vec::Vec<_>>();
                assert!(addresses.iter().all(|address| *address == addresses[0]));
            });
            let borrowed = original.as_big();
            let mut cloned = None;
            let allocations = allocation_counter::measure(|| {
                cloned = Some(original.clone());
            });
            assert_eq!(allocations.count_total, 0);
            assert_eq!(allocations.bytes_total, 0);
            let cloned = cloned.unwrap();
            assert_eq!(
                core::mem::discriminant(&cloned.0),
                core::mem::discriminant(&original.0)
            );
            assert_empty_inline_cache(&cloned);
            assert_eq!(cloned.as_big(), borrowed);
            drop(cloned);
            assert_eq!(borrowed, &BigInt::from(42_i64));
            assert!(core::ptr::eq(borrowed, original.as_big()));
        }
    }

    /// Trace: FR-374-AC-5
    #[test]
    #[should_panic(expected = "canonical divisor must be nonzero")]
    fn canonical_nonzero_premise_rejects_zero_with_its_own_message() {
        NonZeroInteger::from_proven_nonzero(&Integer::zero());
    }

    /// Trace: FR-360-AC-1
    #[test]
    fn minimum_i64_absolute_value_is_positive_two_to_the_63rd() {
        let input = Integer::from(i64::MIN);
        let magnitude = input.abs();
        assert_eq!(magnitude, Integer::from_str("9223372036854775808").unwrap());
        assert!(!magnitude.is_negative());
        assert_ne!(magnitude, input);
        assert!(input.is_negative());
    }

    /// `IntegerInterval::admit` accepts a value inside the declared
    /// domain, and `BoundedInteger` reports back the exact value and domain
    /// that admitted it.
    #[test]
    fn bounded_integer_admits_within_domain() {
        let domain = IntegerInterval::new(Integer::zero(), Integer::from(10_u64)).unwrap();
        let admitted = domain.admit(Integer::from(7_u64)).unwrap();
        assert_eq!(admitted.value(), &Integer::from(7_u64));
        assert_eq!(admitted.domain(), &domain);
    }

    /// a value outside the declared domain is refused with
    /// `OutOfDomain`, never silently narrowed or saturated.
    #[test]
    fn bounded_integer_refuses_outside_domain() {
        let domain = IntegerInterval::new(Integer::zero(), Integer::from(10_u64)).unwrap();
        assert_eq!(domain.admit(Integer::from(11_u64)), Err(OutOfDomain));
    }

    /// parsing round-trips through `Display` for a canonical
    /// decimal integer literal, negative sign included.
    #[test]
    fn integer_parse_display_round_trips() {
        let parsed = Integer::from_str("-42").unwrap();
        assert_eq!(parsed.to_string(), "-42");
    }

    /// Trace: FR-371-AC-1, FR-371-AC-4
    #[test]
    fn native_arithmetic_stays_inline_and_overflow_refines_exactly() {
        let zero = Integer::zero();
        let min = Integer::from(i128::MIN);
        let max = Integer::from(i128::MAX);
        assert_eq!(zero.add(&Integer::one()), Integer::one());
        assert_eq!(
            max.add(&Integer::one()).to_string(),
            "170141183460469231731687303715884105728"
        );
        assert_eq!(
            min.sub(&Integer::one()).to_string(),
            "-170141183460469231731687303715884105729"
        );
        assert_eq!(
            min.neg().to_string(),
            "170141183460469231731687303715884105728"
        );
        assert_eq!(
            Integer::from(i64::MIN)
                .mul(&Integer::from(i64::MIN))
                .to_string(),
            "85070591730234615865843651857942052864"
        );
        assert_eq!(min.magnitude_bits(), 128);
        assert_eq!(
            min.magnitude_limbs().collect::<alloc::vec::Vec<_>>(),
            [0, 0, 0, 0x8000_0000]
        );
        assert!(matches!(&zero.0, IntegerRepr::Small64 { value: 0, .. }));
        assert!(matches!(
            &zero.add(&Integer::one()).0,
            IntegerRepr::Small128 { value: 1, .. }
        ));
    }

    /// Trace: FR-371-AC-2, FR-371-AC-5
    #[test]
    fn cache_is_invisible_to_traits_clones_and_concurrent_borrows() {
        fn assert_send_sync<T: Send + Sync>() {}
        fn assert_unwind_safe<T: core::panic::UnwindSafe + core::panic::RefUnwindSafe>() {}
        assert_send_sync::<Integer>();
        assert_unwind_safe::<Integer>();

        let value = Integer::from(42_i128);
        let parsed = Integer::from_str("42").unwrap();
        let from_big = Integer::from_big(BigInt::from(42));
        let arithmetic = Integer::from(40_i128).add(&Integer::from(2_i128));
        let narrow = Integer::from(42_i64);
        assert_eq!(value, narrow);
        assert_eq!(value, parsed);
        assert_eq!(value, from_big);
        assert_eq!(value, arithmetic);
        assert_eq!(
            value.cmp(&Integer::from(43_i128)),
            core::cmp::Ordering::Less
        );
        assert_eq!(value.to_string(), "42");
        assert_eq!(alloc::format!("{value:?}"), "Integer(42)");

        let retained = value.as_big();
        assert_eq!(retained, &BigInt::from(42));
        assert!(core::ptr::eq(retained, value.as_big()));
        let clone = value.clone();
        assert!(matches!(&clone.0, IntegerRepr::Small128 { cache, .. } if cache.get().is_none()));

        std::thread::scope(|scope| {
            let first = scope.spawn(|| value.as_big() as *const BigInt as usize);
            let second = scope.spawn(|| value.as_big() as *const BigInt as usize);
            assert_eq!(first.join().unwrap(), retained as *const BigInt as usize);
            assert_eq!(second.join().unwrap(), retained as *const BigInt as usize);
        });

        let mut left = std::collections::hash_map::DefaultHasher::new();
        let mut right = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut left);
        arithmetic.hash(&mut right);
        assert_eq!(left.finish(), right.finish());
    }

    /// Trace: FR-371-AC-5
    #[test]
    fn once_box_race_retains_one_candidate_and_releases_losers() {
        const CANDIDATES: usize = 8;
        let cache = once_cell::race::OnceBox::<BigInt>::new();
        let barrier = std::sync::Barrier::new(CANDIDATES);
        let initialized = std::sync::atomic::AtomicUsize::new(0);

        std::thread::scope(|scope| {
            let readers = (0..CANDIDATES)
                .map(|_| {
                    let barrier = &barrier;
                    let cache = &cache;
                    let initialized = &initialized;
                    scope.spawn(move || {
                        let mut address = 0;
                        let allocations = allocation_counter::measure(|| {
                            address = cache.get_or_init(|| {
                                initialized.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                                barrier.wait();
                                alloc::boxed::Box::new(BigInt::from(1_234_567_i128))
                            }) as *const BigInt as usize;
                        });
                        (address, allocations)
                    })
                })
                .collect::<alloc::vec::Vec<_>>();
            let observations = readers
                .into_iter()
                .map(|reader| reader.join().unwrap())
                .collect::<alloc::vec::Vec<_>>();

            assert_eq!(
                initialized.load(core::sync::atomic::Ordering::Relaxed),
                CANDIDATES
            );
            let retained_address = observations[0].0;
            assert!(observations
                .iter()
                .all(|(address, _)| *address == retained_address));

            let workers_with_retained_allocations = observations
                .iter()
                .filter(|(_, allocation)| allocation.count_current > 0)
                .count();
            assert_eq!(workers_with_retained_allocations, 1);
            let total_allocations = observations
                .iter()
                .map(|(_, allocation)| allocation.count_total)
                .sum::<u64>();
            let retained_allocations = observations
                .iter()
                .map(|(_, allocation)| allocation.count_current.max(0) as u64)
                .sum::<u64>();
            assert!(total_allocations > retained_allocations);
        });

        assert_eq!(cache.get(), Some(&BigInt::from(1_234_567_i128)));
    }

    /// Trace: FR-371-AC-4
    #[test]
    #[trace("FR-371-AC-7")]
    fn magnitude_limb_stream_does_not_materialize_small_cache() {
        let values = [
            Integer::zero(),
            Integer::from(-1_i128),
            Integer::from(1_i128 << 32),
            Integer::from(i128::MIN),
            Integer::from(i128::MAX),
            Integer::from(i64::MIN),
            Integer::from(1_i64 << 32),
            Integer::from(0_i128),
        ];
        let expected = [
            [0, 0, 0, 0],
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0x8000_0000],
            [u32::MAX, u32::MAX, u32::MAX, 0x7fff_ffff],
            [0, 0x8000_0000, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0],
        ];
        let expected_lengths = [0, 1, 2, 4, 4, 2, 2, 0];
        let mut observed = [[0_u32; 4]; 8];
        let mut lengths = [0_usize; 8];
        let allocations = allocation_counter::measure(|| {
            for (index, value) in values.iter().enumerate() {
                for limb in value.magnitude_limbs() {
                    observed[index][lengths[index]] = limb;
                    lengths[index] += 1;
                }
            }
        });
        assert_eq!(allocations.bytes_total, 0);
        assert_eq!(allocations.count_total, 0);
        assert_eq!(lengths, expected_lengths);
        assert_eq!(observed, expected);
        assert!(values.iter().all(|value| matches!(
            &value.0,
            IntegerRepr::Small64 { cache, .. } | IntegerRepr::Small128 { cache, .. }
                if cache.get().is_none()
        )));
    }

    /// Trace: FR-390-AC-1, FR-390-AC-2, FR-390-AC-3, FR-390-AC-5
    #[test]
    #[trace("FR-390-AC-1", "FR-390-AC-2", "FR-390-AC-3", "FR-390-AC-5")]
    fn fallible_integer_copy_and_decimal_public_controls() {
        const POSITIVE: &str = "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
        const NEGATIVE: &str = "-98765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210";
        for spelling in ["0", "1", "-1", POSITIVE, NEGATIVE] {
            let source = Integer::from_str(spelling).unwrap();
            let copied = source.try_clone().unwrap();
            assert_eq!(copied, source);
            assert_eq!(
                copied.try_to_decimal_string(spelling.len()).unwrap(),
                spelling
            );
            assert_eq!(copied.try_clone().unwrap(), source);
            drop(copied);
            assert_eq!(source.to_string(), spelling);
            assert_eq!(
                source.try_to_decimal_string(spelling.len()).unwrap(),
                spelling
            );
            assert_eq!(source.to_string(), spelling);
            assert_eq!(
                source.try_to_decimal_string(spelling.len() - 1),
                Err(IntegerDecimalError::ByteLimit {
                    required: spelling.len(),
                    limit: spelling.len() - 1,
                })
            );
        }
        let small = Integer::from(77_i64);
        assert_eq!(
            small.try_clone().unwrap().add(&Integer::one()),
            Integer::from(78_i64)
        );
        assert_eq!(small, Integer::from(77_i64));
    }

    /// Trace: FR-390-AC-1, FR-390-AC-4
    #[test]
    #[trace("FR-390-AC-1", "FR-390-AC-4")]
    fn every_integer_reservation_reports_its_actual_additional_bytes() {
        let source = Integer::from_str("-98765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210").unwrap();
        let expected = source.to_string();
        let mut copy_requests = Vec::new();
        source
            .try_clone_with(&mut |requested| {
                copy_requests.push(requested);
                Ok(())
            })
            .unwrap();
        assert_eq!(copy_requests, [source.magnitude_limbs().count() * 4]);
        assert_eq!(
            source.try_clone_with(&mut |requested| {
                Err(IntegerCopyError::Allocation { requested })
            }),
            Err(IntegerCopyError::Allocation {
                requested: copy_requests[0]
            })
        );
        Integer::from(3_i64)
            .try_clone_with(&mut |_| panic!("inline copy must not reserve"))
            .unwrap();

        for spelling in [
            "0",
            "1",
            "-1",
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
            expected.as_str(),
        ] {
            let value = Integer::from_str(spelling).unwrap();
            let mut decimal_requests = Vec::new();
            value
                .try_to_decimal_string_with(spelling.len(), &mut |requested| {
                    decimal_requests.push(requested);
                    Ok(())
                })
                .unwrap();
            assert!(!decimal_requests.is_empty());
            assert!(decimal_requests[..decimal_requests.len() - 1]
                .iter()
                .all(|requested| *requested == core::mem::size_of::<u32>()));
            assert_eq!(decimal_requests.last(), Some(&spelling.len()));
            for (denied_index, expected_request) in decimal_requests.iter().enumerate() {
                let mut seen = 0;
                assert_eq!(
                    value.try_to_decimal_string_with(spelling.len(), &mut |requested| {
                        let index = seen;
                        seen += 1;
                        if index == denied_index {
                            Err(IntegerDecimalError::Allocation { requested })
                        } else {
                            Ok(())
                        }
                    }),
                    Err(IntegerDecimalError::Allocation {
                        requested: *expected_request
                    })
                );
                assert_eq!(value.to_string(), spelling);
            }
        }
    }

    /// Trace: FR-390-AC-3, FR-390-AC-4
    #[test]
    #[trace("FR-390-AC-3", "FR-390-AC-4")]
    fn checked_integer_reservation_and_output_length_overflow() {
        assert_eq!(reservation_bytes::<u32>(0, usize::MAX / 4), usize::MAX - 3);
        assert_eq!(reservation_bytes::<u32>(0, usize::MAX / 4 + 1), usize::MAX);
        assert_eq!(reservation_bytes::<u32>(usize::MAX, 1), usize::MAX);
        for limit in [7, usize::MAX] {
            assert_eq!(
                decimal_length(usize::MAX, 9, true, limit),
                Err(IntegerDecimalError::OutputSizeOverflow)
            );
        }
        assert_eq!(
            decimal_length(2, 3, true, 12),
            Err(IntegerDecimalError::ByteLimit {
                required: 13,
                limit: 12
            })
        );
    }
}
