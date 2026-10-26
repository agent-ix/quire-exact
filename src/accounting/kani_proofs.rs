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

/// The model's expected refusal. Every amount in the checked schedules is a
/// `u64`, so the amount is compared through [`Integer::to_u64`] rather than
/// by building and comparing a second big integer.
struct Refusal {
    kind: LimitKind,
    limit: u64,
    consumed: u64,
    amount: u64,
    point: ChargePoint,
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
    fn size_short(limit: u64, requests: &Requests, kind: LimitKind) -> Option<u64> {
        let over = |request: Option<Request>| {
            request
                .filter(|(requested, amount)| *requested == kind && *amount > limit)
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
    ) -> Result<(), Refusal> {
        // Each branch names the field and counter directly. The oracle does
        // not reconstruct a field by dispatching a merged symbolic kind.
        macro_rules! size {
            ($kind:ident, $field:ident, $index:literal) => {
                if let Some(amount) = Self::size_short(limits.$field, requests, LimitKind::$kind) {
                    return Err(Refusal {
                        kind: LimitKind::$kind,
                        limit: limits.$field,
                        consumed: self.consumed[$index],
                        amount,
                        point,
                    });
                }
            };
        }
        size!(IntegerBits, integer_bits, 0);
        size!(DecimalDigits, decimal_digits, 1);
        size!(ScaleExpansion, scale_expansion, 2);
        size!(TextInputBytes, text_input_bytes, 3);
        size!(TextScalars, text_scalars, 4);
        size!(NormalizedScalars, normalized_scalars, 5);
        size!(UnitEdges, unit_edges, 6);
        size!(ValueOccurrences, value_occurrences, 7);
        macro_rules! cumulative {
            ($kind:ident, $field:ident, $index:literal, $amount:expr) => {
                if !self.consumed[$index]
                    .checked_add($amount)
                    .is_some_and(|total| total <= limits.$field)
                {
                    return Err(Refusal {
                        kind: LimitKind::$kind,
                        limit: limits.$field,
                        consumed: self.consumed[$index],
                        amount: $amount,
                        point,
                    });
                }
            };
        }
        cumulative!(WorkUnits, work_units, 8, 1);
        cumulative!(ResultUnits, result_units, 9, results);
        for (kind, amount) in requests.iter().flatten() {
            let current = match kind {
                LimitKind::IntegerBits => &mut self.consumed[0],
                LimitKind::DecimalDigits => &mut self.consumed[1],
                LimitKind::ScaleExpansion => &mut self.consumed[2],
                LimitKind::TextInputBytes => &mut self.consumed[3],
                LimitKind::TextScalars => &mut self.consumed[4],
                LimitKind::NormalizedScalars => &mut self.consumed[5],
                LimitKind::UnitEdges => &mut self.consumed[6],
                LimitKind::ValueOccurrences => &mut self.consumed[7],
                LimitKind::WorkUnits | LimitKind::ResultUnits => continue,
            };
            *current = (*current).max(*amount);
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
    match (&actual, &expected) {
        (Ok(()), Ok(())) => {}
        (Err(actual), Err(expected)) => {
            assert_eq!(actual.limit_kind, expected.kind);
            assert_eq!(actual.limit, expected.limit);
            assert_eq!(actual.consumed, expected.consumed);
            assert_eq!(actual.charge_point, expected.point);
            assert_eq!(actual.next_charge.to_u64(), Some(expected.amount));
        }
        _ => panic!("the meter and the model disagree on admission"),
    }
    model.check(meter);
    actual.is_ok()
}

/// The magnitude bit length of `value`, one for zero as specified by FR-362.
fn bits(value: i64) -> u64 {
    u64::from((64 - value.unsigned_abs().leading_zeros()).max(1))
}

// DO NOT RUN: exhausts fixed 16 GiB ceiling, returns Inconclusive MemoryExhausted, proves nothing, blocked work for a week; Peter ordered it never run until all feature work is done.
// FR-359-AC-9 remains unqualified until the healthy proof and both controls pass.
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
