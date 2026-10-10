// SPDX-License-Identifier: AGPL-3.0-or-later
//! Kernel union construction and traversal controls. These exercise trusted
//! kernel seams, not package admission, SV registry eligibility or v2 transport.

use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::sync::Arc;

use ix_trace_rs::trace;
use quire_exact::{
    compare_keys, evaluate_union, form_collection, form_grouped, from_admitted_slots,
    plan_equality, planned_equality, union, ChargePoint, CheckedInvariantCause, CollectionKind,
    CollectionType, Component, ConstructionCause, ConstructionRefusal, Deferred, EnumMember,
    FieldValue, Identifier, IeeeValue, Integer, LimitKind, Meter, NodeKey, OptionValue, Outcome,
    Refusal, ScalarLimits, Undefined, UnionMember, UnionValue, Value, ValueType, VariantId,
};

fn bytes(hex: &str) -> [u8; 32] {
    assert_eq!(hex.len(), 64);
    let mut bytes = [0; 32];
    for (byte, pair) in bytes.iter_mut().zip(hex.as_bytes().chunks_exact(2)) {
        *byte = u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap();
    }
    bytes
}

// Published QSpec node-identity union_member_vectors: the kernel retains
// already-computed keys; it does not implement or test their JCS/SHA minting.
fn circle() -> UnionMember {
    UnionMember::from_admitted(
        declaration(),
        NodeKey::from_digest(bytes(
            "05592dfb3975b157f2db52dafb4473e94087e77151674909aa18d3527b492dc9",
        )),
        Identifier::new("Circle").unwrap(),
    )
}

fn empty() -> UnionMember {
    UnionMember::from_admitted(
        declaration(),
        NodeKey::from_digest(bytes(
            "da73f21b5ade700e44aa1daf13df5cb138be6a9ecc1f6253e37f99c90dc41f30",
        )),
        Identifier::new("Empty").unwrap(),
    )
}

fn declaration() -> NodeKey {
    NodeKey::from_digest(bytes(
        "684d1e7edc4b570ce3f44ad33aaa4019dd121dc09878aae3acfe0ab8aa92d173",
    ))
}

// Structural fixtures use opaque caller-minted identities, as the existing
// kernel tests do. Their deliberately reversed digest order is not an FR-441
// minting oracle. No declaration position is supplied to the kernel.
fn fixture(name: &str, digest: u8) -> UnionMember {
    UnionMember::from_admitted(
        declaration(),
        NodeKey::from_digest([digest; 32]),
        Identifier::new(name).unwrap(),
    )
}

fn integer(value: i64) -> Value {
    Value::Integer(Integer::from(value))
}

fn make(member: UnionMember, payload: &[i64]) -> Value {
    let shape = vec![ValueType::Integer; payload.len()];
    union(
        &shape,
        member,
        payload.iter().copied().map(integer).collect(),
    )
    .unwrap()
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

fn equal(left: &Value, right: &Value) -> bool {
    planned_equality(left, right, &mut meter())
        .completed()
        .expect("valid equality operands")
}

#[trace("FR-319-AC-3", "FR-321-AC-1")]
#[test]
fn union_construction_keeps_identity_positions_and_occurrences() {
    let nullary = make(empty(), &[]);
    let rect = make(fixture("Rect", 1), &[2, 3]);
    assert_eq!(nullary.occ(), Integer::one());
    assert_eq!(rect.occ(), Integer::from(3_u64));
    let Value::Union(rect) = &rect else {
        panic!("a union value")
    };
    assert_eq!(rect.declaration(), declaration());
    assert_eq!(rect.variant(), VariantId::from_digest([1; 32]));
    assert_eq!(rect.member().identifier().as_str(), "Rect");
    assert!(equal(&rect.payload()[0], &integer(2)));
    assert!(equal(&rect.payload()[1], &integer(3)));
    assert!(ValueType::Composite(declaration()).admits(&nullary));
    assert!(!ValueType::Composite(NodeKey::from_digest([7; 32])).admits(&nullary));
    assert!(!ValueType::Integer.admits(&nullary));
}

#[trace("FR-321-AC-1")]
#[test]
fn union_position_and_arity_refusals_are_typed() {
    let cases = [
        (
            vec![],
            vec![integer(1)],
            Component::Value,
            ConstructionCause::WrongArity {
                declared: 0,
                supplied: 1,
            },
        ),
        (
            vec![ValueType::Integer, ValueType::Integer],
            vec![integer(1)],
            Component::Value,
            ConstructionCause::WrongArity {
                declared: 2,
                supplied: 1,
            },
        ),
        (
            vec![ValueType::Integer],
            vec![Value::Boolean(true)],
            Component::Position(0),
            ConstructionCause::TypeMismatch,
        ),
    ];
    for (shape, payload, component, cause) in cases {
        let Err(actual) = union(&shape, circle(), payload) else {
            panic!("construction must refuse")
        };
        assert_eq!(actual, ConstructionRefusal { component, cause });
    }
}

#[trace("FR-319-AC-3")]
#[test]
fn union_equality_compares_members_and_payload_positions() {
    let rect = make(fixture("Rect", 1), &[2, 3]);
    assert!(equal(&rect, &make(fixture("Rect", 1), &[2, 3])));
    assert!(!equal(&rect, &make(fixture("Rect", 1), &[3, 2])));
    assert!(!equal(&make(empty(), &[]), &make(circle(), &[0])));
    assert!(!equal(
        &make(circle(), &[3]),
        &make(fixture("Square", 2), &[3])
    ));
    assert_eq!(
        plan_equality(&rect, &rect).unwrap().pair_events(),
        &Integer::from(3_u64)
    );
    assert!(equal(&make(empty(), &[]), &make(empty(), &[])));
}

#[trace("FR-319-AC-2", "FR-319-AC-3")]
#[test]
fn foreign_union_enum_and_composite_kinds_remain_distinct() {
    let left = make(circle(), &[1]);
    let foreign = UnionMember::from_admitted(
        NodeKey::from_digest(bytes(
            "e96b8de8d4a48b5f8ec25e1ac855b2241afaf46b3b7634c7bff9834652e3d6ec",
        )),
        NodeKey::from_digest(bytes(
            "13c540c64745302d0774ab6d25097aedf5abacba6ac6cef6a0b67e9042dc7591",
        )),
        Identifier::new("Circle").unwrap(),
    );
    for other in [
        make(foreign, &[1]),
        Value::Enum(EnumMember::new(circle().variant(), 0)),
        from_admitted_slots(
            declaration(),
            vec![FieldValue::Present(integer(1))].into_boxed_slice(),
        ),
    ] {
        assert_eq!(compare_keys(&left, &other), None);
        assert_eq!(
            plan_equality(&left, &other),
            Err(Refusal::CheckedInvariant {
                cause: CheckedInvariantCause::ValueKindMismatch,
            })
        );
    }
}

#[trace("FR-323-AC-1")]
#[test]
fn union_keys_order_identifier_ascii_then_payload_positions() {
    let first = make(fixture("Alpha", 250), &[9]);
    let second = make(fixture("Zulu", 1), &[0]);
    assert_eq!(compare_keys(&first, &second), Some(Ordering::Less));
    assert_eq!(compare_keys(&second, &first), Some(Ordering::Greater));
    let rect = make(fixture("Rect", 3), &[2, 9]);
    let later = make(fixture("Rect", 3), &[3, 1]);
    assert_eq!(compare_keys(&rect, &later), Some(Ordering::Less));
    assert_eq!(
        compare_keys(&rect, &make(fixture("Rect", 3), &[2, 10])),
        Some(Ordering::Less)
    );
    assert_eq!(
        compare_keys(&rect, &make(fixture("Rect", 3), &[2, 9])),
        Some(Ordering::Equal)
    );
    assert_eq!(
        compare_keys(&make(circle(), &[1]), &make(empty(), &[])),
        Some(Ordering::Less)
    );
    assert_eq!(
        compare_keys(&make(empty(), &[]), &make(empty(), &[])),
        Some(Ordering::Equal)
    );
}

fn input() -> Vec<Value> {
    vec![
        make(fixture("Rect", 1), &[2, 3]),
        make(empty(), &[]),
        make(circle(), &[5]),
        make(circle(), &[1]),
        make(fixture("Rect", 1), &[2, 3]),
    ]
}

fn assert_elements(value: Value, expected: Vec<Value>) {
    let Value::Collection(collection) = value else {
        panic!("completed collection")
    };
    assert_eq!(collection.elements().len(), expected.len());
    for (actual, expected) in collection.elements().iter().zip(&expected) {
        assert!(equal(actual, expected), "wrong canonical member or payload");
    }
}

#[trace("FR-323-AC-1", "FR-323-AC-2")]
#[test]
fn union_collections_sort_deduplicate_and_preserve_first_occurrence() {
    for reversed in [false, true] {
        for kind in [
            CollectionKind::Set,
            CollectionKind::Bag,
            CollectionKind::OrderedSet,
            CollectionKind::Sequence,
        ] {
            let mut input = input();
            if reversed {
                input.reverse();
            }
            let expected = match kind {
                CollectionKind::Set => vec![
                    make(circle(), &[1]),
                    make(circle(), &[5]),
                    make(empty(), &[]),
                    make(fixture("Rect", 1), &[2, 3]),
                ],
                CollectionKind::Bag => vec![
                    make(circle(), &[1]),
                    make(circle(), &[5]),
                    make(empty(), &[]),
                    make(fixture("Rect", 1), &[2, 3]),
                    make(fixture("Rect", 1), &[2, 3]),
                ],
                CollectionKind::OrderedSet if reversed => vec![
                    make(fixture("Rect", 1), &[2, 3]),
                    make(circle(), &[1]),
                    make(circle(), &[5]),
                    make(empty(), &[]),
                ],
                CollectionKind::OrderedSet => vec![
                    make(fixture("Rect", 1), &[2, 3]),
                    make(empty(), &[]),
                    make(circle(), &[5]),
                    make(circle(), &[1]),
                ],
                CollectionKind::Sequence => input.clone(),
            };
            let declared = CollectionType::new(kind, ValueType::Composite(declaration()), None);
            let value = form_collection(&declared, input, &mut meter())
                .unwrap()
                .completed()
                .unwrap();
            assert_elements(value, expected);
        }
    }
}

#[trace("FR-323-AC-1")]
#[test]
fn member_declaration_order_and_digest_order_do_not_reach_union_key_order() {
    // Two independent admitted declaration fixtures. Neither position nor
    // digest order is the identifier order; identity remains package scoped.
    for (declaration, bindings) in [
        (
            NodeKey::from_digest([10; 32]),
            [("Zulu", 1), ("Alpha", 250), ("Middle", 2)],
        ),
        (
            NodeKey::from_digest([11; 32]),
            [("Middle", 200), ("Zulu", 251), ("Alpha", 252)],
        ),
    ] {
        let values = bindings
            .into_iter()
            .map(|(name, digest)| {
                union(
                    &[],
                    UnionMember::from_admitted(
                        declaration,
                        NodeKey::from_digest([digest; 32]),
                        Identifier::new(name).unwrap(),
                    ),
                    vec![],
                )
                .unwrap()
            })
            .collect();
        let declared =
            CollectionType::new(CollectionKind::Set, ValueType::Composite(declaration), None);
        let Value::Collection(result) = form_collection(&declared, values, &mut meter())
            .unwrap()
            .completed()
            .unwrap()
        else {
            panic!("set")
        };
        let names: Vec<_> = result
            .elements()
            .iter()
            .map(|value| {
                let Value::Union(value) = value else {
                    panic!("union")
                };
                value.member().identifier().as_str()
            })
            .collect();
        assert_eq!(names, ["Alpha", "Middle", "Zulu"]);
    }
}

#[trace("FR-323-AC-3")]
#[test]
fn ieee_union_payload_has_no_key_and_sequence_keeps_occurrences() {
    let value = UnionValue::from_admitted(
        fixture("Measured", 9),
        vec![Value::Float(IeeeValue::binary64(0x3ff0_0000_0000_0000))],
    );
    assert_eq!(compare_keys(&value, &value), None);
    let set = CollectionType::new(
        CollectionKind::Set,
        ValueType::Composite(declaration()),
        None,
    );
    assert!(matches!(
        form_grouped(&set, vec![value.clone(), value.clone()], &mut meter()),
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::CanonicalKeyUnavailable
        })
    ));
    let sequence = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Composite(declaration()),
        None,
    );
    let Value::Collection(result) =
        form_collection(&sequence, vec![value.clone(), value], &mut meter())
            .unwrap()
            .completed()
            .unwrap()
    else {
        panic!("sequence")
    };
    assert_eq!(result.elements().len(), 2);
    for element in result.elements() {
        let Value::Union(union) = element else {
            panic!("union element")
        };
        let [Value::Float(float)] = union.payload() else {
            panic!("one IEEE payload")
        };
        assert_eq!(*float, IeeeValue::binary64(0x3ff0_0000_0000_0000));
    }
}

#[trace("FR-321-AC-1")]
#[test]
fn evaluated_union_preserves_deferred_fault_and_first_stop() {
    let calls = std::cell::Cell::new(0);
    let payload: Vec<Deferred<'_>> = vec![
        Box::new(|_| {
            calls.set(1);
            Outcome::Undefined(Undefined::DivisionByZero)
        }),
        Box::new(|_| {
            calls.set(2);
            Outcome::Completed(integer(3))
        }),
    ];
    let mut stopped_meter = meter();
    assert!(matches!(
        evaluate_union(
            &[ValueType::Integer, ValueType::Integer],
            fixture("Rect", 1),
            payload,
            &mut stopped_meter
        )
        .unwrap(),
        Outcome::Undefined(Undefined::DivisionByZero)
    ));
    assert_eq!(calls.get(), 1);
    assert_eq!(stopped_meter.admission_count(), 0);
    let payload: Vec<Deferred<'_>> = vec![Box::new(|_| Outcome::Completed(Value::Boolean(true)))];
    assert!(matches!(
        evaluate_union(&[ValueType::Integer], circle(), payload, &mut meter()).unwrap(),
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::DeferredResultNotAdmitted
        })
    ));
}

#[trace("FR-321-AC-1")]
#[test]
fn evaluated_union_orders_arguments_and_charges_one_complete_result() {
    let calls = std::cell::RefCell::new(Vec::new());
    let payload: Vec<Deferred<'_>> = vec![
        Box::new(|_| {
            calls.borrow_mut().push(0);
            Outcome::Completed(integer(2))
        }),
        Box::new(|_| {
            calls.borrow_mut().push(1);
            Outcome::Completed(integer(3))
        }),
    ];
    let mut charged = meter();
    let result = evaluate_union(
        &[ValueType::Integer, ValueType::Integer],
        fixture("Rect", 1),
        payload,
        &mut charged,
    )
    .unwrap()
    .completed()
    .unwrap();
    assert_eq!(*calls.borrow(), vec![0, 1]);
    assert_eq!(charged.admission_count(), 1);
    assert_eq!(charged.consumed(LimitKind::ResultUnits), 3);
    assert!(equal(&result, &make(fixture("Rect", 1), &[2, 3])));
    let mut charged = meter();
    let result = evaluate_union(&[], empty(), vec![], &mut charged)
        .unwrap()
        .completed()
        .unwrap();
    assert_eq!(charged.admission_count(), 1);
    assert_eq!(charged.consumed(LimitKind::ResultUnits), 1);
    assert_eq!(result.occ(), Integer::one());
    let mut denied = Meter::new(ScalarLimits {
        value_occurrences: 0,
        ..*meter().limits()
    });
    let Outcome::Incomplete(incomplete) =
        evaluate_union(&[], empty(), vec![], &mut denied).unwrap()
    else {
        panic!("retention limit must stop construction")
    };
    assert_eq!(incomplete.charge_point, ChargePoint::CompositeResultRetain);
    assert_eq!(incomplete.limit_kind, LimitKind::ValueOccurrences);
    assert_eq!(incomplete.limit, 0);
    let payload: Vec<Deferred<'_>> = vec![Box::new(|_| {
        calls.borrow_mut().push(2);
        Outcome::Completed(integer(9))
    })];
    assert!(evaluate_union(&[], empty(), payload, &mut denied).is_err());
    assert_eq!(*calls.borrow(), vec![0, 1]);
}

fn deep(levels: usize) -> Value {
    let mut value = make(empty(), &[]);
    for _ in 0..levels {
        value = union(
            &[ValueType::option(ValueType::Composite(declaration()))],
            fixture("Link", 42),
            vec![OptionValue::present(ValueType::Composite(declaration()), value).unwrap()],
        )
        .unwrap();
    }
    value
}

#[derive(Default)]
struct Brackets {
    open: usize,
    close: usize,
}
impl fmt::Write for Brackets {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            match byte {
                b'(' | b'{' | b'[' => self.open += 1,
                b')' | b'}' | b']' => self.close += 1,
                _ => {}
            }
        }
        Ok(())
    }
}

#[trace("FR-321-AC-4")]
#[test]
fn ten_thousand_union_levels_clone_compare_format_and_drop_on_small_stack() {
    std::thread::Builder::new()
        .stack_size(512 * 1024)
        .spawn(|| {
            let left = deep(10_000);
            let right = deep(10_000);
            let copy = left.clone();
            assert_eq!(left.occ(), Integer::from(20_001_u64));
            assert_eq!(
                plan_equality(&left, &right).unwrap().pair_events(),
                &Integer::from(20_001_u64)
            );
            assert!(equal(&left, &right));
            assert_eq!(compare_keys(&left, &right), Some(Ordering::Equal));
            assert!(ValueType::Composite(declaration()).admits(&left));
            let mut sink = Brackets::default();
            write!(&mut sink, "{left:?}").unwrap();
            assert!(sink.open >= 20_001);
            assert_eq!(sink.open, sink.close);
            drop(left);
            assert_eq!(copy.occ(), Integer::from(20_001_u64));
            drop(copy);
            drop(right);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[trace("FR-321-AC-4")]
#[test]
fn shared_union_children_count_each_occurrence_and_clone_shares_root() {
    let child = deep(1_000);
    let root = UnionValue::from_admitted(fixture("Pair", 7), vec![child.clone(), child]);
    assert_eq!(root.occ(), Integer::from(4_003_u64));
    let copy = root.clone();
    let (Value::Union(root), Value::Union(copy)) = (&root, &copy) else {
        panic!("union")
    };
    assert!(Arc::ptr_eq(root, copy));
    let (Value::Union(first), Value::Union(second)) = (&root.payload()[0], &root.payload()[1])
    else {
        panic!("shared union children")
    };
    assert!(Arc::ptr_eq(first, second));
}

#[trace("FR-321-AC-4")]
#[test]
fn union_storage_grows_linearly_with_depth() {
    let retained = |levels| {
        let mut value = None;
        let allocation = allocation_counter::measure(|| value = Some(deep(levels)));
        assert_eq!(value.as_ref().unwrap().occ(), Integer::from(2 * levels + 1));
        assert!(allocation.bytes_current > 0);
        drop(value);
        allocation.bytes_current
    };
    let small = retained(1_000);
    let large = retained(2_000);
    assert!(large > small);
    assert!(
        large <= 3 * small,
        "nested values must retain linear storage"
    );
}

#[trace("FR-321-AC-4")]
#[test]
fn mixed_union_composite_collection_paths_walk_and_drop_on_small_stack() {
    std::thread::Builder::new()
        .stack_size(512 * 1024)
        .spawn(|| {
            let build = || {
                let mut value = make(empty(), &[]);
                let sequence = CollectionType::new(
                    CollectionKind::Sequence,
                    ValueType::Composite(declaration()),
                    None,
                );
                for _ in 0..10_000 {
                    let collection = quire_exact::from_admitted(sequence.clone(), vec![value]);
                    let record = from_admitted_slots(
                        declaration(),
                        vec![FieldValue::Present(collection)].into_boxed_slice(),
                    );
                    value = union(
                        &[ValueType::Composite(declaration())],
                        fixture("Nested", 81),
                        vec![record],
                    )
                    .unwrap();
                }
                value
            };
            let left = build();
            let right = build();
            assert_eq!(left.occ(), Integer::from(30_001_u64));
            // A present slot's pair is the contained value pair, so each link
            // contributes union, record and collection events.
            assert_eq!(
                plan_equality(&left, &right).unwrap().pair_events(),
                &Integer::from(30_001_u64)
            );
            assert!(equal(&left, &right));
            assert_eq!(compare_keys(&left, &right), Some(Ordering::Equal));
            let Value::Union(root) = &left else {
                panic!("union")
            };
            let node_copy = (**root).clone();
            let mut sink = Brackets::default();
            write!(&mut sink, "{node_copy:?}").unwrap();
            assert!(sink.open > 30_000);
            assert_eq!(sink.open, sink.close);
            drop(left);
            drop(node_copy);
            drop(right);
        })
        .unwrap()
        .join()
        .unwrap();
}
