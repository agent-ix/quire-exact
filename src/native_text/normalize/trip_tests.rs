// SPDX-License-Identifier: AGPL-3.0-or-later
//! The loop-bound measurement of FR-401-AC-9: each loop site counts its
//! trips, and the recorded maximum of one entry is at most its declared bound
//! and equal to it for the sites bounded by N, D and R.
use super::{normalize_text, tables, trips, unwind_bound};
use crate::text::NormalizationForm;
use ix_trace_rs::trace;

const FORMS: [NormalizationForm; 4] = [
    NormalizationForm::Nfc,
    NormalizationForm::Nfd,
    NormalizationForm::Nfkc,
    NormalizationForm::Nfkd,
];

/// Names of the module constants a site can declare, with their values.
fn declared<const N: usize, const D: usize, const R: usize>(name: &str) -> usize {
    match name {
        "N" => N,
        "D" => D,
        "R" => R,
        "CANON_STEPS" => tables::CANON_STEPS,
        "COMPAT_STEPS" => tables::COMPAT_STEPS,
        "CCC_STEPS" => tables::CCC_STEPS,
        "COMPOSE_STEPS" => tables::COMPOSE_STEPS,
        other => panic!("unknown bound {other}"),
    }
}

fn measure<const N: usize, const D: usize, const R: usize>() {
    // N bytes of four-byte scalars fill the payload, the output and the
    // intermediate buffer (N / 4 scalars of D = N / 2).
    let mut payload = [0u8; N];
    for chunk in 0..N / 4 {
        payload[4 * chunk..4 * chunk + 4].copy_from_slice("\u{1F600}".as_bytes());
    }
    let mut seen = [0usize; trips::SITES];
    let mut entry_payload = [0u8; N];
    entry_payload[..3].copy_from_slice("\u{1F82}".as_bytes());
    for form in FORMS {
        trips::reset();
        let mut output = [0u8; R];
        let size = normalize_text::<N, D, R>(form, &payload, N, &mut output).expect("fits");
        assert_eq!((size.scalars, size.bytes), (N / 4, N));
        for (site, _) in trips::SITE_LIST.iter().enumerate() {
            seen[site] = seen[site].max(trips::largest(site));
        }
        trips::reset();
        normalize_text::<N, D, R>(form, &entry_payload, 3, &mut output).expect("fits");
        for (site, (_, bound)) in trips::SITE_LIST.iter().enumerate() {
            seen[site] = seen[site].max(trips::largest(site));
            let limit = declared::<N, D, R>(bound);
            assert!(trips::largest(site) <= limit);
        }
    }
    // The longest table entry (U+FDFA, 18 scalars) in a buffer of D of 18.
    for (site, (name, bound)) in trips::SITE_LIST.iter().enumerate() {
        let limit = declared::<N, D, R>(bound);
        if matches!(*bound, "N" | "D" | "R") {
            assert_eq!(seen[site], limit, "{name} runs its whole bound");
        } else {
            assert!(seen[site] <= limit);
        }
    }
    for bound in [
        tables::CANON_STEPS,
        tables::COMPAT_STEPS,
        tables::CCC_STEPS,
        tables::COMPOSE_STEPS,
    ] {
        assert!(bound < 16);
    }
    assert_eq!(unwind_bound::<N, D, R>(), N.max(D).max(R) + 1);
}

#[test]
#[trace("TC-949", "FR-401-AC-9")]
fn loop_trips_equal_their_declared_bounds() {
    measure::<64, 32, 64>();
    measure::<16, 8, 16>();
    assert_ne!(unwind_bound::<64, 32, 64>(), unwind_bound::<16, 8, 16>());
}

#[test]
#[trace("TC-949", "FR-401-AC-9")]
fn the_longest_entry_fills_the_intermediate_buffer() {
    let mut payload = [0u8; 64];
    payload[..3].copy_from_slice("\u{FDFA}".as_bytes());
    trips::reset();
    let mut output = [0u8; 64];
    let size = normalize_text::<64, 18, 64>(NormalizationForm::Nfkd, &payload, 3, &mut output)
        .expect("fits at D of 18");
    assert_eq!(size.scalars, 18);
    assert_eq!(trips::largest(trips::COPY), 18);
    assert_eq!(trips::largest(trips::ENCODE), 18);
}
