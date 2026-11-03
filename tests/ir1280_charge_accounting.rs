// SPDX-License-Identifier: AGPL-3.0-or-later
//! Fixed-size production interval accounting.
use core::num::NonZeroU64;
use ix_trace_rs::trace;
use quire_exact::{
    Cancel, CancelCause, Charge, ChargeAccounting, ChargePoint, InjectedDenial, Integer, LimitKind,
    Meter, ScalarLimits,
};

const KINDS: [LimitKind; 10] = [
    LimitKind::IntegerBits,
    LimitKind::DecimalDigits,
    LimitKind::ScaleExpansion,
    LimitKind::TextInputBytes,
    LimitKind::TextScalars,
    LimitKind::NormalizedScalars,
    LimitKind::UnitEdges,
    LimitKind::ValueOccurrences,
    LimitKind::WorkUnits,
    LimitKind::ResultUnits,
];
fn limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 100,
        decimal_digits: 100,
        scale_expansion: 100,
        text_input_bytes: 100,
        text_scalars: 100,
        normalized_scalars: 100,
        unit_edges: 100,
        value_occurrences: 100,
        work_units: 10000,
        result_units: 10000,
    }
}
fn charge(work: u64, results: u64) -> Charge {
    Charge::new(ChargePoint::FunctionCall)
        .work(Integer::from(work))
        .results(results)
}
fn amounts(a: &ChargeAccounting) -> [u64; 10] {
    KINDS.map(|k| a.amount(k))
}
fn interval(m: &Meter) -> ChargeAccounting {
    m.clone().take_charge_accounting().unwrap()
}
fn assert_budget(a: &Meter, b: &Meter) {
    assert_eq!(a.limits(), b.limits());
    assert_eq!(a.admission_count(), b.admission_count());
    for k in KINDS {
        assert_eq!(a.consumed(k), b.consumed(k));
    }
    assert_eq!(a.cancel(), b.cancel());
}

#[trace("TC-906", "FR-358-AC-16")]
#[test]
fn lifecycle_preserves_budget_denial_and_unread_amounts() {
    let cancel = Cancel::new();
    let mut m = Meter::new(limits())
        .with_cancel(cancel.clone())
        .with_injected_denial(InjectedDenial {
            point: ChargePoint::EqualityPlan,
            occurrence: NonZeroU64::new(1).unwrap(),
        });
    assert_eq!(m.take_charge_accounting(), None);
    assert_eq!(m.disable_charge_accounting(), None);
    m.charge(charge(7, 2)).unwrap();
    let before = m.clone();
    m.enable_charge_accounting();
    assert_eq!(amounts(&m.take_charge_accounting().unwrap()), [0; 10]);
    assert_eq!(amounts(&m.take_charge_accounting().unwrap()), [0; 10]);
    assert_budget(&m, &before);
    assert_eq!(cancel.tripped(), None);
    m.charge(charge(3, 4)).unwrap();
    let before = m.clone();
    m.enable_charge_accounting();
    assert_eq!(amounts(&interval(&m)), [0, 0, 0, 0, 0, 0, 0, 0, 3, 4]);
    assert_eq!(m.disable_charge_accounting(), Some(interval(&before)));
    assert_budget(&m, &before);
    assert_eq!(m.take_charge_accounting(), None);
    assert_eq!(m.disable_charge_accounting(), None);
    m.enable_charge_accounting();
    assert_eq!(amounts(&interval(&m)), [0; 10]);
    assert!(m.charge_plan(&Integer::from(2_u64)).is_err());
    assert_eq!(amounts(&interval(&m)), [0; 10]);
    m.charge_plan(&Integer::from(2_u64)).unwrap();
}

#[trace("TC-906", "FR-358-AC-17")]
#[test]
fn all_ten_amounts_capture_actual_requests_and_canonical_increments() {
    let mut m = Meter::new(limits());
    m.charge(charge(6, 1).size(LimitKind::IntegerBits, 9))
        .unwrap();
    m.enable_charge_accounting();
    m.charge(charge(2, 3).size(LimitKind::IntegerBits, 3))
        .unwrap();
    assert_eq!(interval(&m).amount(LimitKind::IntegerBits), 3);
    assert_eq!(m.consumed(LimitKind::IntegerBits), 9);
    let mut general = charge(4, 5);
    for (i, k) in KINDS[..8].iter().enumerate() {
        general = general.size(*k, (i + 1) as u64);
    }
    general = general
        .size(LimitKind::IntegerBits, 2)
        .size(LimitKind::IntegerBits, 0)
        .size(LimitKind::WorkUnits, 99)
        .size(LimitKind::ResultUnits, 88);
    m.charge(general).unwrap();
    m.charge(charge(7, 11)).unwrap();
    m.charge(charge(0, 0).size(LimitKind::UnitEdges, 0))
        .unwrap();
    assert_eq!(amounts(&interval(&m)), [3, 2, 3, 4, 5, 6, 7, 8, 13, 19]);
}

#[trace("TC-906", "FR-358-AC-18")]
#[test]
fn every_refusal_preserves_prefix_and_plan_records_actual_commit() {
    for failure in 0..9 {
        let mut l = limits();
        l.work_units = 10;
        l.result_units = 10;
        l.value_occurrences = 4;
        let cancel = Cancel::new();
        let mut m = Meter::new(l).with_cancel(cancel.clone());
        m.enable_charge_accounting();
        m.charge(charge(2, 2).size(LimitKind::IntegerBits, 3))
            .unwrap();
        if failure == 3 {
            m = m.with_injected_denial(InjectedDenial {
                point: ChargePoint::FunctionCall,
                occurrence: NonZeroU64::new(1).unwrap(),
            });
        }
        if failure == 4 {
            cancel.cancel(CancelCause::Requested);
        }
        let before = interval(&m);
        let result = match failure {
            0 => m.charge(charge(9, 0)),
            1 => m.charge(charge(0, 9)),
            2 => m.charge(charge(1, 1).size(LimitKind::IntegerBits, 101)),
            3 | 4 => m.charge(charge(1, 1)),
            5 => m.charge_plan(&Integer::from(5_u64)),
            6 => {
                m.charge(charge(7, 0)).unwrap();
                let prefix = interval(&m);
                assert!(m.charge_plan(&Integer::from(2_u64)).is_err());
                assert_eq!(interval(&m), prefix);
                continue;
            }
            7 => {
                m.charge(charge(0, 8)).unwrap();
                let prefix = interval(&m);
                assert!(m.charge_plan(&Integer::from(2_u64)).is_err());
                assert_eq!(interval(&m), prefix);
                continue;
            }
            _ => m.charge(
                Charge::new(ChargePoint::FunctionCall)
                    .exact_results(Integer::from(u64::MAX).add(&Integer::one())),
            ),
        };
        assert!(result.is_err(), "failure {failure}");
        assert_eq!(interval(&m), before);
        if failure != 4 {
            m.charge(charge(1, 1)).unwrap();
            assert_eq!(amounts(&interval(&m)), [3, 0, 0, 0, 0, 0, 0, 0, 3, 3]);
        }
    }
    let mut m = Meter::new(limits());
    m.enable_charge_accounting();
    m.charge_plan(&Integer::from(2_u64)).unwrap();
    assert_eq!(amounts(&interval(&m)), [0, 0, 0, 0, 0, 0, 0, 2, 1, 0]);
}

#[trace("TC-906", "FR-358-AC-19")]
#[test]
fn partitions_conserve_usage_and_capture_does_not_change_execution() {
    let mut partition = Meter::new(limits());
    partition
        .charge(charge(9, 8).size(LimitKind::IntegerBits, 90))
        .unwrap();
    let mut whole = partition.clone();
    let mut disabled = partition.clone();
    partition.enable_charge_accounting();
    whole.enable_charge_accounting();
    let mut combined = [0; 10];
    assert_eq!(
        amounts(&partition.take_charge_accounting().unwrap()),
        [0; 10]
    );
    for (w, r, size) in [(2, 3, 7), (5, 1, 4), (1, 6, 9)] {
        let c = charge(w, r).size(LimitKind::IntegerBits, size);
        assert_eq!(partition.charge(c.clone()), disabled.charge(c.clone()));
        whole.charge(c).unwrap();
        partition.enable_charge_accounting();
        let part = amounts(&partition.take_charge_accounting().unwrap());
        for i in 0..10 {
            if i < 8 {
                combined[i] = combined[i].max(part[i]);
            } else {
                combined[i] += part[i];
            }
        }
    }
    assert_eq!(combined, [9, 0, 0, 0, 0, 0, 0, 0, 8, 10]);
    assert_eq!(
        combined,
        amounts(&whole.disable_charge_accounting().unwrap())
    );
    assert_eq!(
        amounts(&partition.disable_charge_accounting().unwrap()),
        [0; 10]
    );
    for m in [&mut partition, &mut whole, &mut disabled] {
        assert!(m.charge(charge(10001, 0)).is_err());
    }
    assert_budget(&partition, &whole);
    assert_budget(&partition, &disabled);
}

#[trace("TC-906", "FR-358-AC-20")]
#[test]
fn capture_continues_after_diagnostic_prefix() {
    let mut m = Meter::new(limits());
    m.enable_charge_accounting();
    for i in 1..=4097 {
        m.charge(charge(1, 1).size(LimitKind::IntegerBits, if i == 4097 { 99 } else { 3 }))
            .unwrap();
        #[cfg(feature = "test-support")]
        {
            assert_eq!(m.charge_log_truncated(), i > 4096);
        }
    }
    assert_eq!(
        amounts(&interval(&m)),
        [99, 0, 0, 0, 0, 0, 0, 0, 4097, 4097]
    );
    assert_eq!(m.admission_count(), 4097);
    #[cfg(feature = "test-support")]
    {
        assert_eq!(m.admitted_charges().len(), 4096);
        for record in m.admitted_charges() {
            assert_eq!(record.point, ChargePoint::FunctionCall);
            assert_eq!(record.integer_bits, Some(3));
        }
    }
}

#[trace("TC-906", "FR-358-AC-21")]
#[test]
fn clones_are_independent_and_capture_controls_allocate_nothing() {
    fn traits<T: Clone + core::fmt::Debug + Eq + Send + Sync>() {}
    traits::<Meter>();
    traits::<ChargeAccounting>();
    assert!(!core::mem::needs_drop::<ChargeAccounting>());
    let mut m = Meter::new(limits());
    m.enable_charge_accounting();
    m.charge(charge(2, 3)).unwrap();
    let mut clone = m.clone();
    let expected = m.take_charge_accounting().unwrap();
    assert_eq!(clone.take_charge_accounting(), Some(expected.clone()));
    assert_eq!(amounts(&interval(&m)), [0; 10]);
    clone.charge(charge(1, 0)).unwrap();
    assert_eq!(interval(&clone).amount(LimitKind::WorkUnits), 1);
    assert_eq!(interval(&m).amount(LimitKind::WorkUnits), 0);
    let info = allocation_counter::measure(|| {
        let mut m = Meter::new(limits());
        assert_eq!(m.take_charge_accounting(), None);
        m.enable_charge_accounting();
        m.enable_charge_accounting();
        let _ = m.take_charge_accounting();
        let _ = m.disable_charge_accounting();
    });
    assert_eq!(info.count_total, 0);
}
