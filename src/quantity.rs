// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bare quantity values: a magnitude in a unit, with no unit graph.
//!
//! A quantity is a `(magnitude, UnitId)` pair with no reference to quantity
//! declarations. No unit graph (dimension, composed conversion factors and
//! offsets, edge traversal) is a kernel type; it belongs to the caller's unit
//! declarations. The kernel `Quantity` therefore only supports operations
//! between quantities already in the *same* unit; converting first to a common
//! unit, when the units differ but are compatible, is the caller's job, done
//! above this module with the unit graph it holds.
//!
//! This is a deliberate capability limit at the kernel boundary (no cross-unit
//! arithmetic, comparison or equality), mirroring the Enum-ordering and
//! Reference-identity limits in `crate::value`'s and `crate::key`'s module docs.

use crate::accounting::{Charge, ChargePoint, LimitKind, Meter};
use crate::comparison::{ComparisonOperator, IllTyped, IllTypedCause};
use crate::decimal::{DecimalType, RoundingMode};
use crate::identity::UnitId;
use crate::integer::IntegerInterval;
use crate::numeric::{evaluate_rational_arithmetic, rational_ordering_bits, RationalArithmetic};
use crate::outcome::{Outcome, Refusal, Stop};
use crate::rational::{Rational, RationalDomain};
use alloc::boxed::Box;

/// A quantity value: a magnitude in exactly one unit.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Quantity {
    magnitude: Rational,
    unit: UnitId,
}

/// An exact inclusive result interval for one quantity unit.
///
/// This is the typed target retained by a bounded conversion refusal. The
/// caller has already converted the result into `unit` and charged the unit
/// conversion work before checking membership here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuantityResultInterval {
    unit: UnitId,
    min: Rational,
    max: Rational,
}

/// The lower endpoint of a quantity result interval exceeds its upper endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("quantity result interval is reversed")]
pub struct ReversedQuantityResultInterval;

impl QuantityResultInterval {
    /// Construct a nonempty inclusive interval in one unit.
    pub fn new(
        unit: UnitId,
        min: Rational,
        max: Rational,
    ) -> Result<Self, ReversedQuantityResultInterval> {
        if min > max {
            return Err(ReversedQuantityResultInterval);
        }
        Ok(Self { unit, min, max })
    }

    /// The unit of the result and both endpoints.
    pub fn unit(&self) -> &UnitId {
        &self.unit
    }

    /// The inclusive exact minimum magnitude.
    pub fn min(&self) -> &Rational {
        &self.min
    }

    /// The inclusive exact maximum magnitude.
    pub fn max(&self) -> &Rational {
        &self.max
    }
}

/// Check an already converted quantity against its bounded result target.
///
/// The caller charges `unit.target-domain` before this check and owns the
/// diagnostic locus. A quantity in another unit is ill-typed. A refusal
/// carries the target, never the converted magnitude.
pub fn admit_quantity_result(
    result: Quantity,
    target: &QuantityResultInterval,
) -> Result<Outcome<Quantity>, IllTyped> {
    if result.unit != target.unit {
        return Err(IllTyped {
            cause: IllTypedCause::DistinctUnits,
        });
    }
    if result.magnitude < target.min || result.magnitude > target.max {
        return Ok(Outcome::Refused(Refusal::BoundedQuantityOutOfDomain {
            target: Box::new(target.clone()),
        }));
    }
    Ok(Outcome::Completed(result))
}

/// The bounded scalar representation carried by a quantity type.
///
/// The integer carrier stores its rounding mode separately because the
/// integer domain itself has no scale or rounding metadata. Decimal and
/// rational carriers retain their complete declared types, including bounds,
/// scale range and rounding mode where applicable.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum QuantityCarrier {
    /// A bounded `Decimal[lo, hi; smin, smax; mode]` carrier.
    Decimal(DecimalType),
    /// A bounded `Int[lo, hi]` carrier with its placement mode.
    Integer {
        /// The inclusive integer range.
        domain: IntegerInterval,
        /// The rounding mode used at scale zero.
        rounding: RoundingMode,
    },
    /// A bounded `Rational[nmin, nmax; dmin, dmax]` carrier, with exact mode.
    Rational(RationalDomain),
}

/// The structural type identity of `Quantity<unit, carrier>`.
///
/// Identity includes the unit, every carrier bound and scale, and the
/// rounding mode. This is a type descriptor; the caller performs conversion
/// and carrier membership when producing a carried quantity value.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CarriedQuantityType {
    unit: UnitId,
    carrier: QuantityCarrier,
}

impl CarriedQuantityType {
    /// `Quantity<unit, carrier>` with already-checked carrier components.
    pub fn new(unit: UnitId, carrier: QuantityCarrier) -> Self {
        Self { unit, carrier }
    }

    /// The quantity unit.
    pub fn unit(&self) -> &UnitId {
        &self.unit
    }

    /// The bounded scalar carrier and its rounding mode.
    pub fn carrier(&self) -> &QuantityCarrier {
        &self.carrier
    }
}

impl Quantity {
    /// A quantity of `magnitude` in `unit`.
    pub fn new(magnitude: Rational, unit: UnitId) -> Self {
        Self { magnitude, unit }
    }

    /// The magnitude.
    pub fn magnitude(&self) -> &Rational {
        &self.magnitude
    }

    /// The unit.
    pub fn unit(&self) -> &UnitId {
        &self.unit
    }
}

/// A bounded quantity type `Quantity<U>[min, max]`, or the unbounded
/// `Quantity<U>` spelled as a bounded type with no bounds.
///
/// The bounds are the caller's folded effective bounds, each an exact
/// inclusive `Quantity` in `U`. Inverted bounds (`min > max`) are a valid
/// empty type that admits nothing; the kernel does not normalise empty
/// types, so `[5, 3]` and `[4, 2]` are different types. Type identity is
/// the unit plus both bounds, and bound presence is part of it.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BoundedQuantityType {
    unit: UnitId,
    min: Option<Quantity>,
    max: Option<Quantity>,
}

impl BoundedQuantityType {
    /// `Quantity<unit>[min, max]`. Refused with
    /// [`IllTypedCause::MalformedBoundedQuantityType`] only when a present
    /// bound is a quantity in a unit other than `unit`.
    pub fn new(
        unit: UnitId,
        min: Option<Quantity>,
        max: Option<Quantity>,
    ) -> Result<Self, IllTyped> {
        let in_unit = |bound: &Option<Quantity>| bound.as_ref().is_none_or(|b| b.unit == unit);
        if in_unit(&min) && in_unit(&max) {
            Ok(Self { unit, min, max })
        } else {
            Err(IllTyped {
                cause: IllTypedCause::MalformedBoundedQuantityType,
            })
        }
    }

    /// The unit.
    pub fn unit(&self) -> &UnitId {
        &self.unit
    }

    /// The inclusive lower bound, if any.
    pub fn min(&self) -> Option<&Quantity> {
        self.min.as_ref()
    }

    /// The inclusive upper bound, if any.
    pub fn max(&self) -> Option<&Quantity> {
        self.max.as_ref()
    }

    /// Whether `quantity` is in this type's unit with a magnitude inside
    /// the bounds. An empty type admits nothing.
    pub(crate) fn admits(&self, quantity: &Quantity) -> bool {
        quantity.unit == self.unit
            && self
                .min
                .as_ref()
                .is_none_or(|min| min.magnitude <= quantity.magnitude)
            && self
                .max
                .as_ref()
                .is_none_or(|max| quantity.magnitude <= max.magnitude)
    }
}

fn same_unit(left: &Quantity, right: &Quantity) -> Result<(), IllTyped> {
    if left.unit == right.unit {
        Ok(())
    } else {
        Err(IllTyped {
            cause: IllTypedCause::DistinctUnits,
        })
    }
}

/// Add or subtract two quantities in the same unit. Operands in different
/// units are ill-typed and consume nothing.
pub fn evaluate_quantity_arithmetic(
    operation: QuantityArithmetic<'_>,
    meter: &mut Meter,
) -> Result<Outcome<Quantity>, IllTyped> {
    let (left, right, add) = match operation {
        QuantityArithmetic::Add(left, right) => (left, right, true),
        QuantityArithmetic::Subtract(left, right) => (left, right, false),
    };
    same_unit(left, right)?;
    let arithmetic = if add {
        RationalArithmetic::Add(&left.magnitude, &right.magnitude)
    } else {
        RationalArithmetic::Subtract(&left.magnitude, &right.magnitude)
    };
    let outcome = evaluate_rational_arithmetic(arithmetic, None, meter);
    Ok(match outcome {
        Outcome::Completed(magnitude) => {
            match meter.charge(Charge::new(ChargePoint::UnitResultRetain).results(1)) {
                Ok(()) => Outcome::Completed(Quantity::new(magnitude, left.unit.clone())),
                Err(incomplete) => Outcome::Incomplete(incomplete),
            }
        }
        Outcome::Undefined(reason) => Outcome::Undefined(reason),
        Outcome::Refused(reason) => Outcome::Refused(reason),
        Outcome::Incomplete(record) => Outcome::Incomplete(record),
    })
}

/// One quantity arithmetic operation.
#[derive(Clone, Copy, Debug)]
pub enum QuantityArithmetic<'a> {
    /// `a + b`.
    Add(&'a Quantity, &'a Quantity),
    /// `a - b`.
    Subtract(&'a Quantity, &'a Quantity),
}

/// Compare two quantities in the same unit: `ordering.operands`,
/// `ordering.arithmetic`, then `ordering.result-retain` (the
/// same metered shape as `crate::numeric::order_numbers`'s `Rationals` arm,
/// over the same magnitude comparison -- comparing two quantities is
/// otherwise an unmetered `Rational::cmp`, whose cross-multiplication cost
/// grows with caller-supplied operand size). Operands in different units are
/// ill-typed and consume nothing.
pub fn compare_quantity(
    operator: ComparisonOperator,
    left: &Quantity,
    right: &Quantity,
    meter: &mut Meter,
) -> Result<Outcome<bool>, IllTyped> {
    same_unit(left, right)?;
    Ok(Outcome::from_stop(compare(operator, left, right, meter)))
}

fn compare(
    operator: ComparisonOperator,
    left: &Quantity,
    right: &Quantity,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    let bits = left
        .magnitude
        .max_part_bits()
        .max(right.magnitude.max_part_bits());
    meter.charge(
        Charge::new(ChargePoint::OrderingOperands)
            .size(LimitKind::IntegerBits, bits)
            .size(LimitKind::ValueOccurrences, 2),
    )?;
    meter.charge(Charge::new(ChargePoint::OrderingArithmetic).exact_size(
        LimitKind::IntegerBits,
        rational_ordering_bits(&left.magnitude, &right.magnitude),
    ))?;
    let ordering = left.magnitude.cmp(&right.magnitude);
    meter.charge(Charge::new(ChargePoint::OrderingResultRetain).results(1))?;
    Ok(operator.holds(ordering))
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::*;
    use crate::accounting::ScalarLimits;
    use crate::integer::Integer;

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

    fn unit(byte: u8) -> UnitId {
        UnitId::declared(crate::node::NodeKey::fixture(byte))
    }

    /// The bounded target admits exact endpoints and retains its typed
    /// unit and endpoints when an outside result is refused.
    #[trace("TC-942", "FR-395-AC-1", "FR-395-AC-2", "QSpec-FR-322-AC-384")]
    #[test]
    fn bounded_result_checks_inclusive_endpoints_and_refusal_target() {
        let metres = unit(1);
        let min = Rational::from_integer(Integer::from(-1_i64));
        let max = Rational::from_integer(Integer::one());
        let target = QuantityResultInterval::new(metres.clone(), min.clone(), max.clone())
            .expect("ordered bounds");

        for boundary in [min.clone(), max.clone()] {
            let result =
                admit_quantity_result(Quantity::new(boundary.clone(), metres.clone()), &target)
                    .expect("same unit");
            assert_eq!(
                result,
                Outcome::Completed(Quantity::new(boundary, metres.clone()))
            );
        }

        let outside = Quantity::new(Rational::from_integer(Integer::from(2_u64)), metres.clone());
        let refusal = admit_quantity_result(outside, &target).expect("same unit");
        let Outcome::Refused(Refusal::BoundedQuantityOutOfDomain { target: retained }) = refusal
        else {
            panic!("outside result must refuse without a value");
        };
        assert_eq!(retained.unit(), &metres);
        assert_eq!(retained.min(), &min);
        assert_eq!(retained.max(), &max);
        assert_eq!(
            Refusal::BoundedQuantityOutOfDomain { target: retained }.code(),
            Some("rational_out_of_domain")
        );
    }

    /// adding two quantities in the same unit completes with the
    /// summed magnitude in that unit.
    #[test]
    fn same_unit_addition_completes() {
        let metres = unit(1);
        let left = Quantity::new(Rational::from_integer(Integer::one()), metres.clone());
        let right = Quantity::new(Rational::from_integer(Integer::one()), metres.clone());
        let mut meter = generous_meter();
        let outcome =
            evaluate_quantity_arithmetic(QuantityArithmetic::Add(&left, &right), &mut meter)
                .expect("same unit");
        let sum = outcome.completed().expect("charges available");
        assert_eq!(sum.unit(), &metres);
        assert_eq!(
            sum.magnitude(),
            &Rational::from_integer(Integer::from(2_u64))
        );
    }

    /// comparing or adding quantities of different units is
    /// ill-typed with `DistinctUnits`, before any charge.
    #[trace("QSpec-TC-187", "QSpec-FR-142-AC-9")]
    #[test]
    fn distinct_units_are_ill_typed() {
        let metres = Quantity::new(Rational::from_integer(Integer::one()), unit(1));
        let seconds = Quantity::new(Rational::from_integer(Integer::one()), unit(2));
        let mut meter = generous_meter();
        let err =
            compare_quantity(ComparisonOperator::Equal, &metres, &seconds, &mut meter).unwrap_err();
        assert_eq!(err.cause, IllTypedCause::DistinctUnits);
    }

    /// same-unit quantities compare under a generous meter, and a
    /// meter with no `integer_bits` left cannot admit `ordering.operands`,
    /// so the same comparison returns `Outcome::Incomplete` instead
    /// (the metering, proved end to end).
    #[trace("QSpec-TC-187", "QSpec-FR-142-AC-7")]
    #[test]
    fn compare_quantity_charges_and_a_tight_meter_is_incomplete() {
        let metres = unit(1);
        let left = Quantity::new(Rational::from_integer(Integer::one()), metres.clone());
        let right = Quantity::new(Rational::from_integer(Integer::from(2_u64)), metres);

        let mut generous = generous_meter();
        let outcome = compare_quantity(ComparisonOperator::Less, &left, &right, &mut generous)
            .expect("same unit");
        assert!(outcome.completed().expect("charges available"));

        let mut tight = Meter::new(ScalarLimits {
            integer_bits: 0,
            ..*generous.limits()
        });
        let outcome = compare_quantity(ComparisonOperator::Less, &left, &right, &mut tight)
            .expect("same unit");
        assert!(matches!(outcome, Outcome::Incomplete(_)));
    }
}
