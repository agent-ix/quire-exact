// SPDX-License-Identifier: AGPL-3.0-or-later
//! The local probe harness of FR-404 Behavior 9: `compare_native_text` over
//! two symbolic `binary-utf8` payloads of at most the capacity agrees, for
//! all six operators, with the comparison of the byte slices by the
//! language's `==` and `cmp`, and never panics or refuses.
use crate::accounting::{Meter, ScalarLimits};
use crate::comparison::ComparisonOperator;
use crate::native_text::{compare_native_text, NativeText, NATIVE_TEXT_CAPACITY};
use crate::outcome::Outcome;
use crate::text::TextProfile;
use core::cmp::Ordering;

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

fn symbolic_payload() -> ([u8; NATIVE_TEXT_CAPACITY], usize) {
    let bytes: [u8; NATIVE_TEXT_CAPACITY] = kani::any();
    let length: usize = kani::any();
    kani::assume(length <= NATIVE_TEXT_CAPACITY);
    for index in 0..NATIVE_TEXT_CAPACITY {
        kani::assume(index < length || bytes[index] == 0);
    }
    (bytes, length)
}

#[kani::proof]
#[kani::unwind(129)]
fn native_text_compare_matches_slice_order() {
    let (left_bytes, left_length) = symbolic_payload();
    let (right_bytes, right_length) = symbolic_payload();
    let left_slice = &left_bytes[..left_length];
    let right_slice = &right_bytes[..right_length];
    let left = NativeText::zero_padded(TextProfile::BinaryUtf8, left_slice, left_slice);
    let right = NativeText::zero_padded(TextProfile::BinaryUtf8, right_slice, right_slice);
    let ordering = left_slice.cmp(right_slice);
    kani::cover!(ordering == Ordering::Less, "less witness");
    kani::cover!(
        ordering == Ordering::Equal && left_length == NATIVE_TEXT_CAPACITY,
        "full equal witness"
    );
    kani::cover!(ordering == Ordering::Greater, "greater witness");
    for operator in ComparisonOperator::ALL {
        let mut meter = generous_meter();
        let expected = match operator {
            ComparisonOperator::Equal => left_slice == right_slice,
            ComparisonOperator::NotEqual => left_slice != right_slice,
            ComparisonOperator::Less => ordering == Ordering::Less,
            ComparisonOperator::LessOrEqual => ordering != Ordering::Greater,
            ComparisonOperator::Greater => ordering == Ordering::Greater,
            ComparisonOperator::GreaterOrEqual => ordering != Ordering::Less,
        };
        let outcome = compare_native_text(operator, &left, &right, &mut meter);
        match outcome {
            Ok(Outcome::Completed(value)) => assert_eq!(value, expected),
            _ => panic!("every call returns the value variant"),
        }
    }
}
