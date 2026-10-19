// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public cancellation-handle controls for FR-383.

use ix_trace_rs::trace;
use quire_exact::{
    Cancel, CancelCause, Charge, ChargePoint, Incomplete, Integer, LimitKind, Meter, ScalarLimits,
};

fn limits() -> ScalarLimits {
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

#[trace("TC-933", "FR-383-AC-1")]
#[test]
fn accessor_reports_only_the_installed_handle() {
    let first = Cancel::new();
    let replacement = Cancel::new();
    let independent = Cancel::new();
    let meter = Meter::new(limits());
    assert_eq!(meter.cancel(), None);

    let meter = meter.with_cancel(first.clone());
    assert_eq!(meter.cancel(), Some(&first));
    assert_ne!(meter.cancel(), Some(&independent));
    assert_ne!(meter.cancel(), Some(&replacement));

    let meter = meter.with_cancel(replacement.clone());
    assert_eq!(meter.cancel(), Some(&replacement));
    assert_ne!(meter.cancel(), Some(&first));
    assert_ne!(meter.cancel(), Some(&independent));
}

#[trace("TC-933", "FR-383-AC-2")]
#[test]
fn accessor_handle_cancels_the_next_charge_without_changing_accounting() {
    let point = ChargePoint::FunctionCall;
    let charge = || Charge::new(point).work(Integer::from(3_u64)).results(2);
    let installed = Cancel::new();
    let mut meter = Meter::new(limits()).with_cancel(installed);
    meter
        .charge(charge())
        .expect("live handle admits the first charge");
    assert_eq!(meter.admission_count(), 1);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 2);

    let handle = meter.cancel().expect("meter has a handle").clone();
    let before = meter.clone();
    handle.cancel(CancelCause::Requested);
    assert_eq!(handle.tripped(), None);
    assert_eq!(
        meter
            .charge(charge())
            .expect_err("cancelled handle denies the next charge"),
        Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 3,
            consumed: 3,
            next_charge: Integer::from(3_u64),
            charge_point: point,
        }
    );
    assert_eq!(handle.tripped(), Some(CancelCause::Requested));
    assert_eq!(meter, before);

    let independent = Cancel::new();
    let live = Cancel::new();
    let mut other = Meter::new(limits()).with_cancel(live.clone());
    other
        .charge(charge())
        .expect("live handle admits the first charge");
    independent.cancel(CancelCause::Requested);
    other
        .charge(charge())
        .expect("independent cancellation cannot deny a charge");
    assert_eq!(other.admission_count(), 2);
    assert_eq!(other.consumed(LimitKind::WorkUnits), 6);
    assert_eq!(other.consumed(LimitKind::ResultUnits), 4);
    assert_eq!(live.tripped(), None);
}
