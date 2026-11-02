// SPDX-License-Identifier: AGPL-3.0-or-later
use super::*;
use crate::{
    compare_ieee, convert_ieee_width, evaluate_ieee, exact_to_ieee, CancelCause, IeeeComparison,
    IeeeFlags, IeeeOperation, IeeeResult, IeeeValue, IeeeWidth, Outcome, RoundingMode,
};
use ix_trace_rs::trace;

fn generous_limits() -> ScalarLimits {
    ScalarLimits {
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
    }
}

fn operands(bits: u64, arity: u64) -> Charge {
    Charge::new(ChargePoint::IeeeOperands)
        .size(LimitKind::IntegerBits, bits)
        .size(LimitKind::ValueOccurrences, arity)
}

fn retain() -> Charge {
    Charge::new(ChargePoint::IeeeResultRetain).results(1)
}

fn general_schedule(meter: &mut Meter, charges: impl IntoIterator<Item = Charge>) -> Outcome<()> {
    for charge in charges {
        if let Err(record) = meter.charge_general(charge) {
            return Outcome::Incomplete(record);
        }
    }
    Outcome::Completed(())
}

#[test]
#[trace("FR-359-AC-6", "FR-359-AC-7", "FR-359-AC-8", "FR-359-AC-12")]
fn ieee_comparison_preserves_general_admission_boundaries() {
    for (value, bits, numeric_equal) in [
        (IeeeValue::binary32(0), 32, true),
        (IeeeValue::binary32(0x7fc0_0001), 32, false),
        (IeeeValue::binary64(0xbff0_0000_0000_0000), 64, true),
        (IeeeValue::binary64(0x7ff0_0000_0000_0001), 64, false),
    ] {
        for comparison in [
            IeeeComparison::NumericEqual,
            IeeeComparison::TotalOrder,
            IeeeComparison::BitIdentical,
        ] {
            for (kind, boundary) in [
                (LimitKind::IntegerBits, bits),
                (LimitKind::ValueOccurrences, 2),
                (LimitKind::WorkUnits, 2),
                (LimitKind::ResultUnits, 1),
            ] {
                for limit in [0, boundary - 1, boundary, u64::MAX] {
                    for spent in [0, 1, u64::MAX - 1, u64::MAX] {
                        let mut limits = generous_limits();
                        match kind {
                            LimitKind::IntegerBits => limits.integer_bits = limit,
                            LimitKind::ValueOccurrences => limits.value_occurrences = limit,
                            LimitKind::WorkUnits => limits.work_units = limit,
                            LimitKind::ResultUnits => limits.result_units = limit,
                            _ => unreachable!(),
                        }
                        let mut actual = Meter::new(limits);
                        actual.counters.consumed[LimitKind::WorkUnits.index()] = spent;
                        actual.counters.consumed[LimitKind::ResultUnits.index()] = spent;
                        actual.counters.consumed[LimitKind::TextScalars.index()] = 7;
                        if limits.integer_bits >= 80 {
                            actual.counters.consumed[LimitKind::IntegerBits.index()] = 80;
                        }
                        if limits.value_occurrences >= 5 {
                            actual.counters.consumed[LimitKind::ValueOccurrences.index()] = 5;
                        }
                        actual.counters.admissions = u64::MAX - 1;
                        let mut reference = actual.clone();
                        let expected =
                            general_schedule(&mut reference, [operands(bits, 2), retain()]);
                        let expected = match expected {
                            Outcome::Completed(()) => Outcome::Completed(
                                comparison != IeeeComparison::NumericEqual || numeric_equal,
                            ),
                            Outcome::Incomplete(record) => Outcome::Incomplete(record),
                            _ => unreachable!(),
                        };
                        assert_eq!(
                            compare_ieee(comparison, value, value, &mut actual),
                            Ok(expected),
                            "{bits} bits, {comparison:?}, {kind:?} limit {limit}, spent {spent}"
                        );
                        assert_eq!(actual, reference);
                    }
                }
            }
        }
    }
}

#[test]
#[trace("FR-358-AC-1", "FR-358-AC-13", "FR-359-AC-8")]
fn ieee_comparison_preserves_pending_denial_and_refusal_priority() {
    for point in [ChargePoint::IeeeOperands, ChargePoint::IeeeResultRetain] {
        for occurrence in [1, 2] {
            let mut limits = generous_limits();
            limits.integer_bits = 31;
            limits.value_occurrences = 1;
            let mut actual = Meter::new(limits).with_injected_denial(InjectedDenial {
                point,
                occurrence: NonZeroU64::new(occurrence).unwrap(),
            });
            let mut reference = actual.clone();
            // Ordinary simultaneous shortages must neither admit a prefix nor spend a pending denial.
            for allow in [false, true, true, true] {
                if allow {
                    actual.counters.limits = generous_limits();
                    reference.counters.limits = generous_limits();
                }
                let expected = match general_schedule(&mut reference, [operands(32, 2), retain()]) {
                    Outcome::Completed(()) => Outcome::Completed(true),
                    Outcome::Incomplete(record) => Outcome::Incomplete(record),
                    _ => unreachable!(),
                };
                let value = IeeeValue::binary32(0);
                assert_eq!(
                    compare_ieee(IeeeComparison::NumericEqual, value, value, &mut actual),
                    Ok(expected)
                );
                assert_eq!(actual, reference);
            }
        }
    }
}

fn assert_ieee_result(actual: Outcome<IeeeResult>, expected: Outcome<()>, bits: u64) {
    match (actual, expected) {
        (Outcome::Completed(result), Outcome::Completed(())) => {
            assert_eq!(result.value().bits(), bits);
            assert_eq!(result.flags(), IeeeFlags::EMPTY);
        }
        (Outcome::Incomplete(actual), Outcome::Incomplete(expected)) => {
            assert_eq!(actual, expected)
        }
        (actual, expected) => panic!("IEEE outcome {actual:?} differs from admission {expected:?}"),
    }
}

#[test]
#[trace("FR-359-AC-6", "FR-359-AC-7", "FR-359-AC-12")]
fn ieee_arithmetic_and_zero_conversion_preserve_general_charge_prefixes() {
    for width in IeeeWidth::ALL {
        let bits = u64::from(width.bits());
        let (nan, nan_bits) = match width {
            IeeeWidth::Binary32 => (IeeeValue::binary32(0x7fc0_0001), 0x7fc0_0001),
            IeeeWidth::Binary64 => (
                IeeeValue::binary64(0x7ff8_0000_0000_0001),
                0x7ff8_0000_0000_0001,
            ),
        };
        for work in [0, 1, 2, 3, 4, u64::MAX] {
            for results in [0, 1, u64::MAX] {
                let mut limits = generous_limits();
                limits.work_units = work;
                limits.result_units = results;
                for (operation, arity) in [
                    (IeeeOperation::SquareRoot(nan), 1),
                    (IeeeOperation::Add(nan, nan), 2),
                    (IeeeOperation::FusedMultiplyAdd(nan, nan, nan), 3),
                ] {
                    let mut actual = Meter::new(limits);
                    let mut reference = actual.clone();
                    let expected =
                        general_schedule(&mut reference, [operands(bits, arity), retain()]);
                    let result =
                        evaluate_ieee(operation, RoundingMode::Exact, &mut actual).unwrap();
                    assert_ieee_result(result, expected, nan_bits);
                    assert_eq!(actual, reference);
                }
                let mut actual = Meter::new(limits);
                let mut reference = actual.clone();
                // The accounting profile assigns integer zero one magnitude bit.
                let expected = general_schedule(
                    &mut reference,
                    [
                        operands(1, 1),
                        Charge::new(ChargePoint::IeeeExactIntermediate)
                            .size(LimitKind::IntegerBits, bits),
                        Charge::new(ChargePoint::IeeeRound).size(LimitKind::IntegerBits, bits),
                        retain(),
                    ],
                );
                let result =
                    exact_to_ieee(&Integer::zero(), width, RoundingMode::Exact, &mut actual);
                assert_ieee_result(result, expected, 0);
                assert_eq!(actual, reference);
                let mut actual = Meter::new(limits);
                let mut reference = actual.clone();
                let expected = general_schedule(
                    &mut reference,
                    [
                        operands(32, 1),
                        Charge::new(ChargePoint::IeeeExactIntermediate)
                            .size(LimitKind::IntegerBits, 64),
                        Charge::new(ChargePoint::IeeeRound).size(LimitKind::IntegerBits, 64),
                        retain(),
                    ],
                );
                let result = convert_ieee_width(
                    IeeeValue::binary32(0x3f80_0000),
                    IeeeWidth::Binary64,
                    RoundingMode::Exact,
                    &mut actual,
                );
                assert_ieee_result(result, expected, 0x3ff0_0000_0000_0000);
                assert_eq!(actual, reference);
            }
        }
    }
}

struct WatchedMeter {
    meter: Meter,
    handle: Cancel,
    polls: alloc::sync::Arc<core::sync::atomic::AtomicUsize>,
    slot: alloc::sync::Arc<spin::Mutex<Option<Cancel>>>,
}

fn watched_meter(cancel_on: usize) -> WatchedMeter {
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    let polls = Arc::new(AtomicUsize::new(0));
    let slot = Arc::new(spin::Mutex::new(None::<Cancel>));
    let observed_polls = polls.clone();
    let observed_slot = slot.clone();
    let handle = Cancel::observing(move || {
        if observed_polls.fetch_add(1, Ordering::SeqCst) + 1 == cancel_on {
            if let Some(handle) = observed_slot.lock().as_ref() {
                handle.cancel(CancelCause::Requested);
            }
        }
    });
    *slot.lock() = Some(handle.clone());
    WatchedMeter {
        meter: Meter::new(generous_limits()).with_cancel(handle.clone()),
        handle,
        polls,
        slot,
    }
}

#[test]
#[trace("FR-383-AC-1", "FR-383-AC-2", "FR-359-AC-8")]
fn ieee_comparison_polls_original_cancel_once_at_each_charge() {
    use core::sync::atomic::Ordering;

    for cancel_on in [1, 2, 3] {
        let mut actual = watched_meter(cancel_on);
        let mut reference = watched_meter(cancel_on);
        let value = IeeeValue::binary32(0);
        let result = compare_ieee(
            IeeeComparison::NumericEqual,
            value,
            value,
            &mut actual.meter,
        );
        let expected = match general_schedule(&mut reference.meter, [operands(32, 2), retain()]) {
            Outcome::Completed(()) => Outcome::Completed(true),
            Outcome::Incomplete(record) => Outcome::Incomplete(record),
            _ => unreachable!(),
        };
        // Break the observer/handle cycle before any assertion can unwind.
        actual.slot.lock().take();
        reference.slot.lock().take();
        assert_eq!(result, Ok(expected));
        assert_eq!(actual.polls.load(Ordering::SeqCst), cancel_on.min(2));
        assert_eq!(
            actual.polls.load(Ordering::SeqCst),
            reference.polls.load(Ordering::SeqCst)
        );
        assert_eq!(actual.meter.cancel(), Some(&actual.handle));
        assert_ne!(actual.meter.cancel(), Some(&reference.handle));
        assert_eq!(actual.handle.tripped(), reference.handle.tripped());
        assert_eq!(actual.meter.counters, reference.meter.counters);
        #[cfg(feature = "test-support")]
        {
            assert_eq!(
                actual.meter.admitted_charges(),
                reference.meter.admitted_charges()
            );
            assert_eq!(
                actual.meter.charge_log_truncated(),
                reference.meter.charge_log_truncated()
            );
        }
    }
}

#[cfg(feature = "test-support")]
#[test]
#[trace("FR-358-AC-4", "FR-358-AC-5", "FR-358-AC-15")]
fn ieee_comparison_preserves_full_log_prefix_at_capacity() {
    let mut actual = Meter::new(generous_limits());
    for _ in 0..4095 {
        actual
            .charge_general(Charge::new(ChargePoint::FunctionCall))
            .unwrap();
    }
    let mut reference = actual.clone();
    let expected = general_schedule(&mut reference, [operands(32, 2), retain()]);
    assert_eq!(expected, Outcome::Completed(()));
    let value = IeeeValue::binary32(0);
    assert_eq!(
        compare_ieee(IeeeComparison::NumericEqual, value, value, &mut actual),
        Ok(Outcome::Completed(true))
    );
    assert_eq!(actual, reference);
    assert_eq!(actual.admitted_charges().len(), 4096);
    assert!(actual.charge_log_truncated());
    assert_eq!(actual.admission_count(), 4097);
    assert_eq!(
        actual.admitted_charges()[4095].point,
        ChargePoint::IeeeOperands
    );
    assert_eq!(actual.consumed(LimitKind::ResultUnits), 1);
}
