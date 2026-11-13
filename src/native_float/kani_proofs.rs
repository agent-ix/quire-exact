// SPDX-License-Identifier: AGPL-3.0-or-later
//! Kani harnesses of FR-405 Behavior 19: the native float operations over
//! symbolic bit patterns against an expected side that is the host language's
//! own `f32`/`f64` arithmetic (an independent reference that shares no
//! function with the native body and not even the bit-pattern decode).
//!
//! Expected side. For `NearestEven` and strict `Exact` (whose completed result
//! is the nearest-even one), a completed non-NaN result equals the host
//! operation's bits. A NaN result equals the leftmost operand NaN quieted, or
//! the canonical NaN when no operand is a NaN.
//!
//! Covers: result bits under `NearestEven` and `Exact` (when completed), NaN
//! results (sign, quietness, payload) under every spelling, the `invalid` flag
//! of NaN-producing operations, that only `Exact` refuses, the provenance and
//! the result-unit charge.
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

fn any_rounding() -> RoundingMode {
    let index: u8 = kani::any();
    kani::assume(index < 6);
    RoundingMode::ALL[usize::from(index)]
}

macro_rules! width_checks {
    ($check:ident, $compare:ident, $bits:ty, $float:ty, $width:expr, $fraction:expr, $nan_bits:ty) => {
        const _: () = assert!(<$bits>::BITS > 0);

        fn is_nan(bits: $bits) -> bool {
            let exponent: $bits = ((1 as $bits) << (<$bits>::BITS - 1 - $fraction)) - 1;
            let fraction: $bits = ((1 as $bits) << $fraction) - 1;
            (bits >> $fraction) & exponent == exponent && bits & fraction != 0
        }

        fn is_signaling(bits: $bits) -> bool {
            is_nan(bits) && bits & ((1 as $bits) << ($fraction - 1)) == 0
        }

        /// The leftmost operand NaN quieted, else the canonical NaN.
        fn expected_nan(operands: &[$bits]) -> $bits {
            let quiet: $bits = (1 as $bits) << ($fraction - 1);
            let mut nan: $bits =
                ((((1 as $bits) << (<$bits>::BITS - 1 - $fraction)) - 1) << $fraction) | quiet;
            let mut seen = false;
            let mut index = 0;
            while index < operands.len() {
                if !seen && is_nan(operands[index]) {
                    nan = operands[index] | quiet;
                    seen = true;
                }
                index += 1;
            }
            nan
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
            let sign: $bits = (1 as $bits) << (<$bits>::BITS - 1);
            match outcome {
                Ok(Outcome::Completed(result)) => {
                    kani::cover!(is_nan(operand), "a NaN operand is negated");
                    assert_eq!(result.value.width(), $width);
                    assert_eq!(result.rounding, rounding);
                    assert_eq!(result.value.bits() as $bits, operand ^ sign);
                    assert!(result.flags.is_empty());
                }
                _ => panic!("negation always completes"),
            }
        }

        pub(super) fn $check(
            rounding: RoundingMode,
            outcome: Result<
                Outcome<crate::native_float::NativeFloatResult>,
                crate::comparison::IllTyped,
            >,
            meter: &Meter,
            operands: &[$bits],
            host: $float,
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
            kani::cover!(!any_nan, "no operand is a NaN");
            match outcome {
                Ok(Outcome::Completed(result)) => {
                    kani::cover!(!host.is_nan(), "a numeric result is reached");
                    kani::cover!(host.is_nan(), "a NaN result is reached");
                    assert_eq!(result.value.width(), $width);
                    assert_eq!(result.rounding, rounding);
                    let bits = result.value.bits() as $bits;
                    if host.is_nan() {
                        assert_eq!(bits, expected_nan(operands));
                        assert_eq!(
                            result.flags.contains(IeeeFlag::Invalid),
                            any_signaling || !any_nan
                        );
                    } else if rounding == RoundingMode::NearestEven
                        || rounding == RoundingMode::Exact
                    {
                        assert_eq!(bits, host.to_bits());
                    }
                    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
                }
                Ok(Outcome::Refused(_)) => assert_eq!(rounding, RoundingMode::Exact),
                _ => panic!("a generous meter reports neither incomplete nor undefined"),
            }
        }
    };
}

mod w32 {
    use super::*;
    width_checks!(check, compare, u32, f32, IeeeWidth::Binary32, 23, u32);
}

mod w64 {
    use super::*;
    width_checks!(check, compare, u64, f64, IeeeWidth::Binary64, 52, u64);
}

macro_rules! arithmetic_harness {
    ($name:ident, $module:ident, $bits:ty, $float:ty, $carrier:ident,
     |$($x:ident),+| $operation:expr, $host:expr) => {
        #[kani::proof]
        #[kani::unwind(57)]
        fn $name() {
            $(let $x: $bits = kani::any();)+
            let rounding = any_rounding();
            let mut meter = generous_meter();
            $(let $x = (NativeFloat::$carrier($x), <$float>::from_bits($x));)+
            let operation = $operation;
            let outcome = evaluate_native_float(operation, rounding, &mut meter);
            let host: $float = $host;
            $module::check(rounding, outcome, &meter, &[$($x.0.bits() as $bits),+], host);
        }
    };
}

arithmetic_harness!(
    float32_add,
    w32,
    u32,
    f32,
    binary32,
    |a, b| NativeFloatOperation::Add(a.0, b.0),
    a.1 + b.1
);
arithmetic_harness!(
    float32_subtract,
    w32,
    u32,
    f32,
    binary32,
    |a, b| NativeFloatOperation::Subtract(a.0, b.0),
    a.1 - b.1
);
arithmetic_harness!(
    float32_multiply,
    w32,
    u32,
    f32,
    binary32,
    |a, b| NativeFloatOperation::Multiply(a.0, b.0),
    a.1 * b.1
);
arithmetic_harness!(
    float32_divide,
    w32,
    u32,
    f32,
    binary32,
    |a, b| NativeFloatOperation::Divide(a.0, b.0),
    a.1 / b.1
);
arithmetic_harness!(
    float32_square_root,
    w32,
    u32,
    f32,
    binary32,
    |a| NativeFloatOperation::SquareRoot(a.0),
    a.1.sqrt()
);
arithmetic_harness!(
    float32_fused_multiply_add,
    w32,
    u32,
    f32,
    binary32,
    |a, b, c| NativeFloatOperation::FusedMultiplyAdd(a.0, b.0, c.0),
    a.1.mul_add(b.1, c.1)
);
arithmetic_harness!(
    float64_add,
    w64,
    u64,
    f64,
    binary64,
    |a, b| NativeFloatOperation::Add(a.0, b.0),
    a.1 + b.1
);
arithmetic_harness!(
    float64_subtract,
    w64,
    u64,
    f64,
    binary64,
    |a, b| NativeFloatOperation::Subtract(a.0, b.0),
    a.1 - b.1
);
arithmetic_harness!(
    float64_multiply,
    w64,
    u64,
    f64,
    binary64,
    |a, b| NativeFloatOperation::Multiply(a.0, b.0),
    a.1 * b.1
);
arithmetic_harness!(
    float64_divide,
    w64,
    u64,
    f64,
    binary64,
    |a, b| NativeFloatOperation::Divide(a.0, b.0),
    a.1 / b.1
);
arithmetic_harness!(
    float64_square_root,
    w64,
    u64,
    f64,
    binary64,
    |a| NativeFloatOperation::SquareRoot(a.0),
    a.1.sqrt()
);
arithmetic_harness!(
    float64_fused_multiply_add,
    w64,
    u64,
    f64,
    binary64,
    |a, b, c| NativeFloatOperation::FusedMultiplyAdd(a.0, b.0, c.0),
    a.1.mul_add(b.1, c.1)
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
