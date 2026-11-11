// SPDX-License-Identifier: AGPL-3.0-or-later
//! Admitted native integer operations. Capacity is checked at ingress; the
//! successful retained values and plans contain only native integers.

use alloc::boxed::Box;

use crate::accounting::{ChargePoint, LimitKind, Meter, UnitsCharge};
use crate::division::{self, DivisionMember, DivisionProfile, DivisionSchedule};
use crate::equality;
use crate::integer::{Integer, IntegerInterval};
use crate::numeric::OrderingOperator;
use crate::outcome::{CheckedInvariantCause, Outcome, Refusal, Stop, Undefined};

/// A cache-free native integer. A value alone grants no arithmetic capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NativeInteger(i128);

impl NativeInteger {
    /// Retain every signed i128 payload exactly.
    pub const fn new(value: i128) -> Self {
        Self(value)
    }
    /// Observe the exact signed payload.
    pub const fn value(self) -> i128 {
        self.0
    }
    /// Explicitly cross into the general arbitrary-precision representation.
    pub fn to_integer(self) -> Integer {
        Integer::from(self.0)
    }
    fn bits(self) -> u64 {
        u64::from((128 - self.0.unsigned_abs().leading_zeros()).max(1))
    }
}

/// A nonempty inclusive interval with native endpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeIntegerInterval {
    lower: i128,
    upper: i128,
}

impl NativeIntegerInterval {
    /// Check endpoint order without allocating a general integer.
    pub fn new(lower: i128, upper: i128) -> Result<Self, NativeIntegerAdmissionError> {
        if lower > upper {
            return Err(NativeIntegerAdmissionError::ReversedInterval { lower, upper });
        }
        Ok(Self { lower, upper })
    }
    /// Inclusive lower endpoint.
    pub const fn lower(self) -> i128 {
        self.lower
    }
    /// Inclusive upper endpoint.
    pub const fn upper(self) -> i128 {
        self.upper
    }
    /// Whether the exact native payload belongs to this interval.
    pub fn contains(self, value: NativeInteger) -> bool {
        self.lower <= value.0 && value.0 <= self.upper
    }
    fn to_integer_interval(self) -> IntegerInterval {
        IntegerInterval::from_native_interval(self)
    }
}

impl TryFrom<&IntegerInterval> for NativeIntegerInterval {
    type Error = NativeIntegerAdmissionError;
    /// Explicit general-value ingress, outside native plan construction.
    fn try_from(interval: &IntegerInterval) -> Result<Self, Self::Error> {
        let lower = interval.lower().native().ok_or(
            NativeIntegerAdmissionError::EndpointNotRepresentable {
                endpoint: NativeIntegerEndpoint::Lower,
            },
        )?;
        let upper = interval.upper().native().ok_or(
            NativeIntegerAdmissionError::EndpointNotRepresentable {
                endpoint: NativeIntegerEndpoint::Upper,
            },
        )?;
        Self::new(lower, upper)
    }
}

/// The endpoint that failed explicit general interval ingress.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeIntegerEndpoint {
    /// Lower endpoint.
    Lower,
    /// Upper endpoint.
    Upper,
}

/// The closed set of supported native arithmetic operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeIntegerOperationKind {
    /// Addition.
    Add,
    /// Subtraction.
    Subtract,
    /// Multiplication.
    Multiply,
    /// Unary negation.
    Negate,
}

/// Native admission failure; none of these is an evaluator domain refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeIntegerAdmissionError {
    /// The native interval is empty.
    #[error("reversed native interval [{lower}, {upper}]")]
    ReversedInterval {
        /// Requested lower endpoint.
        lower: i128,
        /// Requested upper endpoint.
        upper: i128,
    },
    /// Explicit general ingress encountered an endpoint outside i128.
    #[error("native interval endpoint {endpoint:?} is not representable")]
    EndpointNotRepresentable {
        /// The nonrepresentable endpoint.
        endpoint: NativeIntegerEndpoint,
    },
    /// The operand domain or complete image exceeds the operation's capacity.
    #[error("native capacity exceeded for {operation:?}")]
    CapacityExceeded {
        /// The rejected operation.
        operation: NativeIntegerOperationKind,
    },
    /// Operands had a different arity from the plan.
    #[error("native operand shape does not match {operation:?}")]
    OperandShapeMismatch {
        /// The plan's operation.
        operation: NativeIntegerOperationKind,
    },
    /// An actual operand was not a member of its certified interval.
    #[error("native operand {operand:?} is outside its interval")]
    OperandOutsideInterval {
        /// The failing operand position.
        operand: NativeIntegerOperandPosition,
        /// Exact failing payload.
        value: i128,
        /// Its required interval.
        interval: NativeIntegerInterval,
    },
}

/// The position of a rejected native operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeIntegerOperandPosition {
    /// Binary left operand or unary operand.
    Left,
    /// Binary right operand.
    Right,
}

/// Closed-arity input domains for one native arithmetic operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeIntegerOperation {
    /// Add the two domains.
    Add {
        /// Left domain.
        left: NativeIntegerInterval,
        /// Right domain.
        right: NativeIntegerInterval,
    },
    /// Subtract the right domain from the left.
    Subtract {
        /// Left domain.
        left: NativeIntegerInterval,
        /// Right domain.
        right: NativeIntegerInterval,
    },
    /// Multiply signed32 operand domains, retaining the signed64 image in i128.
    Multiply {
        /// Left domain.
        left: NativeIntegerInterval,
        /// Right domain.
        right: NativeIntegerInterval,
    },
    /// Negate the domain, reversing its endpoints.
    Negate {
        /// Operand domain.
        operand: NativeIntegerInterval,
    },
}

impl NativeIntegerOperation {
    fn kind(self) -> NativeIntegerOperationKind {
        match self {
            Self::Add { .. } => NativeIntegerOperationKind::Add,
            Self::Subtract { .. } => NativeIntegerOperationKind::Subtract,
            Self::Multiply { .. } => NativeIntegerOperationKind::Multiply,
            Self::Negate { .. } => NativeIntegerOperationKind::Negate,
        }
    }
    fn capacity(self) -> Option<NativeIntegerInterval> {
        let (lower, upper) = match self {
            Self::Add { left, right } => (
                left.lower.checked_add(right.lower)?,
                left.upper.checked_add(right.upper)?,
            ),
            Self::Subtract { left, right } => (
                left.lower.checked_sub(right.upper)?,
                left.upper.checked_sub(right.lower)?,
            ),
            Self::Multiply { left, right } => {
                let a = bounded_product(left.lower, right.lower)?;
                let b = bounded_product(left.lower, right.upper)?;
                let c = bounded_product(left.upper, right.lower)?;
                let d = bounded_product(left.upper, right.upper)?;
                (a.min(b).min(c).min(d), a.max(b).max(c).max(d))
            }
            Self::Negate { operand } => {
                (operand.upper.checked_neg()?, operand.lower.checked_neg()?)
            }
        };
        Some(NativeIntegerInterval { lower, upper })
    }
}

/// A validated operation and its separately retained semantic result domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeIntegerPlan {
    operation: NativeIntegerOperation,
    capacity: NativeIntegerInterval,
    result_domain: Option<NativeIntegerInterval>,
}

impl NativeIntegerPlan {
    /// Derive the complete interval image; no caller certificate is trusted.
    pub fn new(
        operation: NativeIntegerOperation,
        result_domain: Option<NativeIntegerInterval>,
    ) -> Result<Self, NativeIntegerAdmissionError> {
        let capacity =
            operation
                .capacity()
                .ok_or(NativeIntegerAdmissionError::CapacityExceeded {
                    operation: operation.kind(),
                })?;
        Ok(Self {
            operation,
            capacity,
            result_domain,
        })
    }
    /// The derived capacity image, independent of the semantic result domain.
    pub const fn capacity(self) -> NativeIntegerInterval {
        self.capacity
    }
    /// Check the exact payloads against this plan before any evaluator charge.
    pub fn admit(
        self,
        operands: NativeIntegerOperands,
    ) -> Result<AdmittedNativeIntegerOperation, NativeIntegerAdmissionError> {
        let check = |value: NativeInteger, interval: NativeIntegerInterval, operand| {
            if interval.contains(value) {
                Ok(())
            } else {
                Err(NativeIntegerAdmissionError::OperandOutsideInterval {
                    operand,
                    value: value.0,
                    interval,
                })
            }
        };
        match (self.operation, operands) {
            (
                NativeIntegerOperation::Add { left, right }
                | NativeIntegerOperation::Subtract { left, right }
                | NativeIntegerOperation::Multiply { left, right },
                NativeIntegerOperands::Binary { left: a, right: b },
            ) => {
                check(a, left, NativeIntegerOperandPosition::Left)?;
                check(b, right, NativeIntegerOperandPosition::Right)?;
            }
            (
                NativeIntegerOperation::Negate { operand },
                NativeIntegerOperands::Unary { operand: value },
            ) => check(value, operand, NativeIntegerOperandPosition::Left)?,
            _ => {
                return Err(NativeIntegerAdmissionError::OperandShapeMismatch {
                    operation: self.operation.kind(),
                })
            }
        }
        Ok(AdmittedNativeIntegerOperation {
            plan: self,
            operands,
        })
    }
}

/// Closed-arity payloads supplied to a checked plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeIntegerOperands {
    /// Two native operands.
    Binary {
        /// Left value.
        left: NativeInteger,
        /// Right value.
        right: NativeInteger,
    },
    /// One native operand.
    Unary {
        /// Value to negate.
        operand: NativeInteger,
    },
}

/// An opaque operation retaining the exact checked plan and admitted payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmittedNativeIntegerOperation {
    plan: NativeIntegerPlan,
    operands: NativeIntegerOperands,
}

impl AdmittedNativeIntegerOperation {
    // This exact production computation is also the subject of the full-capacity
    // refinement obligation; it never constructs a general Integer.
    fn calculate(self) -> Option<NativeInteger> {
        let value = match (self.plan.operation, self.operands) {
            (NativeIntegerOperation::Add { .. }, NativeIntegerOperands::Binary { left, right }) => {
                left.0.checked_add(right.0)
            }
            (
                NativeIntegerOperation::Subtract { .. },
                NativeIntegerOperands::Binary { left, right },
            ) => left.0.checked_sub(right.0),
            (
                NativeIntegerOperation::Multiply { .. },
                NativeIntegerOperands::Binary { left, right },
            ) => bounded_product(left.0, right.0),
            (NativeIntegerOperation::Negate { .. }, NativeIntegerOperands::Unary { operand }) => {
                operand.0.checked_neg()
            }
            _ => None,
        }?;
        Some(NativeInteger(value))
    }
}

/// Evaluate one admitted operation with the actual production Meter schedule.
pub fn evaluate_native_integer_arithmetic(
    operation: AdmittedNativeIntegerOperation,
    meter: &mut Meter,
) -> Outcome<NativeInteger> {
    Outcome::from_stop(native_arithmetic(operation, meter))
}

fn native_arithmetic(
    operation: AdmittedNativeIntegerOperation,
    meter: &mut Meter,
) -> Result<NativeInteger, Stop> {
    charge_arithmetic_prefix(operation.plan.operation.kind(), operation.operands, meter)?;
    finish_arithmetic(
        operation.calculate(),
        operation.plan.capacity,
        operation.plan.result_domain,
        meter,
    )
}

/// The `integer-arithmetic.operands` and `integer-arithmetic.arithmetic`
/// charges of one native operation, shared by every arithmetic entry and by
/// [`charge_native_integer_schedule`].
fn charge_arithmetic_prefix(
    kind: NativeIntegerOperationKind,
    operands: NativeIntegerOperands,
    meter: &mut Meter,
) -> Result<(), Stop> {
    let (left_bits, right_bits, count) = match operands {
        NativeIntegerOperands::Binary { left, right } => (left.bits(), right.bits(), 2),
        NativeIntegerOperands::Unary { operand } => (operand.bits(), 0, 1),
    };
    meter.charge_units(UnitsCharge::two(
        ChargePoint::IntegerArithmeticOperands,
        (LimitKind::IntegerBits, left_bits.max(right_bits)),
        (LimitKind::ValueOccurrences, count),
    ))?;
    let (first, second) = match kind {
        NativeIntegerOperationKind::Add | NativeIntegerOperationKind::Subtract => {
            (left_bits.max(right_bits), 1)
        }
        NativeIntegerOperationKind::Multiply => (left_bits, right_bits),
        NativeIntegerOperationKind::Negate => (left_bits, 0),
    };
    // Native magnitudes are at most 128 bits, so every amount here is at
    // most 256. Retain a units charge directly; no general Integer amount
    // continuation is needed for this closed native domain.
    meter.charge_units(UnitsCharge::one(
        ChargePoint::IntegerArithmeticArithmetic,
        LimitKind::IntegerBits,
        first + second,
    ))?;
    Ok(())
}

fn charge_arithmetic_retain(meter: &mut Meter) -> Result<(), Stop> {
    meter.charge_units(UnitsCharge::new(ChargePoint::IntegerArithmeticResultRetain).results(1))?;
    Ok(())
}

/// Capacity check, optional semantic-domain refusal, then result-retain.
fn finish_arithmetic(
    calculated: Option<NativeInteger>,
    capacity: NativeIntegerInterval,
    result_domain: Option<NativeIntegerInterval>,
    meter: &mut Meter,
) -> Result<NativeInteger, Stop> {
    let result = calculated
        .filter(|value| capacity.contains(*value))
        .ok_or(Stop::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::NativeIntegerCapacityBreach,
        }))?;
    if let Some(domain) = result_domain.filter(|domain| !domain.contains(result)) {
        return Err(Stop::Refused(Refusal::IntegerOutOfDomain {
            target: Box::new(domain.to_integer_interval()),
        }));
    }
    charge_arithmetic_retain(meter)?;
    Ok(result)
}

const I64_RANGE: NativeIntegerInterval = NativeIntegerInterval {
    lower: i64::MIN as i128,
    upper: i64::MAX as i128,
};

/// Multiply two native operands whose exact product is a signed 64-bit value.
///
/// This is the i64-wide counterpart of the signed32 `Multiply` plan
/// (FR-384): both operands must be signed 64-bit and the exact product must
/// fit signed 64-bit, which holds for every pair within
/// `[-3037000499, 3037000499]` and fails for `3037000500` squared. A pair
/// outside that capacity is a typed admission failure returned before any
/// charge, never a wrapped or saturated product. Charges, result-domain
/// refusal and result-retain are exactly those of
/// [`evaluate_native_integer_arithmetic`] and
/// [`crate::evaluate_integer_arithmetic`] for `Multiply`.
pub fn evaluate_native_integer_multiply_i64(
    left: NativeInteger,
    right: NativeInteger,
    result_domain: Option<NativeIntegerInterval>,
    meter: &mut Meter,
) -> Result<Outcome<NativeInteger>, NativeIntegerAdmissionError> {
    let check = |value: NativeInteger, operand| {
        if I64_RANGE.contains(value) {
            Ok(())
        } else {
            Err(NativeIntegerAdmissionError::OperandOutsideInterval {
                operand,
                value: value.0,
                interval: I64_RANGE,
            })
        }
    };
    check(left, NativeIntegerOperandPosition::Left)?;
    check(right, NativeIntegerOperandPosition::Right)?;
    let product =
        wide_product(left, right).ok_or(NativeIntegerAdmissionError::CapacityExceeded {
            operation: NativeIntegerOperationKind::Multiply,
        })?;
    Ok(Outcome::from_stop(
        charge_arithmetic_prefix(
            NativeIntegerOperationKind::Multiply,
            NativeIntegerOperands::Binary { left, right },
            meter,
        )
        .and_then(|()| finish_arithmetic(Some(product), I64_RANGE, result_domain, meter)),
    ))
}

fn wide_product(left: NativeInteger, right: NativeInteger) -> Option<NativeInteger> {
    let left = i64::try_from(left.0).ok()?;
    let right = i64::try_from(right.0).ok()?;
    left.checked_mul(right)
        .map(|product| NativeInteger(i128::from(product)))
}

/// Exact `(quotient, remainder)` of native `dividend / divisor` under `profile`,
/// `divisor` nonzero. Only the quotient can leave i128 (`i128::MIN / -1`), so
/// it is `None` there; the remainder always fits.
fn native_div_rem(profile: DivisionProfile, dividend: i128, divisor: i128) -> (Option<i128>, i128) {
    let quotient = dividend.checked_div(divisor);
    // `wrapping_rem` differs from `%` only for `i128::MIN % -1`, whose
    // remainder is the exact value 0.
    let remainder = dividend.wrapping_rem(divisor);
    if profile == DivisionProfile::Truncating || remainder == 0 {
        return (quotient, remainder);
    }
    // Floor: a nonzero remainder of the opposite sign to the divisor moves
    // one step; the addition of opposite signs cannot overflow.
    let (floor_q, floor_r) = if (remainder < 0) != (divisor < 0) {
        (
            quotient.and_then(|q| q.checked_sub(1)),
            remainder.wrapping_add(divisor),
        )
    } else {
        (quotient, remainder)
    };
    if profile == DivisionProfile::Floor || floor_r >= 0 {
        return (floor_q, floor_r);
    }
    // Euclidean: only a negative divisor leaves a negative floor remainder.
    (
        floor_q.and_then(|q| q.checked_add(1)),
        floor_r.wrapping_sub(divisor),
    )
}

fn native_division_bits(dividend: NativeInteger, divisor: NativeInteger) -> u64 {
    dividend.bits().max(divisor.bits())
}

/// Evaluate `div` or `rem` on native operands under the selected law.
///
/// Identical to [`crate::divide`] on the same values: the same charges in the
/// same order (`integer-division.operands`, `.arithmetic`, `.domain`,
/// `.result-retain`), `Undefined(DivisionByZero)` after the operands charge for
/// a zero divisor, and `DivisionOutOfDomain` for a `domain` that excludes the
/// exposed member (`None` is the mathematical domain). The one quotient that
/// leaves the i128 carrier, `i128::MIN / -1`, is outside every native domain:
/// it is refused as out of `domain`, or, with no `domain`, as the native
/// capacity breach, after the same charge prefix.
pub fn evaluate_native_integer_division(
    profile: DivisionProfile,
    member: DivisionMember,
    dividend: NativeInteger,
    divisor: NativeInteger,
    domain: Option<NativeIntegerInterval>,
    meter: &mut Meter,
) -> Outcome<NativeInteger> {
    Outcome::from_stop(native_division(
        DivisionSchedule::Division,
        profile,
        member,
        dividend,
        divisor,
        domain,
        meter,
    ))
}

/// Evaluate `mod` on native operands: always the Euclidean remainder,
/// charged at the four `integer-modulus.*` points exactly as
/// [`crate::modulo`] charges the same values, with the same refusals.
pub fn evaluate_native_integer_modulo(
    dividend: NativeInteger,
    divisor: NativeInteger,
    domain: Option<NativeIntegerInterval>,
    meter: &mut Meter,
) -> Outcome<NativeInteger> {
    Outcome::from_stop(native_division(
        DivisionSchedule::Modulus,
        DivisionProfile::Euclidean,
        DivisionMember::Remainder,
        dividend,
        divisor,
        domain,
        meter,
    ))
}

fn native_division(
    schedule: DivisionSchedule,
    profile: DivisionProfile,
    member: DivisionMember,
    dividend: NativeInteger,
    divisor: NativeInteger,
    domain: Option<NativeIntegerInterval>,
    meter: &mut Meter,
) -> Result<NativeInteger, Stop> {
    let bits = native_division_bits(dividend, divisor);
    division::charge_operands(schedule, bits, meter)?;
    if divisor.0 == 0 {
        return Err(Stop::Undefined(Undefined::DivisionByZero));
    }
    division::charge_arithmetic(schedule, bits, meter)?;
    let (quotient, remainder) = native_div_rem(profile, dividend.0, divisor.0);
    let exposed = match member {
        DivisionMember::Quotient => quotient,
        DivisionMember::Remainder => Some(remainder),
    };
    division::charge_domain(schedule, meter)?;
    let result = match (exposed, domain) {
        (Some(value), None) => NativeInteger(value),
        (Some(value), Some(domain)) if domain.contains(NativeInteger(value)) => {
            NativeInteger(value)
        }
        (None, None) => {
            return Err(Stop::Refused(Refusal::CheckedInvariant {
                cause: CheckedInvariantCause::NativeIntegerCapacityBreach,
            }))
        }
        // A bounded domain excludes the member, including the quotient
        // outside i128, which no native interval contains.
        (_, Some(domain)) => {
            let domain = Box::new(domain.to_integer_interval());
            return Err(Stop::Refused(match schedule {
                DivisionSchedule::Division => Refusal::DivisionOutOfDomain { domain, member },
                DivisionSchedule::Modulus => Refusal::ModuloOutOfDomain { domain },
            }));
        }
    };
    division::charge_result_retain(schedule, meter)?;
    Ok(result)
}

/// Decide native integer equality with the `equality.*` schedule
/// [`crate::planned_equality`] charges for a pair of `Integer` values:
/// `plan-form`, `plan`, one `pair`, `result-retain`.
pub fn evaluate_native_integer_equality(
    left: NativeInteger,
    right: NativeInteger,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(
        charge_equality_schedule(meter)
            .map(|()| left == right)
            .map_err(Stop::from),
    )
}

/// Decide native Boolean equality with the `equality.*` schedule
/// [`crate::planned_equality`] charges for a pair of `Boolean` values:
/// `plan-form`, `plan`, one `pair`, `result-retain`. It makes the same four
/// charges as [`evaluate_native_integer_equality`] because a `Boolean` is one
/// value occurrence, and not the one `boolean.result-retain` charge of
/// [`crate::retain_boolean`].
pub fn evaluate_native_boolean_equality(
    left: bool,
    right: bool,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(
        charge_equality_schedule(meter)
            .map(|()| left == right)
            .map_err(Stop::from),
    )
}

fn charge_equality_schedule(meter: &mut Meter) -> Result<(), crate::accounting::Incomplete> {
    // An `Integer` is one value occurrence, so both operands and the plan count one.
    let one = Integer::one();
    equality::charge_plan_form(&one, &one, meter)?;
    equality::charge_pairs(&one, meter)
}

/// One native operation whose complete success-path charge schedule is
/// requested without computing a result.
///
/// Non-exhaustive: consumers must handle future variants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum NativeIntegerSchedule {
    /// Add, subtract, multiply or negate an admitted operation.
    Arithmetic(AdmittedNativeIntegerOperation),
    /// The i64-wide multiply of [`evaluate_native_integer_multiply_i64`].
    MultiplyI64 {
        /// Left operand.
        left: NativeInteger,
        /// Right operand.
        right: NativeInteger,
    },
    /// `div` or `rem` of [`evaluate_native_integer_division`].
    Division {
        /// Dividend.
        dividend: NativeInteger,
        /// Divisor.
        divisor: NativeInteger,
    },
    /// `mod` of [`evaluate_native_integer_modulo`].
    Modulus {
        /// Dividend.
        dividend: NativeInteger,
        /// Divisor.
        divisor: NativeInteger,
    },
    /// Ordering of [`evaluate_native_integer_ordering`].
    Ordering {
        /// Left operand.
        left: NativeInteger,
        /// Right operand.
        right: NativeInteger,
    },
    /// Equality of [`evaluate_native_integer_equality`].
    Equality,
}

/// Charge the kernel's schedule for `schedule` on `meter` and compute nothing.
///
/// A caller that has proved by checked bounds that the result fits (so it
/// skips the kernel call) charges through this entry and the meter sees the
/// identical points, amounts and order as the evaluator: the evaluators call
/// the same charge functions. A zero divisor stops after the operands charge
/// with `Undefined(DivisionByZero)` as the evaluator does; a result-domain
/// refusal cannot arise because no result is formed.
pub fn charge_native_integer_schedule(
    schedule: NativeIntegerSchedule,
    meter: &mut Meter,
) -> Outcome<()> {
    Outcome::from_stop(charge_schedule(schedule, meter))
}

fn charge_schedule(schedule: NativeIntegerSchedule, meter: &mut Meter) -> Result<(), Stop> {
    match schedule {
        NativeIntegerSchedule::Arithmetic(operation) => {
            charge_arithmetic_prefix(operation.plan.operation.kind(), operation.operands, meter)?;
            charge_arithmetic_retain(meter)
        }
        NativeIntegerSchedule::MultiplyI64 { left, right } => {
            charge_arithmetic_prefix(
                NativeIntegerOperationKind::Multiply,
                NativeIntegerOperands::Binary { left, right },
                meter,
            )?;
            charge_arithmetic_retain(meter)
        }
        NativeIntegerSchedule::Division { dividend, divisor } => {
            charge_division_schedule(DivisionSchedule::Division, dividend, divisor, meter)
        }
        NativeIntegerSchedule::Modulus { dividend, divisor } => {
            charge_division_schedule(DivisionSchedule::Modulus, dividend, divisor, meter)
        }
        NativeIntegerSchedule::Ordering { left, right } => {
            Ok(charge_ordering_schedule(left, right, meter)?)
        }
        NativeIntegerSchedule::Equality => Ok(charge_equality_schedule(meter)?),
    }
}

fn charge_division_schedule(
    schedule: DivisionSchedule,
    dividend: NativeInteger,
    divisor: NativeInteger,
    meter: &mut Meter,
) -> Result<(), Stop> {
    let bits = native_division_bits(dividend, divisor);
    division::charge_operands(schedule, bits, meter)?;
    if divisor.0 == 0 {
        return Err(Stop::Undefined(Undefined::DivisionByZero));
    }
    division::charge_arithmetic(schedule, bits, meter)?;
    division::charge_domain(schedule, meter)?;
    division::charge_result_retain(schedule, meter)?;
    Ok(())
}

fn charge_ordering_schedule(
    left: NativeInteger,
    right: NativeInteger,
    meter: &mut Meter,
) -> Result<(), crate::accounting::Incomplete> {
    let bits = left.bits().max(right.bits());
    meter.charge_units(UnitsCharge::two(
        ChargePoint::OrderingOperands,
        (LimitKind::IntegerBits, bits),
        (LimitKind::ValueOccurrences, 2),
    ))?;
    meter.charge_units(UnitsCharge::one(
        ChargePoint::OrderingArithmetic,
        LimitKind::IntegerBits,
        bits,
    ))?;
    meter.charge_units(UnitsCharge::new(ChargePoint::OrderingResultRetain).results(1))
}

/// Order native values under the ordinary three-point integer ordering schedule.
pub fn evaluate_native_integer_ordering(
    operator: OrderingOperator,
    left: NativeInteger,
    right: NativeInteger,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(
        charge_ordering_schedule(left, right, meter)
            .map(|()| operator.holds(left.0.cmp(&right.0)))
            .map_err(Stop::from),
    )
}

#[cfg(kani)]
mod kani_proofs;
#[cfg(any(test, kani))]
mod reference;

fn bounded_product(left: i128, right: i128) -> Option<i128> {
    let left = i32::try_from(left).ok()?;
    let right = i32::try_from(right).ok()?;
    Some(i128::from(i64::from(left) * i64::from(right)))
}
