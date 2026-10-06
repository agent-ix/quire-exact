---
id: SR-1962
title: "EARS review of quire-exact PR #6 FR-359..FR-362"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact@1381f353d8cc4dee73035e316e3d690ecf86cc80; spec/functional/FR-359-cumulative-meter-boundary.md, spec/functional/FR-360-integer-minimum-magnitude.md, spec/functional/FR-361-admission-before-large-exact-work.md, spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-359
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-360
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-361
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-362
    type: reviews
---
# EARS review of quire-exact PR #6 FR-359..FR-362

## Summary

Ticket: IR-653. PR: quire-exact#6, head `1381f353d8cc4dee73035e316e3d690ecf86cc80`, base `efd4a22846ed69a5cf942797923fd6dd4f950acc`. Reviewer model `claude-opus-5-5`, run `f4edaeb0-7b19-4fd7-84bd-75f24ff35ec7`. Four new requirement statements. The engine check (`quire validate --summary`) reports them grammar-clean; its single finding, `quality:mixed-modal`, is in FR-357 outside this diff. Semantic review flags two medium defects and one low: FR-361 rests on the unmeasurable word "large", FR-362 points to amounts "specified" nowhere in this repo, and FR-360's trigger miscasts an input value as an event. FR-359 is clean.

## Verdict

**CONDITIONAL** — FR-359 conforms; FR-360..FR-362 carry one finding each.

## Examined scope

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| FR-359 | examined | spec/functional/FR-359-cumulative-meter-boundary.md:11 | When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete` record for the first unavailable counter without wrapping or changing any consumed counter or admission count. |
| FR-360 | examined | spec/functional/FR-360-integer-minimum-magnitude.md:11 | When `Integer::abs` receives the value `i64::MIN`, the `quire-exact` kernel SHALL return its exact positive magnitude as an arbitrary-precision `Integer`. |
| FR-361 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:11 | When an operand-derived or result-derived size charge is denied, the `quire-exact` kernel SHALL return `Incomplete` at that charge point before materializing the large exact value whose size the charge bounds. |
| FR-362 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:11 | When evaluating kernel-owned integer or rational arithmetic, numeric ordering, or a Boolean connective over already-decided operands, the `quire-exact` kernel SHALL return the exact outcome and charge the specified scalar atom points and amounts. |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-361's response rests on "the large exact value", which is not measurable. Behavior then permits "small bookkeeping allocations" without a bound, so whether a given allocation before denial violates the SHALL depends on the reader. | spec/functional/FR-361-admission-before-large-exact-work.md:11 | wrong-requirement |
| FND-002 | medium | FR-362 requires charging "the specified scalar atom points and amounts", but no quire-exact requirement specifies the amounts. The referent of "specified" resolves only to source doc comments, so the response cannot be verified from the spec. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:11 | missing-requirement |
| FND-003 | low | The trigger "When `Integer::abs` receives the value `i64::MIN`" frames an input value as an event, and `abs` receives an `Integer` (`&self`), not an i64. A ubiquitous form fits better: the operation SHALL return 2^63 for an Integer equal to i64::MIN. | spec/functional/FR-360-integer-minimum-magnitude.md:11 | wrong-requirement |

## Dispositions

Round 1, reviewed `e7cd04e0b79b096e1cffe3dc37775d96efb030e8` against prior `1381f353d8cc4dee73035e316e3d690ecf86cc80`. Model `claude-opus-5-5`, run `0beb8f83-17dc-4263-b553-44c85fabe9c1`, session `1c69c439-7235-4983-862a-16066636b5df`. Each outcome was checked against the actual fix diff and the public source at that SHA; author assertions were not relied on. No code exists for these planned requirements, and nothing was built or run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e7cd04e0b79b096e1cffe3dc37775d96efb030e8 |
| FND-002 | fixed | e7cd04e0b79b096e1cffe3dc37775d96efb030e8 |
| FND-003 | fixed | e7cd04e0b79b096e1cffe3dc37775d96efb030e8 |

### Round 1 after-excerpts

- FND-001: `FR-361` at spec/functional/FR-361-admission-before-large-exact-work.md:15. "Large" is replaced by an allocation as large as the bounded value, and Behavior lines 21-23 state numeric bounds.

```text
When a size charge for integer or rational arithmetic, integer division or modulus, or decimal scale expansion, arithmetic or result retention is denied, the `quire-exact` kernel SHALL return `Incomplete` at that point before requesting an allocation as large as the value being bounded.
```

- FND-002: `FR-362` at spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:13. "Specified" is replaced by "defined below", and the Behavior table defines the amounts.

```text
When evaluating kernel-owned integer or rational arithmetic, integer or rational ordering, or a Boolean connective over already-decided operands, the `quire-exact` kernel SHALL return the exact outcome and charge the point order and amounts defined below.
```

- FND-003: `FR-360` at spec/functional/FR-360-integer-minimum-magnitude.md:11. Ubiquitous form applied to an Integer equal to i64::MIN.

```text
The `quire-exact` kernel SHALL return the exact positive magnitude `2^63` when `Integer::abs` is called on an `Integer` equal to `i64::MIN`.
```
