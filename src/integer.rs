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

use core::fmt;
use core::hash::{Hash, Hasher};
use core::num::NonZeroU32;
use core::str::FromStr;

use num_bigint::{BigInt, BigUint};
use num_integer::Integer as _;
use num_traits::{One, Signed, Zero};

use crate::accounting::length_amount;

/// An exact, arbitrary-precision mathematical integer.
///
/// Values that fit `i128` are stored inline. The first [`as_big`](Self::as_big)
/// call lazily initializes one retained immutable `BigInt` image; clones of an
/// inline value begin with an empty image cache. The safe
/// `once_cell::race::OnceBox` may construct multiple candidates concurrently,
/// retains one, and drops every losing candidate. Values outside `i128` retain
/// their `BigInt` directly.
///
/// quire:canonical
pub struct Integer(IntegerRepr);

enum IntegerRepr {
    Small {
        value: i128,
        cache: once_cell::race::OnceBox<BigInt>,
    },
    Large(BigInt),
}

impl Integer {
    fn small(value: i128) -> Self {
        Self(IntegerRepr::Small {
            value,
            cache: once_cell::race::OnceBox::new(),
        })
    }

    fn from_big_normalized(value: BigInt) -> Self {
        match i128::try_from(&value) {
            Ok(value) => Self::small(value),
            Err(_) => Self(IntegerRepr::Large(value)),
        }
    }

    fn native(&self) -> Option<i128> {
        match &self.0 {
            IntegerRepr::Small { value, .. } => Some(*value),
            IntegerRepr::Large(_) => None,
        }
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self::small(0)
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        match &self.0 {
            IntegerRepr::Small { value, .. } => Self::small(*value),
            IntegerRepr::Large(value) => Self(IntegerRepr::Large(value.clone())),
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => left == right,
            _ => self.as_big() == other.as_big(),
        }
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
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => left.cmp(&right),
            _ => self.as_big().cmp(other.as_big()),
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
/// Plain integer borrows cannot be passed to any division helper:
///
/// ```compile_fail
/// use quire_exact::Integer;
/// let two = Integer::from(2_i64);
/// Integer::one().exact_div(&two);
/// ```
///
/// ```compile_fail
/// use quire_exact::Integer;
/// let two = Integer::from(2_i64);
/// Integer::one().div_rem_truncating(&two);
/// ```
///
/// ```compile_fail
/// use quire_exact::Integer;
/// let two = Integer::from(2_i64);
/// Integer::one().div_mod_floor(&two);
/// ```
///
/// The referent is private, immutable and cannot outlive its owner:
///
/// ```compile_fail
/// use quire_exact::{Integer, NonZeroInteger};
/// let zero = Integer::zero();
/// let token = NonZeroInteger(&zero);
/// ```
///
/// ```compile_fail
/// use quire_exact::{Integer, NonZeroInteger};
/// let two = Integer::from(2_i64);
/// let token = NonZeroInteger::try_from(&two).unwrap();
/// *token.get() = Integer::zero();
/// ```
///
/// ```compile_fail
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
/// ```compile_fail
/// use quire_exact::{Integer, NonZeroInteger};
/// let mut two = Integer::from(2_i64);
/// let token = NonZeroInteger::try_from(&two).unwrap();
/// two = Integer::zero();
/// let _ = token.get();
/// ```
///
/// Zero cannot be produced through a default or internal constructor:
///
/// ```compile_fail
/// use quire_exact::NonZeroInteger;
/// let token: NonZeroInteger<'_> = Default::default();
/// ```
///
/// ```compile_fail
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
        Self::small(0)
    }

    /// The integer one.
    pub fn one() -> Self {
        Self::small(1)
    }

    /// Whether this integer is zero.
    pub fn is_zero(&self) -> bool {
        match self.native() {
            Some(value) => value == 0,
            None => self.as_big().is_zero(),
        }
    }

    /// Whether this integer is strictly negative.
    pub fn is_negative(&self) -> bool {
        match self.native() {
            Some(value) => value < 0,
            None => self.as_big().is_negative(),
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
        match &self.0 {
            IntegerRepr::Small { value, .. } => {
                let magnitude = value.unsigned_abs();
                let bits = 128 - magnitude.leading_zeros();
                let remaining = u8::try_from(bits.div_ceil(32)).unwrap_or(0);
                MagnitudeLimbs(MagnitudeLimbsSource::Small {
                    magnitude,
                    remaining,
                })
            }
            IntegerRepr::Large(value) => {
                MagnitudeLimbs(MagnitudeLimbsSource::Large(value.iter_u32_digits()))
            }
        }
    }

    /// The magnitude `|self|`.
    pub fn abs(&self) -> Self {
        match self.native() {
            Some(value) => value
                .checked_abs()
                .map(Self::small)
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
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => left
                .checked_add(right)
                .map(Self::small)
                .unwrap_or_else(|| Self::from_big_normalized(self.as_big() + other.as_big())),
            _ => Self::from_big_normalized(self.as_big() + other.as_big()),
        }
    }

    /// `self - other`. Exact; unbounded `Integer` subtraction never
    /// overflows.
    pub fn sub(&self, other: &Self) -> Self {
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => left
                .checked_sub(right)
                .map(Self::small)
                .unwrap_or_else(|| Self::from_big_normalized(self.as_big() - other.as_big())),
            _ => Self::from_big_normalized(self.as_big() - other.as_big()),
        }
    }

    /// `self × other`. Exact; unbounded `Integer` multiplication never
    /// overflows.
    pub fn mul(&self, other: &Self) -> Self {
        match (self.native(), other.native()) {
            (Some(left), Some(right)) => left
                .checked_mul(right)
                .map(Self::small)
                .unwrap_or_else(|| Self::from_big_normalized(self.as_big() * other.as_big())),
            _ => Self::from_big_normalized(self.as_big() * other.as_big()),
        }
    }

    /// `-self`.
    pub fn neg(&self) -> Self {
        match self.native() {
            Some(value) => value
                .checked_neg()
                .map(Self::small)
                .unwrap_or_else(|| Self::from_big_normalized(-self.as_big())),
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
        Self::small(i128::from(value))
    }
}

impl From<i128> for Integer {
    fn from(value: i128) -> Self {
        Self::small(value)
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        match i128::try_from(value) {
            Ok(value) => Self::small(value),
            Err(_) => Self::from_big_normalized(BigInt::from(value)),
        }
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Self::small(i128::from(value))
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
            IntegerRepr::Small { value, cache } => {
                cache.get_or_init(|| alloc::boxed::Box::new(BigInt::from(*value)))
            }
            IntegerRepr::Large(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use core::str::FromStr;

    use super::*;

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
        assert!(matches!(&zero.0, IntegerRepr::Small { value: 0, .. }));
        assert!(matches!(
            &zero.add(&Integer::one()).0,
            IntegerRepr::Small { value: 1, .. }
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
        assert!(matches!(&clone.0, IntegerRepr::Small { cache, .. } if cache.get().is_none()));

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
    fn magnitude_limb_stream_does_not_materialize_small_cache() {
        let values = [
            Integer::zero(),
            Integer::from(-1_i128),
            Integer::from(1_i128 << 32),
            Integer::from(i128::MIN),
            Integer::from(i128::MAX),
        ];
        let expected = [
            [0, 0, 0, 0],
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0x8000_0000],
            [u32::MAX, u32::MAX, u32::MAX, 0x7fff_ffff],
        ];
        let expected_lengths = [0, 1, 2, 4, 4];
        let mut observed = [[0_u32; 4]; 5];
        let mut lengths = [0_usize; 5];
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
            IntegerRepr::Small { cache, .. } if cache.get().is_none()
        )));
    }
}
