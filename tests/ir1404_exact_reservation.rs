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
            let _ = (left.is_zero(), left.is_negative());
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
