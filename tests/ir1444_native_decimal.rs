// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native Decimal carrier against literals and the owner route (FR-402,
//! TC-950).
use ix_trace_rs::trace;
use quire_exact::{
    Decimal, DecimalRepresentation, DecimalType, IllTypedCause, Integer, NativeDecimal as D,
    NativeDecimalRefusal as Refusal, NativeDecimalType as T, RoundingMode,
};

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
    let one = |c: i128, s: u32| D::from_retained(c, s).normalized();
    assert_eq!(one(0, 7), (0, 0));
    assert_eq!(one(-1000, 3), (-1, 0));
    assert_eq!(one(120, 5), (12, 4));
    assert_eq!(one(7, 3), (7, 3));
    assert_eq!(one(1_000_000_000_000_000_000, 18), (1, 0));
    assert_eq!(one(1_000_000_000_000_000_000, 19), (1, 1));
    assert_eq!(one(1_000_000_000_000_000_000, 5), (10_000_000_000_000, 0));
    assert_eq!(one(9_000_000_000_000_000_000, 20), (9, 2));
    assert_eq!(one(100, 2), (1, 0));
    assert_eq!(one(10_i128.pow(37), 40), (1, 3));
    assert_eq!(one(-(10_i128.pow(38)), 38), (-1, 0));
    assert_eq!(one(i128::MIN, 0), (i128::MIN, 0));
    assert_eq!(one(i128::MIN, u32::MAX), (i128::MIN, u32::MAX));
    assert_eq!(one(10_i128.pow(38), u32::MAX), (1, 4294967257));
}

#[test]
#[trace("TC-950", "FR-402-AC-6", "FR-402-AC-8")]
fn read_out_and_owner_differential() {
    let types = [
        (0_i128, 100_i128, 0_u64, 2_u64),
        (-1000, 1000, 0, 2),
        (i64::MIN.into(), i64::MAX.into(), 0, 0),
        (0, 100, 2, 3),
        (0, i128::MAX, 38, 38),
        (0, i128::MAX, 39, 39),
        (i128::MIN, 0, 38, 38),
        (0, 100, 60, 70),
        (-100, 0, 60, 70),
        (0, 100, 128, 130),
        (0, 1, 0, 4294967295),
    ];
    let bindings = [
        (5000_i64, 2_i64),
        (10, 3),
        (1, 3),
        (10001, 2),
        (9999, 2),
        (100, 2),
        (1, 0),
        (2, 0),
        (0, 0),
        (-1, 0),
        (-2, 0),
        (0, 7),
        (1000, 0),
        (-1000, 0),
        (1001, 0),
        (-1001, 0),
        (i64::MIN, 0),
        (i64::MAX, 0),
        (1, 4294967295),
        (0, 4294967295),
        (2, 4294967295),
        (10, 4294967295),
    ];
    for (lo, hi, smin, smax) in types {
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
        for (c, s) in bindings {
            let decimal = Decimal::new(Integer::from(i128::from(c)), u32::try_from(s).unwrap());
            let built = build(&native, c, s);
            assert_eq!(
                built.is_ok(),
                owner.contains(&decimal),
                "({lo},{hi};{smin},{smax}) ({c},{s})"
            );
            if let Ok(carried) = built {
                let read = carried.to_decimal();
                assert_eq!(
                    read.representation(),
                    &DecimalRepresentation::new(
                        Integer::from(i128::from(c)),
                        u32::try_from(s).unwrap()
                    )
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
    let t = ty(0, 100, 0, 2);
    let t2 = ty(i128::MIN, 0, 38, 38);
    let info = allocation_counter::measure(|| {
        for (c, s) in [
            (5000_i64, 2_i64),
            (10, 3),
            (1, 3),
            (9999, 2),
            (1, 4294967295),
            (1, -1),
        ] {
            let _ = build(&t, c, s);
        }
        let _ = build(&t2, -2, 0);
        for (c, s) in [
            (10_i128.pow(18), 18_u32),
            (10_i128.pow(38), u32::MAX),
            (0, 7),
        ] {
            let _ = D::from_retained(c, s).normalized();
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

#[test]
#[trace("TC-950", "FR-402-AC-7")]
fn source_inspection_of_proof_path_bodies() {
    let source = include_str!("../src/native_decimal.rs");
    let bodies = [
        body(source, "pub fn from_bindings"),
        body(source, "pub fn normalized"),
        body(source, "fn strip_zeros"),
        body(source, "pub(crate) fn admits_normalized"),
    ];
    let banned_tokens = ["cfg(kani)", "Meter", "Vec", "Box", "String", "alloc"];
    let banned_words = ["for", "while", "loop", "Integer", "Decimal", "Rational"];
    for text in &bodies {
        for token in banned_tokens {
            assert!(!text.contains(token), "{token} in {text}");
        }
        for word in banned_words {
            let found = text
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(|w| w == word);
            assert!(!found, "{word} in {text}");
        }
        for adapter in [
            ".iter(",
            ".map(",
            ".filter(",
            ".fold(",
            ".all(",
            ".any(",
            ".find(",
            "into_iter",
        ] {
            assert!(!text.contains(adapter), "{adapter} in {text}");
        }
    }
}

#[test]
#[trace("TC-950", "FR-402-AC-3")]
fn refusal_carries_no_decimal_and_has_two_causes() {
    let t = ty(0, 100, 0, 2);
    assert_eq!(build(&t, 1, 3).unwrap_err(), Refusal::OutsideCarrierDomain);
    assert_ne!(Refusal::TypeNotRepresentable, Refusal::OutsideCarrierDomain);
}

#[test]
#[trace("TC-950", "FR-402-AC-9")]
fn compile_fail_controls_are_pinned_with_a_positive_twin() {
    let source = include_str!("../src/native_decimal.rs");
    assert_eq!(source.matches("```compile_fail,E0308").count(), 3);
    assert_eq!(source.matches("```compile_fail,E0451").count(), 1);
    assert!(source.contains("//! assert!(NativeDecimal::from_bindings(&ty, 5000, 2).is_ok());"));
}
