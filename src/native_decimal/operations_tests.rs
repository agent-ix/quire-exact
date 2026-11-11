// SPDX-License-Identifier: AGPL-3.0-or-later
//! Unit evidence for the native Decimal operations (FR-403, TC-951): the
//! independent references bound to the owner route (Behavior 38), the
//! `bits(10^k)` formation, the gcd and the wide product.
use ix_trace_rs::trace;

use super::operations::{
    gcd256, gcd_steps_128, order_values, power_of_ten_bits, LOG2_10_FRACTION, LOG2_10_LOWER,
    LOG2_10_UPPER,
};
use super::reference::{
    decide, euclid_gcd, native_decision, order_ref, ref_op, ref_type, Decision, RefLoss,
};
use super::*;
use crate::accounting::{Meter, ScalarLimits};
use crate::decimal::{
    evaluate_decimal, power_of_ten_bits as owner_bits, DecimalLoss, DecimalOperation,
    DecimalRepresentation, DecimalResult,
};
use crate::outcome::{Outcome, Refusal, Undefined};

fn big(value: u128) -> Integer {
    Integer::from((value >> 64) as u64)
        .shifted_left(64)
        .add(&Integer::from(value as u64))
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A coefficient near the edges the grounds care about.
    fn coefficient(&mut self) -> i128 {
        let wide = (i128::from(self.next()) << 64) | i128::from(self.next());
        let shift = self.next() % 128;
        let sign = if self.next().is_multiple_of(2) { 1 } else { -1 };
        let nudge = i128::from(self.next() % 5) - 2;
        match self.next() % 8 {
            0 => nudge,
            1 => sign * (10_i128.pow((self.next() % 39) as u32)).saturating_add(nudge),
            2 => sign * (1_i128 << (shift % 127)).saturating_add(nudge),
            3 => i128::MAX.saturating_add(nudge),
            4 => i128::MIN.saturating_add(nudge),
            5 => sign * i128::from(self.next() % 1000),
            _ => wide >> shift,
        }
    }

    fn scale(&mut self) -> u32 {
        match self.next() % 8 {
            0 => 0,
            1 => (self.next() % 6) as u32,
            2 => 38,
            3 => 39,
            4 => (self.next() % 90) as u32,
            5 => u32::MAX,
            6 => u32::MAX - (self.next() % 3) as u32,
            _ => self.next() as u32,
        }
    }
}

const MODES: [RoundingMode; 6] = RoundingMode::ALL;
const BOUNDS: [(i128, i128); 7] = [
    (0, 100),
    (-100, 100),
    (0, 1),
    (i128::MIN, i128::MAX),
    (-(1 << 126), 1 << 126),
    (0, 10_i128.pow(38)),
    (i128::MIN, 0),
];
const SCALE_RANGES: [(u32, u32); 7] = [(0, 0), (0, 2), (0, 3), (0, 38), (38, 38), (0, 100), (0, 1)];

fn owner_decimal(pair: (i128, u32)) -> Decimal {
    Decimal::new(Integer::from(pair.0), pair.1)
}

fn bounded() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 100_000,
        decimal_digits: 100_000,
        scale_expansion: 4096,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    }
}

fn owner_outcome(ty: &NativeDecimalType, op: NativeDecimalOperation) -> Outcome<DecimalResult> {
    let target = ty.to_decimal_type();
    let operands = match op {
        NativeDecimalOperation::Add(a, b)
        | NativeDecimalOperation::Subtract(a, b)
        | NativeDecimalOperation::Multiply(a, b)
        | NativeDecimalOperation::Divide(a, b) => (a.to_decimal(), Some(b.to_decimal())),
        NativeDecimalOperation::Negate(a) | NativeDecimalOperation::Round(a) => {
            (a.to_decimal(), None)
        }
    };
    let (first, second) = (&operands.0, operands.1.as_ref());
    let operation = match (op, second) {
        (NativeDecimalOperation::Add(..), Some(b)) => DecimalOperation::Add(first, b),
        (NativeDecimalOperation::Subtract(..), Some(b)) => DecimalOperation::Subtract(first, b),
        (NativeDecimalOperation::Multiply(..), Some(b)) => DecimalOperation::Multiply(first, b),
        (NativeDecimalOperation::Divide(..), Some(b)) => DecimalOperation::Divide(first, b),
        (NativeDecimalOperation::Negate(_), _) => DecimalOperation::Negate(first),
        _ => DecimalOperation::Round(first),
    };
    evaluate_decimal(operation, &target, &mut Meter::new(bounded()))
}

fn owner_loss(loss: RefLoss, scale: u32) -> DecimalLoss {
    DecimalLoss::from_factored(
        (Integer::from(loss.numerator), Integer::from(loss.cofactor)),
        (loss.twos, loss.fives),
        DecimalRepresentation::new(Integer::from(loss.rounded), scale),
        loss.mode,
    )
}

/// The reference, the native path and the owner route agree on one case.
fn bind(ty: &NativeDecimalType, op: NativeDecimalOperation) -> bool {
    let reference = decide(ref_type(ty), ref_op(op));
    assert_eq!(native_decision(ty, op), reference, "{ty:?} {op:?}");
    if matches!(reference, Decision::Capacity(_)) {
        return false;
    }
    let owner = owner_outcome(ty, op);
    match reference {
        Decision::Capacity(_) => unreachable!("handled above"),
        Decision::DivisionByZero => {
            assert_eq!(owner, Outcome::Undefined(Undefined::DivisionByZero));
        }
        Decision::Inexact => {
            assert_eq!(owner, Outcome::Refused(ty.to_decimal_type().inexact()));
        }
        Decision::OutOfDomain => {
            assert!(matches!(
                owner,
                Outcome::Refused(Refusal::DecimalOutOfDomain { .. })
            ));
        }
        Decision::Completed {
            retained,
            scale,
            loss,
        } => {
            let expected = DecimalResult::from_parts(
                Decimal::new(Integer::from(retained), scale),
                loss.map(|loss| owner_loss(loss, scale)),
            );
            assert_eq!(owner, Outcome::Completed(expected), "{ty:?} {op:?}");
        }
    }
    true
}

fn operations(a: (i128, u32), b: (i128, u32)) -> [NativeDecimalOperation; 6] {
    let (x, y) = (
        NativeDecimal::from_retained(a.0, a.1),
        NativeDecimal::from_retained(b.0, b.1),
    );
    [
        NativeDecimalOperation::Add(x, y),
        NativeDecimalOperation::Subtract(x, y),
        NativeDecimalOperation::Multiply(x, y),
        NativeDecimalOperation::Divide(x, y),
        NativeDecimalOperation::Negate(x),
        NativeDecimalOperation::Round(x),
    ]
}

#[test]
#[trace("TC-951", "FR-403-AC-11", "FR-403-AC-12")]
fn the_reference_is_bound_to_the_owner_route_over_the_grid() {
    let edge = [
        (0, 0),
        (0, 39),
        (1, 0),
        (-1, 0),
        (3, 0),
        (-3, 0),
        (8, 0),
        (9, 0),
        (5, 1),
        (125, 3),
        (-125, 3),
        (1005, 1),
        (1, 2),
        (1, 38),
        (1, 39),
        (i128::MAX, 0),
        (i128::MIN, 0),
        (i128::MAX, 38),
        (i128::MAX, 39),
        (1 << 126, 0),
        ((1 << 126) - 1, 0),
        ((1 << 126) + 1, 0),
        (i128::MAX - 1, 0),
        (1, u32::MAX),
    ];
    let (mut bound, mut capacity) = (0_u32, 0_u32);
    for (lower, upper) in BOUNDS {
        for (min_scale, max_scale) in SCALE_RANGES {
            for mode in MODES {
                let ty = NativeDecimalType::new(
                    lower,
                    upper,
                    u64::from(min_scale),
                    u64::from(max_scale),
                    mode,
                )
                .unwrap();
                for a in edge {
                    for b in edge {
                        for op in operations(a, b) {
                            if bind(&ty, op) {
                                bound += 1;
                            } else {
                                capacity += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(bound > 20_000 && capacity > 5_000, "{bound} {capacity}");
}

#[test]
#[trace("TC-951", "FR-403-AC-11", "FR-403-AC-12")]
fn the_reference_is_bound_to_the_owner_route_over_random_operands() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut bound = 0_u32;
    for round in 0..60_000_u32 {
        let (lower, upper) = BOUNDS[(rng.next() % BOUNDS.len() as u64) as usize];
        let (min_scale, max_scale) =
            SCALE_RANGES[(rng.next() % SCALE_RANGES.len() as u64) as usize];
        let mode = MODES[(rng.next() % 6) as usize];
        let ty = NativeDecimalType::new(
            lower,
            upper,
            u64::from(min_scale),
            u64::from(max_scale),
            mode,
        )
        .unwrap();
        let (a, b) = (
            (rng.coefficient(), rng.scale()),
            (rng.coefficient(), rng.scale()),
        );
        let op = operations(a, b)[(round % 6) as usize];
        if bind(&ty, op) {
            bound += 1;
        }
        let (x, y) = (
            NativeDecimal::from_retained(a.0, a.1),
            NativeDecimal::from_retained(b.0, b.1),
        );
        let expected = owner_decimal(a).compare(&owner_decimal(b));
        assert_eq!(order_values(x, y), expected, "{a:?} {b:?}");
        assert_eq!(order_ref(a, b), expected, "{a:?} {b:?}");
    }
    assert!(bound > 5_000, "{bound}");
}

const KNOWN_BITS: [(u32, u64); 8] = [
    (1, 4),
    (2, 7),
    (3, 10),
    (38, 127),
    (39, 130),
    (2_147_483_647, 7_133_786_261),
    (2_147_483_648, 7_133_786_264),
    (4_294_967_294, 14_267_572_521),
];

#[test]
#[trace("TC-951", "FR-403-AC-9")]
fn bits_of_a_power_of_ten_is_exact_over_the_shift_range() {
    for shift in 0..=200_u32 {
        assert_eq!(
            Integer::from(power_of_ten_bits(shift)),
            owner_bits(u64::from(shift)),
            "k = {shift}"
        );
    }
    for shift in [
        1_u32 << 31,
        (1 << 31) - 1,
        u32::MAX - 1,
        u32::MAX,
        1_000_000,
        123_456_789,
    ] {
        assert_eq!(
            Integer::from(power_of_ten_bits(shift)),
            owner_bits(u64::from(shift)),
            "k = {shift}"
        );
    }
    for (shift, bits) in KNOWN_BITS {
        assert_eq!(power_of_ten_bits(shift), bits, "k = {shift}");
    }
    assert_eq!(power_of_ten_bits(u32::MAX), 14_267_572_524);
    // a table clamped at 38 or 39 is wrong at 2^32 - 1
    assert_ne!(power_of_ten_bits(u32::MAX), 130);
}

#[test]
#[trace("TC-951", "FR-403-AC-9")]
fn the_log2_ten_approximations_share_a_floor_over_every_u32() {
    assert!((67..=94).contains(&LOG2_10_FRACTION));
    assert_eq!(LOG2_10_UPPER, LOG2_10_LOWER + 1);
    // The approximations differ only where k x log2(10) lies within k x
    // 2^-90 of an integer; for k < 2^32 that forces k to be the denominator
    // of a continued-fraction convergent of log2(10), so those k (and every
    // multiple, which names the same fraction) are the whole obligation.
    let convergent_denominators: [u128; 22] = [
        1,
        3,
        28,
        59,
        146,
        643,
        4004,
        8651,
        12655,
        21306,
        76573,
        97879,
        1_838_395,
        1_936_274,
        13_456_039,
        15_392_313,
        44_240_665,
        59_632_978,
        103_873_643,
        475_127_550,
        579_001_193,
        2_507_285_164,
    ];
    for q in convergent_denominators {
        for multiple in 1..=8_u128 {
            let k = q * multiple;
            if k > u128::from(u32::MAX) {
                continue;
            }
            let lower = (k * LOG2_10_LOWER) >> LOG2_10_FRACTION;
            let upper = (k * LOG2_10_UPPER) >> LOG2_10_FRACTION;
            assert_eq!(lower, upper, "k = {k}");
        }
    }
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    for _ in 0..2_000_000 {
        let k = u128::from(rng.next() as u32);
        assert_eq!(
            (k * LOG2_10_LOWER) >> LOG2_10_FRACTION,
            (k * LOG2_10_UPPER) >> LOG2_10_FRACTION,
            "k = {k}"
        );
    }
    assert!(u128::from(u32::MAX) * LOG2_10_UPPER < u128::MAX);
}

#[test]
#[trace("TC-951", "FR-403-AC-8")]
fn digit_and_bit_counts_are_exact() {
    use super::operations::{bits_of, digits_of};
    assert_eq!((bits_of(0), digits_of(0)), (1, 1));
    for exponent in 0..=38_u32 {
        let power = 10_u128.pow(exponent);
        assert_eq!(digits_of(power), u64::from(exponent) + 1);
        if exponent > 0 {
            assert_eq!(digits_of(power - 1), u64::from(exponent));
        }
        assert_eq!(digits_of(power + 1), u64::from(exponent) + 1);
    }
    assert_eq!(digits_of(1_u128 << 127), 39);
    assert_eq!(bits_of(1_u128 << 127), 128);
    assert_eq!(bits_of(1), 1);
    assert_eq!(bits_of(255), 8);
    assert_eq!(bits_of(256), 9);
    let mut rng = Rng(7);
    for _ in 0..20_000 {
        let value = (u128::from(rng.next()) << 64 | u128::from(rng.next())) >> (rng.next() % 128);
        assert_eq!(
            Integer::from(digits_of(value)),
            Integer::from(value.checked_ilog10().map_or(1, |l| u64::from(l) + 1))
        );
    }
}

#[test]
#[trace("TC-951", "FR-403-AC-8")]
fn the_wide_product_is_exact() {
    use super::operations::widening_mul;
    let mut rng = Rng(11);
    let mut samples = vec![0_u128, 1, u128::MAX, 1 << 127, (1 << 64) - 1, 1 << 64];
    for _ in 0..2_000 {
        samples.push(u128::from(rng.next()) << 64 | u128::from(rng.next()));
        samples.push(u128::from(rng.next()));
    }
    for window in samples.windows(2) {
        let (a, b) = (window[0], window[1]);
        let (high, low) = widening_mul(a, b);
        let product = big(high).shifted_left(128).add(&big(low));
        assert_eq!(product, big(a).mul(&big(b)), "{a} x {b}");
    }
}

#[test]
#[trace("TC-951", "FR-403-AC-5")]
fn the_gcd_takes_exactly_256_steps_and_handles_a_zero_operand() {
    // 252 steps are needed here (Behavior 16): the gcd is 1.
    let (n, d) = ((1_u128 << 126) - 1, 1_u128 << 126);
    assert_eq!(gcd256(n, d), 1);
    // after 128 steps the state is (2^124 - 1, 1) with g = 1; a sequence that
    // short reads the gcd 2^124 and reduces to the wrong pair
    let (u, v, g) = gcd_steps_128((n, d, 1));
    assert_eq!((u, v, g), ((1_u128 << 124) - 1, 1, 1));
    assert_eq!(g * (u + v), 1_u128 << 124);
    assert_ne!(g * (u + v), gcd256(n, d));
    // a zero operand leaves the state unchanged
    assert_eq!(gcd256(0, 12), 12);
    assert_eq!(gcd256(12, 0), 12);
    assert_eq!(gcd256(0, 1), 1);
    // halving an even operand while the other is zero would reach gcd 3
    assert_ne!(gcd256(0, 12), 3);
    assert_eq!(gcd256(1 << 127, 1 << 127), 1 << 127);
    assert_eq!(gcd256(1 << 127, 3), 1);
    let mut rng = Rng(99);
    for _ in 0..20_000 {
        let shift_a = rng.next() % 128;
        let shift_b = rng.next() % 128;
        let a = (u128::from(rng.next()) << 64 | u128::from(rng.next())) >> shift_a;
        let b = (u128::from(rng.next()) << 64 | u128::from(rng.next())) >> shift_b;
        if a == 0 && b == 0 {
            continue;
        }
        assert_eq!(gcd256(a, b), euclid_gcd(a, b), "{a} {b}");
    }
}

#[test]
#[trace("TC-951", "FR-403-AC-12")]
fn the_reference_rejects_wrong_decisions() {
    // Controls: the reference distinguishes the mutants of TC-951.
    let ty = NativeDecimalType::new(0, 100, 0, 2, RoundingMode::Exact).unwrap();
    let five = NativeDecimal::from_retained(5, 1);
    let add = decide(
        ref_type(&ty),
        ref_op(NativeDecimalOperation::Add(five, five)),
    );
    assert_eq!(
        add,
        Decision::Completed {
            retained: 100,
            scale: 2,
            loss: None
        }
    );
    assert_ne!(
        add,
        Decision::Completed {
            retained: 1,
            scale: 0,
            loss: None
        }
    );
    let nearest = NativeDecimalType::new(0, 1, 0, 0, RoundingMode::NearestEven).unwrap();
    let a = NativeDecimal::from_retained(1 << 126, 0);
    let b = NativeDecimal::from_retained((1 << 126) + 1, 0);
    assert_eq!(
        decide(
            ref_type(&nearest),
            ref_op(NativeDecimalOperation::Divide(a, b))
        ),
        Decision::Capacity(NativeDecimalCapacityQuantity::DoubledRemainder)
    );
}
