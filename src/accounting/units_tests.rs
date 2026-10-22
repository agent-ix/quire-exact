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

/// Trace: FR-359-AC-1, FR-359-AC-2, FR-359-AC-5, FR-359-AC-6, FR-359-AC-8
#[test]
#[trace("FR-359-AC-1", "FR-359-AC-2", "FR-359-AC-5", "FR-359-AC-6", "FR-359-AC-8")]
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

/// Trace: FR-359-AC-6, FR-359-AC-7, FR-359-AC-8
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

/// Trace: FR-359-AC-3, FR-359-AC-4, FR-359-AC-8
#[test]
#[trace("FR-359-AC-3", "FR-359-AC-4", "FR-359-AC-8")]
fn units_adapter_keeps_exact_and_unrestricted_fallback() {
    let large = Integer::from(u64::MAX).add(&Integer::one());
    let charges = [
        Charge::new(ChargePoint::FunctionCall).exact_size(LimitKind::IntegerBits, large.clone()),
        Charge::new(ChargePoint::FunctionCall).work(large.clone()),
        Charge::new(ChargePoint::FunctionCall).exact_results(large),
        Charge::new(ChargePoint::FunctionCall)
            .exact_size(LimitKind::IntegerBits, Integer::from(-1_i64)),
        Charge::new(ChargePoint::FunctionCall)
            .size(LimitKind::IntegerBits, 1)
            .size(LimitKind::IntegerBits, 2)
            .size(LimitKind::IntegerBits, 3),
    ];
    for charge in charges {
        assert!(UnitsCharge::from_charge(&charge).is_none());
        let mut actual = Meter::new(limits([u64::MAX; 10]));
        let mut expected = actual.clone();
        assert_eq!(
            actual.charge(charge.clone()),
            expected.charge_general(charge)
        );
        assert_eq!(actual, expected);
    }
}

/// Trace: FR-359-AC-6, FR-359-AC-8, FR-383-AC-1
#[test]
#[trace("FR-359-AC-6", "FR-359-AC-8", "FR-383-AC-1")]
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
                for _ in 0..3 {
                    let charge = Charge::new(point).results(1);
                    assert_eq!(
                        direct.charge_units(UnitsCharge::from_charge(&charge).unwrap()),
                        general.charge_general(charge)
                    );
                    assert_eq!(direct, general);
                }
                if cancelled {
                    assert!(cancel.tripped().is_some());
                    assert!(direct.counters.denial.is_some());
                }
            }
        }
    }
}

/// Trace: FR-359-AC-6, FR-359-AC-8, FR-383-AC-1
#[test]
#[trace("FR-359-AC-6", "FR-359-AC-8", "FR-383-AC-1")]
fn units_boolean_retention_preserves_real_meter_stops() {
    for limit in [0, 1, 2, u64::MAX] {
        for consumed in [0, 1, u64::MAX - 1, u64::MAX] {
            for cancelled in [false, true] {
                for injected in [false, true] {
                    let cancel = Cancel::new();
                    if cancelled { cancel.cancel(crate::CancelCause::Deadline); }
                    let mut actual = Meter::new(limits([limit; 10])).with_cancel(cancel);
                    actual.counters.consumed[8] = consumed;
                    actual.counters.consumed[9] = consumed;
                    if injected { actual = actual.with_injected_denial(InjectedDenial { point: ChargePoint::BooleanResultRetain, occurrence: NonZeroU64::new(1).unwrap() }); }
                    let mut reference = actual.clone();
                    let expected = reference.charge_general(Charge::new(ChargePoint::BooleanResultRetain).results(1)).map(|()| true);
                    assert_eq!(crate::retain_boolean(true, &mut actual), expected);
                    assert_eq!(actual, reference);
                }
            }
        }
    }
}
