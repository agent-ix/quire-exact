---
id: SR-2026110101303
title: IR-1317 combined gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-exact PR #129 and agent-ix/quire-eval PR #19; IR-1317; membership,
  checked collection removal and quantity-power descriptors'
review_set: all
---

## Summary

One combined independent review of quire-exact PR #129 and quire-eval PR #19 for IR-1317. Exact owns canonical float-leaf collection membership; quire-eval owns collection removal and the three checked descriptors. Ordinary equality remains separate. QuantityOperation::Power already belongs to quire-semantic-value; the new checked descriptor deliberately supplies no second arithmetic implementation. Public package admission and source/producer qualification are not inferred from descriptor controls.

## Verdict

**FAIL (inherited repository baseline); scoped delta PASS**. Exact matrix: 266 criteria, 230 tagged, 23 method-without-symbol, 13 untagged. Eval matrix: 21 criteria, 20 tagged, 1 method-without-symbol, zero untagged. All four new Exact, seven collection-removal and two power-read criteria plus amended FR-377-AC-6 have actual matching bindings. No source/test stubs or manufactured acceptance were found in the scoped implementation. No matrix was authored.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Exact repository retains thirteen pre-existing untagged criteria; the repository-wide computed matrix is not fully backed, although this pair introduces no new gap. | quire-exact/spec/functional/FR-359-cumulative-meter-boundary.md:criterion row |

## Coverage

No applicable AssuranceProfile exists in either spec tree. Existing ArchitectureDescription documents provide context but are not enforced review profiles. This reviewer ran no Cargo, Kani, CBMC or other heavy jobs. Supplied final own-lane logs were inspected: quire-eval all-feature workspace 74 pass / 0 fail / 0 ignored; Exact default 402 pass / 1 inherited fail / 2 ignored; Exact test-support 414 pass / 1 inherited fail / 2 ignored. The only failure is value::value_type::tests::value_type_debug_preserves_structure_and_fields, reproduced on clean main with identical actual/expected Debug strings. Formatter, native/bare-metal Clippy, no_std builds, warning-denied docs and deny checks pass in both repositories. All fourteen scoped criterion bindings are present and no new ignored test or gate suppression was introduced. The thirteen untagged Exact criteria are unchanged baseline obligations, not newly discharged here. No plan was supplied.

Plan completion: not assessed

The optional standalone semantic gap fan-out and Jev-dependent criterion-strength analysis were not run; neither was requested or needed for this bounded mixed diff. Required code/spec intent checking was performed directly in this one reviewer invocation. No child agent was spawned.

Examined scope:

```yaml
scope:
- id: FR-377-AC-6
  path: quire-exact/spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, two composites of one declaration key
    and different slot counts, and a pair of `Float` members each return `None` from
    `compare_keys`. As the two members of a `Set`, each refuses `CanonicalKeyUnavailable`
    through `form_grouped`. Through `form`, the composite/integer and mismatched-slot
    pairs refuse `ValueKindMismatch` before ordering; the distinct float pair passes
    bit-identity membership then refuses `CanonicalKeyUnavailable` at ordering.
- id: FR-393-AC-1
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: At both widths identical NaN patterns compare true; changed payload, sign,
    signaling bit, signed zero and infinity sign compare false.
- id: FR-393-AC-2
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: Float leaves nested in options and collections use the same membership
    relation, visiting all occurrence pairs after an unequal leaf.
- id: FR-393-AC-3
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: Nested comparisons retain the exact walk/test charge vectors and no result-retain
    charge; denial at either point returns Incomplete with the admitted prefix unchanged.
- id: FR-393-AC-4
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: Wrong width and wrong value kind preserve typed membership refusals; ordinary
    equality still refuses direct and nested float operands.
- id: FR-003-AC-1
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Borrowed reads distinguish both operations and preserve each ordered child
    handle and checked type/location metadata.
- id: FR-003-AC-2
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Drop removes just the leading sequence/ordered-set occurrence, preserving
    duplicates/order and the empty case; set/bag operands refuse before charging.
- id: FR-003-AC-3
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Remove deletes exactly the first match for every collection kind, reducing
    bag multiplicity by one, and preserves all occurrences on misses and empty inputs.
- id: FR-003-AC-4
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Both operations preserve kind/element type and compute the declared bounded
    or unbounded result type, including zero and maximum bounds.
- id: FR-003-AC-5
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: A wrong-type removal probe refuses before charges; canonical type-environment
    admission remains authoritative.
- id: FR-003-AC-6
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Drop charges exactly survivor formation; remove charges the contains scan
    then formation, with a bag's distinct-member visiting rule and no Boolean result
    retain.
- id: FR-003-AC-7
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: A denied scan or formation charge preserves the admitted charge prefix
    and exposes no partial result.
- id: FR-004-AC-1
  path: quire-eval/spec/functional/FR-004-quantity-power-node.md
  role: examined
  excerpt: A quantity-power read retains the distinct operation and mapped base/exponent
    handles in that order, including when the source handles have descending indices.
- id: FR-004-AC-2
  path: quire-eval/spec/functional/FR-004-quantity-power-node.md
  role: examined
  excerpt: The read borrows the original checked type and location without reconstructing
    either metadata value.
- id: FR-359-AC-9
  path: quire-exact/spec/functional/FR-359-cumulative-meter-boundary.md
  role: context_only
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference`
    uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand
    charge with at most two semantic-size requests, arithmetic charge with one, result-retain
    charge with none) over the full `i64` operand range, plus separate repeated-kind
    charge controls with at most two requests each. Symbolic `i64` operands and symbolic
    limits including `ResultUnits` cover first-short and simultaneous shortages and
    absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and
    600 seconds, the basel
- id: FR-359-AC-10
  path: quire-exact/spec/functional/FR-359-cumulative-meter-boundary.md
  role: context_only
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests
    below pass their own assertions under their original domains, oracle, completion
    and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds. Record their
    individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test
    or altered consumer harness substitutes for it.
- id: FR-371-AC-6
  path: quire-exact/spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic
    mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second
    native ceiling, 16GiB memory ceiling, completion cover and counterexample checks.
    Healthy verification and both substantive falsifications are required. Existing
    all-four-operation, accounting/denial and refinement obligations remain required.
    A compiler error, MemoryExhausted, timeout, missing mutation execution or missing
    teardown is inconclusive and does not satisfy this criterion.
- id: FR-384-AC-5
  path: quire-exact/spec/functional/FR-384-admitted-native-integer-refinement.md
  role: context_only
  excerpt: Genuine CG TC-027 healthy, TC-027 bound mutant, TC-025 arithmetic mutant,
    IR-917 clause implication and IR-948 V2 contract profiles execute the reviewed
    production admitted API over their original inputs, domains and assertions. Healthy
    cases verify with their final completion covers at 16 GiB; each required substantive
    mutant falsifies with original-domain native playback. The IR-948 impossible-precondition
    control must complete as CoverUnsatisfied at the same ceiling, not resource exhaustion
    or a vacuous Verified. Unwind3, Cadical and existing 600-second ceilings remain
    unchanged where speci
- id: FR-385-AC-1
  path: quire-exact/spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: context_only
  excerpt: A Bytes value is admitted only by `ValueType::Bytes` and a JsonObject value
    only by `ValueType::JsonObject`; crossed kinds and Text, Integer, Reference, UUID
    and Timestamp values are rejected by both, without a conversion.
- id: FR-385-AC-2
  path: quire-exact/spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: context_only
  excerpt: Bytes payloads the empty text, `00`, `00ff` and `deadbeef` construct; `00FF`,
    `0`, `0x00`, `00 ff`, ` 00` and `gg` return `BytesNoncanonical` with no value
    and no `CheckedInvariant` fault.
- id: FR-385-AC-3
  path: quire-exact/spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: context_only
  excerpt: "JsonObject payloads `{}`, `[]`, `3`, `\"x\"`, `null`, `{\"a\":1,\"b\"\
    :[true,null]}` and `{\"a\":9007199254740992}`, the literal-character strings `\"\
    \xE9\"` (U+00E9), `\"\\n\"`, `\"\\u001f\"`, `\"\\\\\"`, `\"\\\"\"` and `\"/\"\
    `, and the members `{\"\U00010000\":1,\"\uFFFF\":2}` (printed in canonical form:\
    \ the member names are the literal characters U+10000, whose UTF-16 code units\
    \ D800 DC00 precede U+FFFF, then U+FFFF, with no escape in the payload) construct;\
    \ `{\"b\":1,\"a\":1}`, `{\"a\": 1}`, `{\"a\":1.0}`, `{\"a\":1e2}`, `{\"a\":-0}`,\
    \ `{\"a\":9007199254740993}`, `{\"a\":1e999}`, `{\"a\":1,\"a\":2}`, `\"\\u00e9\"\
    ` (an escape JCS writes literally), `\"\\/\"`, `\"\\u"
- id: FR-385-AC-4
  path: quire-exact/spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: context_only
  excerpt: 'Each same-kind pair follows the one-terminal-pair equality schedule: Bytes
    `00ff` against `00ff` is `true`, against `00fe` and `00ff00` `false`; the JsonObject
    `{"a":1,"b":[true,null]}` against itself is `true`, against `{"a":1,"b":[null,true]}`,
    `{"a":2,"b":[true,null]}` and `{"a":1}` `false`; the planned pair count and charges
    equal the Boolean-leaf schedule.'
- id: FR-385-AC-5
  path: quire-exact/spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: context_only
  excerpt: 'A direct equality plan pairing Bytes or JsonObject with another kind returns
    `Refusal::CheckedInvariant { cause: ValueKindMismatch }`, and a direct mixed-kind
    `compare_keys` returns `None`.'
- id: FR-385-AC-6
  path: quire-exact/spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: context_only
  excerpt: '`compare_keys` over Bytes `02` and `0a` orders `02` first, over `0a` and
    `0a00` orders `0a` first (proper prefix), and over `0200` and `10` orders `0200`
    first, equal payloads have equal keys, and neither kind exposes an ordering operator.'
- id: FR-385-AC-7
  path: quire-exact/spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: context_only
  excerpt: Bytes `deadbeef` accounts four variable octets and the JsonObject `{"a":1}`
    accounts seven; existing scalar, composite, UUID and Timestamp admission, equality,
    key and accounting vectors retain their results.
- id: FR-388-AC-3
  path: quire-exact/spec/functional/FR-388-read-closed-limit-kind-spellings.md
  role: context_only
  excerpt: Against the authoritative QSpec `ScalarLimitsV1` full-vocabulary vector,
    `LimitKind::ALL` has exactly the same members in the same field order; each vector
    spelling is returned by `as_str()` and accepted by `from_code`, and at least one
    undeclared vector control is refused.
- id: FR-391-AC-4
  path: quire-exact/spec/functional/FR-391-fallible-effective-identity-copy.md
  role: context_only
  excerpt: 'A consumer can match the public copy error without inspecting text: a
    representable denied request maps to `EnvironmentFailure::Allocation(StorageRequest::new(requested,
    StorageUnit::Bytes))`, and overflow or layout incapacity maps to `EnvironmentFailure::Capacity`
    with the same byte unit and request amount. Inspection of the copy path finds
    no infallible allocation or depth-proportional recursion.'
```
