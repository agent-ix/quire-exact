// SPDX-License-Identifier: AGPL-3.0-or-later
//! The generator of `src/native_text/normalize/tables.rs` (FR-401 Behavior 2).
//! It enumerates every Unicode scalar through the public API of the
//! `unicode-normalization` dependency. The default test run only compares
//! the checked-in file with an in-memory regeneration; setting
//! `QUIRE_EXACT_GENERATE_TABLES` rewrites the file.
use std::fmt::Write;
use unicode_normalization::char::{
    canonical_combining_class, compose, decompose_canonical, decompose_compatible,
};

pub fn scalars() -> impl Iterator<Item = char> {
    (0..=0x10_FFFFu32).filter_map(char::from_u32)
}

pub fn is_hangul_syllable(c: char) -> bool {
    ('\u{AC00}'..='\u{D7A3}').contains(&c)
}

pub fn canonical_of(c: char) -> Vec<char> {
    let mut out = Vec::new();
    decompose_canonical(c, |d| out.push(d));
    out
}

pub fn compatible_of(c: char) -> Vec<char> {
    let mut out = Vec::new();
    decompose_compatible(c, |d| out.push(d));
    out
}

pub struct Tables {
    pub canon: Vec<(u32, Vec<u32>)>,
    pub compat: Vec<(u32, Vec<u32>)>,
    pub ccc: Vec<(u32, u32, u8)>,
    pub compose: Vec<(u32, u32, u32)>,
}

pub fn build() -> Tables {
    let mut canon = Vec::new();
    let mut compat = Vec::new();
    let mut ccc: Vec<(u32, u32, u8)> = Vec::new();
    let mut triples = std::collections::BTreeSet::new();
    for c in scalars() {
        let class = canonical_combining_class(c);
        if class != 0 {
            match ccc.last_mut() {
                Some(last) if last.1 + 1 == c as u32 && last.2 == class => last.1 = c as u32,
                _ => ccc.push((c as u32, c as u32, class)),
            }
        }
        if is_hangul_syllable(c) {
            continue;
        }
        let canonical = canonical_of(c);
        let compatible = compatible_of(c);
        if canonical != [c] {
            canon.push((c as u32, canonical.iter().map(|d| *d as u32).collect()));
            let mut state = canonical[0];
            for next in &canonical[1..] {
                match compose(state, *next) {
                    Some(composite) => {
                        triples.insert((state as u32, *next as u32, composite as u32));
                        state = composite;
                    }
                    None => break,
                }
            }
        }
        if compatible != canonical {
            compat.push((c as u32, compatible.iter().map(|d| *d as u32).collect()));
        }
    }
    Tables {
        canon,
        compat,
        ccc,
        compose: triples
            .into_iter()
            .filter(|(l, r, _)| !hangul_pair(*l, *r))
            .collect(),
    }
}

fn hangul_pair(l: u32, r: u32) -> bool {
    ((0x1100..0x1113).contains(&l) && (0x1161..0x1176).contains(&r))
        || ((0xAC00..0xD7A4).contains(&l) && (0x11A8..0x11C3).contains(&r))
}

fn steps(count: usize) -> usize {
    let mut steps = 0;
    while (1usize << steps) <= count {
        steps += 1;
    }
    steps
}

fn array<T: std::fmt::Display>(out: &mut String, name: &str, ty: &str, count: &str, items: &[T]) {
    writeln!(out, "pub(super) static {name}: [{ty}; {count}] = [").unwrap();
    for chunk in items.chunks(12) {
        let line: Vec<String> = chunk.iter().map(ToString::to_string).collect();
        writeln!(out, "    {},", line.join(", ")).unwrap();
    }
    writeln!(out, "];").unwrap();
}

fn decomposition_table(out: &mut String, prefix: &str, rows: &[(u32, Vec<u32>)]) {
    let mut starts = Vec::new();
    let mut lens = Vec::new();
    let mut pool = Vec::new();
    for (_, scalars) in rows {
        starts.push(pool.len());
        lens.push(scalars.len());
        pool.extend(scalars.iter().copied());
    }
    let keys: Vec<u32> = rows.iter().map(|(key, _)| *key).collect();
    writeln!(
        out,
        "pub(super) const {prefix}_COUNT: usize = {};",
        rows.len()
    )
    .unwrap();
    writeln!(
        out,
        "pub(super) const {prefix}_POOL_COUNT: usize = {};",
        pool.len()
    )
    .unwrap();
    writeln!(
        out,
        "pub(super) const {prefix}_STEPS: usize = {};",
        steps(rows.len())
    )
    .unwrap();
    array(
        out,
        &format!("{prefix}_KEYS"),
        "u32",
        &format!("{prefix}_COUNT"),
        &keys,
    );
    array(
        out,
        &format!("{prefix}_START"),
        "u16",
        &format!("{prefix}_COUNT"),
        &starts,
    );
    array(
        out,
        &format!("{prefix}_LEN"),
        "u8",
        &format!("{prefix}_COUNT"),
        &lens,
    );
    array(
        out,
        &format!("{prefix}_POOL"),
        "u32",
        &format!("{prefix}_POOL_COUNT"),
        &pool,
    );
    let classes: Vec<u8> = pool
        .iter()
        .map(|scalar| canonical_combining_class(char::from_u32(*scalar).unwrap()))
        .collect();
    array(
        out,
        &format!("{prefix}_POOL_CLASS"),
        "u8",
        &format!("{prefix}_POOL_COUNT"),
        &classes,
    );
}

pub fn render() -> String {
    let tables = build();
    let mut out = String::new();
    out.push_str(
        "// SPDX-License-Identifier: AGPL-3.0-or-later\n\
         //! Generated by `tests/ir1501_normalize/generator.rs` from the public API of\n\
         //! `unicode-normalization` (Unicode 17.0.0). Do not edit by hand: set\n\
         //! `QUIRE_EXACT_GENERATE_TABLES` and run the `ir1501_normalize` test to\n\
         //! regenerate. The default test run compares this file with an in-memory\n\
         //! regeneration and checks every entry against that API.\n\n\
         #![cfg_attr(rustfmt, rustfmt::skip)]\n\n",
    );
    decomposition_table(&mut out, "CANON", &tables.canon);
    decomposition_table(&mut out, "COMPAT", &tables.compat);
    let ccc_start: Vec<u32> = tables.ccc.iter().map(|r| r.0).collect();
    let ccc_end: Vec<u32> = tables.ccc.iter().map(|r| r.1).collect();
    let ccc_class: Vec<u8> = tables.ccc.iter().map(|r| r.2).collect();
    writeln!(
        out,
        "pub(super) const CCC_COUNT: usize = {};",
        tables.ccc.len()
    )
    .unwrap();
    writeln!(
        out,
        "pub(super) const CCC_STEPS: usize = {};",
        steps(tables.ccc.len())
    )
    .unwrap();
    array(&mut out, "CCC_START", "u32", "CCC_COUNT", &ccc_start);
    array(&mut out, "CCC_END", "u32", "CCC_COUNT", &ccc_end);
    array(&mut out, "CCC_CLASS", "u8", "CCC_COUNT", &ccc_class);
    let l: Vec<u32> = tables.compose.iter().map(|r| r.0).collect();
    let r: Vec<u32> = tables.compose.iter().map(|r| r.1).collect();
    let c: Vec<u32> = tables.compose.iter().map(|r| r.2).collect();
    writeln!(
        out,
        "pub(super) const COMPOSE_COUNT: usize = {};",
        tables.compose.len()
    )
    .unwrap();
    writeln!(
        out,
        "pub(super) const COMPOSE_STEPS: usize = {};",
        steps(tables.compose.len())
    )
    .unwrap();
    array(&mut out, "COMPOSE_LEFT", "u32", "COMPOSE_COUNT", &l);
    array(&mut out, "COMPOSE_RIGHT", "u32", "COMPOSE_COUNT", &r);
    array(&mut out, "COMPOSE_RESULT", "u32", "COMPOSE_COUNT", &c);
    out
}
