// SPDX-License-Identifier: AGPL-3.0-or-later
//! Kernel outcomes: `Outcome<T>`, `Undefined`, `Refusal`, `Stop`.
//!
//! A false Boolean is a completed value, never a refusal. `requires-bound`
//! and `unsupported` are per-item provider dispositions, not evaluator
//! outcomes, so they have no variant here.
//!
//! The kernel `Refusal` carries only its own typed cause. A cause whose payload
//! is not a kernel type has no variant here:
//!
//! - a closed cause set for a `pre(..)` anchor mismatch is an evaluator concept,
//!   not a kernel one.
//! - a model-query refusal carries the caller's diagnostic catalog code; the
//!   catalog code and category stay out of the kernel, which names its own
//!   codes.
//!
//! The category-mapping table and the family outcome and result types that union
//! several evaluators' outcomes into one reported shape are layered on top of
//! this and are not kernel.
//!
//! `Undefined::PreconditionFalse` has no variant here. Choosing among several
//! redefinition candidates by the receiver's most-specific runtime type is family
//! dispatch, and family causes are never kernel causes: resolving *which*
//! candidate linked, and reporting that its precondition evaluated false, is the
//! caller's concept, layered on top of this module the same way the
//! category-mapping table above is. Retyping the selected candidate to
//! `EffectiveId` would still leave a dispatch-resolution cause in the kernel's
//! closed `Undefined` set, so the variant is omitted, not retyped.

use crate::accounting::Incomplete;
use crate::collection::{CardinalityBound, CollectionKind};
use crate::decimal::DecimalType;
use crate::identity::UniverseId;
use crate::ieee::{IeeeFlags, IeeeWidth};
use crate::integer::IntegerInterval;
use crate::rational::RationalDomain;
use crate::text::TextType;
use alloc::boxed::Box;

/// Exactly one of a completed value, undefined, refused or incomplete.
///
/// quire:canonical
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use]
pub enum Outcome<T> {
    /// A completed value. `T` itself carries any typed loss where the
    /// operation has one (e.g. `DecimalResult::loss`); `Outcome` does not
    /// separately carry [`crate::Location`]/[`crate::Origin`] provenance -- nothing in this crate
    /// wires the two together: a caller that wants a completed value's
    /// occurrence provenance holds it itself, the way a call locus is
    /// already the caller's own concept.
    Completed(T),
    /// The operation has no mathematical value.
    Undefined(Undefined),
    /// The operation is defined but its result is not admitted.
    Refused(Refusal),
    /// A named charge was unavailable; no partial value exists.
    Incomplete(Incomplete),
}

impl<T> Outcome<T> {
    /// The completed value, if any.
    pub fn completed(self) -> Option<T> {
        match self {
            Self::Completed(value) => Some(value),
            Self::Undefined(_) | Self::Refused(_) | Self::Incomplete(_) => None,
        }
    }
}

impl<T> Outcome<T> {
    pub(crate) fn from_stop(result: Result<T, Stop>) -> Self {
        match result {
            Ok(value) => Self::Completed(value),
            Err(Stop::Undefined(reason)) => Self::Undefined(reason),
            Err(Stop::Refused(reason)) => Self::Refused(reason),
            Err(Stop::Incomplete(record)) => Self::Incomplete(record),
        }
    }

    pub(crate) fn into_stop(self) -> Result<T, Stop> {
        match self {
            Self::Completed(value) => Ok(value),
            Self::Undefined(reason) => Err(Stop::Undefined(reason)),
            Self::Refused(reason) => Err(Stop::Refused(reason)),
            Self::Incomplete(record) => Err(Stop::Incomplete(record)),
        }
    }
}

/// Why an operation is undefined.
///
/// quire:canonical
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Undefined {
    /// A divisor is (normalized) zero.
    DivisionByZero,
    /// An IEEE NaN or infinity has no exact value.
    IeeeNotFinite,
    /// `reduce` over an empty collection has no value. Only direct kernel
    /// evaluation of an unlinked expression can meet it.
    EmptyReduction,
    /// `value(e)` of `none`. Only direct kernel evaluation of an unlinked
    /// expression can meet it.
    NoneValue,
    /// A `sum<N>` seed or running total is not a member of `N`'s domain, so
    /// the fold has no value in `N` (QSpec FR-145). It names no catalog
    /// undefined reason and builds no record. Only a `sum` checked under
    /// `CheckMode::Kernel` can meet it.
    SumOutOfDomain,
}

/// Why a defined result is refused. Refusals never carry the refused value.
///
/// Each of the ten value refusals carries the declared target domain or IEEE
/// width its catalog record renders, so the record is built
/// from the variant and never from a message. Bigint domains are boxed, which
/// keeps `Refusal` small; it is `Clone`, not `Copy`.
///
/// quire:canonical
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Refusal {
    /// Strict `exact` rounding would discard a nonzero digit.
    InexactDecimal {
        /// The declared target the value was placed at.
        target: InexactTarget,
    },
    /// The normalized decimal coefficient is outside the target domain.
    DecimalOutOfDomain {
        /// The declared `Decimal[..]` target.
        target: Box<DecimalType>,
    },
    /// At least one member of a quotient/remainder pair is outside the
    /// consumer domain; neither member is exposed.
    DivisionPairOutOfDomain {
        /// The bounded consumer's `Int[..]` domain.
        domain: Box<IntegerInterval>,
        /// Whether the quotient is a domain member.
        quotient_admitted: bool,
        /// Whether the remainder is a domain member.
        remainder_admitted: bool,
    },
    /// The Euclidean `mod` remainder is outside the consumer domain.
    ModuloOutOfDomain {
        /// The bounded consumer's `Int[..]` domain.
        domain: Box<IntegerInterval>,
    },
    /// The profile length (scalars, or bytes for `binary-utf8`) is outside
    /// the declared `Text[min,max; profile]` bounds.
    TextLengthOutOfDomain {
        /// The declared text type.
        target: TextType,
    },
    /// An exact conversion or arithmetic result is outside the target
    /// integer domain.
    IntegerOutOfDomain {
        /// The target `Int[..]` domain.
        target: Box<IntegerInterval>,
    },
    /// An exact rational arithmetic result is outside its `Rational[..]`
    /// result domain.
    RationalOutOfDomain {
        /// The `Rational[..]` result domain.
        target: Box<RationalDomain>,
    },
    /// Strict IEEE `exact` found an inexact, overflowing or tiny-and-inexact
    /// result; only its would-be flags are reported, never rounded bits.
    IeeeNotExact {
        /// The result width.
        target: IeeeWidth,
        /// The flags the rounded result would have raised.
        would_be: IeeeFlags,
    },
    /// A NaN payload does not fit the explicit conversion's target width.
    IeeeNanPayloadNotRepresentable {
        /// The conversion's target width.
        target: IeeeWidth,
        /// The conversion's source width.
        source: IeeeWidth,
    },
    /// An exact rational converted from an IEEE value is outside the
    /// `Rational[..]` target domain.
    IeeeRationalOutOfDomain {
        /// The grammar-named `Rational[..]` conversion target.
        target: Box<RationalDomain>,
    },
    /// A comparison met two references of different universes: code
    /// `foreign_reference`, cause `foreign-universe`. `required` is the universe
    /// already in force, `supplied` the one tested against it: for a bare
    /// equality (`plan_pairs(left, right)`, `equality.rs`), that is the left
    /// operand's universe and the right's, since equality has no "binding"
    /// side and the raise site's operand order settles which is which; for
    /// membership (`collection.rs`'s `member_equal_stop`, both `Contains`
    /// and collection construction's dedup), that is the already-retained
    /// member's or collection's own universe, not the probed candidate's.
    ForeignReference {
        /// The universe already in force.
        required: UniverseId,
        /// The universe tested against it.
        supplied: UniverseId,
    },
    /// A formed collection's bound count is outside its declared bound; no
    /// collection is materialized.
    CardinalityOutOfBound {
        /// Which side of the bound is violated.
        violation: BoundViolation,
        /// The collection kind of the declared type.
        kind: CollectionKind,
        /// The declared inclusive bound.
        bound: CardinalityBound,
        /// The formed bound count: occurrences for a sequence or bag,
        /// members for a set or ordered set.
        count: u64,
    },
    /// A checked-program invariant failed during evaluation; unreachable
    /// for an admitted program.
    CheckedInvariant,
}

impl Refusal {
    /// The catalog `refused { code }` spelling, `None` only for
    /// `CheckedInvariant`, which is an internal fault and never a refusal
    /// record. The kernel names its own codes; a caller maps them to
    /// its own catalog.
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::InexactDecimal { .. } => Some("inexact_decimal"),
            Self::DecimalOutOfDomain { .. } => Some("decimal_out_of_domain"),
            Self::DivisionPairOutOfDomain { .. } => Some("division_pair_out_of_domain"),
            Self::ModuloOutOfDomain { .. } => Some("modulo_out_of_domain"),
            Self::TextLengthOutOfDomain { .. } => Some("text_length_out_of_domain"),
            Self::IntegerOutOfDomain { .. } => Some("integer_out_of_domain"),
            Self::RationalOutOfDomain { .. } => Some("rational_out_of_domain"),
            Self::IeeeNotExact { .. } => Some("ieee_not_exact"),
            Self::IeeeNanPayloadNotRepresentable { .. } => {
                Some("ieee_nan_payload_not_representable")
            }
            Self::IeeeRationalOutOfDomain { .. } => Some("ieee_rational_out_of_domain"),
            Self::ForeignReference { .. } => Some("foreign_reference"),
            Self::CardinalityOutOfBound { .. } => Some("cardinality_out_of_bound"),
            Self::CheckedInvariant => None,
        }
    }

    /// The closed cause tag its code's catalog row gives, `None` only for
    /// `CheckedInvariant`.
    pub fn cause(&self) -> Option<&'static str> {
        match self {
            Self::InexactDecimal { .. } => Some("nonzero-discarded-digit"),
            Self::DecimalOutOfDomain { .. }
            | Self::ModuloOutOfDomain { .. }
            | Self::TextLengthOutOfDomain { .. }
            | Self::IntegerOutOfDomain { .. }
            | Self::RationalOutOfDomain { .. }
            | Self::IeeeRationalOutOfDomain { .. } => Some("outside-domain"),
            Self::DivisionPairOutOfDomain {
                quotient_admitted,
                remainder_admitted,
                ..
            } => Some(match (quotient_admitted, remainder_admitted) {
                (false, true) => "quotient-outside-domain",
                (true, false) => "remainder-outside-domain",
                // The kernel raises the refusal only when a member is
                // outside, so `(true, true)` cannot occur; it reads as the
                // widest cause rather than inventing a fourth.
                (false, false) | (true, true) => "both-outside-domain",
            }),
            Self::IeeeNotExact { .. } => Some("rounding-required"),
            Self::IeeeNanPayloadNotRepresentable { .. } => Some("payload-exceeds-target"),
            Self::ForeignReference { .. } => Some("foreign-universe"),
            Self::CardinalityOutOfBound { violation, .. } => Some(violation.as_str()),
            Self::CheckedInvariant => None,
        }
    }
}

/// The declared target an inexact decimal placement was refused at.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InexactTarget {
    /// A `Decimal[lo, hi; smin, smax]` target.
    Decimal(Box<DecimalType>),
    /// An integer target (scale zero), rendered as its declared `Int[lo, hi]`.
    Integer(Box<IntegerInterval>),
}

/// The side of a cardinality bound a formed collection violates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BoundViolation {
    /// `below-minimum`.
    BelowMinimum,
    /// `above-maximum`.
    AboveMaximum,
}

impl BoundViolation {
    /// The cause tag.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BelowMinimum => "below-minimum",
            Self::AboveMaximum => "above-maximum",
        }
    }
}

/// Internal early-exit carrier converted into [`Outcome`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Stop {
    Undefined(Undefined),
    Refused(Refusal),
    Incomplete(Incomplete),
}

impl From<Incomplete> for Stop {
    fn from(record: Incomplete) -> Self {
        Self::Incomplete(record)
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::*;

    /// `Outcome::completed` returns the
    /// value for `Completed` and `None` for every other variant --
    /// `Undefined`, `Refused` and `Incomplete` are each exercised, not just
    /// `Undefined` as before.
    #[test]
    fn completed_extracts_only_the_completed_variant() {
        use crate::accounting::{ChargePoint, Incomplete, LimitKind};
        use crate::integer::Integer;

        assert_eq!(Outcome::Completed(1).completed(), Some(1));
        assert_eq!(
            Outcome::<i32>::Undefined(Undefined::DivisionByZero).completed(),
            None
        );
        assert_eq!(
            Outcome::<i32>::Refused(Refusal::CheckedInvariant).completed(),
            None
        );
        assert_eq!(
            Outcome::<i32>::Incomplete(Incomplete {
                limit_kind: LimitKind::IntegerBits,
                limit: 0,
                consumed: 0,
                next_charge: Integer::one(),
                charge_point: ChargePoint::IntegerArithmeticOperands,
            })
            .completed(),
            None
        );
    }

    /// TC-428 (FR-096-AC-8): every kernel refusal but `CheckedInvariant`
    /// returns the catalog code and cause of its row, and
    /// `CheckedInvariant` returns neither.
    #[trace("TC-428", "FR-096-AC-8")]
    #[test]
    fn tc_428_every_record_building_refusal_names_code_and_cause() {
        use crate::integer::Integer;
        use crate::text::TextProfile;

        let interval = || Box::new(IntegerInterval::spanning(Integer::zero(), Integer::one()));
        let rational = || {
            Box::new(
                RationalDomain::new(
                    IntegerInterval::spanning(Integer::zero(), Integer::one()),
                    IntegerInterval::spanning(Integer::one(), Integer::one()),
                )
                .unwrap(),
            )
        };
        let decimal = || {
            Box::new(
                DecimalType::new(
                    Integer::zero(),
                    Integer::one(),
                    0,
                    0,
                    crate::decimal::RoundingMode::Exact,
                )
                .unwrap(),
            )
        };
        let pair = |quotient_admitted, remainder_admitted| Refusal::DivisionPairOutOfDomain {
            domain: interval(),
            quotient_admitted,
            remainder_admitted,
        };
        let universe = UniverseId::from_digest([0; 32]);
        let cases: [(Refusal, &str, &str); 15] = [
            (
                Refusal::InexactDecimal {
                    target: InexactTarget::Integer(interval()),
                },
                "inexact_decimal",
                "nonzero-discarded-digit",
            ),
            (
                Refusal::DecimalOutOfDomain { target: decimal() },
                "decimal_out_of_domain",
                "outside-domain",
            ),
            (
                pair(false, true),
                "division_pair_out_of_domain",
                "quotient-outside-domain",
            ),
            (
                pair(true, false),
                "division_pair_out_of_domain",
                "remainder-outside-domain",
            ),
            (
                pair(false, false),
                "division_pair_out_of_domain",
                "both-outside-domain",
            ),
            (
                Refusal::ModuloOutOfDomain { domain: interval() },
                "modulo_out_of_domain",
                "outside-domain",
            ),
            (
                Refusal::TextLengthOutOfDomain {
                    target: TextType::new(1, 8, TextProfile::Nfc).unwrap(),
                },
                "text_length_out_of_domain",
                "outside-domain",
            ),
            (
                Refusal::IntegerOutOfDomain { target: interval() },
                "integer_out_of_domain",
                "outside-domain",
            ),
            (
                Refusal::RationalOutOfDomain { target: rational() },
                "rational_out_of_domain",
                "outside-domain",
            ),
            (
                Refusal::IeeeNotExact {
                    target: IeeeWidth::Binary32,
                    would_be: IeeeFlags::EMPTY,
                },
                "ieee_not_exact",
                "rounding-required",
            ),
            (
                Refusal::IeeeNanPayloadNotRepresentable {
                    target: IeeeWidth::Binary32,
                    source: IeeeWidth::Binary64,
                },
                "ieee_nan_payload_not_representable",
                "payload-exceeds-target",
            ),
            (
                Refusal::IeeeRationalOutOfDomain { target: rational() },
                "ieee_rational_out_of_domain",
                "outside-domain",
            ),
            (
                Refusal::ForeignReference {
                    required: universe,
                    supplied: universe,
                },
                "foreign_reference",
                "foreign-universe",
            ),
            (
                Refusal::CardinalityOutOfBound {
                    violation: BoundViolation::AboveMaximum,
                    kind: CollectionKind::Sequence,
                    bound: CardinalityBound::new(0, 1).unwrap(),
                    count: 2,
                },
                "cardinality_out_of_bound",
                "above-maximum",
            ),
            (
                Refusal::CardinalityOutOfBound {
                    violation: BoundViolation::BelowMinimum,
                    kind: CollectionKind::Sequence,
                    bound: CardinalityBound::new(1, 2).unwrap(),
                    count: 0,
                },
                "cardinality_out_of_bound",
                "below-minimum",
            ),
        ];
        for (refusal, code, cause) in cases {
            assert_eq!(refusal.code(), Some(code), "{refusal:?}");
            assert_eq!(refusal.cause(), Some(cause), "{refusal:?}");
        }
        assert_eq!(Refusal::CheckedInvariant.code(), None);
        assert_eq!(Refusal::CheckedInvariant.cause(), None);
    }

    /// TC-428 (FR-096-AC-8): a value refusal carries the declared target it
    /// was raised for, so a refusal that dropped or swapped its target
    /// fails here.
    #[trace("TC-428", "FR-096-AC-8")]
    #[test]
    fn tc_428_a_value_refusal_carries_its_target() {
        use crate::integer::Integer;
        use crate::text::TextProfile;

        let range = |lower: i64, upper: i64| {
            IntegerInterval::spanning(Integer::from(lower), Integer::from(upper))
        };

        let Refusal::IntegerOutOfDomain { target } = (Refusal::IntegerOutOfDomain {
            target: Box::new(range(-5, 9)),
        }) else {
            unreachable!()
        };
        assert_eq!(*target, range(-5, 9));

        let narrow = Refusal::InexactDecimal {
            target: InexactTarget::Integer(Box::new(range(0, 9))),
        };
        assert!(
            matches!(&narrow, Refusal::InexactDecimal { target: InexactTarget::Integer(domain) } if **domain == range(0, 9))
        );

        let text = TextType::new(1, 8, TextProfile::Nfc).unwrap();
        assert!(
            matches!(&Refusal::TextLengthOutOfDomain { target: text }, Refusal::TextLengthOutOfDomain { target } if *target == text)
        );

        let flags = IeeeFlags::EMPTY;
        assert!(matches!(
            &Refusal::IeeeNotExact { target: IeeeWidth::Binary32, would_be: flags },
            Refusal::IeeeNotExact { target: IeeeWidth::Binary32, would_be } if *would_be == flags
        ));
        assert!(matches!(
            &Refusal::IeeeNanPayloadNotRepresentable {
                target: IeeeWidth::Binary32,
                source: IeeeWidth::Binary64,
            },
            Refusal::IeeeNanPayloadNotRepresentable {
                target: IeeeWidth::Binary32,
                source: IeeeWidth::Binary64,
            }
        ));

        let domain = DecimalType::new(
            Integer::from(-100_i64),
            Integer::from(100_i64),
            0,
            2,
            crate::decimal::RoundingMode::Exact,
        )
        .unwrap();
        assert!(
            matches!(&Refusal::DecimalOutOfDomain { target: Box::new(domain.clone()) }, Refusal::DecimalOutOfDomain { target } if **target == domain)
        );

        let rational = RationalDomain::new(range(-9, 9), range(1, 9)).unwrap();
        assert!(
            matches!(&Refusal::RationalOutOfDomain { target: Box::new(rational.clone()) }, Refusal::RationalOutOfDomain { target } if **target == rational)
        );
    }
}
