// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public borrowed-divisor and metered preservation evidence for FR-374.

use core::num::NonZeroU64;
use core::str::FromStr;

use quire_exact::{
    divide, modulo, ChargePoint, DivisionMember, DivisionProfile, Incomplete, InjectedDenial,
    Integer, IntegerDomain, IntegerInterval, LimitKind, Meter, NonZeroInteger, Outcome, Refusal,
    ScalarLimits, Undefined, ZeroDivisor,
};

fn int(value: i64) -> Integer {
    Integer::from(value)
}

fn literal(value: &str) -> Integer {
    Integer::from_str(value).unwrap()
}

fn limits() -> ScalarLimits {
    ScalarLimits {
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
    }
}

/// Trace: FR-374-AC-1, FR-374-AC-2
#[test]
fn admission_denies_zero_and_borrows_each_original_nonzero_integer() {
    for zero in [int(0), literal("0"), Integer::one().sub(&Integer::one())] {
        let result: Result<NonZeroInteger<'_>, ZeroDivisor> = NonZeroInteger::try_from(&zero);
        assert_eq!(result, Err(ZeroDivisor));
    }
    for value in [
        int(2),
        int(-2),
        literal("680564733841876926926749214863536422912"),
        literal("-680564733841876926926749214863536422912"),
    ] {
        let token: NonZeroInteger<'_> = NonZeroInteger::try_from(&value).unwrap();
        assert!(core::ptr::eq(token.get(), &value));
        let copy = token;
        assert_eq!(copy, token);
        assert!(core::ptr::eq(copy.get(), &value));
    }
    let divisor = int(2);
    let token = NonZeroInteger::try_from(&divisor).unwrap();
    assert_eq!(int(6).exact_div(token), int(3));
    assert_eq!(int(7).div_rem_truncating(token), (int(3), int(1)));
    assert_eq!(int(-7).div_mod_floor(token), (int(-4), int(1)));
}

/// Trace: FR-374-AC-3
#[test]
fn admitted_pair_helpers_preserve_signed_zero_and_large_results() {
    for (a, b, truncating, floor) in [
        (7, 2, (3, 1), (3, 1)),
        (7, -2, (-3, 1), (-4, -1)),
        (-7, 2, (-3, -1), (-4, 1)),
        (-7, -2, (3, -1), (3, -1)),
        (0, 2, (0, 0), (0, 0)),
        (0, -2, (0, 0), (0, 0)),
    ] {
        let (dividend, divisor) = (int(a), int(b));
        let token = NonZeroInteger::try_from(&divisor).unwrap();
        assert_eq!(
            dividend.div_rem_truncating(token),
            (int(truncating.0), int(truncating.1))
        );
        assert_eq!(dividend.div_mod_floor(token), (int(floor.0), int(floor.1)));
    }
    let dividend = literal("680564733841876926926749214863536422912");
    for (b, quotient) in [
        (2, "340282366920938463463374607431768211456"),
        (-2, "-340282366920938463463374607431768211456"),
    ] {
        let divisor = int(b);
        let token = NonZeroInteger::try_from(&divisor).unwrap();
        assert_eq!(
            dividend.div_rem_truncating(token),
            (literal(quotient), int(0))
        );
        assert_eq!(dividend.div_mod_floor(token), (literal(quotient), int(0)));
    }
}

/// Trace: FR-374-AC-4
#[test]
fn exact_div_keeps_infallible_signed_results_and_nondivisible_truncation() {
    for (a, b, quotient) in [
        (6, 2, 3),
        (6, -2, -3),
        (-6, 2, -3),
        (-6, -2, 3),
        (0, 2, 0),
        (0, -2, 0),
        (5, 2, 2),
        (-5, 2, -2),
    ] {
        let divisor = int(b);
        let token = NonZeroInteger::try_from(&divisor).unwrap();
        let result: Integer = int(a).exact_div(token);
        assert_eq!(result, int(quotient));
    }
    let dividend = literal("680564733841876926926749214863536422912");
    for (b, quotient) in [
        (2, "340282366920938463463374607431768211456"),
        (-2, "-340282366920938463463374607431768211456"),
    ] {
        let divisor = int(b);
        assert_eq!(
            dividend.exact_div(NonZeroInteger::try_from(&divisor).unwrap()),
            literal(quotient)
        );
    }
}

#[derive(Clone, Copy)]
enum Evaluation {
    Division(DivisionProfile, DivisionMember),
    Modulo,
}

impl Evaluation {
    fn operands(self) -> ChargePoint {
        match self {
            Self::Division(_, _) => ChargePoint::IntegerDivisionOperands,
            Self::Modulo => ChargePoint::IntegerModulusOperands,
        }
    }

    fn run(self, a: i64, b: i64, domain: &IntegerDomain, meter: &mut Meter) -> Outcome<Integer> {
        match self {
            Self::Division(profile, member) => {
                divide(profile, member, &int(a), &int(b), domain, meter)
            }
            Self::Modulo => modulo(&int(a), &int(b), domain, meter),
        }
    }
}

fn assert_counters(
    meter: &Meter,
    bits: u64,
    occurrences: u64,
    work: u64,
    results: u64,
    count: u64,
) {
    for kind in LimitKind::ALL {
        let expected = match kind {
            LimitKind::IntegerBits => bits,
            LimitKind::ValueOccurrences => occurrences,
            LimitKind::WorkUnits => work,
            LimitKind::ResultUnits => results,
            _ => 0,
        };
        assert_eq!(meter.consumed(kind), expected, "{kind:?}");
    }
    assert_eq!(meter.admission_count(), count);
}

#[cfg(feature = "test-support")]
fn assert_prefix(meter: &Meter, prefix: &[(ChargePoint, Option<u64>, Option<u64>)]) {
    use quire_exact::AdmittedCharge;
    let expected: Vec<_> = prefix
        .iter()
        .map(|&(point, integer_bits, value_occurrences)| AdmittedCharge {
            point,
            integer_bits,
            value_occurrences,
            decimal_digits: None,
            scale_expansion: None,
            text_input_bytes: None,
            text_scalars: None,
            normalized_scalars: None,
            unit_edges: None,
        })
        .collect();
    assert_eq!(meter.admitted_charges(), expected);
    assert!(!meter.charge_log_truncated());
}

/// Trace: FR-374-AC-6
#[test]
fn metered_zero_stops_after_operands_and_denial_precedes_zero_classification() {
    let evaluations = DivisionProfile::ALL
        .into_iter()
        .flat_map(|profile| {
            [DivisionMember::Quotient, DivisionMember::Remainder]
                .map(|member| Evaluation::Division(profile, member))
        })
        .chain([Evaluation::Modulo]);
    for evaluation in evaluations {
        let point = evaluation.operands();
        let mut meter = Meter::new(limits());
        assert_eq!(
            evaluation.run(1, 0, &IntegerDomain::Mathematical, &mut meter),
            Outcome::Undefined(Undefined::DivisionByZero)
        );
        assert_counters(&meter, 1, 2, 1, 0, 1);
        #[cfg(feature = "test-support")]
        assert_prefix(&meter, &[(point, Some(1), Some(2))]);

        let mut low = limits();
        low.work_units = 0;
        for mut denied in [
            Meter::new(low),
            Meter::new(limits()).with_injected_denial(InjectedDenial {
                point,
                occurrence: NonZeroU64::new(1).unwrap(),
            }),
        ] {
            assert_eq!(
                evaluation.run(1, 0, &IntegerDomain::Mathematical, &mut denied),
                Outcome::Incomplete(Incomplete {
                    limit_kind: LimitKind::WorkUnits,
                    limit: 0,
                    consumed: 0,
                    next_charge: Integer::one(),
                    charge_point: point,
                })
            );
            assert_counters(&denied, 0, 0, 0, 0, 0);
            #[cfg(feature = "test-support")]
            assert_prefix(&denied, &[]);
        }
    }
}

/// Trace: FR-374-AC-6
#[test]
fn metered_nonzero_retains_laws_single_member_domain_and_charge_requests() {
    #[cfg(feature = "test-support")]
    use ChargePoint::{
        IntegerDivisionArithmetic as Arithmetic, IntegerDivisionDomain as Domain,
        IntegerDivisionOperands as Operands, IntegerDivisionResultRetain as Retain,
    };
    let bounded = IntegerDomain::Bounded(IntegerInterval::new(int(1), int(10)).unwrap());
    for (profile, a, b, q, r) in [
        (DivisionProfile::Truncating, -7, 2, -3, -1),
        (DivisionProfile::Floor, -7, 2, -4, 1),
        (DivisionProfile::Euclidean, -7, 2, -4, 1),
        (DivisionProfile::Truncating, -7, -2, 3, -1),
        (DivisionProfile::Floor, -7, -2, 3, -1),
        (DivisionProfile::Euclidean, -7, -2, 4, 1),
    ] {
        for (member, expected) in [
            (DivisionMember::Quotient, q),
            (DivisionMember::Remainder, r),
        ] {
            let mut meter = Meter::new(limits());
            assert_eq!(
                divide(
                    profile,
                    member,
                    &int(a),
                    &int(b),
                    &IntegerDomain::Mathematical,
                    &mut meter
                ),
                Outcome::Completed(int(expected))
            );
            assert_counters(&meter, 3, 2, 4, 1, 4);
            #[cfg(feature = "test-support")]
            assert_prefix(
                &meter,
                &[
                    (Operands, Some(3), Some(2)),
                    (Arithmetic, Some(3), None),
                    (Domain, None, Some(1)),
                    (Retain, None, None),
                ],
            );
        }
    }
    for profile in DivisionProfile::ALL {
        let mut meter = Meter::new(limits());
        assert_eq!(
            divide(
                profile,
                DivisionMember::Quotient,
                &int(10),
                &int(5),
                &bounded,
                &mut meter
            ),
            Outcome::Completed(int(2))
        );
        assert_counters(&meter, 4, 2, 4, 1, 4);
        #[cfg(feature = "test-support")]
        assert_prefix(
            &meter,
            &[
                (Operands, Some(4), Some(2)),
                (Arithmetic, Some(4), None),
                (Domain, None, Some(1)),
                (Retain, None, None),
            ],
        );

        let interval = IntegerInterval::new(int(0), int(5)).unwrap();
        let domain = IntegerDomain::Bounded(interval.clone());
        let mut meter = Meter::new(limits());
        assert_eq!(
            divide(
                profile,
                DivisionMember::Quotient,
                &int(10),
                &int(1),
                &domain,
                &mut meter
            ),
            Outcome::Refused(Refusal::DivisionOutOfDomain {
                domain: Box::new(interval),
                member: DivisionMember::Quotient,
            })
        );
        assert_counters(&meter, 4, 2, 3, 0, 3);
        #[cfg(feature = "test-support")]
        assert_prefix(
            &meter,
            &[
                (Operands, Some(4), Some(2)),
                (Arithmetic, Some(4), None),
                (Domain, None, Some(1)),
            ],
        );
        let mut meter = Meter::new(limits());
        assert_eq!(
            divide(
                profile,
                DivisionMember::Remainder,
                &int(10),
                &int(1),
                &domain,
                &mut meter
            ),
            Outcome::Completed(int(0))
        );
        assert_counters(&meter, 4, 2, 4, 1, 4);
        #[cfg(feature = "test-support")]
        assert_prefix(
            &meter,
            &[
                (Operands, Some(4), Some(2)),
                (Arithmetic, Some(4), None),
                (Domain, None, Some(1)),
                (Retain, None, None),
            ],
        );
    }
    let mut meter = Meter::new(limits());
    assert_eq!(
        modulo(&int(-7), &int(-3), &IntegerDomain::Mathematical, &mut meter),
        Outcome::Completed(int(2))
    );
    assert_counters(&meter, 3, 2, 4, 1, 4);
    #[cfg(feature = "test-support")]
    assert_prefix(
        &meter,
        &[
            (ChargePoint::IntegerModulusOperands, Some(3), Some(2)),
            (ChargePoint::IntegerModulusArithmetic, Some(3), None),
            (ChargePoint::IntegerModulusDomain, None, Some(1)),
            (ChargePoint::IntegerModulusResultRetain, None, None),
        ],
    );
}
