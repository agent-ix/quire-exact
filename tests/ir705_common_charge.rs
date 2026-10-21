// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public common/general charge refinement controls.

use ix_trace_rs::trace;
use quire_exact::{Charge, ChargePoint, Incomplete, Integer, LimitKind, Meter, ScalarLimits};

fn limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 7,
        decimal_digits: 4,
        scale_expansion: 4,
        text_input_bytes: 4,
        text_scalars: 4,
        normalized_scalars: 4,
        unit_edges: 4,
        value_occurrences: 4,
        work_units: 4,
        result_units: 4,
    }
}

#[trace("FR-359-AC-6", "FR-359-AC-8", "FR-358-AC-14")]
#[test]
fn public_charge_transitions_preserve_literal_denials_and_admitted_prefixes() {
    let point = ChargePoint::FunctionCall;
    // The expected record is independent of both storage and the admission
    // implementation: first failing field, first inserted failure within it.
    for (requests, expected) in [
        (vec![], None),
        (vec![(LimitKind::IntegerBits, 7_i128)], None),
        (
            vec![(LimitKind::IntegerBits, 7), (LimitKind::IntegerBits, 0)],
            None,
        ),
        (
            vec![
                (LimitKind::DecimalDigits, 2),
                (LimitKind::IntegerBits, 7),
                (LimitKind::IntegerBits, 0),
            ],
            None,
        ),
        (
            vec![(LimitKind::IntegerBits, 8), (LimitKind::IntegerBits, 9)],
            Some((LimitKind::IntegerBits, 8_i128, 7, 3)),
        ),
        (
            vec![
                (LimitKind::DecimalDigits, 5),
                (LimitKind::IntegerBits, 9),
                (LimitKind::IntegerBits, 8),
                (LimitKind::ScaleExpansion, 6),
            ],
            Some((LimitKind::IntegerBits, 9, 7, 3)),
        ),
        (
            vec![
                (LimitKind::ValueOccurrences, -1),
                (LimitKind::IntegerBits, -2),
            ],
            Some((LimitKind::IntegerBits, -2, 7, 3)),
        ),
        (
            vec![(LimitKind::IntegerBits, 18_446_744_073_709_551_616_i128)],
            Some((LimitKind::IntegerBits, 18_446_744_073_709_551_616, 7, 3)),
        ),
    ] {
        for overwritten in [false, true] {
            let mut charge = Charge::new(point);
            for &(kind, amount) in &requests {
                charge = charge.exact_size(kind, Integer::from(amount));
            }
            if overwritten {
                // An actual exceptional builder value promotes storage, then
                // its overwrite restores the original mathematical charge.
                charge = charge.work(Integer::from(-1_i64)).work(Integer::one());
            }
            let mut meter = Meter::new(limits());
            meter
                .charge(
                    Charge::new(ChargePoint::IntegerArithmeticOperands)
                        .size(LimitKind::IntegerBits, 3)
                        .size(LimitKind::ValueOccurrences, 2)
                        .results(1),
                )
                .unwrap();
            let before = meter.clone();
            let result = meter.charge(charge);
            match expected {
                Some((kind, amount, limit, consumed)) => {
                    assert_eq!(
                        result,
                        Err(Incomplete {
                            limit_kind: kind,
                            limit,
                            consumed,
                            next_charge: Integer::from(amount),
                            charge_point: point,
                        })
                    );
                    assert_eq!(meter, before);
                }
                None => {
                    assert_eq!(result, Ok(()));
                    assert_eq!(meter.consumed(LimitKind::WorkUnits), 2);
                    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
                    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
                    let expected_bits = if requests.is_empty() { 3 } else { 7 };
                    assert_eq!(meter.consumed(LimitKind::IntegerBits), expected_bits);
                    assert_eq!(
                        meter.consumed(LimitKind::DecimalDigits),
                        if requests.len() == 3 { 2 } else { 0 }
                    );
                    assert_eq!(meter.admission_count(), 2);
                    #[cfg(feature = "test-support")]
                    {
                        let records = meter.admitted_charges();
                        assert_eq!(records[0].point, ChargePoint::IntegerArithmeticOperands);
                        assert_eq!(records[1].point, point);
                        assert_eq!(
                            records[1].integer_bits,
                            if requests.is_empty() { None } else { Some(7) }
                        );
                        assert_eq!(records[1].value_occurrences, None);
                    }
                }
            }
        }
    }
}
