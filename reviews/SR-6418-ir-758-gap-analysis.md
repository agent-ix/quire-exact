---
id: SR-6418
title: gap-analysis of IR-758 equality seam
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact@1647a4a7a893034831a7f0cb173a916a3d0c0ada; repository-wide spec/test matrix and reverse-gap scan; Ticket IR-758
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: references
---

## Summary

Ticket: IR-758. Planless repository gap analysis. The computed matrix contains 138 criteria across 25 requirements. The strict static matrix exits 1 with 18 untagged criteria and 0 tagged-only-by-ignored-test criteria. The untagged set is: FR-369-AC-3; FR-369-AC-4; FR-369-AC-7; FR-369-AC-9; FR-371-AC-1; FR-371-AC-2; FR-371-AC-3; FR-371-AC-4; FR-371-AC-5; FR-371-AC-6; FR-372-AC-5; FR-374-AC-1; FR-374-AC-2; FR-374-AC-3; FR-374-AC-4; FR-374-AC-6; FR-375-AC-4; FR-375-AC-6. Only FR-375-AC-4 and FR-375-AC-6 are directly in the equality seam contract; both are intentionally reserved for the downstream checked caller/Kani qualification and are outside this kernel API PR. The other 16 are pre-existing gaps in unrelated requirements. No spec files changed in this PR. Reverse-gap and stub review of the changed `src/equality.rs` and `src/lib.rs` found no newly unowned behavior or placeholder implementation. Optional semantic intent↔test review was skipped as the caller did not opt in.

## Verdict

FAIL for repository-wide gap analysis: the computed strict matrix reports 18 untagged criteria. This does not identify a new spec/code ownership gap introduced by PR #34, whose changes are owned by FR-375; the equality AC-4/6 integration gaps remain assigned downstream to IR-241. No plan was supplied; plan completion is not assessed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-369-AC-3 has no matrix binder | spec/functional/FR-369-typed-checked-invariant-causes.md:153 |
| FND-002 | medium | FR-369-AC-4 has no matrix binder | spec/functional/FR-369-typed-checked-invariant-causes.md:154 |
| FND-003 | medium | FR-369-AC-7 has no matrix binder | spec/functional/FR-369-typed-checked-invariant-causes.md:157 |
| FND-004 | medium | FR-369-AC-9 has no matrix binder | spec/functional/FR-369-typed-checked-invariant-causes.md:159 |
| FND-005 | medium | FR-371-AC-1 has no matrix binder | spec/functional/FR-371-bounded-integer-representation-refinement.md:79 |
| FND-006 | medium | FR-371-AC-2 has no matrix binder | spec/functional/FR-371-bounded-integer-representation-refinement.md:80 |
| FND-007 | medium | FR-371-AC-3 has no matrix binder | spec/functional/FR-371-bounded-integer-representation-refinement.md:81 |
| FND-008 | medium | FR-371-AC-4 has no matrix binder | spec/functional/FR-371-bounded-integer-representation-refinement.md:82 |
| FND-009 | medium | FR-371-AC-5 has no matrix binder | spec/functional/FR-371-bounded-integer-representation-refinement.md:83 |
| FND-010 | medium | FR-371-AC-6 has no matrix binder | spec/functional/FR-371-bounded-integer-representation-refinement.md:84 |
| FND-011 | medium | FR-372-AC-5 has no matrix binder | spec/functional/FR-372-fallible-bounded-decimal-membership.md:171 |
| FND-012 | medium | FR-374-AC-1 has no matrix binder | spec/functional/FR-374-nonzero-integer-divisor-admission.md:168 |
| FND-013 | medium | FR-374-AC-2 has no matrix binder | spec/functional/FR-374-nonzero-integer-divisor-admission.md:169 |
| FND-014 | medium | FR-374-AC-3 has no matrix binder | spec/functional/FR-374-nonzero-integer-divisor-admission.md:170 |
| FND-015 | medium | FR-374-AC-4 has no matrix binder | spec/functional/FR-374-nonzero-integer-divisor-admission.md:171 |
| FND-016 | medium | FR-374-AC-6 has no matrix binder | spec/functional/FR-374-nonzero-integer-divisor-admission.md:173 |
| FND-017 | medium | FR-375-AC-4 has no matrix binder | spec/functional/FR-375-composable-equality-effect-seam.md:34 |
| FND-018 | medium | FR-375-AC-6 has no matrix binder | spec/functional/FR-375-composable-equality-effect-seam.md:36 |
| FND-019 | medium | FR-375-AC-1 tag binds a flat kernel test, not the specified checked E21 path | src/equality.rs:303-365 |
| FND-020 | medium | FR-375-AC-2 tag binds a flat kernel test, not the specified checked E21 path | src/equality.rs:303-365 |

## Coverage

`quoin matrix --repo . --json` completed and recorded the static/evidence matrix. `quire matrix --scope . --strict` exited 1: 18 untagged, 0 ignored-only. The full set of 138 requirement criteria was inspected for computed binders. Reverse-gap/stub check was scoped to the changed Rust paths because this is a frozen PR-diff review; both new exports are owned by FR-375. No AssuranceProfile applies. Plan completion: not assessed. Optional semantic review was skipped because it requires explicit opt-in. No cargo/Kani gate was run by reviewer. `git diff --check` passed.

Full criterion scope and static test-tag bindings:

```yaml
clean: false
scope:
- id: FR-088-AC-11
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: For an enum shape whose canonical member list is `b`, `a`, `c`, the ranks of the variants are `b` 0,
    `a` 1, `c` 2, and a set of all three values visits `b`, `a`, `c`. For an unordered enum whose canonical list
    is `a`, `b`, `c`, the ranks are `a` 0, `b` 1, `c` 2, and the set visits `a`, `b`, `c`. `compare_keys` orders
    two members of one shape by rank, whatever the order of their `VariantId` digests. A value that pairs a variant's
    `VariantId` with a rank that differs from the shape's rank for it is refused at admission. Two shapes that differ
    in the identity of a variant but not in rank and orde
- id: FR-088-AC-12
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: A declared unit `UnitId` and a compound unit `UnitId` are unequal when their domains or bytes differ.
    A declared unit's `UnitId` carries its node key under the `quire.checked-semantic-node/v1` label, and a compound
    unit's carries its digest under the `quire.value.compound-unit/v1` label. Two `UnitId`s of different domains
    never compare equal, whatever their bytes.
- id: FR-089-AC-2
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: The kernel `Value::Population` variant's payload type is `PopulationId`; no kernel source file imports
    a population binding or any other caller's model type to define, construct or match this variant, and the crate's
    `[dependencies]` name no crate that owns one.
- id: FR-089-AC-6
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: 'Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`, kernel `ValueType::admits`
    returns false; given two `Value::Population` operands, kernel `equality::plan_pairs` returns `Err(Refusal::CheckedInvariant
    { cause: PopulationPair })` and kernel `key::compare_keys` returns `None`.'
- id: FR-090-AC-7
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Refusal` has no variant whose payload is a wrong-snapshot cause.'
- id: FR-090-AC-11
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Undefined` has no `PreconditionFalse` variant.'
- id: FR-090-AC-12
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Undefined` has no `AbsentKey` variant.'
- id: FR-090-AC-13
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Refusal::CheckedInvariant` carries only FR-369''s closed typed cause; it has no arbitrary caller message,
    QSL model/checker type, catalog code or generic caller-cause escape hatch.'
- id: FR-096-AC-8
  path: spec/functional/FR-096-kernel-refusal-code-and-cause.md
  role: examined
  excerpt: 'Each catalogued kernel refusal below returns the code and cause spelling named for it from `Refusal::code()`
    and `Refusal::cause()`, and every FR-369 `CheckedInvariant { cause }` returns neither. `InexactDecimal`: `inexact_decimal`,
    `nonzero-discarded-digit`. `DecimalOutOfDomain`, `ModuloOutOfDomain`, `TextLengthOutOfDomain`, `IntegerOutOfDomain`,
    `RationalOutOfDomain`, `IeeeRationalOutOfDomain`: `<name in snake case>`, `outside-domain`. `DivisionOutOfDomain`:
    `division_out_of_domain` with `quotient-outside-domain` for the quotient member or `remainder-outside-domain`
    for the remainder member.'
- id: FR-097-AC-7
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: An unbounded collection type admits a collection value of any size with no cardinality refusal; it still
    charges `collection.bound`, and stops with `Incomplete` at that charge point only when the caller's meter runs
    out. `K<T>` and `K<T>[0, u64::MAX]` are different types.
- id: FR-097-AC-8
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: A caller outside the crate can call `CardinalityBound::violation(count)` and receive the existing `Option<BoundViolation>`
    type. For `[2, 4]`, counts 1, 2, 3, 4 and 5 return below-minimum, none, none, none and above-maximum respectively;
    singleton `[2, 2]` admits only 2. `[0, u64::MAX]` reports no violation at either extreme, `[0, 0]` rejects 1
    above, and `[u64::MAX, u64::MAX]` rejects `u64::MAX - 1` below. The query is pure and unmetered; for `[2, 4]`,
    collection admission agrees with its decisions at counts 1 through 5.
- id: FR-262-AC-2
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s around `Boolean` clones,
    compares equal to its clone, hashes equal to its clone, formats for debug and drops. A `Value` of the same depth
    compares equal to its clone, and it and its clone drop.
- id: FR-262-AC-3
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: A `Value` nested N levels deep, alternating `Option` and `Sequence` levels around an `Integer` with each
    level declared with the type of its depth, holds at least N - 10 and at most 2N distinct `Option` and `Collection`
    type nodes, and the same value 2N levels deep holds at most twice the nodes of the N-level value plus 2. On
    a thread with a 512 KiB stack, such a value 100,000 levels deep and its 100,000-deep type are built, the type
    admits the value, and the type clones, compares equal to its clone, hashes equal to its clone, formats for debug
    with balanced brackets and drops, as does the val
- id: FR-357-AC-1
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a zero divisor under any profile and either member, `divide` returns `Undefined(DivisionByZero)`.
- id: FR-357-AC-2
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `10 div y` over `1..=10` at `y = 5`, `divide` of the quotient returns 2; given `x rem -1` over
    `-10..=5` at `x = -10`, `divide` of the remainder returns 0.
- id: FR-357-AC-3
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a quotient outside the domain, `divide` refuses with cause `quotient-outside-domain`; given a remainder
    outside the domain, it refuses with `remainder-outside-domain`.
- id: FR-357-AC-4
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given -7 and 2, truncating returns (-3, -1), floor (-4, 1) and Euclidean (-4, 1); given -7 and -2, truncating
    returns (3, -1), floor (3, -1) and Euclidean (4, 1).
- id: FR-357-AC-5
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `i64::MIN` and -1 under any profile, `divide` of the quotient returns 2^63 in a domain that holds
    it. In a signed 64-bit domain the quotient refuses with `quotient-outside-domain` and the remainder completes
    with 0.
- id: FR-357-AC-6
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a successful `div` or `rem`, the meter admits `integer-division.operands`, `integer-division.arithmetic`,
    `integer-division.domain` (one value occurrence) and `integer-division.result-retain` (one result unit), in
    that order. A refusal admits no `result-retain`, a zero divisor admits only operands, and a result limit of
    0 reports `integer-division.result-retain` as incomplete.
- id: FR-357-AC-7
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: For dividend/divisor pairs `(7, 3)`, `(7, -3)`, `(-7, 3)` and `(-7, -3)`, public `modulo` in the mathematical
    domain completes to 1, 1, 2 and 2 respectively. Running a `divide` under each of Truncating, Floor and Euclidean
    before each `modulo` call does not change those results; `modulo` has no profile argument.
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit P charge under an ordinary
    limit, then admit P once and an unrelated point once. The next P charge returns `Incomplete` at P with the work
    counter as its limit kind, proving the ordinary refusal did not advance the named occurrence; all consumed counters,
    admission count, and test-support log equal their values immediately before each refusal. A subsequent P charge
    succeeds and updates ordinary accounting and the log exactly once.
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan` returns `Incomplete`
    at `equality.plan` without changing any consumed counter, admission count, or test-support log; a subsequent
    valid `charge_plan` succeeds and records its ordinary plan charge.
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction with zero is rejected
    by that type, while a nonzero occurrence constructs a denial.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor returns all 4096 records with
    their points in order and `charge_log_truncated()` is false. On the 4097th admission the slice still contains
    exactly that prefix and the flag becomes true.
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()` and consumed work units reflect
    all 4097 admissions, including the one omitted from the diagnostic log.
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge log and its `Counters` accounting
    state remains fixed-size and heap-free.
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one result unit. The log retains
    the first 4096 points and reports truncation, while `admission_count()`, consumed work and consumed results
    each equal 4097.
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`, admit 4097 one-work-unit charges
    and request one more work unit. The charge returns `Incomplete` at its requested point with `WorkUnits`, `limit
    = consumed = 4097`, `next_charge = 1`; every counter, admission count, prefix log and truncation flag are unchanged.
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`, admit 4097 charges each consuming
    one result unit, then request one more result unit. The charge returns `Incomplete` with `ResultUnits`, `limit
    = consumed = 4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation flag remain
    unchanged, including the work counter.
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial for one named point. A charge
    at that point returns `Incomplete` with `WorkUnits` and leaves all counters, admission count, prefix log and
    truncation flag unchanged. Retrying that charge under sufficient ordinary limits succeeds once, increments work
    and admission count once, and does not extend the 4096-entry log.
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'After admitting two work units, inject occurrence 1 at point P and attempt a three-work-unit `charge`
    at P under otherwise sufficient limits whose configured work limits are 10 and 100 on separate meters. Both
    refusals report exactly `limit_kind = WorkUnits`, `charge_point = P`, `limit = consumed = 2`, and `next_charge
    = 3`, independently of either configured limit. Repeat with an injected `equality.plan` denial, `pairs = 2`,
    the same pre-consumed work and limits, and sufficient value-occurrence and result limits: both `charge_plan`
    refusals report `WorkUnits`, `charge_point = equality.plan`, '
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'With an occurrence-1 injected denial selected, make the same `charge` independently fail an ordinary
    semantic-size counter and make a `charge_plan` with `pairs = 2` independently fail its work reservation under
    a configured work limit of 3 after two admitted work units, with all other limits sufficient. In each case,
    the injected `WorkUnits` record is returned before the ordinary refusal: `limit = consumed` equals the pre-charge
    work, `next_charge` is the charge''s work amount or the plan''s reservation respectively, and every consumed
    counter and admission count remain unchanged. The criterion '
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request of 9 followed by 3 and then
    a charge with no size request. The records expose respectively 9, 3 and no IntegerBits request while the consumed
    IntegerBits maximum remains 9. A separate charge with IntegerBits requests 9, 3 and zero exposes present 9;
    a charge with any positive number of IntegerBits zero requests exposes present zero. Both have the same fixed
    payload size as a single-request record, with no heap-owned request collection and no retained request list,
    order or multiplicity. A successful `charge_plan` with pai
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently chosen semantic-size requests,
    including repeated same-kind requests and distinct amounts at admissions 4096 and 4097. The single record log
    contains exactly the first 4096 fixed-size records with their points and per-admission semantic-size maxima
    in admission order; truncation is false at admission 4096 and true at 4097. Work consumption and admission count
    reach 4097, including the omitted record; existing size maxima still include the 4097th charge.
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one work unit
    reaches `u64::MAX` exactly. The next one-unit charge returns `Incomplete` at its named point with `limit_kind
    = WorkUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission count
    stay at their pre-refusal values.
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting `u64::MAX` result units. Its
    work consumption is 1. A second charge requesting one result unit returns `Incomplete` at its named point with
    `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission
    count remains 1.
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount is `u64::MAX + 1` returns
    `Incomplete` with `limit_kind = WorkUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes.
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit and `u64::MAX + 1` result units
    returns `Incomplete` with `limit_kind = ResultUnits` and `next_charge = u64::MAX + 1`; no counter or admission
    count changes.
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least 16 bits,
    `u64::MAX` work units and one result unit. Then attempt a charge carrying `integer_bits = 16` and one result
    unit. The result refusal leaves `integer_bits = 8`, work units, result units and admission count at their pre-refusal
    values; under `test-support`, the admitted-charge log is unchanged too.
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters the first unavailable counter
    of a charge, including several simultaneously short counters and semantic sizes attached in either order. The
    reported `limit_kind` follows `ScalarLimits` field order, with the exact limit, previously consumed amount,
    requested amount and point. A refusal after an earlier size check succeeds changes no counter, admission count
    or test-support log. A separate meter with a pending later named denial proves that this ordinary refusal does
    not advance its occurrence.
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers without panic for every member
    of `LimitKind::ALL`: untouched counters are zero, semantic sizes are admitted high-water maxima and work/results
    are admitted cumulative totals.'
- id: FR-360-AC-1
  path: spec/functional/FR-360-integer-minimum-magnitude.md
  role: examined
  excerpt: '`Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808 (`2^63`), is nonnegative,
    and differs from the original negative value. It neither overflows nor returns `i64::MIN`.'
- id: FR-361-AC-1
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For an integer multiplication with a large operand, set `integer_bits` to one below the exact `integer-arithmetic.arithmetic`
    amount while permitting the operand charge. The outcome is `Incomplete` at `integer-arithmetic.arithmetic` with
    `limit_kind = IntegerBits` and that exact `next_charge`; the observed peak request meets the arithmetic allocation
    bound in Behavior. Repeat with operands whose subtraction mathematically cancels to 1 but whose operand-derived
    arithmetic amount exceeds the limit.
- id: FR-361-AC-2
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a rational operation with a large numerator or denominator, set `integer_bits` to one below the exact
    `rational-arithmetic.arithmetic` amount while permitting the operand charge. The outcome is `Incomplete` at
    `rational-arithmetic.arithmetic` with `limit_kind = IntegerBits` and that exact `next_charge`; the observed
    peak request meets the arithmetic allocation bound in Behavior. Include a multiplication whose unreduced large
    factors cancel mathematically to 1.
- id: FR-361-AC-3
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Inject a one-shot denial at each of `integer-division.arithmetic` and `integer-modulus.arithmetic` for
    large nonzero operands. Each returns `Incomplete` at the selected point with `limit_kind = WorkUnits`, admits
    no later point from that call, and meets the division allocation bound in Behavior.
- id: FR-361-AC-4
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For valid decimal operands requiring a `2^20`-place scale expansion, set `scale_expansion` to one below
    the planned shift and leave other limits sufficient. The outcome is `Incomplete` at `decimal.scale-expansion`
    with `limit_kind = ScaleExpansion` and `next_charge` equal to the exact shift; the observed peak request is
    below 4,096 bytes.
- id: FR-361-AC-5
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For decimal addition of small coefficients at scales 0 and `2^20`, set `integer_bits` to the admitted
    scale-expansion bit amount, one below the planned arithmetic bit amount, with other limits sufficient. The outcome
    is `Incomplete` at `decimal.arithmetic` with `limit_kind = IntegerBits` and `next_charge` equal to that planned
    bit amount; the observed peak request is below 4,096 bytes.
- id: FR-361-AC-6
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a decimal value retained at a target scale requiring a `2^20`-place power of ten, set `decimal_digits`
    one below the exact result-retain amount while permitting preceding charges. Evaluation returns `Incomplete`
    at `decimal.result-retain` with `limit_kind = DecimalDigits` and that exact `next_charge`; the observed peak
    request is below 4,096 bytes.
- id: FR-361-AC-7
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: On a fresh meter, floor division of a positive at-least-65,536-bit dividend by 3, exposing the quotient
    in the mathematical domain, completes to the independently calculated floor quotient with `integer_bits` limit
    equal to the dividend magnitude bit length. Final consumed `integer_bits` equals that exact bit length.
- id: FR-361-AC-8
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For that floor quotient fixture, lower the `integer_bits` limit by one. The call returns `Incomplete`
    at `integer-division.operands` with `limit_kind = IntegerBits`, `consumed = 0` and `next_charge` equal to the
    dividend bit length; it admits no point and the measured peak allocation request stays below one eighth of the
    largest operand byte length.
- id: FR-361-AC-9
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Multiply retained decimals `(1, 0)` and `(1, 0)` into an Exact target with coefficient bounds 0 through
    1 and scales 0 through `2^20`. At `decimal_digits = 2^20`, the `decimal.result-retain` charge returns `Incomplete`
    with `limit_kind = DecimalDigits`, `limit = 2^20`, `consumed = 2`, `next_charge = 2^20 + 1`; only the three
    preceding decimal points are admitted, no result unit is consumed, and the peak request is below 4096 bytes.
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`; for
    negate, use each member as a unary operand. Compare integer add, subtract, multiply and negate only when the
    corresponding checked `i128` operation returns a value. Repeat over reproducibly sampled signed inputs. Every
    compared kernel result equals the independent checked result.
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently authored
    decimal value `340282366920938463463374607431768211454`; negating `Integer::from(i128::MIN)` completes to `170141183460469231731687303715884105728`.
    Neither result is narrowed to `i128`.
- id: FR-362-AC-3
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For fixed and reproducibly sampled rational pairs whose checked `i128` reference intermediates fit and
    whose denominators are nonzero, rational add, subtract, multiply, divide by nonzero and negate equal an independently
    reduced-fraction oracle. A zero rational divisor returns `Undefined(DivisionByZero)` after only the operand
    charge.
- id: FR-362-AC-4
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Integer and rational comparisons over fixed and reproducibly sampled pairs equal independently checked
    numeric comparisons for equality and order. Decimal comparison is specified by [FR-363](./FR-363-metered-decimal-ordering.md).
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each rational arithmetic operation,
    integer ordering and rational ordering, the admitted point sequence equals its family row in Behavior. A successful
    already-decided Boolean retention admits only `boolean.result-retain`.
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate the maximum size amount across
    their points from the formulas in Behavior, using the unreduced numerator and denominator for rational normalization.
    The observed final `integer_bits` and `value_occurrences` equal those maxima; work consumption equals the number
    of admitted points and result consumption is one.
- id: FR-362-AC-7
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter, directly add integers 5 and 7, compare rationals 1/2 and 2/3 for less-than, then retain
    an already-decided Boolean `true`. The admitted atom sequence consumes 7 work units and 3 result units, derived
    from the three family rows in Behavior.
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer arithmetic, rational arithmetic,
    integer ordering, rational ordering and already-decided Boolean retention calls. Each returns `Incomplete` naming
    the denied point, with no completed value, no result unit for the denied charge and no later point admitted
    by that call.
- id: FR-362-AC-9
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For every already-decided Boolean connective, the completed value matches its truth table and the kernel
    admits `boolean.result-retain` once, consuming one work and one result unit.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and negate,
    and integer/rational ordering, independently calculate the required bit high-water amount from Behavior. On
    fresh meters with other limits sufficient, that exact bit limit completes with the expected value, while a limit
    one below stops at the first point whose amount exceeds it, with exact prior consumption, request and admitted
    prefix and no result unit. Include cancellation `1000 - 999`, `255 * 255`, rational `(2/3) * (3/2)`, `(3/4)
    / (5/7)`, `(5/7) - (4/7)`, negate `-5/8`, and `7/3 < 5/8`. '
- id: FR-362-AC-11
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`, typed domain `Refused`,
    and `Incomplete` with no partial value; `completed()` returns a value only for `Completed`, including false.
    On fresh meters, rational division by zero retains only operands; integer `2 + 2` outside the bound `[0, 3]`
    retains operands and arithmetic but no result unit; a rational result outside its domain retains operands, arithmetic
    and normalize but no result unit. A denied first operand charge admits nothing. Each stop preserves exactly
    the admitted prefix, work totals and size maxima, w
- id: FR-362-AC-12
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '`Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator 1 and denominator 2, proving
    reduction to lowest terms and cancellation of two negative signs. `Rational::new(1, -2)` exposes -1/2. For either
    denominator 5 or -5, a zero numerator exposes exactly 0/1.'
- id: FR-362-AC-13
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)` is
    false. Dividing `1/1` by `3/1` under that domain returns `Refused(RationalOutOfDomain { target })` after normalization
    with no result unit; the identical call without a domain completes to `1/3`.
- id: FR-362-AC-14
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Call the five scalar atoms `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0` directly on one meter. They complete
    to true, 1, true, 0, false in order, consuming 15 work and 5 result units. With work limit 14, the final ordering
    stops at `ordering.result-retain` with `limit = consumed = 14`, `next_charge = 1` and four result units.
- id: FR-362-AC-15
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Directly divide rational `3/1` by `2/1` on a fresh meter. The result is `3/2`, with high-water `integer_bits
    = 3`, value occurrences 2, work 4 and results 1; work limit 3 stops at `rational-arithmetic.result-retain` after
    the first three points and consumes no result unit.
- id: FR-362-AC-16
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter directly add integers `0 + 1` and then `1 + 2`. The results are 1 and 3; the integer point
    sequence occurs twice, high-water `integer_bits = 3`, occurrences 2, work 6 and results 2. Caller fold evaluation
    is outside this direct-atom criterion.
- id: FR-362-AC-17
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain` after operands and arithmetic,
    with zero result units; work limit 3 completes to 12 with three work and one result unit. For rational `(2/3)
    * (3/2)`, work limit 2 stops at `rational-arithmetic.normalize`, work limit 3 stops at `rational-arithmetic.result-retain`,
    and work limit 4 completes to `1/1`; each denied call preserves exactly its admitted prefix and consumes zero
    result units.
- id: FR-362-AC-18
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For `k` in 63, 64, 65, 127, 128, 200 and 511, compare exact results and charged bit high-water sizes
    around `2^k`: `(2^k - 1)(2^k + 1)` charges `2k + 1` bits for a `2k`-bit result, `(2^k)^2` charges `2k + 2` for
    a `2k + 1`-bit result, and adding `(2^k - 1) + 1` charges `k + 1` for a `k + 1`-bit result. Subtracting `(2^k
    + 1) - 2^k` or `2^k - 2^k` still charges `k + 2` bits despite a one-bit result.'
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add `8 + 3` and
    `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`, without a result bound and with all non-bit limits sufficient.
    The independently calculated `integer-arithmetic.arithmetic` bit amounts are respectively 5, 5, 5 and 6: add/subtract
    use `max(B(a), B(c)) + 1`, including the larger right operand and a result that cancels to one bit; multiply
    uses `B(a) + B(c)`. At each exact `integer_bits` limit, the call completes to respectively 11, 11, 1 and 24,
    admits operands, arithmetic and result-retain in '
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On fresh sufficient public meters under `test-support`, evaluate the six rational calls listed below
    with no result domain. Each completes to the listed canonical value and admits operands, arithmetic, normalize
    and result-retain in order, consuming four work units and one result unit. The actual normalize record has only
    IntegerBits present, with the listed amount independently derived from the unreduced parts and `B(0) = 1`. The
    recorded arithmetic amount equals its separate listed bound. The normalize observation is not replaced by that
    bound, the final bit maximum or the reduced value's pa
- id: FR-363-AC-1
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For decimal pairs with equal values but different retained scales, and for positive, negative and zero
    values at differing scales, `order_numbers` agrees with an independent exact rational comparison for each ordering
    operator. The retained representations remain unchanged.
- id: FR-363-AC-2
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For a successful decimal comparison, the meter admits `ordering.operands`, `ordering.arithmetic`, `ordering.result-retain`
    in that order. On a fresh meter, its size high-water counters equal the maxima in Behavior, with three work
    units and one result unit.
- id: FR-363-AC-3
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: Comparing retained `(0, 5) >= (0, 0)` completes `true` and consumes `integer_bits = 18`, `decimal_digits
    = 6`, `scale_expansion = 5`, `value_occurrences = 2`, `work_units = 3`, `result_units = 1`. Comparing retained
    `(100, 2) <= (2, 0)` completes `true` and consumes `integer_bits = 9`, `decimal_digits = 3`, `scale_expansion
    = 2`, `value_occurrences = 2`, `work_units = 3`, `result_units = 1`.
- id: FR-363-AC-4
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: With all limits at `u64::MAX` except `decimal_digits = 64`, testing retained `(1, 0) < (1, u32::MAX)`
    returns `Incomplete` at `ordering.arithmetic` with `limit_kind = DecimalDigits`, `limit = 64`, `consumed = 1`,
    and `next_charge = 4,294,967,296`. Only `ordering.operands` is admitted; no result unit is consumed and the
    scale-alignment power of ten is not materialized.
- id: FR-363-AC-5
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: 'On separate fresh meters with generous limits, inject a one-shot denial at occurrence 1 of each decimal-ordering
    point: `ordering.operands`, `ordering.arithmetic`, and `ordering.result-retain`. For each point, ordering retained
    `(0, 5) >= (0, 0)` returns `Incomplete` with `limit_kind = WorkUnits`, `charge_point` equal to the selected
    point, `limit = consumed` equal to the number of preceding admitted points (0, 1, or 2), and `next_charge =
    1`. No Boolean result or result unit is exposed; only the preceding point prefix is admitted and logged, with
    no later point admitted.'
- id: FR-363-AC-6
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For retained decimals `(110, 2)` and `(11, 1)`, public unmetered `Decimal::compare` returns `Equal`,
    `numerically_equal` returns true and both normalized representations are `(11, 1)`; their retained representations
    remain `(110, 2)` and `(11, 1)`.
- id: FR-363-AC-7
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` both complete false. The mixed
    pair consumes integer bits, decimal digits and scale expansion 8/3/1; the narrow pair consumes 4/2/0. Both consume
    value occurrences/work/results 2/3/1 with all other counters zero, despite equal normalized values.
- id: FR-364-AC-1
  path: spec/functional/FR-364-ieee-flag-iteration.md
  role: examined
  excerpt: For every subset of the five public flags, collect it into `IeeeFlags` in forward and reverse vocabulary
    order. Both `iter()` results equal that subset filtered from `IeeeFlag::ALL`, without duplicates. The empty
    set iterates empty and the full set iterates all five flags in the stated order.
- id: FR-365-AC-1
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: Round retained `(5, 1)` and `(-5, 1)` to a scale-zero target with coefficient bounds -10 through 10 under
    each of the six modes. Values or `Refused(InexactDecimal { target })` match the tie table in Behavior; each
    completed representation has scale zero, and refusals consume no result unit.
- id: FR-365-AC-2
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: '`RoundingMode::default()` and the mode read back from a target constructed with it are `Exact`. Rounding
    retained `(1, 1)` to that scale-zero target returns `Refused(InexactDecimal { target })`, rather than silently
    rounding 0.1 to zero, with no retained result.'
- id: FR-366-AC-1
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For same-width Add with a positive quiet NaN of payload 5 and a negative signaling NaN of payload 9 in
    either operand order, the result bits are the first NaN quieted and Invalid is present. Two quiet NaNs select
    the first without Invalid. For FusedMultiplyAdd with a quiet first NaN, finite second operand and signaling
    third NaN, the first NaN remains selected and Invalid is present. Exercise both binary widths.
- id: FR-366-AC-2
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: 'Converting a binary64 NaN with payload `2^22` or `2^30` to binary32 returns `Refused(IeeeNanPayloadNotRepresentable
    { source: Binary64, target: Binary32 })` without truncating the payload or retaining a result unit. At payload
    `2^22 - 1`, positive and negative binary64 quiet NaNs both complete as binary32 quiet NaNs with their respective
    signs and all payload bits preserved, and one result unit is consumed.'
- id: FR-366-AC-3
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: Converting either signed binary32 zero to the rational domain with numerator interval `[-1, 1]` and denominator
    interval `[1, 1]` completes to canonical `0/1`. Positive zero has no loss; negative zero has exactly `IeeeExactLoss::NegativeZeroSign`.
- id: FR-366-AC-4
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For each width, distinct bit patterns representing the ten classes in Behavior have strictly increasing
    `total_order_key()` values in that stated order, including quiet/signaling NaNs of both signs and both zeros.
    Positive NaN payloads of the same class ascend by payload and negative ones descend. The public TotalOrder comparison
    agrees with the key order for these pairs; NumericEqual considers the two zeros equal despite their distinct
    keys.
- id: FR-367-AC-1
  path: spec/functional/FR-367-text-profile-mismatch-before-charging.md
  role: examined
  excerpt: 'Admit the same UTF-8 text "a" once with Nfc and once with BinaryUtf8 using separate sufficient meters.
    On a fresh comparison meter whose ten limits are zero, public `compare_text(Equal, left, right)` returns precisely
    `Err(IllTyped { cause: DistinctTextProfiles })`. Every consumed counter and admission count remains zero; under
    `test-support` the charge log is empty and truncation is false.'
- id: FR-368-AC-1
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`, has a unique documented
    spelling from `as_str()` and round-trips through `from_code()`; an unknown spelling returns `None`. Exhaustive
    variant matching makes an added but unaccounted variant fail the check.
- id: FR-368-AC-2
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 'Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`, has a unique documented `as_str()`
    spelling, and the sequence is exactly the ten `ScalarLimits` fields: integer bits, decimal digits, scale expansion,
    text input bytes, text scalars, normalized scalars, unit edges, value occurrences, work units and result units.
    Exhaustive variant matching makes an added but unaccounted variant fail the check.'
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant
    set and four typed `IllTypedCause` payloads equal the thirty table rows; `CheckedInvariantCause` implements
    the stated traits, and the old unit constructor does not compile.
- id: FR-369-AC-2
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Every kernel producer in collection, value, division and equality returns exactly its table cause; two
    population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`,
    and unlike collection kinds return `CollectionKindMismatch`.
- id: FR-369-AC-3
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Every RT checked-package/function invariant producer returns its table cause: unknown name is `UnknownCheckedFunction`,
    a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`. Runtime-managed
    calls use explicit frames and `work_units` fuel under RT FR-273; the final carrier has no `CallDepthExceeded`
    variant, and fuel exhaustion is `Incomplete` with `limit_kind: work_units`, never `CheckedInvariant`.'
- id: FR-369-AC-4
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Every RT collection, composite, equality and quantity producer returns its table cause, including all
    six `operand_value` failures and the original kernel-owned `IllTypedCause` when a scheduled comparison or checked
    equality quantity conversion refuses; no site flattens a comparison or conversion error to a message.
- id: FR-369-AC-5
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and
    `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are
    unchanged.
- id: FR-369-AC-6
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Every received typed cause follows QSL''s existing internal-fault path: no refusal record or `Evaluation`,
    `runtime_invariant`/`S6a`/`checked-program-invariant` at S6a, `CallFailure::Fault` at the public call, and `Failed`
    with unavailable basis for an internal replay fault. It never becomes an ordinary replay refusal, `Inconclusive`,
    `Incomplete(Cancelled)`, a proved result or a certification rejection; an unknown received cause retains fault
    provenance.'
- id: FR-369-AC-7
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned
    causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers
    preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former
    unit constructor nor an invented default.
- id: FR-369-AC-8
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT and QSV
    own no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT,
    QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables.
- id: FR-369-AC-9
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: QSV's deferred field or tuple result uses `DeferredResultNotAdmitted`; missing enum variant and unresolved
    unit use their distinct rows; schedule mismatch and comparator refusal retain their existing rows; each equality
    operand failure uses its exact table row; and scale-zero integer placement uses `ExpectedIntegerPlacement`.
    The comparator and quantity conversion retain their original `IllTypedCause` payload, while prior non-result
    and charge behavior is unchanged.
- id: FR-370-AC-1
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: A UUID value is admitted only by `ValueType::Uuid` and a Timestamp value only by `ValueType::Timestamp`;
    crossed native kinds and Integer, Text and Reference values are rejected by both native types, without a conversion
    or alias.
- id: FR-370-AC-2
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'Each same-kind native pair follows the existing one-terminal-pair equality schedule: an equal pair yields
    `true`, an unequal pair yields `false`, and the planned pair count and charges equal the corresponding Boolean-leaf
    schedule.'
- id: FR-370-AC-3
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'A direct equality plan pairing UUID or Timestamp with another value kind returns `Refusal::CheckedInvariant
    { cause: ValueKindMismatch }`, while a direct mixed-kind `compare_keys` call returns `None`; neither returns
    a Boolean or a false equal key.'
- id: FR-370-AC-4
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: '`compare_keys` gives each native type a total key consistent with its equality relation and ordered
    by canonical payload ASCII bytes; Timestamp `"10"` precedes `"2"` and `"-1"` precedes `"-2"` in the key, independent
    of value construction or insertion order and without exposing a native ordering operator.'
- id: FR-370-AC-5
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Existing scalar and composite admission, equality, key comparison and accounting vectors retain their
    prior results.
- id: FR-370-AC-6
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: UUID values preserve exactly 16 network-order octets, including Nil, Max and non-RFC-version bit patterns;
    only the lowercase ASCII `8-4-4-4-12` caller payload admits, and uppercase, braces, URN form, altered grouping/separators,
    whitespace, length or nonhex characters refuse without repair or value.
- id: FR-370-AC-7
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Timestamp values preserve signed `i128` POSIX-epoch nanoseconds, including inclusive bounds `-170141183460469231731687303715884105728`
    and `170141183460469231731687303715884105727` and `-1`, `0`, `1` as one nanosecond before, at and after the
    epoch; only canonical signed base-10 caller payload admits, while either adjacent exterior count, `-0`, plus
    signs, leading zero, fraction, exponent, non-ASCII digits, calendar/offset text and whitespace refuse without
    repair or value.
- id: FR-370-AC-8
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Kernel constructors accept canonical UUID or Timestamp text supplied after the caller has validated its
    tagged wire envelope. Each malformed text returns `ConstructionRefusal` at `Component::Value` with its tabled
    cause, no value and no internal checked-invariant fault; canonical out-of-range decimal returns `TimestampOutOfDomain`
    after syntax checking. The kernel has no second parser-error type or UUID out-of-domain case.
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact
    value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64
    MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations
    with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals.
    Reject malformed/noncanonical spellings as be
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal,
    identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt
    access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all
    accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted
    borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native,
    native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit
    and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete
    fields and admitted prefixes with the independent normative formulas. No representation transition manufactures
    a FunctionCall or other charge.
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate,
    magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator
    observer around standalone Integer calls confirms no allocation on that native path, including clone after cache
    access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently
    prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty.
    Observation of public metered calls separately iden
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'Public signatures and all existing Integer methods remain available, except that the divisor parameters
    of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger
    signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain
    &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other
    public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync
    and current panic-unwind traits. Default and '
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain
    bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample
    checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation,
    accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing
    mutation execution or missing teardown is inconclusive and does not satisfy this criterion.
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the authoritative membership rule for inclusive lower/upper equality, adjacent
    outsiders, negative and zero values, scale refusal, all rounding spellings and equivalent retained representations;
    neither representation is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true at the
    exact upper bound.
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Production helper observations on admitted `B,S` fixtures stay within the one-request `4 × L_B`-byte
    scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step bound, including a longest-pass endpoint fixture and
    a large shift that terminates when quotient becomes zero. Controls exercise actual inline and Large production
    paths, including zero, 32-bit limb crossings, i128 MIN/MAX and signed magnitudes beyond i128. Stream traversal
    requests no allocation and leaves cold inline caches empty; inline streams yield at most four least-significant-first
    u32 limbs. Both stream mechanisms and all scalar s
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Denying the actual scratch reservation reports its actual native request as typed allocation failure.
    A separate checked-capacity failure retains its own classification. Removing the allocation denial yields the
    same membership as a fresh call, with no partial accepted result after either refusal.
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation observed during the longest admitted
    division/comparison path returns the original cause before another limb step or result; cancellation observed
    immediately after successful reservation prevents scratch work. Neither failure is classified as nonmembership
    or work exhaustion.
- id: FR-372-AC-5
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: An integrated supplied Decimal fixture with independently enumerated N owning logical events succeeds
    at N and denies its actual next event at N-1 before entering that event's helper. Its already-spent budget and
    original cancellation are retained across arguments, with no additional helper event, per-limb debit, evaluator
    meter change or partial descriptor. Allocation/capacity/cancellation causes retain their supplied-admission
    locus.
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder and sign invariants, exact reservation
    boundary, checked layout conversions, allocation-free inline unsigned-magnitude stream and Large borrowed dependency
    iterator, absence of cache initialization or BigInt-image construction on those streams, and stated arithmetic
    polling gap. Its record distinguishes requested storage from allocator overhead and work latency from unbounded
    allocator elapsed time.
- id: FR-373-AC-1
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: The public crate-root `ieee_to_exact` has the exact direct signature in Behavior. `IeeeExactTarget` and
    its re-export are absent; no compatibility entry point or copied conversion algorithm remains. A non-Rational
    target cannot be supplied to this signature.
- id: FR-373-AC-2
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Binary32 `0x3fc00000` and binary64 `0x3ff8000000000000` complete as `3/2` with no loss in numerator domain
    `[-3,3]`, denominator domain `[1,2]`. Both signed zeros at both widths complete as `0/1`; only negative zero
    carries `NegativeZeroSign`, preserving FR-366's original binary32 control.
- id: FR-373-AC-3
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Quiet NaN, signaling NaN and positive/negative infinity at each width return exactly `Undefined(IeeeNotFinite)`
    after one operand charge, one work unit, source-width IntegerBits and one ValueOccurrences high-water unit,
    with zero result units and no intermediate or retain admission.
- id: FR-373-AC-4
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Converting `3/2` to numerator domain `[-1,1]`, denominator domain `[1,2]`, and separately to numerator
    domain `[-3,3]`, denominator domain `[1,1]`, returns `Refused(IeeeRationalOutOfDomain { target })` with the
    corresponding exact target. Two work units are consumed; no result unit or retain charge is admitted.
- id: FR-373-AC-5
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Successful finite conversion admits exactly operands, intermediate and retain in that order, consuming
    three work units and one result unit. Ordinary work limits 0, 1 and 2 deny the respective next point with exact
    limit, consumed and next-charge fields. Injected occurrence-1 denials at each point report WorkUnits with limit
    equal to pre-charge consumed work and next charge 1. Result limit 0 denies retain atomically after two work
    units. Smallest positive subnormals at either width under IntegerBits equal to the source width deny intermediate
    with next-charge sizes 150 and 1075 respectively. E
- id: FR-373-AC-6
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: For Requested and Deadline, pre-cancelling the original handle attached to the supplied meter returns
    WorkUnits incomplete at operands with limit = consumed = 0 and next charge 1. Deterministic cancellation at
    the original handle's second and third meter polls returns the same form at intermediate and retain with limit
    = consumed = 1 and 2 respectively. The original handle records the actual cause, only the prior charge prefix
    is admitted and no result is retained. A live handle completes the same fixture.
- id: FR-373-AC-7
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'Precharge the supplied meter once at FunctionCall with work 2, result 1, IntegerBits 80, ValueOccurrences
    5 and TextScalars 7. Direct conversion of either width''s `3/2` under sufficient limits ends with work 5, results
    2 and admission count 4, preserving all three high-water values and the FunctionCall log prefix. Independently
    repeat with work limit 4, result limit 1, an occurrence-1 retain injection and original-handle cancellation
    at the third conversion poll. Each stops at retain with work 4, results 1 and admission count 3; ordinary records
    report respectively WorkUnits(limit=consumed=4) '
- id: FR-374-AC-1
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Admission of zero constructed by primitive, canonical parse and zero-producing arithmetic returns exactly
    `Err(ZeroDivisor)` and no token. Positive and negative values, including values outside i128, admit; `get` borrows
    the original integer, with no Integer/BigInt clone or materialization for admission. The public exports and
    typed unit error match the stated contract.
- id: FR-374-AC-2
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: An external public-API caller can reuse one borrowed copyable token across all three helpers. A plain
    `&Integer` cannot satisfy any of the three divisor parameters. External field construction, writable access
    and a token outliving its referent are rejected by the public type/lifetime boundary; no Default or unchecked
    public constructor permits zero bypass.
- id: FR-374-AC-3
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)` return respectively truncating
    `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and floor `(3,1)`, `(-4,-1)`, `(-4,1)`, `(3,-1)`. A zero dividend with
    either divisor sign returns `(0,0)` in both helpers. Dividing `680564733841876926926749214863536422912` by either
    `2` or `-2` returns respectively quotient `340282366920938463463374607431768211456` or its negative and remainder
    zero, without narrowing.
- id: FR-374-AC-4
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct exact_div on every sign combination of dividend 6 and divisor 2 returns the signed quotient 3;
    zero dividend with either divisor sign returns zero. The beyond-i128 divisible literals in AC-3 return the same
    signed quotient. The nondivisible controls `5/2 = 2` and `-5/2 = -2` retain the prior behavior, without divisibility
    rejection or a fallible helper result.
- id: FR-374-AC-5
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Inspection accounts for every exact-owned production caller in Rational, Decimal and division. Canonical
    callers establish the documented nonzero invariant without forced Integer/BigInt clones, new public errors in
    existing canonical infallible APIs, caller-divisor admission unwrap/expect, unchecked public admission or fabricated
    numeric fallback. Any private token construction has an explicit owning nonzero invariant.
- id: FR-374-AC-6
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: With sufficient real meters, zero divisor under all three divide profiles and both members, plus public
    modulo, returns `Undefined(DivisionByZero)` after exactly the operation's operands charge, with no arithmetic/domain/result-retain
    or admission charge. Denying operands returns the existing Incomplete first. Nonzero success and exposed-member
    refusal controls retain the existing laws, charge requests, ordered prefixes, counters and outcome fields; public
    metered signatures remain unchanged.
- id: FR-375-AC-1
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path
    completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17
    `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- id: FR-375-AC-2
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes
    `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit
    its occurrence-pair traversal or charge schedule.
- id: FR-375-AC-3
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome
    at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted.
    A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted.
- id: FR-375-AC-4
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: With equal-shaped operands whose composite declaration identities differ, checked construction refuses
    with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality
    pair charge is admitted.
- id: FR-375-AC-5
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work
    of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only
    18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge
    = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned.
- id: FR-375-AC-6
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left,
    right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound
    and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion.
- id: FR-375-AC-7
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool
    control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact
    dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real
    callee remains in the outer CBMC graph.
- id: FR-375-AC-8
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: 'With `result_units = 0` and sufficient work and occurrence limits, the actual checked evaluation admits
    `equality.plan-form`, then refuses `equality.plan` atomically with typed `Incomplete { limit_kind: ResultUnits,
    limit: 0, consumed: 0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no equality
    result is returned. A version that checks result capacity only after pair charges fails this control.'
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-369-AC-3
  related_ids: []
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  lines: '153'
  excerpt: 'Every RT checked-package/function invariant producer returns its table cause: unknown name is `UnknownCheckedFunction`,
    a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`. Runtime-managed
    calls use explicit frames and `work_units` fuel under RT FR-273; the final carrier has no `CallDepthExceeded`
    variant, and fuel exhaustion is `Incomplete` with `limit_kind: work_units`, never `CheckedInvariant`.'
  finding: No test or accepted non-Test verification binding exists for FR-369-AC-3; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-002
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-369-AC-4
  related_ids: []
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  lines: '154'
  excerpt: Every RT collection, composite, equality and quantity producer returns its table cause, including all
    six `operand_value` failures and the original kernel-owned `IllTypedCause` when a scheduled comparison or checked
    equality quantity conversion refuses; no site flattens a comparison or conversion error to a message.
  finding: No test or accepted non-Test verification binding exists for FR-369-AC-4; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-003
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-369-AC-7
  related_ids: []
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  lines: '157'
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned
    causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers
    preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former
    unit constructor nor an invented default.
  finding: No test or accepted non-Test verification binding exists for FR-369-AC-7; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-004
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-369-AC-9
  related_ids: []
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  lines: '159'
  excerpt: QSV's deferred field or tuple result uses `DeferredResultNotAdmitted`; missing enum variant and unresolved
    unit use their distinct rows; schedule mismatch and comparator refusal retain their existing rows; each equality
    operand failure uses its exact table row; and scale-zero integer placement uses `ExpectedIntegerPlacement`.
    The comparator and quantity conversion retain their original `IllTypedCause` payload, while prior non-result
    and charge behavior is unchanged.
  finding: No test or accepted non-Test verification binding exists for FR-369-AC-9; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-005
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-1
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '79'
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact
    value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64
    MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations
    with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals.
    Reject malformed/noncanonical spellings as be
  finding: No test or accepted non-Test verification binding exists for FR-371-AC-1; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-006
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-2
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '80'
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal,
    identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt
    access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all
    accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted
    borrows or introduce a cache-state value.
  finding: No test or accepted non-Test verification binding exists for FR-371-AC-2; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-007
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-3
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '81'
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native,
    native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit
    and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete
    fields and admitted prefixes with the independent normative formulas. No representation transition manufactures
    a FunctionCall or other charge.
  finding: No test or accepted non-Test verification binding exists for FR-371-AC-3; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-008
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-4
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '82'
  excerpt: For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate,
    magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator
    observer around standalone Integer calls confirms no allocation on that native path, including clone after cache
    access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently
    prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty.
    Observation of public metered calls separately iden
  finding: No test or accepted non-Test verification binding exists for FR-371-AC-4; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-009
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-5
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '83'
  excerpt: 'Public signatures and all existing Integer methods remain available, except that the divisor parameters
    of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger
    signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain
    &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other
    public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync
    and current panic-unwind traits. Default and '
  finding: No test or accepted non-Test verification binding exists for FR-371-AC-5; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-010
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-6
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '84'
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain
    bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample
    checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation,
    accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing
    mutation execution or missing teardown is inconclusive and does not satisfy this criterion.
  finding: No test or accepted non-Test verification binding exists for FR-371-AC-6; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-011
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-372-AC-5
  related_ids: []
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  lines: '171'
  excerpt: An integrated supplied Decimal fixture with independently enumerated N owning logical events succeeds
    at N and denies its actual next event at N-1 before entering that event's helper. Its already-spent budget and
    original cancellation are retained across arguments, with no additional helper event, per-limb debit, evaluator
    meter change or partial descriptor. Allocation/capacity/cancellation causes retain their supplied-admission
    locus.
  finding: No test or accepted non-Test verification binding exists for FR-372-AC-5; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-012
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-374-AC-1
  related_ids: []
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  lines: '168'
  excerpt: Admission of zero constructed by primitive, canonical parse and zero-producing arithmetic returns exactly
    `Err(ZeroDivisor)` and no token. Positive and negative values, including values outside i128, admit; `get` borrows
    the original integer, with no Integer/BigInt clone or materialization for admission. The public exports and
    typed unit error match the stated contract.
  finding: No test or accepted non-Test verification binding exists for FR-374-AC-1; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-013
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-374-AC-2
  related_ids: []
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  lines: '169'
  excerpt: An external public-API caller can reuse one borrowed copyable token across all three helpers. A plain
    `&Integer` cannot satisfy any of the three divisor parameters. External field construction, writable access
    and a token outliving its referent are rejected by the public type/lifetime boundary; no Default or unchecked
    public constructor permits zero bypass.
  finding: No test or accepted non-Test verification binding exists for FR-374-AC-2; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-014
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-374-AC-3
  related_ids: []
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  lines: '170'
  excerpt: Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)` return respectively truncating
    `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and floor `(3,1)`, `(-4,-1)`, `(-4,1)`, `(3,-1)`. A zero dividend with
    either divisor sign returns `(0,0)` in both helpers. Dividing `680564733841876926926749214863536422912` by either
    `2` or `-2` returns respectively quotient `340282366920938463463374607431768211456` or its negative and remainder
    zero, without narrowing.
  finding: No test or accepted non-Test verification binding exists for FR-374-AC-3; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-015
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-374-AC-4
  related_ids: []
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  lines: '171'
  excerpt: Direct exact_div on every sign combination of dividend 6 and divisor 2 returns the signed quotient 3;
    zero dividend with either divisor sign returns zero. The beyond-i128 divisible literals in AC-3 return the same
    signed quotient. The nondivisible controls `5/2 = 2` and `-5/2 = -2` retain the prior behavior, without divisibility
    rejection or a fallible helper result.
  finding: No test or accepted non-Test verification binding exists for FR-374-AC-4; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-016
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-374-AC-6
  related_ids: []
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  lines: '173'
  excerpt: With sufficient real meters, zero divisor under all three divide profiles and both members, plus public
    modulo, returns `Undefined(DivisionByZero)` after exactly the operation's operands charge, with no arithmetic/domain/result-retain
    or admission charge. Denying operands returns the existing Incomplete first. Nonzero success and exposed-member
    refusal controls retain the existing laws, charge requests, ordered prefixes, counters and outcome fields; public
    metered signatures remain unchanged.
  finding: No test or accepted non-Test verification binding exists for FR-374-AC-6; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-017
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-375-AC-4
  related_ids: []
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  lines: '34'
  excerpt: With equal-shaped operands whose composite declaration identities differ, checked construction refuses
    with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality
    pair charge is admitted.
  finding: No test or accepted non-Test verification binding exists for FR-375-AC-4; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-018
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-375-AC-6
  related_ids: []
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  lines: '36'
  excerpt: The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left,
    right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound
    and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion.
  finding: No test or accepted non-Test verification binding exists for FR-375-AC-6; the criterion remains unverified
    by the repository's computed matrix.
- fnd: FND-019
  severity: medium
  confidence: high
  method: gap-analysis
  check: trace
  artifact_id: FR-375-AC-1
  related_ids:
  - FR-375-AC-2
  path: src/equality.rs
  lines: 303-365
  excerpt: On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path
    completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17
    `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
  finding: The only static binder for FR-375-AC-1 constructs a flat kernel value and calls `planned_equality_with_plan`;
    it does not exercise QSL's required depth-four E21 operands or `CheckedEquality::evaluate`. The matrix tag resolves,
    but the test does not establish this end-to-end criterion.
  test_id: result_bearing_evaluation_returns_relation_and_same_plan_count
  claimed_ac: FR-375-AC-1
  correct_ac: none
- fnd: FND-020
  severity: medium
  confidence: high
  method: gap-analysis
  check: trace
  artifact_id: FR-375-AC-2
  related_ids:
  - FR-375-AC-1
  path: src/equality.rs
  lines: 303-365
  excerpt: On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes
    `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit
    its occurrence-pair traversal or charge schedule.
  finding: The only static binder for FR-375-AC-2 constructs a flat kernel value and calls `planned_equality_with_plan`;
    it does not exercise QSL's required depth-four E21 operands or `CheckedEquality::evaluate`. The matrix tag resolves,
    but the test does not establish this end-to-end criterion.
  test_id: result_bearing_evaluation_returns_relation_and_same_plan_count
  claimed_ac: FR-375-AC-2
  correct_ac: none
bindings:
- test_id: tests::result_bearing_evaluation_returns_relation_and_same_plan_count
  ac_id: FR-375-AC-1
  trace: wrong
- test_id: tests::result_bearing_evaluation_returns_relation_and_same_plan_count
  ac_id: FR-375-AC-2
  trace: wrong
- test_id: tests::pair_and_result_refusals_stop_at_the_first_denied_charge
  ac_id: FR-375-AC-3
  trace: correct
- test_id: tests::plan_reservation_refusals_are_atomic_before_pair_charges
  ac_id: FR-375-AC-5
  trace: correct
- test_id: tests::plan_reservation_refusals_are_atomic_before_pair_charges
  ac_id: FR-375-AC-8
  trace: correct
```

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-002 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-003 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-004 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-005 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-006 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-007 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-008 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-009 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-010 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-011 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-012 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-013 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-014 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-015 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-016 | deferred | Pre-existing gap in an unrelated FR-369/371/372/374 requirement, outside PR #34 / IR-758 scope; retained as repository-wide context. |
| FND-017 | deferred | IR-241 owns the downstream checked construction/type-mismatch path; kernel PR #34 does not claim this AC. |
| FND-018 | deferred | IR-241 owns the real CheckedEquality::evaluate/Kani qualification; kernel PR #34 does not claim this AC. |
| FND-019 | fixed 7509d09a48598affb7ce48928df99520e892c191 | Removed the FR-375-AC-1 binding from this kernel-only test and replaced the flat fixture with an E21-shaped nested kernel schedule fixture. |
| FND-020 | fixed 7509d09a48598affb7ce48928df99520e892c191 | Removed the FR-375-AC-2 binding from this kernel-only test and added first-head-unequal coverage with exact full schedule assertions. |

After excerpt (src/equality.rs, lines 361-392):

```rust
#[test]
fn kernel_result_bearing_evaluation_returns_relation_plan_and_charge_order() {
    for (first_head_unequal, expected) in [(false, true), (true, false)] {
        let (left, right) = e21_list_shape_operands(first_head_unequal);
        assert_eq!(left.occ(), Integer::from(16_u64));
        assert_eq!(right.occ(), Integer::from(16_u64));
        let plan = plan_equality(&left, &right).unwrap();
        assert_eq!(plan.pair_events(), &Integer::from(17_u64));
```

The historical strict matrix summary remains unchanged. Removing the two false AC-1/2 bindings leaves those downstream criteria without a kernel-repo binder; they are assigned to IR-241. The repository-wide matrix remains failing for its pre-existing unrelated gaps.
