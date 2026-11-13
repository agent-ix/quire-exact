// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani-tractable carried-quantity constructor (FR-398).
//!
//! [`NativeQuantity::from_bindings`] builds the magnitude of a carried
//! quantity of a [`CarriedQuantityType`] from the physical bindings of the
//! type's carrier: one `i64` for `Int`, a coefficient then a scale for
//! `Decimal`, a numerator then a denominator for `Rational`. The result is a
//! canonical fraction of two `i128`, `Copy` and without a unit. The
//! constructor uses fixed-width arithmetic only: no allocation, no loop, no
//! iterator adapter, no owner `Integer`, `Decimal` or `Rational`, no `Meter`,
//! and one body for host and `cfg(kani)` builds.
//!
//! ```
//! # use quire_exact::{
//! #     CarriedQuantityType, Integer, IntegerInterval, NativeQuantity, NodeKey, PathSegment,
//! #     QuantityBindings, QuantityCarrier, RoundingMode, UnitId,
//! # };
//! # let key = NodeKey::new("test", vec![PathSegment::Text("cm".into())], 0).unwrap();
//! let ty = CarriedQuantityType::new(
//!     UnitId::declared(key),
//!     QuantityCarrier::Integer {
//!         domain: IntegerInterval::new(Integer::from(0_i64), Integer::from(100_i64)).unwrap(),
//!         rounding: RoundingMode::Exact,
//!     },
//! );
//! let quantity = NativeQuantity::from_bindings(&ty, QuantityBindings::Int(50)).unwrap();
//! assert_eq!((quantity.numerator(), quantity.denominator()), (50, 1));
//! ```
//!
//! The type argument is a [`CarriedQuantityType`]; no bounded quantity type,
//! bare unit or owner `Rational` is accepted in its place:
//!
//! ```compile_fail,E0308
//! # use quire_exact::{BoundedQuantityType, NativeQuantity, NodeKey, PathSegment, QuantityBindings, UnitId};
//! # let key = NodeKey::new("test", vec![PathSegment::Text("cm".into())], 0).unwrap();
//! let ty = BoundedQuantityType::new(UnitId::declared(key), None, None).unwrap();
//! let _ = NativeQuantity::from_bindings(&ty, QuantityBindings::Int(1));
//! ```
//!
//! ```compile_fail,E0308
//! # use quire_exact::{NativeQuantity, NodeKey, PathSegment, QuantityBindings, UnitId};
//! # let key = NodeKey::new("test", vec![PathSegment::Text("cm".into())], 0).unwrap();
//! let unit = UnitId::declared(key);
//! let _ = NativeQuantity::from_bindings(&unit, QuantityBindings::Int(1));
//! ```
//!
//! ```compile_fail,E0308
//! # use quire_exact::{Integer, NativeQuantity, QuantityBindings, Rational};
//! let rational = Rational::from_integer(Integer::from(1_i64));
//! let _ = NativeQuantity::from_bindings(&rational, QuantityBindings::Int(1));
//! ```

use crate::integer::{Integer, IntegerInterval};
use crate::native_decimal::{NativeDecimal, NativeDecimalType};
use crate::quantity::{CarriedQuantityType, Quantity, QuantityCarrier};
use crate::rational::Rational;

/// The refusals of [`NativeQuantity::from_bindings`] (FR-398 Behavior 6), in
/// the order the checks run.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeQuantityRefusal {
    /// The bindings are not of the type's carrier.
    #[error("bindings are not of the quantity type's carrier")]
    CarrierMismatch,
    /// A bound of the type lies outside signed 128 bits.
    #[error("quantity type bound is not representable in the native width")]
    TypeNotRepresentable,
    /// A rational denominator binding is zero.
    #[error("rational denominator binding is zero")]
    ZeroDenominator,
    /// A rational pair is not in lowest terms, has a negative denominator, or
    /// is zero over a denominator other than one.
    #[error("rational binding is not canonical")]
    NonCanonical,
    /// The binding is not a member of the carrier, or a decimal scale lies
    /// outside `0..=4294967295`.
    #[error("binding is outside the carrier domain")]
    OutsideCarrierDomain,
    /// The canonical numerator or denominator of an admitted decimal does not
    /// fit signed 128 bits.
    #[error("canonical fraction is not representable in the native width")]
    NotRepresentable,
}

/// The physical bindings of one carried-quantity leaf, in the order of the
/// carrier (Contract CodeGen FR-025-AC-18).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuantityBindings {
    /// An `Int` carrier: the integer.
    Int(i64),
    /// A `Decimal` carrier: the coefficient then the scale.
    Decimal {
        /// The coefficient binding.
        coefficient: i64,
        /// The scale binding.
        scale: i64,
    },
    /// A `Rational` carrier: the numerator then the denominator.
    Rational {
        /// The numerator binding.
        numerator: i64,
        /// The denominator binding.
        denominator: i64,
    },
}

/// Which of the three carriers a type or a binding belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CarrierKind {
    Int,
    Decimal,
    Rational,
}

impl QuantityBindings {
    const fn kind(self) -> CarrierKind {
        match self {
            Self::Int(_) => CarrierKind::Int,
            Self::Decimal { .. } => CarrierKind::Decimal,
            Self::Rational { .. } => CarrierKind::Rational,
        }
    }
}

/// An inclusive pair of native bounds.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NativeRange {
    pub(crate) lower: i128,
    pub(crate) upper: i128,
}

impl NativeRange {
    const fn contains(self, value: i128) -> bool {
        self.lower <= value && value <= self.upper
    }
}

/// A carried-quantity type reduced to native-width bounds, or to the fact
/// that some bound does not fit.
#[derive(Clone, Copy, Debug)]
pub(crate) enum NativeQuantityShape {
    Int(NativeRange),
    Decimal(NativeDecimalType),
    Rational {
        numerator: NativeRange,
        denominator: NativeRange,
    },
    Unrepresentable(CarrierKind),
}

impl NativeQuantityShape {
    /// Read the native bounds of the owner's type. This neither allocates nor
    /// builds an owner value.
    pub(crate) fn of(ty: &CarriedQuantityType) -> Self {
        match ty.carrier() {
            QuantityCarrier::Integer { domain, .. } => match range_of(domain) {
                Some(range) => Self::Int(range),
                None => Self::Unrepresentable(CarrierKind::Int),
            },
            QuantityCarrier::Decimal(owner) => match NativeDecimalType::try_from(owner) {
                Ok(native) => Self::Decimal(native),
                Err(_) => Self::Unrepresentable(CarrierKind::Decimal),
            },
            QuantityCarrier::Rational(domain) => {
                match (range_of(domain.numerator()), range_of(domain.denominator())) {
                    (Some(numerator), Some(denominator)) => Self::Rational {
                        numerator,
                        denominator,
                    },
                    _ => Self::Unrepresentable(CarrierKind::Rational),
                }
            }
        }
    }
}

fn range_of(interval: &IntegerInterval) -> Option<NativeRange> {
    Some(NativeRange {
        lower: interval.lower().native()?,
        upper: interval.upper().native()?,
    })
}

/// The magnitude of a carried quantity: a canonical fraction of two signed
/// 128-bit integers (lowest terms, positive denominator, zero as `0/1`). It
/// is `Copy`, owns no heap type and carries no unit; the unit stays with the
/// [`CarriedQuantityType`] it was built against.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NativeQuantity {
    numerator: i128,
    denominator: i128,
}

impl NativeQuantity {
    /// Build the magnitude from the carrier's bindings, admitting a binding
    /// only when the owner's membership admits it (FR-398 Behavior 1 to 7).
    /// Nothing is normalized to make a binding admissible.
    pub fn from_bindings(
        ty: &CarriedQuantityType,
        bindings: QuantityBindings,
    ) -> Result<Self, NativeQuantityRefusal> {
        construct(&NativeQuantityShape::of(ty), bindings)
    }

    /// The numerator of the canonical fraction.
    pub const fn numerator(&self) -> i128 {
        self.numerator
    }

    /// The positive denominator of the canonical fraction.
    pub const fn denominator(&self) -> i128 {
        self.denominator
    }

    /// Read the magnitude out as the owner's `Quantity` in the type's unit.
    /// This allocates and is outside the constructor's zero-allocation window.
    pub fn to_quantity(&self, ty: &CarriedQuantityType) -> Quantity {
        let magnitude = Rational::new(
            Integer::from(self.numerator),
            Integer::from(self.denominator),
        )
        .expect("the denominator is positive by construction");
        Quantity::new(magnitude, ty.unit().clone())
    }
}

/// The constructor proper, over native-width bounds. This is the body that a
/// Kani proof draws symbolic bounds into.
pub(crate) fn construct(
    shape: &NativeQuantityShape,
    bindings: QuantityBindings,
) -> Result<NativeQuantity, NativeQuantityRefusal> {
    use NativeQuantityRefusal as Refusal;
    match (shape, bindings) {
        (NativeQuantityShape::Unrepresentable(kind), bindings) => {
            if bindings.kind() == *kind {
                Err(Refusal::TypeNotRepresentable)
            } else {
                Err(Refusal::CarrierMismatch)
            }
        }
        (NativeQuantityShape::Int(range), QuantityBindings::Int(value)) => {
            if range.contains(i128::from(value)) {
                Ok(fraction(i128::from(value), 1))
            } else {
                Err(Refusal::OutsideCarrierDomain)
            }
        }
        (
            NativeQuantityShape::Decimal(native),
            QuantityBindings::Decimal { coefficient, scale },
        ) => decimal(native, coefficient, scale),
        (
            NativeQuantityShape::Rational {
                numerator,
                denominator,
            },
            QuantityBindings::Rational {
                numerator: n,
                denominator: d,
            },
        ) => rational(*numerator, *denominator, n, d),
        _ => Err(Refusal::CarrierMismatch),
    }
}

const fn fraction(numerator: i128, denominator: i128) -> NativeQuantity {
    NativeQuantity {
        numerator,
        denominator,
    }
}

fn rational(
    numerators: NativeRange,
    denominators: NativeRange,
    numerator: i64,
    denominator: i64,
) -> Result<NativeQuantity, NativeQuantityRefusal> {
    use NativeQuantityRefusal as Refusal;
    if denominator == 0 {
        return Err(Refusal::ZeroDenominator);
    }
    let canonical = if denominator < 0 {
        false
    } else if numerator == 0 {
        denominator == 1
    } else {
        coprime(numerator.unsigned_abs(), denominator.unsigned_abs())
    };
    if !canonical {
        return Err(Refusal::NonCanonical);
    }
    let (numerator, denominator) = (i128::from(numerator), i128::from(denominator));
    if numerators.contains(numerator) && denominators.contains(denominator) {
        Ok(fraction(numerator, denominator))
    } else {
        Err(Refusal::OutsideCarrierDomain)
    }
}

fn decimal(
    native: &NativeDecimalType,
    coefficient: i64,
    scale: i64,
) -> Result<NativeQuantity, NativeQuantityRefusal> {
    use NativeQuantityRefusal as Refusal;
    // The carrier of FR-402 applies the owner's membership rule to the
    // normalized pair, so the two carriers cannot diverge in it.
    let Ok(drawn) = NativeDecimal::from_bindings(native, coefficient, scale) else {
        return Err(Refusal::OutsideCarrierDomain);
    };
    let (coefficient, scale) = drawn.normalized();
    if coefficient == 0 {
        return Ok(fraction(0, 1));
    }
    // `gcd(c, 10^s) = 2^min(v2(c), s) x 5^min(v5(c), s)`, so the canonical
    // fraction is `c / gcd` over `2^(s - twos) x 5^(s - fives)`.
    let twos = coefficient.unsigned_abs().trailing_zeros().min(scale);
    let fives = factors_of_five(coefficient.unsigned_abs()).min(scale);
    let numerator = coefficient / (1_i128 << twos) / FIVE_POWERS[fives as usize];
    let two_exponent = scale - twos;
    let five_exponent = scale - fives;
    if two_exponent > MAX_TWO_EXPONENT || five_exponent > MAX_FIVE_EXPONENT {
        return Err(Refusal::NotRepresentable);
    }
    match (1_i128 << two_exponent).checked_mul(FIVE_POWERS[five_exponent as usize]) {
        Some(denominator) => Ok(fraction(numerator, denominator)),
        None => Err(Refusal::NotRepresentable),
    }
}

/// The largest `k` with `2^k` inside signed 128 bits.
const MAX_TWO_EXPONENT: u32 = 126;
/// The largest `k` with `5^k` inside signed 128 bits.
const MAX_FIVE_EXPONENT: u32 = 54;

/// The multiplicity of the prime 5 in a nonzero magnitude below `2^64`: the
/// binary digits of the count are taken from 16 down, each when `5^places`
/// still divides, which is exact because divisibility by `5^p` is monotone
/// in `p`. At most 27 factors fit, so five steps suffice.
fn factors_of_five(magnitude: u128) -> u32 {
    let (magnitude, count) = strip_fives(magnitude, 0, 16);
    let (magnitude, count) = strip_fives(magnitude, count, 8);
    let (magnitude, count) = strip_fives(magnitude, count, 4);
    let (magnitude, count) = strip_fives(magnitude, count, 2);
    let (_, count) = strip_fives(magnitude, count, 1);
    count
}

fn strip_fives(magnitude: u128, count: u32, places: u32) -> (u128, u32) {
    let power = FIVE_POWERS[places as usize] as u128;
    if magnitude.is_multiple_of(power) {
        (magnitude / power, count + places)
    } else {
        (magnitude, count)
    }
}

/// Whether two nonzero magnitudes share no factor above one, by the binary
/// greatest-common-divisor algorithm unrolled to its worst case (128 steps)
/// so that it holds no loop. Every step at least halves the product of the
/// two odd parts, which starts below `2^126`.
fn coprime(left: u64, right: u64) -> bool {
    if (left | right) & 1 == 0 {
        return false;
    }
    let (divisor, _) = steps_128((
        left >> left.trailing_zeros(),
        right >> right.trailing_zeros(),
    ));
    divisor == 1
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

/// `5^0` through `5^54`, the powers inside signed 128 bits.
const FIVE_POWERS: [i128; 55] = [
    1,
    5,
    25,
    125,
    625,
    3_125,
    15_625,
    78_125,
    390_625,
    1_953_125,
    9_765_625,
    48_828_125,
    244_140_625,
    1_220_703_125,
    6_103_515_625,
    30_517_578_125,
    152_587_890_625,
    762_939_453_125,
    3_814_697_265_625,
    19_073_486_328_125,
    95_367_431_640_625,
    476_837_158_203_125,
    2_384_185_791_015_625,
    11_920_928_955_078_125,
    59_604_644_775_390_625,
    298_023_223_876_953_125,
    1_490_116_119_384_765_625,
    7_450_580_596_923_828_125,
    37_252_902_984_619_140_625,
    186_264_514_923_095_703_125,
    931_322_574_615_478_515_625,
    4_656_612_873_077_392_578_125,
    23_283_064_365_386_962_890_625,
    116_415_321_826_934_814_453_125,
    582_076_609_134_674_072_265_625,
    2_910_383_045_673_370_361_328_125,
    14_551_915_228_366_851_806_640_625,
    72_759_576_141_834_259_033_203_125,
    363_797_880_709_171_295_166_015_625,
    1_818_989_403_545_856_475_830_078_125,
    9_094_947_017_729_282_379_150_390_625,
    45_474_735_088_646_411_895_751_953_125,
    227_373_675_443_232_059_478_759_765_625,
    1_136_868_377_216_160_297_393_798_828_125,
    5_684_341_886_080_801_486_968_994_140_625,
    28_421_709_430_404_007_434_844_970_703_125,
    142_108_547_152_020_037_174_224_853_515_625,
    710_542_735_760_100_185_871_124_267_578_125,
    3_552_713_678_800_500_929_355_621_337_890_625,
    17_763_568_394_002_504_646_778_106_689_453_125,
    88_817_841_970_012_523_233_890_533_447_265_625,
    444_089_209_850_062_616_169_452_667_236_328_125,
    2_220_446_049_250_313_080_847_263_336_181_640_625,
    11_102_230_246_251_565_404_236_316_680_908_203_125,
    55_511_151_231_257_827_021_181_583_404_541_015_625,
];

#[cfg(kani)]
mod kani_proofs;
#[cfg(any(test, kani))]
mod reference;
#[cfg(test)]
mod tests;
