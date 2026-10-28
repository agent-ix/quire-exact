// SPDX-License-Identifier: AGPL-3.0-or-later
//! Admitted native integer operations. Capacity is checked at ingress; the
//! successful retained values and plans contain only native integers.

use alloc::boxed::Box;

use crate::accounting::{ChargePoint, LimitKind, Meter, UnitsCharge};
use crate::integer::{Integer, IntegerInterval};
use crate::numeric::OrderingOperator;
use crate::outcome::{CheckedInvariantCause, Outcome, Refusal, Stop};

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
    fn operand_sizes(self) -> (u64, u64, u64) {
        match self.operands {
            NativeIntegerOperands::Binary { left, right } => (left.bits(), right.bits(), 2),
            NativeIntegerOperands::Unary { operand } => (operand.bits(), 0, 1),
        }
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
    let (left_bits, right_bits, count) = operation.operand_sizes();
    meter.charge_units(UnitsCharge::two(
        ChargePoint::IntegerArithmeticOperands,
        (LimitKind::IntegerBits, left_bits.max(right_bits)),
        (LimitKind::ValueOccurrences, count),
    ))?;
    let (first, second) = match operation.plan.operation {
        NativeIntegerOperation::Add { .. } | NativeIntegerOperation::Subtract { .. } => {
            (left_bits.max(right_bits), 1)
        }
        NativeIntegerOperation::Multiply { .. } => (left_bits, right_bits),
        NativeIntegerOperation::Negate { .. } => (left_bits, 0),
    };
    // Native magnitudes are at most 128 bits, so every amount here is at
    // most 256. Retain a units charge directly; no general Integer amount
    // continuation is needed for this closed native domain.
    meter.charge_units(UnitsCharge::one(
        ChargePoint::IntegerArithmeticArithmetic,
        LimitKind::IntegerBits,
        first + second,
    ))?;
    let result = operation
        .calculate()
        .filter(|value| operation.plan.capacity.contains(*value))
        .ok_or(Stop::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::NativeIntegerCapacityBreach,
        }))?;
    if let Some(domain) = operation
        .plan
        .result_domain
        .filter(|domain| !domain.contains(result))
    {
        return Err(Stop::Refused(Refusal::IntegerOutOfDomain {
            target: Box::new(domain.to_integer_interval()),
        }));
    }
    meter.charge_units(UnitsCharge::new(ChargePoint::IntegerArithmeticResultRetain).results(1))?;
    Ok(result)
}

/// Order native values under the ordinary three-point integer ordering schedule.
pub fn evaluate_native_integer_ordering(
    operator: OrderingOperator,
    left: NativeInteger,
    right: NativeInteger,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop((|| {
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
        let result = operator.holds(left.0.cmp(&right.0));
        meter.charge_units(UnitsCharge::new(ChargePoint::OrderingResultRetain).results(1))?;
        Ok(result)
    })())
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
