// SPDX-License-Identifier: AGPL-3.0-or-later
//! Independent bounded mathematics for the carried-quantity constructor
//! (FR-398 Behavior 9). It shares no code with the constructor: the decimal
//! fraction comes from the multiplicities of 2 and 5 counted by divisibility
//! tests against written-out powers, the membership rule is stated on
//! sign-and-magnitude values, and the rational canonical test is the
//! Euclidean algorithm, not the binary one. Nothing here loops, so a Kani
//! proof over it needs no unwind bound. Host tests bind it to the owner
//! route.

use super::NativeQuantityRefusal as Refusal;

/// The reference outcome: numerator and denominator, or the named refusal.
pub(super) type Outcome = Result<(i128, i128), Refusal>;

/// `2^0` through `2^63`, written out.
const TWO_POWERS: [u128; 64] = [
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1_024,
    2_048,
    4_096,
    8_192,
    16_384,
    32_768,
    65_536,
    131_072,
    262_144,
    524_288,
    1_048_576,
    2_097_152,
    4_194_304,
    8_388_608,
    16_777_216,
    33_554_432,
    67_108_864,
    134_217_728,
    268_435_456,
    536_870_912,
    1_073_741_824,
    2_147_483_648,
    4_294_967_296,
    8_589_934_592,
    17_179_869_184,
    34_359_738_368,
    68_719_476_736,
    137_438_953_472,
    274_877_906_944,
    549_755_813_888,
    1_099_511_627_776,
    2_199_023_255_552,
    4_398_046_511_104,
    8_796_093_022_208,
    17_592_186_044_416,
    35_184_372_088_832,
    70_368_744_177_664,
    140_737_488_355_328,
    281_474_976_710_656,
    562_949_953_421_312,
    1_125_899_906_842_624,
    2_251_799_813_685_248,
    4_503_599_627_370_496,
    9_007_199_254_740_992,
    18_014_398_509_481_984,
    36_028_797_018_963_968,
    72_057_594_037_927_936,
    144_115_188_075_855_872,
    288_230_376_151_711_744,
    576_460_752_303_423_488,
    1_152_921_504_606_846_976,
    2_305_843_009_213_693_952,
    4_611_686_018_427_387_904,
    9_223_372_036_854_775_808,
];

/// `5^0` through `5^54`, written out; `5^55` exceeds signed 128 bits.
const FIVE_POWERS: [u128; 55] = [
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

/// The multiplicity of a prime power in a magnitude: the count of `k` in the
/// listed range with `table[k]` dividing it, unrolled without a loop.
macro_rules! multiplicity {
    ($magnitude:expr, $table:expr, [$($k:literal),*]) => {
        0_u32 $(+ u32::from($magnitude.is_multiple_of($table[$k])))*
    };
}

/// `Int[lower, upper]`.
pub(super) fn int(lower: i128, upper: i128, value: i64) -> Outcome {
    let value = i128::from(value);
    if lower <= value && value <= upper {
        Ok((value, 1))
    } else {
        Err(Refusal::OutsideCarrierDomain)
    }
}

/// One Euclidean step on `(a, b)`; `(a, 0)` is final.
fn euclid((a, b): (u64, u64)) -> (u64, u64) {
    if b == 0 {
        (a, 0)
    } else {
        (b, a % b)
    }
}
fn euclid_2(state: (u64, u64)) -> (u64, u64) {
    euclid(euclid(state))
}
fn euclid_4(state: (u64, u64)) -> (u64, u64) {
    euclid_2(euclid_2(state))
}
fn euclid_8(state: (u64, u64)) -> (u64, u64) {
    euclid_4(euclid_4(state))
}
fn euclid_16(state: (u64, u64)) -> (u64, u64) {
    euclid_8(euclid_8(state))
}
fn euclid_32(state: (u64, u64)) -> (u64, u64) {
    euclid_16(euclid_16(state))
}
fn euclid_64(state: (u64, u64)) -> (u64, u64) {
    euclid_32(euclid_32(state))
}
/// The greatest common divisor; Euclid on 64-bit values ends within 92 steps.
pub(super) fn gcd(a: u64, b: u64) -> u64 {
    euclid_64(euclid_64((a, b))).0
}

/// `Rational[n_lower, n_upper; d_lower, d_upper]`.
pub(super) fn rational(
    (n_lower, n_upper): (i128, i128),
    (d_lower, d_upper): (i128, i128),
    numerator: i64,
    denominator: i64,
) -> Outcome {
    if denominator == 0 {
        return Err(Refusal::ZeroDenominator);
    }
    let lowest_terms = if numerator == 0 {
        denominator == 1
    } else {
        denominator > 0 && gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) == 1
    };
    if !lowest_terms {
        return Err(Refusal::NonCanonical);
    }
    let (n, d) = (i128::from(numerator), i128::from(denominator));
    if n_lower <= n && n <= n_upper && d_lower <= d && d <= d_upper {
        Ok((n, d))
    } else {
        Err(Refusal::OutsideCarrierDomain)
    }
}

/// The multiplicities of 2 and 5 in a nonzero 64-bit magnitude.
fn two_and_five(magnitude: u64) -> (u32, u32) {
    let magnitude = u128::from(magnitude);
    (
        multiplicity!(
            magnitude,
            TWO_POWERS,
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
            ]
        ),
        multiplicity!(
            magnitude,
            FIVE_POWERS,
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27
            ]
        ),
    )
}

/// `Decimal[lower, upper; min_scale, max_scale]`, the value `c / 10^s`.
pub(super) fn decimal(
    (lower, upper): (i128, i128),
    (min_scale, max_scale): (u32, u32),
    coefficient: i64,
    scale: i64,
) -> Outcome {
    let Ok(scale) = u32::try_from(scale) else {
        return Err(Refusal::OutsideCarrierDomain);
    };
    if coefficient == 0 {
        // Normalized scale 0, lifted to `min_scale`; the lifted zero is zero.
        let member = min_scale <= max_scale && lower <= 0 && 0 <= upper;
        return if member {
            Ok((0, 1))
        } else {
            Err(Refusal::OutsideCarrierDomain)
        };
    }
    let magnitude = coefficient.unsigned_abs();
    let (twos, fives) = two_and_five(magnitude);
    // Membership on the least-scale pair: drop `min(twos, fives, scale)`
    // trailing decimal zeros.
    let zeros = twos.min(fives).min(scale);
    let reduced = i128::from(coefficient)
        / (TWO_POWERS[zeros as usize] as i128)
        / (FIVE_POWERS[zeros as usize] as i128);
    let reduced_scale = scale - zeros;
    let lifted_scale = reduced_scale.max(min_scale);
    if lifted_scale > max_scale {
        return Err(Refusal::OutsideCarrierDomain);
    }
    let shift = lifted_scale - reduced_scale;
    let within = if shift > 38 {
        false
    } else {
        let power = (TWO_POWERS[shift as usize] * FIVE_POWERS[shift as usize]) as i128;
        match reduced.checked_mul(power) {
            Some(lifted) => lower <= lifted && lifted <= upper,
            // The product left the native width, so it lies beyond a bound in
            // its own direction.
            None => false,
        }
    };
    if !within {
        return Err(Refusal::OutsideCarrierDomain);
    }
    // The canonical fraction of `c / 10^s`, from `gcd(c, 10^s)`.
    let cancelled_twos = twos.min(scale);
    let cancelled_fives = fives.min(scale);
    let numerator = i128::from(coefficient)
        / (TWO_POWERS[cancelled_twos as usize] as i128)
        / (FIVE_POWERS[cancelled_fives as usize] as i128);
    let two_exponent = scale - cancelled_twos;
    let five_exponent = scale - cancelled_fives;
    if two_exponent > 126 || five_exponent > 54 {
        return Err(Refusal::NotRepresentable);
    }
    let denominator = (1_u128 << two_exponent).checked_mul(FIVE_POWERS[five_exponent as usize]);
    match denominator {
        Some(value) if value <= i128::MAX as u128 => Ok((numerator, value as i128)),
        _ => Err(Refusal::NotRepresentable),
    }
}
