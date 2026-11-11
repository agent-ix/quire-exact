// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native Decimal carrier against literals and the owner route (FR-402,
//! TC-950).
use ix_trace_rs::trace;
use quire_exact::{
    Decimal, DecimalRepresentation, DecimalType, IllTypedCause, Integer, NativeDecimal as D,
    NativeDecimalRefusal as Refusal, NativeDecimalType as T, RoundingMode,
};

type Row = (i128, i128, u64, u64, &'static [(i64, i64)]);
const MAX: i128 = i128::MAX;
const MIN: i128 = i128::MIN;
const I64_LO: i128 = i64::MIN as i128;
const I64_HI: i128 = i64::MAX as i128;

/// The boundary set of TC-950, one row per type, written as independent
/// literals. Every test that walks the boundary set reads this table.
const BOUNDARY: [Row; 14] = [
    (
        0,
        100,
        0,
        2,
        &[
            (5000, 2),
            (10, 3),
            (1, 3),
            (10001, 2),
            (9999, 2),
            (100, 2),
            (1, 0),
            (1, 4294967295),
            (10, 4294967295),
        ],
    ),
    (
        -1000,
        1000,
        0,
        2,
        &[
            (0, 7),
            (1000, 0),
            (-1000, 0),
            (1001, 0),
            (-1001, 0),
            (1, 4294967296),
            (i64::MIN, 0),
            (i64::MAX, 0),
        ],
    ),
    (I64_LO, I64_HI, 0, 0, &[(i64::MIN, 0), (i64::MAX, 0)]),
    (0, 100, 2, 3, &[(1, 0), (2, 0)]),
    (0, MAX, 38, 38, &[(1, 0), (2, 0)]),
    (0, MAX, 39, 39, &[(1, 0), (0, 0)]),
    (MIN, 0, 38, 38, &[(-1, 0), (-2, 0)]),
    (0, 100, 60, 70, &[(1, 0), (-1, 0), (0, 0)]),
    (-100, 0, 60, 70, &[(-1, 0)]),
    (0, 100, 128, 130, &[(1, 0)]),
    (
        0,
        1,
        0,
        4294967295,
        &[(1, 4294967295), (0, 4294967295), (2, 4294967295), (1, -1)],
    ),
    (0, 1, 0, 0, &[(0, 0), (1, 0)]),
    (0, 100, 0, 1, &[(100, 2), (10, 1)]),
    (MIN, MAX, 0, 4294967295, &[(i64::MIN, 4294967295), (1, 0)]),
];

/// Every normalized-view input of TC-950 with its expected literal.
const NORMALIZED: [((i128, u32), (i128, u32)); 14] = [
    ((0, 7), (0, 0)),
    ((-1000, 3), (-1, 0)),
    ((120, 5), (12, 4)),
    ((7, 3), (7, 3)),
    ((1_000_000_000_000_000_000, 18), (1, 0)),
    ((1_000_000_000_000_000_000, 19), (1, 1)),
    ((1_000_000_000_000_000_000, 5), (10_000_000_000_000, 0)),
    ((9_000_000_000_000_000_000, 20), (9, 2)),
    ((100, 2), (1, 0)),
    ((10_i128.pow(37), 40), (1, 3)),
    ((-(10_i128.pow(38)), 38), (-1, 0)),
    ((MIN, 0), (MIN, 0)),
    ((MIN, u32::MAX), (MIN, u32::MAX)),
    ((10_i128.pow(38), u32::MAX), (1, 4294967257)),
];

const OUT: Result<D, Refusal> = Err(Refusal::OutsideCarrierDomain);

fn ty(lo: i128, hi: i128, smin: u64, smax: u64) -> T {
    T::new(lo, hi, smin, smax, RoundingMode::Exact).unwrap()
}
fn ok(c: i128, s: u32) -> Result<D, Refusal> {
    Ok(D::from_retained(c, s))
}
fn build(t: &T, c: i64, s: i64) -> Result<D, Refusal> {
    D::from_bindings(t, c, s)
}
fn check(t: &T, cases: &[(i64, i64, Result<D, Refusal>)]) {
    for (c, s, expected) in cases {
        assert_eq!(build(t, *c, *s), *expected, "({c}, {s})");
    }
}

#[test]
#[trace("TC-950", "FR-402-AC-1")]
fn type_constructor_and_ingress_agree_with_the_owner() {
    for (lo, hi, smin, smax) in [(1, 0, 0, 2), (0, 1, 3, 2), (0, 1, 0, 4294967296)] {
        assert_eq!(
            T::new(lo, hi, smin, smax, RoundingMode::Exact)
                .unwrap_err()
                .cause,
            IllTypedCause::MalformedDecimalType
        );
        assert!(DecimalType::new(
            Integer::from(lo),
            Integer::from(hi),
            smin,
            smax,
            RoundingMode::Exact
        )
        .is_err());
    }
    assert!(T::new(i128::MIN, i128::MAX, 0, 4294967295, RoundingMode::Exact).is_ok());
    assert!(T::new(0, 0, 0, 0, RoundingMode::Exact).is_ok());
    let two127 = Integer::from(i128::MAX).add(&Integer::one());
    let owner =
        |lo: Integer, hi: Integer| DecimalType::new(lo, hi, 0, 2, RoundingMode::Exact).unwrap();
    let wide = [
        owner(Integer::zero(), two127.clone()),
        owner(two127.neg().sub(&Integer::one()), Integer::zero()),
        owner(
            Integer::zero(),
            two127.mul(&Integer::from(2_i64)).add(&Integer::one()),
        ),
    ];
    for t in &wide {
        assert_eq!(T::try_from(t), Err(Refusal::TypeNotRepresentable));
    }
    let edge = owner(Integer::from(i128::MIN), Integer::from(i128::MAX));
    let native = T::try_from(&edge).unwrap();
    assert_eq!((native.lower(), native.upper()), (i128::MIN, i128::MAX));
    assert_eq!((native.min_scale(), native.max_scale()), (0, 2));
}

#[test]
#[trace("TC-950", "FR-402-AC-2")]
fn retention_and_normalized_membership() {
    let t = ty(0, 100, 0, 2);
    check(
        &t,
        &[
            (5000, 2, ok(5000, 2)),
            (10, 3, ok(10, 3)),
            (1, 3, OUT),
            (10001, 2, OUT),
            (9999, 2, OUT),
        ],
    );
    let t = ty(-1000, 1000, 0, 2);
    check(
        &t,
        &[
            (0, 7, ok(0, 7)),
            (-1000, 0, ok(-1000, 0)),
            (1000, 0, ok(1000, 0)),
            (-1001, 0, OUT),
            (1001, 0, OUT),
            (i64::MIN, 0, OUT),
            (i64::MAX, 0, OUT),
        ],
    );
    let t = ty(i128::from(i64::MIN), i128::from(i64::MAX), 0, 0);
    check(
        &t,
        &[
            (i64::MIN, 0, ok(i128::from(i64::MIN), 0)),
            (i64::MAX, 0, ok(i128::from(i64::MAX), 0)),
        ],
    );
}

#[test]
#[trace("TC-950", "FR-402-AC-3")]
fn lift_and_its_overflow() {
    check(&ty(0, 100, 2, 3), &[(1, 0, ok(1, 0)), (2, 0, OUT)]);
    check(&ty(0, i128::MAX, 38, 38), &[(1, 0, ok(1, 0)), (2, 0, OUT)]);
    check(&ty(0, i128::MAX, 39, 39), &[(1, 0, OUT), (0, 0, ok(0, 0))]);
    check(&ty(0, 100, 128, 130), &[(1, 0, OUT)]);
    check(&ty(-100, 0, 60, 70), &[(-1, 0, OUT)]);
    check(
        &ty(0, 100, 60, 70),
        &[(1, 0, OUT), (-1, 0, OUT), (0, 0, ok(0, 0))],
    );
    check(
        &ty(i128::MIN, 0, 38, 38),
        &[(-1, 0, ok(-1, 0)), (-2, 0, OUT)],
    );
    check(&ty(0, 100, 0, 2), &[(9999, 2, OUT)]);
}

#[test]
#[trace("TC-950", "FR-402-AC-4")]
fn scale_range_and_maximum_scale() {
    check(&ty(-1000, 1000, 0, 2), &[(1, 4294967296, OUT)]);
    check(
        &ty(0, 1, 0, 4294967295),
        &[
            (1, -1, OUT),
            (1, 4294967295, ok(1, 4294967295)),
            (0, 4294967295, ok(0, 4294967295)),
            (2, 4294967295, OUT),
        ],
    );
    check(
        &ty(0, 100, 0, 2),
        &[(1, 4294967295, OUT), (10, 4294967295, OUT)],
    );
}

#[test]
#[trace("TC-950", "FR-402-AC-5")]
fn canonical_and_trailing_zero_forms_are_distinct() {
    let t = ty(0, 100, 0, 2);
    let a = build(&t, 100, 2).unwrap();
    let b = build(&t, 1, 0).unwrap();
    assert_ne!(a, b);
    assert_eq!(a.normalized(), (1, 0));
    assert_eq!(b.normalized(), (1, 0));
    for ((c, sc), expected) in NORMALIZED {
        assert_eq!(
            D::from_retained(c, sc).normalized(),
            expected,
            "({c}, {sc})"
        );
    }
}

#[test]
#[trace("TC-950", "FR-402-AC-6", "FR-402-AC-8")]
fn read_out_and_owner_differential() {
    for (lo, hi, smin, smax, bindings) in BOUNDARY {
        let native = ty(lo, hi, smin, smax);
        let owner = DecimalType::new(
            Integer::from(lo),
            Integer::from(hi),
            smin,
            smax,
            RoundingMode::Exact,
        )
        .unwrap();
        assert_eq!(T::try_from(&owner), Ok(native));
        for &(c, s) in bindings {
            let Ok(scale) = u32::try_from(s) else {
                assert_eq!(build(&native, c, s), OUT);
                continue;
            };
            let decimal = Decimal::new(Integer::from(i128::from(c)), scale);
            let built = build(&native, c, s);
            assert_eq!(
                built.is_ok(),
                owner.contains(&decimal),
                "({lo},{hi};{smin},{smax}) ({c},{s})"
            );
            if let Err(refusal) = built {
                assert_eq!(refusal, Refusal::OutsideCarrierDomain);
            }
            if let Ok(carried) = built {
                let read = carried.to_decimal();
                assert_eq!(
                    read.representation(),
                    &DecimalRepresentation::new(Integer::from(i128::from(c)), scale)
                );
                let normalized = read.normalized();
                assert_eq!(
                    (
                        Integer::from(carried.normalized().0),
                        carried.normalized().1
                    ),
                    (normalized.coefficient().clone(), normalized.scale())
                );
            }
        }
    }
}

#[test]
#[trace("TC-950", "FR-402-AC-12")]
fn envelope_accessors() {
    let t = ty(i128::from(i64::MIN), i128::from(i64::MAX), 0, 0);
    let low: i128 = build(&t, i64::MIN, 0).unwrap().coefficient();
    let high: i128 = build(&t, i64::MAX, 0).unwrap().coefficient();
    assert_eq!((low, high), (-9223372036854775808, 9223372036854775807));
    let scale: u32 = build(&ty(0, 1, 0, 4294967295), 0, 4294967295)
        .unwrap()
        .scale();
    assert_eq!(scale, 4294967295);
}

#[test]
#[trace("TC-950", "FR-402-AC-7")]
fn copy_function_pointers_and_zero_allocation() {
    fn is_copy<X: Copy>() {}
    is_copy::<D>();
    is_copy::<T>();
    let _: fn(&T, i64, i64) -> Result<D, Refusal> = D::from_bindings;
    let _: fn(&D) -> (i128, u32) = D::normalized;
    let types: Vec<(T, &[(i64, i64)])> = BOUNDARY
        .iter()
        .map(|&(lo, hi, smin, smax, bindings)| (ty(lo, hi, smin, smax), bindings))
        .collect();
    let info = allocation_counter::measure(|| {
        for (t, bindings) in &types {
            for &(c, sc) in *bindings {
                let _ = build(t, c, sc);
            }
        }
        for ((c, sc), _) in NORMALIZED {
            let _ = D::from_retained(c, sc).normalized();
        }
    });
    assert_eq!(info.count_total, 0);
}

fn body(source: &str, signature: &str) -> String {
    let start = source
        .find(signature)
        .unwrap_or_else(|| panic!("{signature}"));
    let open = start + source[start..].find('{').unwrap();
    let mut depth = 0;
    for (offset, ch) in source[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return source[start..open + offset + 1].to_string();
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced {signature}");
}

/// What in a proof-path body breaks FR-402-AC-7.
fn violations(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for token in ["cfg(kani)", "Meter", "Vec", "Box", "String", "alloc", ".."] {
        if text.contains(token) {
            found.push(token.to_string());
        }
    }
    let words = ["for", "while", "loop", "Integer", "Decimal", "Rational"];
    for word in text.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
        if words.contains(&word) {
            found.push(word.to_string());
        }
    }
    // Any iterator adapter or consumer is a method call; the permitted method
    // calls are an explicit list, so an unnamed adapter is flagged too.
    let allowed = [
        "checked_mul",
        "try_from",
        "normalized",
        "normalized_pair",
        "admits",
        "admits_normalized",
        "from_retained",
    ];
    for piece in text.split('.').skip(1) {
        let name: String = piece
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if piece[name.len()..].starts_with('(') && !allowed.contains(&name.as_str()) {
            found.push(format!(".{name}("));
        }
    }
    found
}

#[test]
#[trace("TC-950", "FR-402-AC-7")]
fn source_inspection_of_proof_path_bodies() {
    let source = include_str!("../src/native_decimal.rs");
    for signature in [
        "pub fn from_bindings",
        "pub fn normalized(",
        "pub(crate) fn normalized_pair",
        "fn strip_zeros",
        "fn admits_normalized",
        "pub(crate) fn admits(",
    ] {
        let text = body(source, signature);
        assert_eq!(violations(&text), Vec::<String>::new(), "{signature}");
    }
}

#[test]
#[trace("TC-950", "FR-402-AC-7")]
fn source_inspection_flags_each_banned_construct() {
    for (sample, expected) in [
        ("(0..3).for_each(|i| drop(i))", ".for_each("),
        ("xs.iter().sum()", ".iter("),
        ("while a < b { a += 1 }", "while"),
        ("loop { break }", "loop"),
        ("for i in xs {}", "for"),
        ("#[cfg(kani)] x", "cfg(kani)"),
        ("Integer::from(1)", "Integer"),
        ("x.try_fold(0, f)", ".try_fold("),
        ("Vec::new()", "Vec"),
    ] {
        let found = violations(sample);
        assert!(found.iter().any(|f| f == expected), "{sample}: {found:?}");
    }
    assert!(violations("match a.checked_mul(b) { Some(v) => v, None => 0 }").is_empty());
}

#[test]
#[trace("TC-950", "FR-402-AC-9")]
fn compile_fail_controls_are_pinned_with_a_positive_twin() {
    let source = include_str!("../src/native_decimal.rs");
    assert_eq!(source.matches("```compile_fail,E0308").count(), 3);
    assert_eq!(source.matches("```compile_fail,E0451").count(), 1);
    assert!(source.contains("//! assert!(NativeDecimal::from_bindings(&ty, 5000, 2).is_ok());"));
}
