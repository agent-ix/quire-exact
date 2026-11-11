// SPDX-License-Identifier: AGPL-3.0-or-later
//! Owner-facing read-out of the native Decimal operations (FR-403 Behavior
//! 34). This file is outside the proof path: it allocates.

use super::operations::{
    evaluate, order, AdmittedNativeDecimalOperation, NativeDecimalLoss, NativeDecimalResult,
    NativeStop,
};
use super::{NativeDecimal, NativeDecimalType};
use crate::accounting::Meter;
use crate::decimal::{DecimalLoss, DecimalRepresentation, DecimalResult, DecimalType};
use crate::integer::Integer;
use crate::numeric::OrderingOperator;
use crate::outcome::{Outcome, Stop, Undefined};

impl NativeDecimalType {
    /// The owner's type of this declaration, outside the proof path.
    pub fn to_decimal_type(&self) -> DecimalType {
        DecimalType::new(
            Integer::from(self.lower),
            Integer::from(self.upper),
            u64::from(self.min_scale),
            u64::from(self.max_scale),
            self.rounding,
        )
        .expect("a native type is a well-formed owner type")
    }
}

impl NativeDecimalLoss {
    /// The owner's `DecimalLoss`, equal under its `Eq` to the owner's own
    /// record for the same rounding step.
    pub fn to_decimal_loss(&self) -> DecimalLoss {
        DecimalLoss::from_factored(
            (Integer::from(self.numerator), Integer::from(self.cofactor)),
            (self.twos, self.fives),
            DecimalRepresentation::new(Integer::from(self.rounded.coefficient), self.rounded.scale),
            self.mode,
        )
    }
}

impl NativeDecimalResult {
    /// The owner's `DecimalResult`: the retained pair and the loss record.
    pub fn to_decimal_result(&self) -> DecimalResult {
        DecimalResult::from_parts(
            self.value.to_decimal(),
            self.loss.as_ref().map(NativeDecimalLoss::to_decimal_loss),
        )
    }
}

/// Evaluate an admitted operation under the owner's charge schedule.
pub fn evaluate_native_decimal(
    operation: AdmittedNativeDecimalOperation,
    meter: &mut Meter,
) -> Outcome<NativeDecimalResult> {
    let ty = operation.ty;
    Outcome::from_stop(evaluate(operation, meter).map_err(|stop| match stop {
        NativeStop::Incomplete(record) => Stop::Incomplete(record),
        NativeStop::DivisionByZero => Stop::Undefined(Undefined::DivisionByZero),
        NativeStop::Inexact => Stop::Refused(ty.to_decimal_type().inexact()),
        NativeStop::OutOfDomain => Stop::Refused(ty.to_decimal_type().out_of_domain()),
    }))
}

/// Order two carried Decimals by value under the owner's three-point
/// ordering schedule. No ordering is a capacity refusal.
pub fn order_native_decimals(
    operator: OrderingOperator,
    left: NativeDecimal,
    right: NativeDecimal,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(order(operator, left, right, meter).map_err(Stop::Incomplete))
}
