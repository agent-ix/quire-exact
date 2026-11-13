// SPDX-License-Identifier: AGPL-3.0-or-later
//! Host binding of the independent reference to the owner route, and of the
//! constructor to the reference, over a grid (FR-398 Behavior 9, TC-946
//! step 5).
use ix_trace_rs::trace;

use super::{
    construct, reference, NativeQuantityRefusal as Refusal, NativeQuantityShape as Shape,
    NativeRange, QuantityBindings,
};
use crate::decimal::{Decimal, DecimalType, RoundingMode};
use crate::integer::{Integer, IntegerInterval};
use crate::native_decimal::NativeDecimalType;
use crate::rational::{Rational, RationalDomain};

type Outcome = reference::Outcome;

const COEFFICIENTS: [i64; 26] = [
    -5,
    1000,
    100_000,
    1001,
    -1001,
    3,
    0,
    1,
    -1,
    2,
    -2,
    5,
    10,
    -10,
    25,
    100,
    -1000,
    5000,
    9999,
    10001,
    1 << 39,
    5_i64.pow(27),
    10_i64.pow(18),
    -(10_i64.pow(18)),
    i64::MAX,
    i64::MIN,
];
/// Scales at which the owner route can still build the power of ten.
const SCALES: [i64; 16] = [
    -1,
    0,
    1,
    2,
    3,
    5,
    7,
    18,
    19,
    20,
    37,
    38,
    39,
    40,
    64,
    4_294_967_296,
];
const BOUNDS: [(i128, i128); 9] = [
    (0, 549_755_813_888),
    (-100, 0),
    (0, 100),
    (-1000, 1000),
    (0, 1),
    (i128::MIN, 0),
    (0, i128::MAX),
    (i128::MIN, i128::MAX),
    (0, 0),
];
const SCALE_RANGES: [(u32, u32); 9] = [
    (0, 0),
    (0, 2),
    (2, 3),
    (38, 38),
    (60, 70),
    (0, 64),
    (64, 64),
    (128, 130),
    (0, u32::MAX),
];

fn owner_int(lower: i128, upper: i128, value: i64) -> Outcome {
    let interval = IntegerInterval::new(Integer::from(lower), Integer::from(upper)).unwrap();
    if interval.contains(&Integer::from(value)) {
        Ok((i128::from(value), 1))
    } else {
        Err(Refusal::OutsideCarrierDomain)
    }
}

fn owner_decimal(
    bounds: (i128, i128),
    scales: (u32, u32),
    coefficient: i64,
    scale: i64,
) -> Outcome {
    let owner = DecimalType::new(
        Integer::from(bounds.0),
        Integer::from(bounds.1),
        u64::from(scales.0),
        u64::from(scales.1),
        RoundingMode::Exact,
    )
    .unwrap();
    let Ok(scale) = u32::try_from(scale) else {
        return Err(Refusal::OutsideCarrierDomain);
    };
    let value = Decimal::new(Integer::from(coefficient), scale);
    if !owner.contains(&value) {
        return Err(Refusal::OutsideCarrierDomain);
    }
    let exact = value.normalized().to_rational();
    match (exact.numerator().native(), exact.denominator().native()) {
        (Some(n), Some(d)) => Ok((n, d)),
        _ => Err(Refusal::NotRepresentable),
    }
}

fn owner_rational(
    numerators: (i128, i128),
    denominators: (i128, i128),
    numerator: i64,
    denominator: i64,
) -> Outcome {
    let domain = RationalDomain::new(
        IntegerInterval::new(Integer::from(numerators.0), Integer::from(numerators.1)).unwrap(),
        IntegerInterval::new(Integer::from(denominators.0), Integer::from(denominators.1)).unwrap(),
    )
    .unwrap();
    let (n, d) = (Integer::from(numerator), Integer::from(denominator));
    let Ok(reduced) = Rational::new(n.clone(), d.clone()) else {
        return Err(Refusal::ZeroDenominator);
    };
    if reduced.numerator() != &n || reduced.denominator() != &d {
        return Err(Refusal::NonCanonical);
    }
    if domain.contains(&reduced) {
        Ok((i128::from(numerator), i128::from(denominator)))
    } else {
        Err(Refusal::OutsideCarrierDomain)
    }
}

fn run(shape: &Shape, bindings: QuantityBindings) -> Outcome {
    construct(shape, bindings).map(|q| (q.numerator(), q.denominator()))
}

#[test]
#[trace("TC-946", "FR-398-AC-8")]
fn decimal_reference_and_constructor_agree_with_the_owner_route() {
    for bounds in BOUNDS {
        for scales in SCALE_RANGES {
            let native = NativeDecimalType::new(
                bounds.0,
                bounds.1,
                u64::from(scales.0),
                u64::from(scales.1),
                RoundingMode::Exact,
            )
            .unwrap();
            let shape = Shape::Decimal(native);
            for coefficient in COEFFICIENTS {
                for scale in SCALES {
                    let case = (bounds, scales, coefficient, scale);
                    let expected = owner_decimal(bounds, scales, coefficient, scale);
                    let reference = reference::decimal(bounds, scales, coefficient, scale);
                    let actual = run(&shape, QuantityBindings::Decimal { coefficient, scale });
                    assert_eq!(reference, expected, "reference {case:?}");
                    assert_eq!(actual, expected, "constructor {case:?}");
                }
            }
        }
    }
}

#[test]
#[trace("TC-946", "FR-398-AC-8")]
fn int_reference_and_constructor_agree_with_the_owner_route() {
    for (lower, upper) in BOUNDS {
        let shape = Shape::Int(NativeRange { lower, upper });
        for value in COEFFICIENTS {
            let expected = owner_int(lower, upper, value);
            assert_eq!(reference::int(lower, upper, value), expected);
            assert_eq!(run(&shape, QuantityBindings::Int(value)), expected);
        }
    }
}

const PAIRS: [(i64, i64); 28] = [
    (2, 3),
    (2, 7),
    (5, 3),
    (1, 4),
    (3, 2),
    (2, 9),
    (0, 1),
    (1, 1),
    (-1, 1),
    (1, 2),
    (2, 1),
    (2, 4),
    (1, -2),
    (-1, -2),
    (0, 2),
    (1, 0),
    (0, 0),
    (6, 10),
    (3, 4),
    (6, 7),
    (5, 7),
    (i64::MIN, 1),
    (i64::MIN, i64::MAX),
    (i64::MAX, i64::MIN),
    (1, i64::MAX),
    (i64::MIN, 3),
    (4_660_046_610_375_530_309, 7_540_113_804_746_346_429),
    (7_540_113_804_746_346_429, 4_660_046_610_375_530_309),
];

#[test]
#[trace("TC-946", "FR-398-AC-8")]
fn rational_reference_and_constructor_agree_with_the_owner_route() {
    let ranges = [
        (
            (i128::from(i64::MIN), i128::from(i64::MAX)),
            (1, i128::from(i64::MAX)),
        ),
        ((2, 5), (3, 7)),
        ((1, 5), (2, 4)),
        ((0, 0), (1, 1)),
    ];
    for (numerators, denominators) in ranges {
        let shape = Shape::Rational {
            numerator: NativeRange {
                lower: numerators.0,
                upper: numerators.1,
            },
            denominator: NativeRange {
                lower: denominators.0,
                upper: denominators.1,
            },
        };
        for (numerator, denominator) in PAIRS {
            let expected = owner_rational(numerators, denominators, numerator, denominator);
            let bindings = QuantityBindings::Rational {
                numerator,
                denominator,
            };
            let case = (numerators, denominators, numerator, denominator);
            assert_eq!(
                reference::rational(numerators, denominators, numerator, denominator),
                expected,
                "reference {case:?}"
            );
            assert_eq!(run(&shape, bindings), expected, "constructor {case:?}");
        }
    }
}

/// The unrolled binary algorithm needs its worst case: consecutive Fibonacci
/// numbers force the longest Euclidean run, and a power of two against an odd
/// number the longest shift run.
#[test]
fn coprime_agrees_with_the_reference_gcd_on_worst_cases() {
    let mut fibonacci = [0_u64; 93];
    fibonacci[1] = 1;
    for k in 2..93 {
        fibonacci[k] = fibonacci[k - 1] + fibonacci[k - 2];
    }
    let mut samples = std::vec![0_u64; 0];
    samples.extend_from_slice(&fibonacci[1..]);
    samples.extend([
        1 << 63,
        (1 << 63) - 1,
        u64::MAX,
        u64::MAX - 1,
        3 << 61,
        5_u64.pow(27),
    ]);
    for &a in &samples {
        for &b in &samples {
            if a != 0 && b != 0 {
                assert_eq!(
                    super::coprime(a, b),
                    reference::gcd(a, b) == 1,
                    "({a}, {b})"
                );
            }
        }
    }
    assert!(super::coprime(fibonacci[92], fibonacci[91]));
}

/// The whole domain of the bounded Kani companions, enumerated.
#[test]
fn the_small_proof_domains_agree_with_the_reference_exhaustively() {
    let native = NativeDecimalType::new(-1000, 1000, 1, 4, RoundingMode::Exact).unwrap();
    let shape = Shape::Decimal(native);
    for coefficient in -(1_i64 << 15)..=(1 << 15) {
        for scale in 0..=8_i64 {
            assert_eq!(
                run(&shape, QuantityBindings::Decimal { coefficient, scale }),
                reference::decimal((-1000, 1000), (1, 4), coefficient, scale)
            );
        }
    }
    let shape = Shape::Rational {
        numerator: NativeRange {
            lower: -100,
            upper: 100,
        },
        denominator: NativeRange {
            lower: 1,
            upper: 100,
        },
    };
    for numerator in -128_i64..=128 {
        for denominator in -128_i64..=128 {
            assert_eq!(
                run(
                    &shape,
                    QuantityBindings::Rational {
                        numerator,
                        denominator
                    }
                ),
                reference::rational((-100, 100), (1, 100), numerator, denominator)
            );
        }
    }
}
