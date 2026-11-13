// SPDX-License-Identifier: AGPL-3.0-or-later
//! Loop-trip counters of the proof-path normalization (FR-401 Behavior 6,
//! AC-9), compiled for `cfg(test)` only. `entry` marks a loop entry and
//! `hit` one trip; the recorded maximum of a site is its trips in one entry.

use core::cell::Cell;

pub(super) const DECODE: usize = 0;
pub(super) const COPY: usize = 1;
pub(super) const CANON_SEARCH: usize = 2;
pub(super) const COMPAT_SEARCH: usize = 3;
pub(super) const CCC_SEARCH: usize = 4;
pub(super) const SORT_OUTER: usize = 5;
pub(super) const SORT_INNER: usize = 6;
pub(super) const COMPOSE: usize = 7;
pub(super) const COMPOSE_SEARCH: usize = 8;
pub(super) const ENCODE: usize = 9;
/// The number of loop sites.
pub(super) const SITES: usize = 10;

/// Each site with the name of the bound its loop header declares.
pub(super) const SITE_LIST: [(&str, &str); SITES] = [
    ("DECODE", "N"),
    ("COPY", "D"),
    ("CANON_SEARCH", "CANON_STEPS"),
    ("COMPAT_SEARCH", "COMPAT_STEPS"),
    ("CCC_SEARCH", "CCC_STEPS"),
    ("SORT_OUTER", "D"),
    ("SORT_INNER", "D"),
    ("COMPOSE", "D"),
    ("COMPOSE_SEARCH", "COMPOSE_STEPS"),
    ("ENCODE", "D"),
];

std::thread_local! {
    static CURRENT: Cell<[usize; SITES]> = const { Cell::new([0; SITES]) };
    static LARGEST: Cell<[usize; SITES]> = const { Cell::new([0; SITES]) };
}

/// Forget every count.
pub(super) fn reset() {
    CURRENT.with(|cell| cell.set([0; SITES]));
    LARGEST.with(|cell| cell.set([0; SITES]));
}

/// A loop at `site` is entered.
pub(super) fn enter(site: usize) {
    CURRENT.with(|cell| {
        let mut counts = cell.get();
        counts[site] = 0;
        cell.set(counts);
    });
}

/// One trip of the loop at `site`.
pub(super) fn hit(site: usize) {
    let now = CURRENT.with(|cell| {
        let mut counts = cell.get();
        counts[site] += 1;
        cell.set(counts);
        counts[site]
    });
    LARGEST.with(|cell| {
        let mut largest = cell.get();
        largest[site] = largest[site].max(now);
        cell.set(largest);
    });
}

/// The most trips of `site` in one entry since the last reset.
pub(super) fn largest(site: usize) -> usize {
    LARGEST.with(|cell| cell.get()[site])
}
