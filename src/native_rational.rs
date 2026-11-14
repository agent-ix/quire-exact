// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani-tractable native Rational carrier and its add, subtract,
//! multiply, ordering and equality (FR-408).
//!
//! A [`NativeRational`] is a canonical fraction of two `i128` (lowest terms,
//! positive denominator, zero as `0/1`), `Copy` and without a heap type. The
//! operations refine the owner's `evaluate_rational_arithmetic`,
//! `order_numbers` and equality path on the Meter charges and on the exact
//! value wherever the formed integers fit `i128`; where one does not, the
//! operation returns a typed [`NativeRationalCapacity`] refusal before any
//! charge. The proof-path bodies use fixed-width arithmetic only: no loop,
//! iterator adapter, recursion or allocation, and one body for host and
//! `cfg(kani)` builds. The owner-facing conversions allocate and sit outside
//! that path.
//!
//! The fields are private, so a value can only be built canonical:
//!
//! ```compile_fail,E0451
//! let _ = quire_exact::NativeRational { numerator: 1, denominator: 1 };
//! ```

use crate::accounting::{ChargePoint, Incomplete, LimitKind, Meter, UnitsCharge};
use crate::integer::Integer;
use crate::native_fraction::{is_canonical_pair, reduce};
use crate::native_integer::{charge_native_integer_schedule, NativeIntegerSchedule};
use crate::numeric::OrderingOperator;
use crate::outcome::Outcome;
use crate::rational::Rational;

/// The first formed integer that left signed 128 bits, in the order the
/// operations check them (FR-408 Behavior 9).
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeRationalCapacity {
    /// A part of an owner `Rational` does not fit signed 128 bits.
    #[error("rational part does not fit signed 128 bits")]
    Input,
    /// `a q`, or for multiply `a c`.
    #[error("left product does not fit signed 128 bits")]
    LeftProduct,
    /// `c p`.
    #[error("right product does not fit signed 128 bits")]
    RightProduct,
    /// The sum or difference of the two products.
    #[error("sum or difference does not fit signed 128 bits")]
    Combination,
    /// `p q`.
    #[error("denominator product does not fit signed 128 bits")]
    Denominator,
}

/// A refusal of the native Rational path. The checks of
/// [`NativeRational::from_i64_parts`] run in the order `ZeroDenominator`,
/// `NonCanonical`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeRationalRefusal {
    /// The denominator is zero.
    #[error("rational denominator is zero")]
    ZeroDenominator,
    /// The pair has a negative denominator, a common factor above one, or is
    /// zero over a denominator other than one.
    #[error("rational pair is not canonical")]
    NonCanonical,
    /// A formed integer does not fit signed 128 bits.
    #[error("native rational capacity exceeded: {0}")]
    Capacity(NativeRationalCapacity),
}

/// A canonical fraction of two signed 128-bit integers. Its fields are
/// private so every value is canonical: positive denominator, lowest terms,
/// zero as `0/1`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NativeRational {
    numerator: i128,
    denominator: i128,
}

impl NativeRational {
    /// Build the carrier from `i64` parts, admitting only a canonical pair.
    /// Nothing is normalized: `2/4` is refused, never built as `1/2`.
    pub fn from_i64_parts(numerator: i64, denominator: i64) -> Result<Self, NativeRationalRefusal> {
        if denominator == 0 {
            Err(NativeRationalRefusal::ZeroDenominator)
        } else if !is_canonical_pair(numerator, denominator) {
            Err(NativeRationalRefusal::NonCanonical)
        } else {
            Ok(Self {
                numerator: i128::from(numerator),
                denominator: i128::from(denominator),
            })
        }
    }

    /// Build the carrier from the owner's canonical `Rational`.
    pub fn from_rational(value: &Rational) -> Result<Self, NativeRationalRefusal> {
        match (value.numerator().native(), value.denominator().native()) {
            (Some(numerator), Some(denominator)) => Ok(Self {
                numerator,
                denominator,
            }),
            _ => Err(NativeRationalRefusal::Capacity(
                NativeRationalCapacity::Input,
            )),
        }
    }

    /// Read the value out as the owner's `Rational`. This allocates.
    pub fn to_rational(&self) -> Rational {
        Rational::new(
            Integer::from(self.numerator),
            Integer::from(self.denominator),
        )
        .expect("the denominator is positive by construction")
    }

    /// The numerator of the canonical fraction.
    pub const fn numerator(&self) -> i128 {
        self.numerator
    }

    /// The positive denominator of the canonical fraction.
    pub const fn denominator(&self) -> i128 {
        self.denominator
    }
}

/// `B(x)`: the bit length of `|x|`, with `B(0) = 1`.
fn bits(value: i128) -> u64 {
    u64::from((128 - value.unsigned_abs().leading_zeros()).max(1))
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
}

/// The unreduced result fraction. The checks run in the order of the
/// capacity quantities; no charge precedes them.
fn form(
    operation: Operation,
    left: &NativeRational,
    right: &NativeRational,
) -> Result<(i128, i128), NativeRationalRefusal> {
    use NativeRationalCapacity as Capacity;
    let refuse = NativeRationalRefusal::Capacity;
    let (a, p, c, q) = (
        left.numerator,
        left.denominator,
        right.numerator,
        right.denominator,
    );
    let numerator = match operation {
        Operation::Multiply => a.checked_mul(c).ok_or(refuse(Capacity::LeftProduct))?,
        Operation::Add | Operation::Subtract => {
            let first = a.checked_mul(q).ok_or(refuse(Capacity::LeftProduct))?;
            let second = c.checked_mul(p).ok_or(refuse(Capacity::RightProduct))?;
            let combined = match operation {
                Operation::Subtract => first.checked_sub(second),
                _ => first.checked_add(second),
            };
            combined.ok_or(refuse(Capacity::Combination))?
        }
    };
    let denominator = p.checked_mul(q).ok_or(refuse(Capacity::Denominator))?;
    Ok((numerator, denominator))
}

/// `integer_bits` of `rational-arithmetic.arithmetic`: `max(N, D)`.
fn arithmetic_bits(operation: Operation, left: &NativeRational, right: &NativeRational) -> u64 {
    let (a, p, c, q) = (
        bits(left.numerator),
        bits(left.denominator),
        bits(right.numerator),
        bits(right.denominator),
    );
    let numerator = match operation {
        Operation::Multiply => a + c,
        Operation::Add | Operation::Subtract => (a + q).max(c + p) + 1,
    };
    numerator.max(p + q)
}

fn operand_bits(left: &NativeRational, right: &NativeRational) -> u64 {
    bits(left.numerator)
        .max(bits(left.denominator))
        .max(bits(right.numerator))
        .max(bits(right.denominator))
}

fn charge_arithmetic(
    operation: Operation,
    left: &NativeRational,
    right: &NativeRational,
    unreduced: (i128, i128),
    meter: &mut Meter,
) -> Result<(), Incomplete> {
    meter.charge_units(UnitsCharge::two(
        ChargePoint::RationalArithmeticOperands,
        (LimitKind::IntegerBits, operand_bits(left, right)),
        (LimitKind::ValueOccurrences, 2),
    ))?;
    meter.charge_units(UnitsCharge::one(
        ChargePoint::RationalArithmeticArithmetic,
        LimitKind::IntegerBits,
        arithmetic_bits(operation, left, right),
    ))?;
    meter.charge_units(UnitsCharge::one(
        ChargePoint::RationalArithmeticNormalize,
        LimitKind::IntegerBits,
        bits(unreduced.0).max(bits(unreduced.1)),
    ))?;
    meter.charge_units(UnitsCharge::new(ChargePoint::RationalArithmeticResultRetain).results(1))
}

fn arithmetic(
    operation: Operation,
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> Result<Outcome<NativeRational>, NativeRationalRefusal> {
    let unreduced = form(operation, left, right)?;
    Ok(
        match charge_arithmetic(operation, left, right, unreduced, meter) {
            Ok(()) => {
                let (numerator, denominator) = reduce(unreduced.0, unreduced.1);
                Outcome::Completed(NativeRational {
                    numerator,
                    denominator,
                })
            }
            Err(record) => Outcome::Incomplete(record),
        },
    )
}

/// `a/p + c/q`: `rational-arithmetic.operands`, `.arithmetic`, `.normalize`,
/// then `.result-retain`.
pub fn native_rational_add(
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> Result<Outcome<NativeRational>, NativeRationalRefusal> {
    arithmetic(Operation::Add, left, right, meter)
}

/// `a/p - c/q`, on the schedule of [`native_rational_add`].
pub fn native_rational_subtract(
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> Result<Outcome<NativeRational>, NativeRationalRefusal> {
    arithmetic(Operation::Subtract, left, right, meter)
}

/// `a/p * c/q`, on the schedule of [`native_rational_add`].
pub fn native_rational_multiply(
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> Result<Outcome<NativeRational>, NativeRationalRefusal> {
    arithmetic(Operation::Multiply, left, right, meter)
}

fn charge_ordering(
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> Result<(), Incomplete> {
    meter.charge_units(UnitsCharge::two(
        ChargePoint::OrderingOperands,
        (LimitKind::IntegerBits, operand_bits(left, right)),
        (LimitKind::ValueOccurrences, 2),
    ))?;
    let cross = (bits(left.numerator) + bits(right.denominator))
        .max(bits(right.numerator) + bits(left.denominator));
    meter.charge_units(UnitsCharge::one(
        ChargePoint::OrderingArithmetic,
        LimitKind::IntegerBits,
        cross,
    ))?;
    meter.charge_units(UnitsCharge::new(ChargePoint::OrderingResultRetain).results(1))
}

/// Order two carriers: `ordering.operands`, `ordering.arithmetic`, then
/// `ordering.result-retain`. The Boolean compares `a q` with `c p`.
pub fn native_rational_order(
    operator: OrderingOperator,
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> Result<Outcome<bool>, NativeRationalRefusal> {
    use NativeRationalCapacity as Capacity;
    let first = left
        .numerator
        .checked_mul(right.denominator)
        .ok_or(NativeRationalRefusal::Capacity(Capacity::LeftProduct))?;
    let second = right
        .numerator
        .checked_mul(left.denominator)
        .ok_or(NativeRationalRefusal::Capacity(Capacity::RightProduct))?;
    Ok(match charge_ordering(left, right, meter) {
        Ok(()) => Outcome::Completed(operator.holds(first.cmp(&second))),
        Err(record) => Outcome::Incomplete(record),
    })
}

/// Equality of two carriers on the owner's equality schedule for two scalar
/// leaves. It forms no product, so it never refuses for capacity.
pub fn native_rational_equal(
    left: &NativeRational,
    right: &NativeRational,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(
        charge_native_integer_schedule(NativeIntegerSchedule::Equality, meter)
            .into_stop()
            .map(|()| left == right),
    )
}

#[cfg(kani)]
mod kani_proofs;
#[cfg(kani)]
mod reference;
