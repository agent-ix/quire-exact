// SPDX-License-Identifier: AGPL-3.0-or-later
//! The bounded quantity value type through the public API (FR-387, TC-936).
use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};

use ix_trace_rs::trace;
use quire_exact::{
    admit_text, BoundedQuantityType, CollectionKind, CollectionType, IllTyped, IllTypedCause,
    Integer, IntegerInterval, MapType, Meter, NodeKey, OptionValue, Quantity, Rational,
    RationalDomain, ScalarLimits, TextPayload, TextProfile, TextType, UnitId, Value, ValueType,
};

/// Levels of nesting in the deep types.
const DEEP: usize = 100_000;

fn unit(byte: u8) -> UnitId {
    UnitId::declared(
        NodeKey::new(
            "test",
            vec![quire_exact::PathSegment::Text(byte.to_string())],
            0,
        )
        .unwrap(),
    )
}

fn ratio(numerator: i64, denominator: i64) -> Rational {
    Rational::new(Integer::from(numerator), Integer::from(denominator)).expect("nonzero")
}

fn amount(numerator: i64, denominator: i64, of: UnitId) -> Quantity {
    Quantity::new(ratio(numerator, denominator), of)
}

fn whole(magnitude: i64, of: UnitId) -> Quantity {
    amount(magnitude, 1, of)
}

fn bounded_type(
    of: UnitId,
    min: Option<i64>,
    max: Option<i64>,
) -> Result<BoundedQuantityType, IllTyped> {
    let min = min.map(|m| whole(m, of.clone()));
    let max = max.map(|m| whole(m, of.clone()));
    BoundedQuantityType::new(of, min, max)
}

fn bounded(of: UnitId, min: Option<i64>, max: Option<i64>) -> ValueType {
    ValueType::bounded_quantity(bounded_type(of, min, max).expect("bounds in the unit"))
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

fn quantity_value(magnitude: i64, of: UnitId) -> Value {
    Value::Quantity(whole(magnitude, of))
}

fn text_value() -> Value {
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
    Value::Text(
        admit_text(&payload, &text_type, &mut meter)
            .completed()
            .expect("admitted text"),
    )
}

fn option_value() -> Value {
    OptionValue::from_admitted(ValueType::Boolean, None)
}

fn rational_type() -> ValueType {
    let span =
        |low: i64, high: i64| IntegerInterval::spanning(Integer::from(low), Integer::from(high));
    ValueType::Rational(RationalDomain::new(span(-9, 9), span(1, 9)).expect("domain"))
}

#[test]
#[trace("TC-936", "FR-387-AC-1")]
fn a_bounded_quantity_type_reports_its_unit_and_bounds() {
    let u = unit(1);
    for (min, max) in [
        (None, None),
        (Some(0), None),
        (None, Some(100)),
        (Some(0), Some(100)),
    ] {
        let built = bounded_type(u.clone(), min, max).expect("valid");
        assert_eq!(built.unit(), &u);
        assert_eq!(built.min(), min.map(|m| whole(m, u.clone())).as_ref());
        assert_eq!(built.max(), max.map(|m| whole(m, u.clone())).as_ref());
        let ValueType::BoundedQuantity(ref wrapped) = ValueType::bounded_quantity(built.clone())
        else {
            panic!("ValueType::bounded_quantity builds a BoundedQuantity");
        };
        assert_eq!(**wrapped, built);
    }
}

#[test]
#[trace("TC-936", "FR-387-AC-2")]
fn only_a_bound_in_another_unit_is_refused() {
    let (u, other) = (unit(1), unit(2));
    let refused = |min, max| {
        BoundedQuantityType::new(u.clone(), min, max)
            .expect_err("a bound in another unit")
            .cause
    };
    assert_eq!(
        refused(Some(whole(0, other.clone())), None),
        IllTypedCause::MalformedBoundedQuantityType
    );
    assert_eq!(
        refused(None, Some(whole(9, other.clone()))),
        IllTypedCause::MalformedBoundedQuantityType
    );

    for (min, max) in [(3, 3), (3, 5), (5, 3), (4, 2)] {
        let built = bounded_type(u.clone(), Some(min), Some(max)).expect("valid, possibly empty");
        assert_eq!(built.min(), Some(&whole(min, u.clone())));
        assert_eq!(built.max(), Some(&whole(max, u.clone())));
    }
    for (min, max) in [
        (amount(1, 3, u.clone()), amount(1, 2, u.clone())),
        (amount(1, 2, u.clone()), amount(1, 3, u.clone())),
    ] {
        let built = BoundedQuantityType::new(u.clone(), Some(min.clone()), Some(max.clone()))
            .expect("valid");
        assert_eq!(built.min(), Some(&min));
        assert_eq!(built.max(), Some(&max));
    }
}

#[test]
#[trace("TC-936", "FR-387-AC-3")]
fn bounded_quantity_types_are_equal_exactly_when_unit_and_bounds_are_equal() {
    let u = unit(1);
    let reference = bounded(u.clone(), Some(0), Some(100));
    let same = bounded(u.clone(), Some(0), Some(100));
    assert!(reference == same);
    assert_eq!(hash_of(&reference), hash_of(&same));

    // Equal magnitudes in different spellings normalise to the same bound.
    let spelled = ValueType::bounded_quantity(
        BoundedQuantityType::new(
            u.clone(),
            Some(amount(0, 5, u.clone())),
            Some(amount(200, 2, u.clone())),
        )
        .expect("valid"),
    );
    assert!(reference == spelled);
    assert_eq!(hash_of(&reference), hash_of(&spelled));

    let differing = [
        bounded(unit(2), Some(0), Some(100)),
        bounded(u.clone(), Some(1), Some(100)),
        bounded(u.clone(), Some(0), Some(99)),
        bounded(u.clone(), None, None),
        bounded(u.clone(), Some(0), None),
        bounded(u.clone(), None, Some(100)),
    ];
    for other in &differing {
        assert!(reference != *other);
    }
    assert!(bounded(u.clone(), Some(0), None) != bounded(u.clone(), None, Some(100)));
    assert!(bounded(u.clone(), None, None) != ValueType::Quantity(u.clone()));
    assert!(ValueType::Quantity(u.clone()) != bounded(u.clone(), None, None));
    assert!(bounded(u.clone(), None, None) != rational_type());
    for each in [&reference, &differing[3]] {
        assert!(*each != ValueType::option(each.clone()));
        assert!(ValueType::option(each.clone()) != *each);
    }
}

#[test]
#[trace("TC-936", "FR-387-AC-4")]
fn a_bounded_quantity_type_compares_by_its_whole_nested_chain() {
    let u = unit(1);
    let nested = |inner: ValueType| {
        [
            ValueType::option(inner.clone()),
            ValueType::collection(CollectionType::new(
                CollectionKind::Sequence,
                inner.clone(),
                None,
            )),
            ValueType::map(MapType::new(inner.clone(), None)),
            ValueType::option(ValueType::map(MapType::new(
                ValueType::collection(CollectionType::new(CollectionKind::Set, inner, None)),
                None,
            ))),
        ]
    };
    for (left, right) in nested(bounded(u.clone(), Some(0), Some(100)))
        .iter()
        .zip(nested(bounded(u.clone(), Some(0), Some(100))))
    {
        assert!(*left == right);
        assert_eq!(hash_of(left), hash_of(&right));
    }
    for (left, right) in nested(bounded(u.clone(), Some(0), Some(100)))
        .iter()
        .zip(nested(bounded(u.clone(), Some(0), Some(99))))
    {
        assert!(*left != right);
    }
}

fn deep(innermost: ValueType) -> ValueType {
    (0..DEEP).fold(innermost, |inner, _| ValueType::option(inner))
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
#[trace("TC-936", "FR-387-AC-5")]
fn deep_bounded_quantity_types_clone_compare_hash_format_and_drop() {
    on_small_stack(|| {
        let u = unit(1);
        let value_type = deep(bounded(u.clone(), Some(0), Some(100)));
        let copy = value_type.clone();
        assert!(value_type == copy);
        assert_eq!(hash_of(&value_type), hash_of(&copy));

        let other = deep(bounded(u.clone(), Some(0), Some(99)));
        assert!(value_type != other);

        let mut sink = ParenCount::default();
        write!(sink, "{value_type:?}").expect("format");
        assert!(sink.open >= DEEP);
        assert_eq!(sink.open, sink.close);

        drop(value_type);
        drop(copy);
        drop(other);
    });
}

const ZERO: &str = "Rational { numerator: Integer(0), denominator: Integer(1) }";
const HUNDRED: &str = "Rational { numerator: Integer(100), denominator: Integer(1) }";

#[test]
#[trace("TC-936", "FR-387-AC-6")]
fn a_bounded_quantity_type_debug_prints_the_derived_layout() {
    let u = unit(1);
    let unit_text = format!("{u:?}");
    assert_eq!(
        format!("{:?}", bounded(u.clone(), Some(0), Some(100))),
        format!(
            "BoundedQuantity(BoundedQuantityType {{ unit: {unit_text}, min: Some(Quantity {{ \
             magnitude: {ZERO}, unit: {unit_text} }}), max: Some(Quantity {{ magnitude: \
             {HUNDRED}, unit: {unit_text} }}) }})"
        )
    );
    assert_eq!(
        format!("{:?}", bounded(u.clone(), None, None)),
        format!(
            "BoundedQuantity(BoundedQuantityType {{ unit: {unit_text}, min: None, max: None }})"
        )
    );
}

#[test]
#[trace("TC-936", "FR-387-AC-6")]
fn a_bounded_quantity_type_alternate_debug_prints_the_nested_layout() {
    let u = unit(1);
    assert_eq!(
        format!("{u:#?}"),
        format!("{u:?}"),
        "the unit prints on one line"
    );
    let unit_text = format!("{u:?}");
    let rational = |value: &str| {
        format!(
            "Rational {{\n                    numerator: Integer(\n                        \
             {value},\n                    ),\n                    denominator: \
             Integer(\n                        1,\n                    ),\n                }}"
        )
    };
    let bound = |name: &str, value: &str| {
        format!(
            "        {name}: Some(\n            Quantity {{\n                magnitude: {},\n                \
             unit: {unit_text},\n            }},\n        ),\n",
            rational(value)
        )
    };
    assert_eq!(
        format!("{:#?}", bounded(u.clone(), Some(0), Some(100))),
        format!(
            "BoundedQuantity(\n    BoundedQuantityType {{\n        unit: {unit_text},\n{}{}    \
             }},\n)",
            bound("min", "0"),
            bound("max", "100")
        )
    );
    assert_eq!(
        format!("{:#?}", bounded(u.clone(), None, None)),
        format!(
            "BoundedQuantity(\n    BoundedQuantityType {{\n        unit: {unit_text},\n        \
             min: None,\n        max: None,\n    }},\n)"
        )
    );
}

#[test]
#[trace("TC-936", "FR-387-AC-7")]
fn admission_is_the_inclusive_range_check() {
    let u = unit(1);
    let closed = bounded(u.clone(), Some(0), Some(100));
    for magnitude in [0, 50, 100] {
        assert!(closed.admits(&quantity_value(magnitude, u.clone())));
    }
    for magnitude in [-1, 101] {
        assert!(!closed.admits(&quantity_value(magnitude, u.clone())));
    }
    let from_one = bounded(u.clone(), Some(1), Some(100));
    assert!(!from_one.admits(&Value::Quantity(amount(1, 2, u.clone()))));
    assert!(from_one.admits(&Value::Quantity(amount(3, 2, u.clone()))));

    let open_above = bounded(u.clone(), Some(0), None);
    assert!(open_above.admits(&quantity_value(1_000_000, u.clone())));
    assert!(!open_above.admits(&quantity_value(-1, u.clone())));
    let open_below = bounded(u.clone(), None, Some(100));
    assert!(open_below.admits(&quantity_value(-1_000_000, u.clone())));
    assert!(!open_below.admits(&quantity_value(101, u.clone())));
}

#[test]
#[trace("TC-936", "FR-387-AC-8")]
fn admission_refuses_other_units_and_non_quantities_unlike_the_unbounded_type() {
    let (u, other) = (unit(1), unit(2));
    let closed = bounded(u.clone(), Some(0), Some(100));
    for value in [
        quantity_value(50, other.clone()),
        text_value(),
        Value::Boolean(true),
        option_value(),
    ] {
        assert!(!closed.admits(&value));
    }
    assert!(ValueType::Quantity(u.clone()).admits(&quantity_value(101, u.clone())));
    assert!(!closed.admits(&quantity_value(101, u.clone())));
}

#[test]
#[trace("TC-936", "FR-387-AC-9")]
fn identity_is_the_effective_bounds_whatever_the_caller_folded_them_from() {
    let u = unit(1);
    // Two callers fold different alias chains to the same effective bounds.
    let fold = |chain: &[(i64, i64)]| {
        let min = chain.iter().map(|bound| bound.0).max().expect("chain");
        let max = chain.iter().map(|bound| bound.1).min().expect("chain");
        bounded_type(u.clone(), Some(min), Some(max)).expect("valid")
    };
    let first = fold(&[(0, 200), (-5, 100)]);
    let second = fold(&[(0, 100)]);
    assert_eq!(first, second);
    assert_eq!(first.unit(), second.unit());
    assert_eq!(first.min(), second.min());
    assert_eq!(first.max(), second.max());

    let refined = bounded(u.clone(), Some(0), Some(50));
    assert!(ValueType::bounded_quantity(first) != refined);
    assert!(bounded(u.clone(), Some(0), Some(100)) != refined);

    // The only constructor refusal is a unit mismatch, never inverted bounds.
    assert!(bounded_type(u.clone(), Some(5), Some(3)).is_ok());
    assert!(bounded_type(u.clone(), Some(3), Some(3)).is_ok());
    assert_eq!(
        BoundedQuantityType::new(u.clone(), Some(whole(0, unit(2))), None)
            .expect_err("unit mismatch")
            .cause,
        IllTypedCause::MalformedBoundedQuantityType
    );
}

#[test]
#[trace("TC-936", "FR-387-AC-10")]
fn an_inverted_type_is_a_valid_empty_type_that_keeps_both_bounds() {
    let (u, other) = (unit(1), unit(2));
    let inverted = bounded_type(u.clone(), Some(5), Some(3)).expect("valid, empty");
    assert_eq!(inverted.min(), Some(&whole(5, u.clone())));
    assert_eq!(inverted.max(), Some(&whole(3, u.clone())));
    let empty = ValueType::bounded_quantity(inverted);
    for magnitude in [2, 3, 4, 5, 100] {
        assert!(!empty.admits(&quantity_value(magnitude, u.clone())));
    }
    for value in [
        quantity_value(4, other.clone()),
        text_value(),
        Value::Boolean(true),
        option_value(),
    ] {
        assert!(!empty.admits(&value));
    }

    let same = bounded(u.clone(), Some(5), Some(3));
    assert!(empty == same);
    assert_eq!(hash_of(&empty), hash_of(&same));
    assert!(empty != bounded(u.clone(), Some(4), Some(2)));

    let text = format!("{empty:?}");
    assert!(text.contains("min: Some(") && text.contains("max: Some("));
    assert!(text.contains("numerator: Integer(5)") && text.contains("numerator: Integer(3)"));
}
