// SPDX-License-Identifier: AGPL-3.0-or-later
//! The copy footprint of an `Integer` is known before any storage is reserved (FR-390).
//!
//! `allocation_counter` supplies the observer outside this unsafe-free crate.
//! Fixtures are prepared and assertions run outside each measurement window.

use ix_trace_rs::trace;
use quire_exact::{Integer, IntegerCopyFootprint};

fn measure<T>(call: impl FnOnce() -> T) -> (T, allocation_counter::AllocationInfo) {
    let mut outcome = None;
    let info = allocation_counter::measure(|| outcome = Some(call()));
    (outcome.expect("measurement returned"), info)
}

/// Inline controls, both `i128` extremes, the first heap-backed values on each
/// side, and a multi-limb value, with the limbs an exact copy holds.
fn controls() -> Vec<(Integer, usize)> {
    let two_127 = Integer::one().shifted_left(127);
    let one = Integer::one();
    vec![
        (Integer::from(0_i64), 0),
        (Integer::from(-7_i64), 0),
        (Integer::from(i128::MAX), 0),
        (Integer::from(i128::MIN), 0),
        (two_127.clone(), 4),
        (two_127.add(&one).neg(), 4),
        (Integer::one().shifted_left(4096), 129),
        (Integer::one().shifted_left(4096).neg(), 129),
    ]
}

/// A copy of each control, so its lazy `BigInt` image is cold.
fn cold(source: &Integer) -> Integer {
    source.try_clone().expect("fixture copy")
}

#[test]
#[trace("TC-939", "FR-390-AC-6")]
fn footprint_reports_limbs_and_bytes_for_inline_cold_and_heap_backed_values() {
    for (value, limbs) in controls() {
        let expected = IntegerCopyFootprint {
            limbs,
            bytes: limbs * 4,
        };
        assert_eq!(value.copy_footprint(), expected, "{value}");
        assert_eq!(cold(&value).copy_footprint(), expected, "cold {value}");
    }
}

#[test]
#[trace("TC-939", "FR-390-AC-6")]
fn footprint_bytes_equal_what_an_actual_copy_allocates() {
    for (value, _) in controls() {
        let cold_value = cold(&value);
        for source in [&value, &cold_value] {
            let footprint = source.copy_footprint();
            let (copy, observed) = measure(|| source.try_clone());
            assert_eq!(copy.expect("copy"), value);
            assert_eq!(observed.bytes_total, footprint.bytes as u64, "{value}");
            assert_eq!(observed.count_total, u64::from(footprint.limbs != 0));

            // Use a new cold source: the preceding value oracle can materialize
            // its BigInt view, and must not warm the measured infallible copy.
            let clone_source = source.try_clone().expect("fresh clone source");
            let (copy, observed) = measure(|| clone_source.clone());
            assert_eq!(copy, value);
            assert_eq!(observed.bytes_total, footprint.bytes as u64, "{value}");
            assert_eq!(observed.count_total, u64::from(footprint.limbs != 0));

            let (copy, observed) = measure(|| value.clone());
            assert_eq!(copy, value);
            assert_eq!(observed.bytes_total, footprint.bytes as u64, "{value}");
            assert_eq!(observed.count_total, u64::from(footprint.limbs != 0));
        }
    }
}

#[test]
#[trace("TC-939", "FR-390-AC-7")]
fn footprint_and_bit_length_allocate_nothing_for_cold_values() {
    for (value, _) in controls() {
        let cold_value = cold(&value);
        let bits = value.magnitude_bits();
        let ((footprint, cold_bits), observed) =
            measure(|| (cold_value.copy_footprint(), cold_value.magnitude_bits()));
        assert_eq!(observed.count_total, 0, "{value}");
        assert_eq!(observed.bytes_total, 0, "{value}");
        assert_eq!(footprint, value.copy_footprint());
        assert_eq!(cold_bits, bits);
    }
}

#[test]
#[trace("TC-939", "FR-390-AC-7")]
fn bit_length_is_exact_at_the_inline_boundary_and_beyond() {
    let cases = [
        (Integer::from(0_i64), 1),
        (Integer::from(i128::MAX), 127),
        (Integer::from(i128::MIN), 128),
        (Integer::one().shifted_left(127), 128),
        (
            Integer::one().shifted_left(127).add(&Integer::one()).neg(),
            128,
        ),
        (Integer::one().shifted_left(128), 129),
        (Integer::one().shifted_left(4096), 4097),
    ];
    for (value, bits) in cases {
        assert_eq!(value.magnitude_bits(), bits, "{value}");
        assert_eq!(cold(&value).magnitude_bits(), bits, "cold {value}");
    }
}
