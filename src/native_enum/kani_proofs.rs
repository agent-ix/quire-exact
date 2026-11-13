// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani harnesses of FR-407 Behavior 9: `compare_native_enum` over
//! symbolic ranks, counts, ordered flag, declaration tags and operator, against
//! an expected side written separately from the comparison body. Meter limits
//! are concrete per harness (sufficient, one-less occurrence, one-less result):
//! a symbolic limit left the outcome variant undecided and CBMC unwound the
//! drop glue of the refusal and incomplete variants without end. Each outcome
//! is held in `ManuallyDrop` and matched by reference: the owner's `Refusal`
//! and `Undefined` variants reach `NodeKey` drop glue, which the proof never
//! needs. No verdict is recorded here (FR-407-AC-9 is planned).
use crate::accounting::{ChargePoint, LimitKind, Meter, ScalarLimits};
use crate::comparison::{ComparisonOperator, IllTypedCause};
use crate::native_enum::{compare_native_enum, NativeEnum};
use crate::outcome::Outcome;
use core::mem::ManuallyDrop;

fn limits(value_occurrences: u64, result_units: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences,
        work_units: u64::MAX,
        result_units,
    }
}

fn symbolic_carrier(declaration: u64, ordered: bool, count: u32) -> NativeEnum {
    let index: u32 = kani::any();
    kani::assume(index < count);
    match NativeEnum::from_index(declaration, ordered, index, count) {
        Ok(carrier) => carrier,
        Err(_) => panic!("an index below the count builds a carrier"),
    }
}

/// A symbolic operator without a loop: each of the six is reachable.
fn symbolic_operator() -> ComparisonOperator {
    match kani::any::<u8>() % 6 {
        0 => ComparisonOperator::Equal,
        1 => ComparisonOperator::NotEqual,
        2 => ComparisonOperator::Less,
        3 => ComparisonOperator::LessOrEqual,
        4 => ComparisonOperator::Greater,
        _ => ComparisonOperator::GreaterOrEqual,
    }
}

fn expected(operator: ComparisonOperator, left: u32, right: u32) -> bool {
    match operator {
        ComparisonOperator::Equal => left == right,
        ComparisonOperator::NotEqual => left != right,
        ComparisonOperator::Less => left < right,
        ComparisonOperator::LessOrEqual => left <= right,
        ComparisonOperator::Greater => left > right,
        ComparisonOperator::GreaterOrEqual => left >= right,
    }
}

/// Two operands of one symbolic declaration, comparable under the operator:
/// an ordering is drawn only for an ordered declaration.
fn comparable_pair() -> (ComparisonOperator, NativeEnum, NativeEnum) {
    let tag: u64 = kani::any();
    let ordered: bool = kani::any();
    let count: u32 = kani::any();
    kani::assume(count > 0);
    let operator = symbolic_operator();
    kani::assume(ordered || !operator.is_ordering());
    (
        operator,
        symbolic_carrier(tag, ordered, count),
        symbolic_carrier(tag, ordered, count),
    )
}

/// A sufficient (exact) meter: every charge is admitted. The meter limits are
/// concrete so the outcome variant is decided by the charges, not by a
/// symbolic limit.
#[kani::proof]
fn native_enum_compare_completes() {
    let (operator, left, right) = comparable_pair();
    let mut meter = Meter::new(limits(2, 1));
    let outcome = ManuallyDrop::new(compare_native_enum(operator, &left, &right, &mut meter));
    match &*outcome {
        Ok(Outcome::Completed(value)) => {
            let value = *value;
            kani::cover!(value, "true witness");
            kani::cover!(!value, "false witness");
            assert_eq!(value, expected(operator, left.index(), right.index()));
        }
        _ => panic!("a sufficient meter completes"),
    }
    assert_eq!(meter.admission_count(), 3);
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
}

/// One less occurrence: the second identity read is denied after one admission.
#[kani::proof]
fn native_enum_compare_denies_second_read() {
    let (operator, left, right) = comparable_pair();
    let mut meter = Meter::new(limits(1, 1));
    let outcome = ManuallyDrop::new(compare_native_enum(operator, &left, &right, &mut meter));
    match &*outcome {
        Ok(Outcome::Incomplete(record)) => {
            kani::cover!(true, "incomplete path");
            assert_eq!(record.charge_point, ChargePoint::EnumIdentityRead);
            assert_eq!(record.limit_kind, LimitKind::ValueOccurrences);
        }
        _ => panic!("one less occurrence is incomplete"),
    }
    assert_eq!(meter.admission_count(), 1);
}

/// One less result: the retain is denied after both reads.
#[kani::proof]
fn native_enum_compare_denies_retain() {
    let (operator, left, right) = comparable_pair();
    let mut meter = Meter::new(limits(2, 0));
    let outcome = ManuallyDrop::new(compare_native_enum(operator, &left, &right, &mut meter));
    match &*outcome {
        Ok(Outcome::Incomplete(record)) => {
            kani::cover!(true, "incomplete path");
            assert_eq!(record.charge_point, ChargePoint::EnumResultRetain);
            assert_eq!(record.limit_kind, LimitKind::ResultUnits);
        }
        _ => panic!("one less result is incomplete"),
    }
    assert_eq!(meter.admission_count(), 2);
}

/// Distinct declarations (any of tag, count or ordered flag differs) are
/// refused first, for every operator, and consume nothing.
#[kani::proof]
fn native_enum_refuses_distinct_declarations() {
    let left_tag: u64 = kani::any();
    let right_tag: u64 = kani::any();
    let left_ordered: bool = kani::any();
    let right_ordered: bool = kani::any();
    let left_count: u32 = kani::any();
    let right_count: u32 = kani::any();
    kani::assume(left_count > 0 && right_count > 0);
    kani::assume(
        left_tag != right_tag || left_count != right_count || left_ordered != right_ordered,
    );
    let left = symbolic_carrier(left_tag, left_ordered, left_count);
    let right = symbolic_carrier(right_tag, right_ordered, right_count);
    let mut meter = Meter::new(limits(2, 1));
    let outcome = ManuallyDrop::new(compare_native_enum(
        symbolic_operator(),
        &left,
        &right,
        &mut meter,
    ));
    match &*outcome {
        Err(error) => {
            kani::cover!(true, "distinct declaration refusal");
            assert_eq!(error.cause, IllTypedCause::DistinctEnumDeclarations);
        }
        Ok(_) => panic!("distinct declarations are refused"),
    }
    assert_eq!(meter.admission_count(), 0);
}

/// An ordering over one unordered declaration is refused and consumes nothing.
#[kani::proof]
fn native_enum_refuses_unordered_ordering() {
    let tag: u64 = kani::any();
    let count: u32 = kani::any();
    kani::assume(count > 0);
    let operator = symbolic_operator();
    kani::assume(operator.is_ordering());
    let left = symbolic_carrier(tag, false, count);
    let right = symbolic_carrier(tag, false, count);
    let mut meter = Meter::new(limits(2, 1));
    let outcome = ManuallyDrop::new(compare_native_enum(operator, &left, &right, &mut meter));
    match &*outcome {
        Err(error) => {
            kani::cover!(true, "unordered refusal");
            assert_eq!(error.cause, IllTypedCause::UnorderedEnumOrdering);
        }
        Ok(_) => panic!("an unordered ordering is refused"),
    }
    assert_eq!(meter.admission_count(), 0);
}
