// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-372 fallible Decimal membership over borrowed normalized values.
//!
//! Endpoint division retains the Euclidean remainder instead of materializing
//! a shifted coefficient. One reusable scratch request holds at most `4L`
//! bytes, where `L = ceil(b / 32)` for the largest operand magnitude bit length.
//! Copy, division and comparison each poll the original handle before every
//! limb step. Allocator calls themselves have no elapsed-time bound.

use core::alloc::Layout;
use core::cmp::Ordering;
use core::mem::size_of;

use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::collections::TryReserveErrorKind;
use allocator_api2::vec::Vec;

use crate::{Cancel, CancelCause, Decimal, DecimalType, Integer};

/// Resource or cancellation failure of bounded Decimal membership.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DecimalMembershipFailure {
    /// The magnitude measure cannot be represented as a native scratch layout.
    #[error(
        "Decimal membership scratch capacity exceeds native layout bounds ({magnitude_bits} bits)"
    )]
    Capacity {
        /// Largest operand magnitude bit length, with zero counted as one bit.
        magnitude_bits: u64,
    },
    /// The allocator denied the actual valid scratch request.
    #[error("Decimal membership scratch allocation denied ({layout_bytes} bytes)")]
    Allocation {
        /// Additional `u32` elements requested from the empty scratch vector.
        additional_elements: usize,
        /// Native width of each scratch element in bytes.
        element_width: usize,
        /// Layout size reported by the failed vector reservation.
        layout_bytes: usize,
    },
    /// The original caller handle was cancelled.
    #[error("Decimal membership cancelled: {0:?}")]
    Cancelled(CancelCause),
}

impl DecimalType {
    /// Fallible inclusive membership using the existing normalized representation.
    ///
    /// The caller establishes admitted finite magnitude and alignment-shift bounds
    /// before invoking this helper. It requests at most one `4L`-byte scratch
    /// allocation and performs at most `2(min(k,b)+2)L` limb steps for maximum
    /// magnitude bit length `b`, alignment shift `k` and `L = ceil(b / 32)`.
    /// Neither operand is changed and no semantic charge is made. Cancellation
    /// polls bound arithmetic response to one fixed-width limb step; allocation
    /// and release are indivisible allocator calls.
    pub fn try_contains(
        &self,
        value: &Decimal,
        cancel: &Cancel,
    ) -> Result<bool, DecimalMembershipFailure> {
        contains_in(self, value, cancel, Global, &Unobserved)
    }
}

/// Observe completed mechanical operations without replacing their arithmetic.
trait Observation {
    fn reserved(&self) {}
    fn step(&self, _kind: LimbStep) {}
}

struct Unobserved;
impl Observation for Unobserved {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LimbStep {
    Copy,
    Divide,
    Compare,
}

fn poll(cancel: &Cancel) -> Result<(), DecimalMembershipFailure> {
    cancel.poll();
    match cancel.cause() {
        Some(cause) => Err(DecimalMembershipFailure::Cancelled(cause)),
        None => Ok(()),
    }
}

fn scratch_layout(magnitude_bits: u64) -> Result<(usize, Layout), DecimalMembershipFailure> {
    let limbs = magnitude_bits / 32 + u64::from(magnitude_bits % 32 != 0);
    checked_scratch_layout(limbs, magnitude_bits)
}

fn checked_scratch_layout(
    limbs: u64,
    magnitude_bits: u64,
) -> Result<(usize, Layout), DecimalMembershipFailure> {
    let failure = DecimalMembershipFailure::Capacity { magnitude_bits };
    let limbs = usize::try_from(limbs).map_err(|_| failure)?;
    let bytes = limbs.checked_mul(size_of::<u32>()).ok_or(failure)?;
    let layout = Layout::array::<u32>(limbs).map_err(|_| failure)?;
    // Keep the checked native byte conversion explicit, including on 32-bit targets.
    debug_assert_eq!(bytes, layout.size());
    Ok((limbs, layout))
}

fn contains_in<A: Allocator, O: Observation>(
    target: &DecimalType,
    value: &Decimal,
    cancel: &Cancel,
    allocator: A,
    observation: &O,
) -> Result<bool, DecimalMembershipFailure> {
    poll(cancel)?;
    let normalized = value.normalized();
    let scale = normalized.scale().max(target.min_scale());
    if scale > target.max_scale() {
        poll(cancel)?;
        return Ok(false);
    }
    let coefficient = normalized.coefficient();
    let magnitude_bits = coefficient
        .magnitude_bits()
        .max(target.lower().magnitude_bits())
        .max(target.upper().magnitude_bits());
    let shift = u64::from(scale - normalized.scale());
    let mut comparison = Comparison {
        scratch: Vec::new_in(allocator),
        magnitude_bits,
        cancel,
        observation,
    };
    let admitted = comparison
        .compare(coefficient, shift, target.lower())?
        .is_ge()
        && comparison
            .compare(coefficient, shift, target.upper())?
            .is_le();
    poll(cancel)?;
    Ok(admitted)
}

struct Comparison<'a, A: Allocator, O: Observation> {
    scratch: Vec<u32, A>,
    magnitude_bits: u64,
    cancel: &'a Cancel,
    observation: &'a O,
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

impl<A: Allocator, O: Observation> Comparison<'_, A, O> {
    fn copy_endpoint(&mut self, endpoint: &Integer) -> Result<(), DecimalMembershipFailure> {
        let mut digits = endpoint.as_big().iter_u32_digits();
        if self.scratch.is_empty() {
            let (limbs, _layout) = scratch_layout(self.magnitude_bits)?;
            poll(self.cancel)?;
            self.scratch
                .try_reserve_exact(limbs)
                .map_err(|error| match error.kind() {
                    TryReserveErrorKind::CapacityOverflow => DecimalMembershipFailure::Capacity {
                        magnitude_bits: self.magnitude_bits,
                    },
                    TryReserveErrorKind::AllocError { layout, .. } => {
                        DecimalMembershipFailure::Allocation {
                            additional_elements: limbs,
                            element_width: size_of::<u32>(),
                            layout_bytes: layout.size(),
                        }
                    }
                })?;
            self.observation.reserved();
            poll(self.cancel)?;
            for _ in 0..limbs {
                poll(self.cancel)?;
                // The fallible reservation has admitted every push in this loop.
                self.scratch.push(digits.next().unwrap_or(0));
                self.observation.step(LimbStep::Copy);
            }
        } else {
            for limb in &mut self.scratch {
                poll(self.cancel)?;
                *limb = digits.next().unwrap_or(0);
                self.observation.step(LimbStep::Copy);
            }
        }
        Ok(())
    }

    fn divide_endpoint(&mut self, shift: u64) -> Result<bool, DecimalMembershipFailure> {
        let mut discarded = false;
        for _ in 0..shift.min(self.magnitude_bits) {
            let mut carry = 0_u64;
            let mut nonzero = false;
            for limb in self.scratch.iter_mut().rev() {
                poll(self.cancel)?;
                let combined = (carry << 32) + u64::from(*limb);
                // carry < 10, so combined / 10 fits one u32 magnitude limb.
                *limb = (combined / 10) as u32;
                carry = combined % 10;
                nonzero |= *limb != 0;
                self.observation.step(LimbStep::Divide);
            }
            discarded |= carry != 0;
            if !nonzero {
                break;
            }
        }
        Ok(discarded)
    }

    fn compare(
        &mut self,
        coefficient: &Integer,
        shift: u64,
        endpoint: &Integer,
    ) -> Result<Ordering, DecimalMembershipFailure> {
        let coefficient_sign = sign(coefficient);
        let signs = coefficient_sign.cmp(&sign(endpoint));
        if signs != Ordering::Equal || coefficient_sign == Ordering::Equal {
            return Ok(signs);
        }
        self.copy_endpoint(endpoint)?;
        let discarded = self.divide_endpoint(shift)?;
        let mut digits = coefficient.as_big().iter_u32_digits();
        let mut magnitude = Ordering::Equal;
        // In least-significant-first order, each later unequal limb supersedes
        // the previous comparison. This pads the borrowed coefficient without
        // a scan or a second scratch buffer.
        for limb in &self.scratch {
            poll(self.cancel)?;
            let order = digits.next().unwrap_or(0).cmp(limb);
            if order != Ordering::Equal {
                magnitude = order;
            }
            self.observation.step(LimbStep::Compare);
        }
        // endpoint = q * 10^k + remainder: equality with q lies below an
        // endpoint with any discarded nonzero remainder.
        if magnitude == Ordering::Equal && discarded {
            magnitude = Ordering::Less;
        }
        Ok(if coefficient_sign == Ordering::Less {
            magnitude.reverse()
        } else {
            magnitude
        })
    }
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    use forge_alloc::{AllocFaultPolicy, Faulty, InlineBacked, NonZeroLayout, StdCompat};

    use super::*;
    use crate::RoundingMode;

    fn target(lower: i64, upper: i64, min_scale: u64, max_scale: u64) -> DecimalType {
        DecimalType::new(
            Integer::from(lower),
            Integer::from(upper),
            min_scale,
            max_scale,
            RoundingMode::Exact,
        )
        .unwrap()
    }

    #[derive(Default)]
    struct Counts {
        reservations: Cell<u64>,
        copies: Cell<u64>,
        divisions: Cell<u64>,
        comparisons: Cell<u64>,
    }

    impl Counts {
        fn total(&self) -> u64 {
            self.copies.get() + self.divisions.get() + self.comparisons.get()
        }
    }

    impl Observation for Counts {
        fn reserved(&self) {
            self.reservations.set(self.reservations.get() + 1);
        }

        fn step(&self, kind: LimbStep) {
            let count = match kind {
                LimbStep::Copy => &self.copies,
                LimbStep::Divide => &self.divisions,
                LimbStep::Compare => &self.comparisons,
            };
            count.set(count.get() + 1);
        }
    }

    #[derive(Default)]
    struct PolicyState {
        deny: Cell<bool>,
        requests: Cell<usize>,
        layout: Cell<Option<(usize, usize)>>,
    }

    struct RecordingPolicy<'a>(&'a PolicyState);

    impl AllocFaultPolicy for RecordingPolicy<'_> {
        fn should_fail(&self, layout: NonZeroLayout) -> bool {
            self.0.requests.set(self.0.requests.get() + 1);
            self.0
                .layout
                .set(Some((layout.size().get(), layout.align().get())));
            self.0.deny.get()
        }
    }

    /// Trace: FR-372-AC-1
    #[test]
    fn inclusive_membership_preserves_representations_and_ignores_rounding() {
        // Expected lifted coefficients are calculated directly from these
        // bounded authored pairs, independently of the endpoint algorithm.
        let cases = [
            (0_i64, 100_i64, 2, 2, 1_i64, 0, true),
            (100, 200, 2, 2, 1, 0, true),
            (0, 99, 2, 2, 1, 0, false),
            (101, 200, 2, 2, 1, 0, false),
            (199, 299, 2, 2, 1, 0, false),
            (-100, 0, 2, 2, -1, 0, true),
            (-200, -100, 2, 2, -1, 0, true),
            (-99, 0, 2, 2, -1, 0, false),
            (-200, -101, 2, 2, -1, 0, false),
            (-299, -199, 2, 2, -1, 0, false),
            (-1, 1, 2, 2, 0, 9, true),
            (1, 100, 2, 2, 0, 0, false),
            (-100, -1, 2, 2, 0, 0, false),
            (-100, 100, 0, 2, 1, 3, false),
            (0, 100, 2, 2, 100, 2, true),
            (0, 100, 0, 2, -1, 0, false),
        ];
        for mode in RoundingMode::ALL {
            for (lo, hi, min, max, coefficient, scale, expected) in cases {
                let declared =
                    DecimalType::new(Integer::from(lo), Integer::from(hi), min, max, mode).unwrap();
                let value = Decimal::new(Integer::from(coefficient), scale);
                let original = value.representation().clone();
                let normalized = value.normalized().clone();
                let original_type = declared.clone();
                // Independent bounded rational oracle: align the mathematical
                // value to s* by multiplying its numerator, not dividing endpoints.
                let canonical_scale = normalized.scale();
                let aligned_scale = canonical_scale.max(u32::try_from(min).unwrap());
                let oracle = if u64::from(aligned_scale) > max {
                    false
                } else {
                    let denominator = Integer::power_of_ten(u64::from(scale));
                    let lifted_numerator = Integer::from(coefficient)
                        .mul(&Integer::power_of_ten(u64::from(aligned_scale)));
                    Integer::from(lo).mul(&denominator) <= lifted_numerator
                        && lifted_numerator <= Integer::from(hi).mul(&denominator)
                };
                assert_eq!(oracle, expected);
                assert_eq!(declared.try_contains(&value, &Cancel::new()), Ok(expected));
                assert_eq!(value.representation(), &original);
                assert_eq!(value.normalized(), &normalized);
                assert_eq!(declared, original_type);
            }
        }
    }

    /// Trace: FR-372-AC-2
    #[test]
    fn longest_alignment_reuses_one_exact_request_within_independent_envelope() {
        let endpoint = 1_000_000_000_000_000_000;
        let declared = target(endpoint, endpoint, 18, 18);
        let value = Decimal::new(Integer::one(), 0);
        let cancel = Cancel::new();
        let counts = Counts::default();
        let mut result = None;
        // Independently admitted B = 64, S = 18, L_B = 2.
        let allocation = allocation_counter::measure(|| {
            result = Some(contains_in(&declared, &value, &cancel, Global, &counts));
        });
        assert_eq!(result, Some(Ok(true)));
        assert_eq!(allocation.count_total, 1);
        assert_eq!(allocation.bytes_total, 8);
        assert_eq!(allocation.count_current, 0);
        assert_eq!(allocation.bytes_current, 0);
        assert_eq!(counts.reservations.get(), 1);
        assert_eq!(counts.copies.get(), 4);
        assert_eq!(counts.divisions.get(), 72);
        assert_eq!(counts.comparisons.get(), 4);
        assert_eq!(counts.total(), 2 * (18 + 2) * 2);
    }

    /// Trace: FR-372-AC-1, FR-372-AC-2
    #[test]
    fn borrowed_limb_crossings_and_huge_shift_have_bounded_work() {
        let declared = target(4_294_967_295, 4_294_967_297, 0, 0);
        let value = Decimal::new(Integer::from(4_294_967_296_u64), 0);
        let counts = Counts::default();
        assert_eq!(
            contains_in(&declared, &value, &Cancel::new(), Global, &counts),
            Ok(true)
        );
        assert_eq!(counts.total(), 8);
        assert_eq!(counts.reservations.get(), 1);

        let declared = target(99, 101, u64::from(u32::MAX), u64::from(u32::MAX));
        let value = Decimal::new(Integer::one(), 0);
        let counts = Counts::default();
        let cancel = Cancel::new();
        let mut result = None;
        let allocation = allocation_counter::measure(|| {
            result = Some(contains_in(&declared, &value, &cancel, Global, &counts));
        });
        assert_eq!(result, Some(Ok(false)));
        assert_eq!(allocation.count_total, 1);
        assert_eq!(allocation.bytes_total, 4);
        assert_eq!(allocation.bytes_current, 0);
        // 99 / 10^2 and 101 / 10^3 first become zero; no later pass is allowed.
        assert_eq!(counts.divisions.get(), 5);
        assert_eq!(counts.total(), 9);
        assert!(counts.total() <= 2 * (8 + 2));
    }

    /// Trace: FR-372-AC-1, FR-372-AC-2
    #[test]
    fn scale_sign_and_zero_paths_request_no_scratch() {
        let cancel = Cancel::new();
        let cases = [
            (target(0, 100, 0, 2), Decimal::new(Integer::one(), 3), false),
            (
                target(0, 100, 0, 2),
                Decimal::new(Integer::from(-1_i64), 0),
                false,
            ),
            (
                target(-100, 100, 2, 2),
                Decimal::new(Integer::zero(), 0),
                true,
            ),
        ];
        for (declared, value, expected) in cases {
            let mut result = None;
            let allocation = allocation_counter::measure(|| {
                result = Some(declared.try_contains(&value, &cancel));
            });
            assert_eq!(result, Some(Ok(expected)));
            assert_eq!(allocation.count_total, 0);
        }
    }

    /// Trace: FR-372-AC-3
    #[test]
    fn actual_small_reservation_denial_is_typed_and_recoverable() {
        let declared = target(0, 100, 2, 2);
        let value = Decimal::new(Integer::one(), 0);
        let cancel = Cancel::new();
        let state = PolicyState::default();
        state.deny.set(true);
        // Borrow this adapter throughout each call: moving its inline backing
        // while a scratch allocation is live is prevented by the borrow.
        let allocator = StdCompat::new(Faulty::new(
            InlineBacked::<64>::new(),
            RecordingPolicy(&state),
        ));
        let counts = Counts::default();
        assert_eq!(
            contains_in(&declared, &value, &cancel, &allocator, &counts),
            Err(DecimalMembershipFailure::Allocation {
                additional_elements: 1,
                element_width: 4,
                layout_bytes: 4,
            }),
        );
        assert_eq!(state.requests.get(), 1);
        assert_eq!(state.layout.get(), Some((4, core::mem::align_of::<u32>())));
        assert_eq!(allocator.inner().inner().allocated(), 0);
        assert_eq!(counts.total(), 0);
        assert_eq!(counts.reservations.get(), 0);
        state.deny.set(false);
        let recovered = contains_in(&declared, &value, &cancel, &allocator, &counts);
        assert_eq!(recovered, Ok(true));
        assert_eq!(recovered, declared.try_contains(&value, &Cancel::new()));
        assert_eq!(state.requests.get(), 2);
        assert_eq!(counts.reservations.get(), 1);
    }

    /// Trace: FR-372-AC-3
    #[test]
    fn native_layout_capacity_failure_is_separate_from_allocator_denial() {
        assert_eq!(scratch_layout(1).unwrap().0, 1);
        assert_eq!(scratch_layout(32).unwrap().0, 1);
        assert_eq!(scratch_layout(33).unwrap().0, 2);
        // Exercise the same native layout calculation with boundary element
        // measures, without constructing an impossibly large Decimal.
        assert_eq!(
            checked_scratch_layout(u64::MAX, u64::MAX),
            Err(DecimalMembershipFailure::Capacity {
                magnitude_bits: u64::MAX
            }),
        );
        let native_max_limbs = u64::try_from(isize::MAX).unwrap() / 4;
        assert_eq!(
            checked_scratch_layout(native_max_limbs, u64::MAX)
                .unwrap()
                .1
                .size(),
            usize::try_from(native_max_limbs * 4).unwrap(),
        );
        assert_eq!(
            checked_scratch_layout(native_max_limbs + 1, u64::MAX),
            Err(DecimalMembershipFailure::Capacity {
                magnitude_bits: u64::MAX
            })
        );
    }

    /// Trace: FR-372-AC-4
    #[test]
    fn original_precancelled_handle_prevents_the_actual_reservation() {
        let declared = target(0, 100, 2, 2);
        let value = Decimal::new(Integer::one(), 0);
        for cause in [CancelCause::Requested, CancelCause::Deadline] {
            let state = PolicyState::default();
            let allocator = StdCompat::new(Faulty::new(
                InlineBacked::<64>::new(),
                RecordingPolicy(&state),
            ));
            let cancel = Cancel::new();
            cancel.cancel(cause);
            assert_eq!(
                contains_in(&declared, &value, &cancel, &allocator, &Unobserved),
                Err(DecimalMembershipFailure::Cancelled(cause)),
            );
            assert_eq!(state.requests.get(), 0);
            assert_eq!(cancel.tripped(), Some(cause));
        }
    }

    struct Cancelling<'a> {
        counts: Counts,
        cancel: &'a Cancel,
        cause: CancelCause,
        after_step: u64,
    }

    impl Observation for Cancelling<'_> {
        fn reserved(&self) {
            self.counts.reserved();
            if self.after_step == 0 {
                self.cancel.cancel(self.cause);
            }
        }

        fn step(&self, kind: LimbStep) {
            self.counts.step(kind);
            if self.counts.total() == self.after_step {
                self.cancel.cancel(self.cause);
            }
        }
    }

    /// Trace: FR-372-AC-4
    #[test]
    fn reservation_division_comparison_and_publication_cancellation_release_scratch() {
        let endpoint = 1_000_000_000_000_000_000;
        let declared = target(endpoint, endpoint, 18, 18);
        let value = Decimal::new(Integer::one(), 0);
        // With L=2, each endpoint has 2 copy, 36 division and 2 comparison
        // steps. These boundaries are independent of the recorded step count.
        for after_step in [0, 1, 9, 39, 80] {
            for cause in [CancelCause::Requested, CancelCause::Deadline] {
                let cancel = Cancel::new();
                let observation = Cancelling {
                    counts: Counts::default(),
                    cancel: &cancel,
                    cause,
                    after_step,
                };
                let mut result = None;
                let allocation = allocation_counter::measure(|| {
                    result = Some(contains_in(
                        &declared,
                        &value,
                        &cancel,
                        Global,
                        &observation,
                    ));
                });
                assert_eq!(
                    result,
                    Some(Err(DecimalMembershipFailure::Cancelled(cause)))
                );
                assert_eq!(observation.counts.total(), after_step);
                assert_eq!(observation.counts.reservations.get(), 1);
                assert_eq!(cancel.tripped(), Some(cause));
                assert_eq!(allocation.count_total, 1);
                assert_eq!(allocation.count_current, 0);
                assert_eq!(allocation.bytes_current, 0);
            }
        }
    }
}
