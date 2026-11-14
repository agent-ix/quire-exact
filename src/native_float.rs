// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani-tractable native IEEE float carrier (FR-405).
//!
//! A [`NativeFloat`] is a width tag and a bit pattern in a `u64`. It is
//! `Copy`, owns no heap type and has no drop glue. [`evaluate_native_float`]
//! and [`compare_native_float`] reimplement the owner's
//! [`crate::evaluate_ieee`] and [`crate::compare_ieee`] with `u128`/`i32`
//! arithmetic only: no `Integer`, `Rational`, arbitrary-precision item,
//! allocation or iterator adapter, and one body for host and Kani builds.
//!
//! Every loop below has a stated constant bound and no operand-dependent
//! `break`, `return` or `?`: the operand loops run `N` times (`N` is the
//! operation arity, a const generic) and the square-root loop runs
//! [`SQRT_STEPS`] times.
//!
//! The exact intermediate is `(mag + e) * 2^exp` for a nonzero `u128` `mag`,
//! with `0 < e < 1` exactly when `sticky`; whenever `sticky` the magnitude
//! holds at least `fraction_bits + 3` bits, so the rounding quantum and two
//! guard bits lie inside `mag`.

use crate::accounting::{Charge, ChargePoint, LimitKind, Meter};
use crate::comparison::{IllTyped, IllTypedCause};
use crate::decimal::RoundingMode;
use crate::ieee::{IeeeComparison, IeeeFlag, IeeeFlags, IeeeValue, IeeeWidth};
use crate::outcome::{Outcome, Refusal, Stop};

/// The bit the normalised operand magnitude of a sum is placed at.
const WIDE_TOP: u32 = 125;

/// The constant trip count of the integer square root: the root of a radicand
/// below `2^112` is below `2^56`.
const SQRT_STEPS: u32 = 56;

/// The refusal of [`NativeFloat::new`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeFloatRefusal {
    /// The bit pattern does not fit the width (a `Binary32` pattern above
    /// `u32::MAX`). It is never masked, wrapped or saturated.
    #[error("bit pattern is outside the width of the float carrier")]
    OutsideCarrierDomain,
}

/// A carried IEEE float: width and exact bit pattern, every NaN included.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeFloat {
    width: IeeeWidth,
    bits: u64,
}

impl NativeFloat {
    /// Build a carried float. Takes no `Meter` and charges nothing.
    pub const fn new(width: IeeeWidth, bits: u64) -> Result<Self, NativeFloatRefusal> {
        match width {
            IeeeWidth::Binary32 if bits > u32::MAX as u64 => {
                Err(NativeFloatRefusal::OutsideCarrierDomain)
            }
            _ => Ok(Self { width, bits }),
        }
    }

    /// A `Binary32` float from its exact bits.
    pub const fn binary32(bits: u32) -> Self {
        Self {
            width: IeeeWidth::Binary32,
            bits: bits as u64,
        }
    }

    /// A `Binary64` float from its exact bits.
    pub const fn binary64(bits: u64) -> Self {
        Self {
            width: IeeeWidth::Binary64,
            bits,
        }
    }

    /// The width.
    pub const fn width(self) -> IeeeWidth {
        self.width
    }

    /// The exact bit pattern.
    pub const fn bits(self) -> u64 {
        self.bits
    }

    /// The owner's value with the same width and bits (host-side conversion).
    pub fn to_ieee_value(self) -> IeeeValue {
        match self.width {
            IeeeWidth::Binary32 => IeeeValue::binary32(self.bits as u32),
            IeeeWidth::Binary64 => IeeeValue::binary64(self.bits),
        }
    }
}

impl From<IeeeValue> for NativeFloat {
    fn from(value: IeeeValue) -> Self {
        Self {
            width: value.width(),
            bits: value.bits(),
        }
    }
}

impl From<NativeFloat> for IeeeValue {
    fn from(value: NativeFloat) -> Self {
        value.to_ieee_value()
    }
}

/// A native arithmetic operation (the seven of FR-405 Behavior 9).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeFloatOperation {
    /// Flip the sign bit.
    Negate(NativeFloat),
    /// `a + b`.
    Add(NativeFloat, NativeFloat),
    /// `a - b`.
    Subtract(NativeFloat, NativeFloat),
    /// `a * b`.
    Multiply(NativeFloat, NativeFloat),
    /// `a / b`.
    Divide(NativeFloat, NativeFloat),
    /// The square root of `a`.
    SquareRoot(NativeFloat),
    /// `a * b + c` with one rounding.
    FusedMultiplyAdd(NativeFloat, NativeFloat, NativeFloat),
}

/// A completed native result: bits, fresh flags and the rounding provenance
/// (the width is the value's).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeFloatResult {
    /// The result bits.
    pub value: NativeFloat,
    /// The operation-local flags.
    pub flags: IeeeFlags,
    /// The selected rounding direction or strict `exact` policy.
    pub rounding: RoundingMode,
}

/// Evaluate one native operation under `rounding`. Operands of different
/// widths are ill-typed and consume nothing.
pub fn evaluate_native_float(
    operation: NativeFloatOperation,
    rounding: RoundingMode,
    meter: &mut Meter,
) -> Result<Outcome<NativeFloatResult>, IllTyped> {
    let width = match operation {
        NativeFloatOperation::Negate(a) | NativeFloatOperation::SquareRoot(a) => a.width,
        NativeFloatOperation::Add(a, b)
        | NativeFloatOperation::Subtract(a, b)
        | NativeFloatOperation::Multiply(a, b)
        | NativeFloatOperation::Divide(a, b) => {
            same_width(a.width, b.width)?;
            a.width
        }
        NativeFloatOperation::FusedMultiplyAdd(a, b, c) => {
            same_width(a.width, b.width)?;
            same_width(a.width, c.width)?;
            a.width
        }
    };
    Ok(Outcome::from_stop(arithmetic(
        operation, width, rounding, meter,
    )))
}

/// Evaluate one native comparison. Cross-width operands are ill-typed and
/// consume nothing.
pub fn compare_native_float(
    comparison: IeeeComparison,
    left: NativeFloat,
    right: NativeFloat,
    meter: &mut Meter,
) -> Result<Outcome<bool>, IllTyped> {
    same_width(left.width, right.width)?;
    Ok(Outcome::from_stop(compare(comparison, left, right, meter)))
}

fn same_width(first: IeeeWidth, second: IeeeWidth) -> Result<(), IllTyped> {
    if first == second {
        Ok(())
    } else {
        Err(IllTyped {
            cause: IllTypedCause::DistinctIeeeWidths,
        })
    }
}

// ---- format ------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Fmt {
    exponent_bits: u32,
    fraction_bits: u32,
    width_bits: u64,
}

impl Fmt {
    const fn of(width: IeeeWidth) -> Self {
        match width {
            IeeeWidth::Binary32 => Self {
                exponent_bits: 8,
                fraction_bits: 23,
                width_bits: 32,
            },
            IeeeWidth::Binary64 => Self {
                exponent_bits: 11,
                fraction_bits: 52,
                width_bits: 64,
            },
        }
    }
    const fn fraction(self) -> i32 {
        self.fraction_bits as i32
    }
    const fn bias(self) -> i32 {
        (1 << (self.exponent_bits - 1)) - 1
    }
    /// Exponent of the least subnormal quantum.
    const fn quantum_min(self) -> i32 {
        1 - self.bias() - self.fraction()
    }
    const fn emin(self) -> i32 {
        1 - self.bias()
    }
    const fn sign_mask(self) -> u64 {
        1 << (self.exponent_bits + self.fraction_bits)
    }
    const fn exponent_ones(self) -> u64 {
        (1 << self.exponent_bits) - 1
    }
    const fn exponent_mask(self) -> u64 {
        self.exponent_ones() << self.fraction_bits
    }
    const fn fraction_mask(self) -> u64 {
        (1 << self.fraction_bits) - 1
    }
    const fn quiet_bit(self) -> u64 {
        1 << (self.fraction_bits - 1)
    }
    const fn sign(self, negative: bool) -> u64 {
        if negative {
            self.sign_mask()
        } else {
            0
        }
    }
    const fn infinity(self, negative: bool) -> u64 {
        self.sign(negative) | self.exponent_mask()
    }
    const fn max_finite(self, negative: bool) -> u64 {
        self.sign(negative)
            | (self.exponent_mask() - (1 << self.fraction_bits))
            | self.fraction_mask()
    }
    const fn canonical_nan(self) -> u64 {
        self.exponent_mask() | self.quiet_bit()
    }
}

// ---- operands ----------------------------------------------------------------

#[derive(Clone, Copy)]
enum Operand {
    Nan {
        signaling: bool,
    },
    Infinite {
        negative: bool,
    },
    Zero {
        negative: bool,
    },
    /// `(-1)^negative * significand * 2^exponent`, significand nonzero.
    Finite {
        negative: bool,
        significand: u64,
        exponent: i32,
    },
}

fn decode(fmt: Fmt, bits: u64) -> Operand {
    let negative = bits & fmt.sign_mask() != 0;
    let biased = (bits >> fmt.fraction_bits) & fmt.exponent_ones();
    let fraction = bits & fmt.fraction_mask();
    if biased == fmt.exponent_ones() {
        if fraction == 0 {
            Operand::Infinite { negative }
        } else {
            Operand::Nan {
                signaling: fraction & fmt.quiet_bit() == 0,
            }
        }
    } else if biased == 0 {
        if fraction == 0 {
            Operand::Zero { negative }
        } else {
            Operand::Finite {
                negative,
                significand: fraction,
                exponent: fmt.quantum_min(),
            }
        }
    } else {
        Operand::Finite {
            negative,
            significand: fraction | (1 << fmt.fraction_bits),
            exponent: biased as i32 - fmt.bias() - fmt.fraction(),
        }
    }
}

impl Operand {
    fn negative(self) -> bool {
        match self {
            Self::Infinite { negative }
            | Self::Zero { negative }
            | Self::Finite { negative, .. } => negative,
            Self::Nan { .. } => false,
        }
    }

    fn negated(self) -> Self {
        match self {
            Self::Infinite { negative } => Self::Infinite {
                negative: !negative,
            },
            Self::Zero { negative } => Self::Zero {
                negative: !negative,
            },
            Self::Finite {
                negative,
                significand,
                exponent,
            } => Self::Finite {
                negative: !negative,
                significand,
                exponent,
            },
            Self::Nan { signaling } => Self::Nan { signaling },
        }
    }
}

// ---- accounting --------------------------------------------------------------

fn charge_operands(meter: &mut Meter, fmt: Fmt, arity: u64) -> Result<(), Stop> {
    meter.charge(
        Charge::new(ChargePoint::IeeeOperands)
            .size(LimitKind::IntegerBits, fmt.width_bits)
            .size(LimitKind::ValueOccurrences, arity),
    )?;
    Ok(())
}

fn charge_width(meter: &mut Meter, point: ChargePoint, fmt: Fmt) -> Result<(), Stop> {
    meter.charge(Charge::new(point).size(LimitKind::IntegerBits, fmt.width_bits))?;
    Ok(())
}

fn charge_result(meter: &mut Meter) -> Result<(), Stop> {
    meter.charge(Charge::new(ChargePoint::IeeeResultRetain).results(1))?;
    Ok(())
}

// ---- comparison --------------------------------------------------------------

/// The IEEE `totalOrder` key: sign-magnitude folded onto an unsigned order.
fn total_order_key(fmt: Fmt, bits: u64) -> u64 {
    if bits & fmt.sign_mask() != 0 {
        // Negative: reverse the order of the magnitudes below every positive.
        fmt.sign_mask() - 1 - (bits & (fmt.sign_mask() - 1))
    } else {
        fmt.sign_mask() + bits
    }
}

fn compare(
    comparison: IeeeComparison,
    left: NativeFloat,
    right: NativeFloat,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    let fmt = Fmt::of(left.width);
    charge_operands(meter, fmt, 2)?;
    let result = match comparison {
        IeeeComparison::NumericEqual => {
            let nan = matches!(decode(fmt, left.bits), Operand::Nan { .. })
                || matches!(decode(fmt, right.bits), Operand::Nan { .. });
            let both_zero = (left.bits | right.bits) & !fmt.sign_mask() == 0;
            !nan && (both_zero || left.bits == right.bits)
        }
        IeeeComparison::TotalOrder => {
            total_order_key(fmt, left.bits) <= total_order_key(fmt, right.bits)
        }
        IeeeComparison::BitIdentical => left.bits == right.bits,
    };
    charge_result(meter)?;
    Ok(result)
}

// ---- evaluation --------------------------------------------------------------

/// What classification or the exact intermediate decided.
enum Step {
    /// Decided by classification alone: bits and flags, no rounding.
    Decided(u64, IeeeFlags),
    /// A finite operation: charge `ieee.exact-intermediate` and `ieee.round`.
    Finite(Exact),
}

enum Exact {
    Zero {
        negative: bool,
    },
    Real {
        negative: bool,
        mag: u128,
        exp: i32,
        sticky: bool,
    },
}

fn invalid(fmt: Fmt) -> Step {
    Step::Decided(
        fmt.canonical_nan(),
        IeeeFlags::EMPTY.with(IeeeFlag::Invalid),
    )
}

fn decided(bits: u64) -> Step {
    Step::Decided(bits, IeeeFlags::EMPTY)
}

fn arithmetic(
    operation: NativeFloatOperation,
    width: IeeeWidth,
    rounding: RoundingMode,
    meter: &mut Meter,
) -> Result<NativeFloatResult, Stop> {
    let fmt = Fmt::of(width);
    let arity = match operation {
        NativeFloatOperation::Negate(..) | NativeFloatOperation::SquareRoot(..) => 1,
        NativeFloatOperation::FusedMultiplyAdd(..) => 3,
        _ => 2,
    };
    charge_operands(meter, fmt, arity)?;
    if let NativeFloatOperation::Negate(value) = operation {
        charge_result(meter)?;
        return Ok(NativeFloatResult {
            value: NativeFloat {
                width,
                bits: value.bits ^ fmt.sign_mask(),
            },
            flags: IeeeFlags::EMPTY,
            rounding,
        });
    }
    let step = match operation {
        NativeFloatOperation::Negate(_) => decided(0),
        NativeFloatOperation::Add(a, b) => classify(fmt, [a.bits, b.bits])
            .map_or_else(|d| d, |[a, b]| plan_add(fmt, rounding, a, b)),
        NativeFloatOperation::Subtract(a, b) => classify(fmt, [a.bits, b.bits])
            .map_or_else(|d| d, |[a, b]| plan_add(fmt, rounding, a, b.negated())),
        NativeFloatOperation::Multiply(a, b) => {
            classify(fmt, [a.bits, b.bits]).map_or_else(|d| d, |[a, b]| plan_multiply(fmt, a, b))
        }
        NativeFloatOperation::Divide(a, b) => {
            classify(fmt, [a.bits, b.bits]).map_or_else(|d| d, |[a, b]| plan_divide(fmt, a, b))
        }
        NativeFloatOperation::SquareRoot(a) => {
            classify(fmt, [a.bits]).map_or_else(|d| d, |[a]| plan_square_root(fmt, a))
        }
        NativeFloatOperation::FusedMultiplyAdd(a, b, c) => classify(fmt, [a.bits, b.bits, c.bits])
            .map_or_else(|d| d, |[a, b, c]| plan_fma(fmt, rounding, a, b, c)),
    };
    match step {
        Step::Decided(bits, flags) => {
            charge_result(meter)?;
            Ok(NativeFloatResult {
                value: NativeFloat { width, bits },
                flags,
                rounding,
            })
        }
        Step::Finite(exact) => {
            charge_width(meter, ChargePoint::IeeeExactIntermediate, fmt)?;
            charge_width(meter, ChargePoint::IeeeRound, fmt)?;
            let (bits, flags) = round(fmt, exact, rounding);
            // Strict `exact` refuses after `ieee.round`, before the retain.
            if rounding == RoundingMode::Exact && !flags.is_empty() {
                return Err(Stop::Refused(Refusal::IeeeNotExact {
                    target: width,
                    would_be: flags,
                }));
            }
            charge_result(meter)?;
            Ok(NativeFloatResult {
                value: NativeFloat { width, bits },
                flags,
                rounding,
            })
        }
    }
}

/// Decode operands in order. Any NaN decides the result: the leftmost NaN,
/// quieted with its sign and payload kept, raising `invalid` when any operand
/// is signaling.
fn classify<const N: usize>(fmt: Fmt, values: [u64; N]) -> Result<[Operand; N], Step> {
    let mut operands = [Operand::Zero { negative: false }; N];
    let mut nan_bits = 0_u64;
    let mut seen = false;
    let mut signaling_seen = false;
    let mut index = 0;
    // bound: N, the arity.
    while index < N {
        let operand = decode(fmt, values[index]);
        if let Operand::Nan { signaling } = operand {
            if !seen {
                nan_bits = values[index];
                seen = true;
            }
            signaling_seen |= signaling;
        } else {
            operands[index] = operand;
        }
        index += 1;
    }
    if seen {
        let flags = if signaling_seen {
            IeeeFlags::EMPTY.with(IeeeFlag::Invalid)
        } else {
            IeeeFlags::EMPTY
        };
        Err(Step::Decided(nan_bits | fmt.quiet_bit(), flags))
    } else {
        Ok(operands)
    }
}

fn plan_add(fmt: Fmt, rounding: RoundingMode, a: Operand, b: Operand) -> Step {
    match (a, b) {
        (Operand::Infinite { negative: x }, Operand::Infinite { negative: y }) => {
            if x == y {
                decided(fmt.infinity(x))
            } else {
                invalid(fmt)
            }
        }
        (Operand::Infinite { negative }, _) | (_, Operand::Infinite { negative }) => {
            decided(fmt.infinity(negative))
        }
        _ => Step::Finite(sum(rounding, term(a), term(b))),
    }
}

fn plan_multiply(fmt: Fmt, a: Operand, b: Operand) -> Step {
    let negative = a.negative() != b.negative();
    match (a, b) {
        (Operand::Infinite { .. }, Operand::Zero { .. })
        | (Operand::Zero { .. }, Operand::Infinite { .. }) => invalid(fmt),
        (Operand::Infinite { .. }, _) | (_, Operand::Infinite { .. }) => {
            decided(fmt.infinity(negative))
        }
        _ => {
            let product = product_term(a, b);
            Step::Finite(if product.mag == 0 {
                Exact::Zero { negative }
            } else {
                Exact::Real {
                    negative,
                    mag: product.mag,
                    exp: product.exp,
                    sticky: false,
                }
            })
        }
    }
}

fn plan_divide(fmt: Fmt, a: Operand, b: Operand) -> Step {
    let negative = a.negative() != b.negative();
    match (a, b) {
        (Operand::Infinite { .. }, Operand::Infinite { .. })
        | (Operand::Zero { .. }, Operand::Zero { .. }) => invalid(fmt),
        (Operand::Infinite { .. }, _) => decided(fmt.infinity(negative)),
        (_, Operand::Infinite { .. }) => decided(fmt.zero_bits(negative)),
        (Operand::Finite { .. }, Operand::Zero { .. }) => Step::Decided(
            fmt.infinity(negative),
            IeeeFlags::EMPTY.with(IeeeFlag::DivideByZero),
        ),
        (Operand::Zero { .. }, Operand::Finite { .. }) => Step::Finite(Exact::Zero { negative }),
        (
            Operand::Finite {
                significand: dividend,
                exponent: dividend_exponent,
                ..
            },
            Operand::Finite {
                significand: divisor,
                exponent: divisor_exponent,
                ..
            },
        ) => {
            // Both significands left-justified to `fraction_bits + 1` bits,
            // so the quotient below has `fraction_bits + 3` or `+ 4` bits.
            let (dividend, dividend_exponent) = justify(fmt, dividend, dividend_exponent);
            let (divisor, divisor_exponent) = justify(fmt, divisor, divisor_exponent);
            let numerator = u128::from(dividend) << (fmt.fraction_bits + 3);
            let divisor = u128::from(divisor);
            Step::Finite(Exact::Real {
                negative,
                mag: numerator / divisor,
                exp: dividend_exponent - divisor_exponent - fmt.fraction() - 3,
                sticky: !numerator.is_multiple_of(divisor),
            })
        }
        // A NaN never reaches planning.
        (Operand::Nan { .. }, _) | (_, Operand::Nan { .. }) => invalid(fmt),
    }
}

/// Shift a nonzero significand left until bit `fraction_bits` is set.
fn justify(fmt: Fmt, significand: u64, exponent: i32) -> (u64, i32) {
    let shift = significand.leading_zeros() - (63 - fmt.fraction_bits);
    (significand << shift, exponent - shift as i32)
}

fn plan_square_root(fmt: Fmt, a: Operand) -> Step {
    match a {
        Operand::Infinite { negative: false } => decided(fmt.infinity(false)),
        Operand::Infinite { negative: true }
        | Operand::Finite { negative: true, .. }
        | Operand::Nan { .. } => invalid(fmt),
        Operand::Zero { negative } => Step::Finite(Exact::Zero { negative }),
        Operand::Finite {
            significand,
            exponent,
            ..
        } => {
            let (mut significand, mut exponent) = justify(fmt, significand, exponent);
            if exponent & 1 != 0 {
                significand <<= 1;
                exponent -= 1;
            }
            // An even left shift by `fraction_bits + 4` (rounded up to even)
            // gives a radicand of at least `2^(2 * fraction_bits + 4)`, so the
            // root has at least `fraction_bits + 3` bits.
            let shift = (fmt.fraction_bits + 5) & !1;
            let radicand = u128::from(significand) << shift;
            let root = integer_square_root(radicand);
            Step::Finite(Exact::Real {
                negative: false,
                mag: root,
                exp: (exponent - shift as i32) / 2,
                sticky: root * root != radicand,
            })
        }
    }
}

/// `floor(sqrt(radicand))` for a radicand below `2^112`: a fixed
/// [`SQRT_STEPS`]-step binary digit search.
fn integer_square_root(radicand: u128) -> u128 {
    let mut root = 0_u128;
    let mut step = SQRT_STEPS;
    // bound: SQRT_STEPS.
    while step > 0 {
        step -= 1;
        let trial = root | (1_u128 << step);
        if trial * trial <= radicand {
            root = trial;
        }
    }
    root
}

fn plan_fma(fmt: Fmt, rounding: RoundingMode, a: Operand, b: Operand, c: Operand) -> Step {
    let product_negative = a.negative() != b.negative();
    let product_infinite = match (a, b) {
        (Operand::Infinite { .. }, Operand::Zero { .. })
        | (Operand::Zero { .. }, Operand::Infinite { .. }) => return invalid(fmt),
        (Operand::Infinite { .. }, _) | (_, Operand::Infinite { .. }) => true,
        _ => false,
    };
    match (product_infinite, c) {
        (true, Operand::Infinite { negative }) if negative != product_negative => invalid(fmt),
        (true, _) => decided(fmt.infinity(product_negative)),
        (false, Operand::Infinite { negative }) => decided(fmt.infinity(negative)),
        (false, _) => Step::Finite(sum(rounding, product_term(a, b), term(c))),
    }
}

impl Fmt {
    const fn zero_bits(self, negative: bool) -> u64 {
        self.sign(negative)
    }
}

// ---- exact sums --------------------------------------------------------------

/// A signed dyadic `mag * 2^exp` whose zero carries an IEEE sign.
#[derive(Clone, Copy)]
struct Term {
    negative: bool,
    mag: u128,
    exp: i32,
}

fn term(operand: Operand) -> Term {
    match operand {
        Operand::Finite {
            negative,
            significand,
            exponent,
        } => Term {
            negative,
            mag: u128::from(significand),
            exp: exponent,
        },
        other => Term {
            negative: other.negative(),
            mag: 0,
            exp: 0,
        },
    }
}

fn product_term(a: Operand, b: Operand) -> Term {
    let negative = a.negative() != b.negative();
    match (a, b) {
        (
            Operand::Finite {
                significand: x,
                exponent: ex,
                ..
            },
            Operand::Finite {
                significand: y,
                exponent: ey,
                ..
            },
        ) => Term {
            negative,
            mag: u128::from(x) * u128::from(y),
            exp: ex + ey,
        },
        _ => Term {
            negative,
            mag: 0,
            exp: 0,
        },
    }
}

/// Move the most significant bit of a nonzero term to [`WIDE_TOP`].
fn normalised(term: Term) -> Term {
    let shift = term.mag.leading_zeros() - (127 - WIDE_TOP);
    Term {
        negative: term.negative,
        mag: term.mag << shift,
        exp: term.exp - shift as i32,
    }
}

fn sum(rounding: RoundingMode, left: Term, right: Term) -> Exact {
    if left.mag == 0 && right.mag == 0 {
        let negative = if left.negative == right.negative {
            left.negative
        } else {
            rounding == RoundingMode::TowardNegative
        };
        return Exact::Zero { negative };
    }
    if left.mag == 0 || right.mag == 0 {
        let only = if left.mag == 0 { right } else { left };
        return Exact::Real {
            negative: only.negative,
            mag: only.mag,
            exp: only.exp,
            sticky: false,
        };
    }
    let (left, right) = (normalised(left), normalised(right));
    let (high, low) = if left.exp >= right.exp {
        (left, right)
    } else {
        (right, left)
    };
    let distance = (high.exp - low.exp) as u32;
    // The smaller term shifted down by the exponent distance; the bits that
    // leave the register are a sticky remainder strictly below one unit.
    let (shifted, jammed) = if distance >= 128 {
        (0, true)
    } else {
        (
            low.mag >> distance,
            low.mag & ((1_u128 << distance) - 1) != 0,
        )
    };
    let (negative, mag, sticky) = if high.negative == low.negative {
        (high.negative, high.mag + shifted, jammed)
    } else if jammed {
        // `high.mag - (shifted + fraction)` lies in `(m, m + 1)`.
        (high.negative, high.mag - shifted - 1, true)
    } else if high.mag >= shifted {
        (high.negative, high.mag - shifted, false)
    } else {
        (low.negative, shifted - high.mag, false)
    };
    if mag == 0 && !sticky {
        return Exact::Zero {
            negative: rounding == RoundingMode::TowardNegative,
        };
    }
    Exact::Real {
        negative,
        mag,
        exp: high.exp,
        sticky,
    }
}

// ---- rounding ----------------------------------------------------------------

fn bit_length(value: u64) -> i32 {
    (u64::BITS - value.leading_zeros()) as i32
}

/// Round `q` (two guard bits, or one) onto the grid; the integer multiple and
/// whether anything was discarded.
fn round_at(
    q: u64,
    sticky: bool,
    guard_bits: u32,
    negative: bool,
    direction: RoundingMode,
) -> (u64, bool) {
    let kept = q >> guard_bits;
    let remainder = q - (kept << guard_bits);
    let half = 1_u64 << (guard_bits - 1);
    let below_half = remainder < half;
    let at_half = remainder == half;
    let inexact = remainder != 0 || sticky;
    let above_half = remainder > half || (at_half && sticky);
    let tie = at_half && !sticky;
    let up = match direction {
        RoundingMode::Exact | RoundingMode::NearestEven => above_half || (tie && kept & 1 == 1),
        RoundingMode::NearestAway => !below_half,
        RoundingMode::TowardZero => false,
        RoundingMode::TowardPositive => inexact && !negative,
        RoundingMode::TowardNegative => inexact && negative,
    };
    (if up { kept + 1 } else { kept }, inexact)
}

/// Round once and encode, with the fresh flag set. Strict `exact` computes its
/// would-be flags under nearest-even.
fn round(fmt: Fmt, exact: Exact, rounding: RoundingMode) -> (u64, IeeeFlags) {
    let (negative, mag, exp, sticky) = match exact {
        Exact::Zero { negative } => return (fmt.sign(negative), IeeeFlags::EMPTY),
        Exact::Real {
            negative,
            mag,
            exp,
            sticky,
        } => (negative, mag, exp, sticky),
    };
    let direction = if rounding == RoundingMode::Exact {
        RoundingMode::NearestEven
    } else {
        rounding
    };
    let precision = fmt.fraction();
    let length = (128 - mag.leading_zeros()) as i32;
    let log2 = length - 1 + exp;
    let above = log2 - precision - fmt.quantum_min();
    let (quantum, mut index) = if above > 0 {
        (log2 - precision, above as u64)
    } else {
        (fmt.quantum_min(), 0_u64)
    };
    // `q = floor(x / 2^(quantum - 2))`; a positive shift drops magnitude bits
    // into the sticky, a non-positive one only occurs for a non-sticky `mag`.
    let shift = quantum - 2 - exp;
    let (q, lost) = if shift >= 128 {
        (0_u64, true)
    } else if shift > 0 {
        (
            (mag >> shift as u32) as u64,
            mag & ((1_u128 << shift as u32) - 1) != 0,
        )
    } else {
        ((mag << (-shift) as u32) as u64, false)
    };
    let sticky = sticky || lost;
    let (mut kept, inexact) = round_at(q, sticky, 2, negative, direction);
    if bit_length(kept) > precision + 1 {
        kept >>= 1;
        index += 1;
    }
    let mut flags = IeeeFlags::EMPTY;
    if inexact {
        flags = flags.with(IeeeFlag::Inexact);
    }
    if bit_length(kept) == precision + 1 && index + 1 >= fmt.exponent_ones() {
        let to_infinity = match direction {
            RoundingMode::Exact | RoundingMode::NearestEven | RoundingMode::NearestAway => true,
            RoundingMode::TowardZero => false,
            RoundingMode::TowardPositive => !negative,
            RoundingMode::TowardNegative => negative,
        };
        let bits = if to_infinity {
            fmt.infinity(negative)
        } else {
            fmt.max_finite(negative)
        };
        return (bits, flags.with(IeeeFlag::Overflow).with(IeeeFlag::Inexact));
    }
    // Tininess after rounding: the result rounded with an unbounded exponent
    // range lies strictly below the least normal magnitude.
    let tiny = log2 < fmt.emin()
        && !(log2 + 1 == fmt.emin()
            && bit_length(round_at(q, sticky, 1, negative, direction).0) > precision + 1);
    if tiny && inexact {
        flags = flags.with(IeeeFlag::Underflow);
    }
    let bits = fmt.sign(negative) | ((index << fmt.fraction_bits) + kept);
    (bits, flags)
}
