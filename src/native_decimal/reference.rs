// SPDX-License-Identifier: AGPL-3.0-or-later
//! Independent bounded mathematics for the native Decimal carrier. It shares
//! no code with the constructor or the normalized view: powers of ten are
//! written as literals, magnitudes are `u128`, and normalization is stated
//! as a postcondition, never computed. Host tests bind it to the owner route.

use core::cmp::Ordering;

use super::operations::{
    settle, NativeDecimalCapacityQuantity as Q, NativeDecimalOperation, Settled,
};
use super::{NativeDecimal, NativeDecimalType};
use crate::decimal::RoundingMode;

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

/// `5^0` through `5^54`, written out; `5^55` exceeds `u128`.
pub(super) const FIVE_POWERS: [u128; 55] = [
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

/// Count of `k` in the listed range, `k <= limit`, with `table[k]` dividing
/// the magnitude: the multiplicity of the prime power, unrolled without a
/// loop so a proof needs no unwind bound.
macro_rules! multiplicity {
    ($magnitude:expr, $limit:expr, $table:expr, [$($k:literal),*]) => {
        0_u64 $(+ u64::from($k <= $limit && $magnitude % $table[$k] == 0))*
    };
}

/// The reference types, written independently of the native type.
#[derive(Clone, Copy, Debug)]
pub(super) struct RefType {
    pub(super) lower: i128,
    pub(super) upper: i128,
    pub(super) min_scale: u32,
    pub(super) max_scale: u32,
    pub(super) mode: RoundingMode,
}

/// One operation over `(coefficient, scale)` operands.
#[derive(Clone, Copy, Debug)]
pub(super) enum RefOp {
    Add((i128, u32), (i128, u32)),
    Subtract((i128, u32), (i128, u32)),
    Multiply((i128, u32), (i128, u32)),
    Divide((i128, u32), (i128, u32)),
    Negate((i128, u32)),
    Round((i128, u32)),
}

/// The factored loss record of the reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RefLoss {
    pub(super) numerator: i128,
    pub(super) cofactor: i128,
    pub(super) twos: u64,
    pub(super) fives: u64,
    pub(super) rounded: i128,
    pub(super) mode: RoundingMode,
}

/// The decision of the reference: admission, outcome, value and loss.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Decision {
    Capacity(Q),
    DivisionByZero,
    Inexact,
    OutOfDomain,
    Completed {
        retained: i128,
        scale: u32,
        loss: Option<RefLoss>,
    },
}

/// The number of base-ten digits of a magnitude, at least 1.
fn digit_count(magnitude: u128) -> u64 {
    1 + multiplicity_ge(magnitude)
}

fn multiplicity_ge(magnitude: u128) -> u64 {
    macro_rules! at_least {
        ($($k:literal),*) => { 0_u64 $(+ u64::from(magnitude >= TEN_POWERS[$k]))* };
    }
    at_least!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38
    )
}

/// Signed value from a sign and a magnitude, when it fits 128 bits.
fn signed(negative: bool, magnitude: u128) -> Option<i128> {
    if magnitude == 0 {
        Some(0)
    } else if negative {
        (magnitude <= 1_u128 << 127).then(|| {
            if magnitude == 1_u128 << 127 {
                i128::MIN
            } else {
                -(magnitude as i128)
            }
        })
    } else {
        (magnitude < 1_u128 << 127).then_some(magnitude as i128)
    }
}

/// `c x 10^k` when it fits 128 bits; `k` is at most 38.
fn up(c: i128, k: u64) -> Option<i128> {
    let part = split(c);
    part.magnitude
        .checked_mul(TEN_POWERS[k as usize])
        .and_then(|magnitude| signed(part.negative, magnitude))
}

type Pair = (u128, u128);

fn euclid_step(pair: Pair) -> Pair {
    if pair.1 == 0 {
        pair
    } else {
        (pair.1, pair.0 % pair.1)
    }
}
fn euclid_2(pair: Pair) -> Pair {
    euclid_step(euclid_step(pair))
}
fn euclid_4(pair: Pair) -> Pair {
    euclid_2(euclid_2(pair))
}
fn euclid_8(pair: Pair) -> Pair {
    euclid_4(euclid_4(pair))
}
fn euclid_16(pair: Pair) -> Pair {
    euclid_8(euclid_8(pair))
}
fn euclid_32(pair: Pair) -> Pair {
    euclid_16(euclid_16(pair))
}
fn euclid_64(pair: Pair) -> Pair {
    euclid_32(euclid_32(pair))
}
fn euclid_256(pair: Pair) -> Pair {
    euclid_64(euclid_64(euclid_64(euclid_64(pair))))
}

/// Euclid's algorithm for two magnitudes, not both zero: at most 185
/// remainder steps reach the gcd for 128-bit inputs, and 256 are taken.
pub(super) fn euclid_gcd(a: u128, b: u128) -> u128 {
    euclid_256((a, b)).0
}

/// The least-scale pair equal in value: trailing decimal zeros removed while
/// the scale allows, counted as the multiplicity of ten in the magnitude.
fn normalize(c: i128, scale: u32) -> (i128, u32) {
    if c == 0 {
        return (0, 0);
    }
    let magnitude = split(c).magnitude;
    let limit = u64::from(scale);
    let zeros = multiplicity!(
        magnitude,
        limit,
        TEN_POWERS,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38
        ]
    );
    (c / TEN_POWERS[zeros as usize] as i128, scale - zeros as u32)
}

fn member(t: RefType, c: i128, scale: u32) -> bool {
    let (c2, s2) = normalize(c, scale);
    is_member(t.lower, t.upper, t.min_scale, t.max_scale, c2, s2)
}

/// The exact value `n / d / 10^ten` of a rounding step as the owner records
/// it: common twos and fives of `n` and `10^ten` cancelled, the denominator's
/// twos and fives factored out, the rest the cofactor.
fn loss_of(n: i128, d: i128, ten: u64, rounded: i128, mode: RoundingMode) -> RefLoss {
    let n_magnitude = split(n).magnitude;
    let cancelled_twos = u64::from(n_magnitude.trailing_zeros()).min(ten);
    let n_odd = n_magnitude >> cancelled_twos;
    let cancelled_fives = multiplicity!(
        n_odd,
        ten,
        FIVE_POWERS,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54
        ]
    );
    let numerator_magnitude = n_odd / FIVE_POWERS[cancelled_fives as usize];
    let d_magnitude = split(d).magnitude;
    let d_twos = u64::from(d_magnitude.trailing_zeros());
    let d_odd = d_magnitude >> d_twos;
    let d_fives = multiplicity!(
        d_odd,
        u64::MAX,
        FIVE_POWERS,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54
        ]
    );
    let cofactor = d_odd / FIVE_POWERS[d_fives as usize];
    let numerator = signed(n < 0, numerator_magnitude).unwrap_or(i128::MIN);
    RefLoss {
        numerator,
        cofactor: cofactor as i128,
        twos: d_twos + ten - cancelled_twos,
        fives: d_fives + ten - cancelled_fives,
        rounded,
        mode,
    }
}

/// Round `n / d` for `d >= 2` as the owner does, from the sign and
/// magnitudes; the nearest modes need twice the floor remainder to fit.
fn round_ref(n: i128, d: i128, mode: RoundingMode) -> Result<i128, Q> {
    let part = split(n);
    let d_magnitude = split(d).magnitude;
    let quotient = part.magnitude / d_magnitude;
    let remainder = part.magnitude % d_magnitude;
    let inexact = remainder != 0;
    // floor and ceiling as sign/magnitude pairs
    let (floor_mag, ceil_mag) = if part.negative {
        (quotient + u128::from(inexact), quotient)
    } else {
        (quotient, quotient + u128::from(inexact))
    };
    let floor = signed(part.negative, floor_mag).unwrap_or(i128::MIN);
    let ceiling = signed(part.negative && ceil_mag != 0, ceil_mag).unwrap_or(i128::MIN);
    let floor_remainder = if part.negative {
        d_magnitude - remainder
    } else {
        remainder
    } % d_magnitude;
    match mode {
        RoundingMode::Exact | RoundingMode::TowardNegative => Ok(floor),
        RoundingMode::TowardPositive => Ok(ceiling),
        RoundingMode::TowardZero => Ok(if part.negative { ceiling } else { floor }),
        RoundingMode::NearestEven | RoundingMode::NearestAway => {
            let twice = 2 * floor_remainder;
            if twice >= 1_u128 << 127 {
                return Err(Q::DoubledRemainder);
            }
            Ok(if twice < d_magnitude {
                floor
            } else if twice > d_magnitude {
                ceiling
            } else if mode == RoundingMode::NearestEven {
                if floor_mag % 2 == 0 {
                    floor
                } else {
                    ceiling
                }
            } else if part.negative {
                floor
            } else {
                ceiling
            })
        }
    }
}

fn finish(t: RefType, placed: (i128, u32), retained: i128, loss: Option<RefLoss>) -> Decision {
    if member(t, placed.0, placed.1) {
        Decision::Completed {
            retained,
            scale: t.max_scale,
            loss,
        }
    } else {
        Decision::OutOfDomain
    }
}

/// A rounding step over `n / d` in units of `10^-T`.
fn step(t: RefType, units: (i128, i128), exact: (i128, i128, u64)) -> Decision {
    if t.mode == RoundingMode::Exact {
        return Decision::Inexact;
    }
    match round_ref(units.0, units.1, t.mode) {
        Err(quantity) => Decision::Capacity(quantity),
        Ok(rounded) => {
            let loss = loss_of(exact.0, exact.1, exact.2, rounded, t.mode);
            finish(t, (rounded, t.max_scale), rounded, Some(loss))
        }
    }
}

fn place_scaled(t: RefType, v: i128, working: u64) -> Decision {
    let target = u64::from(t.max_scale);
    if working <= target {
        return match up(v, target - working) {
            Some(retained) => finish(t, (v, working as u32), retained, None),
            None => Decision::Capacity(Q::RetainedCoefficient),
        };
    }
    if v == 0 {
        return finish(t, (0, t.max_scale), 0, None);
    }
    let excess = working - target;
    let magnitude = split(v).magnitude;
    if excess > digit_count(magnitude) {
        let sign = if v < 0 { -1 } else { 1 };
        return step(t, (sign, 10), (v, 1, working));
    }
    if excess > 38 {
        return Decision::Capacity(Q::PowerOfTen);
    }
    let power = TEN_POWERS[excess as usize];
    if magnitude.is_multiple_of(power) {
        let quotient = signed(v < 0, magnitude / power).unwrap_or(i128::MIN);
        return finish(t, (quotient, t.max_scale), quotient, None);
    }
    let g = euclid_gcd(magnitude, power);
    let n = signed(v < 0, magnitude / g).unwrap_or(i128::MIN);
    step(t, (n, (power / g) as i128), (v, 1, working))
}

fn lift_exponent(t: RefType, working: u64) -> u64 {
    let target = u64::from(t.max_scale);
    target - working.min(target)
}

fn decide_pair(t: RefType, a: (i128, u32), b: (i128, u32), subtract: bool) -> Decision {
    let s = a.1.max(b.1);
    let (ka, kb) = (u64::from(s - a.1), u64::from(s - b.1));
    if ka > 38 || kb > 38 || lift_exponent(t, u64::from(s)) > 38 {
        return Decision::Capacity(Q::Exponent);
    }
    let (Some(xa), Some(xb)) = (up(a.0, ka), up(b.0, kb)) else {
        return Decision::Capacity(Q::ShiftedOperand);
    };
    let (result, quantity) = if subtract {
        (xa.checked_sub(xb), Q::Difference)
    } else {
        (xa.checked_add(xb), Q::Sum)
    };
    match result {
        Some(value) => place_scaled(t, value, u64::from(s)),
        None => Decision::Capacity(quantity),
    }
}

fn decide_divide(t: RefType, a: (i128, u32), b: (i128, u32)) -> Decision {
    if b.0 == 0 {
        return Decision::DivisionByZero;
    }
    let up_scale = u64::from(t.max_scale) + u64::from(b.1);
    let (kn, kd) = (
        up_scale.saturating_sub(u64::from(a.1)),
        u64::from(a.1).saturating_sub(up_scale),
    );
    if kn > 38 || kd > 38 {
        return Decision::Capacity(Q::Exponent);
    }
    let Some(n) = up(a.0, kn) else {
        return Decision::Capacity(Q::Numerator);
    };
    let Some(d) = up(b.0, kd) else {
        return Decision::Capacity(Q::Denominator);
    };
    let (n_magnitude, d_magnitude) = (split(n).magnitude, split(d).magnitude);
    let g = euclid_gcd(n_magnitude, d_magnitude);
    let negative = n != 0 && (n < 0) != (d < 0);
    let (Some(reduced_n), Some(reduced_d)) = (
        signed(negative, n_magnitude / g),
        signed(false, d_magnitude / g),
    ) else {
        return Decision::Capacity(Q::CanonicalQuotient);
    };
    if reduced_d == 1 {
        return finish(t, (reduced_n, t.max_scale), reduced_n, None);
    }
    step(
        t,
        (reduced_n, reduced_d),
        (reduced_n, reduced_d, u64::from(t.max_scale)),
    )
}

/// The reference decision for an operation in a type.
pub(super) fn decide(t: RefType, op: RefOp) -> Decision {
    match op {
        RefOp::Add(a, b) => decide_pair(t, a, b, false),
        RefOp::Subtract(a, b) => decide_pair(t, a, b, true),
        RefOp::Multiply(a, b) => {
            let working = u64::from(a.1) + u64::from(b.1);
            if lift_exponent(t, working) > 38 {
                return Decision::Capacity(Q::Exponent);
            }
            match a.0.checked_mul(b.0) {
                Some(value) => place_scaled(t, value, working),
                None => Decision::Capacity(Q::Product),
            }
        }
        RefOp::Divide(a, b) => decide_divide(t, a, b),
        RefOp::Negate((c, s)) | RefOp::Round((c, s)) => {
            if lift_exponent(t, u64::from(s)) > 38 {
                return Decision::Capacity(Q::Exponent);
            }
            let negate = matches!(op, RefOp::Negate(_));
            match if negate { c.checked_neg() } else { Some(c) } {
                Some(value) => place_scaled(t, value, u64::from(s)),
                None => Decision::Capacity(Q::Negation),
            }
        }
    }
}

/// The order of two values: signs first, then magnitudes aligned to the
/// larger scale. `x x 10^d` against `y` is greater beyond `d = 38` (one
/// above `y`'s 39 digits) and otherwise a `u128` product or an overflow.
pub(super) fn order_ref(a: (i128, u32), b: (i128, u32)) -> Ordering {
    let (x, y) = (split(a.0), split(b.0));
    let by_sign = order(x, y);
    if a.0 == 0 || b.0 == 0 || x.negative != y.negative {
        return by_sign;
    }
    let magnitudes = if a.1 == b.1 {
        x.magnitude.cmp(&y.magnitude)
    } else if a.1 > b.1 {
        shifted_against(y.magnitude, a.1 - b.1, x.magnitude).reverse()
    } else {
        shifted_against(x.magnitude, b.1 - a.1, y.magnitude)
    };
    if x.negative {
        magnitudes.reverse()
    } else {
        magnitudes
    }
}

fn shifted_against(x: u128, d: u32, y: u128) -> Ordering {
    if d > 38 {
        return Ordering::Greater;
    }
    match x.checked_mul(TEN_POWERS[d as usize]) {
        Some(product) => product.cmp(&y),
        None => Ordering::Greater,
    }
}

/// The reference type of a native type.
pub(super) fn ref_type(ty: &NativeDecimalType) -> RefType {
    RefType {
        lower: ty.lower,
        upper: ty.upper,
        min_scale: ty.min_scale,
        max_scale: ty.max_scale,
        mode: ty.rounding,
    }
}

/// The reference operation of a native operation.
pub(super) fn ref_op(op: NativeDecimalOperation) -> RefOp {
    let pair = |d: NativeDecimal| (d.coefficient, d.scale);
    match op {
        NativeDecimalOperation::Add(a, b) => RefOp::Add(pair(a), pair(b)),
        NativeDecimalOperation::Subtract(a, b) => RefOp::Subtract(pair(a), pair(b)),
        NativeDecimalOperation::Multiply(a, b) => RefOp::Multiply(pair(a), pair(b)),
        NativeDecimalOperation::Divide(a, b) => RefOp::Divide(pair(a), pair(b)),
        NativeDecimalOperation::Negate(a) => RefOp::Negate(pair(a)),
        NativeDecimalOperation::Round(a) => RefOp::Round(pair(a)),
    }
}

/// What the native path decides for an operation: its admission, outcome,
/// completed value and loss record, with no charge.
pub(super) fn native_decision(ty: &NativeDecimalType, op: NativeDecimalOperation) -> Decision {
    match ty.admit(op) {
        Err(refusal) => Decision::Capacity(refusal.quantity),
        Ok(admitted) => match settle(&admitted) {
            Settled::DivisionByZero => Decision::DivisionByZero,
            Settled::Inexact => Decision::Inexact,
            Settled::OutOfDomain { .. } => Decision::OutOfDomain,
            Settled::Completed(result, _) => Decision::Completed {
                retained: result.value.coefficient,
                scale: result.value.scale,
                loss: result.loss.map(|loss| RefLoss {
                    numerator: loss.numerator,
                    cofactor: loss.cofactor,
                    twos: loss.twos,
                    fives: loss.fives,
                    rounded: loss.rounded.coefficient,
                    mode: loss.mode,
                }),
            },
        },
    }
}
