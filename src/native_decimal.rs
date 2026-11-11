// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani-tractable native Decimal carrier (FR-402).
//!
//! A [`NativeDecimal`] is the owner's retained representation, a signed
//! 128-bit coefficient and a `u32` scale, held without normalization. A
//! [`NativeDecimalType`] holds the native-width bounds of a
//! `Decimal[lo, hi; smin, smax; mode]` declaration. The constructor
//! [`NativeDecimal::from_bindings`] and the normalized view
//! [`NativeDecimal::normalized`] use fixed-width arithmetic only: no
//! allocation, no loop, no iterator adapter, no owner `Integer`, `Decimal`
//! or `Rational`, and one body for host and `cfg(kani)` builds.
//!
//! The carried Decimal has no public fields, and no constructor takes an
//! owner value, a type or an unchecked pair of fields beyond the total
//! [`NativeDecimal::from_retained`]:
//!
//! ```
//! # use quire_exact::{NativeDecimal, NativeDecimalType, RoundingMode};
//! let ty = NativeDecimalType::new(0, 100, 0, 2, RoundingMode::Exact).unwrap();
//! assert!(NativeDecimal::from_bindings(&ty, 5000, 2).is_ok());
//! ```
//!
//! ```compile_fail,E0308
//! # use quire_exact::{Integer, NativeDecimal, NativeDecimalType, RoundingMode};
//! let ty = NativeDecimalType::new(0, 100, 0, 2, RoundingMode::Exact).unwrap();
//! let _ = NativeDecimal::from_bindings(&ty, Integer::from(5000_i64), 2);
//! ```
//!
//! ```compile_fail,E0308
//! # use quire_exact::{Decimal, Integer, NativeDecimal, NativeDecimalType, RoundingMode};
//! let ty = NativeDecimalType::new(0, 100, 0, 2, RoundingMode::Exact).unwrap();
//! let _ = NativeDecimal::from_bindings(&ty, Decimal::new(Integer::from(5000_i64), 2), 2);
//! ```
//!
//! ```compile_fail,E0308
//! # use quire_exact::{DecimalType, Integer, NativeDecimal, RoundingMode};
//! let owner =
//!     DecimalType::new(Integer::zero(), Integer::from(100_i64), 0, 2, RoundingMode::Exact)
//!         .unwrap();
//! let _ = NativeDecimal::from_bindings(&owner, 5000, 2);
//! ```
//!
//! ```compile_fail,E0451
//! # use quire_exact::NativeDecimal;
//! let _ = NativeDecimal { coefficient: 1, scale: 0 };
//! ```

use crate::comparison::{IllTyped, IllTypedCause};
use crate::decimal::{Decimal, DecimalType, RoundingMode};
use crate::integer::Integer;

/// The two refusals of the carrier (FR-402 Behavior 6).
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeDecimalRefusal {
    /// A bound of the owner type lies outside signed 128 bits; raised at
    /// ingress, before any binding exists.
    #[error("decimal type bound is not representable in the native width")]
    TypeNotRepresentable,
    /// The scale binding is outside `0..=4294967295` or the binding is not a
    /// member of the type by the owner's rule.
    #[error("decimal binding is outside the carrier domain")]
    OutsideCarrierDomain,
}

/// A `Decimal[lo, hi; smin, smax; mode]` type whose bounds fit signed 128
/// bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeDecimalType {
    lower: i128,
    upper: i128,
    min_scale: u32,
    max_scale: u32,
    rounding: RoundingMode,
}

impl NativeDecimalType {
    /// Check a declaration exactly as the owner's `DecimalType::new` does:
    /// `lower > upper`, `min_scale > max_scale` or `max_scale > u32::MAX` is
    /// `IllTyped` with cause `MalformedDecimalType`.
    pub fn new(
        lower: i128,
        upper: i128,
        min_scale: u64,
        max_scale: u64,
        rounding: RoundingMode,
    ) -> Result<Self, IllTyped> {
        let malformed = IllTyped {
            cause: IllTypedCause::MalformedDecimalType,
        };
        let max_scale = u32::try_from(max_scale).map_err(|_| malformed)?;
        let min_scale = u32::try_from(min_scale).map_err(|_| malformed)?;
        if lower > upper || min_scale > max_scale {
            return Err(malformed);
        }
        Ok(Self {
            lower,
            upper,
            min_scale,
            max_scale,
            rounding,
        })
    }

    /// Inclusive lower coefficient bound.
    pub const fn lower(&self) -> i128 {
        self.lower
    }

    /// Inclusive upper coefficient bound.
    pub const fn upper(&self) -> i128 {
        self.upper
    }

    /// Inclusive minimum membership scale.
    pub const fn min_scale(&self) -> u32 {
        self.min_scale
    }

    /// Inclusive maximum membership scale, also the target scale.
    pub const fn max_scale(&self) -> u32 {
        self.max_scale
    }

    /// The rounding spelling.
    pub const fn rounding(&self) -> RoundingMode {
        self.rounding
    }

    /// Whether the carried Decimal is a member of this type by the owner's
    /// rule. Membership is judged on the normalized pair, never on the
    /// retained one, and this is the only way for the rest of the crate to
    /// ask: the rule cannot be given a pair that has not been normalized.
    pub(crate) fn admits(&self, value: &NativeDecimal) -> bool {
        self.admits_normalized(value.normalized_pair())
    }

    /// The owner's membership rule on a normalized pair: with `s*` the larger
    /// of the scale and `min_scale`, the pair is a member exactly when `s*`
    /// is at most `max_scale` and `lower <= coefficient x 10^(s* - scale) <=
    /// upper`. A lift that leaves the native width is decided by sign alone
    /// (both bounds fit the width) and is never wrapped, saturated or
    /// materialized.
    fn admits_normalized(&self, pair: NormalizedPair) -> bool {
        let NormalizedPair { coefficient, scale } = pair;
        let lifted_scale = if scale > self.min_scale {
            scale
        } else {
            self.min_scale
        };
        if lifted_scale > self.max_scale {
            return false;
        }
        if coefficient == 0 {
            return self.lower <= 0 && 0 <= self.upper;
        }
        let shift = lifted_scale - scale;
        if shift > MAX_NATIVE_POWER {
            return false;
        }
        match coefficient.checked_mul(POWERS_OF_TEN[shift as usize]) {
            Some(lifted) => self.lower <= lifted && lifted <= self.upper,
            None => false,
        }
    }
}

/// A pair that [`NativeDecimal::normalized_pair`] produced: the only input of
/// the membership rule.
#[derive(Clone, Copy)]
pub(crate) struct NormalizedPair {
    coefficient: i128,
    scale: u32,
}

impl TryFrom<&DecimalType> for NativeDecimalType {
    type Error = NativeDecimalRefusal;

    /// Explicit ingress from the owner's type, outside the proof path. A
    /// bound outside signed 128 bits is `TypeNotRepresentable`.
    fn try_from(owner: &DecimalType) -> Result<Self, Self::Error> {
        let lower = owner
            .lower()
            .native()
            .ok_or(NativeDecimalRefusal::TypeNotRepresentable)?;
        let upper = owner
            .upper()
            .native()
            .ok_or(NativeDecimalRefusal::TypeNotRepresentable)?;
        Ok(Self {
            lower,
            upper,
            min_scale: owner.min_scale(),
            max_scale: owner.max_scale(),
            rounding: owner.rounding(),
        })
    }
}

/// A Decimal in its retained representation: the signed 128-bit coefficient
/// and `u32` scale as drawn, never normalized. Two values equal in value with
/// different scales are distinct, and equality is equality of the pair.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NativeDecimal {
    coefficient: i128,
    scale: u32,
}

impl NativeDecimal {
    /// Retain any pair exactly. This grants no membership of any type.
    pub const fn from_retained(coefficient: i128, scale: u32) -> Self {
        Self { coefficient, scale }
    }

    /// Build the carried Decimal of a type from the `i64` coefficient and
    /// `i64` scale bindings, retaining exactly the pair drawn. A scale outside
    /// `0..=4294967295` precedes, and a non-member follows, as
    /// `OutsideCarrierDomain`.
    pub fn from_bindings(
        ty: &NativeDecimalType,
        coefficient: i64,
        scale: i64,
    ) -> Result<Self, NativeDecimalRefusal> {
        let Ok(scale) = u32::try_from(scale) else {
            return Err(NativeDecimalRefusal::OutsideCarrierDomain);
        };
        let drawn = Self::from_retained(i128::from(coefficient), scale);
        if ty.admits(&drawn) {
            Ok(drawn)
        } else {
            Err(NativeDecimalRefusal::OutsideCarrierDomain)
        }
    }

    /// The retained coefficient.
    pub const fn coefficient(&self) -> i128 {
        self.coefficient
    }

    /// The retained scale.
    pub const fn scale(&self) -> u32 {
        self.scale
    }

    /// The least-scale pair equal in value: trailing decimal zeros removed
    /// while the scale is above zero, zero as `(0, 0)`.
    pub fn normalized(&self) -> (i128, u32) {
        let pair = self.normalized_pair();
        (pair.coefficient, pair.scale)
    }

    /// The normalized pair, branded as such for the membership rule.
    pub(crate) fn normalized_pair(&self) -> NormalizedPair {
        if self.coefficient == 0 {
            return NormalizedPair {
                coefficient: 0,
                scale: 0,
            };
        }
        let (coefficient, scale) = (self.coefficient, self.scale);
        let (coefficient, scale) = strip_zeros(coefficient, scale, 32, POWERS_OF_TEN[32]);
        let (coefficient, scale) = strip_zeros(coefficient, scale, 16, POWERS_OF_TEN[16]);
        let (coefficient, scale) = strip_zeros(coefficient, scale, 8, POWERS_OF_TEN[8]);
        let (coefficient, scale) = strip_zeros(coefficient, scale, 4, POWERS_OF_TEN[4]);
        let (coefficient, scale) = strip_zeros(coefficient, scale, 2, POWERS_OF_TEN[2]);
        let (coefficient, scale) = strip_zeros(coefficient, scale, 1, POWERS_OF_TEN[1]);
        NormalizedPair { coefficient, scale }
    }

    /// Explicit read-out into the owner's `Decimal`, retaining the pair
    /// without normalizing it. This allocates and is outside the proof path.
    pub fn to_decimal(&self) -> Decimal {
        Decimal::new(Integer::from(self.coefficient), self.scale)
    }
}

/// Remove `places` trailing zeros (`power` is `10^places`) when the scale
/// allows and the coefficient is divisible by `power`. Applied for 32, 16, 8,
/// 4, 2, 1 in turn, the removed counts are the binary digits of the total.
fn strip_zeros(coefficient: i128, scale: u32, places: u32, power: i128) -> (i128, u32) {
    if scale >= places && coefficient % power == 0 {
        (coefficient / power, scale - places)
    } else {
        (coefficient, scale)
    }
}

/// The largest `n` with `10^n` inside signed 128 bits.
const MAX_NATIVE_POWER: u32 = 38;

/// `10^0` through `10^38`, the powers inside signed 128 bits.
const POWERS_OF_TEN: [i128; 39] = {
    let mut table = [1_i128; 39];
    let mut index = 1;
    while index < 39 {
        table[index] = table[index - 1] * 10;
        index += 1;
    }
    table
};

#[cfg(kani)]
mod kani_proofs;
#[cfg(any(test, kani))]
mod reference;
#[cfg(test)]
mod tests;
