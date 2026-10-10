// SPDX-License-Identifier: AGPL-3.0-or-later
//! Actual admission sizes and independent scalar boundary controls.
#![cfg(feature = "test-support")]

use core::num::NonZeroU64;

use quire_exact::{
    evaluate_integer_arithmetic, evaluate_rational_arithmetic, order_numbers, AdmittedCharge,
    Cancel, CancelCause, Charge, ChargePoint, Incomplete, InjectedDenial, Integer,
    IntegerArithmetic, LimitKind, Meter, OrderedOperands, OrderingOperator, Outcome, Rational,
    RationalArithmetic, ScalarLimits,
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

fn projection(record: &AdmittedCharge) -> [Option<u64>; 8] {
    [
        record.integer_bits,
        record.decimal_digits,
        record.scale_expansion,
        record.text_input_bytes,
        record.text_scalars,
        record.normalized_scalars,
        record.unit_edges,
        record.value_occurrences,
    ]
}

fn assert_admitted_state(before: &Meter, after: &Meter) {
    assert_eq!(after.admitted_charges(), before.admitted_charges());
    assert_eq!(after.admission_count(), before.admission_count());
    assert_eq!(after.charge_log_truncated(), before.charge_log_truncated());
    for kind in LimitKind::ALL {
        assert_eq!(after.consumed(kind), before.consumed(kind), "{kind:?}");
    }
}

/// Trace: FR-358-AC-14
#[test]
fn records_actual_sizes_coalesces_repeats_and_distinguishes_absent_zero() {
    let point = ChargePoint::FunctionCall;
    let mut meter = Meter::new(limits());
    for amount in [9, 3] {
        meter
            .charge(Charge::new(point).size(LimitKind::IntegerBits, amount))
            .unwrap();
    }
    meter.charge(Charge::new(point)).unwrap();
    meter
        .charge(
            Charge::new(point)
                .size(LimitKind::IntegerBits, 9)
                .size(LimitKind::IntegerBits, 3)
                .size(LimitKind::IntegerBits, 0),
        )
        .unwrap();
    for count in [1, 2, 17, 4097] {
        let mut charge = Charge::new(point);
        for _ in 0..count {
            charge = charge.size(LimitKind::IntegerBits, 0);
        }
        meter.charge(charge).unwrap();
    }
    let records = meter.admitted_charges();
    assert_eq!(
        records.iter().map(|r| r.integer_bits).collect::<Vec<_>>(),
        [
            Some(9),
            Some(3),
            None,
            Some(9),
            Some(0),
            Some(0),
            Some(0),
            Some(0)
        ]
    );
    assert_eq!(meter.consumed(LimitKind::IntegerBits), 9);
    assert!(!core::mem::needs_drop::<AdmittedCharge>());
    for record in records {
        assert_eq!(
            core::mem::size_of_val(record),
            core::mem::size_of::<AdmittedCharge>()
        );
        assert_eq!(record.point, point);
        assert_eq!(&projection(record)[1..], &[None; 7]);
    }

    // Every semantic field is observed independently; cumulative kinds accepted
    // by exact_size remain excluded from this diagnostic projection.
    let mut charge = Charge::new(point);
    let expected = [
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9),
    ];
    for (index, kind) in LimitKind::ALL.into_iter().enumerate() {
        charge = charge
            .size(kind, u64::try_from(index).unwrap() + 2)
            .size(kind, 0);
    }
    meter.charge(charge).unwrap();
    assert_eq!(
        projection(meter.admitted_charges().last().unwrap()),
        expected
    );
    meter.charge_plan(&Integer::from(2_u64)).unwrap();
    let plan = meter.admitted_charges().last().unwrap();
    assert_eq!(plan.point, ChargePoint::EqualityPlan);
    assert_eq!(
        projection(plan),
        [None, None, None, None, None, None, None, Some(2)]
    );
}

/// Trace: FR-358-AC-14
#[test]
fn ordinary_injected_and_cancelled_charges_preserve_admission_records() {
    let point = ChargePoint::FunctionCall;
    for denial in 0..4 {
        let cancel = Cancel::new();
        let mut meter = Meter::new(ScalarLimits {
            integer_bits: 9,
            work_units: 2,
            ..limits()
        })
        .with_cancel(cancel.clone());
        meter
            .charge(Charge::new(point).size(LimitKind::IntegerBits, 3))
            .unwrap();
        if denial == 2 {
            meter = meter.with_injected_denial(InjectedDenial {
                point,
                occurrence: NonZeroU64::new(1).unwrap(),
            });
        }
        if denial == 3 {
            cancel.cancel(CancelCause::Requested);
        }
        let before = meter.clone();
        let charge = match denial {
            0 => Charge::new(point).size(LimitKind::IntegerBits, 10),
            1 => Charge::new(point)
                .size(LimitKind::IntegerBits, 8)
                .work(Integer::from(2_u64)),
            2 | 3 => Charge::new(point).size(LimitKind::IntegerBits, 8),
            _ => unreachable!(),
        };
        let incomplete = meter.charge(charge).unwrap_err();
        assert_eq!(incomplete.charge_point, point);
        assert_eq!(
            incomplete.limit_kind,
            if denial == 0 {
                LimitKind::IntegerBits
            } else {
                LimitKind::WorkUnits
            }
        );
        assert_admitted_state(&before, &meter);
        if denial == 2 {
            meter
                .charge(Charge::new(point).size(LimitKind::IntegerBits, 8))
                .unwrap();
            assert_eq!(
                meter.admitted_charges().last().unwrap().integer_bits,
                Some(8)
            );
            assert_eq!(meter.admission_count(), 2);
        }
    }
    // Plan availability and cancellation use the same atomic record custody.
    for cancelled in [false, true] {
        let cancel = Cancel::new();
        let mut meter = Meter::new(ScalarLimits {
            work_units: 3,
            ..limits()
        })
        .with_cancel(cancel.clone());
        meter
            .charge(Charge::new(point).size(LimitKind::IntegerBits, 3))
            .unwrap();
        if cancelled {
            cancel.cancel(CancelCause::Requested);
        }
        let before = meter.clone();
        assert_eq!(
            meter
                .charge_plan(&Integer::from(2_u64))
                .unwrap_err()
                .limit_kind,
            LimitKind::WorkUnits
        );
        assert_admitted_state(&before, &meter);
    }
}

/// Trace: FR-358-AC-15
#[test]
fn fixed_records_share_the_existing_prefix_and_truncation_boundary() {
    let mut meter = Meter::new(limits());
    let mut expected = Vec::new();
    for admission in 1_u64..=4097 {
        let point = if admission % 2 == 0 {
            ChargePoint::CollectionVisit
        } else {
            ChargePoint::FunctionCall
        };
        let amount = match admission {
            4096 => 8,
            4097 => 9,
            _ => admission % 8,
        };
        meter
            .charge(
                Charge::new(point)
                    .size(LimitKind::IntegerBits, amount)
                    .size(LimitKind::IntegerBits, 0)
                    .size(LimitKind::TextScalars, admission),
            )
            .unwrap();
        if admission <= 4096 {
            expected.push((point, amount, admission));
        }
        if admission == 4096 {
            assert!(!meter.charge_log_truncated());
            assert_eq!(meter.admitted_charges().len(), 4096);
            assert_eq!(
                meter.admitted_charges().last().unwrap().integer_bits,
                Some(8)
            );
        }
    }
    assert!(meter.charge_log_truncated());
    assert_eq!(meter.admitted_charges().len(), 4096);
    for (record, (point, bits, scalars)) in meter.admitted_charges().iter().zip(expected) {
        assert_eq!(record.point, point);
        assert_eq!(
            projection(record),
            [
                Some(bits),
                None,
                None,
                None,
                Some(scalars),
                None,
                None,
                None
            ]
        );
    }
    assert_eq!(meter.admission_count(), 4097);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 4097);
    assert_eq!(meter.consumed(LimitKind::IntegerBits), 9);
    assert_eq!(meter.consumed(LimitKind::TextScalars), 4097);
}

fn bits(value: i64) -> u64 {
    u64::from(64 - value.unsigned_abs().leading_zeros()).max(1)
}

fn rational(numerator: i64, denominator: i64) -> Rational {
    Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
}

fn points(meter: &Meter) -> Vec<ChargePoint> {
    meter.admitted_charges().iter().map(|r| r.point).collect()
}

fn assert_bit_stop<T>(
    outcome: Outcome<T>,
    meter: &Meter,
    limit: u64,
    operands: u64,
    request: u64,
    charge_points: (ChargePoint, ChargePoint),
    occurrences: u64,
) {
    let Outcome::Incomplete(incomplete) = outcome else {
        panic!("expected bit refusal");
    };
    let (operand_point, arithmetic_point) = charge_points;
    let operands_admitted = operands <= limit;
    let point = if operands_admitted {
        arithmetic_point
    } else {
        operand_point
    };
    let prior = if operands_admitted { operands } else { 0 };
    assert_eq!(
        incomplete,
        Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit,
            consumed: prior,
            next_charge: Integer::from(if operands_admitted { request } else { operands }),
            charge_point: point
        }
    );
    assert_eq!(
        points(meter),
        if operands_admitted {
            vec![operand_point]
        } else {
            vec![]
        }
    );
    assert_eq!(meter.admission_count(), u64::from(operands_admitted));
    assert_eq!(
        meter.consumed(LimitKind::WorkUnits),
        u64::from(operands_admitted)
    );
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    for kind in LimitKind::ALL {
        let expected = match kind {
            LimitKind::IntegerBits => prior,
            LimitKind::ValueOccurrences if operands_admitted => occurrences,
            LimitKind::WorkUnits => u64::from(operands_admitted),
            _ => 0,
        };
        assert_eq!(meter.consumed(kind), expected, "{kind:?}");
    }
    if operands_admitted {
        assert_eq!(
            projection(&meter.admitted_charges()[0]),
            [
                Some(operands),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(occurrences)
            ]
        );
    }
}

/// Trace: FR-362-AC-10, FR-362-AC-20
#[test]
fn rational_normalize_records_unreduced_parts_and_exact_boundary_stops() {
    #[derive(Clone, Copy)]
    enum Op {
        Add,
        Subtract,
        Multiply,
        Divide,
        Negate,
    }
    use ChargePoint::{
        RationalArithmeticArithmetic as Arithmetic, RationalArithmeticNormalize as Normalize,
        RationalArithmeticOperands as Operands, RationalArithmeticResultRetain as Retain,
    };
    // The table's literals are independent controls for formula mistakes and reduction.
    let cases = [
        (Op::Add, (1, 2), (2, 3), (7, 6), (7, 6), 5, 3),
        (Op::Subtract, (5, 7), (4, 7), (7, 49), (1, 7), 7, 6),
        (Op::Multiply, (2, 3), (3, 2), (6, 6), (1, 1), 4, 3),
        (Op::Divide, (3, 4), (5, 7), (21, 20), (21, 20), 6, 5),
        (Op::Negate, (-5, 8), (0, 1), (5, 8), (5, 8), 4, 4),
        (Op::Subtract, (1, 2), (1, 2), (0, 4), (0, 1), 4, 3),
    ];
    for (op, (a, p), (c, q), unreduced, canonical, arithmetic, normalize) in cases {
        let unary = matches!(op, Op::Negate);
        let operands = if unary {
            bits(a).max(bits(p))
        } else {
            bits(a).max(bits(p)).max(bits(c)).max(bits(q))
        };
        let occurrences = if unary { 1 } else { 2 };
        let (n, d, bound) = match op {
            Op::Add => (
                a * q + c * p,
                p * q,
                (bits(a) + bits(q))
                    .max(bits(c) + bits(p))
                    .saturating_add(1)
                    .max(bits(p) + bits(q)),
            ),
            Op::Subtract => (
                a * q - c * p,
                p * q,
                (bits(a) + bits(q))
                    .max(bits(c) + bits(p))
                    .saturating_add(1)
                    .max(bits(p) + bits(q)),
            ),
            Op::Multiply => (a * c, p * q, (bits(a) + bits(c)).max(bits(p) + bits(q))),
            Op::Divide => (a * q, p * c, (bits(a) + bits(q)).max(bits(p) + bits(c))),
            Op::Negate => (-a, p, bits(a).max(bits(p))),
        };
        assert_eq!((n, d), unreduced);
        assert_eq!(bound, arithmetic);
        assert_eq!(bits(n).max(bits(d)), normalize);
        let (left, right) = (rational(a, p), rational(c, q));
        let operation = match op {
            Op::Add => RationalArithmetic::Add(&left, &right),
            Op::Subtract => RationalArithmetic::Subtract(&left, &right),
            Op::Multiply => RationalArithmetic::Multiply(&left, &right),
            Op::Divide => RationalArithmetic::Divide(&left, &right),
            Op::Negate => RationalArithmetic::Negate(&left),
        };
        let mut exact = Meter::new(ScalarLimits {
            integer_bits: bound,
            ..limits()
        });
        assert_eq!(
            evaluate_rational_arithmetic(operation, None, &mut exact).completed(),
            Some(rational(canonical.0, canonical.1))
        );
        assert_eq!(points(&exact), [Operands, Arithmetic, Normalize, Retain]);
        let records = exact.admitted_charges();
        assert_eq!(
            projection(&records[0]),
            [
                Some(operands),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(occurrences)
            ]
        );
        assert_eq!(
            projection(&records[1]),
            [Some(arithmetic), None, None, None, None, None, None, None]
        );
        assert_eq!(
            projection(&records[2]),
            [Some(normalize), None, None, None, None, None, None, None]
        );
        assert_eq!(projection(&records[3]), [None; 8]);
        assert_eq!(exact.consumed(LimitKind::IntegerBits), bound);
        assert_eq!(exact.consumed(LimitKind::WorkUnits), 4);
        assert_eq!(exact.consumed(LimitKind::ResultUnits), 1);
        let mut low = Meter::new(ScalarLimits {
            integer_bits: bound - 1,
            ..limits()
        });
        assert_bit_stop(
            evaluate_rational_arithmetic(operation, None, &mut low),
            &low,
            bound - 1,
            operands,
            bound,
            (Operands, Arithmetic),
            occurrences,
        );
        let mut operand_low = Meter::new(ScalarLimits {
            integer_bits: operands - 1,
            ..limits()
        });
        assert_bit_stop(
            evaluate_rational_arithmetic(operation, None, &mut operand_low),
            &operand_low,
            operands - 1,
            operands,
            bound,
            (Operands, Arithmetic),
            occurrences,
        );
    }
}

/// Trace: FR-362-AC-10
#[test]
fn integer_arithmetic_exact_bits_and_one_under_use_operand_formulas() {
    #[derive(Clone, Copy)]
    enum Op {
        Add,
        Subtract,
        Multiply,
        Negate,
    }
    use ChargePoint::{
        IntegerArithmeticArithmetic as Arithmetic, IntegerArithmeticOperands as Operands,
        IntegerArithmeticResultRetain as Retain,
    };
    for (op, a, b, expected) in [
        (Op::Add, 5, 7, 12),
        (Op::Subtract, 1000, 999, 1),
        (Op::Multiply, 255, 255, 65025),
        (Op::Negate, -128, 0, 128),
    ] {
        let unary = matches!(op, Op::Negate);
        let operands = if unary { bits(a) } else { bits(a).max(bits(b)) };
        let occurrences = if unary { 1 } else { 2 };
        let bound = match op {
            Op::Add | Op::Subtract => operands + 1,
            Op::Multiply => bits(a) + bits(b),
            Op::Negate => bits(a),
        };
        let oracle = match op {
            Op::Add => a.checked_add(b),
            Op::Subtract => a.checked_sub(b),
            Op::Multiply => a.checked_mul(b),
            Op::Negate => a.checked_neg(),
        };
        assert_eq!(oracle, Some(expected));
        let (left, right) = (Integer::from(a), Integer::from(b));
        let operation = match op {
            Op::Add => IntegerArithmetic::Add(&left, &right),
            Op::Subtract => IntegerArithmetic::Subtract(&left, &right),
            Op::Multiply => IntegerArithmetic::Multiply(&left, &right),
            Op::Negate => IntegerArithmetic::Negate(&left),
        };
        let mut exact = Meter::new(ScalarLimits {
            integer_bits: bound,
            ..limits()
        });
        assert_eq!(
            evaluate_integer_arithmetic(operation, None, &mut exact).completed(),
            Some(Integer::from(expected))
        );
        assert_eq!(points(&exact), [Operands, Arithmetic, Retain]);
        assert_eq!(
            projection(&exact.admitted_charges()[0]),
            [
                Some(operands),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(occurrences)
            ]
        );
        assert_eq!(
            projection(&exact.admitted_charges()[1]),
            [Some(bound), None, None, None, None, None, None, None]
        );
        assert_eq!(projection(&exact.admitted_charges()[2]), [None; 8]);
        assert_eq!(exact.consumed(LimitKind::IntegerBits), bound);
        assert_eq!(exact.consumed(LimitKind::WorkUnits), 3);
        assert_eq!(exact.consumed(LimitKind::ResultUnits), 1);
        let mut low = Meter::new(ScalarLimits {
            integer_bits: bound - 1,
            ..limits()
        });
        assert_bit_stop(
            evaluate_integer_arithmetic(operation, None, &mut low),
            &low,
            bound - 1,
            operands,
            bound,
            (Operands, Arithmetic),
            occurrences,
        );
        let mut operand_low = Meter::new(ScalarLimits {
            integer_bits: operands - 1,
            ..limits()
        });
        assert_bit_stop(
            evaluate_integer_arithmetic(operation, None, &mut operand_low),
            &operand_low,
            operands - 1,
            operands,
            bound,
            (Operands, Arithmetic),
            occurrences,
        );
    }
}

/// Trace: FR-362-AC-10
#[test]
fn integer_and_rational_ordering_exact_bits_and_one_under_stop_at_first_short_point() {
    use ChargePoint::{
        OrderingArithmetic as Arithmetic, OrderingOperands as Operands,
        OrderingResultRetain as Retain,
    };
    let (a, b) = (Integer::from(7_i64), Integer::from(8_i64));
    let (left, right) = (rational(7, 3), rational(5, 8));
    let rational_bound = (bits(7) + bits(8)).max(bits(5) + bits(3));
    assert_eq!(rational_bound, 7);
    for (operands, operand_bits, bound, expected) in [
        (
            OrderedOperands::Integers(&a, &b),
            bits(7).max(bits(8)),
            bits(7).max(bits(8)),
            true,
        ),
        (
            OrderedOperands::Rationals(&left, &right),
            bits(7).max(bits(3)).max(bits(5)).max(bits(8)),
            rational_bound,
            false,
        ),
    ] {
        let mut exact = Meter::new(ScalarLimits {
            integer_bits: bound,
            ..limits()
        });
        assert_eq!(
            order_numbers(OrderingOperator::Less, operands, &mut exact).completed(),
            Some(expected)
        );
        assert_eq!(points(&exact), [Operands, Arithmetic, Retain]);
        assert_eq!(
            projection(&exact.admitted_charges()[0]),
            [
                Some(operand_bits),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(2)
            ]
        );
        assert_eq!(
            projection(&exact.admitted_charges()[1]),
            [Some(bound), None, None, None, None, None, None, None]
        );
        assert_eq!(projection(&exact.admitted_charges()[2]), [None; 8]);
        assert_eq!(exact.consumed(LimitKind::IntegerBits), bound);
        assert_eq!(exact.consumed(LimitKind::WorkUnits), 3);
        assert_eq!(exact.consumed(LimitKind::ResultUnits), 1);
        let mut low = Meter::new(ScalarLimits {
            integer_bits: bound - 1,
            ..limits()
        });
        assert_bit_stop(
            order_numbers(OrderingOperator::Less, operands, &mut low),
            &low,
            bound - 1,
            operand_bits,
            bound,
            (Operands, Arithmetic),
            2,
        );
    }
}
