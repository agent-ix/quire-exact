// SPDX-License-Identifier: AGPL-3.0-or-later
//! Universal capacity/refinement obligations over the actual native plan and
//! production arithmetic calculation. Meter trace equivalence is a separate
//! real-Meter obligation; this proof does not claim to establish its schedule.
use super::reference::MathematicalInteger as Math;
use super::*;

fn refinement(kind: NativeIntegerOperationKind) {
    let a: i128 = kani::any();
    let b: i128 = kani::any();
    let c: i128 = kani::any();
    let d: i128 = kani::any();
    let x: i128 = kani::any();
    let y: i128 = kani::any();
    let left = NativeIntegerInterval::new(a.min(b), a.max(b)).unwrap();
    let right = NativeIntegerInterval::new(c.min(d), c.max(d)).unwrap();
    let operation = match kind {
        NativeIntegerOperationKind::Add => NativeIntegerOperation::Add { left, right },
        NativeIntegerOperationKind::Subtract => NativeIntegerOperation::Subtract { left, right },
        NativeIntegerOperationKind::Multiply => NativeIntegerOperation::Multiply { left, right },
        NativeIntegerOperationKind::Negate => NativeIntegerOperation::Negate { operand: left },
    };
    let p = Math::evaluate(kind, left.lower, right.lower);
    let q = Math::evaluate(kind, left.lower, right.upper);
    let r = Math::evaluate(kind, left.upper, right.lower);
    let s = Math::evaluate(kind, left.upper, right.upper);
    let low = p.min(q).min(r).min(s);
    let high = p.max(q).max(r).max(s);
    let plan = NativeIntegerPlan::new(operation, None);
    assert_eq!(
        plan.is_ok(),
        low.fits_native() && high.fits_native(),
        "complete mathematical interval capacity"
    );
    let operands = match kind {
        NativeIntegerOperationKind::Negate => NativeIntegerOperands::Unary {
            operand: NativeInteger::new(x),
        },
        NativeIntegerOperationKind::Add
        | NativeIntegerOperationKind::Subtract
        | NativeIntegerOperationKind::Multiply => NativeIntegerOperands::Binary {
            left: NativeInteger::new(x),
            right: NativeInteger::new(y),
        },
    };
    if let Ok(plan) = plan {
        assert_eq!(Math::from_native(plan.capacity.lower), low);
        assert_eq!(Math::from_native(plan.capacity.upper), high);
        let admitted = plan.admit(operands);
        let membership = left.contains(NativeInteger::new(x))
            && (kind == NativeIntegerOperationKind::Negate
                || right.contains(NativeInteger::new(y)));
        assert_eq!(admitted.is_ok(), membership, "exact payload membership");
        kani::cover!(admitted.is_ok(), "admitted operation witness");
        if let Ok(admitted) = admitted {
            let result = admitted.calculate();
            assert!(result.is_some(), "admitted operation cannot overflow");
            let result = result.unwrap();
            assert_eq!(
                Math::from_native(result.value()),
                Math::evaluate(kind, x, y),
                "production calculation refines independent signed-limb mathematics"
            );
            assert!(
                plan.capacity.contains(result),
                "complete image contains every admitted result"
            );
        }
    }
}

#[kani::proof]
fn native_add_refines_mathematical_integer() {
    refinement(NativeIntegerOperationKind::Add)
}
#[kani::proof]
fn native_subtract_refines_mathematical_integer() {
    refinement(NativeIntegerOperationKind::Subtract)
}
#[kani::proof]
fn native_negate_refines_mathematical_integer() {
    refinement(NativeIntegerOperationKind::Negate)
}

/// All signed-i128 operand pairs and all four ordering operators, through the
/// public evaluator and the real Meter. The independent signed-magnitude
/// abstraction supplies the expected mathematical order.
#[kani::proof]
fn native_ordering_refines_mathematical_integer() {
    use crate::accounting::ScalarLimits;
    use core::cmp::Ordering;
    let left: i128 = kani::any();
    let right: i128 = kani::any();
    let (operator, expected) = match (kani::any::<bool>(), kani::any::<bool>()) {
        (false, false) => (
            OrderingOperator::Less,
            Math::from_native(left).cmp(&Math::from_native(right)) == Ordering::Less,
        ),
        (false, true) => (
            OrderingOperator::LessOrEqual,
            Math::from_native(left).cmp(&Math::from_native(right)) != Ordering::Greater,
        ),
        (true, false) => (
            OrderingOperator::Greater,
            Math::from_native(left).cmp(&Math::from_native(right)) == Ordering::Greater,
        ),
        (true, true) => (
            OrderingOperator::GreaterOrEqual,
            Math::from_native(left).cmp(&Math::from_native(right)) != Ordering::Less,
        ),
    };
    let mut meter = Meter::new(ScalarLimits {
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
    });
    let result = evaluate_native_integer_ordering(
        operator,
        NativeInteger::new(left),
        NativeInteger::new(right),
        &mut meter,
    );
    assert!(
        matches!(result,Outcome::Completed(value) if value==expected),
        "public native ordering refines mathematical order"
    );
    assert_eq!(meter.admission_count(), 3);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    kani::cover!(
        left == i128::MIN && right == i128::MAX && matches!(result, Outcome::Completed(_)),
        "full-width ordering completion witness"
    );
}

/// Every signed32 pair is admitted by the complete supported domain. The
/// unchanged two-limb model is independent of the production signed64 product.
#[kani::proof]
fn native_multiply_signed32_payload_refines_mathematical_integer() {
    let x = i128::from(kani::any::<i32>());
    let y = i128::from(kani::any::<i32>());
    let full = NativeIntegerInterval::new(i32::MIN.into(), i32::MAX.into()).unwrap();
    let plan = NativeIntegerPlan::new(
        NativeIntegerOperation::Multiply {
            left: full,
            right: full,
        },
        None,
    )
    .unwrap();
    assert_eq!(plan.capacity.lower, -4611686016279904256);
    assert_eq!(plan.capacity.upper, 4611686018427387904);
    let admitted = plan
        .admit(NativeIntegerOperands::Binary {
            left: NativeInteger::new(x),
            right: NativeInteger::new(y),
        })
        .unwrap();
    let result = admitted.calculate();
    assert!(
        result.is_some(),
        "every signed32 pair completes production calculation"
    );
    let result = result.unwrap();
    assert_eq!(
        Math::from_native(result.value()),
        Math::evaluate(NativeIntegerOperationKind::Multiply, x, y,),
        "signed64 production product refines independent exact limb mathematics"
    );
    assert!(plan.capacity.contains(result));
    kani::cover!(
        x == i128::from(i32::MIN) && y == i128::from(i32::MIN),
        "MIN squared completion"
    );
    kani::cover!(
        x == i128::from(i32::MIN) && y == i128::from(i32::MAX),
        "negative extreme completion"
    );
    kani::cover!(
        x == 0 && y == i128::from(i32::MIN),
        "zero product completion"
    );
}

// Sorting two unrestricted signed32 endpoints enumerates every nonempty
// supported interval without assuming membership or a capacity theorem.
fn signed32_interval() -> NativeIntegerInterval {
    let a = i128::from(kani::any::<i32>());
    let b = i128::from(kani::any::<i32>());
    NativeIntegerInterval::new(a.min(b), a.max(b)).unwrap()
}

fn semantic_interval() -> Option<NativeIntegerInterval> {
    let a: i128 = kani::any();
    let b: i128 = kani::any();
    if kani::any::<bool>() {
        Some(NativeIntegerInterval::new(a.min(b), a.max(b)).unwrap())
    } else {
        None
    }
}

/// Every supported interval image equals the independent exact four-corner
/// minimum and maximum. Semantic result bounds remain independent signed i128.
#[kani::proof]
fn native_multiply_signed32_corner_image_refines_mathematical_integer() {
    let left = signed32_interval();
    let right = signed32_interval();
    let kind = NativeIntegerOperationKind::Multiply;
    let p = Math::evaluate(kind, left.lower, right.lower);
    let q = Math::evaluate(kind, left.lower, right.upper);
    let r = Math::evaluate(kind, left.upper, right.lower);
    let s = Math::evaluate(kind, left.upper, right.upper);
    let low = p.min(q).min(r).min(s);
    let high = p.max(q).max(r).max(s);
    let semantic = semantic_interval();
    let plan =
        NativeIntegerPlan::new(NativeIntegerOperation::Multiply { left, right }, semantic).unwrap();
    assert_eq!(
        plan.result_domain, semantic,
        "semantic i128 endpoints remain independent"
    );
    assert_eq!(
        Math::from_native(plan.capacity.lower),
        low,
        "exact mathematical lower image endpoint"
    );
    assert_eq!(
        Math::from_native(plan.capacity.upper),
        high,
        "exact mathematical upper image endpoint"
    );
    kani::cover!(
        left.lower == i128::from(i32::MIN) && left.upper == i128::from(i32::MAX) && right == left,
        "full signed32 operand intervals admitted"
    );
    kani::cover!(
        left.lower == 0 && left.upper == 0,
        "zero interval image admitted"
    );
    kani::cover!(
        semantic
            == Some(NativeIntegerInterval {
                lower: i128::MIN,
                upper: i128::MAX
            }),
        "full signed128 semantic domain retained"
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IntervalSign {
    Negative,
    Nonnegative,
    Straddling,
}

fn interval_sign(interval: NativeIntegerInterval) -> IntervalSign {
    if interval.upper < 0 {
        IntervalSign::Negative
    } else if interval.lower >= 0 {
        IntervalSign::Nonnegative
    } else {
        IntervalSign::Straddling
    }
}

/// Independent endpoint predicates check both classifier results and the
/// complete Cartesian partition. All nine cases must later prove enclosure.
#[kani::proof]
fn signed32_interval_sign_pairs_are_exclusive_and_exhaustive() {
    let left = signed32_interval();
    let right = signed32_interval();
    let ln = left.upper < 0;
    let lp = left.lower >= 0;
    let ls = left.lower < 0 && left.upper >= 0;
    let rn = right.upper < 0;
    let rp = right.lower >= 0;
    let rs = right.lower < 0 && right.upper >= 0;
    let lc = interval_sign(left);
    let rc = interval_sign(right);
    assert_eq!(lc == IntervalSign::Negative, ln);
    assert_eq!(lc == IntervalSign::Nonnegative, lp);
    assert_eq!(lc == IntervalSign::Straddling, ls);
    assert_eq!(rc == IntervalSign::Negative, rn);
    assert_eq!(rc == IntervalSign::Nonnegative, rp);
    assert_eq!(rc == IntervalSign::Straddling, rs);
    assert_eq!(u8::from(ln) + u8::from(lp) + u8::from(ls), 1);
    assert_eq!(u8::from(rn) + u8::from(rp) + u8::from(rs), 1);
    assert_eq!(
        u8::from(ln && rn)
            + u8::from(ln && rp)
            + u8::from(ln && rs)
            + u8::from(lp && rn)
            + u8::from(lp && rp)
            + u8::from(lp && rs)
            + u8::from(ls && rn)
            + u8::from(ls && rp)
            + u8::from(ls && rs),
        1,
        "each valid interval pair belongs to exactly one of the nine cases"
    );
    kani::cover!(ln && rn, "negative/negative interval pair");
    kani::cover!(ln && rp, "negative/nonnegative interval pair");
    kani::cover!(ln && rs, "negative/straddling interval pair");
    kani::cover!(lp && rn, "nonnegative/negative interval pair");
    kani::cover!(lp && rp, "nonnegative/nonnegative interval pair");
    kani::cover!(lp && rs, "nonnegative/straddling interval pair");
    kani::cover!(ls && rn, "straddling/negative interval pair");
    kani::cover!(ls && rp, "straddling/nonnegative interval pair");
    kani::cover!(ls && rs, "straddling/straddling interval pair");
}

/// Structural binding for every interval and unrestricted i128 payload: exact
/// membership, successful calculation and equality to the full signed32 plan.
/// Interior enclosure is a separate mandatory sign-partition obligation.
#[kani::proof]
fn native_multiply_signed32_interval_membership_and_calculation() {
    let left = signed32_interval();
    let right = signed32_interval();
    let semantic = semantic_interval();
    let x: i128 = kani::any();
    let y: i128 = kani::any();
    let plan =
        NativeIntegerPlan::new(NativeIntegerOperation::Multiply { left, right }, semantic).unwrap();
    let operands = NativeIntegerOperands::Binary {
        left: NativeInteger::new(x),
        right: NativeInteger::new(y),
    };
    let admitted = plan.admit(operands);
    let membership = left.contains(NativeInteger::new(x)) && right.contains(NativeInteger::new(y));
    assert_eq!(
        admitted.is_ok(),
        membership,
        "exact whole-domain payload membership"
    );
    if let Ok(admitted) = admitted {
        let result = admitted.calculate();
        assert!(
            result.is_some(),
            "every interval-admitted calculation completes"
        );
        let result = result.unwrap();
        let full = NativeIntegerInterval::new(i32::MIN.into(), i32::MAX.into()).unwrap();
        let full_plan = NativeIntegerPlan::new(
            NativeIntegerOperation::Multiply {
                left: full,
                right: full,
            },
            None,
        )
        .unwrap();
        let full_result = full_plan.admit(operands).unwrap().calculate().unwrap();
        assert_eq!(
            result, full_result,
            "same production result as the complete signed32 plan"
        );
        kani::cover!(true, "certified interval calculation completes");
        kani::cover!(
            left.lower < 0 && left.upper > 0 && right.lower < 0 && right.upper > 0,
            "two straddling domains complete"
        );
    }
    kani::cover!(!membership, "outside-interval payload is rejected");
}

/// Only the exhaustive interval-sign case is assumed. Payloads remain full
/// i128 draws and actual kernel admission decides membership. No capacity,
/// overflow or arithmetic identity is assumed. Each of all nine wrappers is
/// required; this helper does not treat a subset as universal refinement.
fn multiply_capacity_in_sign_case(
    left_sign: IntervalSign,
    right_sign: IntervalSign,
) -> Option<(i128, i128)> {
    let left = signed32_interval();
    let right = signed32_interval();
    let semantic = semantic_interval();
    let x: i128 = kani::any();
    let y: i128 = kani::any();
    kani::assume(interval_sign(left) == left_sign && interval_sign(right) == right_sign);
    let plan =
        NativeIntegerPlan::new(NativeIntegerOperation::Multiply { left, right }, semantic).unwrap();
    let admitted = plan.admit(NativeIntegerOperands::Binary {
        left: NativeInteger::new(x),
        right: NativeInteger::new(y),
    });
    if let Ok(admitted) = admitted {
        let result = admitted.calculate();
        assert!(
            result.is_some(),
            "every interval-admitted calculation completes"
        );
        let result = result.unwrap();
        assert!(
            plan.capacity.contains(result),
            "every admitted result belongs to the exact image"
        );
        kani::cover!(
            true,
            "admitted sign-case calculation completes within capacity"
        );
        Some((x, y))
    } else {
        None
    }
}

#[kani::proof]
fn native_multiply_negative_negative_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Negative, IntervalSign::Negative);
}

#[kani::proof]
fn native_multiply_negative_nonnegative_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Negative, IntervalSign::Nonnegative);
}

#[kani::proof]
fn native_multiply_negative_straddling_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Negative, IntervalSign::Straddling);
}

#[kani::proof]
fn native_multiply_nonnegative_negative_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Nonnegative, IntervalSign::Negative);
}

#[kani::proof]
fn native_multiply_nonnegative_nonnegative_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Nonnegative, IntervalSign::Nonnegative);
}

#[kani::proof]
fn native_multiply_nonnegative_straddling_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Nonnegative, IntervalSign::Straddling);
}

#[kani::proof]
fn native_multiply_straddling_negative_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Straddling, IntervalSign::Negative);
}

#[kani::proof]
fn native_multiply_straddling_nonnegative_stays_in_capacity() {
    let _ = multiply_capacity_in_sign_case(IntervalSign::Straddling, IntervalSign::Nonnegative);
}

#[kani::proof]
fn native_multiply_straddling_straddling_stays_in_capacity() {
    let completed =
        multiply_capacity_in_sign_case(IntervalSign::Straddling, IntervalSign::Straddling);
    kani::cover!(
        matches!(completed, Some((x, y)) if x == i128::from(i32::MIN) && y == i128::from(i32::MIN)),
        "both-straddling MIN squared completes within capacity"
    );
    kani::cover!(
        matches!(completed, Some((x, y)) if x == i128::from(i32::MIN) && y == i128::from(i32::MAX)),
        "both-straddling negative extreme completes within capacity"
    );
    kani::cover!(
        matches!(completed, Some((x, y)) if x == 0 && y == i128::from(i32::MIN)),
        "both-straddling zero product completes within capacity"
    );
}

/// The public i128 payload population is not silently replaced by i32 draws:
/// every actual interval-admitted payload roundtrips through signed32 exactly.
#[kani::proof]
fn native_multiply_admitted_payloads_roundtrip_signed32() {
    let left = signed32_interval();
    let right = signed32_interval();
    let semantic = semantic_interval();
    let x: i128 = kani::any();
    let y: i128 = kani::any();
    let plan = NativeIntegerPlan::new(
        NativeIntegerOperation::Multiply { left, right }, semantic,
    ).unwrap();
    let operands = NativeIntegerOperands::Binary {
        left: NativeInteger::new(x), right: NativeInteger::new(y),
    };
    if let Ok(admitted) = plan.admit(operands) {
        assert_eq!(admitted.plan, plan, "admission retains the original checked plan");
        assert_eq!(admitted.operands, operands, "admission retains the original i128 payloads");
        let x32 = i32::try_from(x).unwrap();
        let y32 = i32::try_from(y).unwrap();
        assert_eq!(i128::from(x32), x);
        assert_eq!(i128::from(y32), y);
        kani::cover!(true, "interval-admitted public i128 payloads roundtrip losslessly");
    }
}

/// Bind the actual production calculation to Rust's exact checked wide product.
/// This is a compiler/mathematics bridge, not an independent arithmetic oracle.
#[kani::proof]
fn native_multiply_signed32_matches_checked_wide_product() {
    let x = i128::from(kani::any::<i32>());
    let y = i128::from(kani::any::<i32>());
    let full = NativeIntegerInterval::new(i32::MIN.into(), i32::MAX.into()).unwrap();
    let plan = NativeIntegerPlan::new(
        NativeIntegerOperation::Multiply { left: full, right: full }, None,
    ).unwrap();
    let actual = plan.admit(NativeIntegerOperands::Binary {
        left: NativeInteger::new(x), right: NativeInteger::new(y),
    }).unwrap().calculate();
    let wide = x.checked_mul(y);
    assert!(wide.is_some(), "every signed32 mathematical product fits i128");
    let wide = wide.unwrap();
    let signed64 = i64::try_from(wide).unwrap();
    assert_eq!(i128::from(signed64), wide, "the exact product also roundtrips signed64");
    assert_eq!(actual, Some(NativeInteger::new(wide)));
    kani::cover!(x == i128::from(i32::MIN) && y == i128::from(i32::MIN), "wide bridge MIN squared completion");
    kani::cover!(x == i128::from(i32::MIN) && y == i128::from(i32::MAX), "wide bridge negative extreme completion");
    kani::cover!(x == 0 && y == i128::from(i32::MIN), "wide bridge zero completion");
}

/// Bind actual interval endpoints to four exact wide checked products. No
/// scalar theorem is assumed and no verified stub replaces a production body.
#[kani::proof]
fn native_multiply_capacity_matches_checked_wide_corners() {
    let left = signed32_interval();
    let right = signed32_interval();
    let semantic = semantic_interval();
    let plan = NativeIntegerPlan::new(
        NativeIntegerOperation::Multiply { left, right }, semantic,
    ).unwrap();
    let a = left.lower.checked_mul(right.lower);
    let b = left.lower.checked_mul(right.upper);
    let c = left.upper.checked_mul(right.lower);
    let d = left.upper.checked_mul(right.upper);
    assert!(a.is_some() && b.is_some() && c.is_some() && d.is_some(), "all four exact corner products fit i128");
    let (a, b, c, d) = (a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap());
    assert_eq!(plan.capacity.lower, a.min(b).min(c).min(d));
    assert_eq!(plan.capacity.upper, a.max(b).max(c).max(d));
    assert_eq!(plan.result_domain, semantic, "semantic interval is independent of numerical capacity");
    kani::cover!(left.lower == i128::from(i32::MIN) && left.upper == i128::from(i32::MAX) && left == right, "wide corner bridge full-domain completion");
    kani::cover!(left.lower == 0 && left.upper == 0, "wide corner bridge zero image");
}

/// Bind the real interval method to the mathematical signed-order predicate
/// through Rust's primitive signed i128 comparisons, for every native value.
#[kani::proof]
fn native_interval_contains_matches_signed_wide_order() {
    let a: i128 = kani::any();
    let b: i128 = kani::any();
    let value: i128 = kani::any();
    let lower = a.min(b);
    let upper = a.max(b);
    let interval = NativeIntegerInterval::new(lower, upper).unwrap();
    assert_eq!(interval.lower(), lower);
    assert_eq!(interval.upper(), upper);
    let outside = value < lower || value > upper;
    assert_eq!(interval.contains(NativeInteger::new(value)), !outside);
    kani::cover!(lower == i128::MIN && upper == i128::MAX && value == i128::MIN, "signed-order bridge includes native MIN");
    kani::cover!(value < lower || value > upper, "signed-order bridge rejects outside payload");
}

/// Arbitrary i128 endpoints include every unsupported operand interval, with
/// arbitrary optional semantic bounds. No multiplication theorem is assumed.
#[kani::proof]
fn native_multiply_rejects_every_outside_signed32_interval() {
    let a: i128 = kani::any();
    let b: i128 = kani::any();
    let c: i128 = kani::any();
    let d: i128 = kani::any();
    let left = NativeIntegerInterval::new(a.min(b), a.max(b)).unwrap();
    let right = NativeIntegerInterval::new(c.min(d), c.max(d)).unwrap();
    let e: i128 = kani::any();
    let f: i128 = kani::any();
    let semantic = if kani::any::<bool>() {
        Some(NativeIntegerInterval::new(e.min(f), e.max(f)).unwrap())
    } else {
        None
    };
    let supported = left.lower >= -2147483648
        && left.upper <= 2147483647
        && right.lower >= -2147483648
        && right.upper <= 2147483647;
    let plan = NativeIntegerPlan::new(NativeIntegerOperation::Multiply { left, right }, semantic);
    assert_eq!(
        plan.is_ok(),
        supported,
        "all and only whole signed32 intervals are supported"
    );
    if !supported {
        assert_eq!(
            plan,
            Err(NativeIntegerAdmissionError::CapacityExceeded {
                operation: NativeIntegerOperationKind::Multiply,
            })
        );
    }
    kani::cover!(
        !supported && right.lower == 0 && right.upper == 0 && left.lower == 2147483648,
        "zero opposite operand cannot hide high endpoint"
    );
    kani::cover!(
        !supported
            && right.lower == 0
            && right.upper == 0
            && left.lower == -2147483649
            && left.upper == 0
            && semantic == Some(NativeIntegerInterval { lower: 0, upper: 0 }),
        "zero payload and result domain cannot hide low endpoint"
    );
    kani::cover!(
        !supported && left.lower == 0 && left.upper == 0 && right.upper == 2147483648,
        "right operand capacity checked"
    );
}
