// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bounded, cancellable equality of retained numeric type domains (FR-381).
//!
//! The caller authenticates a finite magnitude ceiling for every endpoint.
//! Borrowed sign, bit length and limb iteration use constant auxiliary storage;
//! neither a native-size conversion nor an allocation is needed.

use crate::{Cancel, CancelCause, DecimalType, Integer, IntegerInterval, RationalDomain};

/// The position of a retained numeric type endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericTypeEndpoint {
    /// Expected Integer interval lower endpoint.
    IntegerExpectedLower,
    /// Expected Integer interval upper endpoint.
    IntegerExpectedUpper,
    /// Actual Integer interval lower endpoint.
    IntegerActualLower,
    /// Actual Integer interval upper endpoint.
    IntegerActualUpper,
    /// Expected Rational numerator interval lower endpoint.
    RationalExpectedNumeratorLower,
    /// Expected Rational numerator interval upper endpoint.
    RationalExpectedNumeratorUpper,
    /// Expected Rational denominator interval lower endpoint.
    RationalExpectedDenominatorLower,
    /// Expected Rational denominator interval upper endpoint.
    RationalExpectedDenominatorUpper,
    /// Actual Rational numerator interval lower endpoint.
    RationalActualNumeratorLower,
    /// Actual Rational numerator interval upper endpoint.
    RationalActualNumeratorUpper,
    /// Actual Rational denominator interval lower endpoint.
    RationalActualDenominatorLower,
    /// Actual Rational denominator interval upper endpoint.
    RationalActualDenominatorUpper,
    /// Expected Decimal type lower coefficient endpoint.
    DecimalExpectedLower,
    /// Expected Decimal type upper coefficient endpoint.
    DecimalExpectedUpper,
    /// Actual Decimal type lower coefficient endpoint.
    DecimalActualLower,
    /// Actual Decimal type upper coefficient endpoint.
    DecimalActualUpper,
}

/// A size or cancellation stop before retained numeric type equality completes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NumericTypeEqualityFailure {
    /// An endpoint exceeds the caller-authenticated magnitude ceiling.
    #[error("numeric type endpoint {position:?} has {measured_bits} bits, above {limit_bits}")]
    UnsupportedSize {
        /// The first oversized endpoint in expected-before-actual order.
        position: NumericTypeEndpoint,
        /// Measured magnitude bits, counting zero as one.
        measured_bits: u64,
        /// Caller-authenticated ceiling in magnitude bits.
        limit_bits: u64,
    },
    /// The original caller handle observed cancellation.
    #[error("numeric type equality cancelled: {0:?}")]
    Cancelled(CancelCause),
}

impl IntegerInterval {
    /// Compare this expected interval with an actual interval under the caller's finite ceiling.
    ///
    /// All four endpoints are checked before comparison. At most
    /// `2 * ceil(magnitude_bits / 32)` borrowed limb pairs are visited.
    /// This method allocates nothing, borrows the original cancellation handle,
    /// and makes no semantic evaluator charge.
    pub fn try_eq_bounded(
        &self,
        actual: &Self,
        magnitude_bits: u64,
        cancel: &Cancel,
    ) -> Result<bool, NumericTypeEqualityFailure> {
        integer_equal(self, actual, magnitude_bits, cancel, &Unobserved)
    }
}

impl RationalDomain {
    /// Compare this expected domain with an actual domain under the caller's finite ceiling.
    ///
    /// All eight endpoints are checked before comparison. At most
    /// `4 * ceil(magnitude_bits / 32)` borrowed limb pairs are visited.
    /// This method allocates nothing, borrows the original cancellation handle,
    /// and makes no semantic evaluator charge.
    pub fn try_eq_bounded(
        &self,
        actual: &Self,
        magnitude_bits: u64,
        cancel: &Cancel,
    ) -> Result<bool, NumericTypeEqualityFailure> {
        rational_equal(self, actual, magnitude_bits, cancel, &Unobserved)
    }
}

impl DecimalType {
    /// Compare this expected retained Decimal type with an actual type under the caller's
    /// finite magnitude ceiling.
    ///
    /// All four coefficient endpoints are checked before comparison. At most
    /// `2 * ceil(magnitude_bits / 32)` borrowed limb pairs are visited. This method
    /// allocates nothing, borrows the original cancellation handle, and makes no
    /// semantic evaluator charge.
    pub fn try_eq_bounded(
        &self,
        actual: &Self,
        magnitude_bits: u64,
        cancel: &Cancel,
    ) -> Result<bool, NumericTypeEqualityFailure> {
        decimal_equal(self, actual, magnitude_bits, cancel, &Unobserved)
    }
}

trait Observation {
    fn limb_pair(&self) {}
}

struct Unobserved;
impl Observation for Unobserved {}

fn poll(cancel: &Cancel) -> Result<(), NumericTypeEqualityFailure> {
    cancel.poll();
    match cancel.cause() {
        Some(cause) => Err(NumericTypeEqualityFailure::Cancelled(cause)),
        None => Ok(()),
    }
}

fn check_endpoints(
    endpoints: &[(&Integer, NumericTypeEndpoint)],
    magnitude_bits: u64,
    cancel: &Cancel,
) -> Result<(), NumericTypeEqualityFailure> {
    poll(cancel)?;
    for &(endpoint, position) in endpoints {
        poll(cancel)?;
        let measured_bits = endpoint.magnitude_bits();
        if measured_bits > magnitude_bits {
            return Err(NumericTypeEqualityFailure::UnsupportedSize {
                position,
                measured_bits,
                limit_bits: magnitude_bits,
            });
        }
    }
    Ok(())
}

fn integer_equal<O: Observation>(
    expected: &IntegerInterval,
    actual: &IntegerInterval,
    magnitude_bits: u64,
    cancel: &Cancel,
    observation: &O,
) -> Result<bool, NumericTypeEqualityFailure> {
    check_endpoints(
        &[
            (expected.lower(), NumericTypeEndpoint::IntegerExpectedLower),
            (expected.upper(), NumericTypeEndpoint::IntegerExpectedUpper),
            (actual.lower(), NumericTypeEndpoint::IntegerActualLower),
            (actual.upper(), NumericTypeEndpoint::IntegerActualUpper),
        ],
        magnitude_bits,
        cancel,
    )?;
    let equal = equal_integer(expected.lower(), actual.lower(), cancel, observation)?
        && equal_integer(expected.upper(), actual.upper(), cancel, observation)?;
    poll(cancel)?;
    Ok(equal)
}

fn rational_equal<O: Observation>(
    expected: &RationalDomain,
    actual: &RationalDomain,
    magnitude_bits: u64,
    cancel: &Cancel,
    observation: &O,
) -> Result<bool, NumericTypeEqualityFailure> {
    check_endpoints(
        &[
            (
                expected.numerator().lower(),
                NumericTypeEndpoint::RationalExpectedNumeratorLower,
            ),
            (
                expected.numerator().upper(),
                NumericTypeEndpoint::RationalExpectedNumeratorUpper,
            ),
            (
                expected.denominator().lower(),
                NumericTypeEndpoint::RationalExpectedDenominatorLower,
            ),
            (
                expected.denominator().upper(),
                NumericTypeEndpoint::RationalExpectedDenominatorUpper,
            ),
            (
                actual.numerator().lower(),
                NumericTypeEndpoint::RationalActualNumeratorLower,
            ),
            (
                actual.numerator().upper(),
                NumericTypeEndpoint::RationalActualNumeratorUpper,
            ),
            (
                actual.denominator().lower(),
                NumericTypeEndpoint::RationalActualDenominatorLower,
            ),
            (
                actual.denominator().upper(),
                NumericTypeEndpoint::RationalActualDenominatorUpper,
            ),
        ],
        magnitude_bits,
        cancel,
    )?;
    let equal = equal_integer(
        expected.numerator().lower(),
        actual.numerator().lower(),
        cancel,
        observation,
    )? && equal_integer(
        expected.numerator().upper(),
        actual.numerator().upper(),
        cancel,
        observation,
    )? && equal_integer(
        expected.denominator().lower(),
        actual.denominator().lower(),
        cancel,
        observation,
    )? && equal_integer(
        expected.denominator().upper(),
        actual.denominator().upper(),
        cancel,
        observation,
    )?;
    poll(cancel)?;
    Ok(equal)
}

fn decimal_equal<O: Observation>(
    expected: &DecimalType,
    actual: &DecimalType,
    magnitude_bits: u64,
    cancel: &Cancel,
    observation: &O,
) -> Result<bool, NumericTypeEqualityFailure> {
    check_endpoints(
        &[
            (expected.lower(), NumericTypeEndpoint::DecimalExpectedLower),
            (expected.upper(), NumericTypeEndpoint::DecimalExpectedUpper),
            (actual.lower(), NumericTypeEndpoint::DecimalActualLower),
            (actual.upper(), NumericTypeEndpoint::DecimalActualUpper),
        ],
        magnitude_bits,
        cancel,
    )?;
    let equal = equal_integer(expected.lower(), actual.lower(), cancel, observation)?
        && equal_integer(expected.upper(), actual.upper(), cancel, observation)?
        && expected.min_scale() == actual.min_scale()
        && expected.max_scale() == actual.max_scale()
        && expected.rounding() == actual.rounding();
    poll(cancel)?;
    Ok(equal)
}

fn equal_integer<O: Observation>(
    expected: &Integer,
    actual: &Integer,
    cancel: &Cancel,
    observation: &O,
) -> Result<bool, NumericTypeEqualityFailure> {
    poll(cancel)?;
    if expected.is_negative() != actual.is_negative() {
        return Ok(false);
    }
    let bits = expected.magnitude_bits();
    if bits != actual.magnitude_bits() {
        return Ok(false);
    }
    let mut expected_limbs = expected.magnitude_limbs();
    let mut actual_limbs = actual.magnitude_limbs();
    let mut equal = true;
    for _ in 0..bits.div_ceil(32) {
        poll(cancel)?;
        equal &= expected_limbs.next() == actual_limbs.next();
        observation.limb_pair();
    }
    Ok(equal)
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;
    use core::str::FromStr;

    use super::*;
    use crate::RoundingMode;

    fn interval(lower: Integer, upper: Integer) -> IntegerInterval {
        IntegerInterval::new(lower, upper).unwrap()
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

    fn decimal(
        lower: Integer,
        upper: Integer,
        min_scale: u64,
        max_scale: u64,
        rounding: RoundingMode,
    ) -> DecimalType {
        DecimalType::new(lower, upper, min_scale, max_scale, rounding).unwrap()
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

    /// Trace: FR-381-AC-1
    #[test]
    fn arithmetic_endpoint_equality_preserves_borrowed_domains() {
        let cancel = Cancel::new();
        let expected = interval(Integer::from(-7_i64), Integer::zero());
        let same = interval(
            Integer::from_str("-7").unwrap(),
            Integer::from_str("0").unwrap(),
        );
        let different = interval(Integer::from(-7_i64), Integer::one());
        let original = expected.clone();
        assert_eq!(expected.try_eq_bounded(&same, 3, &cancel), Ok(true));
        assert_eq!(expected.try_eq_bounded(&different, 3, &cancel), Ok(false));
        assert_eq!(expected, original);
        assert_eq!(same, original);

        let expected = domain(
            Integer::from(-7_i64),
            Integer::zero(),
            Integer::one(),
            Integer::from(7_i64),
        );
        let same = domain(
            Integer::from_str("-7").unwrap(),
            Integer::from_str("0").unwrap(),
            Integer::from_str("1").unwrap(),
            Integer::from_str("7").unwrap(),
        );
        let original = expected.clone();
        assert_eq!(expected.try_eq_bounded(&same, 3, &cancel), Ok(true));
        for different in [
            domain(
                Integer::from(-6_i64),
                Integer::zero(),
                Integer::one(),
                Integer::from(7_i64),
            ),
            domain(
                Integer::from(-7_i64),
                Integer::one(),
                Integer::one(),
                Integer::from(7_i64),
            ),
            domain(
                Integer::from(-7_i64),
                Integer::zero(),
                Integer::from(2_i64),
                Integer::from(7_i64),
            ),
            domain(
                Integer::from(-7_i64),
                Integer::zero(),
                Integer::one(),
                Integer::from(6_i64),
            ),
        ] {
            assert_eq!(expected.try_eq_bounded(&different, 3, &cancel), Ok(false));
        }
        assert_eq!(expected, original);
        assert_eq!(same, original);
    }

    /// Trace: FR-381-AC-2
    #[test]
    fn integer_positions_accept_exact_b_and_report_first_one_over_b() {
        let cancel = Cancel::new();
        let exact = Integer::one().shifted_left(7);
        let over = Integer::one().shifted_left(8);
        let baseline = interval(exact.neg(), exact.clone());
        assert_eq!(
            baseline.try_eq_bounded(&baseline.clone(), 8, &cancel),
            Ok(true)
        );
        for (expected, actual, position) in [
            (
                interval(over.neg(), exact.clone()),
                baseline.clone(),
                NumericTypeEndpoint::IntegerExpectedLower,
            ),
            (
                interval(exact.neg(), over.clone()),
                baseline.clone(),
                NumericTypeEndpoint::IntegerExpectedUpper,
            ),
            (
                baseline.clone(),
                interval(over.neg(), exact.clone()),
                NumericTypeEndpoint::IntegerActualLower,
            ),
            (
                baseline.clone(),
                interval(exact.neg(), over.clone()),
                NumericTypeEndpoint::IntegerActualUpper,
            ),
        ] {
            assert_eq!(
                expected.try_eq_bounded(&actual, 8, &cancel),
                Err(NumericTypeEqualityFailure::UnsupportedSize {
                    position,
                    measured_bits: 9,
                    limit_bits: 8,
                })
            );
        }
        let double_over = interval(over.neg(), over);
        assert_eq!(
            double_over.try_eq_bounded(&double_over, 8, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::IntegerExpectedLower,
                measured_bits: 9,
                limit_bits: 8,
            })
        );
        assert_eq!(
            interval(Integer::zero(), Integer::zero()).try_eq_bounded(
                &interval(Integer::zero(), Integer::zero()),
                0,
                &cancel
            ),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::IntegerExpectedLower,
                measured_bits: 1,
                limit_bits: 0,
            })
        );
    }

    /// Trace: FR-381-AC-2
    #[test]
    fn rational_positions_accept_exact_b_and_report_first_one_over_b() {
        let cancel = Cancel::new();
        let exact = Integer::one().shifted_left(7);
        let over = Integer::one().shifted_left(8);
        let baseline = domain(exact.neg(), exact.clone(), Integer::one(), exact.clone());
        assert_eq!(
            baseline.try_eq_bounded(&baseline.clone(), 8, &cancel),
            Ok(true)
        );
        let exact_denominator_lower =
            domain(exact.neg(), exact.clone(), exact.clone(), exact.clone());
        assert_eq!(
            exact_denominator_lower.try_eq_bounded(&exact_denominator_lower.clone(), 8, &cancel),
            Ok(true)
        );
        let variants = [
            domain(over.neg(), exact.clone(), Integer::one(), exact.clone()),
            domain(exact.neg(), over.clone(), Integer::one(), exact.clone()),
            domain(exact.neg(), exact.clone(), over.clone(), over.clone()),
            domain(exact.neg(), exact.clone(), Integer::one(), over.clone()),
        ];
        let positions = [
            NumericTypeEndpoint::RationalExpectedNumeratorLower,
            NumericTypeEndpoint::RationalExpectedNumeratorUpper,
            NumericTypeEndpoint::RationalExpectedDenominatorLower,
            NumericTypeEndpoint::RationalExpectedDenominatorUpper,
        ];
        for (variant, position) in variants.iter().zip(positions) {
            for (expected, actual, expected_position) in [
                (variant, &baseline, position),
                (
                    &baseline,
                    variant,
                    match position {
                        NumericTypeEndpoint::RationalExpectedNumeratorLower => {
                            NumericTypeEndpoint::RationalActualNumeratorLower
                        }
                        NumericTypeEndpoint::RationalExpectedNumeratorUpper => {
                            NumericTypeEndpoint::RationalActualNumeratorUpper
                        }
                        NumericTypeEndpoint::RationalExpectedDenominatorLower => {
                            NumericTypeEndpoint::RationalActualDenominatorLower
                        }
                        NumericTypeEndpoint::RationalExpectedDenominatorUpper => {
                            NumericTypeEndpoint::RationalActualDenominatorUpper
                        }
                        _ => unreachable!(),
                    },
                ),
            ] {
                assert_eq!(
                    expected.try_eq_bounded(actual, 8, &cancel),
                    Err(NumericTypeEqualityFailure::UnsupportedSize {
                        position: expected_position,
                        measured_bits: 9,
                        limit_bits: 8,
                    })
                );
            }
        }
        let double_over = domain(over.neg(), over, Integer::one(), exact);
        assert_eq!(
            double_over.try_eq_bounded(&double_over, 8, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::RationalExpectedNumeratorLower,
                measured_bits: 9,
                limit_bits: 8,
            })
        );
        let denominator_over = domain(
            Integer::zero(),
            Integer::zero(),
            Integer::one().shifted_left(8),
            Integer::one().shifted_left(8),
        );
        assert_eq!(
            denominator_over.try_eq_bounded(&denominator_over, 8, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::RationalExpectedDenominatorLower,
                measured_bits: 9,
                limit_bits: 8,
            })
        );
        assert_eq!(
            baseline.try_eq_bounded(&denominator_over, 8, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::RationalActualDenominatorLower,
                measured_bits: 9,
                limit_bits: 8,
            })
        );
        let zero = domain(
            Integer::zero(),
            Integer::zero(),
            Integer::one(),
            Integer::one(),
        );
        assert_eq!(
            zero.try_eq_bounded(&zero, 0, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::RationalExpectedNumeratorLower,
                measured_bits: 1,
                limit_bits: 0,
            })
        );
    }

    /// Trace: FR-381-AC-3, FR-381-AC-5
    #[test]
    fn borrowed_limb_work_and_storage_are_bounded_at_repr_boundaries() {
        for bits in [32_u64, 33, 128, 129, 257] {
            let value = Integer::one().shifted_left(bits - 1).add(&Integer::one());
            let integer = interval(value.neg(), value.clone());
            let rational = domain(value.neg(), value.clone(), value.clone(), value.clone());
            let cancel = Cancel::new();
            let steps = Steps::default();
            let mut result = None;
            let allocation = allocation_counter::measure(|| {
                result = Some(integer_equal(&integer, &integer, bits, &cancel, &steps));
            });
            assert_eq!(result, Some(Ok(true)));
            assert_eq!(steps.0.get(), 2 * bits.div_ceil(32));
            assert_eq!(allocation.count_total, 0);

            let steps = Steps::default();
            let mut result = None;
            let allocation = allocation_counter::measure(|| {
                result = Some(rational_equal(&rational, &rational, bits, &cancel, &steps));
            });
            assert_eq!(result, Some(Ok(true)));
            assert_eq!(steps.0.get(), 4 * bits.div_ceil(32));
            assert_eq!(allocation.count_total, 0);

            let late_integer = interval(integer.lower().clone(), value.add(&Integer::from(2_i64)));
            let late_rational = domain(
                rational.numerator().lower().clone(),
                rational.numerator().upper().clone(),
                rational.denominator().lower().clone(),
                value.add(&Integer::from(2_i64)),
            );
            let steps = Steps::default();
            assert_eq!(
                integer_equal(&integer, &late_integer, bits, &cancel, &steps),
                Ok(false)
            );
            assert_eq!(steps.0.get(), 2 * bits.div_ceil(32));
            let steps = Steps::default();
            assert_eq!(
                rational_equal(&rational, &late_rational, bits, &cancel, &steps),
                Ok(false)
            );
            assert_eq!(steps.0.get(), 4 * bits.div_ceil(32));
        }
    }

    /// Trace: FR-381-AC-4, FR-381-AC-5
    #[test]
    fn original_cancel_preempts_size_and_late_comparison_then_retry_completes() {
        let value = Integer::one().shifted_left(256).add(&Integer::one());
        let integer = interval(value.neg(), value.clone());
        let rational = domain(value.neg(), value.clone(), value.clone(), value.clone());
        for cause in [CancelCause::Requested, CancelCause::Deadline] {
            let cancel = Cancel::new();
            cancel.cancel(cause);
            assert_eq!(
                integer.try_eq_bounded(&integer, 0, &cancel),
                Err(NumericTypeEqualityFailure::Cancelled(cause))
            );
            assert_eq!(
                rational.try_eq_bounded(&rational, 0, &cancel),
                Err(NumericTypeEqualityFailure::Cancelled(cause))
            );
            assert_eq!(
                integer.try_eq_bounded(&integer, 257, &cancel),
                Err(NumericTypeEqualityFailure::Cancelled(cause))
            );
            assert_eq!(
                rational.try_eq_bounded(&rational, 257, &cancel),
                Err(NumericTypeEqualityFailure::Cancelled(cause))
            );
            assert_eq!(cancel.tripped(), Some(cause));
            for (at, rational_case) in [(18, false), (36, true)] {
                let cancel = Cancel::new();
                let observer = CancelAt {
                    cancel: &cancel,
                    steps: Steps::default(),
                    at,
                    cause,
                };
                let result = if rational_case {
                    rational_equal(&rational, &rational, 257, &cancel, &observer)
                } else {
                    integer_equal(&integer, &integer, 257, &cancel, &observer)
                };
                assert_eq!(result, Err(NumericTypeEqualityFailure::Cancelled(cause)));
                assert_eq!(observer.steps.0.get(), at);
                assert_eq!(cancel.tripped(), Some(cause));
                let fresh = Cancel::new();
                assert_eq!(integer.try_eq_bounded(&integer, 257, &fresh), Ok(true));
                assert_eq!(rational.try_eq_bounded(&rational, 257, &fresh), Ok(true));
            }
        }
    }

    /// Trace: FR-382-AC-1
    #[test]
    fn retained_decimal_fields_decide_equality_without_changing_borrowed_inputs() {
        let expected = decimal(
            Integer::from(-7_i64),
            Integer::from(7_i64),
            1,
            2,
            RoundingMode::Exact,
        );
        let same = decimal(
            Integer::from_str("-7").unwrap(),
            Integer::from_str("7").unwrap(),
            1,
            2,
            RoundingMode::Exact,
        );
        let expected_before = expected.clone();
        let same_before = same.clone();
        let cancel = Cancel::new();
        assert_eq!(expected.try_eq_bounded(&same, 3, &cancel), Ok(true));
        for actual in [
            decimal(
                Integer::from(-6_i64),
                Integer::from(7_i64),
                1,
                2,
                RoundingMode::Exact,
            ),
            decimal(
                Integer::from(-7_i64),
                Integer::from(6_i64),
                1,
                2,
                RoundingMode::Exact,
            ),
            decimal(
                Integer::from(-7_i64),
                Integer::from(7_i64),
                0,
                2,
                RoundingMode::Exact,
            ),
            decimal(
                Integer::from(-7_i64),
                Integer::from(7_i64),
                1,
                3,
                RoundingMode::Exact,
            ),
        ] {
            let actual_before = actual.clone();
            assert_eq!(expected.try_eq_bounded(&actual, 3, &cancel), Ok(false));
            assert_eq!(actual, actual_before);
        }
        for rounding in RoundingMode::ALL {
            let actual = decimal(Integer::from(-7_i64), Integer::from(7_i64), 1, 2, rounding);
            let actual_before = actual.clone();
            assert_eq!(
                expected.try_eq_bounded(&actual, 3, &cancel),
                Ok(rounding == RoundingMode::Exact)
            );
            assert_eq!(actual, actual_before);
        }
        assert_eq!(expected, expected_before);
        assert_eq!(same, same_before);
    }

    /// Trace: FR-382-AC-2
    #[test]
    fn decimal_endpoints_accept_exact_b_and_report_first_one_over_b() {
        let cancel = Cancel::new();
        let exact = Integer::one().shifted_left(7);
        let over = Integer::one().shifted_left(8);
        let baseline = decimal(exact.neg(), exact.clone(), 0, 2, RoundingMode::Exact);
        assert_eq!(
            baseline.try_eq_bounded(&baseline.clone(), 8, &cancel),
            Ok(true)
        );
        for (expected, actual, position) in [
            (
                decimal(over.neg(), exact.clone(), 0, 2, RoundingMode::Exact),
                baseline.clone(),
                NumericTypeEndpoint::DecimalExpectedLower,
            ),
            (
                decimal(exact.neg(), over.clone(), 0, 2, RoundingMode::Exact),
                baseline.clone(),
                NumericTypeEndpoint::DecimalExpectedUpper,
            ),
            (
                baseline.clone(),
                decimal(over.neg(), exact.clone(), 0, 2, RoundingMode::Exact),
                NumericTypeEndpoint::DecimalActualLower,
            ),
            (
                baseline.clone(),
                decimal(exact.neg(), over.clone(), 0, 2, RoundingMode::Exact),
                NumericTypeEndpoint::DecimalActualUpper,
            ),
        ] {
            assert_eq!(
                expected.try_eq_bounded(&actual, 8, &cancel),
                Err(NumericTypeEqualityFailure::UnsupportedSize {
                    position,
                    measured_bits: 9,
                    limit_bits: 8,
                })
            );
        }
        let double_over = decimal(over.neg(), over.clone(), 0, 2, RoundingMode::Exact);
        assert_eq!(
            double_over.try_eq_bounded(&double_over, 8, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::DecimalExpectedLower,
                measured_bits: 9,
                limit_bits: 8,
            })
        );
        let actual_over = decimal(exact.neg(), over, 0, 3, RoundingMode::NearestEven);
        let small_expected = decimal(
            Integer::from(-1_i64),
            Integer::one(),
            0,
            2,
            RoundingMode::Exact,
        );
        assert_eq!(
            small_expected.try_eq_bounded(&actual_over, 8, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::DecimalActualUpper,
                measured_bits: 9,
                limit_bits: 8,
            })
        );
        let zero = decimal(Integer::zero(), Integer::zero(), 0, 0, RoundingMode::Exact);
        assert_eq!(zero.try_eq_bounded(&zero, 1, &cancel), Ok(true));
        assert_eq!(
            zero.try_eq_bounded(&zero, 0, &cancel),
            Err(NumericTypeEqualityFailure::UnsupportedSize {
                position: NumericTypeEndpoint::DecimalExpectedLower,
                measured_bits: 1,
                limit_bits: 0,
            })
        );
    }

    /// Trace: FR-382-AC-3
    #[test]
    fn decimal_borrowed_limb_work_and_storage_stay_bounded() {
        for bits in [32_u64, 33, 128, 129, 257] {
            let value = Integer::one().shifted_left(bits - 1).add(&Integer::one());
            let expected = decimal(value.neg(), value.clone(), 0, 2, RoundingMode::Exact);
            let actual = decimal(value.neg(), value.clone(), 0, 2, RoundingMode::Exact);
            let cancel = Cancel::new();
            let steps = Steps::default();
            let allocation = allocation_counter::measure(|| {
                assert_eq!(
                    decimal_equal(&expected, &actual, bits, &cancel, &steps),
                    Ok(true)
                );
            });
            assert_eq!(steps.0.get(), 2 * bits.div_ceil(32));
            assert_eq!(allocation.count_total, 0);

            let late = decimal(
                value.neg(),
                value.add(&Integer::from(2_i64)),
                0,
                2,
                RoundingMode::Exact,
            );
            let steps = Steps::default();
            assert_eq!(
                decimal_equal(&expected, &late, bits, &cancel, &steps),
                Ok(false)
            );
            assert_eq!(steps.0.get(), 2 * bits.div_ceil(32));
        }
    }

    /// Trace: FR-382-AC-4
    #[test]
    fn decimal_original_cancel_preempts_size_and_late_comparison() {
        let value = Integer::one().shifted_left(256).add(&Integer::one());
        let expected = decimal(value.neg(), value, 0, 2, RoundingMode::Exact);
        for cause in [CancelCause::Requested, CancelCause::Deadline] {
            let cancelled = Cancel::new();
            cancelled.cancel(cause);
            assert_eq!(
                expected.try_eq_bounded(&expected, 0, &cancelled),
                Err(NumericTypeEqualityFailure::Cancelled(cause))
            );
            assert_eq!(
                expected.try_eq_bounded(&expected, 257, &cancelled),
                Err(NumericTypeEqualityFailure::Cancelled(cause))
            );
            assert_eq!(cancelled.tripped(), Some(cause));

            for at in [17, 18] {
                let cancel = Cancel::new();
                let observer = CancelAt {
                    cancel: &cancel,
                    steps: Steps::default(),
                    at,
                    cause,
                };
                assert_eq!(
                    decimal_equal(&expected, &expected, 257, &cancel, &observer),
                    Err(NumericTypeEqualityFailure::Cancelled(cause))
                );
                assert_eq!(observer.steps.0.get(), at);
                assert_eq!(cancel.tripped(), Some(cause));
            }
            assert_eq!(
                expected.try_eq_bounded(&expected, 257, &Cancel::new()),
                Ok(true)
            );
        }
    }
}
