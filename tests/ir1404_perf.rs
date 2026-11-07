// SPDX-License-Identifier: AGPL-3.0-or-later
//! Timing of the unmetered `Integer` entries against `num-bigint` (FR-396).
//!
//! Ignored by default; run
//! `cargo test --release --test ir1404_perf -- --ignored --nocapture`.
//! Prints nanoseconds per call and the ratio to `num-bigint` for the same
//! operands.

use core::str::FromStr;
use std::hint::black_box;
use std::time::Instant;

use num_bigint::BigInt;
use num_integer::Integer as _;
use quire_exact::{Integer, NonZeroInteger};

fn digits(seed: &mut u64, count: usize) -> String {
    let mut text = String::with_capacity(count);
    for at in 0..count {
        *seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let digit = ((*seed >> 33) % 10) as u8;
        text.push(char::from(
            b'0' + if at == 0 { digit.max(1) } else { digit },
        ));
    }
    text
}

fn nanos(rounds: u32, mut call: impl FnMut()) -> f64 {
    call();
    let start = Instant::now();
    for _ in 0..rounds {
        call();
    }
    start.elapsed().as_nanos() as f64 / f64::from(rounds)
}

fn rounds(count: usize, weight: usize) -> u32 {
    u32::try_from((2_000_000_000_usize / (count * count * weight).max(1)).clamp(3, 200_000))
        .unwrap_or(3)
}

#[test]
#[ignore = "timing harness; prints a table"]
fn unmetered_entries_against_num_bigint() {
    let mut seed = 7_u64;
    println!(
        "{:<10}{:>8}{:>14}{:>14}{:>8}",
        "op", "digits", "quire ns", "bigint ns", "ratio"
    );
    for count in [300_usize, 3_000, 30_000] {
        let (a_text, b_text) = (digits(&mut seed, count), digits(&mut seed, count));
        let (a, b) = (
            Integer::from_str(&a_text).unwrap(),
            Integer::from_str(&b_text).unwrap(),
        );
        let (a_big, b_big) = (
            BigInt::from_str(&a_text).unwrap(),
            BigInt::from_str(&b_text).unwrap(),
        );
        let product = a.mul(&b);
        let product_big = &a_big * &b_big;
        let token = NonZeroInteger::try_from(&b).unwrap();
        let row = |op: &str, ours: f64, theirs: f64| {
            println!(
                "{op:<10}{count:>8}{ours:>14.0}{theirs:>14.0}{:>8.2}",
                ours / theirs
            );
        };
        let n = rounds(count, 1);
        row(
            "mul",
            nanos(n, || drop(black_box(black_box(&a).mul(black_box(&b))))),
            nanos(n, || drop(black_box(black_box(&a_big) * black_box(&b_big)))),
        );
        row(
            "exact_div",
            nanos(n, || drop(black_box(black_box(&product).exact_div(token)))),
            nanos(n, || {
                drop(black_box(black_box(&product_big) / black_box(&b_big)))
            }),
        );
        row(
            "parse",
            nanos(n, || {
                drop(black_box(Integer::from_str(black_box(&a_text)).unwrap()))
            }),
            nanos(n, || {
                drop(black_box(BigInt::from_str(black_box(&a_text)).unwrap()))
            }),
        );
        if count <= 3_000 {
            let n = rounds(count, 8);
            row(
                "gcd",
                nanos(n, || drop(black_box(black_box(&a).gcd(black_box(&b))))),
                nanos(n, || {
                    drop(black_box(black_box(&a_big).gcd(black_box(&b_big))))
                }),
            );
        }
    }
}
