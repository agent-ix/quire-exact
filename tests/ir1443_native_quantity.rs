// SPDX-License-Identifier: AGPL-3.0-or-later
//! The carried-quantity constructor against literals and the owner route
//! (FR-398, TC-946).
use ix_trace_rs::trace;
use quire_exact::{
    CarriedQuantityType, Decimal, DecimalType, Integer, IntegerInterval, NativeQuantity,
    NativeQuantityRefusal as Refusal, NodeKey, PathSegment, Quantity, QuantityBindings as B,
    QuantityCarrier, Rational, RationalDomain, RoundingMode, UnitId,
};
use std::collections::BTreeSet;

type Built = Result<NativeQuantity, Refusal>;
type Magnitude = Result<(i128, i128), Refusal>;

fn unit() -> UnitId {
    UnitId::declared(NodeKey::new("test", vec![PathSegment::Text("cm".into())], 0).unwrap())
}

fn int(n: i128) -> Integer {
    Integer::from(n)
}
fn two_127() -> Integer {
    Integer::one().shifted_left(127)
}
fn two_128_plus_one() -> Integer {
    Integer::one().shifted_left(128).add(&Integer::one())
}

fn int_ty(lower: Integer, upper: Integer) -> CarriedQuantityType {
    CarriedQuantityType::new(
        unit(),
        QuantityCarrier::Integer {
            domain: IntegerInterval::new(lower, upper).unwrap(),
            rounding: RoundingMode::Exact,
        },
    )
}
fn dec_ty(lower: Integer, upper: Integer, smin: u64, smax: u64) -> CarriedQuantityType {
    let owner = DecimalType::new(lower, upper, smin, smax, RoundingMode::Exact).unwrap();
    CarriedQuantityType::new(unit(), QuantityCarrier::Decimal(owner))
}
fn rat_ty(n: (Integer, Integer), d: (Integer, Integer)) -> CarriedQuantityType {
    let domain = RationalDomain::new(
        IntegerInterval::new(n.0, n.1).unwrap(),
        IntegerInterval::new(d.0, d.1).unwrap(),
    )
    .unwrap();
    CarriedQuantityType::new(unit(), QuantityCarrier::Rational(domain))
}
fn d(lower: i128, upper: i128, smin: u64, smax: u64) -> CarriedQuantityType {
    dec_ty(int(lower), int(upper), smin, smax)
}
fn r(n: (i128, i128), den: (i128, i128)) -> CarriedQuantityType {
    rat_ty((int(n.0), int(n.1)), (int(den.0), int(den.1)))
}

fn dec(coefficient: i64, scale: i64) -> B {
    B::Decimal { coefficient, scale }
}
fn rat(numerator: i64, denominator: i64) -> B {
    B::Rational {
        numerator,
        denominator,
    }
}
fn magnitude(built: Built) -> Magnitude {
    built.map(|q| (q.numerator(), q.denominator()))
}
fn build(ty: &CarriedQuantityType, bindings: B) -> Magnitude {
    magnitude(NativeQuantity::from_bindings(ty, bindings))
}
fn check(ty: &CarriedQuantityType, cases: &[(B, Magnitude)]) {
    for (bindings, expected) in cases {
        assert_eq!(build(ty, *bindings), *expected, "{bindings:?}");
    }
}

const OUT: Magnitude = Err(Refusal::OutsideCarrierDomain);
const NON_CANONICAL: Magnitude = Err(Refusal::NonCanonical);
const NOT_REPRESENTABLE: Magnitude = Err(Refusal::NotRepresentable);
const MISMATCH: Magnitude = Err(Refusal::CarrierMismatch);
const OVERSIZE: Magnitude = Err(Refusal::TypeNotRepresentable);
const ZERO_DENOMINATOR: Magnitude = Err(Refusal::ZeroDenominator);

fn wide_int() -> CarriedQuantityType {
    int_ty(int(i128::from(i64::MIN)), int(i128::from(i64::MAX)))
}
fn wide_rational() -> CarriedQuantityType {
    r(
        (i128::from(i64::MIN), i128::from(i64::MAX)),
        (1, i128::from(i64::MAX)),
    )
}
fn wide_decimal() -> CarriedQuantityType {
    d(0, 549_755_813_888, 0, 4_294_967_295)
}

#[test]
#[trace("TC-946", "FR-398-AC-1")]
fn int_bindings_give_n_over_one() {
    let cases: Vec<_> = [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX]
        .map(|n| (B::Int(n), Ok((i128::from(n), 1))))
        .into();
    check(&wide_int(), &cases);
    check(
        &int_ty(int(-5), int(100)),
        &[
            (B::Int(-5), Ok((-5, 1))),
            (B::Int(100), Ok((100, 1))),
            (B::Int(-6), OUT),
            (B::Int(101), OUT),
        ],
    );
}

#[test]
#[trace("TC-946", "FR-398-AC-2")]
fn decimal_bindings_give_the_exact_canonical_value() {
    check(
        &d(-1000, 1000, 0, 2),
        &[
            (dec(10, 1), Ok((1, 1))),
            (dec(-5, 1), Ok((-1, 2))),
            (dec(0, 7), Ok((0, 1))),
            (dec(1000, 0), Ok((1000, 1))),
            (dec(-1000, 0), Ok((-1000, 1))),
            (dec(100_000, 2), Ok((1000, 1))),
            (dec(1001, 0), OUT),
            (dec(-1001, 0), OUT),
            (dec(i64::MIN, 0), OUT),
            (dec(i64::MAX, 0), OUT),
            (dec(1, 4_294_967_296), OUT),
        ],
    );
    check(
        &d(0, 100, 0, 2),
        &[
            (dec(5000, 2), Ok((50, 1))),
            (dec(10, 3), Ok((1, 100))),
            (dec(1, 3), OUT),
            (dec(10_001, 2), OUT),
            (dec(9999, 2), OUT),
        ],
    );
    check(
        &d(0, 100, 2, 3),
        &[(dec(1, 0), Ok((1, 1))), (dec(2, 0), OUT)],
    );
    check(
        &d(0, 100, 60, 70),
        &[(dec(1, 0), OUT), (dec(0, 0), Ok((0, 1)))],
    );
    check(&d(0, 100, 128, 130), &[(dec(1, 0), OUT)]);
    check(&d(-100, 0, 60, 70), &[(dec(-1, 0), OUT)]);
    check(&wide_decimal(), &[(dec(1, -1), OUT)]);
}

#[test]
#[trace("TC-946", "FR-398-AC-3")]
fn rational_bindings_give_n_over_d_in_order() {
    let corners =
        [(2, 3), (2, 7), (5, 3), (5, 7)].map(|(n, den)| (rat(n, den), Ok((n.into(), den.into()))));
    check(&r((2, 5), (3, 7)), &corners);
    check(
        &r((2, 5), (3, 7)),
        &[
            (rat(1, 4), OUT),
            (rat(6, 7), OUT),
            (rat(3, 2), OUT),
            (rat(2, 9), OUT),
        ],
    );
    check(
        &wide_rational(),
        &[
            (rat(0, 1), Ok((0, 1))),
            (rat(1, 1), Ok((1, 1))),
            (rat(-1, 1), Ok((-1, 1))),
            (rat(1, 2), Ok((1, 2))),
            (rat(2, 1), Ok((2, 1))),
            (rat(i64::MIN, 1), Ok((i128::from(i64::MIN), 1))),
            (rat(1, i64::MAX), Ok((1, i128::from(i64::MAX)))),
            (
                rat(i64::MIN, i64::MAX),
                Ok((i128::from(i64::MIN), i128::from(i64::MAX))),
            ),
            (rat(2, 4), NON_CANONICAL),
            (rat(1, -2), NON_CANONICAL),
            (rat(-1, -2), NON_CANONICAL),
            (rat(0, 2), NON_CANONICAL),
            (rat(1, 0), ZERO_DENOMINATOR),
            (rat(0, 0), ZERO_DENOMINATOR),
        ],
    );
}

#[test]
#[trace("TC-946", "FR-398-AC-4")]
fn decimal_fit_is_decided_by_the_canonical_fraction() {
    let ten_38 = 10_i128.pow(38);
    check(
        &wide_decimal(),
        &[
            (dec(1, 37), Ok((1, 10_i128.pow(37)))),
            (dec(1, 38), Ok((1, ten_38))),
            (dec(1, 39), NOT_REPRESENTABLE),
            (dec(1, 4_294_967_295), NOT_REPRESENTABLE),
            (dec(549_755_813_888, 39), Ok((1, 5_i128.pow(39)))),
        ],
    );
}

#[test]
#[trace("TC-946", "FR-398-AC-5")]
fn a_binding_of_another_carrier_is_a_mismatch_before_anything_else() {
    let (i, dc, ra) = (wide_int(), wide_decimal(), wide_rational());
    for ty in [&dc, &ra] {
        assert_eq!(build(ty, B::Int(1)), MISMATCH);
    }
    for ty in [&i, &ra] {
        assert_eq!(build(ty, dec(1, 0)), MISMATCH);
    }
    for ty in [&i, &dc] {
        assert_eq!(build(ty, rat(1, 1)), MISMATCH);
    }
    assert_eq!(
        build(&int_ty(int(0), two_127()), dec(1, 0)),
        MISMATCH,
        "mismatch precedes an oversize type"
    );
    assert_eq!(build(&r((1, 5), (2, 4)), rat(6, 10)), NON_CANONICAL);
    assert_eq!(build(&d(0, 1, 0, 2), dec(1, 39)), OUT);
}

#[test]
#[trace("TC-946", "FR-398-AC-6")]
fn an_oversize_type_bound_refuses_every_binding_of_its_carrier() {
    for upper in [two_127(), two_128_plus_one()] {
        assert_eq!(build(&int_ty(int(0), upper.clone()), B::Int(1)), OVERSIZE);
        assert_eq!(
            build(&dec_ty(int(0), upper.clone(), 0, 2), dec(1, 0)),
            OVERSIZE
        );
        let ty = rat_ty((int(1), int(5)), (int(2), upper));
        assert_eq!(build(&ty, rat(1, 2)), OVERSIZE);
        assert_eq!(build(&ty, rat(1, 0)), OVERSIZE, "zero denominator");
        assert_eq!(build(&ty, rat(2, 4)), OVERSIZE, "non-canonical");
    }
    let below = int(i128::MIN).sub(&Integer::one());
    assert_eq!(build(&int_ty(below, int(0)), B::Int(0)), OVERSIZE);
}

fn same_fn(_: fn(&CarriedQuantityType, B) -> Built) {}

#[test]
#[trace("TC-946", "FR-398-AC-7")]
fn the_constructor_is_loop_free_copy_unit_free_and_allocation_free() {
    // Source inspection of the production part of the module.
    let source = include_str!("../src/native_quantity.rs");
    let production = source.split("\n#[cfg(").next().expect("production part");
    let code: String = production
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    let banned = [
        "for",
        "while",
        "loop",
        "iter",
        "into_iter",
        "map",
        "filter",
        "fold",
        "zip",
        "chain",
        "take",
        "skip",
        "rev",
        "collect",
        "cfg",
        "kani",
        "Meter",
    ];
    for token in code.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
        assert!(!banned.contains(&token), "banned token `{token}`");
    }
    // A Meter-free function pointer; no unit in the result.
    same_fn(NativeQuantity::from_bindings);
    fn copy<T: Copy>() {}
    copy::<NativeQuantity>();
    assert!(!std::mem::needs_drop::<NativeQuantity>());
    assert_eq!(std::mem::size_of::<NativeQuantity>(), 32);
    // Zero allocations around the whole boundary set.
    let types: Vec<_> = boundary().into_iter().collect();
    let info = allocation_counter::measure(|| {
        for (_, ty, bindings) in &types {
            std::hint::black_box(NativeQuantity::from_bindings(ty, *bindings).ok());
        }
    });
    assert_eq!(info.count_total, 0);
    // The read-out is outside the window.
    let ty = int_ty(int(0), int(100));
    let q = NativeQuantity::from_bindings(&ty, B::Int(50)).unwrap();
    assert_eq!(q.to_quantity(&ty).unit(), ty.unit());
}

#[test]
#[trace("TC-946", "FR-398-AC-9")]
fn the_constructor_takes_a_carried_quantity_type() {
    // The compile_fail,E0308 controls are the doctests of the module.
    let ty = int_ty(int(0), int(100));
    assert!(NativeQuantity::from_bindings(&ty, B::Int(1)).is_ok());
}

/// Every boundary binding of TC-946 with its type, labelled.
fn boundary() -> Vec<(String, CarriedQuantityType, B)> {
    let mut all: Vec<(String, CarriedQuantityType, B)> = Vec::new();
    let mut add = |name: &str, ty: CarriedQuantityType, bindings: &[B]| {
        for b in bindings {
            all.push((format!("{name} {b:?}"), ty.clone(), *b));
        }
    };
    add(
        "Int[min,max]",
        wide_int(),
        &[i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX].map(B::Int),
    );
    add(
        "Int[-5,100]",
        int_ty(int(-5), int(100)),
        &[-6, -5, 100, 101].map(B::Int),
    );
    add(
        "D[-1000,1000;0,2]",
        d(-1000, 1000, 0, 2),
        &[
            dec(10, 1),
            dec(-5, 1),
            dec(0, 7),
            dec(1000, 0),
            dec(-1000, 0),
            dec(100_000, 2),
            dec(1001, 0),
            dec(-1001, 0),
            dec(i64::MIN, 0),
            dec(i64::MAX, 0),
            dec(1, 4_294_967_296),
        ],
    );
    add(
        "D[0,100;0,2]",
        d(0, 100, 0, 2),
        &[
            dec(5000, 2),
            dec(10, 3),
            dec(10_001, 2),
            dec(9999, 2),
            dec(1, 3),
        ],
    );
    add("D[0,100;2,3]", d(0, 100, 2, 3), &[dec(1, 0), dec(2, 0)]);
    add("D[0,100;60,70]", d(0, 100, 60, 70), &[dec(1, 0), dec(0, 0)]);
    add("D[0,100;128,130]", d(0, 100, 128, 130), &[dec(1, 0)]);
    add("D[-100,0;60,70]", d(-100, 0, 60, 70), &[dec(-1, 0)]);
    add(
        "D[0,2^39;0,max]",
        wide_decimal(),
        &[
            dec(1, 37),
            dec(1, 38),
            dec(1, 39),
            dec(1, 4_294_967_295),
            dec(549_755_813_888, 39),
            dec(1, -1),
        ],
    );
    add("D[0,1;0,2]", d(0, 1, 0, 2), &[dec(1, 39)]);
    add(
        "R[wide]",
        wide_rational(),
        &[
            rat(0, 1),
            rat(1, 1),
            rat(-1, 1),
            rat(1, 2),
            rat(2, 1),
            rat(i64::MIN, 1),
            rat(1, i64::MAX),
            rat(i64::MIN, i64::MAX),
            rat(2, 4),
            rat(1, -2),
            rat(-1, -2),
            rat(0, 2),
            rat(1, 0),
            rat(0, 0),
        ],
    );
    add(
        "R[2,5;3,7]",
        r((2, 5), (3, 7)),
        &[
            rat(2, 3),
            rat(2, 7),
            rat(5, 3),
            rat(5, 7),
            rat(1, 4),
            rat(6, 7),
            rat(3, 2),
            rat(2, 9),
        ],
    );
    add("R[1,5;2,4]", r((1, 5), (2, 4)), &[rat(6, 10)]);
    for (n, upper) in [("2^127", two_127()), ("2^128+1", two_128_plus_one())] {
        add(
            &format!("Int[0,{n}]"),
            int_ty(int(0), upper.clone()),
            &[B::Int(1)],
        );
        add(
            &format!("D[0,{n};0,2]"),
            dec_ty(int(0), upper.clone(), 0, 2),
            &[dec(1, 0)],
        );
        let ty = rat_ty((int(1), int(5)), (int(2), upper));
        add(
            &format!("R[1,5;2,{n}]"),
            ty,
            &[rat(1, 2), rat(1, 0), rat(2, 4)],
        );
    }
    all
}

/// The exact canonical fraction of `c / 10^s` and whether it fits `i128`,
/// from the multiplicities of 2 and 5, never building `10^s`.
fn exponent_fraction(coefficient: i64, scale: u32) -> Option<(i128, i128)> {
    if coefficient == 0 {
        return Some((0, 1));
    }
    let mut rest = i128::from(coefficient);
    let (mut twos, mut fives) = (0_u32, 0_u32);
    while twos < scale && rest % 2 == 0 {
        rest /= 2;
        twos += 1;
    }
    while fives < scale && rest % 5 == 0 {
        rest /= 5;
        fives += 1;
    }
    let (two_exponent, five_exponent) = (scale - twos, scale - fives);
    if two_exponent > 126 || five_exponent > 54 {
        return None;
    }
    let denominator = 2_i128
        .pow(two_exponent)
        .checked_mul(5_i128.pow(five_exponent))?;
    Some((rest, denominator))
}

/// What the owner route says about a binding of the type's own carrier:
/// `None` for a refused binding, the exact magnitude for an admitted one.
fn owner_route(ty: &CarriedQuantityType, bindings: B) -> Option<Rational> {
    let ratio = |n: Integer, den: Integer| Rational::new(n, den).unwrap();
    match (ty.carrier(), bindings) {
        (QuantityCarrier::Integer { domain, .. }, B::Int(n)) => {
            let value = Integer::from(n);
            domain
                .contains(&value)
                .then(|| Rational::from_integer(value))
        }
        (QuantityCarrier::Decimal(owner), B::Decimal { coefficient, scale }) => {
            let scale = u32::try_from(scale).ok()?;
            let value = Decimal::new(Integer::from(coefficient), scale);
            if !owner.contains(&value) {
                return None;
            }
            if scale <= 64 {
                Some(value.normalized().to_rational())
            } else {
                // Beyond this the owner builds an unholdable power of ten; the
                // exponent reference supplies the fraction when it fits, and a
                // stand-in that does not fit stands for one that does not.
                let fraction = exponent_fraction(coefficient, scale);
                Some(match fraction {
                    Some((n, den)) => ratio(int(n), int(den)),
                    None => ratio(int(1), two_128_plus_one()),
                })
            }
        }
        (
            QuantityCarrier::Rational(domain),
            B::Rational {
                numerator,
                denominator,
            },
        ) => {
            let value = Rational::new(Integer::from(numerator), Integer::from(denominator)).ok()?;
            domain.contains(&value).then_some(value)
        }
        _ => unreachable!("mismatched bindings are not in the differential"),
    }
}

fn is_oversize(ty: &CarriedQuantityType) -> bool {
    let beyond = |i: &Integer| i > &int(i128::MAX) || i < &int(i128::MIN);
    match ty.carrier() {
        QuantityCarrier::Integer { domain, .. } => beyond(domain.lower()) || beyond(domain.upper()),
        QuantityCarrier::Decimal(o) => beyond(o.lower()) || beyond(o.upper()),
        QuantityCarrier::Rational(domain) => [domain.numerator(), domain.denominator()]
            .iter()
            .any(|i| beyond(i.lower()) || beyond(i.upper())),
    }
}

#[test]
#[trace("TC-946", "FR-398-AC-8")]
fn the_constructor_agrees_with_the_owner_route_except_three_sets() {
    let mut differences = BTreeSet::new();
    for (label, ty, bindings) in boundary() {
        let built = NativeQuantity::from_bindings(&ty, bindings);
        if is_oversize(&ty) {
            assert_eq!(built, Err(Refusal::TypeNotRepresentable), "{label}");
            differences.insert(format!("{label} => {:?}", Refusal::TypeNotRepresentable));
            continue;
        }
        let oracle = owner_route(&ty, bindings);
        if let B::Rational {
            numerator,
            denominator,
        } = bindings
        {
            let canonical = denominator > 0
                && Rational::new(Integer::from(numerator), Integer::from(denominator)).is_ok_and(
                    |v| {
                        v.numerator() == &Integer::from(numerator)
                            && v.denominator() == &Integer::from(denominator)
                    },
                );
            if denominator != 0 && !canonical {
                assert_eq!(built, Err(Refusal::NonCanonical), "{label}");
                differences.insert(format!("{label} => {:?}", Refusal::NonCanonical));
                continue;
            }
            if denominator == 0 {
                assert_eq!(built, Err(Refusal::ZeroDenominator), "{label}");
                assert!(oracle.is_none());
                continue;
            }
        }
        match (built, oracle) {
            (Ok(native), Some(exact)) => {
                let expected = Quantity::new(exact, ty.unit().clone());
                assert_eq!(native.to_quantity(&ty), expected, "{label}");
            }
            (Err(Refusal::OutsideCarrierDomain), None) => {}
            (Err(Refusal::NotRepresentable), Some(exact)) => {
                // The owner admits it and the native width cannot hold it.
                let beyond = |i: &Integer| i > &int(i128::MAX);
                assert!(
                    beyond(exact.numerator()) || beyond(exact.denominator()),
                    "{label}"
                );
                differences.insert(format!("{label} => {:?}", Refusal::NotRepresentable));
            }
            (built, oracle) => panic!("{label}: {built:?} against {oracle:?}"),
        }
    }
    let mut expected: BTreeSet<String> = BTreeSet::new();
    let mut expect = |label: String, refusal: Refusal| {
        expected.insert(format!("{label} => {refusal:?}"));
    };
    for (n, den) in [(2, 4), (1, -2), (-1, -2), (0, 2)] {
        expect(format!("R[wide] {:?}", rat(n, den)), Refusal::NonCanonical);
    }
    expect(
        format!("R[1,5;2,4] {:?}", rat(6, 10)),
        Refusal::NonCanonical,
    );
    for scale in [39, 4_294_967_295] {
        expect(
            format!("D[0,2^39;0,max] {:?}", dec(1, scale)),
            Refusal::NotRepresentable,
        );
    }
    for n in ["2^127", "2^128+1"] {
        expect(
            format!("Int[0,{n}] {:?}", B::Int(1)),
            Refusal::TypeNotRepresentable,
        );
        expect(
            format!("D[0,{n};0,2] {:?}", dec(1, 0)),
            Refusal::TypeNotRepresentable,
        );
        for (num, den) in [(1, 2), (1, 0), (2, 4)] {
            expect(
                format!("R[1,5;2,{n}] {:?}", rat(num, den)),
                Refusal::TypeNotRepresentable,
            );
        }
    }
    assert_eq!(differences, expected);
}

#[test]
#[trace("TC-946", "FR-398-AC-8")]
fn the_exponent_reference_agrees_with_the_owner_route_to_scale_64() {
    let wide = d(i64::MIN.into(), i64::MAX.into(), 0, 64);
    for coefficient in [
        0,
        1,
        -1,
        2,
        5,
        10,
        1 << 39,
        5_i64.pow(27),
        i64::MIN,
        i64::MAX,
        999_999,
    ] {
        for scale in 0..=64_u32 {
            let owner = Decimal::new(Integer::from(coefficient), scale)
                .normalized()
                .to_rational();
            let fit = |i: &Integer| i <= &int(i128::MAX);
            let expected = (fit(owner.numerator()) && fit(owner.denominator())).then_some(owner);
            let reference = exponent_fraction(coefficient, scale)
                .map(|(n, den)| Rational::new(int(n), int(den)).unwrap());
            assert_eq!(reference, expected, "({coefficient}, {scale})");
            let built = NativeQuantity::from_bindings(&wide, dec(coefficient, scale.into()));
            assert_eq!(
                built.is_ok(),
                expected.is_some(),
                "({coefficient}, {scale})"
            );
        }
    }
}
