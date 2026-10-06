---
id: SR-1963
title: "Failure-domain review of quire-exact PR #6 FR-359..FR-362"
type: SpecReview
analysis: failure-domain
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
# Failure-domain review of quire-exact PR #6 FR-359..FR-362

## Summary

Ticket: IR-653. PR: quire-exact#6, head `1381f353d8cc4dee73035e316e3d690ecf86cc80`, base `efd4a22846ed69a5cf942797923fd6dd4f950acc`. Reviewer model `claude-opus-5-5`, run `f4edaeb0-7b19-4fd7-84bd-75f24ff35ec7`. Adverse cases and unstated failure modes, checked against the frozen meter, numeric and decimal source. FR-360 and FR-362-AC-4 are sound. Six gaps: refused cumulative charges that carry a size component; FR-359-AC-2's unstated limits; FR-359-AC-3 allowing one counter to go untested; FR-361-AC-3's unstated denial mechanism; decimal ordering missing from FR-362; and the overloaded word "cancellation" in FR-361-AC-1.

## Verdict

**CONDITIONAL** — two medium findings (FND-001, FND-003), one medium-severity coverage gap of medium confidence (FND-004), and three low.

## Examined scope

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| FR-359 | examined | spec/functional/FR-359-cumulative-meter-boundary.md:11 | When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete` record for the first unavailable counter without wrapping or changing any consumed counter or admission count. |
| FR-359-AC-1 | examined | spec/functional/FR-359-cumulative-meter-boundary.md:21 | With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one work unit reaches `u64::MAX` exactly. The next one-unit charge returns `Incomplete` at its named point with `limit_kind = WorkUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission count stay at their pre-refusal values. |
| FR-359-AC-2 | examined | spec/functional/FR-359-cumulative-meter-boundary.md:22 | After admitting a charge of `u64::MAX` result units, a charge requesting one result unit returns `Incomplete` with `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`. Its work unit is also unconsumed and its admission is unrecorded. |
| FR-359-AC-3 | examined | spec/functional/FR-359-cumulative-meter-boundary.md:23 | A cumulative work or result charge whose exact amount is `u64::MAX + 1` returns `Incomplete` with that exact `next_charge`, naming its counter and charge point. It neither wraps to a smaller amount nor changes any counter or admission count. |
| FR-360 | examined | spec/functional/FR-360-integer-minimum-magnitude.md:11 | When `Integer::abs` receives the value `i64::MIN`, the `quire-exact` kernel SHALL return its exact positive magnitude as an arbitrary-precision `Integer`. |
| FR-360-AC-1 | examined | spec/functional/FR-360-integer-minimum-magnitude.md:17 | `Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808 (`2^63`), is nonnegative, and differs from the original negative value. It neither overflows nor returns `i64::MIN`. |
| FR-361 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:11 | When an operand-derived or result-derived size charge is denied, the `quire-exact` kernel SHALL return `Incomplete` at that charge point before materializing the large exact value whose size the charge bounds. |
| FR-361-AC-1 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:21 | For a large integer multiplication and a large rational operation, deny the operand-derived `integer-arithmetic.arithmetic` or `rational-arithmetic.arithmetic` size charge at one below its exact amount. Each returns `Incomplete` with the correct point, counter and exact amount; allocator observation during the call shows no result-sized intermediate was allocated. Include a cancellation case whose mathematical result is small but whose operand-derived charge is large. |
| FR-361-AC-2 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:22 | Inject denial at `integer-division.arithmetic` and `integer-modulus.arithmetic` for large nonzero operands. Each returns `Incomplete` at the named point, admits no later charge, and allocator observation shows no quotient- or remainder-sized allocation before denial. |
| FR-361-AC-3 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:23 | For decimal operands that require a large scale expansion, a denied `decimal.scale-expansion` or `decimal.arithmetic` size charge returns `Incomplete` before allocating the corresponding large power of ten or intermediate coefficient. The incomplete record names the denied point, counter and exact planned amount. |
| FR-361-AC-4 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:24 | For a decimal value retained at a target scale requiring a large power of ten, set `decimal_digits` one below the exact result-retain amount while allowing preceding charges. Evaluation returns `Incomplete` at `decimal.result-retain` with the exact denied amount, and allocator observation shows no target-scale coefficient allocation before denial. |
| FR-362 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:11 | When evaluating kernel-owned integer or rational arithmetic, numeric ordering, or a Boolean connective over already-decided operands, the `quire-exact` kernel SHALL return the exact outcome and charge the specified scalar atom points and amounts. |
| FR-362-AC-1 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:21 | Over a fixed set containing `i128::MIN`, -1, 0, 1 and `i128::MAX`, plus reproducibly sampled signed pairs whose reference operation fits, integer add, subtract, multiply and negate agree with independently checked `i128` results. A product exceeding `i128::MAX` still completes to its exact arbitrary-precision `Integer` under sufficient limits. |
| FR-362-AC-2 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:22 | For fixed and reproducibly sampled rational pairs whose `i128` reference intermediates fit and whose denominators are nonzero, rational add, subtract, multiply, divide by nonzero and negate agree with an independently reduced-fraction oracle; division by zero returns `Undefined(DivisionByZero)`. Integer and rational ordering agree with independently checked comparisons. |
| FR-362-AC-3 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:23 | For integer arithmetic, rational arithmetic and numeric ordering, successful atom calls admit their family’s named operand, arithmetic, normalize where applicable, and result-retain points in order. Independent formulas over input magnitudes and unreduced rational intermediates agree with the observed high-water size counters, value occurrences, cumulative work units and result units. The P11 scalar loop atoms for `n = 2` consume 15 work units and 5 result units; the Q11 two-step integer fold over 1 and 2 consumes 6 work units and 2 result units. |
| FR-362-AC-4 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:24 | Inject a one-shot denial at each named point of representative integer arithmetic, rational arithmetic, ordering and already-decided Boolean retention calls. Each returns `Incomplete` at that point with no completed value, consumes no result unit for the denied charge, and admits no later point from that call. A successful already-decided Boolean connective charges `boolean.result-retain` once. |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | No FR-359 AC refuses a cumulative charge that also carries a semantic-size amount. The statement and Behavior promise that a refusal changes no counter and that high-water sizes are preserved. An implementation that raised sizes before the cumulative check (`src/accounting.rs` currently applies them only after both cumulative counters pass) would still pass all three ACs. | spec/functional/FR-359-cumulative-meter-boundary.md:22 | wrong-requirement |
| FND-002 | low | FR-359-AC-2 gives neither the `work_units` limit nor prior work consumption, and work is checked before results. If the fixture's work budget is tight, the expected ResultUnits record becomes a WorkUnits record. | spec/functional/FR-359-cumulative-meter-boundary.md:22 | wrong-requirement |
| FND-003 | medium | FR-361-AC-3 does not say whether the denial is injected or limit-driven. An injected denial reports `WorkUnits` with `next_charge` equal to the work amount (`src/accounting.rs` `check_injected`, lines 639-666), not "the exact planned amount". A limit-driven denial is ambiguous too: these charges carry two or three size counters (`integer_bits`, `decimal_digits`, and `scale_expansion` on scale-expansion) and the AC names none. "... or ..." also lets one of the two points go untested. | spec/functional/FR-361-admission-before-large-exact-work.md:23 | wrong-requirement |
| FND-004 | medium | FR-362 covers "numeric ordering", and `order_numbers` has a Decimals arm with scale-expansion charges (`src/numeric.rs`:122-161). FR-362-AC-2 checks ordering values only for integer and rational, and AC-3's "numeric ordering" does not settle whether decimal is in scope. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:22 | missing-requirement |
| FND-005 | low | "Include a cancellation case" means arithmetic cancellation, but the crate also has a `Cancel` handle (`Meter::with_cancel`) whose denial is also an `Incomplete`. The AC can be read as asking for a cancelled-meter case. | spec/functional/FR-361-admission-before-large-exact-work.md:21 | wrong-requirement |
| FND-006 | low | "A cumulative work or result charge" is met by testing only one of the two counters. Both have their own `to_u64` refusal path in `Meter::charge`, so both need the u64::MAX + 1 case. | spec/functional/FR-359-cumulative-meter-boundary.md:23 | wrong-requirement |
