// SPDX-License-Identifier: AGPL-3.0-or-later
//! Integer division: the three `div`/`rem` laws, independent Euclidean
//! `mod`, and single-member admission (the domain applies only to the member
//! the expression exposes) with the named integer-division and
//! integer-modulus charges.
//!
//! Whether a *backend* can execute a division item at all is decided ahead of
//! and independent of evaluation; that capability negotiation is not a kernel
//! operation and is not here. [`divide`] takes the selected [`DivisionProfile`]
//! directly.

use crate::accounting::{Charge, ChargePoint, LimitKind, Meter};
use crate::integer::{Integer, IntegerDomain, IntegerInterval};
use crate::outcome::{Outcome, Refusal, Stop, Undefined};
use alloc::boxed::Box;

/// A selectable `div`/`rem` law.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DivisionProfile {
    /// `quire.value.integer-division.truncating/v1`.
    Truncating,
    /// `quire.value.integer-division.floor/v1`.
    Floor,
    /// `quire.value.integer-division.euclidean/v1`.
    Euclidean,
}

impl DivisionProfile {
    /// Every law.
    pub const ALL: [Self; 3] = [Self::Truncating, Self::Floor, Self::Euclidean];

    /// The exact definition identity that selects this law.
    pub fn definition_identity(self) -> &'static str {
        match self {
            Self::Truncating => "quire.value.integer-division.truncating/v1",
            Self::Floor => "quire.value.integer-division.floor/v1",
            Self::Euclidean => "quire.value.integer-division.euclidean/v1",
        }
    }

    /// The unique `(q, r)` with `a = b*q + r` under this law; `b` is nonzero.
    fn apply(self, dividend: &Integer, divisor: &Integer) -> (Integer, Integer) {
        match self {
            Self::Truncating => dividend.div_rem_truncating(divisor),
            Self::Floor => dividend.div_mod_floor(divisor),
            Self::Euclidean => {
                let (quotient, remainder) = dividend.div_mod_floor(divisor);
                if remainder.is_negative() {
                    // Only a negative divisor yields a negative floor remainder.
                    (quotient.add(&Integer::one()), remainder.sub(divisor))
                } else {
                    (quotient, remainder)
                }
            }
        }
    }
}

/// The one member of the `(q, r)` pair an expression exposes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DivisionMember {
    /// The quotient, exposed by `div`.
    Quotient,
    /// The remainder, exposed by `rem`.
    Remainder,
}

impl DivisionMember {
    fn select(self, quotient: Integer, remainder: Integer) -> Integer {
        match self {
            Self::Quotient => quotient,
            Self::Remainder => remainder,
        }
    }
}

/// Evaluate one member of `div`/`rem` under the selected law. Both members
/// are computed exactly; only the exposed `member` must be in `domain`.
pub fn divide(
    profile: DivisionProfile,
    member: DivisionMember,
    dividend: &Integer,
    divisor: &Integer,
    domain: &IntegerDomain,
    meter: &mut Meter,
) -> Outcome<Integer> {
    Outcome::from_stop(member_of(profile, member, dividend, divisor, domain, meter))
}

/// Evaluate `mod`: always the Euclidean remainder, independent of any
/// selected `div`/`rem` law, charged only at the four `integer-modulus.*`
/// points.
pub fn modulo(
    dividend: &Integer,
    divisor: &Integer,
    domain: &IntegerDomain,
    meter: &mut Meter,
) -> Outcome<Integer> {
    Outcome::from_stop(euclidean_remainder(dividend, divisor, domain, meter))
}

fn operand_bits(dividend: &Integer, divisor: &Integer) -> u64 {
    dividend.magnitude_bits().max(divisor.magnitude_bits())
}

/// The `integer-division.arithmetic` and `integer-modulus.arithmetic`
/// amount: `max(bits(a),bits(b))`, which bounds both quotient and
/// remainder.
fn arithmetic_bits(dividend: &Integer, divisor: &Integer) -> u64 {
    operand_bits(dividend, divisor)
}

fn reject_zero_divisor(divisor: &Integer) -> Result<(), Stop> {
    if divisor.is_zero() {
        Err(Stop::Undefined(Undefined::DivisionByZero))
    } else {
        Ok(())
    }
}

/// The bounded consumer interval a failed membership decision was made
/// against. Only a bounded domain can refuse a member, so a mathematical
/// domain here is a checked-program invariant failure.
fn refused_interval(domain: &IntegerDomain) -> Result<Box<IntegerInterval>, Stop> {
    match domain {
        IntegerDomain::Bounded(interval) => Ok(Box::new(interval.clone())),
        IntegerDomain::Mathematical => Err(Stop::Refused(Refusal::CheckedInvariant)),
    }
}

/// Charge, compute and admit the exposed member. Membership is decided after
/// `integer-division.domain` and before `integer-division.result-retain`.
fn member_of(
    profile: DivisionProfile,
    member: DivisionMember,
    dividend: &Integer,
    divisor: &Integer,
    domain: &IntegerDomain,
    meter: &mut Meter,
) -> Result<Integer, Stop> {
    meter.charge(
        Charge::new(ChargePoint::IntegerDivisionOperands)
            .size(LimitKind::IntegerBits, operand_bits(dividend, divisor))
            .size(LimitKind::ValueOccurrences, 2),
    )?;
    reject_zero_divisor(divisor)?;
    meter.charge(
        Charge::new(ChargePoint::IntegerDivisionArithmetic)
            .size(LimitKind::IntegerBits, arithmetic_bits(dividend, divisor)),
    )?;
    let (quotient, remainder) = profile.apply(dividend, divisor);
    let exposed = member.select(quotient, remainder);
    meter.charge(
        Charge::new(ChargePoint::IntegerDivisionDomain).size(LimitKind::ValueOccurrences, 1),
    )?;
    if !domain.contains(&exposed) {
        return Err(Stop::Refused(Refusal::DivisionOutOfDomain {
            domain: refused_interval(domain)?,
            member,
        }));
    }
    meter.charge(Charge::new(ChargePoint::IntegerDivisionResultRetain).results(1))?;
    Ok(exposed)
}

fn euclidean_remainder(
    dividend: &Integer,
    divisor: &Integer,
    domain: &IntegerDomain,
    meter: &mut Meter,
) -> Result<Integer, Stop> {
    meter.charge(
        Charge::new(ChargePoint::IntegerModulusOperands)
            .size(LimitKind::IntegerBits, operand_bits(dividend, divisor))
            .size(LimitKind::ValueOccurrences, 2),
    )?;
    reject_zero_divisor(divisor)?;
    meter.charge(
        Charge::new(ChargePoint::IntegerModulusArithmetic)
            .size(LimitKind::IntegerBits, arithmetic_bits(dividend, divisor)),
    )?;
    let (_, remainder) = DivisionProfile::Euclidean.apply(dividend, divisor);
    meter.charge(
        Charge::new(ChargePoint::IntegerModulusDomain).size(LimitKind::ValueOccurrences, 1),
    )?;
    if !domain.contains(&remainder) {
        return Err(Stop::Refused(Refusal::ModuloOutOfDomain {
            domain: refused_interval(domain)?,
        }));
    }
    meter.charge(Charge::new(ChargePoint::IntegerModulusResultRetain).results(1))?;
    Ok(remainder)
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::*;
    use crate::accounting::ScalarLimits;

    fn generous_meter() -> Meter {
        Meter::new(ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        })
    }

    fn int(value: i64) -> Integer {
        Integer::from(value)
    }

    fn range(low: i64, high: i64) -> IntegerDomain {
        IntegerDomain::Bounded(IntegerInterval::new(int(low), int(high)).unwrap())
    }

    fn run(
        profile: DivisionProfile,
        member: DivisionMember,
        a: i64,
        b: i64,
        domain: &IntegerDomain,
    ) -> Outcome<Integer> {
        divide(
            profile,
            member,
            &int(a),
            &int(b),
            domain,
            &mut generous_meter(),
        )
    }

    /// dividing by zero under any law and either member is undefined, never a panic.
    ///
    /// Also QSpec FR-147-AC-2 ("Division by zero is undefined and produces
    /// no numeric value"), verified through central `QSpec-TC-192`
    /// (`agent-ix/quire-specification`): this is the kernel-level instance
    /// of that requirement.
    #[trace("QSpec-TC-192", "QSpec-FR-147-AC-2", "TC-905", "FR-357-AC-1")]
    #[test]
    fn division_by_zero_is_undefined() {
        let domain = IntegerDomain::Mathematical;
        for profile in DivisionProfile::ALL {
            for member in [DivisionMember::Quotient, DivisionMember::Remainder] {
                assert!(matches!(
                    run(profile, member, 1, 0, &domain),
                    Outcome::Undefined(Undefined::DivisionByZero)
                ));
            }
        }
    }

    /// `10 div y` over `1..=10` at `y = 5` is 2; `x % -1` over `-10..=5` at
    /// `x = -10` is 0 even though the quotient (10) is outside that domain.
    #[trace("TC-905", "FR-357-AC-2")]
    #[test]
    fn only_the_exposed_member_must_be_in_domain() {
        for profile in DivisionProfile::ALL {
            let q = run(profile, DivisionMember::Quotient, 10, 5, &range(1, 10));
            assert_eq!(q.completed(), Some(int(2)));
            let r = run(profile, DivisionMember::Remainder, -10, -1, &range(-10, 5));
            assert_eq!(r.completed(), Some(int(0)));
        }
    }

    /// A quotient outside the domain refuses with `quotient-outside-domain`,
    /// a remainder outside refuses with `remainder-outside-domain`, under
    /// every profile.
    #[trace("TC-905", "FR-357-AC-3")]
    #[test]
    fn exposed_member_outside_domain_refuses_with_its_cause() {
        for profile in DivisionProfile::ALL {
            let Outcome::Refused(refusal) =
                run(profile, DivisionMember::Quotient, 10, 1, &range(0, 5))
            else {
                panic!("quotient 10 is outside 0..=5");
            };
            assert_eq!(refusal.code(), Some("division_out_of_domain"));
            assert_eq!(refusal.cause(), Some("quotient-outside-domain"));
            let Outcome::Refused(refusal) =
                run(profile, DivisionMember::Remainder, 9, 5, &range(0, 3))
            else {
                panic!("remainder 4 is outside 0..=3");
            };
            assert_eq!(refusal.code(), Some("division_out_of_domain"));
            assert_eq!(refusal.cause(), Some("remainder-outside-domain"));
        }
    }

    /// The three laws differ on negative operands exactly as defined, for -7
    /// and 2 (and -7 and -2 for Euclidean).
    #[trace("TC-905", "FR-357-AC-4")]
    #[test]
    fn profiles_differ_on_negative_operands() {
        let domain = IntegerDomain::Mathematical;
        let pair = |profile, a, b| {
            let q = run(profile, DivisionMember::Quotient, a, b, &domain);
            let r = run(profile, DivisionMember::Remainder, a, b, &domain);
            (q.completed().unwrap(), r.completed().unwrap())
        };
        assert_eq!(pair(DivisionProfile::Truncating, -7, 2), (int(-3), int(-1)));
        assert_eq!(pair(DivisionProfile::Floor, -7, 2), (int(-4), int(1)));
        assert_eq!(pair(DivisionProfile::Euclidean, -7, 2), (int(-4), int(1)));
        assert_eq!(pair(DivisionProfile::Truncating, -7, -2), (int(3), int(-1)));
        assert_eq!(pair(DivisionProfile::Floor, -7, -2), (int(3), int(-1)));
        assert_eq!(pair(DivisionProfile::Euclidean, -7, -2), (int(4), int(1)));
    }

    /// `i64::MIN div -1` is exact (2^63): a domain that holds it admits it. In
    /// a signed-64 domain the quotient refuses with `quotient-outside-domain`
    /// while the remainder, 0, completes.
    #[trace("TC-905", "FR-357-AC-5")]
    #[test]
    fn i64_min_divided_by_minus_one_is_exact() {
        let wide = IntegerDomain::Bounded(
            IntegerInterval::new(int(0), Integer::from(i128::from(i64::MAX) + 1)).unwrap(),
        );
        let signed_64 = range(i64::MIN, i64::MAX);
        for profile in DivisionProfile::ALL {
            let q = run(profile, DivisionMember::Quotient, i64::MIN, -1, &wide);
            assert_eq!(q.completed(), Some(Integer::from(1_i128 << 63)));
            let Outcome::Refused(refusal) =
                run(profile, DivisionMember::Quotient, i64::MIN, -1, &signed_64)
            else {
                panic!("quotient 2^63 is outside signed 64-bit");
            };
            assert_eq!(refusal.code(), Some("division_out_of_domain"));
            assert_eq!(refusal.cause(), Some("quotient-outside-domain"));
            let r = run(profile, DivisionMember::Remainder, i64::MIN, -1, &signed_64);
            assert_eq!(r.completed(), Some(int(0)));
        }
    }

    /// The admitted charge sequence is operands, arithmetic, domain, then
    /// result-retain for one result unit. A refusal never reaches
    /// result-retain, a zero divisor stops after operands, and a result limit
    /// one too small reports `integer-division.result-retain`.
    #[cfg(feature = "test-support")]
    #[trace("TC-905", "FR-357-AC-6")]
    #[test]
    fn division_charges_one_domain_occurrence_and_one_result_unit() {
        use crate::accounting::ChargePoint::{
            IntegerDivisionArithmetic as Arithmetic, IntegerDivisionDomain as Domain,
            IntegerDivisionOperands as Operands, IntegerDivisionResultRetain as Retain,
        };
        let limited = |result_units| {
            let mut limits = *generous_meter().limits();
            limits.result_units = result_units;
            Meter::new(limits)
        };
        for member in [DivisionMember::Quotient, DivisionMember::Remainder] {
            let mut meter = limited(1);
            let out = divide(
                DivisionProfile::Floor,
                member,
                &int(7),
                &int(2),
                &range(0, 9),
                &mut meter,
            );
            assert!(out.completed().is_some());
            assert_eq!(
                meter.admitted_charges(),
                [Operands, Arithmetic, Domain, Retain]
            );
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);

            let mut meter = limited(1);
            let out = divide(
                DivisionProfile::Floor,
                member,
                &int(7),
                &int(2),
                &range(5, 5),
                &mut meter,
            );
            assert!(matches!(out, Outcome::Refused(_)));
            assert_eq!(meter.admitted_charges(), [Operands, Arithmetic, Domain]);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);

            let mut meter = limited(1);
            let out = divide(
                DivisionProfile::Floor,
                member,
                &int(7),
                &int(0),
                &range(0, 9),
                &mut meter,
            );
            assert!(matches!(out, Outcome::Undefined(_)));
            assert_eq!(meter.admitted_charges(), [Operands]);

            let mut meter = limited(0);
            let out = divide(
                DivisionProfile::Floor,
                member,
                &int(7),
                &int(2),
                &range(0, 9),
                &mut meter,
            );
            let Outcome::Incomplete(incomplete) = out else {
                panic!("a result limit of 0 cannot retain the member");
            };
            assert_eq!(incomplete.charge_point, Retain);
        }
    }

    /// Euclidean `mod` never returns a negative remainder for a
    /// negative dividend, unlike truncating `rem`.
    #[test]
    fn euclidean_modulo_is_nonnegative() {
        let domain = IntegerDomain::Mathematical;
        let mut meter = generous_meter();
        let dividend = Integer::zero().sub(&Integer::from(7_u64));
        let divisor = Integer::from(3_u64);
        let outcome = modulo(&dividend, &divisor, &domain, &mut meter);
        let remainder = outcome
            .completed()
            .expect("euclidean mod is total for a nonzero divisor");
        assert!(!remainder.is_negative());
    }
}
