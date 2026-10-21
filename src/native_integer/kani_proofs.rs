// SPDX-License-Identifier: AGPL-3.0-or-later
//! Universal capacity/refinement obligations over the actual native plan and
//! production arithmetic calculation. Meter trace equivalence is a separate
//! real-Meter obligation; this proof does not claim to establish its schedule.
use super::*;
use super::reference::MathematicalInteger as Math;

fn refinement(kind:NativeIntegerOperationKind) {
    let a:i128=kani::any();let b:i128=kani::any();
    let c:i128=kani::any();let d:i128=kani::any();
    let x:i128=kani::any();let y:i128=kani::any();
    let left=NativeIntegerInterval::new(a.min(b),a.max(b)).unwrap();
    let right=NativeIntegerInterval::new(c.min(d),c.max(d)).unwrap();
    let operation=match kind {
        NativeIntegerOperationKind::Add=>NativeIntegerOperation::Add{left,right},
        NativeIntegerOperationKind::Subtract=>NativeIntegerOperation::Subtract{left,right},
        NativeIntegerOperationKind::Multiply=>NativeIntegerOperation::Multiply{left,right},
        NativeIntegerOperationKind::Negate=>NativeIntegerOperation::Negate{operand:left},
    };
    let p=Math::evaluate(kind,left.lower,right.lower);
    let q=Math::evaluate(kind,left.lower,right.upper);
    let r=Math::evaluate(kind,left.upper,right.lower);
    let s=Math::evaluate(kind,left.upper,right.upper);
    let low=p.min(q).min(r).min(s);let high=p.max(q).max(r).max(s);
    let plan=NativeIntegerPlan::new(operation,None);
    assert_eq!(plan.is_ok(),low.fits_native()&&high.fits_native(),"complete mathematical interval capacity");
    let operands=match kind {
        NativeIntegerOperationKind::Negate=>NativeIntegerOperands::Unary{operand:NativeInteger::new(x)},
        NativeIntegerOperationKind::Add|NativeIntegerOperationKind::Subtract|NativeIntegerOperationKind::Multiply=>NativeIntegerOperands::Binary{left:NativeInteger::new(x),right:NativeInteger::new(y)},
    };
    if let Ok(plan)=plan {
        assert_eq!(Math::from_native(plan.capacity.lower),low);
        assert_eq!(Math::from_native(plan.capacity.upper),high);
        let admitted=plan.admit(operands);
        let membership=left.contains(NativeInteger::new(x))&&(kind==NativeIntegerOperationKind::Negate||right.contains(NativeInteger::new(y)));
        assert_eq!(admitted.is_ok(),membership,"exact payload membership");
        kani::cover!(admitted.is_ok(),"admitted operation witness");
        if let Ok(admitted)=admitted {
            let result=admitted.calculate();
            assert!(result.is_some(),"admitted operation cannot overflow");
            let result=result.unwrap();
            assert_eq!(Math::from_native(result.value()),Math::evaluate(kind,x,y),"production calculation refines independent signed-limb mathematics");
            assert!(plan.capacity.contains(result),"complete image contains every admitted result");
        }
    }
}

#[kani::proof]
fn native_add_refines_mathematical_integer() {refinement(NativeIntegerOperationKind::Add)}
#[kani::proof]
fn native_subtract_refines_mathematical_integer() {refinement(NativeIntegerOperationKind::Subtract)}
#[kani::proof]
fn native_multiply_refines_mathematical_integer() {refinement(NativeIntegerOperationKind::Multiply)}
#[kani::proof]
fn native_negate_refines_mathematical_integer() {refinement(NativeIntegerOperationKind::Negate)}
