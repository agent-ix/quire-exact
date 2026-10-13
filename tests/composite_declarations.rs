// SPDX-License-Identifier: AGPL-3.0-or-later
//! Composite values of different declarations: equality (FR-376), canonical
//! key (FR-377) and the caller-admitted paths (FR-378).

use std::cmp::Ordering;

use ix_trace_rs::trace;
use quire_exact::{
    compare_keys, construct_collection, evaluate_record, form, form_collection, form_grouped,
    from_admitted_slots, member_equal, plan_equality, planned_equality, planned_equality_with_plan,
    retain_composite, CheckedInvariantCause, CollectionKind, CollectionType, CollectionValue,
    Component, ConstructionCause, ConstructionRefusal, Deferred, EffectiveId, FieldDeclaration,
    FieldExpression, FieldValue, IeeeValue, Integer, LimitKind, MemberId, Meter, NodeKey, ObjectId,
    ObjectReference, OptionValue, Outcome, PopulationId, Presence, Refusal, ScalarLimits,
    UniverseId, Value, ValueType,
};

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

fn key(byte: u8) -> NodeKey {
    NodeKey::from_digest([byte; 32])
}

const P: u8 = 1;
const C: u8 = 2;
const G: u8 = 3;
const W: u8 = 4;
const R: u8 = 5;

fn int(value: i64) -> FieldValue {
    FieldValue::Present(Value::Integer(Integer::from(value)))
}

fn comp(declaration: NodeKey, slots: Vec<FieldValue>) -> Value {
    from_admitted_slots(declaration, slots.into_boxed_slice())
}

fn rec(declaration: u8, slots: Vec<FieldValue>) -> Value {
    comp(key(declaration), slots)
}

fn p100() -> Value {
    rec(P, vec![int(100)])
}
fn p200() -> Value {
    rec(P, vec![int(200)])
}
fn c100() -> Value {
    rec(C, vec![int(100)])
}
fn c5_100() -> Value {
    rec(C, vec![int(5), int(100)])
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

fn coll_type(kind: CollectionKind) -> CollectionType {
    CollectionType::new(kind, ValueType::Composite(key(P)), None)
}

fn wrappers(inner_left: &Value, inner_right: &Value) -> Vec<(&'static str, Value, Value)> {
    let option = |inner: &Value| {
        OptionValue::from_admitted(ValueType::Composite(key(P)), Some(inner.clone()))
    };
    let record = |inner: &Value| rec(W, vec![FieldValue::Present(inner.clone())]);
    let collection =
        |kind, inner: &Value| quire_exact::from_admitted(coll_type(kind), vec![inner.clone()]);
    let mut cases = vec![
        ("option", option(inner_left), option(inner_right)),
        ("record", record(inner_left), record(inner_right)),
    ];
    for (name, kind) in [
        ("sequence", CollectionKind::Sequence),
        ("set", CollectionKind::Set),
        ("bag", CollectionKind::Bag),
    ] {
        cases.push((
            name,
            collection(kind, inner_left),
            collection(kind, inner_right),
        ));
    }
    cases
}

fn pairs(left: &Value, right: &Value) -> Result<(Integer, bool), Refusal> {
    let plan = plan_equality(left, right)?;
    let pair_count = plan.pair_events().clone();
    let mut with_plan_meter = meter();
    let with_plan = planned_equality_with_plan(left, right, &mut with_plan_meter)
        .completed()
        .map(|result| {
            assert_eq!(result.plan().pair_events(), &pair_count);
            result.equal()
        });
    let mut equality_meter = meter();
    let equal = planned_equality(left, right, &mut equality_meter).completed();
    assert_eq!(with_plan, equal);
    Ok((pair_count, equal.expect("completed")))
}

fn refusal_of<T: std::fmt::Debug>(outcome: Outcome<T>) -> Refusal {
    match outcome {
        Outcome::Refused(refusal) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    }
}

fn mismatch() -> Refusal {
    Refusal::CheckedInvariant {
        cause: CheckedInvariantCause::ValueKindMismatch,
    }
}

fn collection(value: &Value) -> &CollectionValue {
    match value {
        Value::Collection(collection) => collection,
        other => panic!("expected a collection, got {other:?}"),
    }
}

fn declarations(value: &Value) -> Vec<NodeKey> {
    collection(value)
        .elements()
        .iter()
        .map(|element| match element {
            Value::Composite(composite) => composite.declaration(),
            other => panic!("expected a composite, got {other:?}"),
        })
        .collect()
}

fn keys(bytes: &[u8]) -> Vec<NodeKey> {
    bytes.iter().map(|byte| key(*byte)).collect()
}

#[cfg(feature = "test-support")]
fn log(meter: &Meter) -> Vec<quire_exact::ChargePoint> {
    meter.admitted_charges().iter().map(|c| c.point).collect()
}

// ---------------------------------------------------------------- FR-376

/// Trace: FR-376-AC-1
#[trace("TC-924", "FR-376-AC-1")]
#[test]
fn different_declarations_are_unequal_with_one_pair_in_either_order() {
    let cases = [
        (p100(), c100()),
        (p100(), c5_100()),
        (p100(), g100()),
        (c100(), g100()),
        (rec(P, vec![]), rec(C, vec![])),
    ];
    for (left, right) in cases {
        for (a, b) in [(&left, &right), (&right, &left)] {
            let (count, equal) = pairs(a, b).expect("no refusal");
            assert_eq!(count, Integer::one());
            assert!(!equal);
        }
    }
}

/// Trace: FR-376-AC-2
#[trace("TC-924", "FR-376-AC-2")]
#[test]
fn different_declarations_charge_the_one_pair_schedule() {
    let (left, right) = (p100(), c100());
    let mut meter = meter();
    assert_eq!(
        planned_equality(&left, &right, &mut meter).completed(),
        Some(false)
    );
    #[cfg(feature = "test-support")]
    assert_eq!(
        log(&meter),
        vec![
            quire_exact::ChargePoint::EqualityPlanForm,
            quire_exact::ChargePoint::EqualityPlan,
            quire_exact::ChargePoint::EqualityPair,
            quire_exact::ChargePoint::EqualityResultRetain,
        ]
    );
    assert_eq!(meter.admission_count(), 4);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    // plan-form: occurrences are the larger operand count (2), work the sum (4).
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 4 + 3);
}

/// Trace: FR-376-AC-3
#[trace("TC-924", "FR-376-AC-3")]
#[test]
fn skipped_slots_never_report_a_foreign_reference() {
    let p_ref = rec(P, vec![reference(1)]);
    let c_ref = rec(C, vec![reference(2)]);
    let (count, equal) = pairs(&p_ref, &c_ref).expect("slots are not visited");
    assert_eq!((count, equal), (Integer::one(), false));

    let p_ref2 = rec(P, vec![reference(2)]);
    let expected = Refusal::ForeignReference {
        required: UniverseId::from_digest([1; 32]),
        supplied: UniverseId::from_digest([2; 32]),
    };
    assert_eq!(plan_equality(&p_ref, &p_ref2), Err(expected.clone()));
    assert_eq!(
        refusal_of(planned_equality(&p_ref, &p_ref2, &mut meter())),
        expected
    );
}

/// Trace: FR-376-AC-4
#[trace("TC-924", "FR-376-AC-4")]
#[test]
fn same_declaration_compares_by_slots_as_before() {
    assert_eq!(
        pairs(&p100(), &p100()).unwrap(),
        (Integer::from(2_u64), true)
    );
    assert_eq!(
        pairs(&p100(), &p200()).unwrap(),
        (Integer::from(2_u64), false)
    );
    assert_eq!(
        pairs(&c100(), &c100()).unwrap(),
        (Integer::from(2_u64), true)
    );
}

/// Trace: FR-376-AC-5
#[trace("TC-924", "FR-376-AC-5")]
#[test]
fn different_declarations_inside_wrappers_are_unequal() {
    for (name, left, right) in wrappers(&p100(), &c100()) {
        for (a, b) in [(&left, &right), (&right, &left)] {
            assert_eq!(
                pairs(a, b).unwrap(),
                (Integer::from(2_u64), false),
                "{name}"
            );
        }
    }
    for (name, left, right) in wrappers(&p100(), &p100()) {
        assert_eq!(
            pairs(&left, &right).unwrap(),
            (Integer::from(3_u64), true),
            "{name} control"
        );
    }
}

/// Trace: FR-376-AC-6
#[trace("TC-924", "FR-376-AC-6")]
#[test]
fn faults_that_remain_still_refuse() {
    let populations = || {
        (
            Value::Population(PopulationId::from_digest([1; 32])),
            Value::Population(PopulationId::from_digest([2; 32])),
        )
    };
    let others = [
        Value::Integer(Integer::from(100_i64)),
        OptionValue::none(ValueType::Composite(key(P))),
        quire_exact::from_admitted(coll_type(CollectionKind::Sequence), vec![]),
        Value::Reference(match reference(1) {
            FieldValue::Present(Value::Reference(reference)) => reference,
            _ => unreachable!(),
        }),
    ];
    for other in &others {
        for (a, b) in [(&p100(), other), (other, &p100())] {
            assert_eq!(plan_equality(a, b), Err(mismatch()));
            assert_eq!(refusal_of(planned_equality(a, b, &mut meter())), mismatch());
        }
    }
    let bad = rec(P, vec![int(100), int(100)]);
    assert_eq!(plan_equality(&p100(), &bad), Err(mismatch()));
    assert_eq!(
        refusal_of(planned_equality(&p100(), &bad, &mut meter())),
        mismatch()
    );
    let (left, right) = populations();
    let population_pair = Refusal::CheckedInvariant {
        cause: CheckedInvariantCause::PopulationPair,
    };
    assert_eq!(plan_equality(&left, &right), Err(population_pair.clone()));
    assert_eq!(
        refusal_of(planned_equality(&left, &right, &mut meter())),
        population_pair
    );
}

/// Trace: FR-376-AC-7
#[trace("TC-924", "FR-376-AC-7")]
#[test]
fn member_equal_of_different_declarations_is_false() {
    let mut meter = meter();
    assert_eq!(
        member_equal(&c100(), &p100(), &mut meter).completed(),
        Some(false)
    );
    #[cfg(feature = "test-support")]
    {
        use quire_exact::ChargePoint::{CollectionMemberTest, CollectionMemberWalk};
        assert_eq!(
            log(&meter),
            vec![CollectionMemberWalk, CollectionMemberTest]
        );
        let test = &meter.admitted_charges()[1];
        assert_eq!(test.value_occurrences, Some(1));
    }
    assert_eq!(meter.admission_count(), 2);
    // walk work 2 + 2, test work equal to the pair count of 1.
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 5);
}

fn ordered_set_form(occurrences: Vec<Value>) -> (Vec<NodeKey>, Meter) {
    let mut meter = meter();
    let formed = form(
        &coll_type(CollectionKind::OrderedSet),
        occurrences,
        &mut meter,
    )
    .completed()
    .expect("OrderedSet completes");
    (declarations(&formed), meter)
}

#[cfg(feature = "test-support")]
fn member_tests(meter: &Meter) -> usize {
    log(meter)
        .iter()
        .filter(|point| **point == quire_exact::ChargePoint::CollectionMemberTest)
        .count()
}

/// Trace: FR-376-AC-8
#[trace("TC-924", "FR-376-AC-8")]
#[test]
fn ordered_set_keeps_supply_order_across_declarations() {
    let (order, _meter) = ordered_set_form(vec![c100(), p100()]);
    assert_eq!(order, keys(&[C, P]));
    #[cfg(feature = "test-support")]
    assert_eq!(member_tests(&_meter), 1);

    let (order, _meter) = ordered_set_form(vec![p100(), c100()]);
    assert_eq!(order, keys(&[P, C]));
    #[cfg(feature = "test-support")]
    assert_eq!(member_tests(&_meter), 1);

    let (order, _meter) = ordered_set_form(vec![c100(), p100(), c100()]);
    assert_eq!(order, keys(&[C, P]));
    #[cfg(feature = "test-support")]
    assert_eq!(member_tests(&_meter), 2);
}

// ---------------------------------------------------------------- FR-377

fn ordering(left: &Value, right: &Value) -> Option<Ordering> {
    compare_keys(left, right)
}

/// Trace: FR-377-AC-1
#[trace("TC-925", "FR-377-AC-1")]
#[test]
fn declaration_key_orders_before_slots() {
    let cases = [
        (p100(), c100()),
        (p200(), c100()),
        (p100(), c5_100()),
        (p100(), g100()),
        (c100(), g100()),
    ];
    for (left, right) in &cases {
        assert_eq!(ordering(left, right), Some(Ordering::Less));
        assert_eq!(ordering(right, left), Some(Ordering::Greater));
    }
}

/// Trace: FR-377-AC-2
#[trace("TC-925", "FR-377-AC-2")]
#[test]
fn first_differing_declaration_byte_decides() {
    let with_last = |last: u8| {
        let mut digest = [1_u8; 32];
        digest[31] = last;
        NodeKey::from_digest(digest)
    };
    let k0 = comp(with_last(0), vec![int(9)]);
    let k1 = comp(with_last(1), vec![int(9)]);
    assert_eq!(ordering(&k0, &k1), Some(Ordering::Less));
    assert_eq!(ordering(&k1, &k0), Some(Ordering::Greater));

    let hi = rec(9, vec![int(0)]);
    let lo = rec(2, vec![int(99)]);
    assert_eq!(ordering(&hi, &lo), Some(Ordering::Greater));
    assert_eq!(ordering(&lo, &hi), Some(Ordering::Less));

    let mut first_low = [9_u8; 32];
    first_low[0] = 1;
    let mut first_high = [1_u8; 32];
    first_high[0] = 2;
    let a = comp(NodeKey::from_digest(first_low), vec![]);
    let b = comp(NodeKey::from_digest(first_high), vec![]);
    assert_eq!(ordering(&a, &b), Some(Ordering::Less));
}

/// Trace: FR-377-AC-3
#[trace("TC-925", "FR-377-AC-3")]
#[test]
fn same_declaration_keeps_slot_order_and_distinct_declarations_never_equal() {
    assert_eq!(ordering(&p100(), &p100()), Some(Ordering::Equal));
    assert_eq!(ordering(&p100(), &p200()), Some(Ordering::Less));
    assert_eq!(ordering(&c100(), &c100()), Some(Ordering::Equal));
    assert_ne!(ordering(&p100(), &c100()), Some(Ordering::Equal));
    assert_ne!(ordering(&p100(), &c100()), None);
}

fn formed_order(kind: CollectionKind, occurrences: Vec<Value>, grouped: bool) -> Vec<NodeKey> {
    let mut meter = meter();
    let collection_type = coll_type(kind);
    let outcome = if grouped {
        form_grouped(&collection_type, occurrences, &mut meter)
    } else {
        form(&collection_type, occurrences, &mut meter)
    };
    declarations(&outcome.completed().expect("completes"))
}

/// Trace: FR-377-AC-4
#[trace("TC-925", "FR-377-AC-4")]
#[test]
fn set_and_bag_visit_the_smaller_declaration_key_first() {
    for kind in [CollectionKind::Set, CollectionKind::Bag] {
        for grouped in [false, true] {
            for supply in [vec![c100(), p100()], vec![p100(), c100()]] {
                assert_eq!(
                    formed_order(kind, supply, grouped),
                    keys(&[P, C]),
                    "{kind:?} grouped={grouped}"
                );
            }
        }
    }
    assert_eq!(
        formed_order(
            CollectionKind::Bag,
            vec![c100(), p100(), c100(), p100()],
            false
        ),
        keys(&[P, P, C, C])
    );
}

/// Trace: FR-377-AC-5
#[trace("TC-925", "FR-377-AC-5")]
#[test]
fn equal_keys_exactly_when_values_are_equal() {
    let values = [c100(), p100(), p200(), g100()];
    for x in &values {
        for y in &values {
            let (_, equal) = pairs(x, y).expect("equality");
            assert_eq!(compare_keys(x, y) == Some(Ordering::Equal), equal);
        }
    }
    let malformed = [(c100(), c5_100()), (c5_100(), c100())];
    for (x, y) in &malformed {
        assert_eq!(compare_keys(x, y), None);
        assert_eq!(plan_equality(x, y), Err(mismatch()));
    }
}

/// Trace: FR-377-AC-6
#[trace("TC-925", "FR-377-AC-6")]
#[test]
fn pairs_without_a_key_stay_unkeyed_and_refuse() {
    let float = || {
        rec(
            P,
            vec![FieldValue::Present(Value::Float(IeeeValue::binary32(0)))],
        )
    };
    let float2 = || {
        rec(
            P,
            vec![FieldValue::Present(Value::Float(IeeeValue::binary32(
                0x3f80_0000,
            )))],
        )
    };
    let integer = Value::Integer(Integer::from(100_i64));
    let bad = rec(P, vec![int(100), int(100)]);
    assert_eq!(compare_keys(&p100(), &integer), None);
    assert_eq!(compare_keys(&p100(), &bad), None);
    assert_eq!(compare_keys(&float(), &float2()), None);

    let unavailable = Refusal::CheckedInvariant {
        cause: CheckedInvariantCause::CanonicalKeyUnavailable,
    };
    for (left, right) in [(float(), float2()), (p100(), integer), (p100(), bad)] {
        let collection_type = coll_type(CollectionKind::Set);
        let grouped = form_grouped(
            &collection_type,
            vec![left.clone(), right.clone()],
            &mut meter(),
        );
        assert_eq!(refusal_of(grouped), unavailable);
        let formed = form(&collection_type, vec![left, right], &mut meter());
        assert_eq!(refusal_of(formed), mismatch());
    }
}

/// Trace: FR-377-AC-7
#[trace("TC-925", "FR-377-AC-7")]
#[test]
fn declaration_order_holds_inside_wrappers() {
    for (name, left, right) in wrappers(&p100(), &c100()) {
        assert_eq!(ordering(&left, &right), Some(Ordering::Less), "{name}");
        assert_eq!(ordering(&right, &left), Some(Ordering::Greater), "{name}");
    }
}

// ---------------------------------------------------------------- FR-378

fn all_kinds() -> [CollectionKind; 4] {
    [
        CollectionKind::Sequence,
        CollectionKind::Set,
        CollectionKind::Bag,
        CollectionKind::OrderedSet,
    ]
}

/// Trace: FR-378-AC-1
#[trace("TC-926", "FR-378-AC-1")]
#[test]
fn form_and_form_grouped_keep_both_declarations_unchanged() {
    for kind in all_kinds() {
        for grouped in [false, true] {
            let mut meter = meter();
            let collection_type = coll_type(kind);
            let occurrences = vec![p100(), c100()];
            let outcome = if grouped {
                form_grouped(&collection_type, occurrences, &mut meter)
            } else {
                form(&collection_type, occurrences, &mut meter)
            };
            let formed = outcome.completed().expect("completes");
            let members = collection(&formed).elements();
            assert_eq!(members.len(), 2, "{kind:?}");
            // Supply order is also the key order here, so every kind agrees.
            assert_eq!(declarations(&formed), keys(&[P, C]));
            assert!(pairs(&members[0], &p100()).unwrap().1);
            assert!(pairs(&members[1], &c100()).unwrap().1);
        }
    }
}

/// Trace: FR-378-AC-2
#[trace("TC-926", "FR-378-AC-2")]
#[test]
fn form_over_a_set_charges_one_member_comparison_and_no_element() {
    let mut meter = meter();
    let outcome = form(
        &coll_type(CollectionKind::Set),
        vec![c100(), p100()],
        &mut meter,
    );
    assert!(outcome.completed().is_some());
    #[cfg(feature = "test-support")]
    {
        use quire_exact::ChargePoint::{
            CollectionBound, CollectionMemberTest, CollectionMemberWalk, CollectionResultRetain,
        };
        assert_eq!(
            log(&meter),
            vec![
                CollectionMemberWalk,
                CollectionMemberTest,
                CollectionBound,
                CollectionResultRetain
            ]
        );
        assert_eq!(meter.admitted_charges()[1].value_occurrences, Some(1));
    }
    assert_eq!(meter.admission_count(), 4);
}

/// Trace: FR-378-AC-3
#[trace("TC-926", "FR-378-AC-3")]
#[test]
fn from_admitted_option_stores_a_descendant_payload_unchanged() {
    let option = OptionValue::from_admitted(ValueType::Composite(key(P)), Some(c100()));
    let Value::Option(option) = option else {
        panic!("option expected");
    };
    assert_eq!(option.payload_type(), &ValueType::Composite(key(P)));
    let Some(Value::Composite(payload)) = option.payload() else {
        panic!("composite payload expected");
    };
    assert_eq!(payload.declaration(), key(C));
    assert!(pairs(option.payload().unwrap(), &c100()).unwrap().1);
}

fn record_shape() -> Vec<FieldDeclaration> {
    vec![FieldDeclaration::new(
        MemberId::from_digest([9; 32]),
        "inner",
        ValueType::Composite(key(P)),
        Presence::Required,
    )]
}

/// Trace: FR-378-AC-4
#[trace("TC-926", "FR-378-AC-4")]
#[test]
fn from_admitted_slots_keeps_a_descendant_slot_and_retains_once() {
    let with_c = rec(R, vec![FieldValue::Present(c100())]);
    let with_p = rec(R, vec![FieldValue::Present(p100())]);
    let mut meter = meter();
    let retained = retain_composite(with_c, &mut meter)
        .completed()
        .expect("retained");
    let Value::Composite(composite) = &retained else {
        panic!("composite expected");
    };
    assert_eq!(composite.declaration(), key(R));
    let FieldValue::Present(Value::Composite(slot)) = &composite.slots()[0] else {
        panic!("present composite slot expected");
    };
    assert_eq!(slot.declaration(), key(C));
    #[cfg(feature = "test-support")]
    assert_eq!(
        log(&meter),
        vec![quire_exact::ChargePoint::CompositeResultRetain]
    );
    assert_eq!(meter.admission_count(), 1);
    assert!(!pairs(&retained, &with_p).unwrap().1);
}

fn c_deferred() -> Deferred<'static> {
    Box::new(|_| Outcome::Completed(c100()))
}

fn p_deferred() -> Deferred<'static> {
    Box::new(|_| Outcome::Completed(p100()))
}

/// Trace: FR-378-AC-5
#[trace("TC-926", "FR-378-AC-5")]
#[test]
fn exact_paths_refuse_a_descendant_and_accept_the_declared_type() {
    let collection_type = coll_type(CollectionKind::Sequence);
    let element = ValueType::Composite(key(P));
    let field = MemberId::from_digest([9; 32]);

    let refused = construct_collection(&collection_type, vec![c_deferred()], &mut meter());
    assert_eq!(
        refusal_of(refused),
        Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::CollectionElementNotAdmitted
        }
    );
    assert_eq!(
        form_collection(&collection_type, vec![c100()], &mut meter()).unwrap_err(),
        ConstructionRefusal {
            component: Component::Element(0),
            cause: ConstructionCause::TypeMismatch
        }
    );
    assert!(!element.admits(&c100()));
    assert_eq!(
        OptionValue::present(element.clone(), c100()).unwrap_err(),
        ConstructionRefusal {
            component: Component::Payload,
            cause: ConstructionCause::TypeMismatch
        }
    );
    let record = evaluate_record(
        &record_shape(),
        key(R),
        vec![(field, FieldExpression::Evaluate(c_deferred()))],
        &mut meter(),
    )
    .expect("construction");
    assert_eq!(
        refusal_of(record),
        Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::DeferredResultNotAdmitted
        }
    );

    // The declared type is accepted by each.
    assert!(
        construct_collection(&collection_type, vec![p_deferred()], &mut meter())
            .completed()
            .is_some()
    );
    assert!(
        form_collection(&collection_type, vec![p100()], &mut meter())
            .unwrap()
            .completed()
            .is_some()
    );
    assert!(element.admits(&p100()));
    assert!(OptionValue::present(element, p100()).is_ok());
    assert!(evaluate_record(
        &record_shape(),
        key(R),
        vec![(field, FieldExpression::Evaluate(p_deferred()))],
        &mut meter(),
    )
    .expect("construction")
    .completed()
    .is_some());
}

/// Trace: FR-378-AC-6
#[trace("TC-926", "FR-378-AC-6")]
#[test]
fn collections_are_not_covariant_and_stored_collections_behave() {
    let set_of = |element: u8, members: Vec<Value>| {
        let collection_type = CollectionType::new(
            CollectionKind::Set,
            ValueType::Composite(key(element)),
            None,
        );
        form(&collection_type, members, &mut meter())
            .completed()
            .expect("set")
    };
    let set_p = set_of(P, vec![p100(), c100()]);
    let type_of = |element: u8| {
        ValueType::collection(CollectionType::new(
            CollectionKind::Set,
            ValueType::Composite(key(element)),
            None,
        ))
    };
    assert!(type_of(P).admits(&set_p));
    assert!(!type_of(C).admits(&set_p));
    let set_c = set_of(C, vec![c100()]);
    assert!(type_of(C).admits(&set_c));
    assert!(!type_of(P).admits(&set_c));

    let cloned = set_p.clone();
    assert!(pairs(&cloned, &set_p).unwrap().1);
    assert!(pairs(&set_p, &set_of(P, vec![p100(), c100()])).unwrap().1);
    assert!(!pairs(&set_p, &set_of(P, vec![p100(), g100()])).unwrap().1);
    drop(cloned);
    drop(set_p);
}
