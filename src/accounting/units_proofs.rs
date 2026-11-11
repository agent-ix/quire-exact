// SPDX-License-Identifier: AGPL-3.0-or-later
//! Full-width common-charge transitions against independent field-order math.
use super::units_reference::{self as reference, KINDS};
use super::*;

/// Names the failing check: Kani drops a formatted `assert!` message, but
/// `kani::assert` reports its literal, so a mutant failure identifies the check.
macro_rules! named {
    ($condition:expr, $name:literal $(,)?) => {
        kani::assert($condition, $name)
    };
}

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

fn request() -> Option<(LimitKind, u64)> {
    if kani::any::<bool>() {
        Some((
            KINDS[usize::from(kani::any::<u8>()) % 10],
            kani::any::<u64>(),
        ))
    } else {
        None
    }
}

fn check() {
    let limit_values: [u64; 10] = kani::any();
    let consumed: [u64; 10] = kani::any();
    let admissions: u64 = kani::any();
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
    let actual = meter.charge_units(UnitsCharge {
        point,
        first,
        second,
        work,
        results,
    });
    macro_rules! counters_equal {
        ($right:expr) => {{
            let right = $right;
            named!(
                meter.counters.consumed[0] == right[0],
                "consumed[0] matches"
            );
            named!(
                meter.counters.consumed[1] == right[1],
                "consumed[1] matches"
            );
            named!(
                meter.counters.consumed[2] == right[2],
                "consumed[2] matches"
            );
            named!(
                meter.counters.consumed[3] == right[3],
                "consumed[3] matches"
            );
            named!(
                meter.counters.consumed[4] == right[4],
                "consumed[4] matches"
            );
            named!(
                meter.counters.consumed[5] == right[5],
                "consumed[5] matches"
            );
            named!(
                meter.counters.consumed[6] == right[6],
                "consumed[6] matches"
            );
            named!(
                meter.counters.consumed[7] == right[7],
                "consumed[7] matches"
            );
            named!(
                meter.counters.consumed[8] == right[8],
                "consumed[8] matches"
            );
            named!(
                meter.counters.consumed[9] == right[9],
                "consumed[9] matches"
            );
        }};
    }
    match (actual, expected) {
        (Ok(()), Ok(output)) => {
            counters_equal!(output);
            named!(
                meter.admission_count() == admissions.saturating_add(1),
                "admission count advances"
            );
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
            named!(actual.limit_kind == expected.kind, "refused kind");
            named!(actual.limit == expected.limit, "refused limit");
            named!(actual.consumed == expected.consumed, "refused consumed");
            named!(
                actual.next_charge.native() == Some(i128::from(expected.next)),
                "refused next_charge"
            );
            named!(actual.charge_point == expected.point, "refused point");
            counters_equal!(consumed);
            named!(
                meter.admission_count() == admissions,
                "admission count unchanged"
            );
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
    check();
}

/// `Charge` built through the common-path setters maps onto the direct
/// `UnitsCharge`. Straight-line construction keeps `ChargeStorage` `Common`;
/// the `Integer`-typed `Charge::work` entry and the general storage are
/// host-test obligations (`units_tests.rs`).
#[kani::proof]
fn units_adapter_maps_common_charge_to_units_charge() {
    let point = ChargePoint::FunctionCall;
    let first = request();
    let second = if first.is_some() { request() } else { None };
    let work: u64 = kani::any();
    let results: u64 = kani::any();
    let mut charge = Charge::new(point)
        .set_units(work, false)
        .set_units(results, true);
    if let Some((kind, amount)) = first {
        charge = charge.push_units(kind, amount);
    }
    if let Some((kind, amount)) = second {
        charge = charge.push_units(kind, amount);
    }
    let Some(units) = UnitsCharge::from_charge(&charge) else {
        panic!("common-path charge lost its common storage");
    };
    named!(units.point == point, "adapter keeps the charge point");
    named!(units.first == first, "adapter keeps the first request");
    named!(units.second == second, "adapter keeps the second request");
    named!(units.work == work, "adapter keeps the work amount");
    named!(units.results == results, "adapter keeps the result amount");
    kani::cover!(first.is_some() && second.is_some(), "two requests map");
    kani::cover!(first.is_none(), "zero requests map");
}

#[kani::proof]
fn units_injection_cancellation_preserves_occurrence_and_saturation() {
    let point = ChargePoint::FunctionCall;
    let cancelled: bool = kani::any();
    let same_point: bool = kani::any();
    let occurrence = NonZeroU64::new(kani::any()).unwrap_or(NonZeroU64::MIN);
    let count: u64 = kani::any();
    let admissions: u64 = kani::any();
    let consumed: [u64; 10] = kani::any();
    let work_consumed = consumed[8];
    let result_consumed = consumed[9];
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
    meter.counters.consumed = consumed;
    let before = consumed;
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
        named!(error.limit_kind == LimitKind::WorkUnits, "injected kind");
        named!(error.limit == work_consumed, "injected limit");
        named!(error.consumed == work_consumed, "injected consumed");
        named!(
            error.next_charge.native() == Some(i128::from(work)),
            "injected next_charge"
        );
        named!(error.charge_point == point, "injected point");
        named!(
            meter.admission_count() == admissions,
            "admission count unchanged"
        );
        named!(
            meter.counters.denied_point_admissions == count,
            "occurrence count unchanged"
        );
        named!(
            meter.counters.consumed[8] == work_consumed,
            "work unchanged"
        );
        named!(
            meter.counters.consumed[9] == result_consumed,
            "results unchanged"
        );
    } else {
        let expected =
            reference::transition([u64::MAX; 10], before, [None, None], work, results, point);
        match (actual, expected) {
            (Ok(()), Ok(output)) => {
                named!(meter.counters.consumed[8] == output[8], "work advances");
                named!(meter.counters.consumed[9] == output[9], "results advance");
                named!(
                    meter.admission_count() == admissions.saturating_add(1),
                    "admission count advances"
                );
                named!(
                    meter.counters.denied_point_admissions
                        == if same_point {
                            count.saturating_add(1)
                        } else {
                            count
                        },
                    "occurrence count advances on the injected point"
                );
            }
            (Err(actual), Err(expected)) => {
                named!(actual.limit_kind == expected.kind, "refused kind");
                named!(actual.limit == expected.limit, "refused limit");
                named!(actual.consumed == expected.consumed, "refused consumed");
                named!(
                    actual.next_charge.native() == Some(i128::from(expected.next)),
                    "refused next_charge"
                );
                named!(actual.charge_point == expected.point, "refused point");
                named!(
                    meter.counters.consumed[8] == work_consumed,
                    "work unchanged"
                );
                named!(
                    meter.counters.consumed[9] == result_consumed,
                    "results unchanged"
                );
                named!(
                    meter.admission_count() == admissions,
                    "admission count unchanged"
                );
                named!(
                    meter.counters.denied_point_admissions == count,
                    "occurrence count unchanged"
                );
            }
            _ => panic!("post-poll accounting disagrees with independent transition"),
        }
    }
    named!(
        meter.counters.consumed[0] == before[0],
        "consumed[0] unchanged"
    );
    named!(
        meter.counters.consumed[1] == before[1],
        "consumed[1] unchanged"
    );
    named!(
        meter.counters.consumed[2] == before[2],
        "consumed[2] unchanged"
    );
    named!(
        meter.counters.consumed[3] == before[3],
        "consumed[3] unchanged"
    );
    named!(
        meter.counters.consumed[4] == before[4],
        "consumed[4] unchanged"
    );
    named!(
        meter.counters.consumed[5] == before[5],
        "consumed[5] unchanged"
    );
    named!(
        meter.counters.consumed[6] == before[6],
        "consumed[6] unchanged"
    );
    named!(
        meter.counters.consumed[7] == before[7],
        "consumed[7] unchanged"
    );
    named!(
        meter.counters.denial == if injected { None } else { Some(denial) },
        "pending injection state"
    );
    named!(
        cancel.tripped()
            == if cancelled {
                Some(crate::CancelCause::Requested)
            } else {
                None
            },
        "cancellation trip state"
    );
    kani::cover!(
        cancelled && same_point && count == 0 && occurrence.get() == 1,
        "cancellation precedes pending injection"
    );
    kani::cover!(injected, "exact injected occurrence denies");
    kani::cover!(
        !cancelled && !injected && !completed && same_point,
        "ordinary refusal leaves the future injection pending"
    );
    kani::cover!(
        !cancelled && same_point && count == u64::MAX && completed,
        "occurrence count saturation still admits"
    );
}
