// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bounded, cancellable membership for already constructed exact numbers (FR-380).
//!
//! The caller authenticates a finite magnitude ceiling for the value and every
//! endpoint. The kernel checks that ceiling before comparing borrowed limbs.
//! There is no native limb-count conversion, scratch storage, or allocation.

use core::cmp::Ordering;

use crate::{Cancel, CancelCause, Integer, IntegerInterval, Rational, RationalDomain};

/// The position of an integer checked by bounded numeric membership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericMembershipOperand {
    /// Integer interval candidate.
    IntegerValue,
    /// Integer interval lower endpoint.
    IntegerLower,
    /// Integer interval upper endpoint.
    IntegerUpper,
    /// Rational candidate's reduced numerator.
    RationalNumerator,
    /// Rational candidate's positive denominator.
    RationalDenominator,
    /// Rational domain numerator's lower endpoint.
    RationalNumeratorLower,
    /// Rational domain numerator's upper endpoint.
    RationalNumeratorUpper,
    /// Rational domain denominator's lower endpoint.
    RationalDenominatorLower,
    /// Rational domain denominator's upper endpoint.
    RationalDenominatorUpper,
}

/// A size or cancellation stop before bounded numeric membership completes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NumericMembershipFailure {
    /// An operand exceeds the caller-authenticated maximum magnitude bit length.
    #[error("numeric membership operand {operand:?} has {measured_bits} bits, above {limit_bits}")]
    UnsupportedSize {
        /// The operand whose magnitude exceeded the ceiling.
        operand: NumericMembershipOperand,
        /// Measured magnitude bits, counting zero as one.
        measured_bits: u64,
        /// Caller-authenticated ceiling in magnitude bits.
        limit_bits: u64,
    },
    /// The original caller handle observed cancellation.
    #[error("numeric membership cancelled: {0:?}")]
    Cancelled(CancelCause),
}

impl IntegerInterval {
    /// Inclusive membership under a caller-authenticated finite magnitude ceiling.
    ///
    /// The ceiling applies to `value` and both endpoints. This method borrows
    /// every operand and the original cancellation handle, inspects at most
    /// `2 * ceil(magnitude_bits / 32)` limb pairs, uses constant auxiliary
    /// storage, allocates nothing, and makes no semantic evaluator charge.
    pub fn try_contains_bounded(
        &self,
        value: &Integer,
        magnitude_bits: u64,
        cancel: &Cancel,
    ) -> Result<bool, NumericMembershipFailure> {
        integer_contains(self, value, magnitude_bits, cancel, &Unobserved)
    }
}

impl RationalDomain {
    /// Reduced numerator and positive denominator membership under a caller-authenticated ceiling.
    ///
    /// The ceiling applies to both value parts and all four endpoints. This
    /// method borrows operands and the original cancellation handle, inspects
    /// at most `4 * ceil(magnitude_bits / 32)` limb pairs, uses constant
    /// auxiliary storage, allocates nothing, and makes no evaluator charge.
    pub fn try_contains_bounded(
        &self,
        value: &Rational,
        magnitude_bits: u64,
        cancel: &Cancel,
    ) -> Result<bool, NumericMembershipFailure> {
        rational_contains(self, value, magnitude_bits, cancel, &Unobserved)
    }
}

trait Observation {
    fn limb_pair(&self) {}
}

struct Unobserved;
impl Observation for Unobserved {}

fn poll(cancel: &Cancel) -> Result<(), NumericMembershipFailure> {
    cancel.poll();
    match cancel.cause() {
        Some(cause) => Err(NumericMembershipFailure::Cancelled(cause)),
        None => Ok(()),
    }
}

fn check_operand(
    value: &Integer,
    operand: NumericMembershipOperand,
    magnitude_bits: u64,
    cancel: &Cancel,
) -> Result<(), NumericMembershipFailure> {
    poll(cancel)?;
    let measured_bits = value.magnitude_bits();
    if measured_bits > magnitude_bits {
        return Err(NumericMembershipFailure::UnsupportedSize {
            operand,
            measured_bits,
            limit_bits: magnitude_bits,
        });
    }
    Ok(())
}

fn integer_contains<O: Observation>(
    interval: &IntegerInterval,
    value: &Integer,
    magnitude_bits: u64,
    cancel: &Cancel,
    observation: &O,
) -> Result<bool, NumericMembershipFailure> {
    poll(cancel)?;
    for (operand, position) in [
        (value, NumericMembershipOperand::IntegerValue),
        (interval.lower(), NumericMembershipOperand::IntegerLower),
        (interval.upper(), NumericMembershipOperand::IntegerUpper),
    ] {
        check_operand(operand, position, magnitude_bits, cancel)?;
    }
    let within = compare(interval.lower(), value, cancel, observation)?.is_le()
        && compare(value, interval.upper(), cancel, observation)?.is_le();
    poll(cancel)?;
    Ok(within)
}

fn rational_contains<O: Observation>(
    domain: &RationalDomain,
    value: &Rational,
    magnitude_bits: u64,
    cancel: &Cancel,
    observation: &O,
) -> Result<bool, NumericMembershipFailure> {
    poll(cancel)?;
    for (operand, position) in [
        (
            value.numerator(),
            NumericMembershipOperand::RationalNumerator,
        ),
        (
            value.denominator(),
            NumericMembershipOperand::RationalDenominator,
        ),
        (
            domain.numerator().lower(),
            NumericMembershipOperand::RationalNumeratorLower,
        ),
        (
            domain.numerator().upper(),
            NumericMembershipOperand::RationalNumeratorUpper,
        ),
        (
            domain.denominator().lower(),
            NumericMembershipOperand::RationalDenominatorLower,
        ),
        (
            domain.denominator().upper(),
            NumericMembershipOperand::RationalDenominatorUpper,
        ),
    ] {
        check_operand(operand, position, magnitude_bits, cancel)?;
    }
    let within = compare(
        domain.numerator().lower(),
        value.numerator(),
        cancel,
        observation,
    )?
    .is_le()
        && compare(
            domain.numerator().upper(),
            value.numerator(),
            cancel,
            observation,
        )?
        .is_ge()
        && compare(
            domain.denominator().lower(),
            value.denominator(),
            cancel,
            observation,
        )?
        .is_le()
        && compare(
            domain.denominator().upper(),
            value.denominator(),
            cancel,
            observation,
        )?
        .is_ge();
    poll(cancel)?;
    Ok(within)
}

fn sign(value: &Integer) -> Ordering {
    if value.is_zero() {
        Ordering::Equal
    } else if value.is_negative() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn compare<O: Observation>(
    left: &Integer,
    right: &Integer,
    cancel: &Cancel,
    observation: &O,
) -> Result<Ordering, NumericMembershipFailure> {
    poll(cancel)?;
    let left_sign = sign(left);
    let right_sign = sign(right);
    if left_sign != right_sign || left_sign == Ordering::Equal {
        return Ok(left_sign.cmp(&right_sign));
    }
    let left_bits = left.magnitude_bits();
    let right_bits = right.magnitude_bits();
    let mut magnitude = left_bits.cmp(&right_bits);
    if magnitude == Ordering::Equal {
        let mut left_limbs = left.magnitude_limbs();
        let mut right_limbs = right.magnitude_limbs();
        // Least-significant-first iteration: the last unequal pair wins.
        for _ in 0..left_bits.div_ceil(32) {
            poll(cancel)?;
            let pair = left_limbs
                .next()
                .unwrap_or(0)
                .cmp(&right_limbs.next().unwrap_or(0));
            if pair != Ordering::Equal {
                magnitude = pair;
            }
            observation.limb_pair();
        }
    }
    Ok(if left_sign == Ordering::Less {
        magnitude.reverse()
    } else {
        magnitude
    })
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    use super::*;

    fn interval(lower: Integer, upper: Integer) -> IntegerInterval {
        IntegerInterval::new(lower, upper).unwrap()
    }

    fn rational(numerator: Integer, denominator: Integer) -> Rational {
        Rational::new(numerator, denominator).unwrap()
    }

    fn domain(
        numerator_lower: Integer,
        numerator_upper: Integer,
        denominator_lower: Integer,
        denominator_upper: Integer,
    ) -> RationalDomain {
        RationalDomain::new(
            interval(numerator_lower, numerator_upper),
            interval(denominator_lower, denominator_upper),
        )
        .unwrap()
    }

    #[derive(Default)]
    struct Steps(Cell<u64>);

    impl Observation for Steps {
        fn limb_pair(&self) {
            self.0.set(self.0.get() + 1);
        }
    }

    struct CancelAt<'a> {
        cancel: &'a Cancel,
        steps: Steps,
        at: u64,
        cause: CancelCause,
    }

    impl Observation for CancelAt<'_> {
        fn limb_pair(&self) {
            self.steps.limb_pair();
            if self.steps.0.get() == self.at {
                self.cancel.cancel(self.cause);
            }
        }
    }

    /// Trace: FR-380-AC-1
    #[test]
    fn inclusive_integer_and_reduced_rational_membership_preserves_inputs() {
        let cancel = Cancel::new();
        let integer_domain = interval(Integer::from(-3_i64), Integer::from(5_i64));
        let original_integer_domain = integer_domain.clone();
        for (candidate, expected) in [
            (-4_i64, false),
            (-3, true),
            (0, true),
            (5, true),
            (6, false),
        ] {
            let value = Integer::from(candidate);
            let original_value = value.clone();
            assert_eq!(
                integer_domain.try_contains_bounded(&value, 3, &cancel),
                Ok(expected)
            );
            assert_eq!(value, original_value);
        }
        assert_eq!(integer_domain, original_integer_domain);

        let rational_domain = domain(
            Integer::from(-3_i64),
            Integer::from(5_i64),
            Integer::from(2_i64),
            Integer::from(7_i64),
        );
        let original_rational_domain = rational_domain.clone();
        for (numerator, denominator, expected) in [
            (-4_i64, 3_i64, false),
            (-3, 2, true),
            (5, 7, true),
            (6, 7, false),
            (1, 1, false),
            (1, 7, true),
            (1, 8, false),
        ] {
            let value = rational(Integer::from(numerator), Integer::from(denominator));
            let original_value = value.clone();
            assert_eq!(
                rational_domain.try_contains_bounded(&value, 4, &cancel),
                Ok(expected)
            );
            assert_eq!(value, original_value);
        }
        assert_eq!(rational_domain, original_rational_domain);
    }

    /// Trace: FR-380-AC-2
    #[test]
    fn exact_ceiling_admits_every_position_and_one_over_names_it() {
        let cancel = Cancel::new();
        let exact = Integer::one().shifted_left(7);
        let over = Integer::one().shifted_left(8);
        let integer_domain = interval(exact.neg(), exact.clone());
        assert_eq!(
            integer_domain.try_contains_bounded(&exact, 8, &cancel),
            Ok(true)
        );
        let integer_cases = [
            (
                interval(Integer::zero(), exact.clone()),
                over.clone(),
                NumericMembershipOperand::IntegerValue,
            ),
            (
                interval(over.neg(), Integer::one()),
                Integer::one(),
                NumericMembershipOperand::IntegerLower,
            ),
            (
                interval(Integer::zero(), over.clone()),
                Integer::one(),
                NumericMembershipOperand::IntegerUpper,
            ),
        ];
        for (domain, value, operand) in integer_cases {
            assert_eq!(
                domain.try_contains_bounded(&value, 8, &cancel),
                Err(NumericMembershipFailure::UnsupportedSize {
                    operand,
                    measured_bits: 9,
                    limit_bits: 8,
                })
            );
        }
        assert_eq!(
            interval(Integer::zero(), Integer::zero()).try_contains_bounded(
                &Integer::zero(),
                0,
                &cancel,
            ),
            Err(NumericMembershipFailure::UnsupportedSize {
                operand: NumericMembershipOperand::IntegerValue,
                measured_bits: 1,
                limit_bits: 0,
            })
        );

        let rational_domain = domain(exact.neg(), exact.clone(), Integer::one(), exact.clone());
        let exact_value = rational(exact.clone(), Integer::from(3_i64));
        assert_eq!(
            rational_domain.try_contains_bounded(&exact_value, 8, &cancel),
            Ok(true)
        );
        let exact_denominator = rational(Integer::one(), exact.clone());
        let exact_denominator_domain =
            domain(Integer::zero(), exact.clone(), exact.clone(), exact.clone());
        assert_eq!(
            exact_denominator_domain.try_contains_bounded(&exact_denominator, 8, &cancel),
            Ok(true)
        );
        let rational_cases = [
            (
                rational(over.clone(), Integer::from(3_i64)),
                rational_domain.clone(),
                NumericMembershipOperand::RationalNumerator,
            ),
            (
                rational(Integer::one(), over.clone()),
                rational_domain.clone(),
                NumericMembershipOperand::RationalDenominator,
            ),
            (
                exact_value.clone(),
                domain(over.neg(), exact.clone(), Integer::one(), exact.clone()),
                NumericMembershipOperand::RationalNumeratorLower,
            ),
            (
                exact_value.clone(),
                domain(Integer::zero(), over.clone(), Integer::one(), exact.clone()),
                NumericMembershipOperand::RationalNumeratorUpper,
            ),
            (
                exact_value.clone(),
                domain(
                    exact.neg(),
                    exact.clone(),
                    over.clone(),
                    over.add(&Integer::one()),
                ),
                NumericMembershipOperand::RationalDenominatorLower,
            ),
            (
                exact_value,
                domain(exact.neg(), exact, Integer::one(), over),
                NumericMembershipOperand::RationalDenominatorUpper,
            ),
        ];
        for (value, domain, operand) in rational_cases {
            assert_eq!(
                domain.try_contains_bounded(&value, 8, &cancel),
                Err(NumericMembershipFailure::UnsupportedSize {
                    operand,
                    measured_bits: 9,
                    limit_bits: 8,
                })
            );
        }
    }

    /// Trace: FR-380-AC-3, FR-380-AC-5
    #[test]
    fn borrowed_inline_and_large_comparisons_stay_within_limb_ceiling_without_allocation() {
        for bits in [32_u64, 33, 128, 129, 257] {
            let value = Integer::one().shifted_left(bits - 1).add(&Integer::one());
            let integer_domain = interval(value.clone(), value.clone());
            let steps = Steps::default();
            let cancel = Cancel::new();
            let mut result = None;
            let allocation = allocation_counter::measure(|| {
                result = Some(integer_contains(
                    &integer_domain,
                    &value,
                    bits,
                    &cancel,
                    &steps,
                ));
            });
            assert_eq!(result, Some(Ok(true)));
            assert_eq!(steps.0.get(), 2 * bits.div_ceil(32));
            assert_eq!(allocation.count_total, 0);

            let denominator = Integer::one()
                .shifted_left(bits - 1)
                .add(&Integer::from(3_i64));
            let rational_value = rational(value.clone(), denominator.clone());
            let rational_domain = domain(
                rational_value.numerator().clone(),
                rational_value.numerator().clone(),
                rational_value.denominator().clone(),
                rational_value.denominator().clone(),
            );
            let steps = Steps::default();
            let cancel = Cancel::new();
            let mut result = None;
            let allocation = allocation_counter::measure(|| {
                result = Some(rational_contains(
                    &rational_domain,
                    &rational_value,
                    bits,
                    &cancel,
                    &steps,
                ));
            });
            assert_eq!(result, Some(Ok(true)));
            assert!(steps.0.get() <= 4 * bits.div_ceil(32));
            assert_eq!(allocation.count_total, 0);
        }
    }

    /// Trace: FR-380-AC-4
    #[test]
    fn original_handle_cancels_before_work_and_at_a_late_limb_step() {
        let value = Integer::one().shifted_left(256).add(&Integer::one());
        let integer_domain = interval(value.clone(), value.clone());
        let rational_value = rational(value.clone(), value.add(&Integer::from(2_i64)));
        let rational_domain = domain(
            rational_value.numerator().clone(),
            rational_value.numerator().clone(),
            rational_value.denominator().clone(),
            rational_value.denominator().clone(),
        );
        for cause in [CancelCause::Requested, CancelCause::Deadline] {
            let cancel = Cancel::new();
            cancel.cancel(cause);
            let steps = Steps::default();
            assert_eq!(
                integer_contains(&integer_domain, &value, 257, &cancel, &steps),
                Err(NumericMembershipFailure::Cancelled(cause))
            );
            assert_eq!(
                rational_contains(&rational_domain, &rational_value, 257, &cancel, &steps),
                Err(NumericMembershipFailure::Cancelled(cause))
            );
            assert_eq!(steps.0.get(), 0);
            assert_eq!(cancel.tripped(), Some(cause));

            for at in [17, 18] {
                let cancel = Cancel::new();
                let observer = CancelAt {
                    cancel: &cancel,
                    steps: Steps::default(),
                    at,
                    cause,
                };
                assert_eq!(
                    integer_contains(&integer_domain, &value, 257, &cancel, &observer),
                    Err(NumericMembershipFailure::Cancelled(cause))
                );
                assert_eq!(observer.steps.0.get(), at);
                assert_eq!(cancel.tripped(), Some(cause));
            }

            for at in [35, 36] {
                let cancel = Cancel::new();
                let observer = CancelAt {
                    cancel: &cancel,
                    steps: Steps::default(),
                    at,
                    cause,
                };
                assert_eq!(
                    rational_contains(&rational_domain, &rational_value, 257, &cancel, &observer),
                    Err(NumericMembershipFailure::Cancelled(cause))
                );
                assert_eq!(observer.steps.0.get(), at);
                assert_eq!(cancel.tripped(), Some(cause));
            }
        }
    }
}
