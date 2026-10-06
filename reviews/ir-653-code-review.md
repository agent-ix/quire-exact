---
id: SR-1960
title: "Code review of quire-exact PR #6 kernel evidence specification"
type: SpecReview
analysis: code-review
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
# Code review of quire-exact PR #6 kernel evidence specification

## Summary

Ticket: IR-653. PR: quire-exact#6, head `1381f353d8cc4dee73035e316e3d690ecf86cc80`, base `efd4a22846ed69a5cf942797923fd6dd4f950acc`. Reviewer model `claude-opus-5-5`, run `f4edaeb0-7b19-4fd7-84bd-75f24ff35ec7`. Code review (with the rust-review lane folded in) of the four new FR files. The diff changes no Rust, so the Rust lane grounded each spec claim against the frozen source (`src/accounting.rs`, `src/integer.rs`, `src/numeric.rs`, `src/division.rs`, `src/decimal.rs`, `Cargo.toml`, `deny.toml`) and the public quire-contract-runtime tests that RT #95 removed. Source claims hold: `Integer` wraps `BigInt` and is unbounded, `Meter::charge` refuses cumulative overflow by `checked_add` and applies nothing before every counter passes, and arithmetic, division and decimal charges precede the values they size. Two defects: FR-361's allocator observation cannot be built the way RT built it under this package's `[lints.rust] unsafe_code = "forbid"`, and FR-362-AC-3 restates totals of externally owned P11/Q11 schedules.

## Verdict

**FAIL** — FND-001 is a duplication finding, which this skill records as `high`. The ACs do not copy RT's tests and the PR adds no dependency, compatibility layer or vendored file. Nothing was built or run here: tests, Kani and gates are out of scope for this spec-only review.

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
| FND-001 | high | FR-362-AC-3 restates the scalar-atom totals of the P11 (n = 2) and Q11 schedules (15/5 and 6/2 work/result units). Those schedules are defined outside quire-exact: the public RT test that RT #95 removed cites them as TC-191 P11 and TC-190 Q11. That puts a second statement of an external schedule in this spec, and it will drift when the owner changes the program. Use a kernel-owned call sequence instead, with totals derived from FR-362's own amounts. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:23 | wrong-requirement |
| FND-002 | medium | FR-361's ACs need allocator observation but do not say how to get it. Cargo.toml's package-wide `[lints.rust] unsafe_code = "forbid"` (lines 37-38) applies to test targets as well, so this crate cannot build the counting `unsafe impl GlobalAlloc` that RT's removed `tests/exact_allocation.rs` used (RT forbade unsafe only at its crate root). The code stage then has to choose between relaxing a lint and adding an allocator dev-dependency, and the spec records neither choice. | spec/functional/FR-361-admission-before-large-exact-work.md:21 | wrong-requirement |
