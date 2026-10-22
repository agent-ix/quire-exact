// SPDX-License-Identifier: AGPL-3.0-or-later
//! Full-width common-charge transitions against independent field-order math.
use super::units_reference::{self as reference, KINDS};
use super::*;

fn limits(values: [u64; 10]) -> ScalarLimits {
    ScalarLimits {
        integer_bits: values[0], decimal_digits: values[1], scale_expansion: values[2],
        text_input_bytes: values[3], text_scalars: values[4], normalized_scalars: values[5],
        unit_edges: values[6], value_occurrences: values[7], work_units: values[8], result_units: values[9],
    }
}

fn check(adapter: bool) {
    let limit_values: [u64; 10] = kani::any();
    let consumed: [u64; 10] = kani::any();
    let admissions: u64 = kani::any();
    let request = || if kani::any::<bool>() { Some((KINDS[usize::from(kani::any::<u8>()) % 10], kani::any::<u64>())) } else { None };
    let first = request();
    // Public construction keeps insertion order; zero/one/two requests all occur.
    let second = if first.is_some() { request() } else { None };
    let work: u64 = kani::any();
    let results: u64 = kani::any();
    let point = ChargePoint::FunctionCall;
    let mut meter = Meter::new(limits(limit_values));
    meter.counters.consumed = consumed;
    meter.counters.admissions = admissions;
    let expected = reference::transition(limit_values, consumed, [first, second], work, results, point);
    let actual = if adapter {
        let mut charge = Charge::new(point).work(Integer::from(work)).results(results);
        if let Some((kind, amount)) = first { charge = charge.size(kind, amount); }
        if let Some((kind, amount)) = second { charge = charge.size(kind, amount); }
        meter.charge(charge)
    } else {
        meter.charge_units(UnitsCharge { point, first, second, work, results })
    };
    match (actual, expected) {
        (Ok(()), Ok(output)) => {
            assert_eq!(meter.counters.consumed, output);
            assert_eq!(meter.admission_count(), admissions.saturating_add(1));
            kani::cover!(first.is_some() && second.is_some(), "two-request common admission completes");
        }
        (Err(actual), Err(expected)) => {
            assert_eq!(actual.limit_kind, expected.kind);
            assert_eq!(actual.limit, expected.limit);
            assert_eq!(actual.consumed, expected.consumed);
            assert_eq!(actual.next_charge.to_u64(), Some(expected.next));
            assert_eq!(actual.charge_point, expected.point);
            assert_eq!(meter.counters.consumed, consumed);
            assert_eq!(meter.admission_count(), admissions);
            kani::cover!(first.is_some() && second.is_some(), "two-request common denial is atomic");
        }
        _ => panic!("production common charge disagrees with independent transition"),
    }
}

#[kani::proof]
fn units_charge_matches_independent_transition() { check(false); }

#[kani::proof]
fn public_units_adapter_matches_independent_transition() { check(true); }
