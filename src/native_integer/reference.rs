// SPDX-License-Identifier: AGPL-3.0-or-later
//! Independent test/proof abstraction: signed magnitude with two u128 limbs.
//! No production arithmetic, native plan transfer, or BigInt continuation is
//! used here. Host tests bind this bounded mathematical representation to BigInt.

use core::cmp::Ordering;
use super::NativeIntegerOperationKind as Kind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MathematicalInteger { negative: bool, high: u128, low: u128 }

impl MathematicalInteger {
    pub(super) fn from_native(value: i128) -> Self {
        let bits=u128::from_le_bytes(value.to_le_bytes());
        Self { negative:value<0, high:0, low:if value<0 { (!bits).wrapping_add(1) } else {bits} }
    }
    pub(super) fn evaluate(kind:Kind,a:i128,b:i128)->Self {
        let a=Self::from_native(a);let mut b=Self::from_native(b);
        match kind {
            Kind::Negate=>Self {negative: !a.negative && a.low!=0,..a},
            Kind::Add|Kind::Subtract=>{
                if kind==Kind::Subtract && b.low!=0 {b.negative=!b.negative;}
                if a.negative==b.negative {
                    let (low,carry)=a.low.overflowing_add(b.low);
                    Self{negative:a.negative,high:u128::from(carry),low}
                } else if a.low>=b.low {Self{negative:a.negative && a.low!=b.low,high:0,low:a.low-b.low}}
                else {Self{negative:b.negative,high:0,low:b.low-a.low}}
            }
            Kind::Multiply=>{
                // Base 2^64 schoolbook multiplication. Every partial product
                // fits u128; two low-limb carries contribute to the high limb.
                let mask=u128::from(u64::MAX);
                let a0=a.low&mask;let a1=a.low>>64;
                let b0=b.low&mask;let b1=b.low>>64;
                let p0=a0*b0;let p1=a1*b0;let p2=a0*b1;let p3=a1*b1;
                let (low,c1)=p0.overflowing_add((p1&mask)<<64);
                let (low,c2)=low.overflowing_add((p2&mask)<<64);
                let high=p3+(p1>>64)+(p2>>64)+u128::from(c1)+u128::from(c2);
                Self{negative:(a.negative!=b.negative)&&(high!=0||low!=0),high,low}
            }
        }
    }
    pub(super) fn fits_native(self)->bool {self.high==0 && self.low<=if self.negative {1_u128<<127}else{(1_u128<<127)-1}}
    #[cfg(test)]
    fn to_big(self)->num_bigint::BigInt {
        let magnitude=(num_bigint::BigInt::from(self.high)<<128_u32)+num_bigint::BigInt::from(self.low);
        if self.negative {-magnitude}else{magnitude}
    }
}
impl Ord for MathematicalInteger {
    fn cmp(&self,other:&Self)->Ordering {
        match (self.negative,other.negative) {
            (true,false)=>Ordering::Less,(false,true)=>Ordering::Greater,
            (negative,_)=>{let magnitude=(self.high,self.low).cmp(&(other.high,other.low));if negative {magnitude.reverse()}else{magnitude}}
        }
    }
}
impl PartialOrd for MathematicalInteger {fn partial_cmp(&self,other:&Self)->Option<Ordering>{Some(self.cmp(other))}}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use num_bigint::BigInt;
    use num_traits::ToPrimitive;

    #[test]
    #[trace("FR-384-AC-2")]
    fn signed_two_limb_abstraction_agrees_with_independent_bigint() {
        let mut values=std::vec![i128::MIN,i128::MAX,0,1,-1,i64::MIN.into(),i64::MAX.into(),1_i128<<64,-(1_i128<<64)];
        let mut state=0xf3ab_45de_7319_c067_91d5_8fa2_b5c3_7691_u128;
        for _ in 0..64 {state^=state<<13;state^=state>>7;state^=state<<17;values.push(i128::from_le_bytes(state.to_le_bytes()));}
        for &a in &values {for &b in &values {for kind in [Kind::Add,Kind::Subtract,Kind::Multiply,Kind::Negate] {
            let aa=BigInt::from(a);let bb=BigInt::from(b);
            let expected=match kind {Kind::Add=>&aa+&bb,Kind::Subtract=>&aa-&bb,Kind::Multiply=>&aa*&bb,Kind::Negate=>-&aa};
            let reference=MathematicalInteger::evaluate(kind,a,b);
            assert_eq!(reference.to_big(),expected);
            assert_eq!(reference.fits_native(),expected.to_i128().is_some());
            assert_eq!(MathematicalInteger::from_native(a).cmp(&MathematicalInteger::from_native(b)),aa.cmp(&bb));
        }}}
    }
}
