// SPDX-License-Identifier: AGPL-3.0-or-later
//! Kani harnesses of FR-405 Behavior 19: the native float operations over
//! symbolic bit patterns against an expected side that is the host language's
//! own `f32`/`f64` arithmetic (an independent reference that shares no
//! function with the native body and not even the bit-pattern decode).
//!
//! Each arithmetic operation and width has three harnesses that together
//! cover the six rounding spellings: one for `NearestEven`, one for strict
//! `Exact`, and one for the four directed and nearest-away spellings (the
//! rounding drawn symbolically among those four). The operands are fully
//! symbolic bit patterns. The cases whose result is a NaN (a NaN operand,
//! infinity minus infinity, zero times infinity, zero over zero, infinity over
//! infinity, the square root of a negative nonzero, the infinite
//! fused-multiply-add cases) are decided first by a predicate over the bits;
//! only the other cases evaluate the host operation, so the host side never
//! executes a NaN-producing float operation (Kani's default `--nan-check`
//! would fail it).
//!
//! Expected side. For `NearestEven` and for a completed strict `Exact`, a
//! non-NaN result equals the host operation's bits. A NaN result equals the
//! leftmost operand NaN with the quiet bit set, or the canonical NaN when no
//! operand is a NaN.
//!
//! Covers: result bits under `NearestEven` and under a completed `Exact`; for
//! every spelling that a numeric (non-NaN) host result gives a non-NaN native
//! result with no `invalid` flag, and a NaN host result gives the NaN above
//! (sign, quietness, payload) with `invalid` raised exactly when an operand is
//! signalling or no operand is a NaN; that only `Exact` refuses; the width and
//! rounding provenance; the result-unit charge.
//!
//! Square root. Kani's model of the host `sqrt` is not bit-exact, so its
//! expected side is an exact-integer bracket of the correctly rounded root. The
//! divide expected side computes the infinite-operand quotients analytically
//! because CBMC's `--nan-check` over-reports a division by infinity.
//!
//! Left to the host differential: result bits under the four directed and
//! nearest-away spellings, the `overflow`, `underflow`, `inexact` and
//! `divide_by_zero` flags, the `Exact` refusal's would-be flags, charge
//! amounts and meter limits (the harness runs a generous meter).
use crate::accounting::{LimitKind, Meter, ScalarLimits};
use crate::decimal::RoundingMode;
use crate::ieee::{IeeeComparison, IeeeFlag, IeeeWidth};
use crate::native_float::{
    compare_native_float, evaluate_native_float, NativeFloat, NativeFloatOperation,
};
use crate::outcome::Outcome;

fn generous_meter() -> Meter {
    Meter::new(ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    })
}

/// Any of the six spellings (used by Negate, which does not round).
fn any_rounding() -> RoundingMode {
    let index: u8 = kani::any();
    kani::assume(index < 6);
    RoundingMode::ALL[usize::from(index)]
}

/// Any of the four directed and nearest-away spellings.
fn any_directed() -> RoundingMode {
    let index: u8 = kani::any();
    kani::assume(index < 4);
    [
        RoundingMode::TowardZero,
        RoundingMode::TowardPositive,
        RoundingMode::TowardNegative,
        RoundingMode::NearestAway,
    ][usize::from(index)]
}

macro_rules! width_checks {
    ($bits:ty, $float:ty, $width:expr, $fraction:expr) => {
        const EXPONENT_ONES: $bits = ((1 as $bits) << (<$bits>::BITS - 1 - $fraction)) - 1;
        const FRACTION_MASK: $bits = ((1 as $bits) << $fraction) - 1;
        const SIGN: $bits = (1 as $bits) << (<$bits>::BITS - 1);
        const QUIET: $bits = (1 as $bits) << ($fraction - 1);

        pub(super) fn is_nan(bits: $bits) -> bool {
            (bits >> $fraction) & EXPONENT_ONES == EXPONENT_ONES && bits & FRACTION_MASK != 0
        }

        pub(super) fn is_infinite(bits: $bits) -> bool {
            (bits >> $fraction) & EXPONENT_ONES == EXPONENT_ONES && bits & FRACTION_MASK == 0
        }

        pub(super) fn is_zero(bits: $bits) -> bool {
            bits & !SIGN == 0
        }

        pub(super) fn is_negative(bits: $bits) -> bool {
            bits & SIGN != 0
        }

        fn is_signaling(bits: $bits) -> bool {
            is_nan(bits) && bits & QUIET == 0
        }

        /// The leftmost operand NaN quieted, else the canonical NaN.
        fn expected_nan(operands: &[$bits]) -> $bits {
            let mut nan: $bits = (EXPONENT_ONES << $fraction) | QUIET;
            let mut seen = false;
            let mut index = 0;
            while index < operands.len() {
                if !seen && is_nan(operands[index]) {
                    nan = operands[index] | QUIET;
                    seen = true;
                }
                index += 1;
            }
            nan
        }

        // Whether the operation's result is a NaN, decided from the bits.
        pub(super) fn nan_add(a: $bits, b: $bits) -> bool {
            is_nan(a)
                || is_nan(b)
                || (is_infinite(a) && is_infinite(b) && is_negative(a) != is_negative(b))
        }

        pub(super) fn nan_subtract(a: $bits, b: $bits) -> bool {
            is_nan(a)
                || is_nan(b)
                || (is_infinite(a) && is_infinite(b) && is_negative(a) == is_negative(b))
        }

        pub(super) fn nan_multiply(a: $bits, b: $bits) -> bool {
            is_nan(a)
                || is_nan(b)
                || (is_infinite(a) && is_zero(b))
                || (is_zero(a) && is_infinite(b))
        }

        pub(super) fn nan_divide(a: $bits, b: $bits) -> bool {
            is_nan(a)
                || is_nan(b)
                || (is_zero(a) && is_zero(b))
                || (is_infinite(a) && is_infinite(b))
        }

        pub(super) fn nan_square_root(a: $bits) -> bool {
            is_nan(a) || (is_negative(a) && !is_zero(a))
        }

        pub(super) fn nan_fused_multiply_add(a: $bits, b: $bits, c: $bits) -> bool {
            let product_infinite = is_infinite(a) || is_infinite(b);
            is_nan(a)
                || is_nan(b)
                || is_nan(c)
                || (is_infinite(a) && is_zero(b))
                || (is_zero(a) && is_infinite(b))
                || (product_infinite
                    && is_infinite(c)
                    && is_negative(c) != (is_negative(a) != is_negative(b)))
        }

        /// The significand and exponent of a finite value: `m * 2^e`.
        fn decode(bits: $bits) -> (u128, i32) {
            let fraction = (bits & FRACTION_MASK) as u128;
            let field = ((bits >> $fraction) & EXPONENT_ONES) as i32;
            let bias = (EXPONENT_ONES >> 1) as i32;
            if field == 0 {
                (fraction, 1 - bias - $fraction)
            } else {
                (fraction | (1u128 << $fraction), field - bias - $fraction)
            }
        }

        /// Orders `mx * 2^ex` against `b^2 * 2^(2 eb)` in exact integers.
        fn compare_to_square(mx: u128, ex: i32, b: u128, eb: i32) -> core::cmp::Ordering {
            use core::cmp::Ordering;
            let square = b * b;
            let shift = ex - 2 * eb;
            if shift >= 0 {
                if shift as u32 > mx.leading_zeros() {
                    Ordering::Greater
                } else {
                    (mx << shift).cmp(&square)
                }
            } else if (-shift) as u32 > square.leading_zeros() {
                Ordering::Less
            } else {
                mx.cmp(&(square << (-shift)))
            }
        }

        /// Whether `r` is the nearest-even square root of the positive finite
        /// nonzero `x`: `x` lies between the squares of the two midpoints that
        /// bound `r`, a midpoint belonging to `r` only when `r` is even.
        fn square_root_is_nearest_even(x: $bits, r: $bits) -> bool {
            use core::cmp::Ordering;
            let (mx, ex) = decode(x);
            let (mr, er) = decode(r);
            if mr == 0 || is_nan(r) || is_infinite(r) {
                return false;
            }
            let emin = 1 - (EXPONENT_ONES >> 1) as i32 - $fraction;
            let (lower, lower_exp) = if mr == (1u128 << $fraction) && er > emin {
                (4 * mr - 1, er - 2)
            } else {
                (2 * mr - 1, er - 1)
            };
            let even = mr % 2 == 0;
            let above = match compare_to_square(mx, ex, lower, lower_exp) {
                Ordering::Greater => true,
                Ordering::Equal => even,
                Ordering::Less => false,
            };
            let below = match compare_to_square(mx, ex, 2 * mr + 1, er - 1) {
                Ordering::Less => true,
                Ordering::Equal => even,
                Ordering::Greater => false,
            };
            above && below
        }

        /// The square root's reference: zero and infinity map to themselves;
        /// every other non-NaN operand is bracketed by `square_root_is_nearest_even`.
        pub(super) fn square_root_reference_holds(x: $bits, r: $bits) -> bool {
            if is_zero(x) || is_infinite(x) {
                r == x
            } else {
                square_root_is_nearest_even(x, r)
            }
        }

        /// Negation is bitwise sign negation and raises no flag (FR-394).
        pub(super) fn check_negate(
            rounding: RoundingMode,
            outcome: Result<
                Outcome<crate::native_float::NativeFloatResult>,
                crate::comparison::IllTyped,
            >,
            operand: $bits,
        ) {
            match outcome {
                Ok(Outcome::Completed(result)) => {
                    kani::cover!(is_nan(operand), "a NaN operand is negated");
                    assert_eq!(result.value.width(), $width);
                    assert_eq!(result.rounding, rounding);
                    assert_eq!(result.value.bits() as $bits, operand ^ SIGN);
                    assert!(result.flags.is_empty());
                }
                _ => panic!("negation always completes"),
            }
        }

        /// `expected` is the host operation's bits, or `None` when the result
        /// is a NaN (decided by the predicates above).
        pub(super) fn check(
            rounding: RoundingMode,
            outcome: Result<
                Outcome<crate::native_float::NativeFloatResult>,
                crate::comparison::IllTyped,
            >,
            meter: &Meter,
            operands: &[$bits],
            expected: Option<$bits>,
        ) {
            let mut any_nan = false;
            let mut any_signaling = false;
            let mut index = 0;
            while index < operands.len() {
                any_nan |= is_nan(operands[index]);
                any_signaling |= is_signaling(operands[index]);
                index += 1;
            }
            kani::cover!(any_nan, "an operand is a NaN");
            kani::cover!(expected.is_none(), "a NaN result is reached");
            kani::cover!(expected.is_some(), "a numeric result is reached");
            match outcome {
                Ok(Outcome::Completed(result)) => {
                    assert_eq!(result.value.width(), $width);
                    assert_eq!(result.rounding, rounding);
                    let bits = result.value.bits() as $bits;
                    match expected {
                        None => {
                            assert_eq!(bits, expected_nan(operands));
                            assert_eq!(
                                result.flags.contains(IeeeFlag::Invalid),
                                any_signaling || !any_nan
                            );
                        }
                        Some(host) => {
                            assert!(!is_nan(bits));
                            assert!(!result.flags.contains(IeeeFlag::Invalid));
                            if rounding == RoundingMode::NearestEven
                                || rounding == RoundingMode::Exact
                            {
                                assert_eq!(bits, host);
                            }
                        }
                    }
                    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
                }
                Ok(Outcome::Refused(_)) => {
                    assert_eq!(rounding, RoundingMode::Exact);
                    assert!(expected.is_some());
                }
                _ => panic!("a generous meter reports neither incomplete nor undefined"),
            }
        }
    };
}

mod w32 {
    use super::*;
    width_checks!(u32, f32, IeeeWidth::Binary32, 23);
}

mod w64 {
    use super::*;
    width_checks!(u64, f64, IeeeWidth::Binary64, 52);
}

macro_rules! arithmetic_harness {
    ($name:ident, $module:ident, $bits:ty, $float:ty, $carrier:ident, $nan:ident,
     $rounding:expr, |$($x:ident),+| $operation:expr, $host:expr) => {
        #[kani::proof]
        #[kani::unwind(57)]
        fn $name() {
            $(let $x: $bits = kani::any();)+
            let rounding: RoundingMode = $rounding;
            let mut meter = generous_meter();
            let operation = {
                $(let $x = NativeFloat::$carrier($x);)+
                $operation
            };
            let outcome = evaluate_native_float(operation, rounding, &mut meter);
            let expected: Option<$bits> = if $module::$nan($($x),+) {
                None
            } else {
                Some({
                    $(let $x = <$float>::from_bits($x);)+
                    let host: $float = $host;
                    host.to_bits()
                })
            };
            $module::check(rounding, outcome, &meter, &[$($x),+], expected);
        }
    };
}

/// The square root harness: the expected side is the exact-integer bracket of
/// the correctly rounded root (Kani's own model of the host `sqrt` is not
/// bit-exact, so the host operation is not used here).
macro_rules! square_root_harness {
    ($name:ident, $module:ident, $bits:ty, $carrier:ident, $rounding:expr) => {
        #[kani::proof]
        #[kani::unwind(57)]
        fn $name() {
            let a: $bits = kani::any();
            let rounding: RoundingMode = $rounding;
            let mut meter = generous_meter();
            let outcome = evaluate_native_float(
                NativeFloatOperation::SquareRoot(NativeFloat::$carrier(a)),
                rounding,
                &mut meter,
            );
            let expected: Option<$bits> = if $module::nan_square_root(a) {
                None
            } else {
                Some(match &outcome {
                    Ok(Outcome::Completed(result)) => {
                        let r = result.value.bits() as $bits;
                        if rounding == RoundingMode::NearestEven || rounding == RoundingMode::Exact
                        {
                            assert!($module::square_root_reference_holds(a, r));
                        }
                        r
                    }
                    _ => 0,
                })
            };
            $module::check(rounding, outcome, &meter, &[a], expected);
        }
    };
}

macro_rules! square_root_trio {
    ($nearest_even:ident, $exact:ident, $directed:ident, $module:ident, $bits:ty, $carrier:ident) => {
        square_root_harness!(
            $nearest_even,
            $module,
            $bits,
            $carrier,
            RoundingMode::NearestEven
        );
        square_root_harness!($exact, $module, $bits, $carrier, RoundingMode::Exact);
        square_root_harness!($directed, $module, $bits, $carrier, any_directed());
    };
}

/// The three harnesses of one operation and width: `NearestEven`, `Exact`,
/// and the four directed and nearest-away spellings.
macro_rules! arithmetic_trio {
    ($nearest_even:ident, $exact:ident, $directed:ident, $module:ident, $bits:ty,
     $float:ty, $carrier:ident, $nan:ident, |$($x:ident),+| $operation:expr, $host:expr) => {
        arithmetic_harness!($nearest_even, $module, $bits, $float, $carrier, $nan,
            RoundingMode::NearestEven, |$($x),+| $operation, $host);
        arithmetic_harness!($exact, $module, $bits, $float, $carrier, $nan,
            RoundingMode::Exact, |$($x),+| $operation, $host);
        arithmetic_harness!($directed, $module, $bits, $float, $carrier, $nan,
            any_directed(), |$($x),+| $operation, $host);
    };
}

arithmetic_trio!(
    float32_add_nearest_even,
    float32_add_exact,
    float32_add_directed,
    w32,
    u32,
    f32,
    binary32,
    nan_add,
    |a, b| NativeFloatOperation::Add(a, b),
    a + b
);
arithmetic_trio!(
    float32_subtract_nearest_even,
    float32_subtract_exact,
    float32_subtract_directed,
    w32,
    u32,
    f32,
    binary32,
    nan_subtract,
    |a, b| NativeFloatOperation::Subtract(a, b),
    a - b
);
arithmetic_trio!(
    float32_multiply_nearest_even,
    float32_multiply_exact,
    float32_multiply_directed,
    w32,
    u32,
    f32,
    binary32,
    nan_multiply,
    |a, b| NativeFloatOperation::Multiply(a, b),
    a * b
);
arithmetic_trio!(
    float32_divide_nearest_even,
    float32_divide_exact,
    float32_divide_directed,
    w32,
    u32,
    f32,
    binary32,
    nan_divide,
    |a, b| NativeFloatOperation::Divide(a, b),
    {
        if a.is_infinite() {
            if a.is_sign_negative() != b.is_sign_negative() {
                -f32::INFINITY
            } else {
                f32::INFINITY
            }
        } else if b.is_infinite() {
            if a.is_sign_negative() != b.is_sign_negative() {
                -0.0f32
            } else {
                0.0f32
            }
        } else {
            a / b
        }
    }
);
square_root_trio!(
    float32_square_root_nearest_even,
    float32_square_root_exact,
    float32_square_root_directed,
    w32,
    u32,
    binary32
);
arithmetic_trio!(
    float32_fused_multiply_add_nearest_even,
    float32_fused_multiply_add_exact,
    float32_fused_multiply_add_directed,
    w32,
    u32,
    f32,
    binary32,
    nan_fused_multiply_add,
    |a, b, c| NativeFloatOperation::FusedMultiplyAdd(a, b, c),
    a.mul_add(b, c)
);
arithmetic_trio!(
    float64_add_nearest_even,
    float64_add_exact,
    float64_add_directed,
    w64,
    u64,
    f64,
    binary64,
    nan_add,
    |a, b| NativeFloatOperation::Add(a, b),
    a + b
);
arithmetic_trio!(
    float64_subtract_nearest_even,
    float64_subtract_exact,
    float64_subtract_directed,
    w64,
    u64,
    f64,
    binary64,
    nan_subtract,
    |a, b| NativeFloatOperation::Subtract(a, b),
    a - b
);
arithmetic_trio!(
    float64_multiply_nearest_even,
    float64_multiply_exact,
    float64_multiply_directed,
    w64,
    u64,
    f64,
    binary64,
    nan_multiply,
    |a, b| NativeFloatOperation::Multiply(a, b),
    a * b
);
arithmetic_trio!(
    float64_divide_nearest_even,
    float64_divide_exact,
    float64_divide_directed,
    w64,
    u64,
    f64,
    binary64,
    nan_divide,
    |a, b| NativeFloatOperation::Divide(a, b),
    {
        if a.is_infinite() {
            if a.is_sign_negative() != b.is_sign_negative() {
                -f64::INFINITY
            } else {
                f64::INFINITY
            }
        } else if b.is_infinite() {
            if a.is_sign_negative() != b.is_sign_negative() {
                -0.0f64
            } else {
                0.0f64
            }
        } else {
            a / b
        }
    }
);
square_root_trio!(
    float64_square_root_nearest_even,
    float64_square_root_exact,
    float64_square_root_directed,
    w64,
    u64,
    binary64
);
arithmetic_trio!(
    float64_fused_multiply_add_nearest_even,
    float64_fused_multiply_add_exact,
    float64_fused_multiply_add_directed,
    w64,
    u64,
    f64,
    binary64,
    nan_fused_multiply_add,
    |a, b, c| NativeFloatOperation::FusedMultiplyAdd(a, b, c),
    a.mul_add(b, c)
);

/// Comparisons. Expected side: the host's `==` for `NumericEqual`, bit
/// equality of the patterns for `BitIdentical`, and the sign-magnitude order
/// written as an `i64` fold for `TotalOrder` (covers the boolean and the
/// result-unit charge; the charge amounts are left to the host differential).
macro_rules! comparison_harness {
    ($name:ident, $bits:ty, $signed:ty, $float:ty, $carrier:ident, $comparison:expr, $expected:expr) => {
        #[kani::proof]
        #[kani::unwind(3)]
        fn $name() {
            let (a, b): ($bits, $bits) = (kani::any(), kani::any());
            let mut meter = generous_meter();
            let outcome = compare_native_float(
                $comparison,
                NativeFloat::$carrier(a),
                NativeFloat::$carrier(b),
                &mut meter,
            );
            let (fa, fb) = (<$float>::from_bits(a), <$float>::from_bits(b));
            let expected: bool = ($expected)(a, b, fa, fb);
            kani::cover!(expected, "the comparison holds");
            kani::cover!(!expected, "the comparison fails");
            match outcome {
                Ok(Outcome::Completed(value)) => assert_eq!(value, expected),
                _ => panic!("a generous meter reports neither incomplete nor undefined"),
            }
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
        }
    };
}

/// `totalOrder(a, b)` as a signed-integer comparison: a negative pattern
/// sorts by its complement (the host's `total_cmp` rule).
fn total_order_32(a: u32, b: u32) -> bool {
    let (mut x, mut y) = (a as i32, b as i32);
    x ^= (((x >> 31) as u32) >> 1) as i32;
    y ^= (((y >> 31) as u32) >> 1) as i32;
    x <= y
}

fn total_order_64(a: u64, b: u64) -> bool {
    let (mut x, mut y) = (a as i64, b as i64);
    x ^= (((x >> 63) as u64) >> 1) as i64;
    y ^= (((y >> 63) as u64) >> 1) as i64;
    x <= y
}

comparison_harness!(
    float32_numeric_equal,
    u32,
    i32,
    f32,
    binary32,
    IeeeComparison::NumericEqual,
    |_a: u32, _b: u32, fa: f32, fb: f32| fa == fb
);
comparison_harness!(
    float32_bit_identical,
    u32,
    i32,
    f32,
    binary32,
    IeeeComparison::BitIdentical,
    |a: u32, b: u32, _fa: f32, _fb: f32| a == b
);
comparison_harness!(
    float32_total_order,
    u32,
    i32,
    f32,
    binary32,
    IeeeComparison::TotalOrder,
    |a: u32, b: u32, _fa: f32, _fb: f32| total_order_32(a, b)
);
comparison_harness!(
    float64_numeric_equal,
    u64,
    i64,
    f64,
    binary64,
    IeeeComparison::NumericEqual,
    |_a: u64, _b: u64, fa: f64, fb: f64| fa == fb
);
comparison_harness!(
    float64_bit_identical,
    u64,
    i64,
    f64,
    binary64,
    IeeeComparison::BitIdentical,
    |a: u64, b: u64, _fa: f64, _fb: f64| a == b
);
comparison_harness!(
    float64_total_order,
    u64,
    i64,
    f64,
    binary64,
    IeeeComparison::TotalOrder,
    |a: u64, b: u64, _fa: f64, _fb: f64| total_order_64(a, b)
);

macro_rules! negate_harness {
    ($name:ident, $module:ident, $bits:ty, $carrier:ident) => {
        #[kani::proof]
        #[kani::unwind(4)]
        fn $name() {
            let operand: $bits = kani::any();
            let rounding = any_rounding();
            let outcome = evaluate_native_float(
                NativeFloatOperation::Negate(NativeFloat::$carrier(operand)),
                rounding,
                &mut generous_meter(),
            );
            $module::check_negate(rounding, outcome, operand);
        }
    };
}

negate_harness!(float32_negate, w32, u32, binary32);
negate_harness!(float64_negate, w64, u64, binary64);
