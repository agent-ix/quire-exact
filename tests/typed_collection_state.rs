// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public checked collection state boundary (TC-941).

use ix_trace_rs::trace;
use quire_exact::{
    build_typed_prestate, from_admitted, from_admitted_slots, inspect_typed_poststate,
    BoundViolation, CardinalityBound, ChargePoint, CollectionKind, CollectionType, Component,
    ConstructionCause, ConstructionRefusal, FieldValue, InjectedDenial, Integer, Meter, NodeKey,
    Outcome, PathSegment, Refusal, ScalarLimits, Value, ValueType,
};
use std::num::NonZeroU64;

fn meter() -> Meter {
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

fn values(numbers: &[i64]) -> Vec<Value> {
    numbers
        .iter()
        .map(|number| Value::Integer(Integer::from(*number)))
        .collect()
}

fn ints(values: &[Value]) -> Vec<Integer> {
    values
        .iter()
        .map(|value| match value {
            Value::Integer(integer) => integer.clone(),
            other => panic!("expected integer, got {other:?}"),
        })
        .collect()
}

fn admitted(_: &ValueType, _: &Value) -> Result<(), ConstructionCause> {
    Ok(())
}

fn ty(kind: CollectionKind, bound: Option<CardinalityBound>) -> CollectionType {
    CollectionType::new(kind, ValueType::Integer, bound)
}

#[trace("TC-941", "FR-392-AC-1", "FR-392-AC-4", "FR-392-AC-6", "FR-392-AC-7")]
#[test]
fn public_builder_and_inspector_preserve_the_single_typed_representation() {
    let bound = CardinalityBound::new(1, 3).unwrap();
    for (kind, expected) in [
        (CollectionKind::Sequence, vec![2, 2, 1]),
        (CollectionKind::OrderedSet, vec![2, 1]),
    ] {
        let declared = ty(kind, Some(bound));
        let value = match build_typed_prestate(
            &declared,
            values(&[2, 2, 1]),
            &mut meter(),
            Some(&mut admitted),
        )
        .unwrap()
        {
            Outcome::Completed(value) => value,
            other => panic!("prestate refused: {other:?}"),
        };
        let Value::Collection(stored) = &value else {
            panic!("expected Value::Collection")
        };
        let view =
            match inspect_typed_poststate(&declared, &value, &mut meter(), Some(&mut admitted))
                .unwrap()
            {
                Outcome::Completed(view) => view,
                other => panic!("poststate refused: {other:?}"),
            };
        assert!(core::ptr::eq(view, stored.as_ref()));
        assert_eq!(view.collection_type(), &declared);
        assert_eq!(view.collection_type().bound(), Some(bound));
        assert_eq!(
            ints(view.elements()),
            expected.into_iter().map(Integer::from).collect::<Vec<_>>()
        );
    }
}

#[trace("TC-941", "FR-392-AC-2")]
#[test]
fn inclusive_counts_use_occurrences_or_retained_members() {
    let bound = CardinalityBound::new(2, 4).unwrap();
    for kind in [CollectionKind::Sequence, CollectionKind::OrderedSet] {
        let declared = ty(kind, Some(bound));
        for count in 1..=5 {
            let input = (0..count).map(i64::from).collect::<Vec<_>>();
            let outcome =
                build_typed_prestate(&declared, values(&input), &mut meter(), Some(&mut admitted))
                    .unwrap();
            match (count, outcome) {
                (
                    1,
                    Outcome::Refused(Refusal::CardinalityOutOfBound {
                        violation: BoundViolation::BelowMinimum,
                        count: 1,
                        ..
                    }),
                ) => {}
                (
                    5,
                    Outcome::Refused(Refusal::CardinalityOutOfBound {
                        violation: BoundViolation::AboveMaximum,
                        count: 5,
                        ..
                    }),
                ) => {}
                (2..=4, Outcome::Completed(_)) => {}
                (_, other) => panic!("wrong bound outcome for {count}: {other:?}"),
            }
        }
    }
    let zero = ty(CollectionKind::Sequence, Some(CardinalityBound::at_most(0)));
    assert!(matches!(
        build_typed_prestate(&zero, vec![], &mut meter(), Some(&mut admitted)),
        Ok(Outcome::Completed(_))
    ));
    assert!(matches!(
        build_typed_prestate(&zero, values(&[1]), &mut meter(), Some(&mut admitted)),
        Ok(Outcome::Refused(Refusal::CardinalityOutOfBound {
            violation: BoundViolation::AboveMaximum,
            ..
        }))
    ));
    let absent = ty(CollectionKind::Sequence, None);
    let max = ty(
        CollectionKind::Sequence,
        Some(CardinalityBound::at_most(u64::MAX)),
    );
    assert_ne!(absent, max);
    for declared in [&absent, &max] {
        assert!(matches!(
            build_typed_prestate(declared, values(&[1]), &mut meter(), Some(&mut admitted)),
            Ok(Outcome::Completed(_))
        ));
    }
    let set = ty(CollectionKind::OrderedSet, Some(bound));
    assert!(matches!(
        build_typed_prestate(
            &set,
            values(&[1, 1, 2, 2, 2]),
            &mut meter(),
            Some(&mut admitted)
        ),
        Ok(Outcome::Completed(_))
    ));
}

#[trace("TC-941", "FR-392-AC-3", "FR-392-AC-5")]
#[test]
fn element_refusals_precede_collection_work_and_authority_checks_nested_slots() {
    let declared = ty(CollectionKind::OrderedSet, None);
    let mut meter = meter();
    assert_eq!(
        build_typed_prestate(
            &declared,
            vec![Value::Boolean(true)],
            &mut meter,
            Some(&mut admitted)
        ),
        Err(ConstructionRefusal {
            component: Component::Element(0),
            cause: ConstructionCause::TypeMismatch
        })
    );
    assert_eq!(meter.admission_count(), 0);
    let mut reject = |_: &ValueType, _: &Value| Err(ConstructionCause::MissingField);
    assert_eq!(
        build_typed_prestate(&declared, values(&[1]), &mut meter, Some(&mut reject)),
        Err(ConstructionRefusal {
            component: Component::Element(0),
            cause: ConstructionCause::MissingField
        })
    );
    assert_eq!(meter.admission_count(), 0);
    assert_eq!(
        build_typed_prestate(&declared, values(&[1]), &mut meter, None),
        Err(ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::MissingAdmissionAuthority
        })
    );

    let parent = NodeKey::new("test", vec![PathSegment::Text("Parent".into())], 0).unwrap();
    let child = NodeKey::new("test", vec![PathSegment::Text("Child".into())], 0).unwrap();
    let composite_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Composite(parent.clone()),
        None,
    );
    let forged = from_admitted_slots(parent.clone(), vec![FieldValue::Absent].into_boxed_slice());
    assert!(composite_type.element().admits(&forged));
    let forged_collection = from_admitted(composite_type.clone(), vec![forged]);
    let mut schema = |_: &ValueType, value: &Value| match value {
        Value::Composite(record)
            if matches!(record.slots(), [FieldValue::Present(Value::Integer(_))]) =>
        {
            Ok(())
        }
        _ => Err(ConstructionCause::MissingField),
    };
    assert!(matches!(
        inspect_typed_poststate(
            &composite_type,
            &forged_collection,
            &mut meter(),
            Some(&mut schema)
        ),
        Err(ConstructionRefusal {
            component: Component::Element(0),
            cause: ConstructionCause::MissingField
        })
    ));
    assert!(matches!(
        inspect_typed_poststate(&composite_type, &forged_collection, &mut meter(), None),
        Err(ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::MissingAdmissionAuthority
        })
    ));
    let descendant = from_admitted_slots(
        child,
        vec![FieldValue::Present(Value::Integer(Integer::one()))].into_boxed_slice(),
    );
    let descendant_collection = from_admitted(composite_type.clone(), vec![descendant]);
    assert!(matches!(
        inspect_typed_poststate(
            &composite_type,
            &descendant_collection,
            &mut meter(),
            Some(&mut admitted)
        ),
        Err(ConstructionRefusal {
            component: Component::Element(0),
            cause: ConstructionCause::TypeMismatch
        })
    ));
}

#[trace("TC-941", "FR-392-AC-4", "FR-392-AC-5", "FR-392-AC-6")]
#[test]
fn inspection_refuses_malformed_storage_and_charges_only_membership_and_bound() {
    let declared = ty(
        CollectionKind::OrderedSet,
        Some(CardinalityBound::new(1, 2).unwrap()),
    );
    let wrong = from_admitted(declared.clone(), vec![Value::Boolean(true)]);
    assert!(matches!(
        inspect_typed_poststate(&declared, &wrong, &mut meter(), Some(&mut admitted)),
        Err(ConstructionRefusal {
            component: Component::Element(0),
            cause: ConstructionCause::TypeMismatch
        })
    ));
    let duplicate = from_admitted(declared.clone(), values(&[1, 1]));
    assert!(matches!(
        inspect_typed_poststate(&declared, &duplicate, &mut meter(), Some(&mut admitted)),
        Err(ConstructionRefusal {
            component: Component::Element(1),
            cause: ConstructionCause::DuplicateMember
        })
    ));
    let over = from_admitted(declared.clone(), values(&[1, 2, 3]));
    assert!(matches!(
        inspect_typed_poststate(&declared, &over, &mut meter(), Some(&mut admitted)),
        Ok(Outcome::Refused(Refusal::CardinalityOutOfBound {
            violation: BoundViolation::AboveMaximum,
            count: 3,
            ..
        }))
    ));
    assert!(matches!(
        inspect_typed_poststate(
            &declared,
            &Value::Boolean(true),
            &mut meter(),
            Some(&mut admitted)
        ),
        Err(ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::TypeMismatch
        })
    ));
    let other_kind = ty(CollectionKind::Sequence, declared.bound());
    assert!(matches!(
        inspect_typed_poststate(&other_kind, &duplicate, &mut meter(), Some(&mut admitted)),
        Err(ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::TypeMismatch
        })
    ));
    let other_bound = ty(CollectionKind::OrderedSet, None);
    assert!(matches!(
        inspect_typed_poststate(&other_bound, &duplicate, &mut meter(), Some(&mut admitted)),
        Err(ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::TypeMismatch
        })
    ));
    let other_element = CollectionType::new(
        CollectionKind::OrderedSet,
        ValueType::Boolean,
        declared.bound(),
    );
    assert!(matches!(
        inspect_typed_poststate(
            &other_element,
            &duplicate,
            &mut meter(),
            Some(&mut admitted)
        ),
        Err(ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::TypeMismatch
        })
    ));
    let reversed = from_admitted(declared.clone(), values(&[2, 1]));
    let forward = from_admitted(declared.clone(), values(&[1, 2]));
    for value in [&reversed, &forward] {
        let mut inspected = meter();
        let view =
            match inspect_typed_poststate(&declared, value, &mut inspected, Some(&mut admitted))
                .unwrap()
            {
                Outcome::Completed(view) => view,
                other => panic!("unexpected inspection: {other:?}"),
            };
        assert_eq!(
            ints(view.elements()),
            match value {
                Value::Collection(c) => ints(c.elements()),
                _ => unreachable!(),
            }
        );
        assert_eq!(inspected.admission_count(), 3); // one comparison, then bound
        for point in [
            ChargePoint::CollectionMemberWalk,
            ChargePoint::CollectionMemberTest,
            ChargePoint::CollectionBound,
        ] {
            let mut denied = meter().with_injected_denial(InjectedDenial {
                point,
                occurrence: NonZeroU64::new(1).unwrap(),
            });
            assert!(matches!(
                inspect_typed_poststate(&declared, value, &mut denied, Some(&mut admitted)),
                Ok(Outcome::Incomplete(_))
            ));
        }
    }
}

#[trace("TC-941", "FR-392-AC-3", "FR-392-AC-6")]
#[test]
fn denied_collection_charges_return_only_incomplete() {
    let ordered = ty(CollectionKind::OrderedSet, None);
    for point in [
        ChargePoint::CollectionMemberWalk,
        ChargePoint::CollectionMemberTest,
        ChargePoint::CollectionBound,
        ChargePoint::CollectionResultRetain,
    ] {
        let mut denied = meter().with_injected_denial(InjectedDenial {
            point,
            occurrence: NonZeroU64::new(1).unwrap(),
        });
        assert!(matches!(
            build_typed_prestate(&ordered, values(&[1, 2]), &mut denied, Some(&mut admitted)),
            Ok(Outcome::Incomplete(_))
        ));
    }
    let sequence = ty(CollectionKind::Sequence, None);
    let stored = from_admitted(sequence.clone(), values(&[2, 2, 1]));
    let mut observed = meter();
    let view = inspect_typed_poststate(&sequence, &stored, &mut observed, Some(&mut admitted));
    assert!(matches!(view, Ok(Outcome::Completed(_))));
    assert_eq!(observed.admission_count(), 1); // collection.bound only
    let mut denied = meter().with_injected_denial(InjectedDenial {
        point: ChargePoint::CollectionBound,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    assert!(matches!(
        inspect_typed_poststate(&sequence, &stored, &mut denied, Some(&mut admitted)),
        Ok(Outcome::Incomplete(_))
    ));
}
