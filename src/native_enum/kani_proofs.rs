// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani harnesses of FR-407 Behavior 9: `compare_native_enum` over
//! symbolic ranks, counts, ordered flag, declaration tags and meter limits,
//! for all six operators, against an expected side written separately from
//! the comparison body. Written only: not compiled or run, and no verdict is
//! recorded (FR-407-AC-9 is planned).
use crate::accounting::{Meter, ScalarLimits};
use crate::comparison::{ComparisonOperator, IllTypedCause};
use crate::native_enum::{compare_native_enum, NativeEnum};
use crate::outcome::Outcome;

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

/// One shared declaration, symbolic meter limits: the Completed path and the
/// incomplete path of each charge, with the admission count.
#[kani::proof]
fn native_enum_compare_same_declaration() {
    let tag: u64 = kani::any();
    let ordered: bool = kani::any();
    let count: u32 = kani::any();
    kani::assume(count > 0);
    let left = symbolic_carrier(tag, ordered, count);
    let right = symbolic_carrier(tag, ordered, count);
    let occurrences: u64 = kani::any();
    let results: u64 = kani::any();
    for operator in ComparisonOperator::ALL {
        let mut meter = Meter::new(limits(occurrences, results));
        let outcome = compare_native_enum(operator, &left, &right, &mut meter);
        if operator.is_ordering() && !ordered {
            kani::cover!(true, "unordered ordering refusal");
            match outcome {
                Err(error) => {
                    assert_eq!(error.cause, IllTypedCause::UnorderedEnumOrdering)
                }
                Ok(_) => panic!("an unordered ordering is refused"),
            }
            assert_eq!(meter.admission_count(), 0);
            continue;
        }
        let sufficient = occurrences >= 2 && results >= 1;
        match outcome {
            Ok(Outcome::Completed(value)) => {
                kani::cover!(true, "completed path");
                assert!(sufficient);
                assert_eq!(value, expected(operator, left.index(), right.index()));
                assert_eq!(meter.admission_count(), 3);
            }
            Ok(Outcome::Incomplete(_)) => {
                kani::cover!(true, "incomplete path");
                assert!(!sufficient);
                let admitted = if occurrences < 1 {
                    0
                } else if occurrences < 2 {
                    1
                } else {
                    2
                };
                assert_eq!(meter.admission_count(), admitted);
            }
            _ => panic!("only Completed or Incomplete follows admission"),
        }
    }
}

/// Two symbolic declarations: refused as distinct exactly when any of tag,
/// count or ordered flag differs, before the unordered check, consuming
/// nothing.
#[kani::proof]
fn native_enum_compare_distinct_declarations() {
    let left_tag: u64 = kani::any();
    let right_tag: u64 = kani::any();
    let left_ordered: bool = kani::any();
    let right_ordered: bool = kani::any();
    let left_count: u32 = kani::any();
    let right_count: u32 = kani::any();
    kani::assume(left_count > 0 && right_count > 0);
    let left = symbolic_carrier(left_tag, left_ordered, left_count);
    let right = symbolic_carrier(right_tag, right_ordered, right_count);
    let distinct =
        left_tag != right_tag || left_count != right_count || left_ordered != right_ordered;
    let operator: ComparisonOperator = match kani::any::<u8>() % 6 {
        0 => ComparisonOperator::Equal,
        1 => ComparisonOperator::NotEqual,
        2 => ComparisonOperator::Less,
        3 => ComparisonOperator::LessOrEqual,
        4 => ComparisonOperator::Greater,
        _ => ComparisonOperator::GreaterOrEqual,
    };
    let mut meter = Meter::new(limits(u64::MAX, u64::MAX));
    let outcome = compare_native_enum(operator, &left, &right, &mut meter);
    if distinct {
        kani::cover!(true, "distinct declaration refusal");
        match outcome {
            Err(error) => assert_eq!(error.cause, IllTypedCause::DistinctEnumDeclarations),
            Ok(_) => panic!("distinct declarations are refused"),
        }
        assert_eq!(meter.admission_count(), 0);
    } else {
        kani::cover!(true, "same declaration reaches the comparison");
        assert!(outcome.is_ok() || (operator.is_ordering() && !left_ordered));
    }
}
