// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public charge vocabulary and meter evidence for temporal visits.

use std::num::NonZeroU64;

use ix_trace_rs::trace;
use quire_exact::{
    Charge, ChargePoint, Incomplete, InjectedDenial, Integer, LimitKind, Meter, ScalarLimits,
};

fn limits(work_units: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units,
        result_units: u64::MAX,
    }
}

fn temporal_visit() -> ChargePoint {
    ChargePoint::from_code("temporal.visit").expect("temporal.visit must be a charge point")
}

#[cfg(feature = "test-support")]
fn assert_plain_visit_record(meter: &Meter, point: ChargePoint) {
    let records = meter.admitted_charges();
    assert_eq!(records.len(), 1);
    let record = records[0];
    assert_eq!(record.point, point);
    assert_eq!(
        (
            record.integer_bits,
            record.decimal_digits,
            record.scale_expansion,
            record.text_input_bytes,
            record.text_scalars,
            record.normalized_scalars,
            record.unit_edges,
            record.value_occurrences,
        ),
        (None, None, None, None, None, None, None, None)
    );
}

#[trace("FR-362-AC-21", "TC-910")]
#[test]
fn temporal_visit_charges_one_work_unit_and_respects_the_boundary() {
    let point = temporal_visit();
    let mut meter = Meter::new(limits(1));
    meter.charge(Charge::new(point)).unwrap();
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    assert_eq!(meter.admission_count(), 1);
    for kind in LimitKind::ALL {
        if kind != LimitKind::WorkUnits {
            assert_eq!(meter.consumed(kind), 0, "{kind:?}");
        }
    }
    #[cfg(feature = "test-support")]
    assert_plain_visit_record(&meter, point);

    let mut denied = Meter::new(limits(0));
    let before = denied.clone();
    assert_eq!(
        denied.charge(Charge::new(point)).unwrap_err(),
        Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: Integer::one(),
            charge_point: point,
        }
    );
    assert_eq!(denied, before);
}

#[trace("FR-362-AC-22", "TC-910")]
#[test]
fn injected_temporal_visit_denial_is_atomic_and_one_shot() {
    let point = temporal_visit();
    let mut meter = Meter::new(limits(1)).with_injected_denial(InjectedDenial {
        point,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    assert_eq!(
        meter.charge(Charge::new(point)).unwrap_err(),
        Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: Integer::one(),
            charge_point: point,
        }
    );
    assert_eq!(meter.admission_count(), 0);
    for kind in LimitKind::ALL {
        assert_eq!(meter.consumed(kind), 0, "{kind:?}");
    }
    #[cfg(feature = "test-support")]
    assert!(meter.admitted_charges().is_empty());

    meter.charge(Charge::new(point)).unwrap();
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    assert_eq!(meter.admission_count(), 1);
    #[cfg(feature = "test-support")]
    assert_plain_visit_record(&meter, point);
}
