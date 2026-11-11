// SPDX-License-Identifier: AGPL-3.0-or-later
//! Native division, modulus, i64 multiply, equality and charge-schedule entries
//! against the existing `Integer` path, comparing outcomes and every captured
//! charge event.
use std::fmt::Debug;

use ix_trace_rs::trace;
use quire_exact::{
    charge_native_integer_schedule, divide, evaluate_boolean, evaluate_integer_arithmetic,
    evaluate_native_boolean_equality, evaluate_native_integer_arithmetic,
    evaluate_native_integer_division, evaluate_native_integer_equality,
    evaluate_native_integer_modulo, evaluate_native_integer_multiply_i64,
    evaluate_native_integer_ordering, modulo, planned_equality, BooleanConnective, ChargeEvent,
    ChargePoint, CheckedInvariantCause, DivisionMember, DivisionProfile, Integer,
    IntegerArithmetic, IntegerDomain, IntegerInterval, LimitKind, Meter, NativeInteger as N,
    NativeIntegerAdmissionError as Error, NativeIntegerInterval as Range,
    NativeIntegerOperands as Args, NativeIntegerOperation as Op,
    NativeIntegerOperationKind as Kind, NativeIntegerPlan as Plan, NativeIntegerSchedule as Sched,
    OrderingOperator, Outcome, Refusal, ScalarLimits, Undefined, Value,
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
const TIGHTENED: [LimitKind; 3] = [
    LimitKind::IntegerBits,
    LimitKind::WorkUnits,
    LimitKind::ResultUnits,
];

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
fn limited(kind: LimitKind, value: u64) -> ScalarLimits {
    let mut l = limits();
    match kind {
        LimitKind::IntegerBits => l.integer_bits = value,
        LimitKind::WorkUnits => l.work_units = value,
        LimitKind::ResultUnits => l.result_units = value,
        _ => {}
    }
    l
}

/// Outcome, every captured charge event and every counter of one run.
type Run<T> = (Outcome<T>, Vec<ChargeEvent>, Vec<u64>);

fn run<T>(limits: ScalarLimits, f: &dyn Fn(&mut Meter) -> Outcome<T>) -> Run<T> {
    let mut meter = Meter::new(limits);
    meter.enable_charge_capture(64, 256);
    let outcome = f(&mut meter);
    let counters = KINDS.iter().map(|k| meter.consumed(*k)).collect();
    let events = match meter.disable_charge_capture() {
        Some(Ok(capture)) => capture.events().to_vec(),
        _ => panic!("capture missing or truncated"),
    };
    (outcome, events, counters)
}

/// Both closures must agree on outcome, events and counters at unlimited
/// limits and with each of three limits one below its unlimited spend.
fn same<T: PartialEq + Debug>(
    label: &str,
    native: &dyn Fn(&mut Meter) -> Outcome<T>,
    existing: &dyn Fn(&mut Meter) -> Outcome<T>,
) {
    let (outcome, events, counters) = run(limits(), existing);
    assert_eq!(
        run(limits(), native),
        (outcome, events, counters.clone()),
        "{label}"
    );
    for kind in TIGHTENED {
        let spend = counters[KINDS.iter().position(|k| *k == kind).unwrap()];
        for bound in [spend, spend.saturating_sub(1)] {
            let l = limited(kind, bound);
            assert_eq!(run(l, native), run(l, existing), "{label} {kind:?} {bound}");
        }
    }
}

fn abstracted(value: Outcome<N>) -> Outcome<Integer> {
    match value {
        Outcome::Completed(v) => Outcome::Completed(v.to_integer()),
        Outcome::Undefined(v) => Outcome::Undefined(v),
        Outcome::Refused(v) => Outcome::Refused(v),
        Outcome::Incomplete(v) => Outcome::Incomplete(v),
    }
}
fn unit<T>(value: Outcome<T>) -> Outcome<()> {
    match value {
        Outcome::Completed(_) => Outcome::Completed(()),
        Outcome::Undefined(v) => Outcome::Undefined(v),
        Outcome::Refused(v) => Outcome::Refused(v),
        Outcome::Incomplete(v) => Outcome::Incomplete(v),
    }
}
fn interval(a: i128, b: i128) -> (Range, IntegerInterval) {
    (
        Range::new(a, b).unwrap(),
        IntegerInterval::new(Integer::from(a), Integer::from(b)).unwrap(),
    )
}

const EDGES: [i128; 18] = [
    0,
    1,
    -1,
    2,
    -2,
    7,
    -7,
    3,
    -3,
    i64::MAX as i128,
    i64::MIN as i128,
    i32::MIN as i128,
    i128::MAX,
    i128::MIN,
    i128::MIN + 1,
    1 << 100,
    1 << 63,
    1 << 64,
];

#[trace("TC-944", "FR-397-AC-1")]
#[test]
fn division_and_modulus_match_the_integer_path() {
    let domains = [
        None,
        Some(interval(-5, 5)),
        Some(interval(i128::MIN, i128::MAX)),
    ];
    for &a in &EDGES {
        for &b in &EDGES {
            for domain in &domains {
                let general_domain = domain.as_ref().map_or(IntegerDomain::Mathematical, |d| {
                    IntegerDomain::Bounded(d.1.clone())
                });
                let native_domain = domain.as_ref().map(|d| d.0);
                let (ia, ib) = (Integer::from(a), Integer::from(b));
                for profile in DivisionProfile::ALL {
                    for member in [DivisionMember::Quotient, DivisionMember::Remainder] {
                        let label = format!("{profile:?} {member:?} {a} {b} {domain:?}");
                        let native = |m: &mut Meter| {
                            abstracted(evaluate_native_integer_division(
                                profile,
                                member,
                                N::new(a),
                                N::new(b),
                                native_domain,
                                m,
                            ))
                        };
                        let existing =
                            |m: &mut Meter| divide(profile, member, &ia, &ib, &general_domain, m);
                        let leaves_i128 = member == DivisionMember::Quotient
                            && a == i128::MIN
                            && b == -1
                            && domain.is_none();
                        if leaves_i128 {
                            // The quotient 2^127 has no native carrier: capacity
                            // breach after the domain charge, before result-retain.
                            let (outcome, events, _) = run(limits(), &native);
                            let (_, general_events, _) = run(limits(), &existing);
                            assert_eq!(
                                outcome,
                                Outcome::Refused(Refusal::CheckedInvariant {
                                    cause: CheckedInvariantCause::NativeIntegerCapacityBreach
                                }),
                                "{label}"
                            );
                            assert_eq!(events[..], general_events[..events.len()], "{label}");
                            assert_eq!(events.len() + 1, general_events.len(), "{label}");
                            // The prefix is charged identically under every tightened limit.
                            for kind in TIGHTENED {
                                for bound in [0, 1, 2, 130] {
                                    let l = limited(kind, bound);
                                    let (n, ne, nc) = run(l, &native);
                                    let (g, ge, gc) = run(l, &existing);
                                    if matches!(n, Outcome::Incomplete(_)) {
                                        assert_eq!((n, ne, nc), (g, ge, gc), "{label} {kind:?}");
                                    } else {
                                        assert_eq!(ne[..], ge[..ne.len()], "{label} {kind:?}");
                                    }
                                }
                            }
                        } else {
                            same(&label, &native, &existing);
                        }
                    }
                }
                let label = format!("mod {a} {b} {domain:?}");
                same(
                    &label,
                    &|m| {
                        abstracted(evaluate_native_integer_modulo(
                            N::new(a),
                            N::new(b),
                            native_domain,
                            m,
                        ))
                    },
                    &|m| modulo(&ia, &ib, &general_domain, m),
                );
            }
        }
    }
}

#[trace("TC-944", "FR-397-AC-1")]
#[test]
fn division_semantics_zero_and_negative_laws() {
    let mut meter = Meter::new(limits());
    let q = |profile, a, b, meter: &mut Meter| {
        evaluate_native_integer_division(
            profile,
            DivisionMember::Quotient,
            N::new(a),
            N::new(b),
            None,
            meter,
        )
    };
    // -7 / 2: truncating -3, floor -4, Euclidean -4; -7 / -2: 3, 3, 4.
    for (profile, expected) in [
        (DivisionProfile::Truncating, [-3, 3]),
        (DivisionProfile::Floor, [-4, 3]),
        (DivisionProfile::Euclidean, [-4, 4]),
    ] {
        assert_eq!(
            q(profile, -7, 2, &mut meter),
            Outcome::Completed(N::new(expected[0]))
        );
        assert_eq!(
            q(profile, -7, -2, &mut meter),
            Outcome::Completed(N::new(expected[1]))
        );
    }
    assert_eq!(
        evaluate_native_integer_modulo(N::new(-7), N::new(2), None, &mut meter),
        Outcome::Completed(N::new(1))
    );
    assert_eq!(
        evaluate_native_integer_modulo(N::new(-7), N::new(-2), None, &mut meter),
        Outcome::Completed(N::new(1))
    );
    assert_eq!(
        q(DivisionProfile::Floor, 1, 0, &mut meter),
        Outcome::Undefined(Undefined::DivisionByZero)
    );
}

fn multiply_pair(a: i128, b: i128, domain: Option<(Range, IntegerInterval)>) {
    let (ia, ib) = (Integer::from(a), Integer::from(b));
    let native_domain = domain.as_ref().map(|d| d.0);
    let general_domain = domain.as_ref().map(|d| d.1.clone());
    same(
        &format!("mul {a} {b} {domain:?}"),
        &|m| {
            abstracted(
                evaluate_native_integer_multiply_i64(N::new(a), N::new(b), native_domain, m)
                    .unwrap(),
            )
        },
        &|m| {
            evaluate_integer_arithmetic(
                IntegerArithmetic::Multiply(&ia, &ib),
                general_domain.as_ref(),
                m,
            )
        },
    );
}

#[trace("TC-944", "FR-397-AC-2")]
#[test]
fn multiply_i64_accepts_the_boundary_and_matches_the_integer_path() {
    let b = 3_037_000_499_i128;
    for (x, y) in [
        (b, b),
        (-b, b),
        (-b, -b),
        (b, -b),
        (3_037_000_500, 1),
        (i64::MIN as i128, 1),
        (i64::MAX as i128, -1),
        (i64::MAX as i128, 1),
        (0, i64::MIN as i128),
        (1, 0),
        (-1, 1),
        (2, -3),
    ] {
        multiply_pair(x, y, None);
        multiply_pair(x, y, Some(interval(i128::MIN, i128::MAX)));
        // A result interval that excludes the product is the same refusal.
        multiply_pair(x, y, Some(interval(-1, 1)));
    }
}

#[trace("TC-944", "FR-397-AC-2")]
#[test]
fn multiply_i64_rejects_out_of_capacity_before_any_charge() {
    let mut meter = Meter::new(limits());
    let outside = |value: i128| Error::OperandOutsideInterval {
        operand: quire_exact::NativeIntegerOperandPosition::Left,
        value,
        interval: Range::new(i64::MIN as i128, i64::MAX as i128).unwrap(),
    };
    let call = |a: i128, b: i128, m: &mut Meter| {
        evaluate_native_integer_multiply_i64(N::new(a), N::new(b), None, m)
    };
    assert_eq!(
        call(3_037_000_500, 3_037_000_500, &mut meter),
        Err(Error::CapacityExceeded {
            operation: Kind::Multiply
        })
    );
    assert_eq!(
        call(i64::MIN as i128, -1, &mut meter),
        Err(Error::CapacityExceeded {
            operation: Kind::Multiply
        })
    );
    // An operand outside signed64 is refused even when the product is zero.
    assert_eq!(call(i128::MAX, 0, &mut meter), Err(outside(i128::MAX)));
    let beyond = i128::from(i64::MAX) + 1;
    assert_eq!(call(beyond, 0, &mut meter), Err(outside(beyond)));
    assert!(matches!(
        call(0, beyond, &mut meter),
        Err(Error::OperandOutsideInterval {
            operand: quire_exact::NativeIntegerOperandPosition::Right,
            ..
        })
    ));
    assert_eq!(meter.admission_count(), 0);
    for kind in KINDS {
        assert_eq!(meter.consumed(kind), 0);
    }
}

#[trace("TC-944", "FR-397-AC-3")]
#[test]
fn equality_matches_planned_equality_over_integer_values() {
    for &a in &EDGES {
        for &b in &EDGES {
            let (va, vb) = (
                Value::Integer(Integer::from(a)),
                Value::Integer(Integer::from(b)),
            );
            same(
                &format!("eq {a} {b}"),
                &|m| evaluate_native_integer_equality(N::new(a), N::new(b), m),
                &|m| planned_equality(&va, &vb, m),
            );
            assert_eq!(
                evaluate_native_integer_equality(N::new(a), N::new(b), &mut Meter::new(limits())),
                Outcome::Completed(a == b)
            );
        }
    }
}

#[trace("TC-944", "FR-397-AC-5")]
#[test]
fn boolean_equality_matches_planned_equality_over_boolean_values() {
    for l in [false, true] {
        for r in [false, true] {
            let (vl, vr) = (Value::Boolean(l), Value::Boolean(r));
            let native = |m: &mut Meter| evaluate_native_boolean_equality(l, r, m);
            let existing = |m: &mut Meter| planned_equality(&vl, &vr, m);
            let (outcome, events, counters) = run(limits(), &existing);
            assert_eq!(outcome, Outcome::Completed(l == r));
            assert_eq!(events.len(), 4);
            assert_eq!(events[0].point, ChargePoint::EqualityPlanForm);
            assert_eq!(events[3].point, ChargePoint::EqualityResultRetain);
            assert_eq!(run(limits(), &native), (outcome, events, counters.clone()));
            // Every bound from zero to the full spend of each counted limit
            // lands on every charge boundary of the four-charge schedule.
            for (kind, spend) in KINDS.iter().zip(counters) {
                for bound in 0..=spend {
                    let mut lim = limits();
                    match kind {
                        LimitKind::IntegerBits => lim.integer_bits = bound,
                        LimitKind::ValueOccurrences => lim.value_occurrences = bound,
                        LimitKind::WorkUnits => lim.work_units = bound,
                        LimitKind::ResultUnits => lim.result_units = bound,
                        _ => continue,
                    }
                    assert_eq!(
                        run(lim, &native),
                        run(lim, &existing),
                        "boolean eq {l} {r} {kind:?} {bound}"
                    );
                }
            }
        }
    }
}

#[trace("TC-944", "FR-397-AC-3")]
#[test]
fn boolean_connectives_truth_tables_charge_only_the_result_retain() {
    use BooleanConnective::{And, Implies, Not, Or};
    for x in [false, true] {
        for y in [false, true] {
            for (connective, expected) in [
                (And(x, y), x && y),
                (Or(x, y), x || y),
                (Implies(x, y), !x || y),
            ] {
                let (outcome, events, _) = run(limits(), &|m| evaluate_boolean(connective, m));
                assert_eq!(outcome, Outcome::Completed(expected));
                assert_eq!(events.len(), 1);
                assert_eq!(events[0].point, ChargePoint::BooleanResultRetain);
                assert_eq!(events[0].result_units, Integer::one());
            }
        }
        let (outcome, events, _) = run(limits(), &|m| evaluate_boolean(Not(x), m));
        assert_eq!(outcome, Outcome::Completed(!x));
        assert_eq!(events.len(), 1);
    }
}

#[trace("TC-944", "FR-397-AC-4")]
#[test]
fn charge_only_schedule_equals_each_evaluator() {
    let small = Range::new(-1000, 1000).unwrap();
    for &(a, b) in &[
        (0_i128, 0_i128),
        (5, -3),
        (-1000, 1000),
        (1 << 40, 3),
        (i128::MAX, 0),
    ] {
        for kind in [Kind::Add, Kind::Subtract, Kind::Negate] {
            let r = Range::new(a.min(b), a.max(b)).unwrap();
            let op = match kind {
                Kind::Add => Op::Add { left: r, right: r },
                Kind::Subtract => Op::Subtract { left: r, right: r },
                _ => Op::Negate { operand: r },
            };
            // Only the Add of [0, i128::MAX] with itself overflows the capacity.
            let planned = Plan::new(op, None);
            let overflows = kind == Kind::Add && a == i128::MAX;
            assert_eq!(planned.is_err(), overflows, "plan {kind:?} {a} {b}");
            let Ok(plan) = planned else {
                assert_eq!(
                    planned,
                    Err(Error::CapacityExceeded {
                        operation: Kind::Add
                    })
                );
                continue;
            };
            let args = if kind == Kind::Negate {
                Args::Unary { operand: N::new(a) }
            } else {
                Args::Binary {
                    left: N::new(a),
                    right: N::new(a),
                }
            };
            let admitted = plan.admit(args).unwrap();
            same(
                &format!("arith {kind:?} {a}"),
                &|m| unit(evaluate_native_integer_arithmetic(admitted, m)),
                &|m| charge_native_integer_schedule(Sched::Arithmetic(admitted), m),
            );
        }
    }
    let plan = Plan::new(
        Op::Multiply {
            left: small,
            right: small,
        },
        None,
    )
    .unwrap();
    let admitted = plan
        .admit(Args::Binary {
            left: N::new(-999),
            right: N::new(7),
        })
        .unwrap();
    same(
        "mul32",
        &|m| unit(evaluate_native_integer_arithmetic(admitted, m)),
        &|m| charge_native_integer_schedule(Sched::Arithmetic(admitted), m),
    );
    for &(a, b) in &[
        (3_037_000_499_i128, -3_037_000_499_i128),
        (0, 1),
        (1, i64::MIN as i128),
    ] {
        same(
            &format!("mul64 {a} {b}"),
            &|m| unit(evaluate_native_integer_multiply_i64(N::new(a), N::new(b), None, m).unwrap()),
            &|m| {
                charge_native_integer_schedule(
                    Sched::MultiplyI64 {
                        left: N::new(a),
                        right: N::new(b),
                    },
                    m,
                )
            },
        );
    }
    for &a in &EDGES {
        for &b in &[0_i128, 1, -1, 3, -17, i128::MAX, i128::MIN] {
            let (x, y) = (N::new(a), N::new(b));
            // A schedule never forms the i128-escaping quotient, so compare on
            // the remainder, which every pair admits.
            same(
                &format!("div {a} {b}"),
                &|m| {
                    unit(evaluate_native_integer_division(
                        DivisionProfile::Floor,
                        DivisionMember::Remainder,
                        x,
                        y,
                        None,
                        m,
                    ))
                },
                &|m| {
                    charge_native_integer_schedule(
                        Sched::Division {
                            dividend: x,
                            divisor: y,
                        },
                        m,
                    )
                },
            );
            same(
                &format!("mod {a} {b}"),
                &|m| unit(evaluate_native_integer_modulo(x, y, None, m)),
                &|m| {
                    charge_native_integer_schedule(
                        Sched::Modulus {
                            dividend: x,
                            divisor: y,
                        },
                        m,
                    )
                },
            );
            same(
                &format!("ord {a} {b}"),
                &|m| {
                    unit(evaluate_native_integer_ordering(
                        OrderingOperator::Less,
                        x,
                        y,
                        m,
                    ))
                },
                &|m| charge_native_integer_schedule(Sched::Ordering { left: x, right: y }, m),
            );
            same(
                &format!("eq {a} {b}"),
                &|m| unit(evaluate_native_integer_equality(x, y, m)),
                &|m| charge_native_integer_schedule(Sched::Equality, m),
            );
        }
    }
}
