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
fn native_multiply_refines_mathematical_integer() {
    refinement(NativeIntegerOperationKind::Multiply)
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
