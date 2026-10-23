---
id: FR-385
title: "Keep Bytes and JsonObject as distinct native kernel values"
type: FR
relationships:
  - target: ix://agent-ix/quire-spec-language/FR-056
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-exact/FR-370
    type: references
---
# FR-385: Keep Bytes and JsonObject as distinct native kernel values

## Description

When a caller supplies a native Bytes or JsonObject value, the `quire-exact`
kernel SHALL retain its native type and value kind distinctly through type
admission, equality planning and canonical-key comparison, as FR-370 does for
UUID and Timestamp. The caller owns the mapping from its declared native type
and input representation to this kernel value. The kernel SHALL NOT treat
either kind as a Text, an Integer, a Reference, an object identity or the
other native kind.

## Use case

A caller admits a payload digest as Bytes and a configuration document as a
JsonObject. Equal Bytes compare equal to Bytes, equal JsonObject values compare
equal to JsonObject values, and a String holding the same characters as either
canonical text is a different value of a different type.

## Behavior

1. **Distinct types.** `ValueType::Bytes` and `ValueType::JsonObject` are
   distinct public type variants with distinct `Value` variants.
   `ValueType::admits` accepts each value only against its matching native
   type and rejects a crossed Bytes/JsonObject pair and a Text, Integer,
   Reference, UUID or Timestamp offered as either.
2. **Native payloads.** A Bytes value is an octet sequence of any length, the
   empty sequence included. Its sole admitted caller payload is the lowercase
   ASCII hexadecimal text of the octets in sequence order, two digits per
   octet, the empty text for the empty sequence. A JsonObject value is one
   JSON value of any kind (object, array, string, number, boolean or
   `null`). Its sole admitted caller payload is its RFC 8785 (JCS)
   serialization: the text parses as one JSON value with no repeated member
   name at any depth, every number is finite in IEEE binary64, and serializing
   the parsed value under JCS gives the text again, so members are in UTF-16
   code unit order, there is no insignificant whitespace, and a number is the
   shortest ECMAScript spelling of its binary64 value. Admission rejects a
   noncanonical payload without normalization, repair, rounding or a partially
   constructed value.
3. **Constructor refusal.** The kernel's `ConstructionRefusal { component:
   Component::Value, cause }` gains two causes, `BytesNoncanonical` for Bytes
   text that is not canonical lowercase hexadecimal with an even digit count
   and `JsonObjectNoncanonical` for JsonObject text that is not one JSON value
   or is not its own JCS serialization. The caller diagnoses its wire envelope
   before the constructor is invoked. A payload refusal SHALL NOT use the
   internal `CheckedInvariant` fault.
4. **Typed equality.** Two admitted Bytes values follow the existing
   equality-plan schedule as one terminal pair contributing one
   `equality.pair` event, equal exactly when their octet sequences are
   identical; two admitted JsonObject values do likewise, equal exactly when
   their canonical texts are identical. No implicit conversion creates
   equality across the two kinds or against another value kind; a direct plan
   over unlike kinds returns `CheckedInvariantCause::ValueKindMismatch`.
5. **Canonical key.** Each kind has a total `compare_keys` order over values
   of its own type, comparing the canonical payload text as unsigned ASCII
   bytes with a proper prefix first, so Bytes `02` sorts before `0a`, `0a` before its extension `0a00`, and `0200` before `10`; two
   values have equal keys exactly when the typed equality plan says they are
   equal. A mixed-kind comparison returns `None`. The key authorizes no
   ordering operator.
6. **Size accounting.** A Bytes value's variable size is its octet count and a
   JsonObject value's is the byte length of its canonical text, charged through
   the existing semantic-size path.
7. **Unchanged kinds.** Admission, equality, key comparison and accounting for
   every other `ValueType` and `Value` variant retain their behavior. A
   `Duration` is a `Quantity` over a unit of the caller's declaration and
   needs no kernel variant.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-385-AC-1 | A Bytes value is admitted only by `ValueType::Bytes` and a JsonObject value only by `ValueType::JsonObject`; crossed kinds and Text, Integer, Reference, UUID and Timestamp values are rejected by both, without a conversion. | Test |
| FR-385-AC-2 | Bytes payloads the empty text, `00`, `00ff` and `deadbeef` construct; `00FF`, `0`, `0x00`, `00 ff`, ` 00` and `gg` return `BytesNoncanonical` with no value and no `CheckedInvariant` fault. | Test |
| FR-385-AC-3 | JsonObject payloads `{}`, `[]`, `3`, `"x"`, `null`, `{"a":1,"b":[true,null]}` and `{"a":9007199254740992}`, the literal-character strings `"é"` (U+00E9), `"\n"`, `"\u001f"`, `"\\"`, `"\""` and `"/"`, and the members `{"𐀀":1,"\uffff":2}` (name U+10000, whose UTF-16 code units D800 DC00 precede U+FFFF, then name U+FFFF written as a literal character) construct; `{"b":1,"a":1}`, `{"a": 1}`, `{"a":1.0}`, `{"a":1e2}`, `{"a":-0}`, `{"a":9007199254740993}`, `{"a":1e999}`, `{"a":1,"a":2}`, `"\u00e9"` (an escape JCS writes literally), `"\/"`, `"\u001F"` (uppercase hex), `"\u000a"` (JCS writes `\n`), `"\ud83d\ude00"` (an escaped supplementary-plane character), `"\ud800"` (a lone surrogate), `{"\uffff":2,"𐀀":1}` (code point order, not UTF-16 code unit order) written with the same literal characters as the accepted members, the empty text and `{` return `JsonObjectNoncanonical` with no value. A payload of 100000 nested arrays constructs without a stack overflow. | Test |
| FR-385-AC-4 | Each same-kind pair follows the one-terminal-pair equality schedule: Bytes `00ff` against `00ff` is `true`, against `00fe` and `00ff00` `false`; the JsonObject `{"a":1,"b":[true,null]}` against itself is `true`, against `{"a":1,"b":[null,true]}`, `{"a":2,"b":[true,null]}` and `{"a":1}` `false`; the planned pair count and charges equal the Boolean-leaf schedule. | Test |
| FR-385-AC-5 | A direct equality plan pairing Bytes or JsonObject with another kind returns `Refusal::CheckedInvariant { cause: ValueKindMismatch }`, and a direct mixed-kind `compare_keys` returns `None`. | Test |
| FR-385-AC-6 | `compare_keys` over Bytes `02` and `0a` orders `02` first, over `0a` and `0a00` orders `0a` first (proper prefix), and over `0200` and `10` orders `0200` first, equal payloads have equal keys, and neither kind exposes an ordering operator. | Test |
| FR-385-AC-7 | Bytes `deadbeef` accounts four variable octets and the JsonObject `{"a":1}` accounts seven; existing scalar, composite, UUID and Timestamp admission, equality, key and accounting vectors retain their results. | Test |

## Dependencies

- QSL `ix://agent-ix/quire-spec-language/FR-056` "Native Bytes, JsonObject and Duration" defines the canonical payloads and the closed caller wire forms and maps the two constructor causes to `invalid_runtime_input`/`invalid-value`; QSpec `ix://agent-ix/quire-specification/FR-149` owns their typed equality and key relation, and QSpec `ix://agent-ix/quire-specification/FR-322` resolves the member types.
- The spellings `BytesNoncanonical` and `JsonObjectNoncanonical` are this kernel's names for the two payload classes that FR-056 leaves to IR-704's Rust variant spellings; IR-704 owns the final variant spellings.
- FR-370 states the UUID and Timestamp natives this requirement parallels.
