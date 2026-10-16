// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bounded charge-level obligation for [`Meter::charge`] over the integer
//! arithmetic schedule.
//!
//! The reference model below shares no code with the meter: it names each of
//! the ten `ScalarLimits` fields itself, checks them in an unrolled chain and
//! keeps its own counters. It asserts nothing about the diagnostic charge log,
//! which only exists under the `test-support` feature.

use super::*;

/// One semantic-size request: its counter and its amount.
type Request = (LimitKind, u64);

/// The most requests one charge in the checked schedules carries.
type Requests = [Option<Request>; 2];

/// Position of `kind` in `ScalarLimits` member order, written independently of
/// the meter's own indexing.
fn slot(kind: LimitKind) -> usize {
    match kind {
        LimitKind::IntegerBits => 0,
        LimitKind::DecimalDigits => 1,
        LimitKind::ScaleExpansion => 2,
        LimitKind::TextInputBytes => 3,
        LimitKind::TextScalars => 4,
        LimitKind::NormalizedScalars => 5,
        LimitKind::UnitEdges => 6,
        LimitKind::ValueOccurrences => 7,
        LimitKind::WorkUnits => 8,
        LimitKind::ResultUnits => 9,
    }
}

/// The limit of `kind`, read straight from the named field.
fn limit_of(limits: &ScalarLimits, kind: LimitKind) -> u64 {
    match kind {
        LimitKind::IntegerBits => limits.integer_bits,
        LimitKind::DecimalDigits => limits.decimal_digits,
        LimitKind::ScaleExpansion => limits.scale_expansion,
        LimitKind::TextInputBytes => limits.text_input_bytes,
        LimitKind::TextScalars => limits.text_scalars,
        LimitKind::NormalizedScalars => limits.normalized_scalars,
        LimitKind::UnitEdges => limits.unit_edges,
        LimitKind::ValueOccurrences => limits.value_occurrences,
        LimitKind::WorkUnits => limits.work_units,
        LimitKind::ResultUnits => limits.result_units,
    }
}

struct Model {
    consumed: [u64; 10],
    admissions: u64,
}

impl Model {
    fn new() -> Self {
        Self {
            consumed: [0; 10],
            admissions: 0,
        }
    }

    /// The first-inserted request of `kind` that exceeds its limit.
    fn size_short(limits: &ScalarLimits, requests: &Requests, kind: LimitKind) -> Option<u64> {
        let over = |request: Option<Request>| {
            request
                .filter(|(requested, amount)| {
                    *requested == kind && *amount > limit_of(limits, kind)
                })
                .map(|(_, amount)| amount)
        };
        over(requests[0]).or(over(requests[1]))
    }

    /// The expected result of one charge of `requests` plus one work unit and
    /// `results` result units; on success the model's counters advance.
    fn step(
        &mut self,
        limits: &ScalarLimits,
        point: ChargePoint,
        requests: &Requests,
        results: u64,
    ) -> Result<(), Incomplete> {
        let size = |kind| Self::size_short(limits, requests, kind).map(|amount| (kind, amount));
        let cumulative = |kind: LimitKind, amount: u64| {
            let total = u128::from(self.consumed[slot(kind)]) + u128::from(amount);
            (total > u128::from(limit_of(limits, kind))).then_some((kind, amount))
        };
        let short = size(LimitKind::IntegerBits)
            .or_else(|| size(LimitKind::DecimalDigits))
            .or_else(|| size(LimitKind::ScaleExpansion))
            .or_else(|| size(LimitKind::TextInputBytes))
            .or_else(|| size(LimitKind::TextScalars))
            .or_else(|| size(LimitKind::NormalizedScalars))
            .or_else(|| size(LimitKind::UnitEdges))
            .or_else(|| size(LimitKind::ValueOccurrences))
            .or_else(|| cumulative(LimitKind::WorkUnits, 1))
            .or_else(|| cumulative(LimitKind::ResultUnits, results));
        if let Some((kind, amount)) = short {
            return Err(Incomplete {
                limit_kind: kind,
                limit: limit_of(limits, kind),
                consumed: self.consumed[slot(kind)],
                next_charge: Integer::from(amount),
                charge_point: point,
            });
        }
        for (kind, amount) in requests.iter().flatten() {
            let current = self.consumed[slot(*kind)];
            self.consumed[slot(*kind)] = if *amount > current { *amount } else { current };
        }
        self.consumed[8] += 1;
        self.consumed[9] += results;
        self.admissions += 1;
        Ok(())
    }

    /// Every counter, `consumed(kind)` for each kind, and the admission count.
    fn check(&self, meter: &Meter) {
        assert_eq!(meter.consumed(LimitKind::IntegerBits), self.consumed[0]);
        assert_eq!(meter.consumed(LimitKind::DecimalDigits), self.consumed[1]);
        assert_eq!(meter.consumed(LimitKind::ScaleExpansion), self.consumed[2]);
        assert_eq!(meter.consumed(LimitKind::TextInputBytes), self.consumed[3]);
        assert_eq!(meter.consumed(LimitKind::TextScalars), self.consumed[4]);
        assert_eq!(
            meter.consumed(LimitKind::NormalizedScalars),
            self.consumed[5]
        );
        assert_eq!(meter.consumed(LimitKind::UnitEdges), self.consumed[6]);
        assert_eq!(
            meter.consumed(LimitKind::ValueOccurrences),
            self.consumed[7]
        );
        assert_eq!(meter.consumed(LimitKind::WorkUnits), self.consumed[8]);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), self.consumed[9]);
        assert_eq!(meter.admission_count(), self.admissions);
    }
}

/// Charge `requests` and `results` on the meter and on the model, require the
/// same result and the same counters, and report whether it was admitted.
fn step(
    model: &mut Model,
    meter: &mut Meter,
    point: ChargePoint,
    requests: Requests,
    results: u64,
    exact: bool,
) -> bool {
    let expected = model.step(meter.limits(), point, &requests, results);
    let mut charge = Charge::new(point).results(results);
    for (kind, amount) in requests.into_iter().flatten() {
        charge = if exact {
            charge.exact_size(kind, Integer::from(amount))
        } else {
            charge.size(kind, amount)
        };
    }
    let actual = meter.charge(charge);
    assert_eq!(actual, expected);
    model.check(meter);
    actual.is_ok()
}

/// The magnitude bit length of `value`, zero for zero.
fn bits(value: i64) -> u64 {
    u64::from(64 - value.unsigned_abs().leading_zeros())
}

/// Trace: FR-359-AC-9
#[kani::proof]
fn meter_integer_arithmetic_schedule_matches_reference() {
    let left: i64 = kani::any();
    let right: i64 = kani::any();
    let operation: u8 = kani::any();
    let limits = ScalarLimits {
        integer_bits: kani::any(),
        value_occurrences: kani::any(),
        work_units: kani::any(),
        result_units: kani::any(),
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
    };

    // The three-charge schedule: operands (up to two size requests),
    // arithmetic (one) and result-retain (none).
    let operand_bits = bits(left).max(bits(right));
    let (operands, count, arithmetic_bits) = match operation % 3 {
        0 => (operand_bits, 2, operand_bits + 1),
        1 => (operand_bits, 2, bits(left) + bits(right)),
        _ => (bits(left), 1, bits(left)),
    };
    let mut meter = Meter::new(limits);
    let mut model = Model::new();
    let admitted = step(
        &mut model,
        &mut meter,
        ChargePoint::IntegerArithmeticOperands,
        [
            Some((LimitKind::IntegerBits, operands)),
            Some((LimitKind::ValueOccurrences, count)),
        ],
        0,
        false,
    ) && step(
        &mut model,
        &mut meter,
        ChargePoint::IntegerArithmeticArithmetic,
        [Some((LimitKind::IntegerBits, arithmetic_bits)), None],
        0,
        true,
    ) && step(
        &mut model,
        &mut meter,
        ChargePoint::IntegerArithmeticResultRetain,
        [None, None],
        1,
        false,
    );
    assert_eq!(admitted, model.admissions == 3);

    // Repeated-kind and reordered controls on a fresh meter: two requests of
    // one kind with symbolic amounts, then the other kind, then both kinds in
    // reverse field order. A refusal leaves the state for the next charge.
    let mut meter = Meter::new(limits);
    let mut model = Model::new();
    let (a, b): (u64, u64) = (kani::any(), kani::any());
    let (c, d): (u64, u64) = (kani::any(), kani::any());
    let results: u64 = kani::any();
    step(
        &mut model,
        &mut meter,
        ChargePoint::FunctionCall,
        [
            Some((LimitKind::IntegerBits, a)),
            Some((LimitKind::IntegerBits, b)),
        ],
        results,
        false,
    );
    step(
        &mut model,
        &mut meter,
        ChargePoint::FunctionCall,
        [
            Some((LimitKind::ValueOccurrences, c)),
            Some((LimitKind::ValueOccurrences, d)),
        ],
        0,
        false,
    );
    step(
        &mut model,
        &mut meter,
        ChargePoint::FunctionCall,
        [
            Some((LimitKind::ValueOccurrences, d)),
            Some((LimitKind::IntegerBits, a)),
        ],
        0,
        false,
    );
}
