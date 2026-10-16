// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bounded charge-level accounting obligation for the integer arithmetic schedule.

use super::*;

#[derive(Clone, Copy)]
struct Model {
    bits: u64,
    occurrences: u64,
    work: u64,
    results: u64,
    admissions: u64,
}

impl Model {
    fn new() -> Self {
        Self {
            bits: 0,
            occurrences: 0,
            work: 0,
            results: 0,
            admissions: 0,
        }
    }

    // The reference checks named fields in ScalarLimits order. It never calls
    // the meter's validation or update helpers.
    fn step(
        &mut self,
        limits: ScalarLimits,
        point: ChargePoint,
        first: Option<(LimitKind, u64)>,
        second: Option<(LimitKind, u64)>,
        result_units: u64,
    ) -> Result<(), Incomplete> {
        let request = |kind| {
            if let Some((requested, amount)) = first {
                if requested == kind && amount > kind.limit(&limits) {
                    return Some(amount);
                }
            }
            if let Some((requested, amount)) = second {
                if requested == kind && amount > kind.limit(&limits) {
                    return Some(amount);
                }
            }
            None
        };
        let fail = if let Some(amount) = request(LimitKind::IntegerBits) {
            Some((LimitKind::IntegerBits, amount))
        } else {
            request(LimitKind::ValueOccurrences).map(|amount| (LimitKind::ValueOccurrences, amount))
        };
        if let Some((kind, amount)) = fail {
            return Err(Incomplete {
                limit_kind: kind,
                limit: kind.limit(&limits),
                consumed: if kind == LimitKind::IntegerBits {
                    self.bits
                } else {
                    self.occurrences
                },
                next_charge: Integer::from(amount),
                charge_point: point,
            });
        }
        if self.work >= limits.work_units {
            return Err(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: limits.work_units,
                consumed: self.work,
                next_charge: Integer::one(),
                charge_point: point,
            });
        }
        if result_units > limits.result_units.saturating_sub(self.results) {
            return Err(Incomplete {
                limit_kind: LimitKind::ResultUnits,
                limit: limits.result_units,
                consumed: self.results,
                next_charge: Integer::from(result_units),
                charge_point: point,
            });
        }
        if let Some((kind, amount)) = first {
            if kind == LimitKind::IntegerBits {
                self.bits = self.bits.max(amount);
            }
            if kind == LimitKind::ValueOccurrences {
                self.occurrences = self.occurrences.max(amount);
            }
        }
        if let Some((kind, amount)) = second {
            if kind == LimitKind::IntegerBits {
                self.bits = self.bits.max(amount);
            }
            if kind == LimitKind::ValueOccurrences {
                self.occurrences = self.occurrences.max(amount);
            }
        }
        self.work += 1;
        self.results += result_units;
        self.admissions += 1;
        Ok(())
    }

    fn check(self, meter: &Meter) {
        assert_eq!(meter.consumed(LimitKind::IntegerBits), self.bits);
        assert_eq!(meter.consumed(LimitKind::DecimalDigits), 0);
        assert_eq!(meter.consumed(LimitKind::ScaleExpansion), 0);
        assert_eq!(meter.consumed(LimitKind::TextInputBytes), 0);
        assert_eq!(meter.consumed(LimitKind::TextScalars), 0);
        assert_eq!(meter.consumed(LimitKind::NormalizedScalars), 0);
        assert_eq!(meter.consumed(LimitKind::UnitEdges), 0);
        assert_eq!(
            meter.consumed(LimitKind::ValueOccurrences),
            self.occurrences
        );
        assert_eq!(meter.consumed(LimitKind::WorkUnits), self.work);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), self.results);
        assert_eq!(meter.admission_count(), self.admissions);
    }
}

fn step(
    model: &mut Model,
    meter: &mut Meter,
    point: ChargePoint,
    first: Option<(LimitKind, u64)>,
    second: Option<(LimitKind, u64)>,
    results: u64,
) -> bool {
    let expected = model.step(*meter.limits(), point, first, second, results);
    let mut charge = Charge::new(point).results(results);
    if let Some((kind, amount)) = first {
        charge = charge.size(kind, amount);
    }
    if let Some((kind, amount)) = second {
        charge = charge.size(kind, amount);
    }
    let actual = meter.charge(charge);
    assert_eq!(actual, expected);
    model.check(meter);
    #[cfg(feature = "test-support")]
    {
        assert_eq!(meter.admitted_charges().len(), model.admissions as usize);
        if let Some(record) = meter.admitted_charges().last() {
            if actual.is_ok() {
                assert_eq!(
                    record.integer_bits,
                    first
                        .filter(|x| x.0 == LimitKind::IntegerBits)
                        .map(|x| x.1)
                        .into_iter()
                        .chain(
                            second
                                .filter(|x| x.0 == LimitKind::IntegerBits)
                                .map(|x| x.1)
                        )
                        .max()
                );
                assert_eq!(
                    record.value_occurrences,
                    first
                        .filter(|x| x.0 == LimitKind::ValueOccurrences)
                        .map(|x| x.1)
                        .into_iter()
                        .chain(
                            second
                                .filter(|x| x.0 == LimitKind::ValueOccurrences)
                                .map(|x| x.1)
                        )
                        .max()
                );
            }
        }
    }
    actual.is_ok()
}

/// Trace: FR-359-AC-9
#[kani::proof]
#[kani::unwind(3)]
fn meter_integer_arithmetic_schedule_matches_reference() {
    let left: i64 = kani::any();
    let right: i64 = kani::any();
    let bit_limit: u64 = kani::any();
    let occurrence_limit: u64 = kani::any();
    let work_limit: u64 = kani::any();
    let result_limit: u64 = kani::any();
    let limits = ScalarLimits {
        integer_bits: bit_limit,
        value_occurrences: occurrence_limit,
        work_units: work_limit,
        result_units: result_limit,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
    };
    let bits = |value: i64| u64::from((64 - value.unsigned_abs().leading_zeros()).max(1));
    let operand_bits = bits(left).max(bits(right));
    let mut meter = Meter::new(limits);
    let mut model = Model::new();
    if !step(
        &mut model,
        &mut meter,
        ChargePoint::IntegerArithmeticOperands,
        Some((LimitKind::IntegerBits, operand_bits)),
        Some((LimitKind::ValueOccurrences, 2)),
        0,
    ) {
        return;
    }
    if !step(
        &mut model,
        &mut meter,
        ChargePoint::IntegerArithmeticArithmetic,
        Some((LimitKind::IntegerBits, operand_bits + 1)),
        None,
        0,
    ) {
        return;
    }
    step(
        &mut model,
        &mut meter,
        ChargePoint::IntegerArithmeticResultRetain,
        None,
        None,
        1,
    );

    // Independent repeated-kind control on a fresh meter.
    let mut repeated = Meter::new(limits);
    let mut repeated_model = Model::new();
    step(
        &mut repeated_model,
        &mut repeated,
        ChargePoint::FunctionCall,
        Some((LimitKind::IntegerBits, bits(left))),
        Some((LimitKind::IntegerBits, bits(right))),
        0,
    );
}
