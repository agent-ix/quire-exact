---
id: SR-15242
title: IR-705 native integer gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact; PR 78; src/native_integer.rs; src/native_integer/reference.rs;
  src/native_integer/kani_proofs.rs; src/integer.rs; src/lib.rs; src/outcome.rs; src/numeric.rs;
  src/accounting.rs; tests/ir705_native_integer.rs; tests/checked_invariant.rs; Cargo.toml;
  Makefile; CLAUDE.md; AGENTS.md
review_set: subset
---

## Summary

Reviewed exact code head 31ed8641522394746bbe352b64568724f9d01399 against merged specification a4e1fade. No Cargo, formatter, backend, provisioning or aggregate command was run by this reviewer because the lead reserved the sole heavy slot for the author proof. Reported focused checks are author/lead evidence, not independent executions here. Final root aggregate remains required.

## Verdict

FAIL — Required new universal proof/mutation evidence is pending; inherited and consumer-phase obligations remain separately tracked. Plan completion: not assessed

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-359-AC-9 retains inherited ignored-only or absent executable trace/qualification evidence in the computed matrix; this native API change does not discharge that owner-held obligation. | spec/functional/FR-359-cumulative-meter-boundary.md:Acceptance Criteria |
| FND-002 | medium | FR-359-AC-10 retains inherited ignored-only or absent executable trace/qualification evidence in the computed matrix; this native API change does not discharge that owner-held obligation. | spec/functional/FR-359-cumulative-meter-boundary.md:Acceptance Criteria |
| FND-003 | medium | FR-371-AC-6 retains inherited ignored-only or absent executable trace/qualification evidence in the computed matrix; this native API change does not discharge that owner-held obligation. | spec/functional/FR-371-bounded-integer-representation-refinement.md:Acceptance Criteria |
| FND-004 | medium | FR-383-AC-4 retains inherited ignored-only or absent executable trace/qualification evidence in the computed matrix; this native API change does not discharge that owner-held obligation. | spec/functional/FR-383-meter-cancellation-handle-access.md:Acceptance Criteria |
| FND-005 | high | Full-capacity numerical/ordering proof receipts and the substantive arithmetic/admission-bound mutant counterexamples are not yet available at reviewed head 31ed8641522394746bbe352b64568724f9d01399. Finite BigInt differential tests establish sampled agreement only; do not merge as proven refinement before all required healthy properties, covers and falsifying controls are qualified. | src/native_integer/kani_proofs.rs:9-145 |
| FND-006 | medium | The native multi-step allocator observation exercises a handwritten public client, not ordinary generated-client execution; authentic generated-client evidence remains consumer-owned IR-705 adoption work. | tests/ir705_native_integer.rs:528-592 |
| FND-007 | medium | Genuine TC-027 healthy/bound mutant, TC-025 arithmetic mutant, IR-917 and IR-948 profiles, original-domain playback and impossible-precondition CoverUnsatisfied remain untagged and unqualified. This is the explicit next consumer phase under IR-705; native source tests provide no 16 GiB fit or program completion. | spec/functional/FR-384-admitted-native-integer-refinement.md:Acceptance Criteria |

## Coverage

Full computed matrix: 194 criteria, 171 tagged, 18 method-without-symbol (inspection), four untagged, one tagged-by-ignored-test. No handwritten matrix. Plan completion: not assessed

src/native_integer.rs; src/native_integer/reference.rs; src/native_integer/kani_proofs.rs; src/integer.rs; src/lib.rs; src/outcome.rs; src/numeric.rs; src/accounting.rs; tests/ir705_native_integer.rs; tests/checked_invariant.rs; Cargo.toml; Makefile; CLAUDE.md; AGENTS.md

The abstraction maps an i128 two's-complement payload to exact sign and unsigned magnitude, including MIN = -2^127. Signed addition/subtraction compares magnitudes; equal-sign sum is at most 2^128 and explicitly retains the carry. Negation changes sign with zero normalization. Multiplication expands (a0+2^64*a1)(b0+2^64*b1), retains both low-word carries, and forms the exact high word; each magnitude is at most 2^127 so the product is at most 2^254 and the high-word accumulation cannot overflow u128. The native-fit predicate includes -2^127 and excludes +2^127. Signed ordering reverses magnitude order only for negative pairs. This is an independently expressed mathematical abstraction, not production checked_i128 operations reused as the expected result. Host BigInt binding tests are finite, explicitly not the universal machine proof. All five full-width harnesses must finish, with covers; arithmetic harnesses call actual production calculate(), and ordering calls the actual public evaluator/real Meter. No assumption narrows the six symbolic i128 values or their sorted interval population. Numerical proofs do not establish arbitrary-budget Meter refinement.

Opaque fields and closed operation/operand enums prevent public certificate forging; input membership is checked before charging. Capacity and semantic result restriction remain separate. Native success contains only Copy i128/range/plan values. General Integer/as_big is unchanged except private crate visibility for endpoint ingress. Refusal target allocation occurs only on real domain refusal after arithmetic charge. No cfg replacement evaluator, unsafe, new dependency, lock or CI-lane change was found. Native schedule compares to normative formulas and existing general evaluator, with actual ten-counter, incomplete payload and support-log observations. Scope tags below are inspected bindings, not proof receipts.

## Examined scope

```yaml
scope:
- id: FR-088-AC-11
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: For an enum shape whose canonical member list is `b`, `a`, `c`, the ranks
    of the variants are `b` 0, `a` 1, `c` 2, and a set of all three values visits
    `b`, `a`, `c`. For an unordered enum whose canonical list is `a`, `b`, `c`, the
    ranks are `a` 0, `b` 1, `c` 2, and the set visits `a`, `b`, `c`. `compare_keys`
    orders two members of one shape by rank, whatever the order of their `VariantId`
    digests. A value that pairs a variant's `VariantId` with a rank that differs from
    the shape's rank for it is refused at admission. Two shapes that differ in the
    identity of a variant but not in rank and orde
- id: FR-088-AC-12
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: A declared unit `UnitId` and a compound unit `UnitId` are unequal when
    their domains or bytes differ. A declared unit's `UnitId` carries its node key
    under the `quire.checked-semantic-node/v1` label, and a compound unit's carries
    its digest under the `quire.value.compound-unit/v1` label. Two `UnitId`s of different
    domains never compare equal, whatever their bytes.
- id: FR-089-AC-2
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: The kernel `Value::Population` variant's payload type is `PopulationId`;
    no kernel source file imports a population binding or any other caller's model
    type to define, construct or match this variant, and the crate's `[dependencies]`
    name no crate that owns one.
- id: FR-089-AC-6
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: 'Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`,
    kernel `ValueType::admits` returns false; given two `Value::Population` operands,
    kernel `equality::plan_pairs` returns `Err(Refusal::CheckedInvariant { cause:
    PopulationPair })` and kernel `key::compare_keys` returns `None`.'
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
  excerpt: '`Refusal::CheckedInvariant` carries only FR-369''s closed typed cause;
    it has no arbitrary caller message, QSL model/checker type, catalog code or generic
    caller-cause escape hatch.'
- id: FR-096-AC-8
  path: spec/functional/FR-096-kernel-refusal-code-and-cause.md
  role: examined
  excerpt: 'Each catalogued kernel refusal below returns the code and cause spelling
    named for it from `Refusal::code()` and `Refusal::cause()`, and every FR-369 `CheckedInvariant
    { cause }` returns neither. `InexactDecimal`: `inexact_decimal`, `nonzero-discarded-digit`.
    `DecimalOutOfDomain`, `ModuloOutOfDomain`, `TextLengthOutOfDomain`, `IntegerOutOfDomain`,
    `RationalOutOfDomain`, `IeeeRationalOutOfDomain`: `<name in snake case>`, `outside-domain`.
    `DivisionOutOfDomain`: `division_out_of_domain` with `quotient-outside-domain`
    for the quotient member or `remainder-outside-domain` for the remainder member.'
- id: FR-097-AC-7
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: An unbounded collection type admits a collection value of any size with
    no cardinality refusal; it still charges `collection.bound`, and stops with `Incomplete`
    at that charge point only when the caller's meter runs out. `K<T>` and `K<T>[0,
    u64::MAX]` are different types.
- id: FR-097-AC-8
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: A caller outside the crate can call `CardinalityBound::violation(count)`
    and receive the existing `Option<BoundViolation>` type. For `[2, 4]`, counts 1,
    2, 3, 4 and 5 return below-minimum, none, none, none and above-maximum respectively;
    singleton `[2, 2]` admits only 2. `[0, u64::MAX]` reports no violation at either
    extreme, `[0, 0]` rejects 1 above, and `[u64::MAX, u64::MAX]` rejects `u64::MAX
    - 1` below. The query is pure and unmetered; for `[2, 4]`, collection admission
    agrees with its decisions at counts 1 through 5.
- id: FR-262-AC-2
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s
    around `Boolean` clones, compares equal to its clone, hashes equal to its clone,
    formats for debug and drops. A `Value` of the same depth compares equal to its
    clone, and it and its clone drop.
- id: FR-262-AC-3
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: A `Value` nested N levels deep, alternating `Option` and `Sequence` levels
    around an `Integer` with each level declared with the type of its depth, holds
    at least N - 10 and at most 2N distinct `Option` and `Collection` type nodes,
    and the same value 2N levels deep holds at most twice the nodes of the N-level
    value plus 2. On a thread with a 512 KiB stack, such a value 100,000 levels deep
    and its 100,000-deep type are built, the type admits the value, and the type clones,
    compares equal to its clone, hashes equal to its clone, formats for debug with
    balanced brackets and drops, as does the val
- id: FR-357-AC-1
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a zero divisor under any profile and either member, `divide` returns
    `Undefined(DivisionByZero)`.
- id: FR-357-AC-2
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `10 div y` over `1..=10` at `y = 5`, `divide` of the quotient returns
    2; given `x rem -1` over `-10..=5` at `x = -10`, `divide` of the remainder returns
    0.
- id: FR-357-AC-3
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a quotient outside the domain, `divide` refuses with cause `quotient-outside-domain`;
    given a remainder outside the domain, it refuses with `remainder-outside-domain`.
- id: FR-357-AC-4
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given -7 and 2, truncating returns (-3, -1), floor (-4, 1) and Euclidean
    (-4, 1); given -7 and -2, truncating returns (3, -1), floor (3, -1) and Euclidean
    (4, 1).
- id: FR-357-AC-5
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `i64::MIN` and -1 under any profile, `divide` of the quotient returns
    2^63 in a domain that holds it. In a signed 64-bit domain the quotient refuses
    with `quotient-outside-domain` and the remainder completes with 0.
- id: FR-357-AC-6
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a successful `div` or `rem`, the meter admits `integer-division.operands`,
    `integer-division.arithmetic`, `integer-division.domain` (one value occurrence)
    and `integer-division.result-retain` (one result unit), in that order. A refusal
    admits no `result-retain`, a zero divisor admits only operands, and a result limit
    of 0 reports `integer-division.result-retain` as incomplete.
- id: FR-357-AC-7
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: For dividend/divisor pairs `(7, 3)`, `(7, -3)`, `(-7, 3)` and `(-7, -3)`,
    public `modulo` in the mathematical domain completes to 1, 1, 2 and 2 respectively.
    Running a `divide` under each of Truncating, Floor and Euclidean before each `modulo`
    call does not change those results; `modulo` has no profile argument.
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit
    P charge under an ordinary limit, then admit P once and an unrelated point once.
    The next P charge returns `Incomplete` at P with the work counter as its limit
    kind, proving the ordinary refusal did not advance the named occurrence; all consumed
    counters, admission count, and test-support log equal their values immediately
    before each refusal. A subsequent P charge succeeds and updates ordinary accounting
    and the log exactly once.
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan`
    returns `Incomplete` at `equality.plan` without changing any consumed counter,
    admission count, or test-support log; a subsequent valid `charge_plan` succeeds
    and records its ordinary plan charge.
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction
    with zero is rejected by that type, while a nonzero occurrence constructs a denial.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor
    returns all 4096 records with their points in order and `charge_log_truncated()`
    is false. On the 4097th admission the slice still contains exactly that prefix
    and the flag becomes true.
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()`
    and consumed work units reflect all 4097 admissions, including the one omitted
    from the diagnostic log.
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge
    log and its `Counters` accounting state remains fixed-size and heap-free.
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one
    result unit. The log retains the first 4096 points and reports truncation, while
    `admission_count()`, consumed work and consumed results each equal 4097.
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`,
    admit 4097 one-work-unit charges and request one more work unit. The charge returns
    `Incomplete` at its requested point with `WorkUnits`, `limit = consumed = 4097`,
    `next_charge = 1`; every counter, admission count, prefix log and truncation flag
    are unchanged.
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`,
    admit 4097 charges each consuming one result unit, then request one more result
    unit. The charge returns `Incomplete` with `ResultUnits`, `limit = consumed =
    4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation
    flag remain unchanged, including the work counter.
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial
    for one named point. A charge at that point returns `Incomplete` with `WorkUnits`
    and leaves all counters, admission count, prefix log and truncation flag unchanged.
    Retrying that charge under sufficient ordinary limits succeeds once, increments
    work and admission count once, and does not extend the 4096-entry log.
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'After admitting two work units, inject occurrence 1 at point P and attempt
    a three-work-unit `charge` at P under otherwise sufficient limits whose configured
    work limits are 10 and 100 on separate meters. Both refusals report exactly `limit_kind
    = WorkUnits`, `charge_point = P`, `limit = consumed = 2`, and `next_charge = 3`,
    independently of either configured limit. Repeat with an injected `equality.plan`
    denial, `pairs = 2`, the same pre-consumed work and limits, and sufficient value-occurrence
    and result limits: both `charge_plan` refusals report `WorkUnits`, `charge_point
    = equality.plan`, '
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'With an occurrence-1 injected denial selected, make the same `charge`
    independently fail an ordinary semantic-size counter and make a `charge_plan`
    with `pairs = 2` independently fail its work reservation under a configured work
    limit of 3 after two admitted work units, with all other limits sufficient. In
    each case, the injected `WorkUnits` record is returned before the ordinary refusal:
    `limit = consumed` equals the pre-charge work, `next_charge` is the charge''s
    work amount or the plan''s reservation respectively, and every consumed counter
    and admission count remain unchanged. The criterion '
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request
    of 9 followed by 3 and then a charge with no size request. The records expose
    respectively 9, 3 and no IntegerBits request while the consumed IntegerBits maximum
    remains 9. A separate charge with IntegerBits requests 9, 3 and zero exposes present
    9; a charge with any positive number of IntegerBits zero requests exposes present
    zero. Both have the same fixed payload size as a single-request record, with no
    heap-owned request collection and no retained request list, order or multiplicity.
    A successful `charge_plan` with pai
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently
    chosen semantic-size requests, including repeated same-kind requests and distinct
    amounts at admissions 4096 and 4097. The single record log contains exactly the
    first 4096 fixed-size records with their points and per-admission semantic-size
    maxima in admission order; truncation is false at admission 4096 and true at 4097.
    Work consumption and admission count reach 4097, including the omitted record;
    existing size maxima still include the 4097th charge.
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX -
    1` followed by one work unit reaches `u64::MAX` exactly. The next one-unit charge
    returns `Incomplete` at its named point with `limit_kind = WorkUnits`, `limit
    = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission
    count stay at their pre-refusal values.
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting
    `u64::MAX` result units. Its work consumption is 1. A second charge requesting
    one result unit returns `Incomplete` at its named point with `limit_kind = ResultUnits`,
    `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission
    count remains 1.
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount
    is `u64::MAX + 1` returns `Incomplete` with `limit_kind = WorkUnits` and `next_charge
    = u64::MAX + 1`; no counter or admission count changes.
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit
    and `u64::MAX + 1` result units returns `Incomplete` with `limit_kind = ResultUnits`
    and `next_charge = u64::MAX + 1`; no counter or admission count changes.
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit
    under limits of at least 16 bits, `u64::MAX` work units and one result unit. Then
    attempt a charge carrying `integer_bits = 16` and one result unit. The result
    refusal leaves `integer_bits = 8`, work units, result units and admission count
    at their pre-refusal values; under `test-support`, the admitted-charge log is
    unchanged too.
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters
    the first unavailable counter of a charge, including several simultaneously short
    counters and semantic sizes attached in either order. The reported `limit_kind`
    follows `ScalarLimits` field order, with the exact limit, previously consumed
    amount, requested amount and point. A refusal after an earlier size check succeeds
    changes no counter, admission count or test-support log. A separate meter with
    a pending later named denial proves that this ordinary refusal does not advance
    its occurrence.
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers
    without panic for every member of `LimitKind::ALL`: untouched counters are zero,
    semantic sizes are admitted high-water maxima and work/results are admitted cumulative
    totals.'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum
    observable through `consumed()`; the diagnostic log record is verified under FR-359-AC-12.
    With `IntegerBits` limit 5 and requests 7 then 9 for that kind in one charge,
    refusal reports `next_charge = 7`, the first-inserted failing amount; the existing
    9 then 7 case alone cannot distinguish maximum coalescing before refusal. Simultaneous
    shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters,
    high-water, admissions, log and a pending injected occurrence. Cancellation and
    injection precedence, c
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference`
    uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand
    charge with at most two semantic-size requests, arithmetic charge with one, result-retain
    charge with none) over the full `i64` operand range, plus separate repeated-kind
    charge controls with at most two requests each. Symbolic `i64` operands and symbolic
    limits including `ResultUnits` cover first-short and simultaneous shortages and
    absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and
    600 seconds, the basel
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests
    below pass their own assertions under their original domains, oracle, completion
    and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds. Record their
    individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test
    or altered consumer harness substitutes for it.
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and
    called helpers finds no traversal with a trip count greater than that charge's
    semantic-size request count, including no traversal of `LimitKind::ALL` or another
    independently sized collection. The FR-362 proof schedule has at most two requests
    per charge, while the public `Charge` API retains no request-count ceiling.
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362
    schedule contains exactly the admitted prefix in charge order, with each record's
    point and absent-or-present per-admission semantic-size maxima matching an independent
    model. Reordered and repeated requests of the same kind, including zero and distinct
    amounts, coalesce to the per-admission maximum in that record. A refused charge
    appends no record and leaves the existing log unchanged. The named `tc_931_admitted_log_matches_model_ordered_prefix`
    and `tc_931_admitted_log_matches_model_same_kind` tests verify these prope
- id: FR-360-AC-1
  path: spec/functional/FR-360-integer-minimum-magnitude.md
  role: examined
  excerpt: '`Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808
    (`2^63`), is nonnegative, and differs from the original negative value. It neither
    overflows nor returns `i64::MIN`.'
- id: FR-361-AC-1
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For an integer multiplication with a large operand, set `integer_bits`
    to one below the exact `integer-arithmetic.arithmetic` amount while permitting
    the operand charge. The outcome is `Incomplete` at `integer-arithmetic.arithmetic`
    with `limit_kind = IntegerBits` and that exact `next_charge`; the observed peak
    request meets the arithmetic allocation bound in Behavior. Repeat with operands
    whose subtraction mathematically cancels to 1 but whose operand-derived arithmetic
    amount exceeds the limit.
- id: FR-361-AC-2
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a rational operation with a large numerator or denominator, set `integer_bits`
    to one below the exact `rational-arithmetic.arithmetic` amount while permitting
    the operand charge. The outcome is `Incomplete` at `rational-arithmetic.arithmetic`
    with `limit_kind = IntegerBits` and that exact `next_charge`; the observed peak
    request meets the arithmetic allocation bound in Behavior. Include a multiplication
    whose unreduced large factors cancel mathematically to 1.
- id: FR-361-AC-3
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Inject a one-shot denial at each of `integer-division.arithmetic` and `integer-modulus.arithmetic`
    for large nonzero operands. Each returns `Incomplete` at the selected point with
    `limit_kind = WorkUnits`, admits no later point from that call, and meets the
    division allocation bound in Behavior.
- id: FR-361-AC-4
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For valid decimal operands requiring a `2^20`-place scale expansion, set
    `scale_expansion` to one below the planned shift and leave other limits sufficient.
    The outcome is `Incomplete` at `decimal.scale-expansion` with `limit_kind = ScaleExpansion`
    and `next_charge` equal to the exact shift; the observed peak request is below
    4,096 bytes.
- id: FR-361-AC-5
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For decimal addition of small coefficients at scales 0 and `2^20`, set
    `integer_bits` to the admitted scale-expansion bit amount, one below the planned
    arithmetic bit amount, with other limits sufficient. The outcome is `Incomplete`
    at `decimal.arithmetic` with `limit_kind = IntegerBits` and `next_charge` equal
    to that planned bit amount; the observed peak request is below 4,096 bytes.
- id: FR-361-AC-6
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a decimal value retained at a target scale requiring a `2^20`-place
    power of ten, set `decimal_digits` one below the exact result-retain amount while
    permitting preceding charges. Evaluation returns `Incomplete` at `decimal.result-retain`
    with `limit_kind = DecimalDigits` and that exact `next_charge`; the observed peak
    request is below 4,096 bytes.
- id: FR-361-AC-7
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: On a fresh meter, floor division of a positive at-least-65,536-bit dividend
    by 3, exposing the quotient in the mathematical domain, completes to the independently
    calculated floor quotient with `integer_bits` limit equal to the dividend magnitude
    bit length. Final consumed `integer_bits` equals that exact bit length.
- id: FR-361-AC-8
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For that floor quotient fixture, lower the `integer_bits` limit by one.
    The call returns `Incomplete` at `integer-division.operands` with `limit_kind
    = IntegerBits`, `consumed = 0` and `next_charge` equal to the dividend bit length;
    it admits no point and the measured peak allocation request stays below one eighth
    of the largest operand byte length.
- id: FR-361-AC-9
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Multiply retained decimals `(1, 0)` and `(1, 0)` into an Exact target with
    coefficient bounds 0 through 1 and scales 0 through `2^20`. At `decimal_digits
    = 2^20`, the `decimal.result-retain` charge returns `Incomplete` with `limit_kind
    = DecimalDigits`, `limit = 2^20`, `consumed = 2`, `next_charge = 2^20 + 1`; only
    the three preceding decimal points are admitted, no result unit is consumed, and
    the peak request is below 4096 bytes.
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`,
    -1, 0, 1 and `i128::MAX`; for negate, use each member as a unary operand. Compare
    integer add, subtract, multiply and negate only when the corresponding checked
    `i128` operation returns a value. Repeat over reproducibly sampled signed inputs.
    Every compared kernel result equals the independent checked result.
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes
    to the independently authored decimal value `340282366920938463463374607431768211454`;
    negating `Integer::from(i128::MIN)` completes to `170141183460469231731687303715884105728`.
    Neither result is narrowed to `i128`.
- id: FR-362-AC-3
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For fixed and reproducibly sampled rational pairs whose checked `i128`
    reference intermediates fit and whose denominators are nonzero, rational add,
    subtract, multiply, divide by nonzero and negate equal an independently reduced-fraction
    oracle. A zero rational divisor returns `Undefined(DivisionByZero)` after only
    the operand charge.
- id: FR-362-AC-4
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Integer and rational comparisons over fixed and reproducibly sampled pairs
    equal independently checked numeric comparisons for equality and order. Decimal
    comparison is specified by [FR-363](./FR-363-metered-decimal-ordering.md).
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each
    rational arithmetic operation, integer ordering and rational ordering, the admitted
    point sequence equals its family row in Behavior. A successful already-decided
    Boolean retention admits only `boolean.result-retain`.
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate
    the maximum size amount across their points from the formulas in Behavior, using
    the unreduced numerator and denominator for rational normalization. The observed
    final `integer_bits` and `value_occurrences` equal those maxima; work consumption
    equals the number of admitted points and result consumption is one.
- id: FR-362-AC-7
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter, directly add integers 5 and 7, compare rationals 1/2 and
    2/3 for less-than, then retain an already-decided Boolean `true`. The admitted
    atom sequence consumes 7 work units and 3 result units, derived from the three
    family rows in Behavior.
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer
    arithmetic, rational arithmetic, integer ordering, rational ordering and already-decided
    Boolean retention calls. Each returns `Incomplete` naming the denied point, with
    no completed value, no result unit for the denied charge and no later point admitted
    by that call.
- id: FR-362-AC-9
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For every already-decided Boolean connective, the completed value matches
    its truth table and the kernel admits `boolean.result-retain` once, consuming
    one work and one result unit.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract,
    multiply, divide and negate, and integer/rational ordering, independently calculate
    the required bit high-water amount from Behavior. On fresh meters with other limits
    sufficient, that exact bit limit completes with the expected value, while a limit
    one below stops at the first point whose amount exceeds it, with exact prior consumption,
    request and admitted prefix and no result unit. Include cancellation `1000 - 999`,
    `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate
    `-5/8`, and `7/3 < 5/8`. '
- id: FR-362-AC-11
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`,
    typed domain `Refused`, and `Incomplete` with no partial value; `completed()`
    returns a value only for `Completed`, including false. On fresh meters, rational
    division by zero retains only operands; integer `2 + 2` outside the bound `[0,
    3]` retains operands and arithmetic but no result unit; a rational result outside
    its domain retains operands, arithmetic and normalize but no result unit. A denied
    first operand charge admits nothing. Each stop preserves exactly the admitted
    prefix, work totals and size maxima, w
- id: FR-362-AC-12
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '`Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator
    1 and denominator 2, proving reduction to lowest terms and cancellation of two
    negative signs. `Rational::new(1, -2)` exposes -1/2. For either denominator 5
    or -5, a zero numerator exposes exactly 0/1.'
- id: FR-362-AC-13
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)`
    is false. Dividing `1/1` by `3/1` under that domain returns `Refused(RationalOutOfDomain
    { target })` after normalization with no result unit; the identical call without
    a domain completes to `1/3`.
- id: FR-362-AC-14
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Call the five scalar atoms `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0`
    directly on one meter. They complete to true, 1, true, 0, false in order, consuming
    15 work and 5 result units. With work limit 14, the final ordering stops at `ordering.result-retain`
    with `limit = consumed = 14`, `next_charge = 1` and four result units.
- id: FR-362-AC-15
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Directly divide rational `3/1` by `2/1` on a fresh meter. The result is
    `3/2`, with high-water `integer_bits = 3`, value occurrences 2, work 4 and results
    1; work limit 3 stops at `rational-arithmetic.result-retain` after the first three
    points and consumes no result unit.
- id: FR-362-AC-16
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter directly add integers `0 + 1` and then `1 + 2`. The results
    are 1 and 3; the integer point sequence occurs twice, high-water `integer_bits
    = 3`, occurrences 2, work 6 and results 2. Caller fold evaluation is outside this
    direct-atom criterion.
- id: FR-362-AC-17
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain`
    after operands and arithmetic, with zero result units; work limit 3 completes
    to 12 with three work and one result unit. For rational `(2/3) * (3/2)`, work
    limit 2 stops at `rational-arithmetic.normalize`, work limit 3 stops at `rational-arithmetic.result-retain`,
    and work limit 4 completes to `1/1`; each denied call preserves exactly its admitted
    prefix and consumes zero result units.
- id: FR-362-AC-18
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For `k` in 63, 64, 65, 127, 128, 200 and 511, compare exact results and
    charged bit high-water sizes around `2^k`: `(2^k - 1)(2^k + 1)` charges `2k +
    1` bits for a `2k`-bit result, `(2^k)^2` charges `2k + 2` for a `2k + 1`-bit result,
    and adding `(2^k - 1) + 1` charges `k + 1` for a `k + 1`-bit result. Subtracting
    `(2^k + 1) - 2^k` or `2^k - 2^k` still charges `k + 2` bits despite a one-bit
    result.'
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'Through public `evaluate_integer_arithmetic` and a fresh public `Meter`
    for each call, add `8 + 3` and `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`,
    without a result bound and with all non-bit limits sufficient. The independently
    calculated `integer-arithmetic.arithmetic` bit amounts are respectively 5, 5,
    5 and 6: add/subtract use `max(B(a), B(c)) + 1`, including the larger right operand
    and a result that cancels to one bit; multiply uses `B(a) + B(c)`. At each exact
    `integer_bits` limit, the call completes to respectively 11, 11, 1 and 24, admits
    operands, arithmetic and result-retain in '
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On fresh sufficient public meters under `test-support`, evaluate the six
    rational calls listed below with no result domain. Each completes to the listed
    canonical value and admits operands, arithmetic, normalize and result-retain in
    order, consuming four work units and one result unit. The actual normalize record
    has only IntegerBits present, with the listed amount independently derived from
    the unreduced parts and `B(0) = 1`. The recorded arithmetic amount equals its
    separate listed bound. The normalize observation is not replaced by that bound,
    the final bit maximum or the reduced value's pa
- id: FR-362-AC-21
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On a fresh public meter, a direct default charge at `TemporalVisit` (`temporal.visit`)
    succeeds with `work_units = 1`, `result_units = 0` and no semantic-size request
    when the work limit is exactly 1. With work limit 0, the same call returns `Incomplete`
    with `limit_kind = WorkUnits`, `limit = consumed = 0`, `next_charge = 1` and `charge_point
    = TemporalVisit`; no charge is admitted and all counters remain zero.
- id: FR-362-AC-22
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: A one-shot injected denial of the first direct charge at `TemporalVisit`
    (`temporal.visit`) returns `Incomplete` naming `TemporalVisit` and `WorkUnits`,
    with `limit = consumed = 0` and `next_charge = 1`; it admits no charge, changes
    no counter and consumes the one-shot denial, so a second identical charge on the
    same otherwise sufficient meter succeeds.
- id: FR-363-AC-1
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For decimal pairs with equal values but different retained scales, and
    for positive, negative and zero values at differing scales, `order_numbers` agrees
    with an independent exact rational comparison for each ordering operator. The
    retained representations remain unchanged.
- id: FR-363-AC-2
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For a successful decimal comparison, the meter admits `ordering.operands`,
    `ordering.arithmetic`, `ordering.result-retain` in that order. On a fresh meter,
    its size high-water counters equal the maxima in Behavior, with three work units
    and one result unit.
- id: FR-363-AC-3
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: Comparing retained `(0, 5) >= (0, 0)` completes `true` and consumes `integer_bits
    = 18`, `decimal_digits = 6`, `scale_expansion = 5`, `value_occurrences = 2`, `work_units
    = 3`, `result_units = 1`. Comparing retained `(100, 2) <= (2, 0)` completes `true`
    and consumes `integer_bits = 9`, `decimal_digits = 3`, `scale_expansion = 2`,
    `value_occurrences = 2`, `work_units = 3`, `result_units = 1`.
- id: FR-363-AC-4
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: With all limits at `u64::MAX` except `decimal_digits = 64`, testing retained
    `(1, 0) < (1, u32::MAX)` returns `Incomplete` at `ordering.arithmetic` with `limit_kind
    = DecimalDigits`, `limit = 64`, `consumed = 1`, and `next_charge = 4,294,967,296`.
    Only `ordering.operands` is admitted; no result unit is consumed and the scale-alignment
    power of ten is not materialized.
- id: FR-363-AC-5
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: 'On separate fresh meters with generous limits, inject a one-shot denial
    at occurrence 1 of each decimal-ordering point: `ordering.operands`, `ordering.arithmetic`,
    and `ordering.result-retain`. For each point, ordering retained `(0, 5) >= (0,
    0)` returns `Incomplete` with `limit_kind = WorkUnits`, `charge_point` equal to
    the selected point, `limit = consumed` equal to the number of preceding admitted
    points (0, 1, or 2), and `next_charge = 1`. No Boolean result or result unit is
    exposed; only the preceding point prefix is admitted and logged, with no later
    point admitted.'
- id: FR-363-AC-6
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For retained decimals `(110, 2)` and `(11, 1)`, public unmetered `Decimal::compare`
    returns `Equal`, `numerically_equal` returns true and both normalized representations
    are `(11, 1)`; their retained representations remain `(110, 2)` and `(11, 1)`.
- id: FR-363-AC-7
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)`
    both complete false. The mixed pair consumes integer bits, decimal digits and
    scale expansion 8/3/1; the narrow pair consumes 4/2/0. Both consume value occurrences/work/results
    2/3/1 with all other counters zero, despite equal normalized values.
- id: FR-364-AC-1
  path: spec/functional/FR-364-ieee-flag-iteration.md
  role: examined
  excerpt: For every subset of the five public flags, collect it into `IeeeFlags`
    in forward and reverse vocabulary order. Both `iter()` results equal that subset
    filtered from `IeeeFlag::ALL`, without duplicates. The empty set iterates empty
    and the full set iterates all five flags in the stated order.
- id: FR-365-AC-1
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: Round retained `(5, 1)` and `(-5, 1)` to a scale-zero target with coefficient
    bounds -10 through 10 under each of the six modes. Values or `Refused(InexactDecimal
    { target })` match the tie table in Behavior; each completed representation has
    scale zero, and refusals consume no result unit.
- id: FR-365-AC-2
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: '`RoundingMode::default()` and the mode read back from a target constructed
    with it are `Exact`. Rounding retained `(1, 1)` to that scale-zero target returns
    `Refused(InexactDecimal { target })`, rather than silently rounding 0.1 to zero,
    with no retained result.'
- id: FR-366-AC-1
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For same-width Add with a positive quiet NaN of payload 5 and a negative
    signaling NaN of payload 9 in either operand order, the result bits are the first
    NaN quieted and Invalid is present. Two quiet NaNs select the first without Invalid.
    For FusedMultiplyAdd with a quiet first NaN, finite second operand and signaling
    third NaN, the first NaN remains selected and Invalid is present. Exercise both
    binary widths.
- id: FR-366-AC-2
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: 'Converting a binary64 NaN with payload `2^22` or `2^30` to binary32 returns
    `Refused(IeeeNanPayloadNotRepresentable { source: Binary64, target: Binary32 })`
    without truncating the payload or retaining a result unit. At payload `2^22 -
    1`, positive and negative binary64 quiet NaNs both complete as binary32 quiet
    NaNs with their respective signs and all payload bits preserved, and one result
    unit is consumed.'
- id: FR-366-AC-3
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: Converting either signed binary32 zero to the rational domain with numerator
    interval `[-1, 1]` and denominator interval `[1, 1]` completes to canonical `0/1`.
    Positive zero has no loss; negative zero has exactly `IeeeExactLoss::NegativeZeroSign`.
- id: FR-366-AC-4
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For each width, distinct bit patterns representing the ten classes in Behavior
    have strictly increasing `total_order_key()` values in that stated order, including
    quiet/signaling NaNs of both signs and both zeros. Positive NaN payloads of the
    same class ascend by payload and negative ones descend. The public TotalOrder
    comparison agrees with the key order for these pairs; NumericEqual considers the
    two zeros equal despite their distinct keys.
- id: FR-367-AC-1
  path: spec/functional/FR-367-text-profile-mismatch-before-charging.md
  role: examined
  excerpt: 'Admit the same UTF-8 text "a" once with Nfc and once with BinaryUtf8 using
    separate sufficient meters. On a fresh comparison meter whose ten limits are zero,
    public `compare_text(Equal, left, right)` returns precisely `Err(IllTyped { cause:
    DistinctTextProfiles })`. Every consumed counter and admission count remains zero;
    under `test-support` the charge log is empty and truncation is false.'
- id: FR-368-AC-1
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`,
    has a unique documented spelling from `as_str()` and round-trips through `from_code()`;
    an unknown spelling returns `None`. `TemporalVisit` spells `temporal.visit` and
    is at index 62 after `ModelNavigate` at index 61; entries 0 through 61 retain
    their pre-extension order. Exhaustive variant matching makes an added but unaccounted
    variant fail the check.
- id: FR-368-AC-2
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 'Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`,
    has a unique documented `as_str()` spelling, and the sequence is exactly the ten
    `ScalarLimits` fields: integer bits, decimal digits, scale expansion, text input
    bytes, text scalars, normalized scalars, unit edges, value occurrences, work units
    and result units. Exhaustive variant matching makes an added but unaccounted variant
    fail the check.'
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause`
    payload whose closed variant set and four typed `IllTypedCause` payloads equal
    the thirty table rows; `CheckedInvariantCause` implements the stated traits, and
    the old unit constructor does not compile.
- id: FR-369-AC-2
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Every kernel producer in collection, value, division and equality returns
    exactly its table cause; two population values return `PopulationPair`, a population
    paired with another kind returns `ValueKindMismatch`, and unlike collection kinds
    return `CollectionKindMismatch`. Two composites of different declarations return
    `false` and a composite paired with a non-composite returns `ValueKindMismatch`
    (FR-376-AC-1, FR-376-AC-6).
- id: FR-369-AC-5
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()`
    return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ.
    Ordinary kernel refusal codes and causes are unchanged.
- id: FR-369-AC-6
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Every received typed cause follows QSL''s existing internal-fault path:
    no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant`
    at S6a, `CallFailure::Fault` at the public call, and `Failed` with unavailable
    basis for an internal replay fault. It never becomes an ordinary replay refusal,
    `Inconclusive`, `Incomplete(Cancelled)`, a proved result or a certification rejection;
    an unknown received cause retains fault provenance.'
- id: FR-369-AC-8
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public carrier contains no QSL model/checker/catalog type or dependency
    on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and
    an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant
    constructor assigned exactly one cause from the two tables.
- id: FR-370-AC-1
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: A UUID value is admitted only by `ValueType::Uuid` and a Timestamp value
    only by `ValueType::Timestamp`; crossed native kinds and Integer, Text and Reference
    values are rejected by both native types, without a conversion or alias.
- id: FR-370-AC-2
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'Each same-kind native pair follows the existing one-terminal-pair equality
    schedule: an equal pair yields `true`, an unequal pair yields `false`, and the
    planned pair count and charges equal the corresponding Boolean-leaf schedule.'
- id: FR-370-AC-3
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'A direct equality plan pairing UUID or Timestamp with another value kind
    returns `Refusal::CheckedInvariant { cause: ValueKindMismatch }`, while a direct
    mixed-kind `compare_keys` call returns `None`; neither returns a Boolean or a
    false equal key.'
- id: FR-370-AC-4
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: '`compare_keys` gives each native type a total key consistent with its
    equality relation and ordered by canonical payload ASCII bytes; Timestamp `"10"`
    precedes `"2"` and `"-1"` precedes `"-2"` in the key, independent of value construction
    or insertion order and without exposing a native ordering operator.'
- id: FR-370-AC-5
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Existing scalar and composite admission, equality, key comparison and accounting
    vectors retain their prior results.
- id: FR-370-AC-6
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: UUID values preserve exactly 16 network-order octets, including Nil, Max
    and non-RFC-version bit patterns; only the lowercase ASCII `8-4-4-4-12` caller
    payload admits, and uppercase, braces, URN form, altered grouping/separators,
    whitespace, length or nonhex characters refuse without repair or value.
- id: FR-370-AC-7
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Timestamp values preserve signed `i128` POSIX-epoch nanoseconds, including
    inclusive bounds `-170141183460469231731687303715884105728` and `170141183460469231731687303715884105727`
    and `-1`, `0`, `1` as one nanosecond before, at and after the epoch; only canonical
    signed base-10 caller payload admits, while either adjacent exterior count, `-0`,
    plus signs, leading zero, fraction, exponent, non-ASCII digits, calendar/offset
    text and whitespace refuse without repair or value.
- id: FR-370-AC-8
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Kernel constructors accept canonical UUID or Timestamp text supplied after
    the caller has validated its tagged wire envelope. Each malformed text returns
    `ConstructionRefusal` at `Component::Value` with its tabled cause, no value and
    no internal checked-invariant fault; canonical out-of-range decimal returns `TimestampOutOfDomain`
    after syntax checking. The kernel has no second parser-error type or UUID out-of-domain
    case.
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate
    preserve the exact value for fixed boundaries and reproducibly sampled inputs.
    Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated
    MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation.
    Compare fitting operations with independent checked i128 arithmetic; compare non-fitting
    results with independently authored decimal literals. Reject malformed/noncanonical
    spellings as be
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and
    arithmetic routes remain equal, identically ordered, identically hashed with the
    same hasher and identically formatted before and after BigInt access and cloning.
    A borrowed as_big reports the exact value throughout concurrent read-only accesses;
    all accesses to one retained materialization observe the same object. Clone/drop
    do not invalidate surviving permitted borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362
    schedule and counters on native, native-overflow and arbitrary-size cases. Test
    sufficient limits, independently calculated exact/one-less bit and work boundaries,
    denial injected at every named point, and outside-domain results. Compare all
    completed/refused/incomplete fields and admitted prefixes with the independent
    normative formulas. No representation transition manufactures a FunctionCall or
    other charge.
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native
    Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt
    construction/materialization. A safe external allocator observer around standalone
    Integer calls confirms no allocation on that native path, including clone after
    cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings
    and i128 MIN/MAX, compare independently prepared expected limbs, observe no stream
    allocation and confirm that initially empty caches remain empty. Observation of
    public metered calls separately iden
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'Public signatures and all existing Integer methods remain available, except
    that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating
    and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined
    by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their
    former plain &Integer parameters. Their existing nonzero results and infallible
    return types remain preserved; every other public signature and Integer method
    retains its existing preservation guarantee. Trait checks preserve Send/Sync and
    current panic-unwind traits. Default and '
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic
    mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second
    native ceiling, 16GiB memory ceiling, completion cover and counterexample checks.
    Healthy verification and both substantive falsifications are required. Existing
    all-four-operation, accounting/denial and refinement obligations remain required.
    A compiler error, MemoryExhausted, timeout, missing mutation execution or missing
    teardown is inconclusive and does not satisfy this criterion.
- id: FR-371-AC-7
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Private mechanism checks distinguish Small64 and Small128 construction,
    normalization and clone routes according to AD-003 without exposing a public tag
    API. Source inspection identifies storage matching before native-value extraction
    for Small64 Add/Subtract/Multiply/Negate, widening before ordinary arithmetic,
    Small128 result construction and no BigInt continuation inside those arms. Check
    all other inline pair classes, failed numeric-width guards, genuine i128 overflow
    and Large cancellation; a small Small128 result remains Small128 on chaining.
    Confirm both inline MIN magnitude streams and
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the authoritative membership rule for inclusive
    lower/upper equality, adjacent outsiders, negative and zero values, scale refusal,
    all rounding spellings and equivalent retained representations; neither representation
    is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true
    at the exact upper bound.
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "Production helper observations on admitted `B,S` fixtures stay within\
    \ the one-request `4 \xD7 L_B`-byte scratch envelope and `2 \xD7 (min(S,B)+2)\
    \ \xD7 L_B` limb-step bound, including a longest-pass endpoint fixture and a large\
    \ shift that terminates when quotient becomes zero. Controls exercise actual inline\
    \ and Large production paths, including zero, 32-bit limb crossings, i128 MIN/MAX\
    \ and signed magnitudes beyond i128. Stream traversal requests no allocation and\
    \ leaves cold inline caches empty; inline streams yield at most four least-significant-first\
    \ u32 limbs. Both stream mechanisms and all scalar s"
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Denying the actual scratch reservation reports its actual native request
    as typed allocation failure. A separate checked-capacity failure retains its own
    classification. Removing the allocation denial yields the same membership as a
    fresh call, with no partial accepted result after either refusal.
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation
    observed during the longest admitted division/comparison path returns the original
    cause before another limb step or result; cancellation observed immediately after
    successful reservation prevents scratch work. Neither failure is classified as
    nonmembership or work exhaustion.
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder
    and sign invariants, exact reservation boundary, checked layout conversions, allocation-free
    inline unsigned-magnitude stream and Large borrowed dependency iterator, absence
    of cache initialization or BigInt-image construction on those streams, and stated
    arithmetic polling gap. Its record distinguishes requested storage from allocator
    overhead and work latency from unbounded allocator elapsed time.
- id: FR-373-AC-1
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: The public crate-root `ieee_to_exact` has the exact direct signature in
    Behavior. `IeeeExactTarget` and its re-export are absent; no compatibility entry
    point or copied conversion algorithm remains. A non-Rational target cannot be
    supplied to this signature.
- id: FR-373-AC-2
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Binary32 `0x3fc00000` and binary64 `0x3ff8000000000000` complete as `3/2`
    with no loss in numerator domain `[-3,3]`, denominator domain `[1,2]`. Both signed
    zeros at both widths complete as `0/1`; only negative zero carries `NegativeZeroSign`,
    preserving FR-366's original binary32 control.
- id: FR-373-AC-3
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Quiet NaN, signaling NaN and positive/negative infinity at each width return
    exactly `Undefined(IeeeNotFinite)` after one operand charge, one work unit, source-width
    IntegerBits and one ValueOccurrences high-water unit, with zero result units and
    no intermediate or retain admission.
- id: FR-373-AC-4
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Converting `3/2` to numerator domain `[-1,1]`, denominator domain `[1,2]`,
    and separately to numerator domain `[-3,3]`, denominator domain `[1,1]`, returns
    `Refused(IeeeRationalOutOfDomain { target })` with the corresponding exact target.
    Two work units are consumed; no result unit or retain charge is admitted.
- id: FR-373-AC-5
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Successful finite conversion admits exactly operands, intermediate and
    retain in that order, consuming three work units and one result unit. Ordinary
    work limits 0, 1 and 2 deny the respective next point with exact limit, consumed
    and next-charge fields. Injected occurrence-1 denials at each point report WorkUnits
    with limit equal to pre-charge consumed work and next charge 1. Result limit 0
    denies retain atomically after two work units. Smallest positive subnormals at
    either width under IntegerBits equal to the source width deny intermediate with
    next-charge sizes 150 and 1075 respectively. E
- id: FR-373-AC-6
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: For Requested and Deadline, pre-cancelling the original handle attached
    to the supplied meter returns WorkUnits incomplete at operands with limit = consumed
    = 0 and next charge 1. Deterministic cancellation at the original handle's second
    and third meter polls returns the same form at intermediate and retain with limit
    = consumed = 1 and 2 respectively. The original handle records the actual cause,
    only the prior charge prefix is admitted and no result is retained. A live handle
    completes the same fixture.
- id: FR-373-AC-7
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'Precharge the supplied meter once at FunctionCall with work 2, result
    1, IntegerBits 80, ValueOccurrences 5 and TextScalars 7. Direct conversion of
    either width''s `3/2` under sufficient limits ends with work 5, results 2 and
    admission count 4, preserving all three high-water values and the FunctionCall
    log prefix. Independently repeat with work limit 4, result limit 1, an occurrence-1
    retain injection and original-handle cancellation at the third conversion poll.
    Each stops at retain with work 4, results 1 and admission count 3; ordinary records
    report respectively WorkUnits(limit=consumed=4) '
- id: FR-374-AC-1
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Admission of zero constructed by primitive, canonical parse and zero-producing
    arithmetic returns exactly `Err(ZeroDivisor)` and no token. Positive and negative
    values, including values outside i128, admit; `get` borrows the original integer,
    with no Integer/BigInt clone or materialization for admission. The public exports
    and typed unit error match the stated contract.
- id: FR-374-AC-2
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: An external public-API caller can reuse one borrowed copyable token across
    all three helpers. A plain `&Integer` cannot satisfy any of the three divisor
    parameters. External field construction, writable access and a token outliving
    its referent are rejected by the public type/lifetime boundary; no Default or
    unchecked public constructor permits zero bypass.
- id: FR-374-AC-3
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)`
    return respectively truncating `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and floor
    `(3,1)`, `(-4,-1)`, `(-4,1)`, `(3,-1)`. A zero dividend with either divisor sign
    returns `(0,0)` in both helpers. Dividing `680564733841876926926749214863536422912`
    by either `2` or `-2` returns respectively quotient `340282366920938463463374607431768211456`
    or its negative and remainder zero, without narrowing.
- id: FR-374-AC-4
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct exact_div on every sign combination of dividend 6 and divisor 2
    returns the signed quotient 3; zero dividend with either divisor sign returns
    zero. The beyond-i128 divisible literals in AC-3 return the same signed quotient.
    The nondivisible controls `5/2 = 2` and `-5/2 = -2` retain the prior behavior,
    without divisibility rejection or a fallible helper result.
- id: FR-374-AC-5
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Inspection accounts for every exact-owned production caller in Rational,
    Decimal and division. Canonical callers establish the documented nonzero invariant
    without forced Integer/BigInt clones, new public errors in existing canonical
    infallible APIs, caller-divisor admission unwrap/expect, unchecked public admission
    or fabricated numeric fallback. Any private token construction has an explicit
    owning nonzero invariant.
- id: FR-374-AC-6
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: With sufficient real meters, zero divisor under all three divide profiles
    and both members, plus public modulo, returns `Undefined(DivisionByZero)` after
    exactly the operation's operands charge, with no arithmetic/domain/result-retain
    or admission charge. Denying operands returns the existing Incomplete first. Nonzero
    success and exposed-member refusal controls retain the existing laws, charge requests,
    ordered prefixes, counters and outcome fields; public metered signatures remain
    unchanged.
- id: FR-375-AC-7
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: No cross-crate Kani contract, stub, or assumed postcondition is treated
    as checked unless a focused tool control demonstrates both a passing and a deliberately
    failing dependency-contract case through the actual RT-to-quire-exact dependency
    boundary. Until that evidence exists, the proof status is unsupported at that
    boundary and the real callee remains in the outer CBMC graph.
- id: FR-375-AC-9
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: On equal and first-head-unequal bounded kernel operands, `planned_equality_with_plan`
    returns the Boolean relation and 17 planned pair events from the same formed plan;
    `plan_equality` agrees on that count, and the admitted meter log contains 17 pair
    charges on success.
- id: FR-375-AC-10
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: When `Meter::charge` refuses the 17th pair, the result-bearing kernel operation
    returns the typed stop with exactly the prior 16 pair charges admitted and no
    result-retain charge; it does not report all 17 planned pairs as admitted. A result-retain
    refusal after all 17 pair charges retains that admitted prefix.
- id: FR-376-AC-1
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`plan_equality`, `planned_equality` and `planned_equality_with_plan` on
    two composites whose declaration keys differ return `false`, with a pair count
    of 1, and no `Refusal`. The result is the same in either operand order, for operands
    with equal slots, with different slots and with different slot counts, and for
    a pair of empty-slot composites.'
- id: FR-376-AC-2
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: On that pair, `planned_equality` admits `equality.plan-form`, `equality.plan`,
    exactly one `equality.pair` and `equality.result-retain`, in that order, with
    the plan-form charge computed from the two operands' occurrence counts as FR-375
    states.
- id: FR-376-AC-3
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite pair of different declarations whose slots hold `Reference`
    values of different universes returns `false` and does not report `foreign_reference`,
    because the slots are not visited. As a control, two composites of one declaration
    whose slots hold `Reference` values of those two universes still report `foreign_reference`.
- id: FR-376-AC-4
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: Two composites with equal declaration keys and equal slots give `true`.
    Equal declaration keys with a differing slot give `false`. The pair counts are
    those of the same plan before this requirement.
- id: FR-376-AC-5
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A pair of different declarations that sits as the payload of two present
    `Option`s, in a one-slot record, as the element of two one-element `Sequence`,
    `Set` and `Bag`, gives `false`, with pair counts of 2, 2 and 2, and no refusal.
- id: FR-376-AC-6
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, `Option`, `Collection` or `Reference`,
    in either order, still refuses `ValueKindMismatch`. Two composites of one declaration
    key and different slot counts still refuse `ValueKindMismatch`. Two populations
    still refuse `PopulationPair`. Neither refusal returns a completed Boolean.
- id: FR-376-AC-7
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`member_equal` of a composite candidate against a member of a different
    declaration returns `false`, charging `collection.member-walk` then `collection.member-test`
    with one pair.'
- id: FR-376-AC-8
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`form` over an `OrderedSet` type whose element type is a declaration P,
    with occurrences of one composite of declaration C then one of P, completes with
    the two members in that supply order, charging one membership comparison; the
    occurrences P then C give P then C; C, P, C give C then P. None refuses `ValueKindMismatch`.
    An `OrderedSet` is not sorted by the canonical key, so it needs this requirement
    and not FR-377.'
- id: FR-376-AC-9
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A union member whose one position holds a `P` composite in one value and
    a `C` composite in the other (also `C` against `G`), in both orders, gives `false`
    with a pair count of 2 and no refusal; reference slots of different universes
    inside the skipped composites do not report `foreign_reference`; a two-position
    payload keeps its second pair after a mismatch in the first (3 pairs); the same
    pair as the payload of two `Option`s gives `false` with 3 pairs; equal declarations
    walk the slots; a union against an `Integer`, two unions of one member with different
    payload lengths and two unions of di
- id: FR-377-AC-1
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: '`compare_keys(a, b)` for composites of declaration keys `[1; 32]` and
    `[2; 32]` is `Some(Less)` and `compare_keys(b, a)` is `Some(Greater)`, whatever
    their slots: slots equal, slots that would order the other way, and different
    slot counts.'
- id: FR-377-AC-2
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: Declaration keys that differ only in the last byte (`[1; 31]` followed
    by `0`, against the same followed by `1`) order by that byte; keys that differ
    in the first byte order by the first byte even when the later bytes order the
    other way.
- id: FR-377-AC-3
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: Two composites of one declaration key keep the slot order of the rule before
    this requirement; `compare_keys(x, x)` is `Some(Equal)`; two composites of different
    declarations are never `Some(Equal)`, even with identical slots.
- id: FR-377-AC-4
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A `Set` and a `Bag` of element type `Composite(P)` formed through `form`
    from one composite of each of two declarations, in each of the two supply orders,
    complete and visit the member of the smaller declaration key first. `form_grouped`
    over the same two members gives the same order. A `Bag` formed through `form`
    from two equal members of each declaration visits the two groups in that order.
- id: FR-377-AC-5
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'Equal keys occur exactly when values are equal: for a set of composites
    of two declarations with equal and unequal slots, every pair has `compare_keys
    == Some(Equal)` exactly when `planned_equality` gives `true`.'
- id: FR-377-AC-6
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, two composites of one declaration key
    and different slot counts, and a pair of `Float` members each return `None` from
    `compare_keys`. As the two members of a `Set`, each refuses `CanonicalKeyUnavailable`
    through `form_grouped` and `ValueKindMismatch` through `form`, which refuses at
    membership comparison before any ordering.
- id: FR-377-AC-7
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: The key order of a pair of different declarations holds as the payload
    of two `Option`s, in a record slot and as an element of two `Sequence` values,
    ordering the enclosing values by the same declaration-key rule.
- id: FR-377-AC-8
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: '`compare_keys` on unions of one member that hold a `P`, `C` or `G` composite
    at the payload position returns `Some(Less)` for `P` against `C` and `C` against
    `G`, whatever the slots, and `Some(Equal)` agrees with `planned_equality` on all
    nine ordered pairs of the three; an `Alpha` union holding `C` orders before a
    `Zulu` union holding `P`; `compare_keys` returns `None` for unions of different
    declarations and for one member with different payload lengths; `form` of a `Set`
    over `C`, `P`, `C` unions (and over `P`, `C`, `P`) keeps the `P` union then the
    `C` union; `form` of an `OrderedSet` keep'
- id: FR-378-AC-1
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` and `form_grouped` over an element type `Composite(P)` and one
    `P` member and one `C` member complete for `Sequence`, `Set`, `Bag` and `OrderedSet`,
    keep both members unchanged (declaration key and slots), and for `Set` and `OrderedSet`
    hold two members even with equal shared slots. No collection call tests an occurrence
    against the element type.'
- id: FR-378-AC-2
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` over a `Set` of those two members charges, in order, `collection.member-walk`,
    `collection.member-test` with one pair, `collection.bound` and `collection.result-retain`,
    and no `collection.element`.'
- id: FR-378-AC-3
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted(Composite(P), Some(c))` returns an `Option` whose `payload_type()`
    is `Composite(P)` and whose payload has declaration `C` and its slots unchanged.'
- id: FR-378-AC-4
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted_slots` of a declaration whose declared field type is `Composite(P)`
    and whose slot holds a `C` composite keeps the slot unchanged; `retain_composite`
    charges `composite.result-retain` once; the composite is unequal to one with the
    same declaration and a `P` composite in that slot (FR-376).'
- id: FR-378-AC-5
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: 'For the same `C` composite at element type `Composite(P)`: `construct_collection`
    refuses `CollectionElementNotAdmitted`; `form_collection` refuses `Element(index)`
    `TypeMismatch`; `ValueType::admits` is `false`; `OptionValue::present` refuses
    `Payload` `TypeMismatch`; `evaluate_record` with that field refuses `DeferredResultNotAdmitted`.
    A `P` composite is accepted by each.'
- id: FR-378-AC-6
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: A collection of element type `Composite(C)` is not admitted by `ValueType::admits`
    at the collection type of element type `Composite(P)`, nor the reverse. A stored
    collection clones, compares equal to an identical build (`true`), compares unequal
    to one that differs in a member's declaration (`false`), and drops on the iterative
    paths of any `Value`.
- id: FR-378-AC-7
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: The rustdoc of the seven items in Behavior 8 carries the stated text.
- id: FR-378-AC-8
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`UnionValue::from_admitted` of a position typed `Composite(P)` and a `C`
    composite keeps the payload unchanged (declaration `C`, slots unchanged). For
    that `C` composite `union` refuses `Position(0)` `TypeMismatch` and `evaluate_union`
    refuses `DeferredResultNotAdmitted`; a `P` composite is accepted by each.'
- id: FR-379-AC-1
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A nullary member has an occurrence count of 1 and a two-position member
    over two integers has 3. The value reports its declaration, `VariantId`, identifier
    and payload in position order. `ValueType::Composite` of its declaration admits
    it, `Composite` of another declaration and `Integer` do not.
- id: FR-379-AC-2
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`union` refuses an arity of 0 against 1 supplied and 2 against 1 supplied
    with `Value` `WrongArity`, and a `Boolean` at an `Integer` position with `Position(0)`
    `TypeMismatch`.'
- id: FR-379-AC-3
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`evaluate_union` returns `Undefined` for the first argument that is undefined,
    runs no later argument and admits no charge. A completed `Boolean` at an `Integer`
    position refuses `DeferredResultNotAdmitted`. A completed evaluation runs the
    arguments in order and admits one `composite.result-retain` with the occurrence
    count of the result, and a retain limit of one fewer returns `Incomplete` naming
    the charge point, the limit kind and the limit.'
- id: FR-379-AC-4
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Unions of one declaration and member with equal payloads are equal, with
    differing payload positions or members are not, and the pair count is one plus
    the payload pairs.
- id: FR-379-AC-5
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A union against a union of another declaration, an enum member and a composite
    refuses `ValueKindMismatch` in `plan_equality` and gives `None` from `compare_keys`.
    One member with payload lengths 1 and 2 refuses `ValueKindMismatch` and gives
    `None`.
- id: FR-379-AC-6
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Members `Alpha` and `Zulu` order by identifier whatever their digests and
    payloads. One member orders by the payload positions in order. Declaration order
    and digest order do not reach the order. Equal identifiers with different `VariantId`s
    give `None`.
- id: FR-379-AC-7
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`Set`, `Bag`, `OrderedSet` and `Sequence` over four unions, from a supply
    order and its reverse, hold the stated elements. A union with an IEEE payload
    refuses `CanonicalKeyUnavailable` through `form_grouped` for a `Set` and is kept
    by a `Sequence`.'
- id: FR-379-AC-8
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Clone, equality, key comparison, `Debug` and drop of a union nested ten
    thousand levels complete on a 512 KiB stack, a mixed union, composite and collection
    nest does the same, storage grows linearly with depth, and a shared child counts
    each occurrence.
- id: FR-379-AC-9
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A present `Option` of declared type `Composite(K)` over a union of declaration
    `K` is admitted, two such `Option` values with equal unions are equal and with
    different members are not, and they compare by the key of the union.
- id: FR-380-AC-1
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: With an authenticated B, Integer and reduced Rational membership agree
    with the inclusive existing rule at both endpoints, one outside each endpoint
    and a denominator boundary, without mutating operands.
- id: FR-380-AC-2
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Exact-B value and every exact-B endpoint are accepted for comparison; one-over-B
    in each of the three Integer or six Rational operand positions returns typed unsupported
    size naming that position and measured bits before any comparison. A value-only
    guard mutant fails an endpoint case.
- id: FR-380-AC-3
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Observed production comparison steps do not exceed 2L for Integer or 4L
    for Rational, with fixed-width per-step work and constant auxiliary storage on
    the allocation-free path; source inspection confirms magnitude checks and borrowed
    limb iteration do not hide input-sized work/allocation.
- id: FR-380-AC-4
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: A pre-cancelled original handle prevents input-sized work. Deterministically
    cancelling during a longest admitted comparison returns that cause before the
    next step/result; a fresh-handle mutant fails.
- id: FR-380-AC-5
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Inspection identifies whether the production comparison actually performs
    a fallible native conversion or storage request. For each reachable boundary,
    a denied checked conversion is typed capacity/resource refusal and an actual reservation
    denial carries its measured request; neither becomes false, unsupported size or
    traversal Limit, and retry after removing allocation denial agrees with a fresh
    run. When neither boundary exists, no artificial capacity/allocator refusal fixture
    or ceiling is required.
- id: FR-380-AC-7
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: 'Inspection of both direct exact-kernel `IntegerInterval::try_contains_bounded`
    and `RationalDomain::try_contains_bounded` signatures verifies that authenticated
    finite B is a required `magnitude_bits: u64` parameter, applied to the value and
    every endpoint, and neither helper selects a default B. A call omitting B is not
    part of either helper''s API.'
- id: FR-381-AC-1
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: With authenticated B, equal and unequal Integer intervals and Rational
    domains return the same Boolean equality as independent endpoint arithmetic, including
    negative, zero, and denominator endpoints, without mutation. Equal spellings or
    identity of borrowed objects do not substitute for endpoint equality.
- id: FR-381-AC-2
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: At B >= 1, every one of the four Integer and eight Rational endpoint positions
    completes at exact B and returns typed unsupported size when made one bit over
    B, before endpoint comparison. Two oversized endpoints report the first in the
    stated expected-before-actual order. At B = 0, even zero (one magnitude bit) is
    refused at the first expected endpoint.
- id: FR-381-AC-3
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: Observed production comparisons use no more than 2L Integer or 4L Rational
    limb-pair steps, fixed-width work per step, and the measured temporary-storage
    bound, including inline/Large and 32-bit limb boundaries. Inspection finds no
    input-sized clone or unbounded fallback.
- id: FR-381-AC-4
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: A pre-cancelled original handle stops before bound validation, including
    when the first endpoint exceeds B; cancellation on a late observed limb step stops
    before the next step or equality result with the original cause.
- id: FR-381-AC-5
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: Source inspection establishes whether production performs native-size conversion
    or allocation. For each reachable boundary, an executed denial/retry test establishes
    the typed measured request and retry result. An allocation-free route with no
    failing conversion has no synthetic resource fixture.
- id: FR-382-AC-1
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: Equal distinct Decimal types complete `true`; a difference in either signed
    endpoint, either scale or rounding mode completes `false` under authenticated
    `B`. Endpoint equality is mathematical integer equality. Borrowed inputs remain
    unchanged.
- id: FR-382-AC-2
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: At `B >= 1`, every endpoint position completes at exact `B` and returns
    typed `UnsupportedSize` with the correct position, measured `B+1` bits and `B`
    when one bit over. Two oversized endpoints report the first in expected-lower,
    expected-upper, actual-lower, actual-upper order, even if an earlier retained
    field differs. At `B = 0`, zero is refused at expected lower with measured one
    bit.
- id: FR-382-AC-3
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: Observed production comparisons at inline/Large and 32-bit limb boundaries
    perform at most `2 ceil(B/32)` limb-pair steps of fixed-width work and no input-sized
    clone or allocation; auxiliary storage is constant. Source inspection finds no
    hidden unbounded fallback or reachable storage/capacity failure.
- id: FR-382-AC-4
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: A pre-cancelled original handle stops before bound validation, including
    when expected lower exceeds `B`; cancellation observed after a late limb step
    stops before another step or Boolean publication with the original cause. A fresh
    handle completes the same operands.
- id: FR-383-AC-1
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: '`Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`,
    `cancel()` is `Some(&Cancel)` equal by shared-state identity to `handle`, and
    unequal to an independently created handle. Replacing the handle reports only
    the replacement.'
- id: FR-383-AC-2
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: 'Cancel the handle obtained from `meter.cancel()` with `Requested`, then
    charge the meter under sufficient limits: the charge is denied at its named point,
    that handle records `tripped() = Some(Requested)`, and all accounting state stays
    unchanged. On a separate fresh meter, cancel an independent handle while its installed
    handle remains live: an otherwise identical charge is admitted.'
- id: FR-383-AC-3
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The accessor's implementation borrows the existing optional handle without
    cloning it or constructing a second `Meter` or `Counters`; calling it changes
    no meter accounting or injection state.
- id: FR-383-AC-4
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles
    in the crate's `no_std` + `alloc` build.
- id: FR-384-AC-1
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Admission accepts valid finite Add/Subtract/Multiply/Negate domains and
    rejects reversed ranges, nonrepresentable endpoints and overflowing intermediate
    result intervals with distinct typed reasons. Boundaries cover native MIN/MAX,
    zero, -1, singleton domains and multiplication sign combinations. A narrowing
    final domain does not conceal an unrepresentable intermediate. Public clients
    cannot forge or widen certificates.
- id: FR-384-AC-2
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: A machine-checked refinement against an independently expressed mathematical
    abstraction of the actual BigInt semantics covers every admitted native payload/operation
    and its abstraction to mathematical Integer/BigInt. Independent BigInt differential
    tests exercise extrema, cancellation, products and negation plus reproducibly
    sampled accepted domains; deliberate arithmetic and admission-bound mutants falsify
    the relevant property. Evidence states exactly which obligation each method establishes.
- id: FR-384-AC-3
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Fresh and cumulative real Meter comparisons against the normative formulas
    and general exact calls agree on completed values, domain refusal, cancellation,
    every named injected denial, all counters, admitted prefix and complete incomplete
    fields at sufficient, exact and one-less limits. Every operation and Boolean/comparison
    consumer used by the genuine profiles is exercised; arithmetic equivalence alone
    does not satisfy accounting refinement.
- id: FR-384-AC-4
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Source inspection and ordinary generated-client execution demonstrate native
    carriers persist through a multi-operation expression without successful-path
    BigInt/cache/general Integer construction. Input/output conversions are identified
    separately. Default and test-support checks, required no_std target and existing
    borrowed as_big regression checks pass without local unsafe or verification-only
    implementation.
- id: FR-384-AC-5
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Genuine CG TC-027 healthy, TC-027 bound mutant, TC-025 arithmetic mutant,
    IR-917 clause implication and IR-948 V2 contract profiles execute the reviewed
    production admitted API over their original inputs, domains and assertions. Healthy
    cases verify with their final completion covers at 16 GiB; each required substantive
    mutant falsifies with original-domain native playback. The IR-948 impossible-precondition
    control must complete as CoverUnsatisfied at the same ceiling, not resource exhaustion
    or a vacuous Verified. Unwind3, Cadical and existing 600-second ceilings remain
    unchanged where speci
```
