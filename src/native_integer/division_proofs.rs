// SPDX-License-Identifier: AGPL-3.0-or-later
//! Kani harnesses of FR-409 Behavior 13 to 16: the native `Divide` and
//! `Modulo` plan, from construction through admission to evaluation under a
//! real `Meter`.
//!
//! Plan harnesses (one per operation kind: quotient, remainder, `Modulo`)
//! draw both intervals' endpoints and the result interval unrestricted. They
//! determine whether `NativeIntegerPlan::new` refuses, the capacity interval
//! and whether admission succeeds. They perform no division.
//!
//! Evaluation harnesses (seven member combinations; five symbolic-divisor
//! widths and six constant-divisor shapes each) draw both operands within the
//! shape's intervals and the plan's result interval as `None` or a sorted
//! pair of unrestricted endpoints, then evaluate under a `Meter` that denies
//! nothing. Each determines the outcome kind (a zero divisor is
//! `Undefined(DivisionByZero)`; a member outside the result interval is the
//! out-of-domain refusal of its kind with its `member`; every other outcome
//! is unreachable) and the completed value. The completed value is tied to
//! the inputs only by the relation `a - q * b` in the law's remainder window
//! `W` (`W` holds exactly `|b|` integers, so it determines `q`), written
//! with products ordered quotient then divisor, through `i128` (width up to
//! 64) or the independent signed-limb abstraction of `reference.rs` (width
//! 128). The expected side holds no Rust division or remainder operator and
//! no function of the production body. A remainder or `Modulo` harness takes
//! its companion quotient from the plan-free
//! `evaluate_native_integer_division`, as a second subject output that the
//! relation pins. Charge amounts and limit denials are left to the host
//! differential.
use super::reference::MathematicalInteger as Math;
use super::*;
use crate::accounting::ScalarLimits;

fn generous_meter() -> Meter {
    Meter::new(ScalarLimits {
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
    })
}

fn sorted(a: i128, b: i128) -> NativeIntegerInterval {
    NativeIntegerInterval::new(a.min(b), a.max(b)).unwrap()
}

fn drawn_result_interval() -> Option<NativeIntegerInterval> {
    let present: bool = kani::any();
    let a: i128 = kani::any();
    let b: i128 = kani::any();
    present.then(|| sorted(a, b))
}

/// The member combination a harness evaluates.
#[derive(Clone, Copy)]
enum Combo {
    Quotient(DivisionProfile),
    Remainder(DivisionProfile),
    Modulo,
}

impl Combo {
    fn profile(self) -> DivisionProfile {
        match self {
            Self::Quotient(p) | Self::Remainder(p) => p,
            Self::Modulo => DivisionProfile::Euclidean,
        }
    }
    fn operation(
        self,
        dividend: NativeIntegerInterval,
        divisor: NativeIntegerInterval,
    ) -> NativeIntegerOperation {
        match self {
            Self::Quotient(profile) => NativeIntegerOperation::Divide {
                profile,
                member: DivisionMember::Quotient,
                dividend,
                divisor,
            },
            Self::Remainder(profile) => NativeIntegerOperation::Divide {
                profile,
                member: DivisionMember::Remainder,
                dividend,
                divisor,
            },
            Self::Modulo => NativeIntegerOperation::Modulo { dividend, divisor },
        }
    }
}

/// The profile's remainder window `W` for dividend `a` and divisor `b != 0`:
/// exactly `|b|` consecutive integers.
fn window(profile: DivisionProfile, a: i128, b: i128) -> (i128, i128) {
    let reach = i128::try_from(b.unsigned_abs() - 1).unwrap_or(i128::MAX);
    match profile {
        DivisionProfile::Truncating if a >= 0 => (0, reach),
        DivisionProfile::Truncating => (-reach, 0),
        DivisionProfile::Floor if b > 0 => (0, reach),
        DivisionProfile::Floor => (b + 1, 0),
        DivisionProfile::Euclidean => (0, reach),
    }
}

/// Whether `low <= a - q * b <= high`, exactly. Narrow widths use `i128`,
/// where every product fits; width 128 uses the signed-limb abstraction.
fn residual_within(wide: bool, a: i128, q: i128, b: i128, low: i128, high: i128) -> bool {
    if wide {
        let product = Math::evaluate(NativeIntegerOperationKind::Multiply, q, b);
        // low <= a - q*b <= high  <=>  a - high <= q*b <= a - low
        Math::evaluate(NativeIntegerOperationKind::Subtract, a, high) <= product
            && product <= Math::evaluate(NativeIntegerOperationKind::Subtract, a, low)
    } else {
        let residual = a - q * b;
        low <= residual && residual <= high
    }
}

fn width_interval(bits: u32) -> NativeIntegerInterval {
    if bits == 128 {
        NativeIntegerInterval::new(i128::MIN, i128::MAX).unwrap()
    } else {
        NativeIntegerInterval::new(-(1_i128 << (bits - 1)), (1_i128 << (bits - 1)) - 1).unwrap()
    }
}

fn is_breach(outcome: &Outcome<NativeInteger>) -> bool {
    matches!(
        outcome,
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::NativeIntegerCapacityBreach
        })
    )
}

/// Evaluate one shape: `divisor` is `Some(literal)` for a constant divisor
/// and `None` for the symbolic `bits`-wide divisor interval.
fn evaluation(combo: Combo, divisor: Option<i128>, bits: u32) {
    let wide = bits == 128;
    let mut dividend_interval = width_interval(bits);
    if wide && matches!(combo, Combo::Quotient(_)) {
        dividend_interval = NativeIntegerInterval::new(i128::MIN + 1, i128::MAX).unwrap();
    }
    let divisor_interval = match divisor {
        Some(d) => NativeIntegerInterval::new(d, d).unwrap(),
        None => width_interval(bits),
    };
    let result = drawn_result_interval();
    let plan = NativeIntegerPlan::new(combo.operation(dividend_interval, divisor_interval), result)
        .unwrap();
    let a: i128 = kani::any();
    let b: i128 = kani::any();
    kani::assume(dividend_interval.contains(NativeInteger(a)));
    kani::assume(divisor_interval.contains(NativeInteger(b)));
    let admitted = plan
        .admit(NativeIntegerOperands::Binary {
            left: NativeInteger(a),
            right: NativeInteger(b),
        })
        .unwrap();
    // Refusals own boxed intervals whose drop glue Kani unwinds without bound;
    // the outcomes are inspected by reference and never dropped.
    let outcome = core::mem::ManuallyDrop::new(evaluate_native_integer_arithmetic(
        admitted,
        &mut generous_meter(),
    ));
    let outcome = &*outcome;
    let law = combo.profile();
    let excludes = |value_in: &dyn Fn(NativeIntegerInterval) -> bool| {
        result.is_some_and(|interval| !value_in(interval))
    };
    if b == 0 {
        assert!(
            matches!(*outcome, Outcome::Undefined(Undefined::DivisionByZero)),
            "a zero divisor is Undefined(DivisionByZero)"
        );
        kani::cover!(true, "zero divisor refusal reached");
        return;
    }
    let (low, high) = window(law, a, b);
    // The plan-free quotient under the harness's own law, a second subject
    // output; the relation below pins it.
    let companion = core::mem::ManuallyDrop::new(evaluate_native_integer_division(
        law,
        DivisionMember::Quotient,
        NativeInteger(a),
        NativeInteger(b),
        None,
        &mut generous_meter(),
    ));
    let companion = &*companion;
    match combo {
        Combo::Quotient(_) => {
            let Outcome::Completed(q) = companion else {
                assert!(false, "the quotient plan excludes MIN / -1");
                return;
            };
            let q = q.value();
            assert!(
                residual_within(wide, a, q, b, low, high),
                "companion quotient relation"
            );
            let in_result = |i: NativeIntegerInterval| i.contains(NativeInteger(q));
            match outcome {
                Outcome::Completed(v) => {
                    assert!(!excludes(&in_result), "an excluded member must refuse");
                    assert!(
                        residual_within(wide, a, v.value(), b, low, high),
                        "completed quotient satisfies a - q*b in W"
                    );
                    kani::cover!(true, "completed quotient reached");
                }
                Outcome::Refused(Refusal::DivisionOutOfDomain { member, .. }) => {
                    assert!(*member == DivisionMember::Quotient);
                    assert!(excludes(&in_result), "only an excluded member refuses");
                    kani::cover!(true, "quotient out-of-domain refusal reached");
                }
                _ => assert!(false, "no other outcome is reachable"),
            }
        }
        Combo::Remainder(_) | Combo::Modulo => {
            // The exact remainder as a relation on the companion quotient:
            // `a - q * b`, or 0 where the quotient is 2^127 (i128::MIN, -1).
            let exact_in = |i: NativeIntegerInterval| match companion {
                Outcome::Completed(q) => {
                    let q = q.value();
                    assert!(
                        residual_within(wide, a, q, b, low, high),
                        "companion relation"
                    );
                    residual_within(wide, a, q, b, i.lower(), i.upper())
                }
                other => {
                    assert!(
                        is_breach(other),
                        "companion is a quotient or the MIN / -1 breach"
                    );
                    assert!(a == i128::MIN && b == -1 && low == 0 && high == 0);
                    i.contains(NativeInteger(0))
                }
            };
            match outcome {
                Outcome::Completed(r) => {
                    let r = r.value();
                    assert!(low <= r && r <= high, "completed remainder lies in W");
                    match companion {
                        Outcome::Completed(q) => assert!(
                            residual_within(wide, a, q.value(), b, r, r),
                            "a == q * b + r"
                        ),
                        _ => assert!(r == 0, "MIN / -1 has remainder 0"),
                    }
                    assert!(!excludes(&exact_in), "an excluded member must refuse");
                    kani::cover!(true, "completed remainder reached");
                }
                Outcome::Refused(Refusal::DivisionOutOfDomain { member, .. }) => {
                    assert!(matches!(combo, Combo::Remainder(_)));
                    assert!(*member == DivisionMember::Remainder);
                    assert!(excludes(&exact_in), "only an excluded member refuses");
                    kani::cover!(true, "remainder out-of-domain refusal reached");
                }
                Outcome::Refused(Refusal::ModuloOutOfDomain { .. }) => {
                    assert!(matches!(combo, Combo::Modulo));
                    assert!(excludes(&exact_in), "only an excluded member refuses");
                    kani::cover!(true, "modulo out-of-domain refusal reached");
                }
                _ => assert!(false, "no other outcome is reachable"),
            }
        }
    }
}

fn plan_intervals() -> (
    NativeIntegerInterval,
    NativeIntegerInterval,
    Option<NativeIntegerInterval>,
) {
    let (a, b, c, d): (i128, i128, i128, i128) =
        (kani::any(), kani::any(), kani::any(), kani::any());
    (sorted(a, b), sorted(c, d), drawn_result_interval())
}

fn plan_admission(
    plan: NativeIntegerPlan,
    dividend: NativeIntegerInterval,
    divisor: NativeIntegerInterval,
) {
    let (x, y): (i128, i128) = (kani::any(), kani::any());
    let admitted = plan.admit(NativeIntegerOperands::Binary {
        left: NativeInteger(x),
        right: NativeInteger(y),
    });
    let member = dividend.lower() <= x
        && x <= dividend.upper()
        && divisor.lower() <= y
        && y <= divisor.upper();
    assert_eq!(
        admitted.is_ok(),
        member,
        "admission succeeds exactly on members"
    );
    kani::cover!(admitted.is_ok(), "admitted pair reached");
    kani::cover!(!admitted.is_ok(), "rejected pair reached");
}

/// The largest endpoint magnitude, written without the production helper.
fn widest(interval: NativeIntegerInterval) -> u128 {
    let magnitude = |v: i128| if v < 0 { v.unsigned_abs() } else { v as u128 };
    magnitude(interval.lower()).max(magnitude(interval.upper()))
}

/// Quotient plan: construction refuses exactly for `i128::MIN` in the dividend
/// with `-1` in the divisor; otherwise the capacity is `[-M, M]` clamped.
/// Assigned mutant: MIN / -1 unguarded.
#[kani::proof]
fn divide_quotient_plan_refuses_min_over_minus_one_and_derives_capacity() {
    let (dividend, divisor, result) = plan_intervals();
    let law = match kani::any::<u8>() {
        0 => DivisionProfile::Truncating,
        1 => DivisionProfile::Floor,
        _ => DivisionProfile::Euclidean,
    };
    let plan = NativeIntegerPlan::new(
        NativeIntegerOperation::Divide {
            profile: law,
            member: DivisionMember::Quotient,
            dividend,
            divisor,
        },
        result,
    );
    let refused = dividend.lower() == i128::MIN && divisor.lower() <= -1 && -1 <= divisor.upper();
    assert_eq!(plan.is_err(), refused, "refuses exactly MIN / -1");
    kani::cover!(refused, "MIN / -1 refused");
    if let Ok(plan) = plan {
        let m = widest(dividend);
        let expected_upper = if m > i128::MAX as u128 {
            i128::MAX
        } else {
            m as i128
        };
        let expected_lower = if m > i128::MAX as u128 {
            i128::MIN
        } else {
            -(m as i128)
        };
        assert_eq!(plan.capacity.lower, expected_lower);
        assert_eq!(plan.capacity.upper, expected_upper);
        plan_admission(plan, dividend, divisor);
        kani::cover!(m > i128::MAX as u128, "full-width quotient capacity");
    } else {
        assert_eq!(
            plan,
            Err(NativeIntegerAdmissionError::CapacityExceeded {
                operation: NativeIntegerOperationKind::Divide
            })
        );
    }
}

/// Remainder plan: never refused; capacity `[-(B' - 1), B' - 1]`.
/// Assigned mutant: remainder capacity widened.
#[kani::proof]
fn divide_remainder_plan_derives_capacity_and_never_refuses() {
    let (dividend, divisor, result) = plan_intervals();
    let law = match kani::any::<u8>() {
        0 => DivisionProfile::Truncating,
        1 => DivisionProfile::Floor,
        _ => DivisionProfile::Euclidean,
    };
    let plan = NativeIntegerPlan::new(
        NativeIntegerOperation::Divide {
            profile: law,
            member: DivisionMember::Remainder,
            dividend,
            divisor,
        },
        result,
    )
    .unwrap();
    let reach = (widest(divisor).max(1) - 1) as i128;
    assert_eq!(plan.capacity.lower, -reach);
    assert_eq!(plan.capacity.upper, reach);
    plan_admission(plan, dividend, divisor);
    kani::cover!(reach == i128::MAX, "full-width remainder capacity");
}

/// `Modulo` plan: never refused; capacity `[0, B' - 1]`.
/// Assigned mutant: `Modulo` capacity lowered.
#[kani::proof]
fn modulo_plan_derives_capacity_and_never_refuses() {
    let (dividend, divisor, result) = plan_intervals();
    let plan = NativeIntegerPlan::new(NativeIntegerOperation::Modulo { dividend, divisor }, result)
        .unwrap();
    let reach = (widest(divisor).max(1) - 1) as i128;
    assert_eq!(plan.capacity.lower, 0);
    assert_eq!(plan.capacity.upper, reach);
    plan_admission(plan, dividend, divisor);
    kani::cover!(reach == i128::MAX, "full-width modulo capacity");
}

macro_rules! evaluation_harness {
    ($name:ident, $combo:expr, $divisor:expr, $bits:expr, $doc:expr) => {
        #[doc = $doc]
        #[kani::proof]
        fn $name() {
            evaluation($combo, $divisor, $bits)
        }
    };
}

evaluation_harness!(trunc_quotient_sym_w8, Combo::Quotient(DivisionProfile::Truncating), None, 8, "Evaluation, truncating quotient, symbolic divisor, width 8. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_sym_w16, Combo::Quotient(DivisionProfile::Truncating), None, 16, "Evaluation, truncating quotient, symbolic divisor, width 16. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_sym_w32, Combo::Quotient(DivisionProfile::Truncating), None, 32, "Evaluation, truncating quotient, symbolic divisor, width 32. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_sym_w64, Combo::Quotient(DivisionProfile::Truncating), None, 64, "Evaluation, truncating quotient, symbolic divisor, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_sym_w128, Combo::Quotient(DivisionProfile::Truncating), None, 128, "Evaluation, truncating quotient, symbolic divisor, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_const_7_w64, Combo::Quotient(DivisionProfile::Truncating), Some(7), 64, "Evaluation, truncating quotient, constant divisor 7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_const_7_w128, Combo::Quotient(DivisionProfile::Truncating), Some(7), 128, "Evaluation, truncating quotient, constant divisor 7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_const_neg7_w64, Combo::Quotient(DivisionProfile::Truncating), Some(-7), 64, "Evaluation, truncating quotient, constant divisor -7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_const_neg7_w128, Combo::Quotient(DivisionProfile::Truncating), Some(-7), 128, "Evaluation, truncating quotient, constant divisor -7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_quotient_const_neg1_w64, Combo::Quotient(DivisionProfile::Truncating), Some(-1), 64, "Evaluation, truncating quotient, constant divisor -1, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(trunc_quotient_const_neg1_w128, Combo::Quotient(DivisionProfile::Truncating), Some(-1), 128, "Evaluation, truncating quotient, constant divisor -1, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(floor_quotient_sym_w8, Combo::Quotient(DivisionProfile::Floor), None, 8, "Evaluation, floor quotient, symbolic divisor, width 8. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_sym_w16, Combo::Quotient(DivisionProfile::Floor), None, 16, "Evaluation, floor quotient, symbolic divisor, width 16. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_sym_w32, Combo::Quotient(DivisionProfile::Floor), None, 32, "Evaluation, floor quotient, symbolic divisor, width 32. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_sym_w64, Combo::Quotient(DivisionProfile::Floor), None, 64, "Evaluation, floor quotient, symbolic divisor, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_sym_w128, Combo::Quotient(DivisionProfile::Floor), None, 128, "Evaluation, floor quotient, symbolic divisor, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_const_7_w64, Combo::Quotient(DivisionProfile::Floor), Some(7), 64, "Evaluation, floor quotient, constant divisor 7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_const_7_w128, Combo::Quotient(DivisionProfile::Floor), Some(7), 128, "Evaluation, floor quotient, constant divisor 7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_const_neg7_w64, Combo::Quotient(DivisionProfile::Floor), Some(-7), 64, "Evaluation, floor quotient, constant divisor -7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_const_neg7_w128, Combo::Quotient(DivisionProfile::Floor), Some(-7), 128, "Evaluation, floor quotient, constant divisor -7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_quotient_const_neg1_w64, Combo::Quotient(DivisionProfile::Floor), Some(-1), 64, "Evaluation, floor quotient, constant divisor -1, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(floor_quotient_const_neg1_w128, Combo::Quotient(DivisionProfile::Floor), Some(-1), 128, "Evaluation, floor quotient, constant divisor -1, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(euclid_quotient_sym_w8, Combo::Quotient(DivisionProfile::Euclidean), None, 8, "Evaluation, Euclidean quotient, symbolic divisor, width 8. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_sym_w16, Combo::Quotient(DivisionProfile::Euclidean), None, 16, "Evaluation, Euclidean quotient, symbolic divisor, width 16. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_sym_w32, Combo::Quotient(DivisionProfile::Euclidean), None, 32, "Evaluation, Euclidean quotient, symbolic divisor, width 32. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_sym_w64, Combo::Quotient(DivisionProfile::Euclidean), None, 64, "Evaluation, Euclidean quotient, symbolic divisor, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_sym_w128, Combo::Quotient(DivisionProfile::Euclidean), None, 128, "Evaluation, Euclidean quotient, symbolic divisor, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_const_7_w64, Combo::Quotient(DivisionProfile::Euclidean), Some(7), 64, "Evaluation, Euclidean quotient, constant divisor 7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_const_7_w128, Combo::Quotient(DivisionProfile::Euclidean), Some(7), 128, "Evaluation, Euclidean quotient, constant divisor 7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_const_neg7_w64, Combo::Quotient(DivisionProfile::Euclidean), Some(-7), 64, "Evaluation, Euclidean quotient, constant divisor -7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_const_neg7_w128, Combo::Quotient(DivisionProfile::Euclidean), Some(-7), 128, "Evaluation, Euclidean quotient, constant divisor -7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_quotient_const_neg1_w64, Combo::Quotient(DivisionProfile::Euclidean), Some(-1), 64, "Evaluation, Euclidean quotient, constant divisor -1, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(euclid_quotient_const_neg1_w128, Combo::Quotient(DivisionProfile::Euclidean), Some(-1), 128, "Evaluation, Euclidean quotient, constant divisor -1, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(trunc_remainder_sym_w8, Combo::Remainder(DivisionProfile::Truncating), None, 8, "Evaluation, truncating remainder, symbolic divisor, width 8. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_sym_w16, Combo::Remainder(DivisionProfile::Truncating), None, 16, "Evaluation, truncating remainder, symbolic divisor, width 16. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_sym_w32, Combo::Remainder(DivisionProfile::Truncating), None, 32, "Evaluation, truncating remainder, symbolic divisor, width 32. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_sym_w64, Combo::Remainder(DivisionProfile::Truncating), None, 64, "Evaluation, truncating remainder, symbolic divisor, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_sym_w128, Combo::Remainder(DivisionProfile::Truncating), None, 128, "Evaluation, truncating remainder, symbolic divisor, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_const_7_w64, Combo::Remainder(DivisionProfile::Truncating), Some(7), 64, "Evaluation, truncating remainder, constant divisor 7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_const_7_w128, Combo::Remainder(DivisionProfile::Truncating), Some(7), 128, "Evaluation, truncating remainder, constant divisor 7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_const_neg7_w64, Combo::Remainder(DivisionProfile::Truncating), Some(-7), 64, "Evaluation, truncating remainder, constant divisor -7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_const_neg7_w128, Combo::Remainder(DivisionProfile::Truncating), Some(-7), 128, "Evaluation, truncating remainder, constant divisor -7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to floor.");
evaluation_harness!(trunc_remainder_const_neg1_w64, Combo::Remainder(DivisionProfile::Truncating), Some(-1), 64, "Evaluation, truncating remainder, constant divisor -1, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(trunc_remainder_const_neg1_w128, Combo::Remainder(DivisionProfile::Truncating), Some(-1), 128, "Evaluation, truncating remainder, constant divisor -1, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(floor_remainder_sym_w8, Combo::Remainder(DivisionProfile::Floor), None, 8, "Evaluation, floor remainder, symbolic divisor, width 8. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_sym_w16, Combo::Remainder(DivisionProfile::Floor), None, 16, "Evaluation, floor remainder, symbolic divisor, width 16. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_sym_w32, Combo::Remainder(DivisionProfile::Floor), None, 32, "Evaluation, floor remainder, symbolic divisor, width 32. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_sym_w64, Combo::Remainder(DivisionProfile::Floor), None, 64, "Evaluation, floor remainder, symbolic divisor, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_sym_w128, Combo::Remainder(DivisionProfile::Floor), None, 128, "Evaluation, floor remainder, symbolic divisor, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_const_7_w64, Combo::Remainder(DivisionProfile::Floor), Some(7), 64, "Evaluation, floor remainder, constant divisor 7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_const_7_w128, Combo::Remainder(DivisionProfile::Floor), Some(7), 128, "Evaluation, floor remainder, constant divisor 7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_const_neg7_w64, Combo::Remainder(DivisionProfile::Floor), Some(-7), 64, "Evaluation, floor remainder, constant divisor -7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_const_neg7_w128, Combo::Remainder(DivisionProfile::Floor), Some(-7), 128, "Evaluation, floor remainder, constant divisor -7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(floor_remainder_const_neg1_w64, Combo::Remainder(DivisionProfile::Floor), Some(-1), 64, "Evaluation, floor remainder, constant divisor -1, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(floor_remainder_const_neg1_w128, Combo::Remainder(DivisionProfile::Floor), Some(-1), 128, "Evaluation, floor remainder, constant divisor -1, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(euclid_remainder_sym_w8, Combo::Remainder(DivisionProfile::Euclidean), None, 8, "Evaluation, Euclidean remainder, symbolic divisor, width 8. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_sym_w16, Combo::Remainder(DivisionProfile::Euclidean), None, 16, "Evaluation, Euclidean remainder, symbolic divisor, width 16. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_sym_w32, Combo::Remainder(DivisionProfile::Euclidean), None, 32, "Evaluation, Euclidean remainder, symbolic divisor, width 32. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_sym_w64, Combo::Remainder(DivisionProfile::Euclidean), None, 64, "Evaluation, Euclidean remainder, symbolic divisor, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_sym_w128, Combo::Remainder(DivisionProfile::Euclidean), None, 128, "Evaluation, Euclidean remainder, symbolic divisor, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_const_7_w64, Combo::Remainder(DivisionProfile::Euclidean), Some(7), 64, "Evaluation, Euclidean remainder, constant divisor 7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_const_7_w128, Combo::Remainder(DivisionProfile::Euclidean), Some(7), 128, "Evaluation, Euclidean remainder, constant divisor 7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_const_neg7_w64, Combo::Remainder(DivisionProfile::Euclidean), Some(-7), 64, "Evaluation, Euclidean remainder, constant divisor -7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_const_neg7_w128, Combo::Remainder(DivisionProfile::Euclidean), Some(-7), 128, "Evaluation, Euclidean remainder, constant divisor -7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: the law swapped to truncating.");
evaluation_harness!(euclid_remainder_const_neg1_w64, Combo::Remainder(DivisionProfile::Euclidean), Some(-1), 64, "Evaluation, Euclidean remainder, constant divisor -1, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(euclid_remainder_const_neg1_w128, Combo::Remainder(DivisionProfile::Euclidean), Some(-1), 128, "Evaluation, Euclidean remainder, constant divisor -1, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(modulo_sym_w8, Combo::Modulo, None, 8, "Evaluation, Modulo, symbolic divisor, width 8. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_sym_w16, Combo::Modulo, None, 16, "Evaluation, Modulo, symbolic divisor, width 16. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_sym_w32, Combo::Modulo, None, 32, "Evaluation, Modulo, symbolic divisor, width 32. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_sym_w64, Combo::Modulo, None, 64, "Evaluation, Modulo, symbolic divisor, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_sym_w128, Combo::Modulo, None, 128, "Evaluation, Modulo, symbolic divisor, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_const_7_w64, Combo::Modulo, Some(7), 64, "Evaluation, Modulo, constant divisor 7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_const_7_w128, Combo::Modulo, Some(7), 128, "Evaluation, Modulo, constant divisor 7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_const_neg7_w64, Combo::Modulo, Some(-7), 64, "Evaluation, Modulo, constant divisor -7, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_const_neg7_w128, Combo::Modulo, Some(-7), 128, "Evaluation, Modulo, constant divisor -7, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: Modulo evaluated as the truncating remainder.");
evaluation_harness!(modulo_const_neg1_w64, Combo::Modulo, Some(-1), 64, "Evaluation, Modulo, constant divisor -1, width 64. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
evaluation_harness!(modulo_const_neg1_w128, Combo::Modulo, Some(-1), 128, "Evaluation, Modulo, constant divisor -1, width 128. Determines the outcome kind, the member value and the out-of-domain payload. Assigned mutant: result interval skipped.");
