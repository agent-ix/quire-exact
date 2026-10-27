// SPDX-License-Identifier: AGPL-3.0-or-later
//! The `Map<Text, V>` value type through the public API (FR-386, TC-935).
use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};

use ix_trace_rs::trace;
use quire_exact::{
    admit_text, from_admitted, CardinalityBound, CollectionKind, CollectionType, MapType, Meter,
    OptionValue, ScalarLimits, TextPayload, TextProfile, TextType, Value, ValueType,
};

/// Levels of nesting in the deep types.
const DEEP: usize = 100_000;

fn bound(minimum: u64, maximum: u64) -> Option<CardinalityBound> {
    Some(CardinalityBound::new(minimum, maximum).expect("ordered"))
}

fn map_of(values: ValueType, bound: Option<CardinalityBound>) -> ValueType {
    ValueType::map(MapType::new(values, bound))
}

fn hash_of(value_type: &ValueType) -> u64 {
    let mut hasher = DefaultHasher::new();
    value_type.hash(&mut hasher);
    hasher.finish()
}

fn on_small_stack(walk: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(512 * 1024)
        .spawn(walk)
        .expect("spawn the walk thread")
        .join()
        .expect("the walk completes without overflowing the stack");
}

#[test]
#[trace("TC-935", "FR-386-AC-1")]
fn a_map_type_reports_its_value_type_and_bound() {
    for declared in [None, bound(1, 3)] {
        let built = map_of(ValueType::Integer, declared);
        let ValueType::Map(ref map) = built else {
            panic!("ValueType::map builds a Map");
        };
        assert!(*map.values() == ValueType::Integer);
        assert_eq!(map.bound(), declared);
    }
}

#[test]
#[trace("TC-935", "FR-386-AC-2")]
fn map_types_are_equal_exactly_when_value_type_and_bound_are_equal() {
    let reference = map_of(ValueType::Boolean, bound(1, 3));
    let same = map_of(ValueType::Boolean, bound(1, 3));
    assert!(reference == same);
    assert_eq!(hash_of(&reference), hash_of(&same));

    let differing = [
        map_of(ValueType::Integer, bound(1, 3)),
        map_of(ValueType::Boolean, bound(1, 4)),
        map_of(ValueType::Boolean, bound(0, 3)),
        map_of(ValueType::Boolean, None),
    ];
    for other in &differing {
        assert!(reference != *other);
    }

    let unbounded = map_of(ValueType::Boolean, None);
    let full = map_of(ValueType::Boolean, bound(0, u64::MAX));
    assert!(unbounded != full);
}

#[test]
#[trace("TC-935", "FR-386-AC-2")]
fn a_map_type_is_not_a_collection_or_option_type() {
    for declared in [None, bound(1, 3)] {
        let map = map_of(ValueType::Boolean, declared);
        for kind in CollectionKind::ALL {
            let collection =
                ValueType::collection(CollectionType::new(kind, ValueType::Boolean, declared));
            assert!(map != collection);
            assert!(collection != map);
        }
        let option = ValueType::option(ValueType::Boolean);
        assert!(map != option);
    }
}

#[test]
#[trace("TC-935", "FR-386-AC-3")]
fn a_map_type_compares_by_its_whole_nested_chain() {
    let nested = |inner: ValueType| {
        [
            map_of(map_of(inner.clone(), None), bound(0, 2)),
            map_of(ValueType::option(inner.clone()), None),
            map_of(
                ValueType::collection(CollectionType::new(CollectionKind::Sequence, inner, None)),
                None,
            ),
        ]
    };
    for (left, right) in nested(ValueType::Boolean)
        .iter()
        .zip(nested(ValueType::Boolean))
    {
        assert!(*left == right);
        assert_eq!(hash_of(left), hash_of(&right));
    }
    for (left, right) in nested(ValueType::Boolean)
        .iter()
        .zip(nested(ValueType::Integer))
    {
        assert!(*left != right);
    }
}

/// `DEEP` map links around `innermost`.
fn deep_map(innermost: ValueType) -> ValueType {
    (0..DEEP).fold(innermost, |inner, _| map_of(inner, None))
}

/// `DEEP` links cycling `Map`, `Option` and unbounded `Sequence` around `innermost`.
fn deep_mixed(innermost: ValueType) -> ValueType {
    (0..DEEP).fold(innermost, |inner, level| match level % 3 {
        0 => map_of(inner, bound(0, 1)),
        1 => ValueType::option(inner),
        _ => ValueType::collection(CollectionType::new(CollectionKind::Sequence, inner, None)),
    })
}

/// Counts opening and closing parentheses without buffering the output.
#[derive(Default)]
struct ParenCount {
    open: usize,
    close: usize,
}

impl Write for ParenCount {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            match byte {
                b'(' => self.open += 1,
                b')' => self.close += 1,
                _ => {}
            }
        }
        Ok(())
    }
}

#[test]
#[trace("TC-935", "FR-386-AC-4")]
fn deep_map_types_clone_compare_hash_format_and_drop() {
    on_small_stack(|| {
        for build in [deep_map as fn(ValueType) -> ValueType, deep_mixed] {
            let value_type = build(ValueType::Boolean);
            let copy = value_type.clone();
            assert!(value_type == copy);
            assert_eq!(hash_of(&value_type), hash_of(&copy));

            let other = build(ValueType::Integer);
            assert!(value_type != other);

            let mut sink = ParenCount::default();
            write!(sink, "{value_type:?}").expect("format");
            assert!(sink.open >= DEEP);
            assert_eq!(sink.open, sink.close);

            drop(value_type);
            drop(copy);
            drop(other);
        }
    });
}

#[test]
#[trace("TC-935", "FR-386-AC-5")]
fn a_map_type_debug_names_the_value_type_and_bound() {
    assert_eq!(
        format!("{:?}", map_of(ValueType::Boolean, None)),
        "Map(MapType { values: Boolean, bound: None })"
    );
    assert_eq!(
        format!(
            "{:?}",
            map_of(ValueType::option(ValueType::Boolean), bound(1, 3))
        ),
        "Map(MapType { values: Option(Boolean), bound: Some(CardinalityBound { minimum: 1, \
         maximum: 3 }) })"
    );
}

#[test]
#[trace("TC-935", "FR-386-AC-7")]
fn a_map_type_alternate_debug_names_the_value_type_and_bound() {
    assert_eq!(
        format!("{:#?}", map_of(ValueType::Boolean, None)),
        "Map(\n    MapType {\n        values: Boolean,\n        bound: None,\n    },\n)"
    );
    assert_eq!(
        format!(
            "{:#?}",
            map_of(ValueType::option(ValueType::Boolean), bound(1, 3))
        ),
        "Map(\n    MapType {\n        values: Option(\n            Boolean,\n        ),\n        \
         bound: Some(\n            CardinalityBound {\n                minimum: 1,\n                \
         maximum: 3,\n            },\n        ),\n    },\n)"
    );
}

#[test]
#[trace("TC-935", "FR-386-AC-6")]
fn a_map_type_admits_no_value() {
    let map = map_of(ValueType::Boolean, None);
    let sequence = CollectionType::new(CollectionKind::Sequence, ValueType::Boolean, None);
    let collection = from_admitted(sequence, Vec::new());
    let option = OptionValue::from_admitted(ValueType::Boolean, None);
    let payload = TextPayload::from_utf8(b"a").expect("utf-8");
    let text_type = TextType::new(0, 4, TextProfile::Nfc).expect("ordered");
    let limit = u64::MAX;
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: limit,
        decimal_digits: limit,
        scale_expansion: limit,
        text_input_bytes: limit,
        text_scalars: limit,
        normalized_scalars: limit,
        unit_edges: limit,
        value_occurrences: limit,
        work_units: limit,
        result_units: limit,
    });
    let text = Value::Text(
        admit_text(&payload, &text_type, &mut meter)
            .completed()
            .expect("admitted text"),
    );
    for value in [text, collection, option] {
        assert!(!map.admits(&value));
    }
}
