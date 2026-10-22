// SPDX-License-Identifier: AGPL-3.0-or-later
use super::units_reference::{self as reference, KINDS};
use super::*;
use ix_trace_rs::trace;

fn limits(values: [u64; 10]) -> ScalarLimits {
    ScalarLimits {
        integer_bits: values[0],
        decimal_digits: values[1],
        scale_expansion: values[2],
        text_input_bytes: values[3],
        text_scalars: values[4],
        normalized_scalars: values[5],
        unit_edges: values[6],
        value_occurrences: values[7],
        work_units: values[8],
        result_units: values[9],
    }
}

fn check(
    limits_array: [u64; 10],
    consumed: [u64; 10],
    requests: [Option<(LimitKind, u64)>; 2],
    work: u64,
    results: u64,
) {
    let point = ChargePoint::FunctionCall;
    let mut direct = Meter::new(limits(limits_array));
    direct.counters.consumed = consumed;
    direct.counters.admissions = u64::MAX - 1;
    let mut adapter = direct.clone();
    let mut general = direct.clone();
    let mut charge = Charge::new(point)
        .work(Integer::from(work))
        .results(results);
    for (kind, amount) in requests.into_iter().flatten() {
        charge = charge.size(kind, amount);
    }
    let units = UnitsCharge::from_charge(&charge).unwrap();
    let expected = reference::transition(limits_array, consumed, requests, work, results, point);
    let actual = direct.charge_units(units);
    assert_eq!(adapter.charge(charge.clone()), actual);
    assert_eq!(general.charge_general(charge), actual);
    assert_eq!(adapter, direct);
    assert_eq!(general, direct);
    #[cfg(feature = "test-support")]
    {
        if actual.is_ok() {
            let log = direct.admitted_charges();
            assert_eq!(log.len(), 1);
            assert_eq!(log[0].point, point);
            let observed = [
                log[0].integer_bits,
                log[0].decimal_digits,
                log[0].scale_expansion,
                log[0].text_input_bytes,
                log[0].text_scalars,
                log[0].normalized_scalars,
                log[0].unit_edges,
                log[0].value_occurrences,
            ];
            for (index, kind) in KINDS[..8].iter().enumerate() {
                let expected = requests
                    .iter()
                    .flatten()
                    .filter(|(requested, _)| requested == kind)
                    .map(|(_, amount)| *amount)
                    .max();
                assert_eq!(observed[index], expected);
            }
        } else {
            assert!(direct.admitted_charges().is_empty());
        }
    }
    match (actual, expected) {
        (Ok(()), Ok(output)) => {
            assert_eq!(direct.counters.consumed, output);
            assert_eq!(direct.admission_count(), u64::MAX);
        }
        (Err(actual), Err(expected)) => {
            assert_eq!(actual.limit_kind, expected.kind);
            assert_eq!(actual.limit, expected.limit);
            assert_eq!(actual.consumed, expected.consumed);
            assert_eq!(actual.next_charge.to_u64(), Some(expected.next));
            assert_eq!(actual.charge_point, expected.point);
            assert_eq!(direct.counters.consumed, consumed);
            assert_eq!(direct.admission_count(), u64::MAX - 1);
        }
        _ => panic!("common charge disagrees with independent transition"),
    }
}

#[test]
#[trace(
    "FR-359-AC-1",
    "FR-359-AC-2",
    "FR-359-AC-5",
    "FR-359-AC-6",
    "FR-359-AC-8"
)]
fn units_charge_matches_independent_boundary_model() {
    let edges = [0, 1, 2, u64::MAX - 1, u64::MAX];
    for first in KINDS {
        for second in KINDS {
            for amount in edges {
                for limit in edges {
                    let requests = [
                        Some((first, amount)),
                        Some((second, amount.saturating_add(1))),
                    ];
                    check(
                        [limit; 10],
                        [limit.saturating_sub(1); 10],
                        requests,
                        1,
                        amount,
                    );
                    check([limit; 10], [0; 10], [requests[1], requests[0]], amount, 1);
                }
            }
        }
    }
    for work in edges {
        for results in edges {
            check(
                [u64::MAX; 10],
                [u64::MAX - 1; 10],
                [None, None],
                work,
                results,
            );
        }
    }
}

#[test]
#[trace("FR-359-AC-6", "FR-359-AC-7", "FR-359-AC-8")]
fn units_charge_matches_independent_mixed_state_model() {
    let mut state = 0x7053_5900_0000_0001_u64;
    let mut draw = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        state
    };
    for _ in 0..4096 {
        let values = core::array::from_fn(|_| draw());
        let consumed = core::array::from_fn(|_| draw());
        let first = KINDS[(draw() % 10) as usize];
        let second = KINDS[(draw() % 10) as usize];
        let requests = [
            Some((first, draw())),
            if draw() & 1 == 0 {
                Some((second, draw()))
            } else {
                None
            },
        ];
        check(values, consumed, requests, draw(), draw());
    }
}

#[test]
#[trace("FR-359-AC-3", "FR-359-AC-4", "FR-359-AC-8")]
fn units_adapter_keeps_exact_and_unrestricted_fallback() {
    let large = Integer::from(u64::MAX).add(&Integer::one());
    let charges = [
        Charge::new(ChargePoint::FunctionCall).exact_size(LimitKind::IntegerBits, large.clone()),
        Charge::new(ChargePoint::FunctionCall).work(large.clone()),
        Charge::new(ChargePoint::FunctionCall).exact_results(large.clone()),
        Charge::new(ChargePoint::FunctionCall)
            .exact_size(LimitKind::IntegerBits, Integer::from(-1_i64)),
        Charge::new(ChargePoint::FunctionCall)
            .size(LimitKind::IntegerBits, 1)
            .size(LimitKind::IntegerBits, 2)
            .size(LimitKind::IntegerBits, 3),
    ];
    for (index, charge) in charges.into_iter().enumerate() {
        assert!(UnitsCharge::from_charge(&charge).is_none());
        let mut actual = Meter::new(limits([u64::MAX; 10]));
        let mut expected = actual.clone();
        let outcome = actual.charge(charge.clone());
        assert_eq!(outcome, expected.charge_general(charge));
        assert_eq!(actual, expected);
        if index < 4 {
            let error = outcome.unwrap_err();
            let kind = [
                LimitKind::IntegerBits,
                LimitKind::WorkUnits,
                LimitKind::ResultUnits,
                LimitKind::IntegerBits,
            ][index];
            assert_eq!(error.limit_kind, kind);
            assert_eq!(error.limit, u64::MAX);
            assert_eq!(error.consumed, 0);
            assert_eq!(
                error.next_charge,
                if index == 3 {
                    Integer::from(-1_i64)
                } else {
                    large.clone()
                }
            );
            assert_eq!(error.charge_point, ChargePoint::FunctionCall);
            assert_eq!(actual.admission_count(), 0);
        } else {
            assert!(outcome.is_ok());
            assert_eq!(actual.consumed(LimitKind::IntegerBits), 3);
            assert_eq!(actual.consumed(LimitKind::WorkUnits), 1);
            assert_eq!(actual.admission_count(), 1);
        }
    }
}

#[test]
#[trace("FR-359-AC-6", "FR-359-AC-8")]
fn units_injection_cancellation_and_saturation_match_general() {
    for occurrence in [1, 2, 3, u64::MAX] {
        for count in [0, 1, 2, u64::MAX - 1, u64::MAX] {
            for cancelled in [false, true] {
                let point = ChargePoint::FunctionCall;
                let cancel = Cancel::new();
                if cancelled {
                    cancel.cancel(crate::CancelCause::Requested);
                }
                let mut direct = Meter::new(limits([u64::MAX; 10]))
                    .with_cancel(cancel.clone())
                    .with_injected_denial(InjectedDenial {
                        point,
                        occurrence: NonZeroU64::new(occurrence).unwrap(),
                    });
                direct.counters.denied_point_admissions = count;
                direct.counters.admissions = count;
                let mut general = direct.clone();
                let mut pending = true;
                let mut expected_count = count;
                let mut expected_admissions = count;
                let mut expected_units = 0;
                for _ in 0..3 {
                    let hit =
                        !cancelled && pending && expected_count.checked_add(1) == Some(occurrence);
                    let denied = cancelled || hit;
                    let charge = Charge::new(point).results(1);
                    assert_eq!(
                        direct.charge_units(UnitsCharge::from_charge(&charge).unwrap()),
                        general.charge_general(charge)
                    );
                    assert_eq!(direct, general);
                    if hit {
                        pending = false;
                    }
                    if !denied {
                        expected_units += 1;
                        expected_admissions = expected_admissions.saturating_add(1);
                        if pending {
                            expected_count = expected_count.saturating_add(1);
                        }
                    }
                    assert_eq!(direct.counters.consumed[8], expected_units);
                    assert_eq!(direct.counters.consumed[9], expected_units);
                    assert_eq!(direct.counters.denial.is_some(), pending);
                    assert_eq!(direct.counters.denied_point_admissions, expected_count);
                    assert_eq!(direct.admission_count(), expected_admissions);
                    #[cfg(feature = "test-support")]
                    assert_eq!(direct.admitted_charges().len(), expected_units as usize);
                }
                if cancelled {
                    assert!(cancel.tripped().is_some());
                    assert!(direct.counters.denial.is_some());
                }
            }
        }
    }
}

#[test]
#[trace("FR-359-AC-6", "FR-359-AC-8")]
fn units_boolean_retention_preserves_real_meter_stops() {
    for limit in [0, 1, 2, u64::MAX] {
        for consumed in [0, 1, u64::MAX - 1, u64::MAX] {
            for cancelled in [false, true] {
                for injected in [false, true] {
                    let cancel = Cancel::new();
                    if cancelled {
                        cancel.cancel(crate::CancelCause::Deadline);
                    }
                    let mut actual = Meter::new(limits([limit; 10])).with_cancel(cancel);
                    actual.counters.consumed[8] = consumed;
                    actual.counters.consumed[9] = consumed;
                    if injected {
                        actual = actual.with_injected_denial(InjectedDenial {
                            point: ChargePoint::BooleanResultRetain,
                            occurrence: NonZeroU64::new(1).unwrap(),
                        });
                    }
                    let mut reference = actual.clone();
                    let expected = reference
                        .charge_general(Charge::new(ChargePoint::BooleanResultRetain).results(1))
                        .map(|()| true);
                    assert_eq!(crate::retain_boolean(true, &mut actual), expected);
                    assert_eq!(actual, reference);
                }
            }
        }
    }
}

#[test]
#[trace("FR-359-AC-8")]
fn units_and_adapter_poll_cancel_exactly_once_per_attempt() {
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};
    let polls = Arc::new(AtomicUsize::new(0));
    let observed = polls.clone();
    let cancel = Cancel::observing(move || {
        observed.fetch_add(1, Ordering::SeqCst);
    });
    let mut meter = Meter::new(limits([0; 10])).with_cancel(cancel.clone());
    for attempt in 1..=3 {
        if attempt == 3 {
            cancel.cancel(crate::CancelCause::Requested);
        }
        let charge = UnitsCharge::new(ChargePoint::FunctionCall);
        assert!(meter.charge_units(charge).is_err());
        assert_eq!(polls.load(Ordering::SeqCst), attempt * 2 - 1);
        assert!(meter
            .charge(Charge::new(ChargePoint::FunctionCall))
            .is_err());
        assert_eq!(polls.load(Ordering::SeqCst), attempt * 2);
    }
}

#[test]
#[trace("FR-359-AC-6", "FR-359-AC-8", "FR-359-AC-12", "FR-383-AC-1")]
fn units_public_and_fallback_share_one_meter_and_pending_denial() {
    let point = ChargePoint::FunctionCall;
    let cancel = Cancel::new();
    let mut meter = Meter::new(limits([10; 10]))
        .with_cancel(cancel.clone())
        .with_injected_denial(InjectedDenial {
            point,
            occurrence: NonZeroU64::new(4).unwrap(),
        });
    assert_eq!(meter.cancel(), Some(&cancel));
    meter
        .charge_units(UnitsCharge::one(point, LimitKind::IntegerBits, 0))
        .unwrap();
    meter
        .charge(Charge::new(point).size(LimitKind::IntegerBits, 2))
        .unwrap();
    // A normal denial at the same point cannot spend the pending occurrence.
    let before = meter.clone();
    let error = meter
        .charge_units(UnitsCharge::two(
            point,
            (LimitKind::IntegerBits, 11),
            (LimitKind::IntegerBits, 12),
        ))
        .unwrap_err();
    assert_eq!(error.next_charge.to_u64(), Some(11));
    assert_eq!(meter, before);
    meter
        .charge(
            Charge::new(point)
                .size(LimitKind::IntegerBits, 3)
                .size(LimitKind::IntegerBits, 7)
                .size(LimitKind::IntegerBits, 0),
        )
        .unwrap();
    assert_eq!(meter.consumed(LimitKind::IntegerBits), 7);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
    assert_eq!(meter.admission_count(), 3);
    let before = meter.clone();
    let error = meter.charge_units(UnitsCharge::new(point)).unwrap_err();
    assert_eq!(
        (
            error.limit_kind,
            error.limit,
            error.consumed,
            error.next_charge.to_u64(),
            error.charge_point
        ),
        (LimitKind::WorkUnits, 3, 3, Some(1), point)
    );
    assert_eq!(meter.counters.consumed, before.counters.consumed);
    assert_eq!(meter.admission_count(), 3);
    assert!(meter.counters.denial.is_none());
    #[cfg(feature = "test-support")]
    {
        let log = meter.admitted_charges();
        assert_eq!(log.len(), 3);
        assert_eq!(log[0].integer_bits, Some(0));
        assert_eq!(log[1].integer_bits, Some(2));
        assert_eq!(log[2].integer_bits, Some(7));
        assert_eq!(log, before.admitted_charges());
    }
}
