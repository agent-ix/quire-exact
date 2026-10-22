// SPDX-License-Identifier: AGPL-3.0-or-later
//! Full-width common-charge transitions against independent field-order math.
use super::units_reference::{self as reference, KINDS};
use super::*;

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

fn check(adapter: bool) {
    let limit_values: [u64; 10] = kani::any();
    let consumed: [u64; 10] = kani::any();
    let admissions: u64 = kani::any();
    let request = || {
        if kani::any::<bool>() {
            Some((
                KINDS[usize::from(kani::any::<u8>()) % 10],
                kani::any::<u64>(),
            ))
        } else {
            None
        }
    };
    let first = request();
    // Public construction keeps insertion order; zero/one/two requests all occur.
    let second = if first.is_some() { request() } else { None };
    let work: u64 = kani::any();
    let results: u64 = kani::any();
    let point = ChargePoint::FunctionCall;
    let mut meter = Meter::new(limits(limit_values));
    meter.counters.consumed = consumed;
    meter.counters.admissions = admissions;
    let expected = reference::transition(
        limit_values,
        consumed,
        [first, second],
        work,
        results,
        point,
    );
    let actual = if adapter {
        let mut charge = Charge::new(point)
            .work(Integer::from(work))
            .results(results);
        if let Some((kind, amount)) = first {
            charge = charge.size(kind, amount);
        }
        if let Some((kind, amount)) = second {
            charge = charge.size(kind, amount);
        }
        meter.charge(charge)
    } else {
        meter.charge_units(UnitsCharge {
            point,
            first,
            second,
            work,
            results,
        })
    };
    macro_rules! counters_equal {
        ($right:expr) => {{
            let right = $right;
            assert_eq!(meter.counters.consumed[0], right[0]);
            assert_eq!(meter.counters.consumed[1], right[1]);
            assert_eq!(meter.counters.consumed[2], right[2]);
            assert_eq!(meter.counters.consumed[3], right[3]);
            assert_eq!(meter.counters.consumed[4], right[4]);
            assert_eq!(meter.counters.consumed[5], right[5]);
            assert_eq!(meter.counters.consumed[6], right[6]);
            assert_eq!(meter.counters.consumed[7], right[7]);
            assert_eq!(meter.counters.consumed[8], right[8]);
            assert_eq!(meter.counters.consumed[9], right[9]);
        }};
    }
    match (actual, expected) {
        (Ok(()), Ok(output)) => {
            counters_equal!(output);
            assert_eq!(meter.admission_count(), admissions.saturating_add(1));
            kani::cover!(first.is_none(), "zero-request common admission completes");
            kani::cover!(
                first.is_some() && second.is_none(),
                "one-request common admission completes"
            );
            kani::cover!(
                first.is_some() && second.is_some(),
                "two-request common admission completes"
            );
        }
        (Err(actual), Err(expected)) => {
            assert_eq!(actual.limit_kind, expected.kind);
            assert_eq!(actual.limit, expected.limit);
            assert_eq!(actual.consumed, expected.consumed);
            assert_eq!(actual.next_charge.to_u64(), Some(expected.next));
            assert_eq!(actual.charge_point, expected.point);
            counters_equal!(consumed);
            assert_eq!(meter.admission_count(), admissions);
            kani::cover!(
                first.is_some() && second.is_some(),
                "two-request common denial is atomic"
            );
        }
        _ => panic!("production common charge disagrees with independent transition"),
    }
}

#[kani::proof]
fn units_charge_matches_independent_transition() {
    check(false);
}

#[kani::proof]
fn public_units_adapter_matches_independent_transition() {
    check(true);
}

#[kani::proof]
fn units_injection_cancellation_preserves_occurrence_and_saturation() {
    let point = ChargePoint::FunctionCall;
    let cancelled: bool = kani::any();
    let same_point: bool = kani::any();
    let occurrence = NonZeroU64::new(kani::any()).unwrap_or(NonZeroU64::MIN);
    let count: u64 = kani::any();
    let admissions: u64 = kani::any();
    let work_consumed: u64 = kani::any();
    let result_consumed: u64 = kani::any();
    let work: u64 = kani::any();
    let results: u64 = kani::any();
    let cancel = Cancel::new();
    if cancelled {
        cancel.cancel(crate::CancelCause::Requested);
    }
    let denial = InjectedDenial {
        point: if same_point {
            point
        } else {
            ChargePoint::BooleanResultRetain
        },
        occurrence,
    };
    let mut meter = Meter::new(limits([u64::MAX; 10]))
        .with_cancel(cancel.clone())
        .with_injected_denial(denial);
    meter.counters.denied_point_admissions = count;
    meter.counters.admissions = admissions;
    meter.counters.consumed[8] = work_consumed;
    meter.counters.consumed[9] = result_consumed;
    let before = meter.counters.consumed;
    let charge = UnitsCharge {
        point,
        first: None,
        second: None,
        work,
        results,
    };
    let actual = meter.charge_units(charge);
    let completed = actual.is_ok();
    let injected = !cancelled && same_point && count.checked_add(1) == Some(occurrence.get());
    if cancelled || injected {
        let error = actual.unwrap_err();
        assert_eq!(error.limit_kind, LimitKind::WorkUnits);
        assert_eq!(error.limit, work_consumed);
        assert_eq!(error.consumed, work_consumed);
        assert_eq!(error.next_charge.to_u64(), Some(work));
        assert_eq!(error.charge_point, point);
        assert_eq!(meter.admission_count(), admissions);
        assert_eq!(meter.counters.denied_point_admissions, count);
        assert_eq!(meter.counters.consumed[8], work_consumed);
        assert_eq!(meter.counters.consumed[9], result_consumed);
    } else {
        let expected =
            reference::transition([u64::MAX; 10], before, [None, None], work, results, point);
        match (actual, expected) {
            (Ok(()), Ok(output)) => {
                assert_eq!(meter.counters.consumed[8], output[8]);
                assert_eq!(meter.counters.consumed[9], output[9]);
                assert_eq!(meter.admission_count(), admissions.saturating_add(1));
                assert_eq!(
                    meter.counters.denied_point_admissions,
                    if same_point {
                        count.saturating_add(1)
                    } else {
                        count
                    }
                );
            }
            (Err(actual), Err(expected)) => {
                assert_eq!(actual.limit_kind, expected.kind);
                assert_eq!(actual.limit, expected.limit);
                assert_eq!(actual.consumed, expected.consumed);
                assert_eq!(actual.next_charge.to_u64(), Some(expected.next));
                assert_eq!(actual.charge_point, expected.point);
                assert_eq!(meter.counters.consumed[8], work_consumed);
                assert_eq!(meter.counters.consumed[9], result_consumed);
                assert_eq!(meter.admission_count(), admissions);
                assert_eq!(meter.counters.denied_point_admissions, count);
            }
            _ => panic!("post-poll accounting disagrees with independent transition"),
        }
    }
    assert_eq!(
        meter.counters.denial,
        if injected { None } else { Some(denial) }
    );
    assert_eq!(
        cancel.tripped(),
        if cancelled {
            Some(crate::CancelCause::Requested)
        } else {
            None
        }
    );
    kani::cover!(
        cancelled && same_point && count == 0 && occurrence.get() == 1,
        "cancellation precedes pending injection"
    );
    kani::cover!(injected, "exact injected occurrence denies");
    kani::cover!(
        !cancelled && same_point && count == u64::MAX && completed,
        "occurrence count saturation still admits"
    );
}
