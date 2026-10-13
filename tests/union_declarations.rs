// SPDX-License-Identifier: AGPL-3.0-or-later
//! A union member payload follows the composite rules across declarations:
//! equality (FR-376-AC-9), canonical key (FR-377-AC-8) and the caller-admitted
//! path (FR-378-AC-8). The payload position names `Composite(P)` and holds a
//! `P` or a descendant-declared `C`.

use std::cmp::Ordering;

use ix_trace_rs::trace;
use quire_exact::{
    compare_keys, evaluate_union, form, from_admitted_slots, plan_equality, planned_equality,
    union, CheckedInvariantCause, CollectionKind, CollectionType, Component, ConstructionCause,
    ConstructionRefusal, Deferred, EffectiveId, FieldValue, Identifier, Integer, Meter, NodeKey,
    ObjectId, ObjectReference, Outcome, Refusal, ScalarLimits, UnionMember, UnionValue, UniverseId,
    Value, ValueType,
};

const P: u8 = 1;
const C: u8 = 2;
const G: u8 = 3;
const U: u8 = 9;

fn key(byte: u8) -> NodeKey {
    NodeKey::from_digest([byte; 32])
}

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

fn int(value: i64) -> FieldValue {
    FieldValue::Present(Value::Integer(Integer::from(value)))
}

fn rec(declaration: u8, slots: Vec<FieldValue>) -> Value {
    from_admitted_slots(key(declaration), slots.into_boxed_slice())
}

fn p100() -> Value {
    rec(P, vec![int(100)])
}
fn c100() -> Value {
    rec(C, vec![int(100)])
}
fn g100() -> Value {
    rec(G, vec![int(100)])
}

fn reference(universe: u8) -> FieldValue {
    FieldValue::Present(Value::Reference(ObjectReference::new(
        UniverseId::from_digest([universe; 32]),
        EffectiveId::from_digest([7; 32]),
        ObjectId::new("o1").unwrap(),
    )))
}

fn member(name: &str) -> UnionMember {
    UnionMember::from_admitted(
        key(U),
        NodeKey::from_digest([name.len() as u8; 32]),
        Identifier::new(name).unwrap(),
    )
}

/// A one-position union whose payload is already admitted by the caller.
fn holding(payload: Value) -> Value {
    UnionValue::from_admitted(member("Held"), vec![payload])
}

fn pair_count(left: &Value, right: &Value) -> Integer {
    plan_equality(left, right).unwrap().pair_events().clone()
}

fn equal(left: &Value, right: &Value) -> bool {
    planned_equality(left, right, &mut meter())
        .completed()
        .expect("equality completes")
}

#[trace("FR-376-AC-9")]
#[test]
fn union_payloads_of_different_declarations_are_unequal_without_a_fault() {
    for (left, right) in [(p100(), c100()), (c100(), g100()), (p100(), g100())] {
        for (a, b) in [(&left, &right), (&right, &left)] {
            let (a, b) = (holding(a.clone()), holding(b.clone()));
            assert!(!equal(&a, &b));
            // The union pair and the payload pair; no slot is visited.
            assert_eq!(pair_count(&a, &b), Integer::from(2_i64));
        }
    }
    // Controls: equal declarations walk the slots, as the bare composite does.
    let same = (holding(c100()), holding(c100()));
    assert!(equal(&same.0, &same.1));
    assert_eq!(
        pair_count(&same.0, &same.1),
        Integer::one().add(&pair_count(&c100(), &c100()))
    );
    assert!(!equal(&holding(p100()), &holding(rec(P, vec![int(200)]))));
}

#[trace("FR-376-AC-9")]
#[test]
fn skipped_payload_slots_of_other_declarations_never_report_a_foreign_reference() {
    let left = holding(rec(P, vec![reference(1)]));
    let right = holding(rec(C, vec![reference(2)]));
    assert!(!equal(&left, &right));
    let control = holding(rec(P, vec![reference(2)]));
    assert!(matches!(
        planned_equality(&left, &control, &mut meter()),
        Outcome::Refused(Refusal::ForeignReference { .. })
    ));
}

#[trace("FR-376-AC-9")]
#[test]
fn traversal_does_not_short_circuit_after_a_declaration_mismatch() {
    let member = member("Pair");
    let build = |first: Value, second: i64| {
        UnionValue::from_admitted(
            member.clone(),
            vec![first, Value::Integer(Integer::from(second))],
        )
    };
    let (left, right) = (build(p100(), 1), build(c100(), 2));
    assert!(!equal(&left, &right));
    // union, payload 0, payload 1
    assert_eq!(pair_count(&left, &right), Integer::from(3_i64));
}

#[trace("FR-376-AC-9")]
#[test]
fn union_payload_pair_inside_another_value_leaves_enclosing_counts() {
    let option = |payload: Value| {
        quire_exact::OptionValue::from_admitted(ValueType::Composite(key(U)), Some(payload))
    };
    let (left, right) = (option(holding(p100())), option(holding(c100())));
    assert!(!equal(&left, &right));
    // option, union, payload
    assert_eq!(pair_count(&left, &right), Integer::from(3_i64));
}

#[trace("FR-376-AC-9")]
#[test]
fn unlike_union_pairings_stay_checked_invariant_faults() {
    let kind = |outcome: Outcome<bool>| match outcome {
        Outcome::Refused(Refusal::CheckedInvariant { cause }) => cause,
        other => panic!("expected a checked invariant, got {other:?}"),
    };
    let held = holding(p100());
    let bare = Value::Integer(Integer::from(1_i64));
    assert_eq!(
        kind(planned_equality(&held, &bare, &mut meter())),
        CheckedInvariantCause::ValueKindMismatch
    );
    let two = UnionValue::from_admitted(member("Held"), vec![p100(), p100()]);
    assert_eq!(
        kind(planned_equality(&held, &two, &mut meter())),
        CheckedInvariantCause::ValueKindMismatch
    );
}

#[trace("FR-377-AC-8")]
#[test]
fn union_payload_declarations_order_by_declaration_key_bytes() {
    let (p, c, g) = (holding(p100()), holding(c100()), holding(g100()));
    assert_eq!(compare_keys(&p, &c), Some(Ordering::Less));
    assert_eq!(compare_keys(&c, &p), Some(Ordering::Greater));
    assert_eq!(compare_keys(&c, &g), Some(Ordering::Less));
    assert_eq!(compare_keys(&c, &holding(c100())), Some(Ordering::Equal));
    // The declaration decides before any slot.
    let big_p = holding(rec(P, vec![int(1_000_000)]));
    let small_c = holding(rec(C, vec![int(-1)]));
    assert_eq!(compare_keys(&big_p, &small_c), Some(Ordering::Less));
    // A pair is equal exactly when the values are equal.
    for (a, b) in [(&p, &c), (&p, &holding(rec(P, vec![int(200)]))), (&p, &p)] {
        assert_eq!(compare_keys(a, b) == Some(Ordering::Equal), equal(a, b));
    }
}

#[trace("FR-377-AC-8")]
#[test]
fn union_payload_key_order_sorts_set_and_keeps_ordered_set_supply_order() {
    let element = ValueType::Composite(key(U));
    for supply in [
        vec![holding(c100()), holding(p100()), holding(c100())],
        vec![holding(p100()), holding(c100()), holding(p100())],
    ] {
        let set = form(
            &CollectionType::new(CollectionKind::Set, element.clone(), None),
            supply.clone(),
            &mut meter(),
        )
        .completed()
        .expect("set forms");
        let Value::Collection(set) = set else {
            panic!("set is a collection")
        };
        let order: Vec<Option<NodeKey>> = set
            .elements()
            .iter()
            .map(|value| match value {
                Value::Union(union) => match &union.payload()[0] {
                    Value::Composite(composite) => Some(composite.declaration()),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(order, vec![Some(key(P)), Some(key(C))]);

        let ordered = form(
            &CollectionType::new(CollectionKind::OrderedSet, element.clone(), None),
            supply.clone(),
            &mut meter(),
        )
        .completed()
        .expect("ordered set forms");
        let Value::Collection(ordered) = ordered else {
            panic!("ordered set is a collection")
        };
        assert_eq!(ordered.elements().len(), 2);
        assert!(equal(&ordered.elements()[0], &supply[0]));
        assert!(equal(&ordered.elements()[1], &supply[1]));
    }
}

#[trace("FR-378-AC-8")]
#[test]
fn admitted_union_keeps_a_descendant_payload_and_exact_paths_refuse_it() {
    let shape = [ValueType::Composite(key(P))];
    let admitted = UnionValue::from_admitted(member("Held"), vec![c100()]);
    let Value::Union(stored) = &admitted else {
        panic!("a union")
    };
    let Value::Composite(payload) = &stored.payload()[0] else {
        panic!("a composite payload")
    };
    assert_eq!(payload.declaration(), key(C));
    assert_eq!(payload.slots().len(), 1);

    assert_eq!(
        union(&shape, member("Held"), vec![c100()]).unwrap_err(),
        ConstructionRefusal {
            component: Component::Position(0),
            cause: ConstructionCause::TypeMismatch
        }
    );
    assert!(union(&shape, member("Held"), vec![p100()]).is_ok());

    let deferred = |value: Value| -> Vec<Deferred<'static>> {
        vec![Box::new(move |_| Outcome::Completed(value))]
    };
    let refused = evaluate_union(&shape, member("Held"), deferred(c100()), &mut meter());
    assert!(matches!(
        refused,
        Ok(Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::DeferredResultNotAdmitted
        }))
    ));
    assert!(matches!(
        evaluate_union(&shape, member("Held"), deferred(p100()), &mut meter()),
        Ok(Outcome::Completed(_))
    ));
}
