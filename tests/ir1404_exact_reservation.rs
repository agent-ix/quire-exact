// SPDX-License-Identifier: AGPL-3.0-or-later
//! Reservation-charged exact integer entries (FR-396).
//!
//! `allocation_counter` observes the bytes actually requested; the reservation
//! records the bytes asked. Fixtures and the independent `num-bigint` oracle
//! are built outside each measurement window.

use core::str::FromStr;

use ix_trace_rs::trace;
use num_bigint::BigInt;
use quire_exact::{
    Integer, IntegerParseError, IntegerReserveError, NonZeroInteger, Reservation, ReservationDenied,
};

/// Admits requests until the provision is spent; records what it admitted.
struct Provision {
    remaining: usize,
    admitted: usize,
    requests: usize,
}

impl Provision {
    fn new(bytes: usize) -> Self {
        Self {
            remaining: bytes,
            admitted: 0,
            requests: 0,
        }
    }
}

impl Reservation for Provision {
    fn reserve(&mut self, bytes: usize) -> Result<(), ReservationDenied> {
        if bytes > self.remaining {
            return Err(ReservationDenied);
        }
        self.remaining -= bytes;
        self.admitted += bytes;
        self.requests += 1;
        Ok(())
    }
}

fn measure<T>(call: impl FnOnce() -> T) -> (T, allocation_counter::AllocationInfo) {
    let mut outcome = None;
    let info = allocation_counter::measure(|| outcome = Some(call()));
    (outcome.expect("measurement returned"), info)
}

fn int(spelling: &str) -> Integer {
    Integer::from_str(spelling).expect("canonical spelling")
}

/// Each spelling parsed (limb-backed) and built from a `BigInt` (BigInt-backed),
/// so both large representations are operands.
fn operands() -> Vec<(&'static str, Integer)> {
    SPELLINGS
        .iter()
        .flat_map(|s| [(*s, int(s)), (*s, Integer::from_big(big(s)))])
        .collect()
}

fn big(spelling: &str) -> BigInt {
    BigInt::from_str(spelling).expect("oracle spelling")
}

/// Zero, signs, the `i128` edges, the first heap values and multi-limb values.
const SPELLINGS: &[&str] = &[
    "0",
    "1",
    "-1",
    "7",
    "-12",
    "170141183460469231731687303715884105727",
    "-170141183460469231731687303715884105728",
    "170141183460469231731687303715884105728",
    "-170141183460469231731687303715884105729",
    "340282366920938463463374607431768211456",
    "-1606938044258990275541962092341162602522202993782792835301376",
    "1606938044258990275541962092341162602522202993782792835301376",
    "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "-98765432109876543210987654321098765432109876543210987654321098765432109876543210",
];

/// Run `call` once under an unlimited recording provision, then check the
/// charged bytes equal the bytes allocated, the exact provision admits the
/// same value, and every smaller provision refuses with nothing allocated
/// beyond what it admitted.
fn check<T: PartialEq + core::fmt::Debug>(
    label: &str,
    expected: &T,
    call: impl Fn(&mut Provision) -> Result<T, IntegerReserveError>,
) {
    let mut recording = Provision::new(usize::MAX);
    let (value, info) = measure(|| call(&mut recording));
    let value = value.unwrap_or_else(|error| panic!("{label}: {error}"));
    assert_eq!(&value, expected, "{label}");
    assert_eq!(
        info.bytes_total, recording.admitted as u64,
        "{label}: charged == requested"
    );
    assert_eq!(
        info.count_total, recording.requests as u64,
        "{label}: one request per block"
    );

    let mut exact = Provision::new(recording.admitted);
    let again = call(&mut exact).unwrap_or_else(|error| panic!("{label}: exact provision {error}"));
    assert_eq!(&again, expected, "{label}: exact provision");
    assert_eq!(exact.remaining, 0, "{label}");

    for budget in 0..recording.admitted {
        let mut short = Provision::new(budget);
        let (outcome, info) = measure(|| call(&mut short));
        assert_eq!(
            outcome.err(),
            Some(IntegerReserveError::Denied),
            "{label}: budget {budget}"
        );
        assert_eq!(
            info.bytes_total, short.admitted as u64,
            "{label}: budget {budget} allocates only what it admitted"
        );
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-1")]
fn parse_charges_what_it_allocates_and_refuses_every_smaller_provision() {
    for spelling in SPELLINGS {
        let expected = big(spelling);
        let mut recording = Provision::new(usize::MAX);
        let (parsed, info) = measure(|| Integer::parse_reserved(spelling, &mut recording));
        assert_eq!(parsed.expect("parse").as_big(), &expected, "{spelling}");
        assert_eq!(info.bytes_total, recording.admitted as u64, "{spelling}");
        for budget in 0..recording.admitted {
            let mut short = Provision::new(budget);
            let (outcome, info) = measure(|| Integer::parse_reserved(spelling, &mut short));
            assert_eq!(
                outcome.err(),
                Some(IntegerParseError::Reserve(IntegerReserveError::Denied)),
                "{spelling} at {budget}"
            );
            assert_eq!(
                info.bytes_total, short.admitted as u64,
                "{spelling} at {budget}"
            );
        }
        let mut exact = Provision::new(recording.admitted);
        assert!(Integer::parse_reserved(spelling, &mut exact).is_ok());
    }
    // A spelling inside i128 asks for nothing, and noncanonical text is refused first.
    let mut none = Provision::new(0);
    assert!(Integer::parse_reserved("170141183460469231731687303715884105727", &mut none).is_ok());
    for bad in ["", "-", "01", "-0", "+1", "1.0", "1e3", " 1", "-01"] {
        let mut none = Provision::new(0);
        assert!(
            matches!(
                Integer::parse_reserved(bad, &mut none),
                Err(IntegerParseError::NonCanonical(_))
            ),
            "{bad:?}"
        );
        assert_eq!(none.requests, 0);
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-2")]
fn addition_charges_what_it_allocates_and_refuses_every_smaller_provision() {
    let operands = operands();
    for (left_text, left) in &operands {
        for (right_text, right) in &operands {
            let expected = big(left_text) + big(right_text);
            check(
                &format!("{left_text} + {right_text}"),
                &Integer::from_big(expected),
                |reservation| left.add_reserved(right, reservation),
            );
        }
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-3")]
fn multiplication_charges_what_it_allocates_and_refuses_every_smaller_provision() {
    let operands = operands();
    for (left_text, left) in &operands {
        for (right_text, right) in &operands {
            let expected = big(left_text) * big(right_text);
            check(
                &format!("{left_text} * {right_text}"),
                &Integer::from_big(expected),
                |reservation| left.mul_reserved(right, reservation),
            );
        }
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-4")]
fn exact_division_charges_what_it_allocates_and_refuses_every_smaller_provision() {
    let operands = operands();
    for (divisor_text, divisor) in operands.iter().filter(|(_, d)| !d.is_zero()) {
        let divisor_ref = NonZeroInteger::try_from(divisor).expect("nonzero");
        for (factor_text, factor) in &operands {
            let dividend = factor.mul(divisor);
            let expected = big(factor_text);
            check(
                &format!("({factor_text} * {divisor_text}) / {divisor_text}"),
                &Integer::from_big(expected),
                |reservation| dividend.exact_div_reserved(divisor_ref, reservation),
            );
        }
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-5")]
fn gcd_charges_what_it_allocates_and_refuses_every_smaller_provision() {
    let common = int("340282366920938463463374607431768211456123456789");
    let cases = [
        (
            int("6").mul(&common),
            int("-4").mul(&common),
            int("2").mul(&common),
        ),
        (common.clone(), common.neg(), common.clone()),
        (int("0"), common.neg(), common.clone()),
        (common.clone(), int("0"), common.clone()),
        (int("0"), int("0"), int("0")),
        (
            int("-170141183460469231731687303715884105728"),
            int("-170141183460469231731687303715884105728"),
            int("170141183460469231731687303715884105728"),
        ),
        (int("12"), int("-18"), int("6")),
        (common.mul(&int("35")), int("21"), int("7")),
        (
            int("1606938044258990275541962092341162602522202993782792835301376"),
            int("1024"),
            int("1024"),
        ),
    ];
    for (left, right, expected) in cases {
        check(&format!("gcd({left}, {right})"), &expected, |reservation| {
            left.gcd_reserved(&right, reservation)
        });
        assert_eq!(left.gcd(&right), expected, "unmetered gcd({left}, {right})");
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-6")]
fn comparison_and_sign_tests_allocate_nothing() {
    let values: Vec<Integer> = SPELLINGS.iter().map(|s| int(s)).collect();
    let (_, info) = measure(|| {
        for left in &values {
            for right in &values {
                let _ = left.cmp(right);
                let _ = left == right;
            }
            let _ = (left.is_zero(), left.is_negative(), left.is_even());
        }
    });
    assert_eq!(info.bytes_total, 0);
    assert_eq!(info.count_total, 0);
    for (left, right) in values.iter().zip(values.iter().skip(1)) {
        assert_eq!(left.cmp(right), big_of(left).cmp(&big_of(right)));
    }
}

fn big_of(value: &Integer) -> BigInt {
    BigInt::from_str(&value.to_string()).expect("display spelling")
}

#[test]
#[trace("TC-943", "FR-396-AC-7")]
fn unmetered_entries_agree_with_the_reserved_ones() {
    for left_text in SPELLINGS {
        let (left, left_big) = (int(left_text), big(left_text));
        for right_text in SPELLINGS {
            let (right, right_big) = (int(right_text), big(right_text));
            assert_eq!(left.add(&right).as_big(), &(&left_big + &right_big));
            assert_eq!(left.mul(&right).as_big(), &(&left_big * &right_big));
            if let Ok(divisor) = NonZeroInteger::try_from(&right) {
                let product = left.mul(&right);
                assert_eq!(product.exact_div(divisor), left);
            }
            let mut unlimited = Provision::new(usize::MAX);
            assert_eq!(
                left.add_reserved(&right, &mut unlimited)
                    .expect("unlimited"),
                left.add(&right)
            );
        }
    }
}

/// Deterministic digits: a nonzero leading digit, then a seeded sequence.
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

/// Digit counts that cross the limb-pair, schoolbook/Karatsuba and unbalanced
/// boundaries.
const SIZES: &[usize] = &[
    1, 19, 20, 38, 39, 40, 77, 100, 300, 307, 308, 309, 330, 600, 619, 1000, 1337, 2000, 3000,
];

/// Check a large call: the charge equals the allocation, an exact provision
/// admits it, and a provision one byte short of each cumulative request refuses
/// having allocated exactly the requests before it.
fn check_boundaries<T: PartialEq + core::fmt::Debug>(
    label: &str,
    expected: &T,
    call: impl Fn(&mut Recorder) -> Result<T, IntegerReserveError>,
) {
    let mut recorder = Recorder::unlimited();
    let (value, info) = measure(|| call(&mut recorder));
    assert_eq!(
        &value.unwrap_or_else(|error| panic!("{label}: {error}")),
        expected,
        "{label}"
    );
    let total: usize = recorder.requests.iter().sum();
    assert_eq!(
        info.bytes_total, total as u64,
        "{label}: charged == requested"
    );
    assert_eq!(info.count_total, recorder.requests.len() as u64, "{label}");
    let mut exact = Recorder::limited(total);
    assert_eq!(
        &call(&mut exact).expect("exact provision"),
        expected,
        "{label}"
    );
    let mut before = 0;
    for request in &recorder.requests {
        let mut short = Recorder::limited(before + request - 1);
        let (outcome, info) = measure(|| call(&mut short));
        assert_eq!(outcome.err(), Some(IntegerReserveError::Denied), "{label}");
        assert_eq!(
            info.bytes_total, before as u64,
            "{label}: refused request allocates nothing"
        );
        before += request;
    }
}

struct Recorder {
    remaining: usize,
    requests: Vec<usize>,
}

impl Recorder {
    fn unlimited() -> Self {
        Self::limited(usize::MAX)
    }

    fn limited(bytes: usize) -> Self {
        Self {
            remaining: bytes,
            requests: Vec::with_capacity(64),
        }
    }
}

impl Reservation for Recorder {
    fn reserve(&mut self, bytes: usize) -> Result<(), ReservationDenied> {
        if bytes > self.remaining {
            return Err(ReservationDenied);
        }
        self.remaining -= bytes;
        self.requests.push(bytes);
        Ok(())
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-2", "FR-396-AC-3", "FR-396-AC-8")]
fn large_sums_and_products_match_the_oracle_and_charge_every_block() {
    let mut seed = 0x1404_u64;
    for &left_digits in SIZES {
        for &right_digits in SIZES {
            let (left_text, right_text) = (
                digits(&mut seed, left_digits),
                digits(&mut seed, right_digits),
            );
            let (left, right) = (int(&left_text), int(&right_text));
            for (left, right, sign) in [
                (left.clone(), right.clone(), "+"),
                (left.neg(), right.clone(), "-"),
            ] {
                let (left_big, right_big) = (big_of(&left), big_of(&right));
                let label = format!("{left_digits}x{right_digits} {sign}");
                check_boundaries(
                    &format!("sum {label}"),
                    &Integer::from_big(&left_big + &right_big),
                    |reservation| left.add_reserved(&right, reservation),
                );
                check_boundaries(
                    &format!("product {label}"),
                    &Integer::from_big(&left_big * &right_big),
                    |reservation| left.mul_reserved(&right, reservation),
                );
            }
        }
    }
    // Long runs of equal limbs stress the carry and borrow chains.
    for exponent in [255_u32, 256, 1023, 1024, 4095, 4096] {
        let power = Integer::one().shifted_left(u64::from(exponent));
        let all_ones = power.add(&Integer::from(-1_i64));
        for (left, right) in [
            (&power, &power),
            (&all_ones, &all_ones),
            (&all_ones, &power),
        ] {
            check_boundaries(
                &format!("2^{exponent} product"),
                &Integer::from_big(big_of(left) * big_of(right)),
                |reservation| left.mul_reserved(right, reservation),
            );
        }
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-1", "FR-396-AC-8")]
fn large_parses_match_the_oracle_and_charge_every_block() {
    let mut seed = 0x1405_u64;
    for &count in SIZES.iter().chain(&[6000, 12_000]) {
        for text in [
            digits(&mut seed, count),
            "9".repeat(count),
            format!("1{}", "0".repeat(count)),
        ] {
            for spelling in [text.clone(), format!("-{text}")] {
                let expected = Integer::from_big(big(&spelling));
                check_boundaries(&format!("parse {count}"), &expected, |reservation| {
                    Integer::parse_reserved(&spelling, reservation).map_err(|error| match error {
                        IntegerParseError::Reserve(error) => error,
                        IntegerParseError::NonCanonical(_) => unreachable!("canonical"),
                    })
                });
            }
        }
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-4", "FR-396-AC-8")]
fn large_exact_quotients_match_the_oracle_and_charge_every_block() {
    let mut seed = 0x1406_u64;
    for &divisor_digits in SIZES {
        for &quotient_digits in SIZES {
            let (divisor_text, quotient_text) = (
                digits(&mut seed, divisor_digits),
                digits(&mut seed, quotient_digits),
            );
            let (divisor, quotient) = (int(&divisor_text), int(&quotient_text).neg());
            let dividend = quotient.mul(&divisor);
            let token = NonZeroInteger::try_from(&divisor).expect("nonzero");
            check_boundaries(
                &format!("quotient {quotient_digits}x{divisor_digits}"),
                &quotient,
                |reservation| dividend.exact_div_reserved(token, reservation),
            );
        }
    }
    // Divisors whose top digit is already normalized or tiny, and powers of two.
    for exponent in [64_u64, 127, 128, 129, 191, 192, 200, 1024, 4000] {
        let divisor = Integer::one().shifted_left(exponent).add(&Integer::one());
        let quotient = Integer::one()
            .shifted_left(exponent * 2 + 3)
            .add(&Integer::from(-12345_i64));
        let dividend = quotient.mul(&divisor);
        let token = NonZeroInteger::try_from(&divisor).expect("nonzero");
        check_boundaries(&format!("2^{exponent}+1"), &quotient, |reservation| {
            dividend.exact_div_reserved(token, reservation)
        });
    }
}

#[test]
#[trace("TC-943", "FR-396-AC-5", "FR-396-AC-8")]
fn large_gcds_match_the_oracle_and_charge_every_block() {
    use num_integer::Integer as _;
    let mut seed = 0x1407_u64;
    for &common_digits in &[1_usize, 20, 40, 300, 700] {
        for &part_digits in &[1_usize, 25, 41, 300, 1000] {
            let common = int(&digits(&mut seed, common_digits));
            let (left_part, right_part) = (
                int(&digits(&mut seed, part_digits)),
                int(&digits(&mut seed, part_digits + 3)),
            );
            let (left, right) = (common.mul(&left_part), common.mul(&right_part).neg());
            let expected = Integer::from_big(big_of(&left).gcd(&big_of(&right)));
            check_boundaries(
                &format!("gcd {common_digits}/{part_digits}"),
                &expected,
                |reservation| left.gcd_reserved(&right, reservation),
            );
            assert_eq!(left.gcd(&right), expected);
        }
    }
    let power = Integer::one().shifted_left(5000);
    let other = power.mul(&int("3"));
    let expected = Integer::from_big(big_of(&power).gcd(&big_of(&other)));
    check_boundaries("gcd powers of two", &expected, |reservation| {
        power.gcd_reserved(&other, reservation)
    });
}
