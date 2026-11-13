// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native enum carrier (FR-407, TC-955) through the public kernel API.
//! The differential against the owner's `compare_enum` (FR-407-AC-7) lives in
//! quire-semantic-value.
use ix_trace_rs::trace;
use quire_exact::{
    compare_native_enum, ChargePoint, ComparisonOperator, EnumMember, EnumShape, IllTyped,
    IllTypedCause, Integer, LimitKind, Meter, NativeEnum, NativeEnumRefusal, NodeKey, Outcome,
    PathSegment, ScalarLimits, VariantId,
};

fn limits(value_occurrences: u64, result_units: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences,
        work_units: u64::MAX,
        result_units,
    }
}

fn generous() -> Meter {
    Meter::new(limits(u64::MAX, u64::MAX))
}

fn carrier(declaration: u64, ordered: bool, index: u32, count: u32) -> NativeEnum {
    NativeEnum::from_index(declaration, ordered, index, count).unwrap()
}

fn compare(
    operator: ComparisonOperator,
    left: &NativeEnum,
    right: &NativeEnum,
    meter: &mut Meter,
) -> Result<Outcome<bool>, IllTyped> {
    compare_native_enum(operator, left, right, meter)
}

fn assert_copy<T: Copy>() {}

// ---- AC-1 ----

#[test]
#[trace("TC-955", "FR-407-AC-1")]
fn carrier_is_copy_without_drop_glue_and_constructor_takes_no_meter() {
    assert_copy::<NativeEnum>();
    const { assert!(!std::mem::needs_drop::<NativeEnum>()) };
    let _: fn(u64, bool, u32, u32) -> Result<NativeEnum, NativeEnumRefusal> =
        NativeEnum::from_index;
}

#[test]
#[trace("TC-955", "FR-407-AC-1")]
fn construction_and_round_trip_allocate_nothing() {
    let mut result = None;
    let info = allocation_counter::measure(|| {
        let right = NativeEnum::from_index(7, true, 2, 3).unwrap();
        let (index, count, ordered) = right.parts();
        let again = NativeEnum::from_index(7, ordered, index, count).unwrap();
        result = Some(again == right);
    });
    assert_eq!(info.count_total, 0);
    assert_eq!(result, Some(true));
}

// The `test-support` charge log is a `Vec` that allocates on each admitted
// charge, so the comparison itself is measured without it.
#[cfg(not(feature = "test-support"))]
#[test]
#[trace("TC-955", "FR-407-AC-1")]
fn comparison_allocates_nothing() {
    let (left, right) = (carrier(7, true, 0, 3), carrier(7, true, 2, 3));
    let mut meter = generous();
    let mut outcome = None;
    let info = allocation_counter::measure(|| {
        outcome = Some(compare_native_enum(
            ComparisonOperator::Less,
            &left,
            &right,
            &mut meter,
        ));
    });
    assert_eq!(info.count_total, 0);
    assert_eq!(outcome, Some(Ok(Outcome::Completed(true))));
}

// ---- AC-2 ----

#[test]
#[trace("TC-955", "FR-407-AC-2")]
fn from_index_refuses_out_of_range_and_admits_the_edges() {
    for (index, count) in [(3, 3), (4, 3), (u32::MAX, 3), (0, 0), (u32::MAX, u32::MAX)] {
        assert_eq!(
            NativeEnum::from_index(1, true, index, count),
            Err(NativeEnumRefusal::IndexOutOfRange),
            "{index} {count}"
        );
    }
    assert_eq!(carrier(1, true, 0, 3).index(), 0);
    assert_eq!(carrier(1, true, 2, 3).index(), 2);
    assert_eq!(
        carrier(1, false, u32::MAX - 1, u32::MAX).index(),
        u32::MAX - 1
    );
}

// ---- AC-1 / AC-3: round trip, structural equality, from_member ----

#[test]
#[trace("TC-955", "FR-407-AC-1")]
fn parts_round_trip_and_equality_covers_all_four_fields() {
    let base = carrier(9, true, 1, 3);
    assert_eq!(base.parts(), (1, 3, true));
    assert_eq!(base.declaration(), 9);
    assert_eq!(base, carrier(9, true, 1, 3));
    assert_ne!(base, carrier(10, true, 1, 3));
    assert_ne!(base, carrier(9, false, 1, 3));
    assert_ne!(base, carrier(9, true, 2, 3));
    assert_ne!(base, carrier(9, true, 1, 4));
}

fn owner() -> NodeKey {
    NodeKey::new("test", vec![PathSegment::Text("Level".into())], 0).unwrap()
}

fn variant(name: &str) -> VariantId {
    VariantId::new(owner(), name).unwrap()
}

fn shape(ordered: bool) -> EnumShape {
    EnumShape::new(ordered, ["Low", "Mid", "High"].map(variant))
}

#[test]
#[trace("TC-955", "FR-407-AC-3")]
fn from_member_agrees_with_from_index_and_takes_count_and_flag_from_the_shape() {
    for ordered in [true, false] {
        let shape = shape(ordered);
        for (rank, name) in ["Low", "Mid", "High"].into_iter().enumerate() {
            let rank = rank as u32;
            let member = EnumMember::new(variant(name), rank);
            assert_eq!(
                NativeEnum::from_member(5, &shape, &member),
                NativeEnum::from_index(5, ordered, rank, 3)
            );
        }
    }
}

#[test]
#[trace("TC-955", "FR-407-AC-3")]
fn from_member_refuses_wrong_rank_and_foreign_variant() {
    let shape = shape(true);
    let wrong_rank = EnumMember::new(variant("Mid"), 2);
    assert_eq!(
        NativeEnum::from_member(5, &shape, &wrong_rank),
        Err(NativeEnumRefusal::MemberNotInShape)
    );
    let foreign = EnumMember::new(variant("Other"), 1);
    assert_eq!(
        NativeEnum::from_member(5, &shape, &foreign),
        Err(NativeEnumRefusal::MemberNotInShape)
    );
    let other_owner = NodeKey::new("test", vec![PathSegment::Text("Other".into())], 0).unwrap();
    let minted = EnumMember::new(VariantId::new(other_owner, "Mid").unwrap(), 1);
    assert_eq!(
        NativeEnum::from_member(5, &shape, &minted),
        Err(NativeEnumRefusal::MemberNotInShape)
    );
}

// ---- AC-4 ----

/// Independent expectation of the six operators over ranks `a` and `b`.
fn native(operator: ComparisonOperator, a: u32, b: u32) -> bool {
    match operator {
        ComparisonOperator::Equal => a == b,
        ComparisonOperator::NotEqual => a != b,
        ComparisonOperator::Less => a < b,
        ComparisonOperator::LessOrEqual => a <= b,
        ComparisonOperator::Greater => a > b,
        ComparisonOperator::GreaterOrEqual => a >= b,
    }
}

#[test]
#[trace("TC-955", "FR-407-AC-4")]
fn six_operators_over_every_pair_of_an_ordered_enum() {
    for operator in ComparisonOperator::ALL {
        for a in 0..3 {
            for b in 0..3 {
                let outcome = compare(
                    operator,
                    &carrier(1, true, a, 3),
                    &carrier(1, true, b, 3),
                    &mut generous(),
                );
                assert_eq!(
                    outcome,
                    Ok(Outcome::Completed(native(operator, a, b))),
                    "{operator:?} {a} {b}"
                );
            }
        }
    }
}

#[test]
#[trace("TC-955", "FR-407-AC-4")]
fn unordered_enum_equates_and_refuses_orderings() {
    for operator in ComparisonOperator::ALL {
        for a in 0..3 {
            for b in 0..3 {
                let mut meter = generous();
                let outcome = compare(
                    operator,
                    &carrier(1, false, a, 3),
                    &carrier(1, false, b, 3),
                    &mut meter,
                );
                if operator.is_ordering() {
                    assert_eq!(
                        outcome,
                        Err(IllTyped {
                            cause: IllTypedCause::UnorderedEnumOrdering
                        })
                    );
                    assert_eq!(meter.admission_count(), 0);
                } else {
                    assert_eq!(outcome, Ok(Outcome::Completed(native(operator, a, b))));
                }
            }
        }
    }
}

// ---- AC-5 ----

fn cause(
    operator: ComparisonOperator,
    left: &NativeEnum,
    right: &NativeEnum,
) -> Option<IllTypedCause> {
    let mut meter = generous();
    let before = meter.clone();
    let outcome = compare(operator, left, right, &mut meter);
    let refused = outcome.err().map(|error| error.cause);
    if refused.is_some() {
        assert_eq!(meter, before, "a refused call leaves the meter unchanged");
    }
    refused
}

#[test]
#[trace("TC-955", "FR-407-AC-5")]
fn distinct_declarations_are_refused_by_tag_count_or_flag() {
    let distinct = Some(IllTypedCause::DistinctEnumDeclarations);
    let base = carrier(1, true, 0, 3);
    for other in [
        carrier(2, true, 0, 3),
        carrier(1, true, 0, 4),
        carrier(1, false, 0, 3),
    ] {
        for operator in ComparisonOperator::ALL {
            assert_eq!(cause(operator, &base, &other), distinct, "{other:?}");
        }
    }
}

#[test]
#[trace("TC-955", "FR-407-AC-5")]
fn distinct_declaration_takes_precedence_over_unordered_ordering() {
    assert_eq!(
        cause(
            ComparisonOperator::Less,
            &carrier(1, false, 0, 3),
            &carrier(2, false, 0, 3)
        ),
        Some(IllTypedCause::DistinctEnumDeclarations)
    );
    assert_eq!(
        cause(
            ComparisonOperator::Less,
            &carrier(1, false, 0, 3),
            &carrier(1, false, 1, 3)
        ),
        Some(IllTypedCause::UnorderedEnumOrdering)
    );
}

// ---- AC-6 ----

#[cfg(feature = "test-support")]
fn admitted(point: ChargePoint, value_occurrences: Option<u64>) -> quire_exact::AdmittedCharge {
    quire_exact::AdmittedCharge {
        point,
        integer_bits: None,
        decimal_digits: None,
        scale_expansion: None,
        text_input_bytes: None,
        text_scalars: None,
        normalized_scalars: None,
        unit_edges: None,
        value_occurrences,
    }
}

#[cfg(feature = "test-support")]
fn full_sequence() -> Vec<quire_exact::AdmittedCharge> {
    vec![
        admitted(ChargePoint::EnumIdentityRead, Some(1)),
        admitted(ChargePoint::EnumIdentityRead, Some(2)),
        admitted(ChargePoint::EnumResultRetain, None),
    ]
}

fn ordered_pair() -> (NativeEnum, NativeEnum) {
    (carrier(1, true, 0, 3), carrier(1, true, 2, 3))
}

#[test]
#[trace("TC-955", "FR-407-AC-6")]
fn sufficient_and_exact_limits_charge_the_literal_sequence() {
    let (left, right) = ordered_pair();
    // The sufficient limit and the exact limit (2 occurrences, 1 result).
    for budget in [limits(u64::MAX, u64::MAX), limits(2, 1)] {
        for operator in ComparisonOperator::ALL {
            let mut meter = Meter::new(budget);
            let outcome = compare(operator, &left, &right, &mut meter);
            assert_eq!(
                outcome,
                Ok(Outcome::Completed(native(operator, 0, 2))),
                "{operator:?}"
            );
            assert_eq!(meter.admission_count(), 3);
            assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
            #[cfg(feature = "test-support")]
            assert_eq!(meter.admitted_charges(), full_sequence().as_slice());
        }
    }
}

#[test]
#[trace("TC-955", "FR-407-AC-6")]
fn one_less_limit_stops_at_the_denied_charge_with_the_incomplete_record() {
    let (left, right) = ordered_pair();
    // One less occurrence: the second read (size 2) is denied.
    for operator in ComparisonOperator::ALL {
        let mut meter = Meter::new(limits(1, 1));
        let Ok(Outcome::Incomplete(record)) = compare(operator, &left, &right, &mut meter) else {
            panic!("expected incomplete");
        };
        assert_eq!(record.limit_kind, LimitKind::ValueOccurrences);
        assert_eq!(record.limit, 1);
        assert_eq!(record.consumed, 1);
        assert_eq!(record.next_charge, Integer::from(2u64));
        assert_eq!(record.charge_point, ChargePoint::EnumIdentityRead);
        assert_eq!(meter.admission_count(), 1);
        #[cfg(feature = "test-support")]
        assert_eq!(
            meter.admitted_charges(),
            [admitted(ChargePoint::EnumIdentityRead, Some(1))].as_slice()
        );
    }
    // One less result: the retain is denied after both reads.
    for operator in ComparisonOperator::ALL {
        let mut meter = Meter::new(limits(2, 0));
        let Ok(Outcome::Incomplete(record)) = compare(operator, &left, &right, &mut meter) else {
            panic!("expected incomplete");
        };
        assert_eq!(record.limit_kind, LimitKind::ResultUnits);
        assert_eq!(record.limit, 0);
        assert_eq!(record.consumed, 0);
        assert_eq!(record.next_charge, Integer::from(1u64));
        assert_eq!(record.charge_point, ChargePoint::EnumResultRetain);
        assert_eq!(meter.admission_count(), 2);
        #[cfg(feature = "test-support")]
        assert_eq!(meter.admitted_charges(), &full_sequence()[..2]);
    }
    // No occurrences at all: the first read is denied and nothing is admitted.
    let mut meter = Meter::new(limits(0, 1));
    let Ok(Outcome::Incomplete(record)) =
        compare(ComparisonOperator::Equal, &left, &right, &mut meter)
    else {
        panic!("expected incomplete");
    };
    assert_eq!(record.charge_point, ChargePoint::EnumIdentityRead);
    assert_eq!(record.next_charge, Integer::from(1u64));
    assert_eq!(meter.admission_count(), 0);
}

#[test]
#[trace("TC-955", "FR-407-AC-6")]
fn equality_and_ordering_charge_identically_on_one_meter() {
    let (left, right) = ordered_pair();
    let mut meter = Meter::new(limits(u64::MAX, u64::MAX));
    for operator in [ComparisonOperator::Equal, ComparisonOperator::Less] {
        assert!(compare(operator, &left, &right, &mut meter).is_ok());
    }
    assert_eq!(meter.admission_count(), 6);
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 2);
    #[cfg(feature = "test-support")]
    {
        let mut expected = full_sequence();
        expected.extend(full_sequence());
        assert_eq!(meter.admitted_charges(), expected.as_slice());
    }
}

// ---- AC-8 ----

#[test]
#[trace("TC-955", "FR-407-AC-8")]
fn comparison_source_names_no_banned_construct() {
    let source = include_str!("../src/native_enum.rs");
    let start = source
        .find("pub fn compare_native_enum")
        .expect("comparison entry point");
    let body = &source[start..];
    for banned in [
        "cfg(kani)",
        "cfg!(",
        "for ",
        "while ",
        "loop ",
        "loop{",
        ".iter()",
        ".map(",
        ".collect",
        ".count(",
        "Vec",
        "Box",
        "String",
        "Arc",
        "vec!",
        "format!",
        "num_bigint",
        "Integer",
        "NodeKey",
        "EnumValue",
    ] {
        assert!(
            !body.contains(banned),
            "comparison source contains {banned}"
        );
    }
}
