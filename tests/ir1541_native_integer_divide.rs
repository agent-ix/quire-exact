// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native integer `Divide` and `Modulo` plan against `divide` and `modulo`
//! over `Integer`, the plan-free evaluators and independent literals
//! (FR-409, TC-957).
use ix_trace_rs::trace;
use quire_exact::{
    charge_native_integer_schedule, divide, evaluate_native_integer_arithmetic,
    evaluate_native_integer_division, evaluate_native_integer_modulo, modulo, ChargeEvent,
    ChargePoint, DivisionMember, DivisionProfile, Integer, IntegerDomain, IntegerInterval,
    LimitKind, Meter, NativeInteger as N, NativeIntegerAdmissionError as Error,
    NativeIntegerInterval as Range, NativeIntegerOperandPosition as Position,
    NativeIntegerOperands as Args, NativeIntegerOperation as Op,
    NativeIntegerOperationKind as Kind, NativeIntegerPlan as Plan, NativeIntegerSchedule as Sched,
    Outcome, Refusal, ScalarLimits, Undefined,
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
const TIGHTENED: [LimitKind; 4] = [
    LimitKind::IntegerBits,
    LimitKind::ValueOccurrences,
    LimitKind::WorkUnits,
    LimitKind::ResultUnits,
];

/// The grid of TC-957, as independent literals.
const GRID: [i128; 13] = [
    0,
    1,
    -1,
    2,
    -2,
    7,
    -7,
    i64::MIN as i128,
    i64::MIN as i128 + 1,
    i64::MAX as i128,
    i128::MIN,
    i128::MIN + 1,
    i128::MAX,
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
        LimitKind::ValueOccurrences => l.value_occurrences = value,
        LimitKind::WorkUnits => l.work_units = value,
        LimitKind::ResultUnits => l.result_units = value,
        _ => {}
    }
    l
}

type Run = (Outcome<Integer>, Vec<ChargeEvent>, Vec<u64>);

fn run(limits: ScalarLimits, f: &dyn Fn(&mut Meter) -> Outcome<Integer>) -> Run {
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

/// Both closures agree on outcome, every event and every counter at a
/// sufficient limit and at one below the spend of each limit kind.
fn same(
    label: &str,
    actual: &dyn Fn(&mut Meter) -> Outcome<Integer>,
    expected: &dyn Fn(&mut Meter) -> Outcome<Integer>,
) {
    let (outcome, events, counters) = run(limits(), expected);
    assert_eq!(
        run(limits(), actual),
        (outcome, events, counters.clone()),
        "{label}"
    );
    for kind in TIGHTENED {
        let spend = counters[KINDS.iter().position(|k| *k == kind).unwrap()];
        for bound in [spend, spend.saturating_sub(1)] {
            let l = limited(kind, bound);
            assert_eq!(run(l, actual), run(l, expected), "{label} {kind:?} {bound}");
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

fn range(a: i128, b: i128) -> Range {
    Range::new(a, b).unwrap()
}

fn general(domain: Option<Range>) -> IntegerDomain {
    domain.map_or(IntegerDomain::Mathematical, |d| {
        IntegerDomain::Bounded(
            IntegerInterval::new(Integer::from(d.lower()), Integer::from(d.upper())).unwrap(),
        )
    })
}

/// The seven member combinations of FR-409.
#[derive(Clone, Copy, Debug)]
enum Combo {
    Quotient(DivisionProfile),
    Remainder(DivisionProfile),
    Modulo,
}

fn combos() -> Vec<Combo> {
    let mut all = Vec::new();
    for profile in DivisionProfile::ALL {
        all.push(Combo::Quotient(profile));
        all.push(Combo::Remainder(profile));
    }
    all.push(Combo::Modulo);
    all
}

impl Combo {
    fn operation(self, dividend: Range, divisor: Range) -> Op {
        match self {
            Self::Quotient(profile) => Op::Divide {
                profile,
                member: DivisionMember::Quotient,
                dividend,
                divisor,
            },
            Self::Remainder(profile) => Op::Divide {
                profile,
                member: DivisionMember::Remainder,
                dividend,
                divisor,
            },
            Self::Modulo => Op::Modulo { dividend, divisor },
        }
    }
    /// The exact `divide` or `modulo` over `Integer`.
    fn existing(
        self,
        a: i128,
        b: i128,
        domain: Option<Range>,
        meter: &mut Meter,
    ) -> Outcome<Integer> {
        let (ia, ib, dom) = (Integer::from(a), Integer::from(b), general(domain));
        match self {
            Self::Quotient(p) => divide(p, DivisionMember::Quotient, &ia, &ib, &dom, meter),
            Self::Remainder(p) => divide(p, DivisionMember::Remainder, &ia, &ib, &dom, meter),
            Self::Modulo => modulo(&ia, &ib, &dom, meter),
        }
    }
    /// The plan-free native evaluator.
    fn plan_free(self, a: i128, b: i128, domain: Option<Range>, meter: &mut Meter) -> Outcome<N> {
        match self {
            Self::Quotient(p) => evaluate_native_integer_division(
                p,
                DivisionMember::Quotient,
                N::new(a),
                N::new(b),
                domain,
                meter,
            ),
            Self::Remainder(p) => evaluate_native_integer_division(
                p,
                DivisionMember::Remainder,
                N::new(a),
                N::new(b),
                domain,
                meter,
            ),
            Self::Modulo => evaluate_native_integer_modulo(N::new(a), N::new(b), domain, meter),
        }
    }
    fn is_quotient(self) -> bool {
        matches!(self, Self::Quotient(_))
    }
}

fn plan_outcome(plan: Plan, a: i128, b: i128, meter: &mut Meter) -> Outcome<N> {
    let admitted = plan
        .admit(Args::Binary {
            left: N::new(a),
            right: N::new(b),
        })
        .unwrap();
    evaluate_native_integer_arithmetic(admitted, meter)
}

#[test]
#[trace("TC-957", "FR-409-AC-1")]
fn plans_admit_members_and_reject_everything_else_before_any_charge() {
    for combo in combos() {
        let dividend = range(-10, 10);
        let divisor = range(-3, 3);
        let plan = Plan::new(combo.operation(dividend, divisor), None).unwrap();
        let kind = if matches!(combo, Combo::Modulo) {
            Kind::Modulo
        } else {
            Kind::Divide
        };
        // Members, including the zero divisor.
        for (a, b) in [(7, 2), (-10, 0), (10, -3), (0, 0)] {
            assert!(plan
                .admit(Args::Binary {
                    left: N::new(a),
                    right: N::new(b)
                })
                .is_ok());
        }
        let outside_dividend = plan.admit(Args::Binary {
            left: N::new(11),
            right: N::new(1),
        });
        assert_eq!(
            outside_dividend,
            Err(Error::OperandOutsideInterval {
                operand: Position::Left,
                value: 11,
                interval: dividend
            })
        );
        let outside_divisor = plan.admit(Args::Binary {
            left: N::new(1),
            right: N::new(-4),
        });
        assert_eq!(
            outside_divisor,
            Err(Error::OperandOutsideInterval {
                operand: Position::Right,
                value: -4,
                interval: divisor
            })
        );
        assert_eq!(
            plan.admit(Args::Unary { operand: N::new(1) }),
            Err(Error::OperandShapeMismatch { operation: kind })
        );
    }
    // None of the rejections charged: admission takes no Meter, and the
    // recording meter below sees nothing from a refused plan construction.
    let mut meter = Meter::new(limits());
    meter.enable_charge_capture(8, 8);
    let refused = Plan::new(
        Combo::Quotient(DivisionProfile::Floor).operation(range(i128::MIN, 0), range(-1, -1)),
        None,
    );
    assert!(refused.is_err());
    let capture = meter.disable_charge_capture().unwrap().unwrap();
    assert!(capture.events().is_empty());
}

fn capacity(combo: Combo, dividend: Range, divisor: Range) -> Range {
    Plan::new(combo.operation(dividend, divisor), None)
        .unwrap()
        .capacity()
}

#[test]
#[trace("TC-957", "FR-409-AC-2")]
fn quotient_capacity_refuses_min_over_minus_one_and_other_capacities_match_literals() {
    let none: Option<Range> = None;
    for profile in DivisionProfile::ALL {
        let quotient = Combo::Quotient(profile);
        for result in [none, Some(range(-1, 1))] {
            for divisor in [
                range(-1, -1),
                range(-5, 5),
                range(-3, 0),
                range(i128::MIN, -1),
            ] {
                assert_eq!(
                    Plan::new(quotient.operation(range(i128::MIN, 0), divisor), result),
                    Err(Error::CapacityExceeded {
                        operation: Kind::Divide
                    }),
                    "{profile:?} {divisor:?} {result:?}"
                );
            }
        }
        // The same intervals construct for the remainder member.
        let remainder = Combo::Remainder(profile);
        assert!(Plan::new(
            remainder.operation(range(i128::MIN, 0), range(-1, -1)),
            none
        )
        .is_ok());
        // MIN with a divisor that excludes -1, and -1 with a dividend above MIN.
        assert!(Plan::new(
            quotient.operation(range(i128::MIN, i128::MIN), range(1, 2)),
            none
        )
        .is_ok());
        assert!(Plan::new(
            quotient.operation(range(i128::MIN + 1, 0), range(-1, -1)),
            none
        )
        .is_ok());
        assert_eq!(
            capacity(quotient, range(i128::MIN, i128::MIN), range(1, 2)),
            range(i128::MIN, i128::MAX)
        );
        assert_eq!(
            capacity(quotient, range(i128::MIN + 1, 0), range(-1, -1)),
            range(i128::MIN + 1, i128::MAX)
        );
        assert_eq!(capacity(quotient, range(-5, 3), range(-2, 2)), range(-5, 5));
        assert_eq!(capacity(quotient, range(4, 4), range(0, 0)), range(-4, 4));
        // Remainder: [-(B'-1), B'-1].
        assert_eq!(
            capacity(remainder, range(-100, 100), range(2, 3)),
            range(-2, 2)
        );
        assert_eq!(
            capacity(remainder, range(0, 0), range(i128::MIN, 5)),
            range(-i128::MAX, i128::MAX)
        );
        assert_eq!(capacity(remainder, range(0, 5), range(0, 0)), range(0, 0));
        assert_eq!(capacity(remainder, range(0, 5), range(1, 1)), range(0, 0));
        assert_eq!(capacity(remainder, range(0, 5), range(-7, 7)), range(-6, 6));
    }
    // Modulo: [0, B'-1], never refused.
    let m = Combo::Modulo;
    assert_eq!(capacity(m, range(i128::MIN, 0), range(-1, -1)), range(0, 0));
    assert_eq!(capacity(m, range(-100, 100), range(2, 3)), range(0, 2));
    assert_eq!(
        capacity(m, range(0, 0), range(i128::MIN, 5)),
        range(0, i128::MAX)
    );
    assert_eq!(capacity(m, range(0, 5), range(0, 0)), range(0, 0));
    assert!(Plan::new(
        m.operation(range(i128::MIN, i128::MAX), range(i128::MIN, i128::MAX)),
        None
    )
    .is_ok());
}

/// A member of the exact result and a result interval that holds, and one
/// that excludes, it; `None` where the member leaves `i128`.
fn domains_for(combo: Combo, a: i128, b: i128) -> Vec<Option<Range>> {
    let mut domains = vec![None];
    if b != 0 {
        let mut meter = Meter::new(limits());
        // The exact member, from the plan-free evaluator over no domain; the
        // quotient 2^127 has no native value and gets only the fixed domain.
        if let Outcome::Completed(m) = combo.plan_free(a, b, None, &mut meter) {
            let m = m.value();
            domains.push(Some(range(m, m)));
            let other = if m == i128::MAX { m - 1 } else { m + 1 };
            domains.push(Some(range(other, other)));
        }
    }
    domains.push(Some(range(-5, 5)));
    domains
}

#[test]
#[trace("TC-957", "FR-409-AC-3")]
fn the_grid_matches_divide_modulo_and_the_plan_free_evaluators() {
    for combo in combos() {
        for &a in &GRID {
            for &b in &GRID {
                for domain in domains_for(combo, a, b) {
                    let label = format!("{combo:?} {a} {b} {domain:?}");
                    let refused_for_capacity = combo.is_quotient() && a == i128::MIN && b == -1;
                    // The plan over the exact operands, and over a wider pair.
                    let wide = (range(a.min(0), a.max(0)), range(b.min(0), b.max(0)));
                    for (dividend, divisor) in [(range(a, a), range(b, b)), wide] {
                        let leaves = combo.is_quotient()
                            && dividend.lower() == i128::MIN
                            && divisor.lower() <= -1
                            && -1 <= divisor.upper();
                        let plan = Plan::new(combo.operation(dividend, divisor), domain);
                        if leaves {
                            assert_eq!(
                                plan,
                                Err(Error::CapacityExceeded {
                                    operation: Kind::Divide
                                }),
                                "{label}"
                            );
                            continue;
                        }
                        let plan = plan.expect(&label);
                        assert!(!refused_for_capacity, "{label}");
                        same(&label, &|m| abstracted(plan_outcome(plan, a, b, m)), &|m| {
                            combo.existing(a, b, domain, m)
                        });
                        same(&label, &|m| abstracted(plan_outcome(plan, a, b, m)), &|m| {
                            abstracted(combo.plan_free(a, b, domain, m))
                        });
                    }
                }
            }
        }
    }
}

#[test]
#[trace("TC-957", "FR-409-AC-3")]
fn independent_literals_zero_divisor_and_out_of_domain_payloads() {
    let mut meter = Meter::new(limits());
    let eval = |combo: Combo, a: i128, b: i128, domain: Option<Range>, meter: &mut Meter| {
        let plan = Plan::new(
            combo.operation(range(-i128::MAX, i128::MAX), range(-i128::MAX, i128::MAX)),
            domain,
        )
        .unwrap();
        plan_outcome(plan, a, b, meter)
    };
    // -7 div 2 and -7 div -2 under the three laws.
    for (profile, expected) in [
        (DivisionProfile::Truncating, [-3, 3]),
        (DivisionProfile::Floor, [-4, 3]),
        (DivisionProfile::Euclidean, [-4, 4]),
    ] {
        let c = Combo::Quotient(profile);
        assert_eq!(
            eval(c, -7, 2, None, &mut meter),
            Outcome::Completed(N::new(expected[0]))
        );
        assert_eq!(
            eval(c, -7, -2, None, &mut meter),
            Outcome::Completed(N::new(expected[1]))
        );
    }
    assert_eq!(
        eval(Combo::Modulo, -7, 2, None, &mut meter),
        Outcome::Completed(N::new(1))
    );
    assert_eq!(
        eval(Combo::Modulo, -7, -2, None, &mut meter),
        Outcome::Completed(N::new(1))
    );
    // i64::MIN div -1 is 2^63 and i64::MIN mod -1 is 0.
    let min64 = i128::from(i64::MIN);
    for profile in DivisionProfile::ALL {
        assert_eq!(
            eval(Combo::Quotient(profile), min64, -1, None, &mut meter),
            Outcome::Completed(N::new(1 << 63))
        );
    }
    assert_eq!(
        eval(Combo::Modulo, min64, -1, None, &mut meter),
        Outcome::Completed(N::new(0))
    );
    // A zero divisor stops after the operands charge.
    for combo in combos() {
        let mut m = Meter::new(limits());
        m.enable_charge_capture(8, 8);
        assert_eq!(
            eval(combo, 5, 0, None, &mut m),
            Outcome::Undefined(Undefined::DivisionByZero)
        );
        let events = m.disable_charge_capture().unwrap().unwrap();
        assert_eq!(events.events().len(), 1, "{combo:?}");
        assert!(matches!(
            events.events()[0].point,
            ChargePoint::IntegerDivisionOperands | ChargePoint::IntegerModulusOperands
        ));
    }
    // Out-of-domain payloads: 100 div 3 is 33 and 100 rem 3 is 1.
    let payload = |low: i128, high: i128| {
        Box::new(IntegerInterval::new(Integer::from(low), Integer::from(high)).unwrap())
    };
    assert_eq!(
        eval(
            Combo::Quotient(DivisionProfile::Truncating),
            100,
            3,
            Some(range(0, 2)),
            &mut meter
        ),
        Outcome::Refused(Refusal::DivisionOutOfDomain {
            domain: payload(0, 2),
            member: DivisionMember::Quotient
        })
    );
    assert_eq!(
        eval(
            Combo::Remainder(DivisionProfile::Floor),
            100,
            3,
            Some(range(5, 6)),
            &mut meter
        ),
        Outcome::Refused(Refusal::DivisionOutOfDomain {
            domain: payload(5, 6),
            member: DivisionMember::Remainder
        })
    );
    assert_eq!(
        eval(Combo::Modulo, 100, 3, Some(range(5, 6)), &mut meter),
        Outcome::Refused(Refusal::ModuloOutOfDomain {
            domain: payload(5, 6)
        })
    );
    assert_eq!(
        eval(
            Combo::Remainder(DivisionProfile::Floor),
            100,
            3,
            Some(range(1, 1)),
            &mut meter
        ),
        Outcome::Completed(N::new(1))
    );
}

#[test]
#[trace("TC-957", "FR-409-AC-4")]
fn the_schedule_over_an_admitted_division_equals_the_division_schedule() {
    for combo in combos() {
        for &a in &[0, 1, -7, i64::MIN as i128, i128::MAX] {
            for &b in &[0, 1, -1, 2, -7, i128::MIN, i128::MAX] {
                let plan = match Plan::new(combo.operation(range(a, a), range(b, b)), None) {
                    Ok(plan) => plan,
                    Err(_) => continue,
                };
                let admitted = plan
                    .admit(Args::Binary {
                        left: N::new(a),
                        right: N::new(b),
                    })
                    .unwrap();
                let direct = if matches!(combo, Combo::Modulo) {
                    Sched::Modulus {
                        dividend: N::new(a),
                        divisor: N::new(b),
                    }
                } else {
                    Sched::Division {
                        dividend: N::new(a),
                        divisor: N::new(b),
                    }
                };
                let schedule = |s: Sched, m: &mut Meter| {
                    let outcome = charge_native_integer_schedule(s, m);
                    match outcome {
                        Outcome::Completed(()) => Outcome::Completed(Integer::from(0_i128)),
                        Outcome::Undefined(v) => Outcome::Undefined(v),
                        Outcome::Refused(v) => Outcome::Refused(v),
                        Outcome::Incomplete(v) => Outcome::Incomplete(v),
                    }
                };
                same(
                    &format!("{combo:?} {a} {b}"),
                    &|m| schedule(Sched::Arithmetic(admitted), m),
                    &|m| schedule(direct, m),
                );
            }
        }
    }
}

fn replay_combo(token: &str) -> Combo {
    let (kind, law) = token.split_once(':').unwrap_or((token, "Euclidean"));
    let profile = match law {
        "Truncating" => DivisionProfile::Truncating,
        "Floor" => DivisionProfile::Floor,
        _ => DivisionProfile::Euclidean,
    };
    match kind {
        "Q" => Combo::Quotient(profile),
        "R" => Combo::Remainder(profile),
        _ => Combo::Modulo,
    }
}

/// Replay of a decoded Kani playback (TC-957 step 8), driven by the mutant
/// script through `IR1541_REPLAY`; it does nothing without it.
///
/// `eval|combo|a|b|dividend_lo|dividend_hi|divisor_lo|divisor_hi|present|lo|hi`
/// prints the plan outcome beside `divide` or `modulo` over `Integer`;
/// `plan|combo|dividend_lo|dividend_hi|divisor_lo|divisor_hi|present|lo|hi`
/// prints the plan construction.
#[test]
#[ignore = "driven by scripts/ir1541_native_integer_mutants.py through IR1541_REPLAY"]
#[trace("TC-957", "FR-409-AC-9")]
fn replay_a_decoded_playback() {
    let Ok(spec) = std::env::var("IR1541_REPLAY") else {
        return;
    };
    let f: Vec<&str> = spec.split('|').collect();
    let n = |i: usize| f[i].parse::<i128>().unwrap();
    let domain = |at: usize| (f[at] == "1").then(|| range(n(at + 1), n(at + 2)));
    let combo = replay_combo(f[1]);
    if f[0] == "plan" {
        let plan = Plan::new(
            combo.operation(range(n(2), n(3)), range(n(4), n(5))),
            domain(6),
        );
        println!("plan={:?}", plan.map(Plan::capacity));
        return;
    }
    let (a, b) = (n(2), n(3));
    let result = domain(8);
    let plan = Plan::new(
        combo.operation(range(n(4), n(5)), range(n(6), n(7))),
        result,
    )
    .unwrap();
    let outcome = abstracted(plan_outcome(plan, a, b, &mut Meter::new(limits())));
    println!("plan={outcome:?}");
    println!(
        "exact={:?}",
        combo.existing(a, b, result, &mut Meter::new(limits()))
    );
}
