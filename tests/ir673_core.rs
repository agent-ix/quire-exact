// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public collection, division, meter and vocabulary evidence for IR-673.

use std::{collections::HashSet, num::NonZeroU64};

use quire_exact::{
    divide, form_collection, modulo, BoundViolation, CardinalityBound, Charge, ChargePoint,
    CollectionKind, CollectionType, DivisionMember, DivisionProfile, InjectedDenial, Integer,
    IntegerDomain, LimitKind, Meter, Outcome, Refusal, ScalarLimits, Value, ValueType,
};

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

/// Trace: TC-441, FR-097-AC-8
#[test]
fn public_cardinality_query_is_inclusive_and_agrees_with_admission() {
    let bound = CardinalityBound::new(2, 4).unwrap();
    for (count, expected) in [
        (1, Some(BoundViolation::BelowMinimum)),
        (2, None),
        (3, None),
        (4, None),
        (5, Some(BoundViolation::AboveMaximum)),
    ] {
        let actual: Option<BoundViolation> = bound.violation(count);
        assert_eq!(actual, expected);
        let ty = CollectionType::new(CollectionKind::Sequence, ValueType::Integer, Some(bound));
        let values = (0..count)
            .map(|v| Value::Integer(Integer::from(v)))
            .collect();
        let result = form_collection(&ty, values, &mut Meter::new(limits())).unwrap();
        match expected {
            None => assert!(matches!(result, Outcome::Completed(_))),
            Some(violation) => assert!(matches!(result, Outcome::Refused(
                Refusal::CardinalityOutOfBound { violation: actual, count: actual_count, .. }
            ) if actual == violation && actual_count == count)),
        }
    }
    let singleton = CardinalityBound::new(2, 2).unwrap();
    assert_eq!(singleton.violation(1), Some(BoundViolation::BelowMinimum));
    assert_eq!(singleton.violation(2), None);
    assert_eq!(singleton.violation(3), Some(BoundViolation::AboveMaximum));
    let widest = CardinalityBound::new(0, u64::MAX).unwrap();
    assert_eq!(widest.violation(0), None);
    assert_eq!(widest.violation(u64::MAX), None);
    assert_eq!(
        CardinalityBound::new(0, 0).unwrap().violation(1),
        Some(BoundViolation::AboveMaximum)
    );
    assert_eq!(
        CardinalityBound::new(u64::MAX, u64::MAX)
            .unwrap()
            .violation(u64::MAX - 1),
        Some(BoundViolation::BelowMinimum)
    );
}

/// Trace: TC-905, FR-357-AC-7
#[test]
fn modulo_is_euclidean_after_every_division_profile() {
    for (a, b, expected) in [(7_i64, 3_i64, 1_i64), (7, -3, 1), (-7, 3, 2), (-7, -3, 2)] {
        let (a, b) = (Integer::from(a), Integer::from(b));
        for profile in DivisionProfile::ALL {
            let _ = divide(
                profile,
                DivisionMember::Quotient,
                &a,
                &b,
                &IntegerDomain::Mathematical,
                &mut Meter::new(limits()),
            )
            .completed()
            .unwrap();
            assert_eq!(
                modulo(
                    &a,
                    &b,
                    &IntegerDomain::Mathematical,
                    &mut Meter::new(limits())
                )
                .completed(),
                Some(Integer::from(expected))
            );
        }
    }
}

/// Trace: TC-906, FR-358-AC-8, FR-358-AC-9, FR-358-AC-10, FR-358-AC-11
#[cfg(feature = "test-support")]
#[test]
fn truncated_meter_log_preserves_accounting_and_denial_atomicity() {
    let point = ChargePoint::FunctionCall;
    let other = ChargePoint::GraphEdge;
    let mut meter = Meter::new(ScalarLimits {
        work_units: 4097,
        result_units: 4097,
        ..limits()
    });
    for _ in 0..4097 {
        meter.charge(Charge::new(point).results(1)).unwrap();
    }
    assert_eq!(meter.admission_count(), 4097);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 4097);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 4097);
    assert_eq!(meter.admitted_charges(), vec![point; 4096]);
    assert!(meter.charge_log_truncated());
    let before = meter.clone();
    let denial = meter.charge(Charge::new(other)).unwrap_err();
    assert_eq!(
        (
            denial.charge_point,
            denial.limit_kind,
            denial.limit,
            denial.consumed,
            denial.next_charge
        ),
        (other, LimitKind::WorkUnits, 4097, 4097, Integer::one())
    );
    assert_eq!(meter, before);

    let mut result_meter = Meter::new(ScalarLimits {
        result_units: 4097,
        ..limits()
    });
    for _ in 0..4097 {
        result_meter.charge(Charge::new(point).results(1)).unwrap();
    }
    let before = result_meter.clone();
    let denial = result_meter
        .charge(Charge::new(other).results(1))
        .unwrap_err();
    assert_eq!(
        (
            denial.charge_point,
            denial.limit_kind,
            denial.limit,
            denial.consumed,
            denial.next_charge
        ),
        (other, LimitKind::ResultUnits, 4097, 4097, Integer::one())
    );
    assert_eq!(result_meter, before);

    let mut injected = Meter::new(limits());
    for _ in 0..4097 {
        injected.charge(Charge::new(point)).unwrap();
    }
    injected = injected.with_injected_denial(InjectedDenial {
        point: other,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    let before = injected.clone();
    let denial = injected.charge(Charge::new(other)).unwrap_err();
    assert_eq!(
        (
            denial.charge_point,
            denial.limit_kind,
            denial.limit,
            denial.consumed,
            denial.next_charge
        ),
        (other, LimitKind::WorkUnits, 4097, 4097, Integer::one())
    );
    for kind in LimitKind::ALL {
        assert_eq!(injected.consumed(kind), before.consumed(kind));
    }
    assert_eq!(injected.admission_count(), before.admission_count());
    assert_eq!(injected.admitted_charges(), before.admitted_charges());
    assert_eq!(
        injected.charge_log_truncated(),
        before.charge_log_truncated()
    );
    injected.charge(Charge::new(other)).unwrap();
    assert_eq!(injected.admission_count(), 4098);
    assert_eq!(injected.consumed(LimitKind::WorkUnits), 4098);
    assert_eq!(injected.admitted_charges(), vec![point; 4096]);
}

/// Trace: TC-907, FR-359-AC-6, FR-359-AC-7
#[test]
fn meter_uses_field_order_and_all_counters_are_queryable() {
    let point = ChargePoint::FunctionCall;
    let kinds = LimitKind::ALL;
    let prior = [11, 22, 33, 44, 55, 66, 77, 88, 99, 111];
    for (position, unavailable) in kinds.into_iter().enumerate() {
        let mut configured = limits();
        let mut charge = Charge::new(point).work(Integer::from(2_u64)).results(3);
        let sizes: Vec<_> = if position % 2 == 0 {
            kinds.into_iter().enumerate().take(8).collect()
        } else {
            kinds.into_iter().enumerate().take(8).rev().collect()
        };
        for (index, kind) in sizes {
            charge = charge.size(kind, prior[index] + 2);
            if index >= position {
                match kind {
                    LimitKind::IntegerBits => configured.integer_bits = prior[index] + 1,
                    LimitKind::DecimalDigits => configured.decimal_digits = prior[index] + 1,
                    LimitKind::ScaleExpansion => configured.scale_expansion = prior[index] + 1,
                    LimitKind::TextInputBytes => configured.text_input_bytes = prior[index] + 1,
                    LimitKind::TextScalars => configured.text_scalars = prior[index] + 1,
                    LimitKind::NormalizedScalars => {
                        configured.normalized_scalars = prior[index] + 1
                    }
                    LimitKind::UnitEdges => configured.unit_edges = prior[index] + 1,
                    LimitKind::ValueOccurrences => configured.value_occurrences = prior[index] + 1,
                    LimitKind::WorkUnits | LimitKind::ResultUnits => unreachable!(),
                }
            }
        }
        if position <= 8 {
            configured.work_units = prior[8] + 1;
        }
        if position <= 9 {
            configured.result_units = prior[9] + 2;
        }
        let mut meter = Meter::new(configured);
        let initial = kinds.into_iter().take(8).enumerate().fold(
            Charge::new(point)
                .work(Integer::from(prior[8]))
                .results(prior[9]),
            |charge, (index, kind)| charge.size(kind, prior[index]),
        );
        meter.charge(initial).unwrap();
        for (kind, expected) in kinds.into_iter().zip(prior) {
            assert_eq!(meter.consumed(kind), expected);
        }
        let before = meter.clone();
        let denial = meter.charge(charge).unwrap_err();
        assert_eq!(denial.limit_kind, unavailable);
        assert_eq!(denial.charge_point, point);
        assert_eq!(
            denial.limit,
            if position < 8 {
                prior[position] + 1
            } else if position == 8 {
                100
            } else {
                113
            }
        );
        assert_eq!(denial.consumed, prior[position]);
        assert_eq!(
            denial.next_charge,
            Integer::from(if position == 8 {
                2_u64
            } else if position == 9 {
                3
            } else {
                prior[position] + 2
            })
        );
        assert_eq!(meter, before);
        for (kind, expected) in kinds.into_iter().zip(prior) {
            assert_eq!(meter.consumed(kind), expected);
        }
    }
    let mut meter = Meter::new(limits());
    let initial = kinds
        .into_iter()
        .take(8)
        .enumerate()
        .fold(Charge::new(point).results(2), |charge, (index, kind)| {
            charge.size(kind, 8 - index as u64)
        });
    meter.charge(initial).unwrap();
    let lower = kinds
        .into_iter()
        .take(8)
        .fold(Charge::new(point).results(1), |charge, kind| {
            charge.size(kind, 1)
        });
    meter.charge(lower).unwrap();
    for (kind, expected) in kinds.into_iter().zip([8, 7, 6, 5, 4, 3, 2, 1, 2, 3]) {
        assert_eq!(meter.consumed(kind), expected, "{kind:?}");
    }
    let before = meter.clone();
    let _ = meter
        .charge(Charge::new(point).work(Integer::from(u64::MAX)))
        .unwrap_err();
    assert_eq!(meter, before);
    for (kind, expected) in kinds.into_iter().zip([8, 7, 6, 5, 4, 3, 2, 1, 2, 3]) {
        assert_eq!(meter.consumed(kind), expected, "{kind:?}");
    }

    let denial = InjectedDenial {
        point,
        occurrence: NonZeroU64::new(3).unwrap(),
    };
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: 8,
        ..limits()
    })
    .with_injected_denial(denial);
    meter.charge(Charge::new(point)).unwrap();
    let before = meter.clone();
    let refused = meter
        .charge(Charge::new(point).size(LimitKind::IntegerBits, 9))
        .unwrap_err();
    assert_eq!(
        (
            refused.charge_point,
            refused.limit_kind,
            refused.limit,
            refused.consumed,
            refused.next_charge
        ),
        (point, LimitKind::IntegerBits, 8, 0, Integer::from(9_u64))
    );
    assert_eq!(meter, before);
    meter.charge(Charge::new(point)).unwrap();
    let injected = meter.charge(Charge::new(point)).unwrap_err();
    assert_eq!(
        (
            injected.charge_point,
            injected.limit_kind,
            injected.consumed
        ),
        (point, LimitKind::WorkUnits, 2)
    );
}

/// Trace: TC-916, FR-368-AC-1
#[test]
fn charge_point_vocabulary_is_unique_and_round_trips() {
    const _: () = {
        assert!(matches!(ChargePoint::ALL[0], ChargePoint::DecimalOperands));
        assert!(matches!(
            ChargePoint::ALL[1],
            ChargePoint::DecimalScaleExpansion
        ));
        assert!(matches!(
            ChargePoint::ALL[2],
            ChargePoint::DecimalArithmetic
        ));
        assert!(matches!(ChargePoint::ALL[3], ChargePoint::DecimalRounding));
        assert!(matches!(
            ChargePoint::ALL[4],
            ChargePoint::DecimalResultRetain
        ));
        assert!(matches!(ChargePoint::ALL[5], ChargePoint::TextInputBytes));
        assert!(matches!(
            ChargePoint::ALL[6],
            ChargePoint::TextDecodeScalars
        ));
        assert!(matches!(
            ChargePoint::ALL[7],
            ChargePoint::TextNormalizeInput
        ));
        assert!(matches!(
            ChargePoint::ALL[8],
            ChargePoint::TextNormalizeOutput
        ));
        assert!(matches!(ChargePoint::ALL[9], ChargePoint::TextResultRetain));
        assert!(matches!(
            ChargePoint::ALL[10],
            ChargePoint::EnumIdentityRead
        ));
        assert!(matches!(
            ChargePoint::ALL[11],
            ChargePoint::EnumResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[12],
            ChargePoint::UnitIdentityRead
        ));
        assert!(matches!(ChargePoint::ALL[13], ChargePoint::UnitEdge));
        assert!(matches!(
            ChargePoint::ALL[14],
            ChargePoint::UnitRationalArithmetic
        ));
        assert!(matches!(
            ChargePoint::ALL[15],
            ChargePoint::UnitTargetDomain
        ));
        assert!(matches!(
            ChargePoint::ALL[16],
            ChargePoint::UnitResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[17],
            ChargePoint::IntegerDivisionOperands
        ));
        assert!(matches!(
            ChargePoint::ALL[18],
            ChargePoint::IntegerDivisionArithmetic
        ));
        assert!(matches!(
            ChargePoint::ALL[19],
            ChargePoint::IntegerDivisionDomain
        ));
        assert!(matches!(
            ChargePoint::ALL[20],
            ChargePoint::IntegerDivisionResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[21],
            ChargePoint::IntegerModulusOperands
        ));
        assert!(matches!(
            ChargePoint::ALL[22],
            ChargePoint::IntegerModulusArithmetic
        ));
        assert!(matches!(
            ChargePoint::ALL[23],
            ChargePoint::IntegerModulusDomain
        ));
        assert!(matches!(
            ChargePoint::ALL[24],
            ChargePoint::IntegerModulusResultRetain
        ));
        assert!(matches!(ChargePoint::ALL[25], ChargePoint::IeeeOperands));
        assert!(matches!(
            ChargePoint::ALL[26],
            ChargePoint::IeeeExactIntermediate
        ));
        assert!(matches!(ChargePoint::ALL[27], ChargePoint::IeeeRound));
        assert!(matches!(
            ChargePoint::ALL[28],
            ChargePoint::IeeeResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[29],
            ChargePoint::EqualityPlanForm
        ));
        assert!(matches!(ChargePoint::ALL[30], ChargePoint::EqualityPlan));
        assert!(matches!(ChargePoint::ALL[31], ChargePoint::EqualityPair));
        assert!(matches!(
            ChargePoint::ALL[32],
            ChargePoint::EqualityResultRetain
        ));
        assert!(matches!(ChargePoint::ALL[33], ChargePoint::FunctionCall));
        assert!(matches!(
            ChargePoint::ALL[34],
            ChargePoint::CollectionElement
        ));
        assert!(matches!(ChargePoint::ALL[35], ChargePoint::CollectionVisit));
        assert!(matches!(
            ChargePoint::ALL[36],
            ChargePoint::CollectionMemberWalk
        ));
        assert!(matches!(
            ChargePoint::ALL[37],
            ChargePoint::CollectionMemberTest
        ));
        assert!(matches!(ChargePoint::ALL[38], ChargePoint::CollectionBound));
        assert!(matches!(
            ChargePoint::ALL[39],
            ChargePoint::CollectionResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[40],
            ChargePoint::CompositeResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[41],
            ChargePoint::IntegerArithmeticOperands
        ));
        assert!(matches!(
            ChargePoint::ALL[42],
            ChargePoint::IntegerArithmeticArithmetic
        ));
        assert!(matches!(
            ChargePoint::ALL[43],
            ChargePoint::IntegerArithmeticResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[44],
            ChargePoint::RationalArithmeticOperands
        ));
        assert!(matches!(
            ChargePoint::ALL[45],
            ChargePoint::RationalArithmeticArithmetic
        ));
        assert!(matches!(
            ChargePoint::ALL[46],
            ChargePoint::RationalArithmeticNormalize
        ));
        assert!(matches!(
            ChargePoint::ALL[47],
            ChargePoint::RationalArithmeticResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[48],
            ChargePoint::OrderingOperands
        ));
        assert!(matches!(
            ChargePoint::ALL[49],
            ChargePoint::OrderingArithmetic
        ));
        assert!(matches!(
            ChargePoint::ALL[50],
            ChargePoint::OrderingResultRetain
        ));
        assert!(matches!(
            ChargePoint::ALL[51],
            ChargePoint::BooleanResultRetain
        ));
        assert!(matches!(ChargePoint::ALL[52], ChargePoint::LookupKey));
        assert!(matches!(
            ChargePoint::ALL[53],
            ChargePoint::LookupResultRetain
        ));
        assert!(matches!(ChargePoint::ALL[54], ChargePoint::PopulationVisit));
        assert!(matches!(ChargePoint::ALL[55], ChargePoint::DispatchSelect));
        assert!(matches!(
            ChargePoint::ALL[56],
            ChargePoint::DeclarationCheck
        ));
        assert!(matches!(ChargePoint::ALL[57], ChargePoint::GraphExpand));
        assert!(matches!(ChargePoint::ALL[58], ChargePoint::GraphEdge));
        assert!(matches!(
            ChargePoint::ALL[59],
            ChargePoint::GraphResultRetain
        ));
        assert!(matches!(ChargePoint::ALL[60], ChargePoint::ModelDeref));
        assert!(matches!(ChargePoint::ALL[61], ChargePoint::ModelNavigate));
    };
    let mut seen = HashSet::new();
    for point in ChargePoint::ALL {
        let name = match point {
            ChargePoint::DecimalOperands => "decimal.operands",
            ChargePoint::DecimalScaleExpansion => "decimal.scale-expansion",
            ChargePoint::DecimalArithmetic => "decimal.arithmetic",
            ChargePoint::DecimalRounding => "decimal.rounding",
            ChargePoint::DecimalResultRetain => "decimal.result-retain",
            ChargePoint::TextInputBytes => "text.input-bytes",
            ChargePoint::TextDecodeScalars => "text.decode-scalars",
            ChargePoint::TextNormalizeInput => "text.normalize-input",
            ChargePoint::TextNormalizeOutput => "text.normalize-output",
            ChargePoint::TextResultRetain => "text.result-retain",
            ChargePoint::EnumIdentityRead => "enum.identity-read",
            ChargePoint::EnumResultRetain => "enum.result-retain",
            ChargePoint::UnitIdentityRead => "unit.identity-read",
            ChargePoint::UnitEdge => "unit.edge",
            ChargePoint::UnitRationalArithmetic => "unit.rational-arithmetic",
            ChargePoint::UnitTargetDomain => "unit.target-domain",
            ChargePoint::UnitResultRetain => "unit.result-retain",
            ChargePoint::IntegerDivisionOperands => "integer-division.operands",
            ChargePoint::IntegerDivisionArithmetic => "integer-division.arithmetic",
            ChargePoint::IntegerDivisionDomain => "integer-division.domain",
            ChargePoint::IntegerDivisionResultRetain => "integer-division.result-retain",
            ChargePoint::IntegerModulusOperands => "integer-modulus.operands",
            ChargePoint::IntegerModulusArithmetic => "integer-modulus.arithmetic",
            ChargePoint::IntegerModulusDomain => "integer-modulus.domain",
            ChargePoint::IntegerModulusResultRetain => "integer-modulus.result-retain",
            ChargePoint::IeeeOperands => "ieee.operands",
            ChargePoint::IeeeExactIntermediate => "ieee.exact-intermediate",
            ChargePoint::IeeeRound => "ieee.round",
            ChargePoint::IeeeResultRetain => "ieee.result-retain",
            ChargePoint::EqualityPlanForm => "equality.plan-form",
            ChargePoint::EqualityPlan => "equality.plan",
            ChargePoint::EqualityPair => "equality.pair",
            ChargePoint::EqualityResultRetain => "equality.result-retain",
            ChargePoint::FunctionCall => "function.call",
            ChargePoint::CollectionElement => "collection.element",
            ChargePoint::CollectionVisit => "collection.visit",
            ChargePoint::CollectionMemberWalk => "collection.member-walk",
            ChargePoint::CollectionMemberTest => "collection.member-test",
            ChargePoint::CollectionBound => "collection.bound",
            ChargePoint::CollectionResultRetain => "collection.result-retain",
            ChargePoint::CompositeResultRetain => "composite.result-retain",
            ChargePoint::IntegerArithmeticOperands => "integer-arithmetic.operands",
            ChargePoint::IntegerArithmeticArithmetic => "integer-arithmetic.arithmetic",
            ChargePoint::IntegerArithmeticResultRetain => "integer-arithmetic.result-retain",
            ChargePoint::RationalArithmeticOperands => "rational-arithmetic.operands",
            ChargePoint::RationalArithmeticArithmetic => "rational-arithmetic.arithmetic",
            ChargePoint::RationalArithmeticNormalize => "rational-arithmetic.normalize",
            ChargePoint::RationalArithmeticResultRetain => "rational-arithmetic.result-retain",
            ChargePoint::OrderingOperands => "ordering.operands",
            ChargePoint::OrderingArithmetic => "ordering.arithmetic",
            ChargePoint::OrderingResultRetain => "ordering.result-retain",
            ChargePoint::BooleanResultRetain => "boolean.result-retain",
            ChargePoint::LookupKey => "lookup.key",
            ChargePoint::LookupResultRetain => "lookup.result-retain",
            ChargePoint::PopulationVisit => "population.visit",
            ChargePoint::DispatchSelect => "dispatch.select",
            ChargePoint::DeclarationCheck => "declaration.check",
            ChargePoint::GraphExpand => "graph.expand",
            ChargePoint::GraphEdge => "graph.edge",
            ChargePoint::GraphResultRetain => "graph.result-retain",
            ChargePoint::ModelDeref => "model.deref",
            ChargePoint::ModelNavigate => "model.navigate",
        };
        assert!(seen.insert(point), "duplicate {point:?}");
        assert_eq!(point.as_str(), name);
        assert_eq!(ChargePoint::from_code(name), Some(point));
    }
    assert_eq!(seen.len(), 62);
    assert_eq!(ChargePoint::from_code("not-a-charge"), None);
}

/// Trace: TC-916, FR-368-AC-2
#[test]
fn limit_kind_vocabulary_matches_scalar_limit_fields() {
    let expected = [
        (LimitKind::IntegerBits, "integer_bits"),
        (LimitKind::DecimalDigits, "decimal_digits"),
        (LimitKind::ScaleExpansion, "scale_expansion"),
        (LimitKind::TextInputBytes, "text_input_bytes"),
        (LimitKind::TextScalars, "text_scalars"),
        (LimitKind::NormalizedScalars, "normalized_scalars"),
        (LimitKind::UnitEdges, "unit_edges"),
        (LimitKind::ValueOccurrences, "value_occurrences"),
        (LimitKind::WorkUnits, "work_units"),
        (LimitKind::ResultUnits, "result_units"),
    ];
    assert_eq!(LimitKind::ALL.len(), expected.len());
    for (position, (actual, (kind, name))) in LimitKind::ALL.into_iter().zip(expected).enumerate() {
        let index = match actual {
            LimitKind::IntegerBits => 0,
            LimitKind::DecimalDigits => 1,
            LimitKind::ScaleExpansion => 2,
            LimitKind::TextInputBytes => 3,
            LimitKind::TextScalars => 4,
            LimitKind::NormalizedScalars => 5,
            LimitKind::UnitEdges => 6,
            LimitKind::ValueOccurrences => 7,
            LimitKind::WorkUnits => 8,
            LimitKind::ResultUnits => 9,
        };
        assert_eq!(index, position);
        assert_eq!(actual, kind);
        assert_eq!(actual.as_str(), name);
    }
    let mut seen = HashSet::new();
    for kind in LimitKind::ALL {
        assert!(seen.insert(kind));
    }
}
