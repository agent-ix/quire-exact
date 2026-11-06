// SPDX-License-Identifier: AGPL-3.0-or-later
//! Production per-charge capture across direct and opaque Exact meter boundaries.

use quire_exact::{
    evaluate_integer_arithmetic, evaluate_native_integer_ordering, Charge, ChargeAdmission,
    ChargePoint, Integer, IntegerArithmetic, LimitKind, Meter, NativeInteger, OrderingOperator,
    Outcome, ScalarLimits,
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

/// Trace: FR-358-AC-22
#[test]
fn capture_preserves_order_all_requests_and_denial() {
    let mut limits = limits();
    limits.result_units = 1;
    let mut meter = Meter::new(limits);
    meter.enable_charge_capture(3, 12);
    let sizes = [
        (LimitKind::IntegerBits, 3),
        (LimitKind::DecimalDigits, 4),
        (LimitKind::ScaleExpansion, 5),
        (LimitKind::TextInputBytes, 6),
        (LimitKind::TextScalars, 7),
        (LimitKind::NormalizedScalars, 8),
        (LimitKind::UnitEdges, 9),
        (LimitKind::ValueOccurrences, 10),
        (LimitKind::IntegerBits, 2),
    ];
    let mut charge = Charge::new(ChargePoint::FunctionCall)
        .work(2_u64.into())
        .results(1);
    for (kind, amount) in sizes {
        charge = charge.size(kind, amount);
    }
    meter.charge(charge).unwrap();
    let denied = meter
        .charge(
            Charge::new(ChargePoint::GraphEdge)
                .work(3_u64.into())
                .results(1),
        )
        .unwrap_err();
    assert_eq!(denied.limit_kind, LimitKind::ResultUnits);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 2);
    let capture = meter.disable_charge_capture().unwrap().unwrap();
    assert_eq!(capture.events().len(), 2);
    let first = &capture.events()[0];
    assert_eq!(first.point, ChargePoint::FunctionCall);
    assert_eq!(first.work_units, Integer::from(2_u64));
    assert_eq!(first.result_units, Integer::one());
    assert_eq!(
        first.sizes,
        sizes
            .map(|(kind, amount)| (kind, Integer::from(amount)))
            .to_vec()
    );
    assert_eq!(first.admission, ChargeAdmission::Admitted);
    let second = &capture.events()[1];
    assert_eq!(second.point, ChargePoint::GraphEdge);
    assert_eq!(second.work_units, Integer::from(3_u64));
    assert_eq!(second.result_units, Integer::one());
    assert!(second.sizes.is_empty());
    assert_eq!(second.admission, ChargeAdmission::Denied);
}

/// Trace: FR-358-AC-23
#[test]
fn capture_truncation_returns_complete_prefix_and_resets_scope() {
    let mut meter = Meter::new(limits());
    meter.enable_charge_capture(2, 1);
    meter
        .charge(Charge::new(ChargePoint::FunctionCall))
        .unwrap();
    meter
        .charge(Charge::new(ChargePoint::GraphEdge).size(LimitKind::IntegerBits, 4))
        .unwrap();
    meter
        .charge(Charge::new(ChargePoint::TemporalVisit))
        .unwrap();
    let truncated = meter.take_charge_capture().unwrap().unwrap_err();
    assert_eq!(truncated.prefix.events().len(), 2);
    assert_eq!(
        truncated.prefix.events()[0].point,
        ChargePoint::FunctionCall
    );
    assert_eq!(truncated.prefix.events()[1].point, ChargePoint::GraphEdge);
    assert_eq!(meter.admission_count(), 3);
    meter.charge(Charge::new(ChargePoint::FrameWrite)).unwrap();
    let next = meter.disable_charge_capture().unwrap().unwrap();
    assert_eq!(next.events().len(), 1);
    assert_eq!(next.events()[0].point, ChargePoint::FrameWrite);

    meter.enable_charge_capture(2, 0);
    meter
        .charge(Charge::new(ChargePoint::FunctionCall).size(LimitKind::IntegerBits, 0))
        .unwrap();
    let truncated = meter.disable_charge_capture().unwrap().unwrap_err();
    assert!(truncated.prefix.events().is_empty());
}

/// Trace: FR-358-AC-24
#[test]
fn capture_observes_opaque_exact_call_and_plan_boundary() {
    let mut meter = Meter::new(limits());
    meter.enable_charge_capture(5, 5);
    let left = Integer::from(2_u64);
    let right = Integer::from(3_u64);
    let result =
        evaluate_integer_arithmetic(IntegerArithmetic::Add(&left, &right), None, &mut meter);
    assert!(matches!(result, Outcome::Completed(value) if value == Integer::from(5_u64)));
    meter.charge_plan(&Integer::from(2_u64)).unwrap();
    let capture = meter.disable_charge_capture().unwrap().unwrap();
    let events = capture.events();
    assert_eq!(events.len(), 4);
    assert_eq!(
        &events[..3],
        &[
            expected_event(
                ChargePoint::IntegerArithmeticOperands,
                1,
                0,
                &[
                    (LimitKind::IntegerBits, 2),
                    (LimitKind::ValueOccurrences, 2)
                ],
            ),
            expected_event(
                ChargePoint::IntegerArithmeticArithmetic,
                1,
                0,
                &[(LimitKind::IntegerBits, 3)],
            ),
            expected_event(ChargePoint::IntegerArithmeticResultRetain, 1, 1, &[]),
        ]
    );
    assert_eq!(events[3].point, ChargePoint::EqualityPlan);
    assert_eq!(events[3].work_units, Integer::one());
    assert_eq!(events[3].result_units, Integer::from(0_u64));
    assert_eq!(events[3].work_reservation, Some(Integer::from(4_u64)));
    assert_eq!(events[3].result_reservation, Some(Integer::one()));
    assert_eq!(
        events[3].sizes,
        vec![(LimitKind::ValueOccurrences, Integer::from(2_u64))]
    );
    assert!(events
        .iter()
        .all(|event| event.admission == ChargeAdmission::Admitted));
}

/// Trace: FR-358-AC-22
#[test]
fn capture_preserves_exact_denied_amount() {
    let mut meter = Meter::new(limits());
    meter.enable_charge_capture(1, 1);
    let huge = Integer::from(u64::MAX).add(&Integer::one());
    let refusal = meter
        .charge(
            Charge::new(ChargePoint::GraphExpand)
                .exact_size(LimitKind::IntegerBits, huge.clone())
                .work(Integer::from(7_u64)),
        )
        .unwrap_err();
    assert_eq!(refusal.next_charge, huge);
    let capture = meter.disable_charge_capture().unwrap().unwrap();
    assert_eq!(
        capture.events()[0].sizes,
        vec![(LimitKind::IntegerBits, huge)]
    );
    assert_eq!(capture.events()[0].work_units, Integer::from(7_u64));
    assert_eq!(capture.events()[0].admission, ChargeAdmission::Denied);
    assert_eq!(meter.admission_count(), 0);
}

/// Trace: FR-358-AC-24
#[test]
fn capture_observes_direct_internal_units_boundary() {
    let mut meter = Meter::new(limits());
    meter.enable_charge_capture(3, 3);
    let result = evaluate_native_integer_ordering(
        OrderingOperator::Less,
        NativeInteger::new(2),
        NativeInteger::new(3),
        &mut meter,
    );
    assert!(matches!(result, Outcome::Completed(true)));
    let capture = meter.disable_charge_capture().unwrap().unwrap();
    assert_eq!(capture.events().len(), 3);
    assert_eq!(
        capture.events(),
        &[
            expected_event(
                ChargePoint::OrderingOperands,
                1,
                0,
                &[
                    (LimitKind::IntegerBits, 2),
                    (LimitKind::ValueOccurrences, 2)
                ],
            ),
            expected_event(
                ChargePoint::OrderingArithmetic,
                1,
                0,
                &[(LimitKind::IntegerBits, 2)],
            ),
            expected_event(ChargePoint::OrderingResultRetain, 1, 1, &[]),
        ]
    );
}

fn expected_event(
    point: ChargePoint,
    work: u64,
    results: u64,
    sizes: &[(LimitKind, u64)],
) -> quire_exact::ChargeEvent {
    quire_exact::ChargeEvent {
        point,
        work_units: Integer::from(work),
        result_units: Integer::from(results),
        work_reservation: None,
        result_reservation: None,
        sizes: sizes
            .iter()
            .map(|(kind, amount)| (*kind, Integer::from(*amount)))
            .collect(),
        admission: ChargeAdmission::Admitted,
    }
}
