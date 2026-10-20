---
id: SR-14161
title: IR-935 gap-analysis of quire-exact PR 64
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact; whole_repo; all declared spec criteria, src/, tests/;
  IR-935
review_set: subset
---

## Summary

A genuine whole-repository planless computed matrix and reverse ownership/stub inventory found four inherited untagged criteria. All 188 criterion statements and their computed binder census are recorded; the Unicode correction adds no new locally owned criterion gap.

Ticket: IR-935. Frozen head: c77706ba1e3cbb68085ab583c977ee1a4a4f2db3; base: aa63308958b7fa3a08142a24ce3694e94c28d62b. Runtime model: gpt-6.1-sol; run: ir935-655382c9-d124-4240-bff0-0a331bc7603c. Repo visibility is public; review comments are private Linear only. No source, branch or specification changes by reviewer.

## Verdict

**FAIL** — Whole-repository strict traceability is not clean; the four inherited gaps must remain explicit. The focused Unicode correction is not a proof delivery.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The computed matrix has no test binding for the explicitly PLANNED Meter proof obligation; it remains deferred to IR-844 and is inherited unchanged from the base. Record that deferred proof gap rather than crediting Unicode tests as evidence. | spec/functional/FR-359-cumulative-meter-boundary.md:37 |
| FND-002 | medium | The computed matrix has no binding or current external receipt for the five consumer Kani obligations; this inherited IR-705/IR-17 gap requires the genuine consumer qualification and cannot be closed by owner Unicode tests. | spec/functional/FR-359-cumulative-meter-boundary.md:38 |
| FND-003 | medium | The computed matrix has no binding or delivered healthy/mutant consumer qualification for the inherited representation-refinement consumer obligation. Keep its IR-705/IR-17 qualification outstanding; owner normalization tests provide no proof credit. | spec/functional/FR-371-bounded-integer-representation-refinement.md:84 |
| FND-004 | low | The computed matrix reports the existing no_std signature criterion as untagged because its documented make build-no-std gate has no source Trace binder. This inherited structural evidence gap is distinct from missing implementation; reconcile it in the cancellation-accessor assurance scope rather than misreporting matrix strict as passing. | spec/functional/FR-383-meter-cancellation-handle-access.md:50 |

## Coverage

Plan completion: not assessed

Repository scope: agent-ix/quire-exact only, spec/ for declared requirements, src/ and tests/ for implementation. No spec/spec.md root artifact exists, so no invented root relationship is emitted. The module set declares traceability; no grep fallback was substituted. quire matrix --scope . --format json and --strict actually ran: 33 owner FR documents, 188 criteria, 166 tagged, 18 method-without-symbol (declared Inspection), 4 untagged, 0 ignored-only, 248 criterion-to-test binder records covering 214 distinct source declarations. The same matrix over a git-archive of the exact base has identical criterion statements and statuses; all four untagged gaps predate this change. FR-359-AC-9 is explicitly planned IR-844 work; FR-359-AC-10 and FR-371-AC-6 are genuine external consumer proof obligations; FR-383-AC-4 documents its no_std build-gate-only evidence. Each is reported separately without pretending planned proofs or a build-gate bookkeeping limitation are Unicode regressions.

Whole reverse inventory: all 27 src Rust files, 17 integration test files and the consumer fixture were inventoried by module ownership, curated facade, documentation, trace declarations and source/test stub scan. Existing kernel value semantics delegate to QSpec as src/lib.rs states; local allocation, accounting, numeric membership, equality and cancellation refinements map to the declared FR set. Foundational identity/provenance carriers are part of the documented kernel facade, not new unspecified Unicode behavior. No confirmed unowned behavior, concrete source stub, test stub, unconditional skip, TODO/todo!/unimplemented! implementation, or coverage inflation was found by this mechanical inventory (0 confirmed each). No coverage percentage was available or asserted. Six coverage suspicions were inspected at their exact symbols: the identity assertion macro is invoked for all five types; decimal tie expectations exhaustively match both Some and None; checked-i128 guards bound the oracle to its valid native domain and have a separate beyond-i128 test. The three helper-similarity signals compare a trivial NodeKey fixture constructor across tests, not a copied production algorithm or fake implementation oracle. These signals do not demonstrate hollow evidence. quire coverage additionally reports 86 untracked-symbol records and 28 unmatched tags; these include existing QSpec cross-owner tags, TC references and parser false positives such as UTF-8. They are not 86 demonstrated unspecified behaviors. The new bare external FR-141 tag is separately examined and reported by code-review FND-002; do not copy an upstream requirement into this repo to satisfy owner-only lookup.

Binding records marked correct assert only that the engine resolved an existing local criterion to the exact recorded test declaration; they do not certify every assertion or rerun any test. Optional repository-wide semantic intent review was not requested and was skipped. Focused normalization correctness was inspected in the code-review lane. No plan was discovered, selected or authored. No applicable AssuranceProfile exists under this owner spec tree. Existing architecture decision artifacts are context, not new normative changes. Spec-review/sub-analyses are not applicable. Tool diagnostics for duplicate ambient archetypes/inverse edges were emitted; matrix JSON was usable, while strict correctly exited 1 for the four gaps.

## Verification and limitations

Reviewer performed static Rust inspection and Quire only. All Cargo/rustc/rustfmt/Kani execution is UNRUN by this reviewer under the no-heavy grant. The author raw logs show old-range fresh resolution selecting 0.1.26 and failing E0080, then new-range success with the retained fresh lock selecting 0.1.25. Author fmt/lint covers default, test-support and bare-metal no_std; author text/native units and profile-mismatch boundary pass in both feature modes. Aggregate make ci, deny, docs and full test suites await the lead after latest artifact custody. No CG native, negative-feature, Kani, mutation, proof or full IR-935 acceptance credit is claimed.

## Review data

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
    in the identity of a variant but not in ran
  lines: '26'
  artifact_type: FR criterion
- id: FR-088-AC-12
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: A declared unit `UnitId` and a compound unit `UnitId` are unequal when their domains or bytes differ.
    A declared unit's `UnitId` carries its node key under the `quire.checked-semantic-node/v1` label, and a compound
    unit's carries its digest under the `quire.value.compound-unit/v1` label. Two `UnitId`s of different domains
    never compare equal, whatever their bytes.
  lines: '27'
  artifact_type: FR criterion
- id: FR-089-AC-2
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: The kernel `Value::Population` variant's payload type is `PopulationId`; no kernel source file imports
    a population binding or any other caller's model type to define, construct or match this variant, and the crate's
    `[dependencies]` name no crate that owns one.
  lines: '26'
  artifact_type: FR criterion
- id: FR-089-AC-6
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: 'Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`, kernel `ValueType::admits`
    returns false; given two `Value::Population` operands, kernel `equality::plan_pairs` returns `Err(Refusal::CheckedInvariant
    { cause: PopulationPair })` and kernel `key::compare_keys` returns `None`.'
  lines: '27'
  artifact_type: FR criterion
- id: FR-090-AC-7
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Refusal` has no variant whose payload is a wrong-snapshot cause.'
  lines: '25'
  artifact_type: FR criterion
- id: FR-090-AC-11
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Undefined` has no `PreconditionFalse` variant.'
  lines: '26'
  artifact_type: FR criterion
- id: FR-090-AC-12
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Undefined` has no `AbsentKey` variant.'
  lines: '27'
  artifact_type: FR criterion
- id: FR-090-AC-13
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Refusal::CheckedInvariant` carries only FR-369''s closed typed cause; it has no arbitrary caller message,
    QSL model/checker type, catalog code or generic caller-cause escape hatch.'
  lines: '28'
  artifact_type: FR criterion
- id: FR-096-AC-8
  path: spec/functional/FR-096-kernel-refusal-code-and-cause.md
  role: examined
  excerpt: 'Each catalogued kernel refusal below returns the code and cause spelling named for it from `Refusal::code()`
    and `Refusal::cause()`, and every FR-369 `CheckedInvariant { cause }` returns neither. `InexactDecimal`: `inexact_decimal`,
    `nonzero-discarded-digit`. `DecimalOutOfDomain`, `ModuloOutOfDomain`, `TextLengthOutOfDomain`, `IntegerOutOfDomain`,
    `RationalOutOfDomain`, `IeeeRationalOutOfDomain`: `<name in snake case>`, `outside-domain`. `DivisionOutOfDomain`:
    `division_out_of_domain` with `quotient-outside-domain` for the quotient member or `remainder-outside-domain`
    for the remaind'
  lines: '25'
  artifact_type: FR criterion
- id: FR-097-AC-7
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: An unbounded collection type admits a collection value of any size with no cardinality refusal; it still
    charges `collection.bound`, and stops with `Incomplete` at that charge point only when the caller's meter runs
    out. `K<T>` and `K<T>[0, u64::MAX]` are different types.
  lines: '27'
  artifact_type: FR criterion
- id: FR-097-AC-8
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: A caller outside the crate can call `CardinalityBound::violation(count)` and receive the existing `Option<BoundViolation>`
    type. For `[2, 4]`, counts 1, 2, 3, 4 and 5 return below-minimum, none, none, none and above-maximum respectively;
    singleton `[2, 2]` admits only 2. `[0, u64::MAX]` reports no violation at either extreme, `[0, 0]` rejects 1
    above, and `[u64::MAX, u64::MAX]` rejects `u64::MAX - 1` below. The query is pure and unmetered; for `[2, 4]`,
    collection admission agrees with its decisions at counts 1 through 5.
  lines: '28'
  artifact_type: FR criterion
- id: FR-262-AC-2
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s around `Boolean` clones,
    compares equal to its clone, hashes equal to its clone, formats for debug and drops. A `Value` of the same depth
    compares equal to its clone, and it and its clone drop.
  lines: '26'
  artifact_type: FR criterion
- id: FR-262-AC-3
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: A `Value` nested N levels deep, alternating `Option` and `Sequence` levels around an `Integer` with each
    level declared with the type of its depth, holds at least N - 10 and at most 2N distinct `Option` and `Collection`
    type nodes, and the same value 2N levels deep holds at most twice the nodes of the N-level value plus 2. On
    a thread with a 512 KiB stack, such a value 100,000 levels deep and its 100,000-deep type are built, the type
    admits the value, and the type clones, compares equal to its clone, hashes equal to its clone, formats for debug
    with balanced brackets and drops, as do
  lines: '27'
  artifact_type: FR criterion
- id: FR-357-AC-1
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a zero divisor under any profile and either member, `divide` returns `Undefined(DivisionByZero)`.
  lines: '31'
  artifact_type: FR criterion
- id: FR-357-AC-2
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `10 div y` over `1..=10` at `y = 5`, `divide` of the quotient returns 2; given `x rem -1` over
    `-10..=5` at `x = -10`, `divide` of the remainder returns 0.
  lines: '32'
  artifact_type: FR criterion
- id: FR-357-AC-3
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a quotient outside the domain, `divide` refuses with cause `quotient-outside-domain`; given a remainder
    outside the domain, it refuses with `remainder-outside-domain`.
  lines: '33'
  artifact_type: FR criterion
- id: FR-357-AC-4
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given -7 and 2, truncating returns (-3, -1), floor (-4, 1) and Euclidean (-4, 1); given -7 and -2, truncating
    returns (3, -1), floor (3, -1) and Euclidean (4, 1).
  lines: '34'
  artifact_type: FR criterion
- id: FR-357-AC-5
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `i64::MIN` and -1 under any profile, `divide` of the quotient returns 2^63 in a domain that holds
    it. In a signed 64-bit domain the quotient refuses with `quotient-outside-domain` and the remainder completes
    with 0.
  lines: '35'
  artifact_type: FR criterion
- id: FR-357-AC-6
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a successful `div` or `rem`, the meter admits `integer-division.operands`, `integer-division.arithmetic`,
    `integer-division.domain` (one value occurrence) and `integer-division.result-retain` (one result unit), in
    that order. A refusal admits no `result-retain`, a zero divisor admits only operands, and a result limit of
    0 reports `integer-division.result-retain` as incomplete.
  lines: '36'
  artifact_type: FR criterion
- id: FR-357-AC-7
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: For dividend/divisor pairs `(7, 3)`, `(7, -3)`, `(-7, 3)` and `(-7, -3)`, public `modulo` in the mathematical
    domain completes to 1, 1, 2 and 2 respectively. Running a `divide` under each of Truncating, Floor and Euclidean
    before each `modulo` call does not change those results; `modulo` has no profile argument.
  lines: '37'
  artifact_type: FR criterion
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit P charge under an ordinary
    limit, then admit P once and an unrelated point once. The next P charge returns `Incomplete` at P with the work
    counter as its limit kind, proving the ordinary refusal did not advance the named occurrence; all consumed counters,
    admission count, and test-support log equal their values immediately before each refusal. A subsequent P charge
    succeeds and updates ordinary accounting and the log exactly once.
  lines: '58'
  artifact_type: FR criterion
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan` returns `Incomplete`
    at `equality.plan` without changing any consumed counter, admission count, or test-support log; a subsequent
    valid `charge_plan` succeeds and records its ordinary plan charge.
  lines: '59'
  artifact_type: FR criterion
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction with zero is rejected
    by that type, while a nonzero occurrence constructs a denial.
  lines: '60'
  artifact_type: FR criterion
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor returns all 4096 records with
    their points in order and `charge_log_truncated()` is false. On the 4097th admission the slice still contains
    exactly that prefix and the flag becomes true.
  lines: '61'
  artifact_type: FR criterion
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()` and consumed work units reflect
    all 4097 admissions, including the one omitted from the diagnostic log.
  lines: '62'
  artifact_type: FR criterion
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge log and its `Counters` accounting
    state remains fixed-size and heap-free.
  lines: '63'
  artifact_type: FR criterion
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one result unit. The log retains
    the first 4096 points and reports truncation, while `admission_count()`, consumed work and consumed results
    each equal 4097.
  lines: '64'
  artifact_type: FR criterion
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`, admit 4097 one-work-unit charges
    and request one more work unit. The charge returns `Incomplete` at its requested point with `WorkUnits`, `limit
    = consumed = 4097`, `next_charge = 1`; every counter, admission count, prefix log and truncation flag are unchanged.
  lines: '65'
  artifact_type: FR criterion
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`, admit 4097 charges each consuming
    one result unit, then request one more result unit. The charge returns `Incomplete` with `ResultUnits`, `limit
    = consumed = 4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation flag remain
    unchanged, including the work counter.
  lines: '66'
  artifact_type: FR criterion
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial for one named point. A charge
    at that point returns `Incomplete` with `WorkUnits` and leaves all counters, admission count, prefix log and
    truncation flag unchanged. Retrying that charge under sufficient ordinary limits succeeds once, increments work
    and admission count once, and does not extend the 4096-entry log.
  lines: '67'
  artifact_type: FR criterion
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'After admitting two work units, inject occurrence 1 at point P and attempt a three-work-unit `charge`
    at P under otherwise sufficient limits whose configured work limits are 10 and 100 on separate meters. Both
    refusals report exactly `limit_kind = WorkUnits`, `charge_point = P`, `limit = consumed = 2`, and `next_charge
    = 3`, independently of either configured limit. Repeat with an injected `equality.plan` denial, `pairs = 2`,
    the same pre-consumed work and limits, and sufficient value-occurrence and result limits: both `charge_plan`
    refusals report `WorkUnits`, `charge_point = equali'
  lines: '68'
  artifact_type: FR criterion
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'With an occurrence-1 injected denial selected, make the same `charge` independently fail an ordinary
    semantic-size counter and make a `charge_plan` with `pairs = 2` independently fail its work reservation under
    a configured work limit of 3 after two admitted work units, with all other limits sufficient. In each case,
    the injected `WorkUnits` record is returned before the ordinary refusal: `limit = consumed` equals the pre-charge
    work, `next_charge` is the charge''s work amount or the plan''s reservation respectively, and every consumed
    counter and admission count remain unchanged. The '
  lines: '69'
  artifact_type: FR criterion
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request of 9 followed by 3 and then
    a charge with no size request. The records expose respectively 9, 3 and no IntegerBits request while the consumed
    IntegerBits maximum remains 9. A separate charge with IntegerBits requests 9, 3 and zero exposes present 9;
    a charge with any positive number of IntegerBits zero requests exposes present zero. Both have the same fixed
    payload size as a single-request record, with no heap-owned request collection and no retained request list,
    order or multiplicity. A successful `charge_plan
  lines: '70'
  artifact_type: FR criterion
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently chosen semantic-size requests,
    including repeated same-kind requests and distinct amounts at admissions 4096 and 4097. The single record log
    contains exactly the first 4096 fixed-size records with their points and per-admission semantic-size maxima
    in admission order; truncation is false at admission 4096 and true at 4097. Work consumption and admission count
    reach 4097, including the omitted record; existing size maxima still include the 4097th charge.
  lines: '71'
  artifact_type: FR criterion
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one work unit
    reaches `u64::MAX` exactly. The next one-unit charge returns `Incomplete` at its named point with `limit_kind
    = WorkUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission count
    stay at their pre-refusal values.
  lines: '29'
  artifact_type: FR criterion
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting `u64::MAX` result units. Its
    work consumption is 1. A second charge requesting one result unit returns `Incomplete` at its named point with
    `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission
    count remains 1.
  lines: '30'
  artifact_type: FR criterion
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount is `u64::MAX + 1` returns
    `Incomplete` with `limit_kind = WorkUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes.
  lines: '31'
  artifact_type: FR criterion
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit and `u64::MAX + 1` result units
    returns `Incomplete` with `limit_kind = ResultUnits` and `next_charge = u64::MAX + 1`; no counter or admission
    count changes.
  lines: '32'
  artifact_type: FR criterion
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least 16 bits,
    `u64::MAX` work units and one result unit. Then attempt a charge carrying `integer_bits = 16` and one result
    unit. The result refusal leaves `integer_bits = 8`, work units, result units and admission count at their pre-refusal
    values; under `test-support`, the admitted-charge log is unchanged too.
  lines: '33'
  artifact_type: FR criterion
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters the first unavailable counter
    of a charge, including several simultaneously short counters and semantic sizes attached in either order. The
    reported `limit_kind` follows `ScalarLimits` field order, with the exact limit, previously consumed amount,
    requested amount and point. A refusal after an earlier size check succeeds changes no counter, admission count
    or test-support log. A separate meter with a pending later named denial proves that this ordinary refusal does
    not advance its occurrence.
  lines: '34'
  artifact_type: FR criterion
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers without panic for every member
    of `LimitKind::ALL`: untouched counters are zero, semantic sizes are admitted high-water maxima and work/results
    are admitted cumulative totals.'
  lines: '35'
  artifact_type: FR criterion
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum observable through `consumed()`;
    the diagnostic log record is verified under FR-359-AC-12. With `IntegerBits` limit 5 and requests 7 then 9 for
    that kind in one charge, refusal reports `next_charge = 7`, the first-inserted failing amount; the existing
    9 then 7 case alone cannot distinguish maximum coalescing before refusal. Simultaneous shortages refuse in `ScalarLimits`
    field order; ordinary refusal preserves counters, high-water, admissions, log and a pending injected occurrence.
    Cancellation and injection pre
  lines: '36'
  artifact_type: FR criterion
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'PLANNED, deferred to IR-844 (u64 Meter redesign). The planned `cfg(kani)` harness, tagged `Trace: FR-359-AC-9`
    only once verified, uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand charge
    with at most two semantic-size requests, arithmetic charge with one, result-retain charge with none) over the
    full `i64` operand range, plus separate repeated-kind charge controls with at most two requests each. Symbolic
    `i64` operands and symbolic limits including `ResultUnits` cover first-short and simultaneous shortages and
    absence of the third admission. Under `'
  lines: '37'
  artifact_type: FR criterion
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their own assertions
    under their original domains, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600
    seconds. Record their individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test or altered
    consumer harness substitutes for it.
  lines: '38'
  artifact_type: FR criterion
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and called helpers finds no traversal
    with a trip count greater than that charge's semantic-size request count, including no traversal of `LimitKind::ALL`
    or another independently sized collection. The FR-362 proof schedule has at most two requests per charge, while
    the public `Charge` API retains no request-count ceiling.
  lines: '39'
  artifact_type: FR criterion
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362 schedule contains exactly the
    admitted prefix in charge order, with each record's point and absent-or-present per-admission semantic-size
    maxima matching an independent model. Reordered and repeated requests of the same kind, including zero and distinct
    amounts, coalesce to the per-admission maximum in that record. A refused charge appends no record and leaves
    the existing log unchanged. The named `tc_931_admitted_log_matches_model_ordered_prefix` and `tc_931_admitted_log_matches_model_same_kind`
    tests verify t
  lines: '40'
  artifact_type: FR criterion
- id: FR-360-AC-1
  path: spec/functional/FR-360-integer-minimum-magnitude.md
  role: examined
  excerpt: '`Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808 (`2^63`), is nonnegative,
    and differs from the original negative value. It neither overflows nor returns `i64::MIN`.'
  lines: '17'
  artifact_type: FR criterion
- id: FR-361-AC-1
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For an integer multiplication with a large operand, set `integer_bits` to one below the exact `integer-arithmetic.arithmetic`
    amount while permitting the operand charge. The outcome is `Incomplete` at `integer-arithmetic.arithmetic` with
    `limit_kind = IntegerBits` and that exact `next_charge`; the observed peak request meets the arithmetic allocation
    bound in Behavior. Repeat with operands whose subtraction mathematically cancels to 1 but whose operand-derived
    arithmetic amount exceeds the limit.
  lines: '29'
  artifact_type: FR criterion
- id: FR-361-AC-2
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a rational operation with a large numerator or denominator, set `integer_bits` to one below the exact
    `rational-arithmetic.arithmetic` amount while permitting the operand charge. The outcome is `Incomplete` at
    `rational-arithmetic.arithmetic` with `limit_kind = IntegerBits` and that exact `next_charge`; the observed
    peak request meets the arithmetic allocation bound in Behavior. Include a multiplication whose unreduced large
    factors cancel mathematically to 1.
  lines: '30'
  artifact_type: FR criterion
- id: FR-361-AC-3
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Inject a one-shot denial at each of `integer-division.arithmetic` and `integer-modulus.arithmetic` for
    large nonzero operands. Each returns `Incomplete` at the selected point with `limit_kind = WorkUnits`, admits
    no later point from that call, and meets the division allocation bound in Behavior.
  lines: '31'
  artifact_type: FR criterion
- id: FR-361-AC-4
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For valid decimal operands requiring a `2^20`-place scale expansion, set `scale_expansion` to one below
    the planned shift and leave other limits sufficient. The outcome is `Incomplete` at `decimal.scale-expansion`
    with `limit_kind = ScaleExpansion` and `next_charge` equal to the exact shift; the observed peak request is
    below 4,096 bytes.
  lines: '32'
  artifact_type: FR criterion
- id: FR-361-AC-5
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For decimal addition of small coefficients at scales 0 and `2^20`, set `integer_bits` to the admitted
    scale-expansion bit amount, one below the planned arithmetic bit amount, with other limits sufficient. The outcome
    is `Incomplete` at `decimal.arithmetic` with `limit_kind = IntegerBits` and `next_charge` equal to that planned
    bit amount; the observed peak request is below 4,096 bytes.
  lines: '33'
  artifact_type: FR criterion
- id: FR-361-AC-6
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a decimal value retained at a target scale requiring a `2^20`-place power of ten, set `decimal_digits`
    one below the exact result-retain amount while permitting preceding charges. Evaluation returns `Incomplete`
    at `decimal.result-retain` with `limit_kind = DecimalDigits` and that exact `next_charge`; the observed peak
    request is below 4,096 bytes.
  lines: '34'
  artifact_type: FR criterion
- id: FR-361-AC-7
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: On a fresh meter, floor division of a positive at-least-65,536-bit dividend by 3, exposing the quotient
    in the mathematical domain, completes to the independently calculated floor quotient with `integer_bits` limit
    equal to the dividend magnitude bit length. Final consumed `integer_bits` equals that exact bit length.
  lines: '35'
  artifact_type: FR criterion
- id: FR-361-AC-8
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For that floor quotient fixture, lower the `integer_bits` limit by one. The call returns `Incomplete`
    at `integer-division.operands` with `limit_kind = IntegerBits`, `consumed = 0` and `next_charge` equal to the
    dividend bit length; it admits no point and the measured peak allocation request stays below one eighth of the
    largest operand byte length.
  lines: '36'
  artifact_type: FR criterion
- id: FR-361-AC-9
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Multiply retained decimals `(1, 0)` and `(1, 0)` into an Exact target with coefficient bounds 0 through
    1 and scales 0 through `2^20`. At `decimal_digits = 2^20`, the `decimal.result-retain` charge returns `Incomplete`
    with `limit_kind = DecimalDigits`, `limit = 2^20`, `consumed = 2`, `next_charge = 2^20 + 1`; only the three
    preceding decimal points are admitted, no result unit is consumed, and the peak request is below 4096 bytes.
  lines: '37'
  artifact_type: FR criterion
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`; for
    negate, use each member as a unary operand. Compare integer add, subtract, multiply and negate only when the
    corresponding checked `i128` operation returns a value. Repeat over reproducibly sampled signed inputs. Every
    compared kernel result equals the independent checked result.
  lines: '42'
  artifact_type: FR criterion
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently authored
    decimal value `340282366920938463463374607431768211454`; negating `Integer::from(i128::MIN)` completes to `170141183460469231731687303715884105728`.
    Neither result is narrowed to `i128`.
  lines: '43'
  artifact_type: FR criterion
- id: FR-362-AC-3
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For fixed and reproducibly sampled rational pairs whose checked `i128` reference intermediates fit and
    whose denominators are nonzero, rational add, subtract, multiply, divide by nonzero and negate equal an independently
    reduced-fraction oracle. A zero rational divisor returns `Undefined(DivisionByZero)` after only the operand
    charge.
  lines: '44'
  artifact_type: FR criterion
- id: FR-362-AC-4
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Integer and rational comparisons over fixed and reproducibly sampled pairs equal independently checked
    numeric comparisons for equality and order. Decimal comparison is specified by [FR-363](./FR-363-metered-decimal-ordering.md).
  lines: '45'
  artifact_type: FR criterion
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each rational arithmetic operation,
    integer ordering and rational ordering, the admitted point sequence equals its family row in Behavior. A successful
    already-decided Boolean retention admits only `boolean.result-retain`.
  lines: '46'
  artifact_type: FR criterion
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate the maximum size amount across
    their points from the formulas in Behavior, using the unreduced numerator and denominator for rational normalization.
    The observed final `integer_bits` and `value_occurrences` equal those maxima; work consumption equals the number
    of admitted points and result consumption is one.
  lines: '47'
  artifact_type: FR criterion
- id: FR-362-AC-7
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter, directly add integers 5 and 7, compare rationals 1/2 and 2/3 for less-than, then retain
    an already-decided Boolean `true`. The admitted atom sequence consumes 7 work units and 3 result units, derived
    from the three family rows in Behavior.
  lines: '48'
  artifact_type: FR criterion
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer arithmetic, rational arithmetic,
    integer ordering, rational ordering and already-decided Boolean retention calls. Each returns `Incomplete` naming
    the denied point, with no completed value, no result unit for the denied charge and no later point admitted
    by that call.
  lines: '49'
  artifact_type: FR criterion
- id: FR-362-AC-9
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For every already-decided Boolean connective, the completed value matches its truth table and the kernel
    admits `boolean.result-retain` once, consuming one work and one result unit.
  lines: '50'
  artifact_type: FR criterion
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and negate,
    and integer/rational ordering, independently calculate the required bit high-water amount from Behavior. On
    fresh meters with other limits sufficient, that exact bit limit completes with the expected value, while a limit
    one below stops at the first point whose amount exceeds it, with exact prior consumption, request and admitted
    prefix and no result unit. Include cancellation `1000 - 999`, `255 * 255`, rational `(2/3) * (3/2)`, `(3/4)
    / (5/7)`, `(5/7) - (4/7)`, negate `-5/8`, and `7/
  lines: '51'
  artifact_type: FR criterion
- id: FR-362-AC-11
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`, typed domain `Refused`,
    and `Incomplete` with no partial value; `completed()` returns a value only for `Completed`, including false.
    On fresh meters, rational division by zero retains only operands; integer `2 + 2` outside the bound `[0, 3]`
    retains operands and arithmetic but no result unit; a rational result outside its domain retains operands, arithmetic
    and normalize but no result unit. A denied first operand charge admits nothing. Each stop preserves exactly
    the admitted prefix, work totals and size
  lines: '52'
  artifact_type: FR criterion
- id: FR-362-AC-12
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '`Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator 1 and denominator 2, proving
    reduction to lowest terms and cancellation of two negative signs. `Rational::new(1, -2)` exposes -1/2. For either
    denominator 5 or -5, a zero numerator exposes exactly 0/1.'
  lines: '53'
  artifact_type: FR criterion
- id: FR-362-AC-13
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)` is
    false. Dividing `1/1` by `3/1` under that domain returns `Refused(RationalOutOfDomain { target })` after normalization
    with no result unit; the identical call without a domain completes to `1/3`.
  lines: '54'
  artifact_type: FR criterion
- id: FR-362-AC-14
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Call the five scalar atoms `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0` directly on one meter. They complete
    to true, 1, true, 0, false in order, consuming 15 work and 5 result units. With work limit 14, the final ordering
    stops at `ordering.result-retain` with `limit = consumed = 14`, `next_charge = 1` and four result units.
  lines: '55'
  artifact_type: FR criterion
- id: FR-362-AC-15
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Directly divide rational `3/1` by `2/1` on a fresh meter. The result is `3/2`, with high-water `integer_bits
    = 3`, value occurrences 2, work 4 and results 1; work limit 3 stops at `rational-arithmetic.result-retain` after
    the first three points and consumes no result unit.
  lines: '56'
  artifact_type: FR criterion
- id: FR-362-AC-16
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter directly add integers `0 + 1` and then `1 + 2`. The results are 1 and 3; the integer point
    sequence occurs twice, high-water `integer_bits = 3`, occurrences 2, work 6 and results 2. Caller fold evaluation
    is outside this direct-atom criterion.
  lines: '57'
  artifact_type: FR criterion
- id: FR-362-AC-17
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain` after operands and arithmetic,
    with zero result units; work limit 3 completes to 12 with three work and one result unit. For rational `(2/3)
    * (3/2)`, work limit 2 stops at `rational-arithmetic.normalize`, work limit 3 stops at `rational-arithmetic.result-retain`,
    and work limit 4 completes to `1/1`; each denied call preserves exactly its admitted prefix and consumes zero
    result units.
  lines: '58'
  artifact_type: FR criterion
- id: FR-362-AC-18
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For `k` in 63, 64, 65, 127, 128, 200 and 511, compare exact results and charged bit high-water sizes
    around `2^k`: `(2^k - 1)(2^k + 1)` charges `2k + 1` bits for a `2k`-bit result, `(2^k)^2` charges `2k + 2` for
    a `2k + 1`-bit result, and adding `(2^k - 1) + 1` charges `k + 1` for a `k + 1`-bit result. Subtracting `(2^k
    + 1) - 2^k` or `2^k - 2^k` still charges `k + 2` bits despite a one-bit result.'
  lines: '59'
  artifact_type: FR criterion
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add `8 + 3` and
    `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`, without a result bound and with all non-bit limits sufficient.
    The independently calculated `integer-arithmetic.arithmetic` bit amounts are respectively 5, 5, 5 and 6: add/subtract
    use `max(B(a), B(c)) + 1`, including the larger right operand and a result that cancels to one bit; multiply
    uses `B(a) + B(c)`. At each exact `integer_bits` limit, the call completes to respectively 11, 11, 1 and 24,
    admits operands, arithmetic and result-'
  lines: '60'
  artifact_type: FR criterion
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'On fresh sufficient public meters under `test-support`, evaluate the six rational calls listed below
    with no result domain. Each completes to the listed canonical value and admits operands, arithmetic, normalize
    and result-retain in order, consuming four work units and one result unit. The actual normalize record has only
    IntegerBits present, with the listed amount independently derived from the unreduced parts and `B(0) = 1`. The
    recorded arithmetic amount equals its separate listed bound. The normalize observation is not replaced by that
    bound, the final bit maximum or the reduced '
  lines: '61'
  artifact_type: FR criterion
- id: FR-362-AC-21
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On a fresh public meter, a direct default charge at `TemporalVisit` (`temporal.visit`) succeeds with
    `work_units = 1`, `result_units = 0` and no semantic-size request when the work limit is exactly 1. With work
    limit 0, the same call returns `Incomplete` with `limit_kind = WorkUnits`, `limit = consumed = 0`, `next_charge
    = 1` and `charge_point = TemporalVisit`; no charge is admitted and all counters remain zero.
  lines: '62'
  artifact_type: FR criterion
- id: FR-362-AC-22
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: A one-shot injected denial of the first direct charge at `TemporalVisit` (`temporal.visit`) returns `Incomplete`
    naming `TemporalVisit` and `WorkUnits`, with `limit = consumed = 0` and `next_charge = 1`; it admits no charge,
    changes no counter and consumes the one-shot denial, so a second identical charge on the same otherwise sufficient
    meter succeeds.
  lines: '63'
  artifact_type: FR criterion
- id: FR-363-AC-1
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For decimal pairs with equal values but different retained scales, and for positive, negative and zero
    values at differing scales, `order_numbers` agrees with an independent exact rational comparison for each ordering
    operator. The retained representations remain unchanged.
  lines: '35'
  artifact_type: FR criterion
- id: FR-363-AC-2
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For a successful decimal comparison, the meter admits `ordering.operands`, `ordering.arithmetic`, `ordering.result-retain`
    in that order. On a fresh meter, its size high-water counters equal the maxima in Behavior, with three work
    units and one result unit.
  lines: '36'
  artifact_type: FR criterion
- id: FR-363-AC-3
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: Comparing retained `(0, 5) >= (0, 0)` completes `true` and consumes `integer_bits = 18`, `decimal_digits
    = 6`, `scale_expansion = 5`, `value_occurrences = 2`, `work_units = 3`, `result_units = 1`. Comparing retained
    `(100, 2) <= (2, 0)` completes `true` and consumes `integer_bits = 9`, `decimal_digits = 3`, `scale_expansion
    = 2`, `value_occurrences = 2`, `work_units = 3`, `result_units = 1`.
  lines: '37'
  artifact_type: FR criterion
- id: FR-363-AC-4
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: With all limits at `u64::MAX` except `decimal_digits = 64`, testing retained `(1, 0) < (1, u32::MAX)`
    returns `Incomplete` at `ordering.arithmetic` with `limit_kind = DecimalDigits`, `limit = 64`, `consumed = 1`,
    and `next_charge = 4,294,967,296`. Only `ordering.operands` is admitted; no result unit is consumed and the
    scale-alignment power of ten is not materialized.
  lines: '38'
  artifact_type: FR criterion
- id: FR-363-AC-5
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: 'On separate fresh meters with generous limits, inject a one-shot denial at occurrence 1 of each decimal-ordering
    point: `ordering.operands`, `ordering.arithmetic`, and `ordering.result-retain`. For each point, ordering retained
    `(0, 5) >= (0, 0)` returns `Incomplete` with `limit_kind = WorkUnits`, `charge_point` equal to the selected
    point, `limit = consumed` equal to the number of preceding admitted points (0, 1, or 2), and `next_charge =
    1`. No Boolean result or result unit is exposed; only the preceding point prefix is admitted and logged, with
    no later point admitted.'
  lines: '39'
  artifact_type: FR criterion
- id: FR-363-AC-6
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For retained decimals `(110, 2)` and `(11, 1)`, public unmetered `Decimal::compare` returns `Equal`,
    `numerically_equal` returns true and both normalized representations are `(11, 1)`; their retained representations
    remain `(110, 2)` and `(11, 1)`.
  lines: '40'
  artifact_type: FR criterion
- id: FR-363-AC-7
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` both complete false. The mixed
    pair consumes integer bits, decimal digits and scale expansion 8/3/1; the narrow pair consumes 4/2/0. Both consume
    value occurrences/work/results 2/3/1 with all other counters zero, despite equal normalized values.
  lines: '41'
  artifact_type: FR criterion
- id: FR-364-AC-1
  path: spec/functional/FR-364-ieee-flag-iteration.md
  role: examined
  excerpt: For every subset of the five public flags, collect it into `IeeeFlags` in forward and reverse vocabulary
    order. Both `iter()` results equal that subset filtered from `IeeeFlag::ALL`, without duplicates. The empty
    set iterates empty and the full set iterates all five flags in the stated order.
  lines: '21'
  artifact_type: FR criterion
- id: FR-365-AC-1
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: Round retained `(5, 1)` and `(-5, 1)` to a scale-zero target with coefficient bounds -10 through 10 under
    each of the six modes. Values or `Refused(InexactDecimal { target })` match the tie table in Behavior; each
    completed representation has scale zero, and refusals consume no result unit.
  lines: '23'
  artifact_type: FR criterion
- id: FR-365-AC-2
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: '`RoundingMode::default()` and the mode read back from a target constructed with it are `Exact`. Rounding
    retained `(1, 1)` to that scale-zero target returns `Refused(InexactDecimal { target })`, rather than silently
    rounding 0.1 to zero, with no retained result.'
  lines: '24'
  artifact_type: FR criterion
- id: FR-366-AC-1
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For same-width Add with a positive quiet NaN of payload 5 and a negative signaling NaN of payload 9 in
    either operand order, the result bits are the first NaN quieted and Invalid is present. Two quiet NaNs select
    the first without Invalid. For FusedMultiplyAdd with a quiet first NaN, finite second operand and signaling
    third NaN, the first NaN remains selected and Invalid is present. Exercise both binary widths.
  lines: '21'
  artifact_type: FR criterion
- id: FR-366-AC-2
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: 'Converting a binary64 NaN with payload `2^22` or `2^30` to binary32 returns `Refused(IeeeNanPayloadNotRepresentable
    { source: Binary64, target: Binary32 })` without truncating the payload or retaining a result unit. At payload
    `2^22 - 1`, positive and negative binary64 quiet NaNs both complete as binary32 quiet NaNs with their respective
    signs and all payload bits preserved, and one result unit is consumed.'
  lines: '22'
  artifact_type: FR criterion
- id: FR-366-AC-3
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: Converting either signed binary32 zero to the rational domain with numerator interval `[-1, 1]` and denominator
    interval `[1, 1]` completes to canonical `0/1`. Positive zero has no loss; negative zero has exactly `IeeeExactLoss::NegativeZeroSign`.
  lines: '23'
  artifact_type: FR criterion
- id: FR-366-AC-4
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For each width, distinct bit patterns representing the ten classes in Behavior have strictly increasing
    `total_order_key()` values in that stated order, including quiet/signaling NaNs of both signs and both zeros.
    Positive NaN payloads of the same class ascend by payload and negative ones descend. The public TotalOrder comparison
    agrees with the key order for these pairs; NumericEqual considers the two zeros equal despite their distinct
    keys.
  lines: '24'
  artifact_type: FR criterion
- id: FR-367-AC-1
  path: spec/functional/FR-367-text-profile-mismatch-before-charging.md
  role: examined
  excerpt: 'Admit the same UTF-8 text "a" once with Nfc and once with BinaryUtf8 using separate sufficient meters.
    On a fresh comparison meter whose ten limits are zero, public `compare_text(Equal, left, right)` returns precisely
    `Err(IllTyped { cause: DistinctTextProfiles })`. Every consumed counter and admission count remains zero; under
    `test-support` the charge log is empty and truncation is false.'
  lines: '21'
  artifact_type: FR criterion
- id: FR-368-AC-1
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`, has a unique documented
    spelling from `as_str()` and round-trips through `from_code()`; an unknown spelling returns `None`. `TemporalVisit`
    spells `temporal.visit` and is at index 62 after `ModelNavigate` at index 61; entries 0 through 61 retain their
    pre-extension order. Exhaustive variant matching makes an added but unaccounted variant fail the check.
  lines: '23'
  artifact_type: FR criterion
- id: FR-368-AC-2
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 'Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`, has a unique documented `as_str()`
    spelling, and the sequence is exactly the ten `ScalarLimits` fields: integer bits, decimal digits, scale expansion,
    text input bytes, text scalars, normalized scalars, unit edges, value occurrences, work units and result units.
    Exhaustive variant matching makes an added but unaccounted variant fail the check.'
  lines: '24'
  artifact_type: FR criterion
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant
    set and four typed `IllTypedCause` payloads equal the thirty table rows; `CheckedInvariantCause` implements
    the stated traits, and the old unit constructor does not compile.
  lines: '153'
  artifact_type: FR criterion
- id: FR-369-AC-2
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Every kernel producer in collection, value, division and equality returns exactly its table cause; two
    population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`,
    and unlike collection kinds return `CollectionKindMismatch`. Two composites of different declarations return
    `false` and a composite paired with a non-composite returns `ValueKindMismatch` (FR-376-AC-1, FR-376-AC-6).
  lines: '154'
  artifact_type: FR criterion
- id: FR-369-AC-5
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and
    `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are
    unchanged.
  lines: '155'
  artifact_type: FR criterion
- id: FR-369-AC-6
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Every received typed cause follows QSL''s existing internal-fault path: no refusal record or `Evaluation`,
    `runtime_invariant`/`S6a`/`checked-program-invariant` at S6a, `CallFailure::Fault` at the public call, and `Failed`
    with unavailable basis for an internal replay fault. It never becomes an ordinary replay refusal, `Inconclusive`,
    `Incomplete(Cancelled)`, a proved result or a certification rejection; an unknown received cause retains fault
    provenance.'
  lines: '156'
  artifact_type: FR criterion
- id: FR-369-AC-8
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT and QSV
    own no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT,
    QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables.
  lines: '157'
  artifact_type: FR criterion
- id: FR-370-AC-1
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: A UUID value is admitted only by `ValueType::Uuid` and a Timestamp value only by `ValueType::Timestamp`;
    crossed native kinds and Integer, Text and Reference values are rejected by both native types, without a conversion
    or alias.
  lines: '105'
  artifact_type: FR criterion
- id: FR-370-AC-2
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'Each same-kind native pair follows the existing one-terminal-pair equality schedule: an equal pair yields
    `true`, an unequal pair yields `false`, and the planned pair count and charges equal the corresponding Boolean-leaf
    schedule.'
  lines: '106'
  artifact_type: FR criterion
- id: FR-370-AC-3
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'A direct equality plan pairing UUID or Timestamp with another value kind returns `Refusal::CheckedInvariant
    { cause: ValueKindMismatch }`, while a direct mixed-kind `compare_keys` call returns `None`; neither returns
    a Boolean or a false equal key.'
  lines: '107'
  artifact_type: FR criterion
- id: FR-370-AC-4
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: '`compare_keys` gives each native type a total key consistent with its equality relation and ordered
    by canonical payload ASCII bytes; Timestamp `"10"` precedes `"2"` and `"-1"` precedes `"-2"` in the key, independent
    of value construction or insertion order and without exposing a native ordering operator.'
  lines: '108'
  artifact_type: FR criterion
- id: FR-370-AC-5
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Existing scalar and composite admission, equality, key comparison and accounting vectors retain their
    prior results.
  lines: '109'
  artifact_type: FR criterion
- id: FR-370-AC-6
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: UUID values preserve exactly 16 network-order octets, including Nil, Max and non-RFC-version bit patterns;
    only the lowercase ASCII `8-4-4-4-12` caller payload admits, and uppercase, braces, URN form, altered grouping/separators,
    whitespace, length or nonhex characters refuse without repair or value.
  lines: '110'
  artifact_type: FR criterion
- id: FR-370-AC-7
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Timestamp values preserve signed `i128` POSIX-epoch nanoseconds, including inclusive bounds `-170141183460469231731687303715884105728`
    and `170141183460469231731687303715884105727` and `-1`, `0`, `1` as one nanosecond before, at and after the
    epoch; only canonical signed base-10 caller payload admits, while either adjacent exterior count, `-0`, plus
    signs, leading zero, fraction, exponent, non-ASCII digits, calendar/offset text and whitespace refuse without
    repair or value.
  lines: '111'
  artifact_type: FR criterion
- id: FR-370-AC-8
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Kernel constructors accept canonical UUID or Timestamp text supplied after the caller has validated its
    tagged wire envelope. Each malformed text returns `ConstructionRefusal` at `Component::Value` with its tabled
    cause, no value and no internal checked-invariant fault; canonical out-of-range decimal returns `TimestampOutOfDomain`
    after syntax checking. The kernel has no second parser-error type or UUID out-of-domain case.
  lines: '112'
  artifact_type: FR criterion
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact
    value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64
    MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations
    with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals.
    Reject malformed/noncanonical spell
  lines: '79'
  artifact_type: FR criterion
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal,
    identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt
    access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all
    accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted
    borrows or introduce a cache-state value.
  lines: '80'
  artifact_type: FR criterion
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native,
    native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit
    and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete
    fields and admitted prefixes with the independent normative formulas. No representation transition manufactures
    a FunctionCall or other charge.
  lines: '81'
  artifact_type: FR criterion
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate,
    magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator
    observer around standalone Integer calls confirms no allocation on that native path, including clone after cache
    access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently
    prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty.
    Observation of public metered calls separ
  lines: '82'
  artifact_type: FR criterion
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Public signatures and all existing Integer methods remain available, except that the divisor parameters
    of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger
    signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain
    &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other
    public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync
    and current panic-unwind traits. De
  lines: '83'
  artifact_type: FR criterion
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain
    bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample
    checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation,
    accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing
    mutation execution or missing teardown is inconclusive and does not satisfy this criterion.
  lines: '84'
  artifact_type: FR criterion
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the authoritative membership rule for inclusive lower/upper equality, adjacent
    outsiders, negative and zero values, scale refusal, all rounding spellings and equivalent retained representations;
    neither representation is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true at the
    exact upper bound.
  lines: '167'
  artifact_type: FR criterion
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Production helper observations on admitted `B,S` fixtures stay within the one-request `4 × L_B`-byte
    scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step bound, including a longest-pass endpoint fixture and
    a large shift that terminates when quotient becomes zero. Controls exercise actual inline and Large production
    paths, including zero, 32-bit limb crossings, i128 MIN/MAX and signed magnitudes beyond i128. Stream traversal
    requests no allocation and leaves cold inline caches empty; inline streams yield at most four least-significant-first
    u32 limbs. Both stream mechanisms and al
  lines: '168'
  artifact_type: FR criterion
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Denying the actual scratch reservation reports its actual native request as typed allocation failure.
    A separate checked-capacity failure retains its own classification. Removing the allocation denial yields the
    same membership as a fresh call, with no partial accepted result after either refusal.
  lines: '169'
  artifact_type: FR criterion
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation observed during the longest admitted
    division/comparison path returns the original cause before another limb step or result; cancellation observed
    immediately after successful reservation prevents scratch work. Neither failure is classified as nonmembership
    or work exhaustion.
  lines: '170'
  artifact_type: FR criterion
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder and sign invariants, exact reservation
    boundary, checked layout conversions, allocation-free inline unsigned-magnitude stream and Large borrowed dependency
    iterator, absence of cache initialization or BigInt-image construction on those streams, and stated arithmetic
    polling gap. Its record distinguishes requested storage from allocator overhead and work latency from unbounded
    allocator elapsed time.
  lines: '171'
  artifact_type: FR criterion
- id: FR-373-AC-1
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: The public crate-root `ieee_to_exact` has the exact direct signature in Behavior. `IeeeExactTarget` and
    its re-export are absent; no compatibility entry point or copied conversion algorithm remains. A non-Rational
    target cannot be supplied to this signature.
  lines: '76'
  artifact_type: FR criterion
- id: FR-373-AC-2
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Binary32 `0x3fc00000` and binary64 `0x3ff8000000000000` complete as `3/2` with no loss in numerator domain
    `[-3,3]`, denominator domain `[1,2]`. Both signed zeros at both widths complete as `0/1`; only negative zero
    carries `NegativeZeroSign`, preserving FR-366's original binary32 control.
  lines: '77'
  artifact_type: FR criterion
- id: FR-373-AC-3
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Quiet NaN, signaling NaN and positive/negative infinity at each width return exactly `Undefined(IeeeNotFinite)`
    after one operand charge, one work unit, source-width IntegerBits and one ValueOccurrences high-water unit,
    with zero result units and no intermediate or retain admission.
  lines: '78'
  artifact_type: FR criterion
- id: FR-373-AC-4
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Converting `3/2` to numerator domain `[-1,1]`, denominator domain `[1,2]`, and separately to numerator
    domain `[-3,3]`, denominator domain `[1,1]`, returns `Refused(IeeeRationalOutOfDomain { target })` with the
    corresponding exact target. Two work units are consumed; no result unit or retain charge is admitted.
  lines: '79'
  artifact_type: FR criterion
- id: FR-373-AC-5
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Successful finite conversion admits exactly operands, intermediate and retain in that order, consuming
    three work units and one result unit. Ordinary work limits 0, 1 and 2 deny the respective next point with exact
    limit, consumed and next-charge fields. Injected occurrence-1 denials at each point report WorkUnits with limit
    equal to pre-charge consumed work and next charge 1. Result limit 0 denies retain atomically after two work
    units. Smallest positive subnormals at either width under IntegerBits equal to the source width deny intermediate
    with next-charge sizes 150 and 1075 respe
  lines: '80'
  artifact_type: FR criterion
- id: FR-373-AC-6
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: For Requested and Deadline, pre-cancelling the original handle attached to the supplied meter returns
    WorkUnits incomplete at operands with limit = consumed = 0 and next charge 1. Deterministic cancellation at
    the original handle's second and third meter polls returns the same form at intermediate and retain with limit
    = consumed = 1 and 2 respectively. The original handle records the actual cause, only the prior charge prefix
    is admitted and no result is retained. A live handle completes the same fixture.
  lines: '81'
  artifact_type: FR criterion
- id: FR-373-AC-7
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Precharge the supplied meter once at FunctionCall with work 2, result 1, IntegerBits 80, ValueOccurrences
    5 and TextScalars 7. Direct conversion of either width's `3/2` under sufficient limits ends with work 5, results
    2 and admission count 4, preserving all three high-water values and the FunctionCall log prefix. Independently
    repeat with work limit 4, result limit 1, an occurrence-1 retain injection and original-handle cancellation
    at the third conversion poll. Each stops at retain with work 4, results 1 and admission count 3; ordinary records
    report respectively WorkUnits(limit=co
  lines: '82'
  artifact_type: FR criterion
- id: FR-374-AC-1
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Admission of zero constructed by primitive, canonical parse and zero-producing arithmetic returns exactly
    `Err(ZeroDivisor)` and no token. Positive and negative values, including values outside i128, admit; `get` borrows
    the original integer, with no Integer/BigInt clone or materialization for admission. The public exports and
    typed unit error match the stated contract.
  lines: '168'
  artifact_type: FR criterion
- id: FR-374-AC-2
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: An external public-API caller can reuse one borrowed copyable token across all three helpers. A plain
    `&Integer` cannot satisfy any of the three divisor parameters. External field construction, writable access
    and a token outliving its referent are rejected by the public type/lifetime boundary; no Default or unchecked
    public constructor permits zero bypass.
  lines: '169'
  artifact_type: FR criterion
- id: FR-374-AC-3
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)` return respectively truncating
    `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and floor `(3,1)`, `(-4,-1)`, `(-4,1)`, `(3,-1)`. A zero dividend with
    either divisor sign returns `(0,0)` in both helpers. Dividing `680564733841876926926749214863536422912` by either
    `2` or `-2` returns respectively quotient `340282366920938463463374607431768211456` or its negative and remainder
    zero, without narrowing.
  lines: '170'
  artifact_type: FR criterion
- id: FR-374-AC-4
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct exact_div on every sign combination of dividend 6 and divisor 2 returns the signed quotient 3;
    zero dividend with either divisor sign returns zero. The beyond-i128 divisible literals in AC-3 return the same
    signed quotient. The nondivisible controls `5/2 = 2` and `-5/2 = -2` retain the prior behavior, without divisibility
    rejection or a fallible helper result.
  lines: '171'
  artifact_type: FR criterion
- id: FR-374-AC-5
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Inspection accounts for every exact-owned production caller in Rational, Decimal and division. Canonical
    callers establish the documented nonzero invariant without forced Integer/BigInt clones, new public errors in
    existing canonical infallible APIs, caller-divisor admission unwrap/expect, unchecked public admission or fabricated
    numeric fallback. Any private token construction has an explicit owning nonzero invariant.
  lines: '172'
  artifact_type: FR criterion
- id: FR-374-AC-6
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: With sufficient real meters, zero divisor under all three divide profiles and both members, plus public
    modulo, returns `Undefined(DivisionByZero)` after exactly the operation's operands charge, with no arithmetic/domain/result-retain
    or admission charge. Denying operands returns the existing Incomplete first. Nonzero success and exposed-member
    refusal controls retain the existing laws, charge requests, ordered prefixes, counters and outcome fields; public
    metered signatures remain unchanged.
  lines: '173'
  artifact_type: FR criterion
- id: FR-375-AC-7
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool
    control demonstrates both a passing and a deliberately failing dependency-contract case through the actual RT-to-quire-exact
    dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real
    callee remains in the outer CBMC graph.
  lines: '31'
  artifact_type: FR criterion
- id: FR-375-AC-9
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: On equal and first-head-unequal bounded kernel operands, `planned_equality_with_plan` returns the Boolean
    relation and 17 planned pair events from the same formed plan; `plan_equality` agrees on that count, and the
    admitted meter log contains 17 pair charges on success.
  lines: '32'
  artifact_type: FR criterion
- id: FR-375-AC-10
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: When `Meter::charge` refuses the 17th pair, the result-bearing kernel operation returns the typed stop
    with exactly the prior 16 pair charges admitted and no result-retain charge; it does not report all 17 planned
    pairs as admitted. A result-retain refusal after all 17 pair charges retains that admitted prefix.
  lines: '33'
  artifact_type: FR criterion
- id: FR-376-AC-1
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`plan_equality`, `planned_equality` and `planned_equality_with_plan` on two composites whose declaration
    keys differ return `false`, with a pair count of 1, and no `Refusal`. The result is the same in either operand
    order, for operands with equal slots, with different slots and with different slot counts, and for a pair of
    empty-slot composites.'
  lines: '44'
  artifact_type: FR criterion
- id: FR-376-AC-2
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: On that pair, `planned_equality` admits `equality.plan-form`, `equality.plan`, exactly one `equality.pair`
    and `equality.result-retain`, in that order, with the plan-form charge computed from the two operands' occurrence
    counts as FR-375 states.
  lines: '45'
  artifact_type: FR criterion
- id: FR-376-AC-3
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite pair of different declarations whose slots hold `Reference` values of different universes
    returns `false` and does not report `foreign_reference`, because the slots are not visited. As a control, two
    composites of one declaration whose slots hold `Reference` values of those two universes still report `foreign_reference`.
  lines: '46'
  artifact_type: FR criterion
- id: FR-376-AC-4
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: Two composites with equal declaration keys and equal slots give `true`. Equal declaration keys with a
    differing slot give `false`. The pair counts are those of the same plan before this requirement.
  lines: '47'
  artifact_type: FR criterion
- id: FR-376-AC-5
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A pair of different declarations that sits as the payload of two present `Option`s, in a one-slot record,
    as the element of two one-element `Sequence`, `Set` and `Bag`, gives `false`, with pair counts of 2, 2 and 2,
    and no refusal.
  lines: '48'
  artifact_type: FR criterion
- id: FR-376-AC-6
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, `Option`, `Collection` or `Reference`, in either order, still refuses
    `ValueKindMismatch`. Two composites of one declaration key and different slot counts still refuse `ValueKindMismatch`.
    Two populations still refuse `PopulationPair`. Neither refusal returns a completed Boolean.
  lines: '49'
  artifact_type: FR criterion
- id: FR-376-AC-7
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`member_equal` of a composite candidate against a member of a different declaration returns `false`,
    charging `collection.member-walk` then `collection.member-test` with one pair.'
  lines: '50'
  artifact_type: FR criterion
- id: FR-376-AC-8
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`form` over an `OrderedSet` type whose element type is a declaration P, with occurrences of one composite
    of declaration C then one of P, completes with the two members in that supply order, charging one membership
    comparison; the occurrences P then C give P then C; C, P, C give C then P. None refuses `ValueKindMismatch`.
    An `OrderedSet` is not sorted by the canonical key, so it needs this requirement and not FR-377.'
  lines: '51'
  artifact_type: FR criterion
- id: FR-376-AC-9
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A union member whose one position holds a `P` composite in one value and a `C` composite in the other
    (also `C` against `G`), in both orders, gives `false` with a pair count of 2 and no refusal; reference slots
    of different universes inside the skipped composites do not report `foreign_reference`; a two-position payload
    keeps its second pair after a mismatch in the first (3 pairs); the same pair as the payload of two `Option`s
    gives `false` with 3 pairs; equal declarations walk the slots; a union against an `Integer`, two unions of one
    member with different payload lengths and two un
  lines: '52'
  artifact_type: FR criterion
- id: FR-377-AC-1
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: '`compare_keys(a, b)` for composites of declaration keys `[1; 32]` and `[2; 32]` is `Some(Less)` and
    `compare_keys(b, a)` is `Some(Greater)`, whatever their slots: slots equal, slots that would order the other
    way, and different slot counts.'
  lines: '46'
  artifact_type: FR criterion
- id: FR-377-AC-2
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: Declaration keys that differ only in the last byte (`[1; 31]` followed by `0`, against the same followed
    by `1`) order by that byte; keys that differ in the first byte order by the first byte even when the later bytes
    order the other way.
  lines: '47'
  artifact_type: FR criterion
- id: FR-377-AC-3
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: Two composites of one declaration key keep the slot order of the rule before this requirement; `compare_keys(x,
    x)` is `Some(Equal)`; two composites of different declarations are never `Some(Equal)`, even with identical
    slots.
  lines: '48'
  artifact_type: FR criterion
- id: FR-377-AC-4
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A `Set` and a `Bag` of element type `Composite(P)` formed through `form` from one composite of each of
    two declarations, in each of the two supply orders, complete and visit the member of the smaller declaration
    key first. `form_grouped` over the same two members gives the same order. A `Bag` formed through `form` from
    two equal members of each declaration visits the two groups in that order.
  lines: '49'
  artifact_type: FR criterion
- id: FR-377-AC-5
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'Equal keys occur exactly when values are equal: for a set of composites of two declarations with equal
    and unequal slots, every pair has `compare_keys == Some(Equal)` exactly when `planned_equality` gives `true`.'
  lines: '50'
  artifact_type: FR criterion
- id: FR-377-AC-6
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, two composites of one declaration key and different slot counts, and
    a pair of `Float` members each return `None` from `compare_keys`. As the two members of a `Set`, each refuses
    `CanonicalKeyUnavailable` through `form_grouped` and `ValueKindMismatch` through `form`, which refuses at membership
    comparison before any ordering.
  lines: '51'
  artifact_type: FR criterion
- id: FR-377-AC-7
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: The key order of a pair of different declarations holds as the payload of two `Option`s, in a record
    slot and as an element of two `Sequence` values, ordering the enclosing values by the same declaration-key rule.
  lines: '52'
  artifact_type: FR criterion
- id: FR-377-AC-8
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: '`compare_keys` on unions of one member that hold a `P`, `C` or `G` composite at the payload position
    returns `Some(Less)` for `P` against `C` and `C` against `G`, whatever the slots, and `Some(Equal)` agrees with
    `planned_equality` on all nine ordered pairs of the three; an `Alpha` union holding `C` orders before a `Zulu`
    union holding `P`; `compare_keys` returns `None` for unions of different declarations and for one member with
    different payload lengths; `form` of a `Set` over `C`, `P`, `C` unions (and over `P`, `C`, `P`) keeps the `P`
    union then the `C` union; `form` of an `Ordere'
  lines: '53'
  artifact_type: FR criterion
- id: FR-378-AC-1
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` and `form_grouped` over an element type `Composite(P)` and one `P` member and one `C` member
    complete for `Sequence`, `Set`, `Bag` and `OrderedSet`, keep both members unchanged (declaration key and slots),
    and for `Set` and `OrderedSet` hold two members even with equal shared slots. No collection call tests an occurrence
    against the element type.'
  lines: '51'
  artifact_type: FR criterion
- id: FR-378-AC-2
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` over a `Set` of those two members charges, in order, `collection.member-walk`, `collection.member-test`
    with one pair, `collection.bound` and `collection.result-retain`, and no `collection.element`.'
  lines: '52'
  artifact_type: FR criterion
- id: FR-378-AC-3
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted(Composite(P), Some(c))` returns an `Option` whose `payload_type()` is `Composite(P)`
    and whose payload has declaration `C` and its slots unchanged.'
  lines: '53'
  artifact_type: FR criterion
- id: FR-378-AC-4
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted_slots` of a declaration whose declared field type is `Composite(P)` and whose slot holds
    a `C` composite keeps the slot unchanged; `retain_composite` charges `composite.result-retain` once; the composite
    is unequal to one with the same declaration and a `P` composite in that slot (FR-376).'
  lines: '54'
  artifact_type: FR criterion
- id: FR-378-AC-5
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: 'For the same `C` composite at element type `Composite(P)`: `construct_collection` refuses `CollectionElementNotAdmitted`;
    `form_collection` refuses `Element(index)` `TypeMismatch`; `ValueType::admits` is `false`; `OptionValue::present`
    refuses `Payload` `TypeMismatch`; `evaluate_record` with that field refuses `DeferredResultNotAdmitted`. A `P`
    composite is accepted by each.'
  lines: '55'
  artifact_type: FR criterion
- id: FR-378-AC-6
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: A collection of element type `Composite(C)` is not admitted by `ValueType::admits` at the collection
    type of element type `Composite(P)`, nor the reverse. A stored collection clones, compares equal to an identical
    build (`true`), compares unequal to one that differs in a member's declaration (`false`), and drops on the iterative
    paths of any `Value`.
  lines: '56'
  artifact_type: FR criterion
- id: FR-378-AC-7
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: The rustdoc of the seven items in Behavior 8 carries the stated text.
  lines: '57'
  artifact_type: FR criterion
- id: FR-378-AC-8
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`UnionValue::from_admitted` of a position typed `Composite(P)` and a `C` composite keeps the payload
    unchanged (declaration `C`, slots unchanged). For that `C` composite `union` refuses `Position(0)` `TypeMismatch`
    and `evaluate_union` refuses `DeferredResultNotAdmitted`; a `P` composite is accepted by each.'
  lines: '58'
  artifact_type: FR criterion
- id: FR-379-AC-1
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A nullary member has an occurrence count of 1 and a two-position member over two integers has 3. The
    value reports its declaration, `VariantId`, identifier and payload in position order. `ValueType::Composite`
    of its declaration admits it, `Composite` of another declaration and `Integer` do not.
  lines: '53'
  artifact_type: FR criterion
- id: FR-379-AC-2
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`union` refuses an arity of 0 against 1 supplied and 2 against 1 supplied with `Value` `WrongArity`,
    and a `Boolean` at an `Integer` position with `Position(0)` `TypeMismatch`.'
  lines: '54'
  artifact_type: FR criterion
- id: FR-379-AC-3
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`evaluate_union` returns `Undefined` for the first argument that is undefined, runs no later argument
    and admits no charge. A completed `Boolean` at an `Integer` position refuses `DeferredResultNotAdmitted`. A
    completed evaluation runs the arguments in order and admits one `composite.result-retain` with the occurrence
    count of the result, and a retain limit of one fewer returns `Incomplete` naming the charge point, the limit
    kind and the limit.'
  lines: '55'
  artifact_type: FR criterion
- id: FR-379-AC-4
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Unions of one declaration and member with equal payloads are equal, with differing payload positions
    or members are not, and the pair count is one plus the payload pairs.
  lines: '56'
  artifact_type: FR criterion
- id: FR-379-AC-5
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A union against a union of another declaration, an enum member and a composite refuses `ValueKindMismatch`
    in `plan_equality` and gives `None` from `compare_keys`. One member with payload lengths 1 and 2 refuses `ValueKindMismatch`
    and gives `None`.
  lines: '57'
  artifact_type: FR criterion
- id: FR-379-AC-6
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Members `Alpha` and `Zulu` order by identifier whatever their digests and payloads. One member orders
    by the payload positions in order. Declaration order and digest order do not reach the order. Equal identifiers
    with different `VariantId`s give `None`.
  lines: '58'
  artifact_type: FR criterion
- id: FR-379-AC-7
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`Set`, `Bag`, `OrderedSet` and `Sequence` over four unions, from a supply order and its reverse, hold
    the stated elements. A union with an IEEE payload refuses `CanonicalKeyUnavailable` through `form_grouped` for
    a `Set` and is kept by a `Sequence`.'
  lines: '59'
  artifact_type: FR criterion
- id: FR-379-AC-8
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Clone, equality, key comparison, `Debug` and drop of a union nested ten thousand levels complete on a
    512 KiB stack, a mixed union, composite and collection nest does the same, storage grows linearly with depth,
    and a shared child counts each occurrence.
  lines: '60'
  artifact_type: FR criterion
- id: FR-379-AC-9
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A present `Option` of declared type `Composite(K)` over a union of declaration `K` is admitted, two such
    `Option` values with equal unions are equal and with different members are not, and they compare by the key
    of the union.
  lines: '61'
  artifact_type: FR criterion
- id: FR-380-AC-1
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: With an authenticated B, Integer and reduced Rational membership agree with the inclusive existing rule
    at both endpoints, one outside each endpoint and a denominator boundary, without mutating operands.
  lines: '115'
  artifact_type: FR criterion
- id: FR-380-AC-2
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Exact-B value and every exact-B endpoint are accepted for comparison; one-over-B in each of the three
    Integer or six Rational operand positions returns typed unsupported size naming that position and measured bits
    before any comparison. A value-only guard mutant fails an endpoint case.
  lines: '116'
  artifact_type: FR criterion
- id: FR-380-AC-3
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Observed production comparison steps do not exceed 2L for Integer or 4L for Rational, with fixed-width
    per-step work and constant auxiliary storage on the allocation-free path; source inspection confirms magnitude
    checks and borrowed limb iteration do not hide input-sized work/allocation.
  lines: '117'
  artifact_type: FR criterion
- id: FR-380-AC-4
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: A pre-cancelled original handle prevents input-sized work. Deterministically cancelling during a longest
    admitted comparison returns that cause before the next step/result; a fresh-handle mutant fails.
  lines: '118'
  artifact_type: FR criterion
- id: FR-380-AC-5
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Inspection identifies whether the production comparison actually performs a fallible native conversion
    or storage request. For each reachable boundary, a denied checked conversion is typed capacity/resource refusal
    and an actual reservation denial carries its measured request; neither becomes false, unsupported size or traversal
    Limit, and retry after removing allocation denial agrees with a fresh run. When neither boundary exists, no
    artificial capacity/allocator refusal fixture or ceiling is required.
  lines: '119'
  artifact_type: FR criterion
- id: FR-380-AC-7
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: 'Inspection of both direct exact-kernel `IntegerInterval::try_contains_bounded` and `RationalDomain::try_contains_bounded`
    signatures verifies that authenticated finite B is a required `magnitude_bits: u64` parameter, applied to the
    value and every endpoint, and neither helper selects a default B. A call omitting B is not part of either helper''s
    API.'
  lines: '120'
  artifact_type: FR criterion
- id: FR-381-AC-1
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: With authenticated B, equal and unequal Integer intervals and Rational domains return the same Boolean
    equality as independent endpoint arithmetic, including negative, zero, and denominator endpoints, without mutation.
    Equal spellings or identity of borrowed objects do not substitute for endpoint equality.
  lines: '82'
  artifact_type: FR criterion
- id: FR-381-AC-2
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: At B >= 1, every one of the four Integer and eight Rational endpoint positions completes at exact B and
    returns typed unsupported size when made one bit over B, before endpoint comparison. Two oversized endpoints
    report the first in the stated expected-before-actual order. At B = 0, even zero (one magnitude bit) is refused
    at the first expected endpoint.
  lines: '83'
  artifact_type: FR criterion
- id: FR-381-AC-3
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: Observed production comparisons use no more than 2L Integer or 4L Rational limb-pair steps, fixed-width
    work per step, and the measured temporary-storage bound, including inline/Large and 32-bit limb boundaries.
    Inspection finds no input-sized clone or unbounded fallback.
  lines: '84'
  artifact_type: FR criterion
- id: FR-381-AC-4
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: A pre-cancelled original handle stops before bound validation, including when the first endpoint exceeds
    B; cancellation on a late observed limb step stops before the next step or equality result with the original
    cause.
  lines: '85'
  artifact_type: FR criterion
- id: FR-381-AC-5
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: Source inspection establishes whether production performs native-size conversion or allocation. For each
    reachable boundary, an executed denial/retry test establishes the typed measured request and retry result. An
    allocation-free route with no failing conversion has no synthetic resource fixture.
  lines: '86'
  artifact_type: FR criterion
- id: FR-382-AC-1
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: Equal distinct Decimal types complete `true`; a difference in either signed endpoint, either scale or
    rounding mode completes `false` under authenticated `B`. Endpoint equality is mathematical integer equality.
    Borrowed inputs remain unchanged.
  lines: '83'
  artifact_type: FR criterion
- id: FR-382-AC-2
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: At `B >= 1`, every endpoint position completes at exact `B` and returns typed `UnsupportedSize` with
    the correct position, measured `B+1` bits and `B` when one bit over. Two oversized endpoints report the first
    in expected-lower, expected-upper, actual-lower, actual-upper order, even if an earlier retained field differs.
    At `B = 0`, zero is refused at expected lower with measured one bit.
  lines: '84'
  artifact_type: FR criterion
- id: FR-382-AC-3
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: Observed production comparisons at inline/Large and 32-bit limb boundaries perform at most `2 ceil(B/32)`
    limb-pair steps of fixed-width work and no input-sized clone or allocation; auxiliary storage is constant. Source
    inspection finds no hidden unbounded fallback or reachable storage/capacity failure.
  lines: '85'
  artifact_type: FR criterion
- id: FR-382-AC-4
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: A pre-cancelled original handle stops before bound validation, including when expected lower exceeds
    `B`; cancellation observed after a late limb step stops before another step or Boolean publication with the
    original cause. A fresh handle completes the same operands.
  lines: '86'
  artifact_type: FR criterion
- id: FR-383-AC-1
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: '`Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`, `cancel()` is `Some(&Cancel)`
    equal by shared-state identity to `handle`, and unequal to an independently created handle. Replacing the handle
    reports only the replacement.'
  lines: '47'
  artifact_type: FR criterion
- id: FR-383-AC-2
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: 'Cancel the handle obtained from `meter.cancel()` with `Requested`, then charge the meter under sufficient
    limits: the charge is denied at its named point, that handle records `tripped() = Some(Requested)`, and all
    accounting state stays unchanged. On a separate fresh meter, cancel an independent handle while its installed
    handle remains live: an otherwise identical charge is admitted.'
  lines: '48'
  artifact_type: FR criterion
- id: FR-383-AC-3
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The accessor's implementation borrows the existing optional handle without cloning it or constructing
    a second `Meter` or `Counters`; calling it changes no meter accounting or injection state.
  lines: '49'
  artifact_type: FR criterion
- id: FR-383-AC-4
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std` + `alloc`
    build.
  lines: '50'
  artifact_type: FR criterion
- id: inventory:src/accounting/kani_proofs.rs
  path: src/accounting/kani_proofs.rs
  role: examined
  excerpt: '//! Bounded charge-level obligation for [`Meter::charge`] over the integer

    //! arithmetic schedule.

    //!

    //! The reference model below shares no code with the meter: it names each of

    //! the ten `ScalarLimits` fields itself, checks them in an unrolled chain and

    //! keeps its own counters. It asserts nothing about the diagnostic charge log,

    //! which only exists under the `test-support` feature.


    use super::*;


    /// One semantic-size request: its counter and its amount.'
  inventory_lines: 292
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/accounting.rs
  path: src/accounting.rs
  role: examined
  excerpt: '//! `quire.value.accounting/v1`: `u64` limits, named ordered charge points and

    //! the exact incomplete record.

    //!

    //! Semantic-size counters record a high-water maximum; `work_units` and

    //! `result_units` are cumulative. Every charge is atomic: the first

    //! unavailable counter, in the order written in its definition row, returns

    //! [`Incomplete`] and nothing is consumed.

    //!

    //! Every type here references only [`crate::integer::Integer`], which is itself

    //! a kernel type. `ScalarLimits` does not derive `serde::Serialize`/`Deserialize`:

    //! the kernel takes no wire dependency, a'
  inventory_lines: 2014
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/cancel.rs
  path: src/cancel.rs
  role: examined
  excerpt: '//! Cancellation of a lifecycle operation.

    //!

    //! [`Cancel`] is a cloneable handle the caller owns. Any clone can cancel it

    //! from any thread, with a [`CancelCause`]. The library reads no clock: a

    //! frontend that owns a timer cancels with [`CancelCause::Deadline`].

    //!

    //! A work meter given the handle ([`crate::Meter::with_cancel`]) calls

    //! [`Cancel::poll`] at every charge. A cancelled handle denies the charge as

    //! an exhausted budget does, and the handle records that a charge saw it

    //! ([`Cancel::tripped`]), so the operation can report its cancellation

    //! rather than the'
  inventory_lines: 307
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/collection.rs
  path: src/collection.rs
  role: examined
  excerpt: '//! Bounded collection kinds: metered construction and the canonical order.

    //!

    //! A collection type `K<T>[min, max]` includes its bound, and `K<T>` with no

    //! bound is a different, unbounded type. An

    //! unbounded collection''s construction never refuses for cardinality: it

    //! still charges `collection.bound`, and stops only when the caller''s meter

    //! does. Construction

    //! charges the `quire.value.accounting/v1` collection family: one

    //! `collection.element` before each element expression; for a set, bag or

    //! ordered set, one `collection.member-walk` and `collection.member-te'
  inventory_lines: 682
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/comparison.rs
  path: src/comparison.rs
  role: examined
  excerpt: '//! Scalar comparison operators and the type-checking refusals that precede

    //! evaluation.

    //!

    //! An [`IllTyped`] comparison is `refused { code: ill_typed }` at type checking;

    //! it is not a peer [`crate::outcome::Outcome`] and consumes no charge.

    //!

    //! [`ComparisonOperator::holds`] is `pub`: it is a pure decision over an

    //! already-computed `Ordering`, so exposing it exposes no unmetered

    //! computation.


    use core::cmp::Ordering;'
  inventory_lines: 168
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/decimal.rs
  path: src/decimal.rs
  role: examined
  excerpt: '//! QSpec-FR-140 exact decimals: coefficient/scale values, the six rounding spellings,

    //! canonical-rational loss records and metered evaluation.

    //!

    //! Every intermediate is an exact integer or reduced rational; no binary

    //! floating-point value exists anywhere on this path.

    //!

    //! [`DecimalType::placement`], [`Placement::materialize`], [`Placed::admit`]

    //! and [`Admitted`] place an exact rational at a decimal target''s scale for a

    //! caller that meters the placement itself, such as a unit

    //! conversion into a `Decimal` or integer target, which charges

    //! `unit.target-domain`'
  inventory_lines: 1368
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/decimal_membership.rs
  path: src/decimal_membership.rs
  role: examined
  excerpt: '//! FR-372 fallible Decimal membership over borrowed normalized values.

    //!

    //! Endpoint division retains the Euclidean remainder instead of materializing

    //! a shifted coefficient. One reusable scratch request holds at most `4L`

    //! bytes, where `L = ceil(b / 32)` for the largest operand magnitude bit length.

    //! Copy, division and comparison each poll the original handle before every

    //! limb step. Allocator calls themselves have no elapsed-time bound.


    use core::alloc::Layout;

    use core::cmp::Ordering;

    use core::mem::size_of;'
  inventory_lines: 614
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/division.rs
  path: src/division.rs
  role: examined
  excerpt: '//! Integer division: the three `div`/`rem` laws, independent Euclidean

    //! `mod`, and single-member admission (the domain applies only to the member

    //! the expression exposes) with the named integer-division and

    //! integer-modulus charges.

    //!

    //! Whether a *backend* can execute a division item at all is decided ahead of

    //! and independent of evaluation; that capability negotiation is not a kernel

    //! operation and is not here. [`divide`] takes the selected [`DivisionProfile`]

    //! directly.


    use crate::accounting::{Charge, ChargePoint, LimitKind, Meter};'
  inventory_lines: 464
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/equality.rs
  path: src/equality.rs
  role: examined
  excerpt: '//! Occurrence-pair equality planning.

    //!

    //! The module holds the operand-agnostic occurrence-pair plan: [`plan_pairs`],

    //! [`plan_equality`], `planned_equality`, [`EqualityPlan`] and

    //! [`EqualityEvaluation`]. Anything that

    //! needs a declaration registry or a unit graph is the caller''s, not the

    //! kernel''s: selecting and running the top-level text/enum/quantity schedules

    //! ahead of the generic occurrence-pair plan is done above the kernel with the

    //! registry and unit graph the caller holds. An enum''s ordering and case lookup

    //! needs no declaration here, since `ValueType'
  inventory_lines: 694
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/identity.rs
  path: src/identity.rs
  role: examined
  excerpt: '//! Value-component identities.

    //!

    //! Seven identities appear as bare payload inside kernel `Value`s:

    //! `EffectiveId` (declaration identity), `UniverseId` and `ObjectId` (a

    //! `Reference` payload), `UnitId` (a `Quantity` payload), `VariantId` (an `Enum`

    //! payload), `MemberId` and `PopulationId` (a `Population` payload). Five of the

    //! seven -- `EffectiveId`, `UniverseId`, `VariantId`, `MemberId`,

    //! `PopulationId` -- are single-domain opaque 32-byte digests with exactly one

    //! public constructor, `from_digest` (each type''s own, e.g.

    //! [`EffectiveId::from_digest`]), mirror'
  inventory_lines: 422
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/ieee.rs
  path: src/ieee.rs
  role: examined
  excerpt: '//! `quire.value.ieee754-2019-default/v1`: binary32/binary64 bit-pattern

    //! values, the five IEEE rounding directions plus strict `exact`,

    //! deterministic NaN propagation, operation-local exception flags, the

    //! three distinct comparison intrinsics and explicit conversions.

    //!

    //! Every value is an exact bit pattern. Arithmetic decodes finite operands

    //! into exact dyadic integers, forms the exact real intermediate with

    //! arbitrary precision integers (a truncated integer square root plus a

    //! sticky bit for `sqrt`) and rounds exactly once. No host floating-point

    //! value or'
  inventory_lines: 2275
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/integer.rs
  path: src/integer.rs
  role: examined
  excerpt: '//! Mathematical integers and explicit inclusive integer domains (QSpec FR-147).

    //!

    //! `Integer` is unbounded. A finite consumer never narrows it: membership in an

    //! [`IntegerInterval`] is an explicit admission that either returns a

    //! [`BoundedInteger`] or refuses.

    //!

    //! Seventeen `Integer`/`IntegerInterval` methods are `pub` so a caller above this

    //! crate can use them: [`abs`](Integer::abs), [`pow`](Integer::pow),

    //! [`split_factor_two`](Integer::split_factor_two),

    //! [`shifted_left`](Integer::shifted_left), [`add`](Integer::add),

    //! [`sub`](Integer::sub), [`mul`](Integ'
  inventory_lines: 988
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/key.rs
  path: src/key.rs
  role: examined
  excerpt: '//! The type-owned total canonical key.

    //!

    //! Every type that admits `=` has a key, and two values of one type have

    //! equal keys exactly when they are equal under [`crate::equality`]. The key

    //! fixes set and bag canonical order and visiting order; it is not an

    //! ordering operator. Comparison is iterative, so value depth never reaches

    //! the host stack.

    //!

    //! Two composites of different declarations are unequal under the equality

    //! plan, so their keys differ: they order by the ascending bytes of the two

    //! 32-byte declaration keys, before any slot. The order is total and'
  inventory_lines: 345
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: '//! `quire-exact`: the exact-value kernel, a `no_std` + `alloc` leaf that

    //! depends on no other crate in the quire ecosystem.

    //!

    //! The crate holds checked identity ([`NodeKey`] and the seven opaque digest

    //! identities, such as [`EffectiveId`]), provenance ([`Location`]), kernel

    //! outcomes and refusals ([`Outcome`], [`Refusal`]), bounds and accounting

    //! ([`Meter`], [`BoundedInteger`], [`CardinalityBound`]), and the exact

    //! semantic value kernel ([`Value`]/[`ValueType`] and every value-family

    //! module it composes: `numeric`, `rational`, `decimal`, `text`, `ieee`,

    //! `di'
  inventory_lines: 162
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/location.rs
  path: src/location.rs
  role: examined
  excerpt: '//! Source occurrence identity: `Origin` and `Location`.

    //!

    //! The kernel provenance shape names a node id and an occurrence key and carries

    //! no bytes. The occurrence key is `(NodeKey, role, ordinal)`. `Origin` and

    //! `Location` say which source occurrence of which checked node a value or

    //! outcome traces back to.

    //!

    //! `Role` is a lexical-string newtype rather than a closed enum: the set of

    //! occurrence roles ("declaration", "reference", "default", ...) is the caller''s

    //! preimage schema''s to define and extend, and the kernel takes no position on

    //! which roles exist -'
  inventory_lines: 158
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/node.rs
  path: src/node.rs
  role: examined
  excerpt: '//! Checked semantic node identity.

    //!

    //! A `NodeKey` is an opaque 32-byte digest in domain

    //! `quire.checked-semantic-node/v1`. Node ids are content-addressed over the

    //! preimage the caller defines: equal keys mean

    //! structurally identical nodes.

    //!

    //! Minting rule: the kernel `NodeKey` has one minting constructor,

    //! [`NodeKey::from_digest`], which takes an already-computed digest. It does not

    //! hash: the preimage schema, RFC 8785 JCS canonicalization and the SHA-256

    //! computation over that canonical form all stay with the caller, who passes the'
  inventory_lines: 208
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/numeric.rs
  path: src/numeric.rs
  role: examined
  excerpt: '//! Metered exact numeric kernels: `Integer`/`Int[..]` arithmetic,

    //! `Rational[..]` arithmetic, numeric ordering and Boolean connectives, in the

    //! `quire.value.accounting/v1` charge order.

    //!

    //! Every size amount is derived before the value it measures is retained; no

    //! power of ten is allocated to measure an aligned decimal coefficient.

    //!

    //! [`ArithmeticOperator`] is `pub` because

    //! [`crate::rational::RationalDomain::result_of`] (a scalar operation this crate

    //! exposes to its consumers) names it in a public signature.

    //!'
  inventory_lines: 472
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/numeric_membership.rs
  path: src/numeric_membership.rs
  role: examined
  excerpt: '//! Bounded, cancellable membership for already constructed exact numbers (FR-380).

    //!

    //! The caller authenticates a finite magnitude ceiling for the value and every

    //! endpoint. The kernel checks that ceiling before comparing borrowed limbs.

    //! There is no native limb-count conversion, scratch storage, or allocation.


    use core::cmp::Ordering;


    use crate::{Cancel, CancelCause, Integer, IntegerInterval, Rational, RationalDomain};


    /// The position of an integer checked by bounded numeric membership.'
  inventory_lines: 582
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/numeric_type_equality.rs
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: '//! Bounded, cancellable equality of retained numeric type domains (FR-381).

    //!

    //! The caller authenticates a finite magnitude ceiling for every endpoint.

    //! Borrowed sign, bit length and limb iteration use constant auxiliary storage;

    //! neither a native-size conversion nor an allocation is needed.


    use crate::{Cancel, CancelCause, DecimalType, Integer, IntegerInterval, RationalDomain};


    /// The position of a retained numeric type endpoint.

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]

    pub enum NumericTypeEndpoint {'
  inventory_lines: 927
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/outcome.rs
  path: src/outcome.rs
  role: examined
  excerpt: '//! Kernel outcomes: `Outcome<T>`, `Undefined`, `Refusal`, `Stop`.

    //!

    //! A false Boolean is a completed value, never a refusal. `requires-bound`

    //! and `unsupported` are per-item provider dispositions, not evaluator

    //! outcomes, so they have no variant here.

    //!

    //! The kernel `Refusal` carries only its own typed cause. A cause whose payload

    //! is not a kernel type has no variant here:

    //!

    //! - a closed cause set for a `pre(..)` anchor mismatch is an evaluator concept,

    //!   not a kernel one.'
  inventory_lines: 571
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/quantity.rs
  path: src/quantity.rs
  role: examined
  excerpt: '//! Bare quantity values: a magnitude in a unit, with no unit graph.

    //!

    //! A quantity is a `(magnitude, UnitId)` pair with no reference to quantity

    //! declarations. No unit graph (dimension, composed conversion factors and

    //! offsets, edge traversal) is a kernel type; it belongs to the caller''s unit

    //! declarations. The kernel `Quantity` therefore only supports operations

    //! between quantities already in the *same* unit; converting first to a common

    //! unit, when the units differ but are compatible, is the caller''s job, done

    //! above this module with the unit graph it holds.

    '
  inventory_lines: 222
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/rational.rs
  path: src/rational.rs
  role: examined
  excerpt: '//! Canonical exact rationals (QSpec FR-140 loss records).

    //!

    //! `Rational`''s arithmetic (`add`/`sub`/`neg`/`mul`/`div`/`pow`,

    //! `divided_by_power_of_ten`/`divided_by_power_of_two`) is unmetered and

    //! exact, the same discipline [`crate::integer::Integer`]''s own arithmetic

    //! carries: a caller charges `quire.value.accounting/v1` before calling it.

    //! `divided_by_power_of_ten`/`divided_by_power_of_two` take a

    //! caller-supplied exponent with no bound of their own, so an uncharged call

    //! can build an arbitrarily large `BigInt`; callers bound or charge the

    //! exponent first, '
  inventory_lines: 384
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/reference.rs
  path: src/reference.rs
  role: examined
  excerpt: '//! Terminal `Reference<T>` values.

    //!

    //! A reference is terminal: its identity is the triple of `EffectiveId` (the

    //! object type), `UniverseId` and `ObjectId`, and equality never inspects

    //! referenced state. `EffectiveId` and `UniverseId` are opaque digests minted by

    //! the caller from its own preimage; `ObjectId` is the object''s own authored

    //! UTF-8 bytes, never a digest. Checking a reference against a declaration

    //! registry is not a kernel concern; a caller does it.


    use crate::identity::{EffectiveId, ObjectId, UniverseId};

    '
  inventory_lines: 75
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/text.rs
  path: src/text.rs
  role: examined
  excerpt: '//! QSpec-FR-141 text: validated UTF-8 payloads, the six text profiles, bounded

    //! `Text[min,max; profile]` admission and metered comparison.

    //!

    //! Normalization uses exactly the Unicode 17.0.0 data selected by

    //! `quire.value.text.unicode-17.0.0/v1`. The build refuses any other

    //! `unicode-normalization` table version. No locale, case folding, collation

    //! or grapheme segmentation is applied.

    //!

    //! Decoding a source-lexer literal spelling''s escapes is a source-stage

    //! concern, not a value concern, so it is not here: the caller decodes a

    //! literal''s escapes before handing'
  inventory_lines: 621
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/value/native.rs
  path: src/value/native.rs
  role: examined
  excerpt: '//! Canonical native UUID and POSIX-epoch nanosecond values.


    use alloc::string::ToString;

    use core::fmt::{self, Write};


    use super::{Component, ConstructionCause, ConstructionRefusal};


    /// An opaque UUID represented by its 16 network-order octets.

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]

    pub struct Uuid([u8; 16]);

    '
  inventory_lines: 228
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/value/value_type.rs
  path: src/value/value_type.rs
  role: examined
  excerpt: '//! `ValueType`''s structural traits, walked without native recursion.

    //!

    //! A `ValueType` nests through exactly two variants, `Option` and

    //! `Collection`, and each holds one nested type behind an `Arc`, so a type

    //! is a chain whose links are shared: `Clone` is one `Arc` increment and a

    //! value of any depth keeps one node per level, not one copy of the chain

    //! below it. `PartialEq` and `Hash` follow the chain with a cursor and need

    //! no stack at all: each step handles one link''s own data

    //! (`ValueType::shallow_eq`, `ValueType::shallow_hash`) and moves to

    //! `ValueType::'
  inventory_lines: 584
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:src/value.rs
  path: src/value.rs
  role: examined
  excerpt: '//! Exact kernel `Value`/`ValueType` and composite construction.

    //!

    //! Every kernel-crossing payload is trimmed to the bare id(s) it needs. The

    //! declaration *registry* (duplicate-key checks, the recursion rule,

    //! generalization, `Reference<T>` target lookup) is the caller''s, not the

    //! kernel''s.

    //!

    //! - `ValueType::Enum(EnumShape)`: the shape carries its admitted variants

    //!   inline, as a canonically ranked list of opaque `VariantId` digests, so

    //!   `ValueType::admits` needs no declaration lookup at all -- membership and

    //!   rank agreement are checked against the shap'
  inventory_lines: 2901
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/checked_invariant.rs
  path: tests/checked_invariant.rs
  role: examined
  excerpt: "//! Public checked-invariant carrier controls for FR-369.\n\nuse quire_exact::{CheckedInvariantCause,\
    \ IllTypedCause, Refusal};\n\n// Independently authored exhaustive expectations, rather than a production list.\n\
    fn expected_name(cause: CheckedInvariantCause) -> &'static str {\n    match cause {\n        CheckedInvariantCause::CollectionElementNotAdmitted\
    \ => \"collection-element\",\n        CheckedInvariantCause::DeferredResultNotAdmitted => \"deferred-result\"\
    ,\n        CheckedInvariantCause::CanonicalKeyUnavailable => \"canonical-key\",\n        CheckedInvariantCause::BoundedDivisionExpected\
    \ => \"b"
  inventory_lines: 382
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/composite_declarations.rs
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "//! Composite values of different declarations: equality (FR-376), canonical\n//! key (FR-377) and the\
    \ caller-admitted paths (FR-378).\n\nuse std::cmp::Ordering;\n\nuse ix_trace_rs::trace;\nuse quire_exact::{\n\
    \    compare_keys, construct_collection, evaluate_record, form, form_collection, form_grouped,\n    from_admitted_slots,\
    \ member_equal, plan_equality, planned_equality, planned_equality_with_plan,\n    retain_composite, CheckedInvariantCause,\
    \ CollectionKind, CollectionType, CollectionValue,\n    Component, ConstructionCause, ConstructionRefusal, Deferred,\
    \ EffectiveId, FieldDeclaration,"
  inventory_lines: 776
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/fixtures/unicode17_consumer.rs
  path: tests/fixtures/unicode17_consumer.rs
  role: examined
  excerpt: "//! Exercise the actual owner's public Unicode profile from an ordinary consumer.\n\nuse quire_exact::{NormalizationForm,\
    \ TextProfile, UNICODE_TEXT_DEFINITION, UNICODE_VERSION};\n\nfn main() {\n    assert_eq!(UNICODE_VERSION, (17,\
    \ 0, 0));\n    assert_eq!(\n        UNICODE_TEXT_DEFINITION,\n        \"quire.value.text.unicode-17.0.0/v1\"\
    \n    );\n    for (profile, form, input, expected) in ["
  inventory_lines: 36
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/fresh_unicode_consumer.rs
  path: tests/fresh_unicode_consumer.rs
  role: examined
  excerpt: "//! Fresh dependency resolution must preserve QSpec-FR-141's Unicode 17 profile.\n\nuse std::{fs, path::Path,\
    \ process::Command};\n\n/// Trace: FR-141-AC-1\n/// Provenance: IR-935, QSpec TC-186\n#[test]\nfn fresh_consumer_resolves_and_runs_unicode17_normalization()\
    \ {\n    let owner = Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    let directory = owner\n        .join(\"target\"\
    )"
  inventory_lines: 50
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir653_admission.rs
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "//! Denied size charges precede large exact allocation requests (FR-361).\n//!\n//! `allocation_counter`\
    \ supplies the observer outside this unsafe-free crate.\n//! Total requested bytes bound every single request,\
    \ so this test asserts the\n//! stronger total bound in each measurement window.\n\nuse core::num::NonZeroU64;\n\
    \nuse quire_exact::{\n    divide, evaluate_decimal, evaluate_integer_arithmetic, evaluate_rational_arithmetic,\
    \ modulo,\n    ChargePoint, Decimal, DecimalOperation, DecimalType, DivisionMember, DivisionProfile,"
  inventory_lines: 361
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir653_decimal_order.rs
  path: tests/ir653_decimal_order.rs
  role: examined
  excerpt: "//! Retained decimal ordering evidence for FR-363.\n\nuse core::num::NonZeroU64;\n\nuse quire_exact::{\n\
    \    order_numbers, ChargePoint, Decimal, InjectedDenial, Integer, LimitKind, Meter,\n    OrderedOperands, OrderingOperator,\
    \ Outcome, ScalarLimits,\n};\n\nfn limits() -> ScalarLimits {\n    ScalarLimits {"
  inventory_lines: 223
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir653_scalar.rs
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "//! Independent scalar value and charge evidence for FR-362.\n\nuse core::num::NonZeroU64;\nuse core::str::FromStr;\n\
    \nuse quire_exact::{\n    evaluate_boolean, evaluate_integer_arithmetic, evaluate_rational_arithmetic, order_numbers,\n\
    \    retain_boolean, BooleanConnective, ChargePoint, Incomplete, InjectedDenial, Integer,\n    IntegerArithmetic,\
    \ LimitKind, Meter, OrderedOperands, OrderingOperator, Outcome, Rational,\n    RationalArithmetic, ScalarLimits,\
    \ Undefined,\n};"
  inventory_lines: 638
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir667_normalize.rs
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: "//! Actual admission sizes and independent scalar boundary controls.\n#![cfg(feature = \"test-support\"\
    )]\n\nuse core::num::NonZeroU64;\n\nuse quire_exact::{\n    evaluate_integer_arithmetic, evaluate_rational_arithmetic,\
    \ order_numbers, AdmittedCharge,\n    Cancel, CancelCause, Charge, ChargePoint, Incomplete, InjectedDenial,\
    \ Integer,\n    IntegerArithmetic, LimitKind, Meter, OrderedOperands, OrderingOperator, Outcome, Rational,\n\
    \    RationalArithmetic, ScalarLimits,\n};"
  inventory_lines: 655
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir673_core.rs
  path: tests/ir673_core.rs
  role: examined
  excerpt: "//! Public collection, division, meter and vocabulary evidence for IR-673.\n\nuse std::{collections::HashSet,\
    \ num::NonZeroU64};\n\nuse ix_trace_rs::trace;\nuse quire_exact::{\n    divide, form_collection, modulo, BoundViolation,\
    \ CardinalityBound, Charge, ChargePoint,\n    CollectionKind, CollectionType, DivisionMember, DivisionProfile,\
    \ InjectedDenial, Integer,\n    IntegerDomain, LimitKind, Meter, Outcome, Refusal, ScalarLimits, Value, ValueType,\n\
    };\n"
  inventory_lines: 660
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir673_decimal.rs
  path: tests/ir673_decimal.rs
  role: examined
  excerpt: "//! Retained decimal comparison and rounding evidence for IR-673.\n\nuse std::cmp::Ordering;\n\nuse\
    \ quire_exact::{\n    evaluate_decimal, order_numbers, Decimal, DecimalOperation, DecimalType, InexactTarget,\n\
    \    Integer, LimitKind, Meter, OrderedOperands, OrderingOperator, Outcome, Refusal, RoundingMode,\n    ScalarLimits,\n\
    };\n\nfn limits() -> ScalarLimits {"
  inventory_lines: 159
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir673_ieee_text.rs
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "//! Public IEEE and text owner evidence for IR-673.\n\nuse quire_exact::{\n    admit_text, compare_ieee,\
    \ compare_text, convert_ieee_width, evaluate_ieee, ieee_to_exact,\n    ComparisonOperator, IeeeComparison, IeeeExactLoss,\
    \ IeeeFlag, IeeeFlags, IeeeOperation,\n    IeeeValue, IeeeWidth, IllTyped, IllTypedCause, Integer, IntegerInterval,\
    \ LimitKind, Meter,\n    Outcome, Rational, RationalDomain, Refusal, RoundingMode, ScalarLimits, TextPayload,\n\
    \    TextProfile, TextType,\n};\n\nfn limits() -> ScalarLimits {"
  inventory_lines: 300
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir673_scalar.rs
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "//! Public scalar boundary and direct atom evidence for IR-673.\n\nuse std::num::NonZeroU64;\n\nuse\
    \ quire_exact::{\n    evaluate_integer_arithmetic as integer_op, evaluate_rational_arithmetic as rational_op,\n\
    \    order_numbers, ChargePoint, InjectedDenial, Integer, IntegerArithmetic, IntegerInterval,\n    LimitKind,\
    \ Meter, OrderedOperands, OrderingOperator, Outcome, Rational, RationalArithmetic,\n    RationalDomain, Refusal,\
    \ ScalarLimits, Undefined,\n};\n"
  inventory_lines: 746
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir705_bounded_integer.rs
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "//! Integer representation refinement observations (FR-371).\n\nuse core::num::{NonZeroU32, NonZeroU64};\n\
    use num_bigint::BigInt;\nuse std::{str::FromStr, string::ToString};\n\n#[cfg(not(feature = \"test-support\"\
    ))]\nuse quire_exact::Charge;\nuse quire_exact::{\n    evaluate_integer_arithmetic, ChargePoint, InjectedDenial,\
    \ Integer, IntegerArithmetic,\n    IntegerInterval, LimitKind, Meter, Outcome, ScalarLimits,"
  inventory_lines: 642
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir873_temporal_visit.rs
  path: tests/ir873_temporal_visit.rs
  role: examined
  excerpt: "//! Public charge vocabulary and meter evidence for temporal visits.\n\nuse std::num::NonZeroU64;\n\n\
    use ix_trace_rs::trace;\nuse quire_exact::{\n    Charge, ChargePoint, Incomplete, InjectedDenial, Integer, LimitKind,\
    \ Meter, ScalarLimits,\n};\n\nfn limits(work_units: u64) -> ScalarLimits {\n    ScalarLimits {"
  inventory_lines: 114
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/ir895_meter_cancel.rs
  path: tests/ir895_meter_cancel.rs
  role: examined
  excerpt: "//! Public cancellation-handle controls for FR-383.\n\nuse ix_trace_rs::trace;\nuse quire_exact::{\n\
    \    Cancel, CancelCause, Charge, ChargePoint, Incomplete, Integer, LimitKind, Meter, ScalarLimits,\n};\n\n\
    fn limits() -> ScalarLimits {\n    ScalarLimits {\n        integer_bits: u64::MAX,\n        decimal_digits:\
    \ u64::MAX,"
  inventory_lines: 91
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/nonzero_integer.rs
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "//! Public borrowed-divisor and metered preservation evidence for FR-374.\n\nuse core::num::NonZeroU64;\n\
    use core::str::FromStr;\n\nuse quire_exact::{\n    divide, modulo, ChargePoint, DivisionMember, DivisionProfile,\
    \ Incomplete, InjectedDenial,\n    Integer, IntegerDomain, IntegerInterval, LimitKind, Meter, NonZeroInteger,\
    \ Outcome, Refusal,\n    ScalarLimits, Undefined, ZeroDivisor,\n};\n"
  inventory_lines: 405
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/union.rs
  path: tests/union.rs
  role: examined
  excerpt: "//! Kernel union construction and traversal controls. These exercise trusted\n//! kernel seams, not\
    \ package admission, SV registry eligibility or v2 transport.\n\nuse std::cmp::Ordering;\nuse std::fmt::{self,\
    \ Write};\nuse std::sync::Arc;\n\nuse ix_trace_rs::trace;\nuse quire_exact::{\n    compare_keys, evaluate_union,\
    \ form_collection, form_grouped, from_admitted_slots,\n    plan_equality, planned_equality, union, ChargePoint,\
    \ CheckedInvariantCause, CollectionKind,"
  inventory_lines: 648
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: inventory:tests/union_declarations.rs
  path: tests/union_declarations.rs
  role: examined
  excerpt: "//! A union member payload follows the composite rules across declarations:\n//! equality (FR-376-AC-9),\
    \ canonical key (FR-377-AC-8) and the caller-admitted\n//! path (FR-378-AC-8). The payload position names `Composite(P)`\
    \ and holds a\n//! `P` or a descendant-declared `C`.\n\nuse std::cmp::Ordering;\n\nuse ix_trace_rs::trace;\n\
    use quire_exact::{\n    compare_keys, evaluate_union, form, from_admitted_slots, plan_equality, planned_equality,\n\
    \    union, CheckedInvariantCause, CollectionKind, CollectionType, Component, ConstructionCause,"
  inventory_lines: 336
  basis: Whole repository ownership/header, trace and stub inventory; no full semantic implementation review claimed.
- id: src/key.rs#tests::compare_keys_orders_members_of_one_shape_by_rank
  path: src/key.rs
  role: examined
  excerpt: "    /// Two members of one shape order by their rank, whatever the order of\n    /// their `VariantId`\
    \ digests.\n    #[trace(\"TC-409\", \"FR-088-AC-11\")]\n    #[test]\n    fn compare_keys_orders_members_of_one_shape_by_rank()\
    \ {\n        use crate::identity::VariantId;\n        use crate::value::EnumMember;\n"
  lines: '304'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/key.rs#tests::compare_keys_refuses_same_rank_different_declaration_members
  path: src/key.rs
  role: examined
  excerpt: "    /// `left.rank().cmp(&right.rank())` makes this return\n    /// `Some(Ordering::Equal)` instead\
    \ of `None`.\n    #[trace(\"TC-409\", \"FR-088-AC-11\")]\n    #[test]\n    fn compare_keys_refuses_same_rank_different_declaration_members()\
    \ {\n        use crate::identity::VariantId;\n        use crate::value::EnumMember;\n"
  lines: '331'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::enum_shape_admits_only_its_own_variants
  path: src/value.rs
  role: examined
  excerpt: "    /// Also FR-088-AC-11: admission checks the whole `(VariantId, rank)` pair,\n    /// not membership\
    \ alone.\n    #[trace(\"TC-409\", \"FR-088-AC-11\")]\n    #[test]\n    fn enum_shape_admits_only_its_own_variants()\
    \ {\n        let in_shape = VariantId::from_digest(digest(1));\n        let out_of_shape = VariantId::from_digest(digest(2));\n\
    \        let shape = ValueType::Enum(EnumShape::new(false, [in_shape]));"
  lines: '1990'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::admits_refuses_a_known_variant_at_the_wrong_rank
  path: src/value.rs
  role: examined
  excerpt: "    /// Also TC-409 step 3 (FR-088-AC-11): a value that pairs a known\n    /// `VariantId` with the\
    \ wrong rank is refused at admission.\n    #[trace(\"TC-409\", \"FR-088-AC-11\")]\n    #[test]\n    fn admits_refuses_a_known_variant_at_the_wrong_rank()\
    \ {\n        let first = VariantId::from_digest(digest(1));\n        let second = VariantId::from_digest(digest(2));\n\
    \        let shape = ValueType::Enum(EnumShape::new(true, [first, second]));"
  lines: '2008'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::enum_shape_rank_matches_canonical_position
  path: src/value.rs
  role: examined
  excerpt: "    /// Also TC-409 steps 1 and 2 (FR-088-AC-11): rank is the canonical-list\n    /// position.\n  \
    \  #[trace(\"TC-409\", \"FR-088-AC-11\")]\n    #[test]\n    fn enum_shape_rank_matches_canonical_position()\
    \ {\n        let a = VariantId::from_digest(digest(1));\n        let b = VariantId::from_digest(digest(2));\n\
    \        let c = VariantId::from_digest(digest(3));"
  lines: '2026'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::a_set_visits_enum_values_in_canonical_rank_order
  path: src/value.rs
  role: examined
  excerpt: "    /// identity but not in rank and order give the same ranks and the same\n    /// visiting order.\n\
    \    #[trace(\"TC-409\", \"FR-088-AC-11\")]\n    #[test]\n    fn a_set_visits_enum_values_in_canonical_rank_order()\
    \ {\n        // Declared order `b`, `a`, `c`; the digests order them `a`, `c`, `b`.\n        let (a, b, c) =\
    \ (\n            VariantId::from_digest(digest(1)),"
  lines: '2089'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/identity.rs#tests::tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes
  path: src/identity.rs
  role: examined
  excerpt: "    /// and every declared unit orders before every compound unit\n    /// (`quire.checked-semantic-node/v1`\
    \ < `quire.value.compound-unit/v1`).\n    #[trace(\"FR-088-AC-12\", \"TC-411\")]\n    #[test]\n    fn tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes()\
    \ {\n        let declared = UnitId::declared(NodeKey::from_digest(digest(2)));\n        let compound = UnitId::compound(digest(2));\n\
    \        assert_eq!(declared.as_bytes(), compound.as_bytes());"
  lines: '382'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/equality.rs#tests::plan_pairs_refuses_a_population_pair
  path: src/equality.rs
  role: examined
  excerpt: "    /// TC-297 (FR-089-AC-6): a population pair is not an equality operand\n    /// pair in the kernel.\n\
    \    #[trace(\"TC-297\", \"FR-089-AC-6\", \"FR-369-AC-2\")]\n    #[test]\n    fn plan_pairs_refuses_a_population_pair()\
    \ {\n        use crate::identity::PopulationId;\n\n        fn digest(byte: u8) -> [u8; 32] {"
  lines: '585'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/key.rs#tests::compare_keys_yields_no_key_for_a_population_pair
  path: src/key.rs
  role: examined
  excerpt: "\n    /// TC-297 (FR-089-AC-6): a population pair has no key in the kernel.\n    #[trace(\"TC-297\"\
    , \"FR-089-AC-6\")]\n    #[test]\n    fn compare_keys_yields_no_key_for_a_population_pair() {\n        use crate::identity::PopulationId;\n\
    \n        fn digest(byte: u8) -> [u8; 32] {"
  lines: '275'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::admits_refuses_a_population_pair
  path: src/value.rs
  role: examined
  excerpt: "    /// TC-297 (FR-089-AC-6): kernel `admits` refuses every population pair;\n    /// the declared-maximum\
    \ comparison is the caller layer's.\n    #[trace(\"TC-297\", \"FR-089-AC-6\")]\n    #[test]\n    fn admits_refuses_a_population_pair()\
    \ {\n        assert!(!ValueType::Population(Some(5))\n            .admits(&Value::Population(PopulationId::from_digest(digest(1)))));\n\
    \    }"
  lines: '1977'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/outcome.rs#tests::tc_428_every_record_building_refusal_names_code_and_cause
  path: src/outcome.rs
  role: examined
  excerpt: "    /// code and cause of its row. The public carrier tests cover every\n    /// checked-invariant cause's\
    \ separate `None` mapping.\n    #[trace(\"TC-428\", \"FR-096-AC-8\")]\n    #[test]\n    fn tc_428_every_record_building_refusal_names_code_and_cause()\
    \ {\n        use crate::integer::Integer;\n        use crate::text::TextProfile;\n"
  lines: '440'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/checked_invariant.rs#all_typed_causes_are_distinct_internal_faults_with_required_traits
  path: tests/checked_invariant.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-369-AC-1, FR-369-AC-5, FR-096-AC-8\n#[test]\nfn all_typed_causes_are_distinct_internal_faults_with_required_traits()\
    \ {\n    fn cause_traits<T: Copy + Clone + std::fmt::Debug + Eq + std::hash::Hash + PartialEq>() {}\n    fn\
    \ refusal_traits<T: Clone + std::fmt::Debug + Eq + PartialEq>() {}\n    cause_traits::<CheckedInvariantCause>();"
  lines: '45'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/collection.rs#tests::tc_441_an_unbounded_collection_never_refuses_for_cardinality
  path: src/collection.rs
  role: examined
  excerpt: "    /// cardinality refusal, where the same elements under `[0, 1]` refuse;\n    /// `K<T>` and `K<T>[0,\
    \ u64::MAX]` are different types.\n    #[trace(\"TC-441\", \"FR-097-AC-7\")]\n    #[test]\n    fn tc_441_an_unbounded_collection_never_refuses_for_cardinality()\
    \ {\n        let unbounded = CollectionType::new(CollectionKind::Sequence, ValueType::Integer, None);\n    \
    \    let widest = CollectionType::new(\n            CollectionKind::Sequence,"
  lines: '625'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/collection.rs#tests::tc_441_an_unbounded_collection_stops_only_on_the_callers_meter
  path: src/collection.rs
  role: examined
  excerpt: "    /// point only when the caller's meter runs out, never with a\n    /// cardinality refusal.\n  \
    \  #[trace(\"TC-441\", \"FR-097-AC-7\")]\n    #[test]\n    fn tc_441_an_unbounded_collection_stops_only_on_the_callers_meter()\
    \ {\n        let unbounded = CollectionType::new(CollectionKind::Sequence, ValueType::Integer, None);\n    \
    \    let mut meter = Meter::new(ScalarLimits {\n            value_occurrences: 999,"
  lines: '668'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_core.rs#public_cardinality_query_is_inclusive_and_agrees_with_admission
  path: tests/ir673_core.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-441, FR-097-AC-8\n#[test]\nfn public_cardinality_query_is_inclusive_and_agrees_with_admission()\
    \ {\n    let bound = CardinalityBound::new(2, 4).unwrap();\n    for (count, expected) in [\n        (1, Some(BoundViolation::BelowMinimum)),"
  lines: '30'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::a_deep_value_equals_its_clone_and_both_drop
  path: src/value.rs
  role: examined
  excerpt: "    /// A value nested `DEEP` levels deep compares equal to its clone, and it\n    /// and its clone\
    \ drop, on a 512 KiB stack.\n    #[trace(\"TC-735\", \"FR-262-AC-2\")]\n    #[test]\n    fn a_deep_value_equals_its_clone_and_both_drop()\
    \ {\n        on_small_stack(|| {\n            let (_, value) = deep_value();\n            let clone = value.clone();"
  lines: '2697'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::a_deep_value_drops_on_a_small_stack
  path: src/value.rs
  role: examined
  excerpt: "    /// Dropping a value nested `DEEP` levels deep completes on a 512 KiB\n    /// stack.\n    #[trace(\"\
    TC-735\", \"FR-262-AC-2\")]\n    #[test]\n    fn a_deep_value_drops_on_a_small_stack() {\n        on_small_stack(||\
    \ {\n            let (_, value) = deep_value();\n            drop(value);"
  lines: '2714'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value/value_type.rs#tests::tc_735_a_deep_option_type_clones_compares_hashes_formats_and_drops
  path: src/value/value_type.rs
  role: examined
  excerpt: "    /// hashes equal to its clone, formats for debug and drops, with its\n    /// clone, on a 512 KiB\
    \ stack.\n    #[test]\n    #[trace(\"TC-735\", \"FR-262-AC-2\")]\n    fn tc_735_a_deep_option_type_clones_compares_hashes_formats_and_drops()\
    \ {\n        on_small_stack(|| {\n            let value_type = deep_option();\n            let copy = value_type.clone();"
  lines: '483'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value/value_type.rs#tests::tc_735_value_type_debug_matches_the_derived_format
  path: src/value/value_type.rs
  role: examined
  excerpt: "    /// variant in compact mode, and for both nesting variants in alternate\n    /// mode.\n    #[test]\n\
    \    #[trace(\"TC-735\", \"FR-262-AC-2\")]\n    fn tc_735_value_type_debug_matches_the_derived_format() {\n\
    \        let samples = samples();\n        for (value_type, compact) in samples.iter().zip(DERIVED_COMPACT)\
    \ {\n            assert_eq!(format!(\"{value_type:?}\"), compact);"
  lines: '509'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value/value_type.rs#tests::tc_735_value_types_equal_and_hash_equal_their_clones_only
  path: src/value/value_type.rs
  role: examined
  excerpt: "    /// Every sample compares equal to and hashes equal to its clone, and\n    /// compares unequal\
    \ to and hashes differently from every other sample.\n    #[test]\n    #[trace(\"TC-735\", \"FR-262-AC-2\")]\n\
    \    fn tc_735_value_types_equal_and_hash_equal_their_clones_only() {\n        let samples = samples();\n  \
    \      for (index, value_type) in samples.iter().enumerate() {\n            let copy = value_type.clone();"
  lines: '522'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value/value_type.rs#tests::tc_735_the_debug_worklist_grows_by_a_constant_per_type_node
  path: src/value/value_type.rs
  role: examined
  excerpt: "    /// bound it. Each `Option` link is one node; each `Collection` link is\n    /// two, the `ValueType`\
    \ and its `CollectionType`.\n    #[test]\n    #[trace(\"TC-735\", \"FR-262-AC-2\")]\n    fn tc_735_the_debug_worklist_grows_by_a_constant_per_type_node()\
    \ {\n        /// Formats a type through the worklist and records its peak.\n        struct Peak<'a>(&'a ValueType,\
    \ &'a Cell<usize>);\n"
  lines: '551'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::type_nodes_grow_linearly_with_value_depth
  path: src/value.rs
  role: examined
  excerpt: "    /// twice the type nodes of a value `N` deep: payload types are shared,\n    /// not copied per\
    \ level, so memory is linear in the value.\n    #[trace(\"TC-735\", \"FR-262-AC-3\")]\n    #[test]\n    fn type_nodes_grow_linearly_with_value_depth()\
    \ {\n        let small = type_nodes(&deep_typed_value(1_000).1);\n        let large = type_nodes(&deep_typed_value(2_000).1);\n\
    \        assert!(small >= 990, \"{small}\");"
  lines: '2861'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::a_deep_value_with_deep_types_is_admitted_in_linear_memory
  path: src/value.rs
  role: examined
  excerpt: "    /// types `DEEP` deep, is built, admitted, cloned, compared, hashed,\n    /// formatted and dropped\
    \ on a 512 KiB stack.\n    #[trace(\"TC-735\", \"FR-262-AC-3\")]\n    #[test]\n    fn a_deep_value_with_deep_types_is_admitted_in_linear_memory()\
    \ {\n        on_small_stack(|| {\n            use core::hash::{Hash, Hasher};\n            let (value_type,\
    \ value) = deep_typed_value(DEEP);"
  lines: '2873'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/division.rs#tests::division_by_zero_is_undefined
  path: src/division.rs
  role: examined
  excerpt: "    /// (`agent-ix/quire-specification`): this is the kernel-level instance\n    /// of that requirement.\n\
    \    #[trace(\"QSpec-TC-192\", \"QSpec-FR-147-AC-2\", \"TC-905\", \"FR-357-AC-1\")]\n    #[test]\n    fn division_by_zero_is_undefined()\
    \ {\n        let domain = IntegerDomain::Mathematical;\n        for profile in DivisionProfile::ALL {\n    \
    \        for member in [DivisionMember::Quotient, DivisionMember::Remainder] {"
  lines: '264'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/division.rs#tests::only_the_exposed_member_must_be_in_domain
  path: src/division.rs
  role: examined
  excerpt: "    /// `10 div y` over `1..=10` at `y = 5` is 2; `x rem -1` over `-10..=5` at\n    /// `x = -10` is\
    \ 0 even though the quotient (10) is outside that domain.\n    #[trace(\"TC-905\", \"FR-357-AC-2\")]\n    #[test]\n\
    \    fn only_the_exposed_member_must_be_in_domain() {\n        for profile in DivisionProfile::ALL {\n     \
    \       let q = run(profile, DivisionMember::Quotient, 10, 5, &range(1, 10));\n            assert_eq!(q.completed(),\
    \ Some(int(2)));"
  lines: '280'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/division.rs#tests::exposed_member_outside_domain_refuses_with_its_cause
  path: src/division.rs
  role: examined
  excerpt: "    /// a remainder outside refuses with `remainder-outside-domain`, under\n    /// every profile.\n\
    \    #[trace(\"TC-905\", \"FR-357-AC-3\")]\n    #[test]\n    fn exposed_member_outside_domain_refuses_with_its_cause()\
    \ {\n        for profile in DivisionProfile::ALL {\n            let Outcome::Refused(refusal) =\n          \
    \      run(profile, DivisionMember::Quotient, 10, 1, &range(0, 5))"
  lines: '294'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/division.rs#tests::profiles_differ_on_negative_operands
  path: src/division.rs
  role: examined
  excerpt: "    /// The three laws differ on negative operands exactly as defined, for -7\n    /// and 2 (and -7\
    \ and -2 for Euclidean).\n    #[trace(\"TC-905\", \"FR-357-AC-4\")]\n    #[test]\n    fn profiles_differ_on_negative_operands()\
    \ {\n        let domain = IntegerDomain::Mathematical;\n        let pair = |profile, a, b| {\n            let\
    \ q = run(profile, DivisionMember::Quotient, a, b, &domain);"
  lines: '317'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/division.rs#tests::i64_min_divided_by_minus_one_is_exact
  path: src/division.rs
  role: examined
  excerpt: "    /// a signed-64 domain the quotient refuses with `quotient-outside-domain`\n    /// while the remainder,\
    \ 0, completes.\n    #[trace(\"TC-905\", \"FR-357-AC-5\")]\n    #[test]\n    fn i64_min_divided_by_minus_one_is_exact()\
    \ {\n        let wide = IntegerDomain::Bounded(\n            IntegerInterval::new(int(0), Integer::from(i128::from(i64::MAX)\
    \ + 1)).unwrap(),\n        );"
  lines: '337'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/division.rs#tests::division_charges_one_domain_occurrence_and_one_result_unit
  path: src/division.rs
  role: examined
  excerpt: "    /// one too small reports `integer-division.result-retain`.\n    #[cfg(feature = \"test-support\"\
    )]\n    #[trace(\"TC-905\", \"FR-357-AC-6\")]\n    #[test]\n    fn division_charges_one_domain_occurrence_and_one_result_unit()\
    \ {\n        use crate::accounting::ChargePoint::{\n            IntegerDivisionArithmetic as Arithmetic, IntegerDivisionDomain\
    \ as Domain,\n            IntegerDivisionOperands as Operands, IntegerDivisionResultRetain as Retain,"
  lines: '364'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_core.rs#modulo_is_euclidean_after_every_division_profile
  path: tests/ir673_core.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-905, FR-357-AC-7\n#[test]\nfn modulo_is_euclidean_after_every_division_profile()\
    \ {\n    for (a, b, expected) in [(7_i64, 3_i64, 1_i64), (7, -3, 1), (-7, 3, 2), (-7, -3, 2)] {\n        let\
    \ (a, b) = (Integer::from(a), Integer::from(b));\n        for profile in DivisionProfile::ALL {"
  lines: '74'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::injected_denial_spends_only_on_its_named_admitted_position
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-358-AC-1\n    #[test]\n    fn injected_denial_spends_only_on_its_named_admitted_position()\
    \ {\n        let mut meter = Meter::new(tight_limits()).with_injected_denial(InjectedDenial {\n            point:\
    \ ChargePoint::FunctionCall,\n            occurrence: NonZeroU64::new(2).expect(\"two is nonzero\"),"
  lines: '1410'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::injected_plan_denial_spends_after_one_refusal
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-358-AC-2\n    #[test]\n    fn injected_plan_denial_spends_after_one_refusal()\
    \ {\n        let mut meter = Meter::new(unlimited()).with_injected_denial(InjectedDenial {\n            point:\
    \ ChargePoint::EqualityPlan,\n            occurrence: NonZeroU64::new(1).expect(\"one is nonzero\"),"
  lines: '1470'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::injected_denial_occurrence_requires_nonzero
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-358-AC-3\n    #[test]\n    fn injected_denial_occurrence_requires_nonzero()\
    \ {\n        assert_eq!(NonZeroU64::new(0), None);\n        let denial = InjectedDenial {\n            point:\
    \ ChargePoint::FunctionCall,"
  lines: '1699'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::charge_log_retains_only_first_4096_admissions
  path: src/accounting.rs
  role: examined
  excerpt: "\n    /// Trace: FR-358-AC-4, FR-358-AC-5\n    #[cfg(feature = \"test-support\")]\n    #[test]\n   \
    \ fn charge_log_retains_only_first_4096_admissions() {\n        let mut meter = Meter::new(unlimited());\n \
    \       let expected: Vec<_> = (0..CHARGE_LOG_CAPACITY)\n            .map(|index| {"
  lines: '1712'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::production_meter_size_is_constant_as_charges_grow
  path: src/accounting.rs
  role: examined
  excerpt: "    /// `make ci` runs): with it, the meter keeps its ordered charge log,\n    /// which allocates.\n\
    \    #[cfg(not(feature = \"test-support\"))]\n    #[test]\n    fn production_meter_size_is_constant_as_charges_grow()\
    \ {\n        assert!(\n            !core::mem::needs_drop::<Counters>(),\n            \"a production Meter's\
    \ counters must hold no field that grows with its charges\""
  lines: '1300'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_core.rs#truncated_meter_log_preserves_accounting_and_denial_atomicity
  path: tests/ir673_core.rs
  role: examined
  excerpt: "\n/// Trace: TC-906, FR-358-AC-8, FR-358-AC-9, FR-358-AC-10, FR-358-AC-11\n#[cfg(feature = \"test-support\"\
    )]\n#[test]\nfn truncated_meter_log_preserves_accounting_and_denial_atomicity() {\n    let point = ChargePoint::FunctionCall;\n\
    \    let other = ChargePoint::GraphEdge;\n    let mut meter = Meter::new(ScalarLimits {"
  lines: '105'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::injected_record_uses_precharge_work_and_plan_reservation
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-358-AC-12\n    #[test]\n    fn injected_record_uses_precharge_work_and_plan_reservation()\
    \ {\n        for work_limit in [10, 100] {\n            let limits = ScalarLimits {\n                work_units:\
    \ work_limit,"
  lines: '1507'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::injected_denial_precedes_ordinary_size_and_plan_shortages
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-358-AC-13, FR-359-AC-8\n    #[test]\n    fn injected_denial_precedes_ordinary_size_and_plan_shortages()\
    \ {\n        let size_limits = ScalarLimits {\n            integer_bits: 8,\n            work_units: 10,"
  lines: '1568'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir667_normalize.rs#records_actual_sizes_coalesces_repeats_and_distinguishes_absent_zero
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-358-AC-14\n#[test]\nfn records_actual_sizes_coalesces_repeats_and_distinguishes_absent_zero()\
    \ {\n    let point = ChargePoint::FunctionCall;\n    let mut meter = Meter::new(limits());\n    for amount in\
    \ [9, 3] {"
  lines: '53'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir667_normalize.rs#ordinary_injected_and_cancelled_charges_preserve_admission_records
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-358-AC-14\n#[test]\nfn ordinary_injected_and_cancelled_charges_preserve_admission_records()\
    \ {\n    let point = ChargePoint::FunctionCall;\n    for denial in 0..4 {\n        let cancel = Cancel::new();"
  lines: '136'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir667_normalize.rs#fixed_records_share_the_existing_prefix_and_truncation_boundary
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-358-AC-15\n#[test]\nfn fixed_records_share_the_existing_prefix_and_truncation_boundary()\
    \ {\n    let mut meter = Meter::new(limits());\n    let mut expected = Vec::new();\n    for admission in 1_u64..=4097\
    \ {"
  lines: '217'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::cumulative_work_reaches_max_then_refuses_atomically
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-1\n    #[test]\n    fn cumulative_work_reaches_max_then_refuses_atomically()\
    \ {\n        let mut meter = Meter::new(unlimited());\n        let point = ChargePoint::FunctionCall;\n    \
    \    meter"
  lines: '975'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::cumulative_results_reach_max_then_refuse_atomically
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-2\n    #[test]\n    fn cumulative_results_reach_max_then_refuse_atomically()\
    \ {\n        let mut meter = Meter::new(unlimited());\n        let point = ChargePoint::FunctionCall;\n    \
    \    meter.charge(Charge::new(point).results(u64::MAX)).unwrap();"
  lines: '1006'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::exact_cumulative_amount_above_max_refuses_without_changes
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-3, FR-359-AC-4\n    #[test]\n    fn exact_cumulative_amount_above_max_refuses_without_changes()\
    \ {\n        let excess = Integer::from(u64::MAX).add(&Integer::one());\n        let point = ChargePoint::FunctionCall;\n\
    \        for (charge, kind) in ["
  lines: '1034'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::result_refusal_preserves_prior_size_high_water
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-5\n    #[test]\n    fn result_refusal_preserves_prior_size_high_water()\
    \ {\n        let mut limits = unlimited();\n        limits.result_units = 1;\n        let mut meter = Meter::new(limits);"
  lines: '1066'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_core.rs#meter_uses_field_order_and_all_counters_are_queryable
  path: tests/ir673_core.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-907, FR-359-AC-6, FR-359-AC-7\n#[test]\nfn meter_uses_field_order_and_all_counters_are_queryable()\
    \ {\n    let point = ChargePoint::FunctionCall;\n    let kinds = LimitKind::ALL;\n    let prior = [11, 22, 33,\
    \ 44, 55, 66, 77, 88, 99, 111];"
  lines: '209'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::duplicate_size_requests_preserve_first_refusal_and_are_atomic
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn duplicate_size_requests_preserve_first_refusal_and_are_atomic()\
    \ {\n        let mut meter = Meter::new(tight_limits());\n        let before = meter.clone();\n        let refusal\
    \ = meter"
  lines: '1170'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::duplicate_sizes_coalesce_by_max_and_retain_explicit_zero
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn duplicate_sizes_coalesce_by_max_and_retain_explicit_zero()\
    \ {\n        let mut meter = Meter::new(unlimited());\n        meter\n            .charge("
  lines: '1189'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::first_inserted_failing_size_precedes_later_same_kind_maximum
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn first_inserted_failing_size_precedes_later_same_kind_maximum()\
    \ {\n        for (first, second) in [(7, 9), (9, 7)] {\n            let mut limits = unlimited();\n        \
    \    limits.integer_bits = 5;"
  lines: '1213'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::size_shortages_use_field_order_across_attachment_orders
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn size_shortages_use_field_order_across_attachment_orders()\
    \ {\n        for reverse in [false, true] {\n            let mut limits = unlimited();\n            limits.integer_bits\
    \ = 5;"
  lines: '1244'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::cancellation_and_injection_precede_size_refusal
  path: src/accounting.rs
  role: examined
  excerpt: "    /// validation: an oversized request is reported as the cancellation\n    /// (`WorkUnits`, `limit\
    \ = consumed`), never as its own size refusal, and\n    /// the refused charge changes nothing.\n    #[test]\n\
    \    fn cancellation_and_injection_precede_size_refusal() {\n        let cancel = Cancel::new();\n        let\
    \ mut cancelled = Meter::new(tight_limits()).with_cancel(cancel.clone());\n        let before = cancelled.clone();"
  lines: '1326'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::size_request_naming_a_cumulative_counter_is_validated_in_field_order
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn size_request_naming_a_cumulative_counter_is_validated_in_field_order()\
    \ {\n        let mut limits = unlimited();\n        limits.work_units = 4;\n        limits.result_units = 4;"
  lines: '1757'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::request_counts_from_zero_to_many_gain_no_denial
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn request_counts_from_zero_to_many_gain_no_denial()\
    \ {\n        let mut meter = Meter::new(unlimited());\n        let mut expected_bits = 0_u64;\n        for (admitted,\
    \ count) in [0_u64, 1, 2, 3, 64, 1000].into_iter().enumerate() {"
  lines: '1787'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::reordered_and_repeated_requests_keep_the_high_water_maximum
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn reordered_and_repeated_requests_keep_the_high_water_maximum()\
    \ {\n        for amounts in [[3_u64, 9, 5], [9, 3, 5], [5, 9, 3], [0, 0, 0]] {\n            let mut meter =\
    \ Meter::new(unlimited());\n            let charge = amounts.into_iter().fold("
  lines: '1809'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::ordinary_refusal_leaves_a_pending_injected_occurrence_unspent
  path: src/accounting.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-359-AC-8\n    #[test]\n    fn ordinary_refusal_leaves_a_pending_injected_occurrence_unspent()\
    \ {\n        let mut limits = unlimited();\n        limits.integer_bits = 5;\n        let point = ChargePoint::FunctionCall;"
  lines: '1837'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::tc_931_admitted_log_matches_model_ordered_prefix
  path: src/accounting.rs
  role: examined
  excerpt: "\n    /// Trace: FR-359-AC-12\n    #[cfg(feature = \"test-support\")]\n    #[test]\n    fn tc_931_admitted_log_matches_model_ordered_prefix()\
    \ {\n        let schedule: [ScheduledCharge; 3] = [\n            (\n                ChargePoint::IntegerArithmeticOperands,"
  lines: '1918'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/accounting.rs#tests::tc_931_admitted_log_matches_model_same_kind
  path: src/accounting.rs
  role: examined
  excerpt: "\n    /// Trace: FR-359-AC-12\n    #[cfg(feature = \"test-support\")]\n    #[test]\n    fn tc_931_admitted_log_matches_model_same_kind()\
    \ {\n        let point = ChargePoint::FunctionCall;\n        let cases: [&[(LimitKind, u64)]; 5] = [\n     \
    \       &[(LimitKind::IntegerBits, 3), (LimitKind::IntegerBits, 9)],"
  lines: '1971'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/integer.rs#tests::minimum_i64_absolute_value_is_positive_two_to_the_63rd
  path: src/integer.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-360-AC-1\n    #[test]\n    fn minimum_i64_absolute_value_is_positive_two_to_the_63rd()\
    \ {\n        let input = Integer::from(i64::MIN);\n        let magnitude = input.abs();\n        assert_eq!(magnitude,\
    \ Integer::from_str(\"9223372036854775808\").unwrap());"
  lines: '776'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#integer_arithmetic_denial_precedes_large_intermediate_even_when_result_cancels
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-361-AC-1\n#[test]\nfn integer_arithmetic_denial_precedes_large_intermediate_even_when_result_cancels()\
    \ {\n    let large = Integer::one().shifted_left(LARGE_BITS - 1);\n    let one = Integer::one();\n    let almost\
    \ = large.sub(&one);"
  lines: '64'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#rational_arithmetic_denial_precedes_large_unreduced_products
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-361-AC-2\n#[test]\nfn rational_arithmetic_denial_precedes_large_unreduced_products()\
    \ {\n    let large = Integer::one().shifted_left(LARGE_BITS - 1);\n    let one = Integer::one();\n    let huge\
    \ = Rational::new(large.clone(), one.clone()).unwrap();"
  lines: '91'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#division_and_modulus_denial_precedes_large_quotient
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-361-AC-3\n#[test]\nfn division_and_modulus_denial_precedes_large_quotient() {\n \
    \   let large = Integer::one().shifted_left(LARGE_BITS - 1);\n    let three = Integer::from(3_i64);\n    let\
    \ ceiling = (LARGE_BITS / 8) / 8;"
  lines: '123'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#decimal_scale_denial_precedes_power_of_ten_allocation
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-361-AC-4\n#[test]\nfn decimal_scale_denial_precedes_power_of_ten_allocation() {\n\
    \    let (left, right) = (decimal(1, 0), decimal(1, DECIMAL_SHIFT));\n    let mut configured = limits();\n \
    \   configured.scale_expansion = u64::from(DECIMAL_SHIFT) - 1;"
  lines: '186'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#decimal_arithmetic_denial_precedes_aligned_intermediate
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-361-AC-5\n#[test]\nfn decimal_arithmetic_denial_precedes_aligned_intermediate() {\n\
    \    let (left, right) = (decimal(1, 0), decimal(1, DECIMAL_SHIFT));\n    // B(10^(2^20)) = 3,483,295, so scale\
    \ expansion charges 3,483,296\n    // and addition arithmetic charges one more bit."
  lines: '206'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#decimal_retain_denial_precedes_target_scale_expansion
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-361-AC-6\n#[test]\nfn decimal_retain_denial_precedes_target_scale_expansion() {\n\
    \    let input = decimal(1, 0);\n    let target = target(DECIMAL_SHIFT);\n    let mut configured = limits();"
  lines: '229'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#large_floor_quotient_completes_at_exact_operand_bit_limit_and_denies_below
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-909, FR-361-AC-7, FR-361-AC-8\n#[test]\nfn large_floor_quotient_completes_at_exact_operand_bit_limit_and_denies_below()\
    \ {\n    let dividend = Integer::one().shifted_left(LARGE_BITS - 1);\n    let divisor = Integer::from(3_i64);\n\
    \    let reference = Integer::from_big(dividend.as_big() / divisor.as_big());"
  lines: '249'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_admission.rs#huge_decimal_target_denies_retain_before_power_allocation
  path: tests/ir653_admission.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-909, FR-361-AC-9\n#[test]\nfn huge_decimal_target_denies_retain_before_power_allocation()\
    \ {\n    let (left, right) = (decimal(1, 0), decimal(1, 0));\n    let target = DecimalType::new(\n        Integer::from(0_i64),"
  lines: '306'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#integer_arithmetic_matches_checked_i128_oracle
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-1\n#[test]\nfn integer_arithmetic_matches_checked_i128_oracle() {\n    let\
    \ values = samples();\n    for &a in &values {\n        let left = Integer::from(a);"
  lines: '151'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#integer_arithmetic_retains_results_beyond_i128
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-2\n#[test]\nfn integer_arithmetic_retains_results_beyond_i128() {\n    let\
    \ maximum = Integer::from(i128::MAX);\n    let minimum = Integer::from(i128::MIN);\n    assert_eq!("
  lines: '182'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#rational_arithmetic_matches_reduced_fraction_oracle
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-3\n#[test]\nfn rational_arithmetic_matches_reduced_fraction_oracle() {\n \
    \   let mut state = 0x7f4a_7c15_u64;\n    let mut cases = vec![(0, 1, 1, 2), (-1, 2, 2, 3), (7, 5, -3, 4)];\n\
    \    for _ in 0..64 {"
  lines: '215'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#integer_and_rational_ordering_match_checked_oracle
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-4\n#[test]\nfn integer_and_rational_ordering_match_checked_oracle() {\n  \
    \  let values = samples();\n    for pair in values.windows(2) {\n        let (a, b) = (pair[0], pair[1]);"
  lines: '294'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#scalar_families_charge_order_and_independent_size_maxima
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "\n/// Trace: FR-362-AC-5, FR-362-AC-6\n#[cfg(feature = \"test-support\")]\n#[test]\nfn scalar_families_charge_order_and_independent_size_maxima()\
    \ {\n    for (left, right) in [(5_i64, 128_i64), (128, 5)] {\n        let (a, b) = (Integer::from(left), Integer::from(right));\n\
    \        let operand_max = bit_len(i128::from(left)).max(bit_len(i128::from(right)));"
  lines: '376'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#three_direct_atoms_consume_seven_work_and_three_results
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-7\n#[test]\nfn three_direct_atoms_consume_seven_work_and_three_results() {\n\
    \    let mut meter = meter();\n    let (a, b) = (Integer::from(5_i64), Integer::from(7_i64));\n    assert_eq!("
  lines: '474'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#each_scalar_charge_point_can_stop_before_result
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-8\n#[test]\nfn each_scalar_charge_point_can_stop_before_result() {\n    let\
    \ (a, b) = (Integer::from(5_i64), Integer::from(7_i64));\n    let (left, right) = (rational(1, 2), rational(2,\
    \ 3));\n    assert_denials("
  lines: '498'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#already_decided_boolean_connectives_follow_truth_tables_and_one_charge
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-9\n#[test]\nfn already_decided_boolean_connectives_follow_truth_tables_and_one_charge()\
    \ {\n    for left in [false, true] {\n        for right in [false, true] {\n            for (connective, expected)\
    \ in ["
  lines: '585'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir667_normalize.rs#rational_normalize_records_unreduced_parts_and_exact_boundary_stops
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-10, FR-362-AC-20\n#[test]\nfn rational_normalize_records_unreduced_parts_and_exact_boundary_stops()\
    \ {\n    #[derive(Clone, Copy)]\n    enum Op {\n        Add,"
  lines: '359'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir667_normalize.rs#integer_arithmetic_exact_bits_and_one_under_use_operand_formulas
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-10\n#[test]\nfn integer_arithmetic_exact_bits_and_one_under_use_operand_formulas()\
    \ {\n    #[derive(Clone, Copy)]\n    enum Op {\n        Add,"
  lines: '487'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir667_normalize.rs#integer_and_rational_ordering_exact_bits_and_one_under_stop_at_first_short_point
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-10\n#[test]\nfn integer_and_rational_ordering_exact_bits_and_one_under_stop_at_first_short_point()\
    \ {\n    use ChargePoint::{\n        OrderingArithmetic as Arithmetic, OrderingOperands as Operands,\n     \
    \   OrderingResultRetain as Retain,"
  lines: '588'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#scalar_outcomes_keep_only_admitted_prefixes
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-11\n#[test]\nfn scalar_outcomes_keep_only_admitted_prefixes() {\n\
    \    let mut m = meter();\n    let false_result = order_numbers(\n        OrderingOperator::Less,"
  lines: '223'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#rational_constructor_reduces_sign_and_zero
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-12\n#[test]\nfn rational_constructor_reduces_sign_and_zero() {\n \
    \   for (a, b, c, d) in [\n        (4, 8, 1, 2),\n        (-4, -8, 1, 2),"
  lines: '348'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#rational_domain_refuses_denominator_outside_interval
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-13\n#[test]\nfn rational_domain_refuses_denominator_outside_interval()\
    \ {\n    let target = domain(0, 1, 1, 2);\n    assert!(!target.contains(&r(1, 3)));\n    let mut m = meter();"
  lines: '363'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#direct_countdown_atoms_stop_at_fifth_result
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-14\n#[test]\nfn direct_countdown_atoms_stop_at_fifth_result() {\n\
    \    fn run(m: &mut Meter) -> Vec<Outcome<Integer>> {\n        let mut values = Vec::new();\n        assert_eq!("
  lines: '403'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#direct_rational_division_charges_four_points
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-15\n#[test]\nfn direct_rational_division_charges_four_points() {\n\
    \    let (a, b) = (r(3, 1), r(2, 1));\n    let mut m = meter();\n    assert_eq!("
  lines: '515'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#direct_integer_additions_share_meter_without_fold
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-16\n#[test]\nfn direct_integer_additions_share_meter_without_fold()\
    \ {\n    let mut m = meter();\n    assert_eq!(\n        integer_op(IntegerArithmetic::Add(&n(0), &n(1)), None,\
    \ &mut m).completed(),"
  lines: '574'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#atom_work_boundaries_stop_on_named_points
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-17\n#[test]\nfn atom_work_boundaries_stop_on_named_points() {\n  \
    \  for (work, point) in [\n        (2, Some(ChargePoint::IntegerArithmeticResultRetain)),\n        (3, None),"
  lines: '612'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_scalar.rs#powers_of_two_charge_operand_derived_bounds
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-910, FR-362-AC-18\n#[test]\nfn powers_of_two_charge_operand_derived_bounds() {\n\
    \    for k in [63, 64, 65, 127, 128, 200, 511] {\n        let power = Integer::one().shifted_left(k);\n    \
    \    let below = power.sub(&Integer::one());"
  lines: '704'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_scalar.rs#public_integer_arithmetic_admits_exact_bits_and_denies_one_under
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-362-AC-19\n#[test]\nfn public_integer_arithmetic_admits_exact_bits_and_denies_one_under()\
    \ {\n    #[derive(Clone, Copy)]\n    enum Operation {\n        Add,"
  lines: '35'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir873_temporal_visit.rs#temporal_visit_charges_one_work_unit_and_respects_the_boundary
  path: tests/ir873_temporal_visit.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-362-AC-21\", \"TC-910\")]\n#[test]\nfn temporal_visit_charges_one_work_unit_and_respects_the_boundary()\
    \ {\n    let point = temporal_visit();\n    let mut meter = Meter::new(limits(1));\n    meter.charge(Charge::new(point)).unwrap();"
  lines: '53'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir873_temporal_visit.rs#injected_temporal_visit_denial_is_atomic_and_one_shot
  path: tests/ir873_temporal_visit.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-362-AC-22\", \"TC-910\")]\n#[test]\nfn injected_temporal_visit_denial_is_atomic_and_one_shot()\
    \ {\n    let point = temporal_visit();\n    let mut meter = Meter::new(limits(1)).with_injected_denial(InjectedDenial\
    \ {\n        point,"
  lines: '85'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_decimal_order.rs#decimal_ordering_matches_independent_scaled_integer_oracle
  path: tests/ir653_decimal_order.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-363-AC-1\n#[test]\nfn decimal_ordering_matches_independent_scaled_integer_oracle()\
    \ {\n    let cases = [\n        ((100, 2), (1, 0)),\n        ((0, 5), (0, 0)),"
  lines: '41'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_decimal_order.rs#decimal_ordering_charges_retained_zero_and_equal_value_representations
  path: tests/ir653_decimal_order.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-363-AC-2, FR-363-AC-3\n#[test]\nfn decimal_ordering_charges_retained_zero_and_equal_value_representations()\
    \ {\n    for (left, right, operator, expected_bits, expected_digits, expected_shift) in [\n        (\n     \
    \       decimal(0, 5),"
  lines: '79'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_decimal_order.rs#huge_retained_scale_refuses_analytically_before_comparison
  path: tests/ir653_decimal_order.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-363-AC-4\n#[test]\nfn huge_retained_scale_refuses_analytically_before_comparison()\
    \ {\n    let (left, right) = (decimal(1, 0), decimal(1, u32::MAX));\n    let mut configured = limits();\n  \
    \  configured.decimal_digits = 64;"
  lines: '127'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir653_decimal_order.rs#each_decimal_ordering_point_denies_its_suffix
  path: tests/ir653_decimal_order.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-363-AC-5\n#[test]\nfn each_decimal_ordering_point_denies_its_suffix() {\n    let\
    \ (left, right) = (decimal(0, 5), decimal(0, 0));\n    let points = [\n        ChargePoint::OrderingOperands,"
  lines: '178'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_decimal.rs#normalized_equal_decimals_keep_distinct_retained_representations
  path: tests/ir673_decimal.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-911, FR-363-AC-6\n#[test]\nfn normalized_equal_decimals_keep_distinct_retained_representations()\
    \ {\n    let (left, right) = (decimal(110, 2), decimal(11, 1));\n    assert_eq!(left.compare(&right), Ordering::Equal);\n\
    \    assert!(left.numerically_equal(&right));"
  lines: '37'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_decimal.rs#metered_ordering_uses_retained_decimal_sizes
  path: tests/ir673_decimal.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-911, FR-363-AC-7\n#[test]\nfn metered_ordering_uses_retained_decimal_sizes() {\n\
    \    for (left, right, bits, digits, expansion) in [\n        (decimal(110, 2), decimal(11, 1), 8, 3, 1),\n\
    \        (decimal(11, 1), decimal(11, 1), 4, 2, 0),"
  lines: '67'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_ieee_text.rs#ieee_flags_iterate_all_subsets_in_vocabulary_order
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-912, FR-364-AC-1\n#[test]\nfn ieee_flags_iterate_all_subsets_in_vocabulary_order()\
    \ {\n    let vocabulary = [\n        IeeeFlag::Invalid,\n        IeeeFlag::DivideByZero,"
  lines: '40'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_decimal.rs#decimal_half_ties_follow_all_six_rounding_modes
  path: tests/ir673_decimal.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-913, FR-365-AC-1\n#[test]\nfn decimal_half_ties_follow_all_six_rounding_modes() {\n\
    \    for (mode, positive, negative) in [\n        (RoundingMode::Exact, None, None),\n        (RoundingMode::TowardZero,\
    \ Some(0), Some(0)),"
  lines: '105'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_decimal.rs#default_decimal_rounding_is_strict_exact
  path: tests/ir673_decimal.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-913, FR-365-AC-2\n#[test]\nfn default_decimal_rounding_is_strict_exact() {\n    assert_eq!(RoundingMode::default(),\
    \ RoundingMode::Exact);\n    let target = target(RoundingMode::default());\n    assert_eq!(target.rounding(),\
    \ RoundingMode::Exact);"
  lines: '147'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_ieee_text.rs#ieee_nan_selection_quiets_first_and_reports_later_signaling
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-914, FR-366-AC-1\n#[test]\nfn ieee_nan_selection_quiets_first_and_reports_later_signaling()\
    \ {\n    for (width, exp, quiet, sign, one) in [\n        (\n            IeeeWidth::Binary32,"
  lines: '65'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_ieee_text.rs#ieee_nan_width_conversion_refuses_unrepresentable_payload
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-914, FR-366-AC-2\n#[test]\nfn ieee_nan_width_conversion_refuses_unrepresentable_payload()\
    \ {\n    let exp64 = 0x7ff0_0000_0000_0000_u64;\n    let quiet64 = 1_u64 << 51;\n    for payload in [1_u64 <<\
    \ 22, 1_u64 << 30] {"
  lines: '129'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_ieee_text.rs#ieee_signed_zero_converts_to_canonical_rational_with_loss
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-914, FR-366-AC-3\n#[test]\nfn ieee_signed_zero_converts_to_canonical_rational_with_loss()\
    \ {\n    let domain = RationalDomain::new(\n        IntegerInterval::new(Integer::from(-1_i64), Integer::from(1_i64)).unwrap(),\n\
    \        IntegerInterval::new(Integer::from(1_i64), Integer::from(1_i64)).unwrap(),"
  lines: '168'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_ieee_text.rs#ieee_total_order_separates_ten_classes_and_signed_zero
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-914, FR-366-AC-4\n#[test]\nfn ieee_total_order_separates_ten_classes_and_signed_zero()\
    \ {\n    for (width, sign, exp, quiet, one) in [\n        (\n            IeeeWidth::Binary32,"
  lines: '191'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_ieee_text.rs#text_profile_mismatch_refuses_before_any_comparison_charge
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-915, FR-367-AC-1\n#[test]\nfn text_profile_mismatch_refuses_before_any_comparison_charge()\
    \ {\n    let payload = TextPayload::from_utf8(b\"a\").unwrap();\n    let nfc = TextType::new(0, 4, TextProfile::Nfc).unwrap();\n\
    \    let binary = TextType::new(0, 4, TextProfile::BinaryUtf8).unwrap();"
  lines: '264'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_core.rs#charge_point_vocabulary_is_unique_and_round_trips
  path: tests/ir673_core.rs
  role: examined
  excerpt: "}\n\n#[trace(\"TC-916\", \"FR-368-AC-1\")]\n#[test]\nfn charge_point_vocabulary_is_unique_and_round_trips()\
    \ {\n    const _: () = {\n        assert!(matches!(ChargePoint::ALL[0], ChargePoint::DecimalOperands));\n  \
    \      assert!(matches!("
  lines: '353'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir673_core.rs#limit_kind_vocabulary_matches_scalar_limit_fields
  path: tests/ir673_core.rs
  role: examined
  excerpt: "}\n\n/// Trace: TC-916, FR-368-AC-2\n#[test]\nfn limit_kind_vocabulary_matches_scalar_limit_fields()\
    \ {\n    let expected = [\n        (LimitKind::IntegerBits, \"integer_bits\"),\n        (LimitKind::DecimalDigits,\
    \ \"decimal_digits\"),"
  lines: '625'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/checked_invariant.rs#comparison_payloads_retain_the_original_typed_cause
  path: tests/checked_invariant.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-369-AC-1, FR-369-AC-5\n#[test]\nfn comparison_payloads_retain_the_original_typed_cause()\
    \ {\n    let constructors: [fn(IllTypedCause) -> CheckedInvariantCause; 4] = [\n        |cause| CheckedInvariantCause::ScheduledComparisonRefused\
    \ { cause },\n        |cause| CheckedInvariantCause::EqualityQuantityConversionRejected { cause },"
  lines: '191'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/checked_invariant.rs#rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause
  path: tests/checked_invariant.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-369-AC-1, FR-369-AC-5\n#[test]\nfn rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause()\
    \ {\n    use std::{fs, process::Command};\n\n    let executable = std::env::current_exe().expect(\"integration\
    \ test executable\");"
  lines: '230'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/collection.rs#tests::evaluated_element_outside_its_type_reports_exact_cause
  path: src/collection.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-369-AC-2\n    #[test]\n    fn evaluated_element_outside_its_type_reports_exact_cause()\
    \ {\n        let collection_type =\n            CollectionType::new(CollectionKind::Sequence, ValueType::Integer,\
    \ None);\n        let elements: Vec<Deferred<'_>> ="
  lines: '555'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/collection.rs#tests::grouped_float_members_without_keys_report_exact_cause
  path: src/collection.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-369-AC-2\n    #[test]\n    fn grouped_float_members_without_keys_report_exact_cause()\
    \ {\n        use crate::ieee::{FloatType, IeeeValue, IeeeWidth};\n\n        let collection_type = CollectionType::new("
  lines: '570'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/division.rs#tests::refused_interval_requires_a_bounded_domain
  path: src/division.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-369-AC-2\n    #[test]\n    fn refused_interval_requires_a_bounded_domain()\
    \ {\n        assert_eq!(\n            refused_interval(&IntegerDomain::Mathematical),\n            Err(Stop::Refused(Refusal::CheckedInvariant\
    \ {"
  lines: '225'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/equality.rs#tests::incompatible_pairs_report_specific_causes_without_a_boolean
  path: src/equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-369-AC-2\n    #[test]\n    fn incompatible_pairs_report_specific_causes_without_a_boolean()\
    \ {\n        use crate::collection::{from_admitted, CollectionKind, CollectionType};\n        use crate::identity::PopulationId;\n\
    \        use crate::value::ValueType;"
  lines: '618'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::deferred_record_field_outside_its_type_reports_exact_cause
  path: src/value.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-369-AC-2\n    #[test]\n    fn deferred_record_field_outside_its_type_reports_exact_cause()\
    \ {\n        let member = MemberId::from_digest(digest(1));\n        let shape = [FieldDeclaration::new(\n \
    \           member,"
  lines: '1918'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::deferred_tuple_position_outside_its_type_reports_exact_cause
  path: src/value.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-369-AC-2\n    #[test]\n    fn deferred_tuple_position_outside_its_type_reports_exact_cause()\
    \ {\n        let positions: Vec<Deferred<'_>> =\n            vec![Box::new(|_| Outcome::Completed(Value::Boolean(true)))];\n\
    \        let outcome = evaluate_tuple("
  lines: '1947'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::native_type_admission_has_no_alias
  path: src/value.rs
  role: examined
  excerpt: "\n    /// Native types admit their own payload and reject existing and crossed kinds.\n    #[trace(\"\
    TC-918\", \"FR-370-AC-1\", \"FR-370-AC-5\")]\n    #[test]\n    fn native_type_admission_has_no_alias() {\n \
    \       let uuid = uuid(\"00112233-4455-6677-8899-aabbccddeeff\");\n        let timestamp = timestamp(\"1\"\
    );\n        assert!(ValueType::Uuid.admits(&uuid));"
  lines: '1545'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::native_equality_uses_boolean_leaf_schedule
  path: src/value.rs
  role: examined
  excerpt: "\n    /// Both native leaves use the same one-pair plan and charges as Boolean.\n    #[trace(\"TC-918\"\
    , \"FR-370-AC-2\", \"FR-370-AC-5\")]\n    #[test]\n    fn native_equality_uses_boolean_leaf_schedule() {\n \
    \       let pairs = [\n            (Value::Boolean(true), Value::Boolean(false)),\n            ("
  lines: '1692'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::native_mixed_kind_plans_refuse_in_both_orders
  path: src/value.rs
  role: examined
  excerpt: "\n    /// Every existing value kind remains distinct from both new native kinds.\n    #[trace(\"TC-918\"\
    , \"FR-370-AC-3\", \"FR-370-AC-5\")]\n    #[test]\n    fn native_mixed_kind_plans_refuse_in_both_orders() {\n\
    \        use crate::outcome::CheckedInvariantCause;\n\n        let key = NodeKey::from_digest(digest(9));"
  lines: '1630'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::native_keys_order_canonical_content_and_sets
  path: src/value.rs
  role: examined
  excerpt: "\n    /// Native keys order canonical ASCII payloads and set visiting order is stable.\n    #[trace(\"\
    TC-918\", \"FR-370-AC-4\", \"FR-370-AC-5\")]\n    #[test]\n    fn native_keys_order_canonical_content_and_sets()\
    \ {\n        use core::cmp::Ordering;\n        let pairs = [\n            (timestamp(\"10\"), timestamp(\"2\"\
    )),"
  lines: '1744'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value.rs#tests::existing_kind_controls_remain_admitted_and_keyed
  path: src/value.rs
  role: examined
  excerpt: "\n    /// Existing admitted leaves and nested forms retain their equality and keys.\n    #[trace(\"\
    TC-918\", \"FR-370-AC-5\")]\n    #[test]\n    fn existing_kind_controls_remain_admitted_and_keyed() {\n    \
    \    let text_type =\n            TextType::new(0, 10, crate::text::TextProfile::UnicodeScalars).expect(\"bounds\"\
    );\n        let text = crate::text::admit_text("
  lines: '1808'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value/native.rs#tests::uuid_canonical_text_preserves_network_octets
  path: src/value/native.rs
  role: examined
  excerpt: "\n    /// Native UUID octets retain all bits and reject every noncanonical spelling.\n    #[trace(\"\
    TC-918\", \"FR-370-AC-6\", \"FR-370-AC-8\")]\n    #[test]\n    fn uuid_canonical_text_preserves_network_octets()\
    \ {\n        for (text, expected) in [\n            (\"00000000-0000-0000-0000-000000000000\", [0; 16]),\n \
    \           (\"ffffffff-ffff-ffff-ffff-ffffffffffff\", [0xff; 16]),"
  lines: '138'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/value/native.rs#tests::timestamp_canonical_text_preserves_full_signed_nanoseconds
  path: src/value/native.rs
  role: examined
  excerpt: "\n    /// Canonical Timestamp syntax precedes signed-range checking.\n    #[trace(\"TC-918\", \"FR-370-AC-7\"\
    , \"FR-370-AC-8\")]\n    #[test]\n    fn timestamp_canonical_text_preserves_full_signed_nanoseconds() {\n  \
    \      for (text, expected) in [\n            (\"-170141183460469231731687303715884105728\", i128::MIN),\n \
    \           (\"-1\", -1),"
  lines: '178'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/integer.rs#tests::native_arithmetic_stays_inline_and_overflow_refines_exactly
  path: src/integer.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-371-AC-1, FR-371-AC-4\n    #[test]\n    fn native_arithmetic_stays_inline_and_overflow_refines_exactly()\
    \ {\n        let zero = Integer::zero();\n        let min = Integer::from(i128::MIN);\n        let max = Integer::from(i128::MAX);"
  lines: '814'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir705_bounded_integer.rs#inline_signed_bounds_refine_the_explicit_128_bit_domain
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-371-AC-1, FR-371-AC-4\n#[test]\nfn inline_signed_bounds_refine_the_explicit_128_bit_domain()\
    \ {\n    let domain = IntegerInterval::signed_twos_complement(NonZeroU32::new(128).unwrap());\n    let lower\
    \ = Integer::from(i128::MIN);\n    let upper = Integer::from(i128::MAX);"
  lines: '66'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir705_bounded_integer.rs#construction_boundaries_and_sampled_arithmetic_match_independent_oracles
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-371-AC-1\n#[test]\nfn construction_boundaries_and_sampled_arithmetic_match_independent_oracles()\
    \ {\n    for (value, spelling) in [\n        (i128::MIN, \"-170141183460469231731687303715884105728\"),\n  \
    \      (i128::MAX, \"170141183460469231731687303715884105727\"),"
  lines: '81'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/integer.rs#tests::cache_is_invisible_to_traits_clones_and_concurrent_borrows
  path: src/integer.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-371-AC-2, FR-371-AC-5\n    #[test]\n    fn cache_is_invisible_to_traits_clones_and_concurrent_borrows()\
    \ {\n        fn assert_send_sync<T: Send + Sync>() {}\n        fn assert_unwind_safe<T: core::panic::UnwindSafe\
    \ + core::panic::RefUnwindSafe>() {}\n        assert_send_sync::<Integer>();"
  lines: '851'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir705_bounded_integer.rs#metered_native_and_overflow_operations_keep_the_existing_charge_schedule
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-371-AC-3\n#[test]\nfn metered_native_and_overflow_operations_keep_the_existing_charge_schedule()\
    \ {\n    let native_left = Integer::from(8_i128);\n    let native_right = Integer::from(3_i128);\n    let overflow_left\
    \ = Integer::from(i128::MAX);"
  lines: '177'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir705_bounded_integer.rs#metered_operation_matrix_preserves_fr362_amounts_work_and_denial_prefixes
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-371-AC-3\n#[test]\nfn metered_operation_matrix_preserves_fr362_amounts_work_and_denial_prefixes()\
    \ {\n    let small_pos = Integer::from(8_i128);\n    let small_pos_rhs = Integer::from(3_i128);\n    let small_sub_rhs\
    \ = Integer::from(7_i128);"
  lines: '268'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/integer.rs#tests::magnitude_limb_stream_does_not_materialize_small_cache
  path: src/integer.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-371-AC-4\n    #[test]\n    fn magnitude_limb_stream_does_not_materialize_small_cache()\
    \ {\n        let values = [\n            Integer::zero(),\n            Integer::from(-1_i128),"
  lines: '953'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir705_bounded_integer.rs#fitting_integer_operations_and_cached_source_clone_allocate_nothing
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-371-AC-4\n#[test]\nfn fitting_integer_operations_and_cached_source_clone_allocate_nothing()\
    \ {\n    let left = Integer::from(123_i128);\n    let right = Integer::from(456_i128);\n    let domain = IntegerInterval::signed_twos_complement(NonZeroU32::new(16).unwrap());"
  lines: '32'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir705_bounded_integer.rs#native_public_metered_call_matches_accounting_only_allocation_observation
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "\n/// Trace: FR-371-AC-4\n#[cfg(not(feature = \"test-support\"))]\n#[test]\nfn native_public_metered_call_matches_accounting_only_allocation_observation()\
    \ {\n    let left = Integer::from(8_i128);\n    let right = Integer::from(3_i128);\n    let mut public_meter\
    \ = Meter::new(limits());"
  lines: '554'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/integer.rs#tests::once_box_race_retains_one_candidate_and_releases_losers
  path: src/integer.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-371-AC-5\n    #[test]\n    fn once_box_race_retains_one_candidate_and_releases_losers()\
    \ {\n        const CANDIDATES: usize = 8;\n        let cache = once_cell::race::OnceBox::<BigInt>::new();\n\
    \        let barrier = std::sync::Barrier::new(CANDIDATES);"
  lines: '893'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::inclusive_membership_preserves_representations_and_ignores_rounding
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-1\n    #[test]\n    fn inclusive_membership_preserves_representations_and_ignores_rounding()\
    \ {\n        // Expected lifted coefficients are calculated directly from these\n        // bounded authored\
    \ pairs, independently of the endpoint algorithm.\n        let cases = ["
  lines: '329'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::borrowed_limb_crossings_and_huge_shift_have_bounded_work
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-1, FR-372-AC-2\n    #[test]\n    fn borrowed_limb_crossings_and_huge_shift_have_bounded_work()\
    \ {\n        let declared = target(4_294_967_295, 4_294_967_297, 0, 0);\n        let value = Decimal::new(Integer::from(4_294_967_296_u64),\
    \ 0);\n        let counts = Counts::default();"
  lines: '407'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::scale_sign_and_zero_paths_request_no_scratch
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-1, FR-372-AC-2\n    #[test]\n    fn scale_sign_and_zero_paths_request_no_scratch()\
    \ {\n        let cancel = Cancel::new();\n        let cases = [\n            (target(0, 100, 0, 2), Decimal::new(Integer::one(),\
    \ 3), false),"
  lines: '438'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::longest_alignment_reuses_one_exact_request_within_independent_envelope
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-2\n    #[test]\n    fn longest_alignment_reuses_one_exact_request_within_independent_envelope()\
    \ {\n        let endpoint = 1_000_000_000_000_000_000;\n        let declared = target(endpoint, endpoint, 18,\
    \ 18);\n        let value = Decimal::new(Integer::one(), 0);"
  lines: '382'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::actual_small_reservation_denial_is_typed_and_recoverable
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-3\n    #[test]\n    fn actual_small_reservation_denial_is_typed_and_recoverable()\
    \ {\n        let declared = target(0, 100, 2, 2);\n        let value = Decimal::new(Integer::one(), 0);\n  \
    \      let cancel = Cancel::new();"
  lines: '465'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::native_layout_capacity_failure_is_separate_from_allocator_denial
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-3\n    #[test]\n    fn native_layout_capacity_failure_is_separate_from_allocator_denial()\
    \ {\n        assert_eq!(scratch_layout(1).unwrap().0, 1);\n        assert_eq!(scratch_layout(32).unwrap().0,\
    \ 1);\n        assert_eq!(scratch_layout(33).unwrap().0, 2);"
  lines: '501'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::original_precancelled_handle_prevents_the_actual_reservation
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-4\n    #[test]\n    fn original_precancelled_handle_prevents_the_actual_reservation()\
    \ {\n        let declared = target(0, 100, 2, 2);\n        let value = Decimal::new(Integer::one(), 0);\n  \
    \      for cause in [CancelCause::Requested, CancelCause::Deadline] {"
  lines: '531'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/decimal_membership.rs#tests::reservation_division_comparison_and_publication_cancellation_release_scratch
  path: src/decimal_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-372-AC-4\n    #[test]\n    fn reservation_division_comparison_and_publication_cancellation_release_scratch()\
    \ {\n        let endpoint = 1_000_000_000_000_000_000;\n        let declared = target(endpoint, endpoint, 18,\
    \ 18);\n        let value = Decimal::new(Integer::one(), 0);"
  lines: '576'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/ieee.rs#rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  path: src/ieee.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-373-AC-1, FR-373-AC-2, FR-373-AC-5\n    #[test]\n    fn finite_values_and_both_signed_zeros_complete_directly()\
    \ {\n        let domain = domain(3, Integer::from(2_i64));\n        for (value, width) in fixtures() {\n   \
    \         let mut meter = Meter::new(limits());"
  lines: '1984'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/ieee.rs#rational_target_ieee::nonfinite_values_stop_after_operands
  path: src/ieee.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-373-AC-3\n    #[test]\n    fn nonfinite_values_stop_after_operands() {\n\
    \        let domain = domain(3, Integer::from(2_i64));\n        for (value, width) in [\n            (IeeeValue::binary32(0x7fc0_0001),\
    \ 32),"
  lines: '2014'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/ieee.rs#rational_target_ieee::numerator_and_denominator_refusals_retain_the_declared_target
  path: src/ieee.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-373-AC-4\n    #[test]\n    fn numerator_and_denominator_refusals_retain_the_declared_target()\
    \ {\n        for domain in [domain(1, Integer::from(2_i64)), domain(3, Integer::one())] {\n            for (value,\
    \ width) in fixtures() {\n                let mut meter = Meter::new(limits());"
  lines: '2037'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/ieee.rs#rational_target_ieee::ordinary_and_injected_denials_preserve_the_admitted_prefix
  path: src/ieee.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-373-AC-5\n    #[test]\n    fn ordinary_and_injected_denials_preserve_the_admitted_prefix()\
    \ {\n        let domain = domain(3, Integer::from(2_i64));\n        for (value, width) in fixtures() {\n   \
    \         for (admitted, point) in POINTS.into_iter().enumerate() {"
  lines: '2054'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/ieee.rs#rational_target_ieee::subnormal_intermediates_require_the_exact_denominator_bits
  path: src/ieee.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-373-AC-5\n    #[test]\n    fn subnormal_intermediates_require_the_exact_denominator_bits()\
    \ {\n        for (value, width, next, exponent) in [\n            (IeeeValue::binary32(1), 32, 150, 149),\n\
    \            (IeeeValue::binary64(1), 64, 1075, 1074),"
  lines: '2111'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/ieee.rs#rational_target_ieee::original_cancellation_handle_stops_each_poll_and_live_handle_completes
  path: src/ieee.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-373-AC-6\n    #[test]\n    fn original_cancellation_handle_stops_each_poll_and_live_handle_completes()\
    \ {\n        let domain = domain(3, Integer::from(2_i64));\n        for (value, width) in fixtures() {\n   \
    \         for cause in [CancelCause::Requested, CancelCause::Deadline] {"
  lines: '2160'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/ieee.rs#rational_target_ieee::already_spent_meter_prefix_survives_success_and_every_retain_stop
  path: src/ieee.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-373-AC-7\n    #[test]\n    fn already_spent_meter_prefix_survives_success_and_every_retain_stop()\
    \ {\n        let domain = domain(3, Integer::from(2_i64));\n        for (value, width) in fixtures() {\n   \
    \         let configured = limits();"
  lines: '2202'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/nonzero_integer.rs#admission_denies_zero_and_borrows_each_original_nonzero_integer
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-374-AC-1, FR-374-AC-2\n#[test]\nfn admission_denies_zero_and_borrows_each_original_nonzero_integer()\
    \ {\n    for zero in [int(0), literal(\"0\"), Integer::one().sub(&Integer::one())] {\n        let result: Result<NonZeroInteger<'_>,\
    \ ZeroDivisor> = NonZeroInteger::try_from(&zero);\n        assert_eq!(result, Err(ZeroDivisor));"
  lines: '38'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/nonzero_integer.rs#admitting_an_inline_integer_does_not_materialize_its_bigint
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-374-AC-1\n#[test]\nfn admitting_an_inline_integer_does_not_materialize_its_bigint()\
    \ {\n    let divisor = int(2);\n    let mut admitted = None;\n    let allocations = allocation_counter::measure(||\
    \ {"
  lines: '64'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/nonzero_integer.rs#admitted_token_has_no_default_construction
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-374-AC-2\n#[test]\nfn admitted_token_has_no_default_construction() {\n    // The\
    \ inferred marker is unique only while NonZeroInteger lacks Default.\n    trait AmbiguousIfDefault<Marker> {\n\
    \        fn check() {}"
  lines: '78'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/nonzero_integer.rs#admitted_pair_helpers_preserve_signed_zero_and_large_results
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-374-AC-3\n#[test]\nfn admitted_pair_helpers_preserve_signed_zero_and_large_results()\
    \ {\n    for (a, b, truncating, floor) in [\n        (7, 2, (3, 1), (3, 1)),\n        (7, -2, (-3, 1), (-4,\
    \ -1)),"
  lines: '91'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/nonzero_integer.rs#exact_div_keeps_infallible_signed_results_and_nondivisible_truncation
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-374-AC-4\n#[test]\nfn exact_div_keeps_infallible_signed_results_and_nondivisible_truncation()\
    \ {\n    for (a, b, quotient) in [\n        (6, 2, 3),\n        (6, -2, -3),"
  lines: '125'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/integer.rs#tests::canonical_nonzero_premise_rejects_zero_with_its_own_message
  path: src/integer.rs
  role: examined
  excerpt: "\n    /// Trace: FR-374-AC-5\n    #[test]\n    #[should_panic(expected = \"canonical divisor must be\
    \ nonzero\")]\n    fn canonical_nonzero_premise_rejects_zero_with_its_own_message() {\n        NonZeroInteger::from_proven_nonzero(&Integer::zero());\n\
    \    }\n"
  lines: '770'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/nonzero_integer.rs#metered_zero_stops_after_operands_and_denial_precedes_zero_classification
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-374-AC-6\n#[test]\nfn metered_zero_stops_after_operands_and_denial_precedes_zero_classification()\
    \ {\n    let evaluations = DivisionProfile::ALL\n        .into_iter()\n        .flat_map(|profile| {"
  lines: '222'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/nonzero_integer.rs#metered_nonzero_retains_laws_single_member_domain_and_charge_requests
  path: tests/nonzero_integer.rs
  role: examined
  excerpt: "}\n\n/// Trace: FR-374-AC-6\n#[test]\nfn metered_nonzero_retains_laws_single_member_domain_and_charge_requests()\
    \ {\n    #[cfg(feature = \"test-support\")]\n    use ChargePoint::{\n        IntegerDivisionArithmetic as Arithmetic,\
    \ IntegerDivisionDomain as Domain,"
  lines: '269'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/equality.rs#tests::kernel_result_bearing_evaluation_returns_relation_plan_and_charge_order
  path: src/equality.rs
  role: examined
  excerpt: "    }\n\n    #[trace(\"TC-932\", \"FR-375-AC-9\")]\n    #[test]\n    fn kernel_result_bearing_evaluation_returns_relation_plan_and_charge_order()\
    \ {\n        for (first_head_unequal, expected) in [(false, true), (true, false)] {\n            let (left,\
    \ right) = e21_list_shape_operands(first_head_unequal);\n            assert_eq!(left.occ(), Integer::from(16_u64));"
  lines: '436'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/equality.rs#tests::pair_and_result_refusals_stop_at_the_first_denied_charge
  path: src/equality.rs
  role: examined
  excerpt: "\n    #[cfg(feature = \"test-support\")]\n    #[trace(\"TC-932\", \"FR-375-AC-10\")]\n    #[test]\n\
    \    fn pair_and_result_refusals_stop_at_the_first_denied_charge() {\n        let (left, right) = e21_list_shape_operands(true);\n\
    \        let pair_denial = InjectedDenial {\n            point: ChargePoint::EqualityPair,"
  lines: '527'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#different_declarations_are_unequal_with_one_pair_in_either_order
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-1\n#[trace(\"TC-924\", \"FR-376-AC-1\")]\n#[test]\nfn different_declarations_are_unequal_with_one_pair_in_either_order()\
    \ {\n    let cases = [\n        (p100(), c100()),\n        (p200(), c100()),"
  lines: '169'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#different_declarations_charge_the_one_pair_schedule
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-2\n#[trace(\"TC-924\", \"FR-376-AC-2\")]\n#[test]\nfn different_declarations_charge_the_one_pair_schedule()\
    \ {\n    let (left, right) = (p100(), c100());\n    let mut meter = meter();\n    assert_eq!("
  lines: '190'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#skipped_slots_never_report_a_foreign_reference
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-3\n#[trace(\"TC-924\", \"FR-376-AC-3\")]\n#[test]\nfn skipped_slots_never_report_a_foreign_reference()\
    \ {\n    let p_ref = rec(P, vec![reference(1)]);\n    let c_ref = rec(C, vec![reference(2)]);\n    let (count,\
    \ equal) = pairs(&p_ref, &c_ref).expect(\"slots are not visited\");"
  lines: '217'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#same_declaration_compares_by_slots_as_before
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-4\n#[trace(\"TC-924\", \"FR-376-AC-4\")]\n#[test]\nfn same_declaration_compares_by_slots_as_before()\
    \ {\n    assert_eq!(\n        pairs(&p100(), &p100()).unwrap(),\n        (Integer::from(2_u64), true)"
  lines: '238'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#different_declarations_inside_wrappers_are_unequal
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-5\n#[trace(\"TC-924\", \"FR-376-AC-5\")]\n#[test]\nfn different_declarations_inside_wrappers_are_unequal()\
    \ {\n    for (name, left, right) in wrappers(&p100(), &c100()) {\n        for (a, b) in [(&left, &right), (&right,\
    \ &left)] {\n            assert_eq!("
  lines: '256'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#faults_that_remain_still_refuse
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-6\n#[trace(\"TC-924\", \"FR-376-AC-6\")]\n#[test]\nfn faults_that_remain_still_refuse()\
    \ {\n    let populations = || {\n        (\n            Value::Population(PopulationId::from_digest([1; 32])),"
  lines: '278'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#member_equal_of_different_declarations_is_false
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-7\n#[trace(\"TC-924\", \"FR-376-AC-7\")]\n#[test]\nfn member_equal_of_different_declarations_is_false()\
    \ {\n    let mut meter = meter();\n    assert_eq!(\n        member_equal(&c100(), &p100(), &mut meter).completed(),"
  lines: '320'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#ordered_set_keeps_supply_order_across_declarations
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-376-AC-8\n#[trace(\"TC-924\", \"FR-376-AC-8\")]\n#[test]\nfn ordered_set_keeps_supply_order_across_declarations()\
    \ {\n    let (order, _meter) = ordered_set_form(vec![c100(), p100()]);\n    assert_eq!(order, keys(&[C, P]));\n\
    \    #[cfg(feature = \"test-support\")]"
  lines: '364'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#union_payloads_of_different_declarations_are_unequal_without_a_fault
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-376-AC-9\")]\n#[test]\nfn union_payloads_of_different_declarations_are_unequal_without_a_fault()\
    \ {\n    for (left, right) in [(p100(), c100()), (c100(), g100()), (p100(), g100())] {\n        for (a, b) in\
    \ [(&left, &right), (&right, &left)] {\n            let (a, b) = (holding(a.clone()), holding(b.clone()));"
  lines: '93'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#skipped_payload_slots_of_other_declarations_never_report_a_foreign_reference
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-376-AC-9\")]\n#[test]\nfn skipped_payload_slots_of_other_declarations_never_report_a_foreign_reference()\
    \ {\n    let left = holding(rec(P, vec![reference(1)]));\n    let right = holding(rec(C, vec![reference(2)]));\n\
    \    assert!(!equal(&left, &right));"
  lines: '114'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#traversal_does_not_short_circuit_after_a_declaration_mismatch
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-376-AC-9\")]\n#[test]\nfn traversal_does_not_short_circuit_after_a_declaration_mismatch()\
    \ {\n    let member = member(\"Pair\");\n    let build = |first: Value, second: i64| {\n        UnionValue::from_admitted("
  lines: '127'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#union_payload_pair_inside_another_value_leaves_enclosing_counts
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-376-AC-9\")]\n#[test]\nfn union_payload_pair_inside_another_value_leaves_enclosing_counts()\
    \ {\n    let option = |payload: Value| {\n        quire_exact::OptionValue::from_admitted(ValueType::Composite(key(U)),\
    \ Some(payload))\n    };"
  lines: '143'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#unlike_union_pairings_stay_checked_invariant_faults
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-376-AC-9\")]\n#[test]\nfn unlike_union_pairings_stay_checked_invariant_faults() {\n\
    \    let kind = |outcome: Outcome<bool>| match outcome {\n        Outcome::Refused(Refusal::CheckedInvariant\
    \ { cause }) => cause,\n        other => panic!(\"expected a checked invariant, got {other:?}\"),"
  lines: '155'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#declaration_key_orders_before_slots
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-377-AC-1\n#[trace(\"TC-925\", \"FR-377-AC-1\")]\n#[test]\nfn declaration_key_orders_before_slots()\
    \ {\n    let cases = [\n        (p100(), c100()),\n        (p200(), c100()),"
  lines: '390'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#first_differing_declaration_byte_decides
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-377-AC-2\n#[trace(\"TC-925\", \"FR-377-AC-2\")]\n#[test]\nfn first_differing_declaration_byte_decides()\
    \ {\n    let with_last = |last: u8| {\n        let mut digest = [1_u8; 32];\n        digest[31] = last;"
  lines: '407'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#same_declaration_keeps_slot_order_and_distinct_declarations_never_equal
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-377-AC-3\n#[trace(\"TC-925\", \"FR-377-AC-3\")]\n#[test]\nfn same_declaration_keeps_slot_order_and_distinct_declarations_never_equal()\
    \ {\n    assert_eq!(ordering(&p100(), &p100()), Some(Ordering::Equal));\n    assert_eq!(ordering(&p100(), &p200()),\
    \ Some(Ordering::Less));\n    assert_eq!(ordering(&c100(), &c100()), Some(Ordering::Equal));"
  lines: '435'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#set_and_bag_visit_the_smaller_declaration_key_first
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-377-AC-4\n#[trace(\"TC-925\", \"FR-377-AC-4\")]\n#[test]\nfn set_and_bag_visit_the_smaller_declaration_key_first()\
    \ {\n    for kind in [CollectionKind::Set, CollectionKind::Bag] {\n        for grouped in [false, true] {\n\
    \            for supply in [vec![c100(), p100()], vec![p100(), c100()]] {"
  lines: '457'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#equal_keys_exactly_when_values_are_equal
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-377-AC-5\n#[trace(\"TC-925\", \"FR-377-AC-5\")]\n#[test]\nfn equal_keys_exactly_when_values_are_equal()\
    \ {\n    let values = [c100(), p100(), p200(), g100()];\n    for x in &values {\n        for y in &values {"
  lines: '482'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#pairs_without_a_key_stay_unkeyed_and_refuse
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-377-AC-6\n#[trace(\"TC-925\", \"FR-377-AC-6\")]\n#[test]\nfn pairs_without_a_key_stay_unkeyed_and_refuse()\
    \ {\n    let float = || {\n        rec(\n            P,"
  lines: '500'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#declaration_order_holds_inside_wrappers
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-377-AC-7\n#[trace(\"TC-925\", \"FR-377-AC-7\")]\n#[test]\nfn declaration_order_holds_inside_wrappers()\
    \ {\n    for (name, left, right) in wrappers(&p100(), &c100()) {\n        assert_eq!(ordering(&left, &right),\
    \ Some(Ordering::Less), \"{name}\");\n        assert_eq!(ordering(&right, &left), Some(Ordering::Greater), \"\
    {name}\");"
  lines: '540'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#union_payload_declarations_order_by_declaration_key_bytes
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-377-AC-8\")]\n#[test]\nfn union_payload_declarations_order_by_declaration_key_bytes()\
    \ {\n    let (p, c, g) = (holding(p100()), holding(c100()), holding(g100()));\n    assert_eq!(compare_keys(&p,\
    \ &c), Some(Ordering::Less));\n    assert_eq!(compare_keys(&c, &p), Some(Ordering::Greater));"
  lines: '187'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#unlike_unions_have_no_key_and_refuse_equality
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-377-AC-8\", \"FR-379-AC-5\")]\n#[test]\nfn unlike_unions_have_no_key_and_refuse_equality()\
    \ {\n    let other_declaration = UnionValue::from_admitted(\n        UnionMember::from_admitted(\n         \
    \   key(U + 1),"
  lines: '215'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#union_payload_key_order_sorts_set_and_keeps_ordered_set_supply_order
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-377-AC-8\")]\n#[test]\nfn union_payload_key_order_sorts_set_and_keeps_ordered_set_supply_order()\
    \ {\n    let element = ValueType::Composite(key(U));\n    for supply in [\n        vec![holding(c100()), holding(p100()),\
    \ holding(c100())],"
  lines: '238'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#form_and_form_grouped_keep_both_declarations_unchanged
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-378-AC-1\n#[trace(\"TC-926\", \"FR-378-AC-1\")]\n#[test]\nfn form_and_form_grouped_keep_both_declarations_unchanged()\
    \ {\n    for kind in all_kinds() {\n        for grouped in [false, true] {\n            let mut meter = meter();"
  lines: '561'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#form_over_a_set_charges_one_member_comparison_and_no_element
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-378-AC-2\n#[trace(\"TC-926\", \"FR-378-AC-2\")]\n#[test]\nfn form_over_a_set_charges_one_member_comparison_and_no_element()\
    \ {\n    let mut meter = meter();\n    let outcome = form(\n        &coll_type(CollectionKind::Set),"
  lines: '586'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#from_admitted_option_stores_a_descendant_payload_unchanged
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-378-AC-3\n#[trace(\"TC-926\", \"FR-378-AC-3\")]\n#[test]\nfn from_admitted_option_stores_a_descendant_payload_unchanged()\
    \ {\n    let option = OptionValue::from_admitted(ValueType::Composite(key(P)), Some(c100()));\n    let Value::Option(option)\
    \ = option else {\n        panic!(\"option expected\");"
  lines: '616'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#from_admitted_slots_keeps_a_descendant_slot_and_retains_once
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-378-AC-4\n#[trace(\"TC-926\", \"FR-378-AC-4\")]\n#[test]\nfn from_admitted_slots_keeps_a_descendant_slot_and_retains_once()\
    \ {\n    let with_c = rec(R, vec![FieldValue::Present(c100())]);\n    let with_p = rec(R, vec![FieldValue::Present(p100())]);\n\
    \    let mut meter = meter();"
  lines: '641'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#exact_paths_refuse_a_descendant_and_accept_the_declared_type
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-378-AC-5\n#[trace(\"TC-926\", \"FR-378-AC-5\")]\n#[test]\nfn exact_paths_refuse_a_descendant_and_accept_the_declared_type()\
    \ {\n    let collection_type = coll_type(CollectionKind::Sequence);\n    let element = ValueType::Composite(key(P));\n\
    \    let field = MemberId::from_digest([9; 32]);"
  lines: '676'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/composite_declarations.rs#collections_are_not_covariant_and_stored_collections_behave
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "\n/// Trace: FR-378-AC-6\n#[trace(\"TC-926\", \"FR-378-AC-6\")]\n#[test]\nfn collections_are_not_covariant_and_stored_collections_behave()\
    \ {\n    let set_of = |element: u8, members: Vec<Value>| {\n        let collection_type = CollectionType::new(\n\
    \            CollectionKind::Set,"
  lines: '745'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#admitted_union_keeps_a_descendant_payload_and_exact_paths_refuse_it
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-378-AC-8\")]\n#[test]\nfn admitted_union_keeps_a_descendant_payload_and_exact_paths_refuse_it()\
    \ {\n    let shape = [ValueType::Composite(key(P))];\n    let admitted = UnionValue::from_admitted(member(\"\
    Held\"), vec![c100()]);\n    let Value::Union(stored) = &admitted else {"
  lines: '285'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#union_construction_keeps_identity_positions_and_occurrences
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-1\")]\n#[test]\nfn union_construction_keeps_identity_positions_and_occurrences()\
    \ {\n    let nullary = make(empty(), &[]);\n    let rect = make(fixture(\"Rect\", 1), &[2, 3]);\n    assert_eq!(nullary.occ(),\
    \ Integer::one());"
  lines: '88'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#union_position_and_arity_refusals_are_typed
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-2\")]\n#[test]\nfn union_position_and_arity_refusals_are_typed() {\n    let\
    \ cases = [\n        (\n            vec![],"
  lines: '108'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#evaluated_union_preserves_deferred_fault_and_first_stop
  path: tests/union.rs
  role: examined
  excerpt: "\n/// Kernel construction-stop subset; caller `case` evaluation remains external.\n#[trace(\"FR-379-AC-3\"\
    )]\n#[test]\nfn evaluated_union_preserves_deferred_fault_and_first_stop() {\n    let calls = std::cell::Cell::new(0);\n\
    \    let payload: Vec<Deferred<'_>> = vec![\n        Box::new(|_| {"
  lines: '403'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#evaluated_union_orders_arguments_and_charges_one_complete_result
  path: tests/union.rs
  role: examined
  excerpt: "\n/// Kernel construction accounting subset; caller `case` charges remain external.\n#[trace(\"FR-379-AC-3\"\
    )]\n#[test]\nfn evaluated_union_orders_arguments_and_charges_one_complete_result() {\n    let calls = std::cell::RefCell::new(Vec::new());\n\
    \    let payload: Vec<Deferred<'_>> = vec![\n        Box::new(|_| {"
  lines: '440'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#union_equality_compares_members_and_payload_positions
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-4\")]\n#[test]\nfn union_equality_compares_members_and_payload_positions()\
    \ {\n    let rect = make(fixture(\"Rect\", 1), &[2, 3]);\n    assert!(equal(&rect, &make(fixture(\"Rect\", 1),\
    \ &[2, 3])));\n    assert!(!equal(&rect, &make(fixture(\"Rect\", 1), &[3, 2])));"
  lines: '145'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#foreign_union_enum_and_composite_kinds_remain_distinct
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-5\")]\n#[test]\nfn foreign_union_enum_and_composite_kinds_remain_distinct()\
    \ {\n    let left = make(circle(), &[1]);\n    let foreign = UnionMember::from_admitted(\n        NodeKey::from_digest([0xD2;\
    \ 32]),"
  lines: '163'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#union_keys_order_identifier_ascii_then_payload_positions
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-6\")]\n#[test]\nfn union_keys_order_identifier_ascii_then_payload_positions()\
    \ {\n    let one = make(fixture(\"Rect\", 1), &[1]);\n    let other = make(fixture(\"Rect\", 2), &[1]);\n  \
    \  assert_eq!(compare_keys(&one, &other), None);"
  lines: '199'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#member_declaration_order_and_digest_order_do_not_reach_union_key_order
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-6\")]\n#[test]\nfn member_declaration_order_and_digest_order_do_not_reach_union_key_order()\
    \ {\n    // Two independent admitted declaration fixtures. Neither position nor\n    // digest order is the\
    \ identifier order; identity remains package scoped.\n    for (declaration, bindings) in ["
  lines: '303'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#union_collections_sort_deduplicate_and_preserve_first_occurrence
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-7\")]\n#[test]\nfn union_collections_sort_deduplicate_and_preserve_first_occurrence()\
    \ {\n    for reversed in [false, true] {\n        for kind in [\n            CollectionKind::Set,"
  lines: '251'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#ieee_union_payload_has_no_key_and_sequence_keeps_occurrences
  path: tests/union.rs
  role: examined
  excerpt: "/// Trusted kernel keylessness and sequence storage only; S3 type-reference\n/// eligibility in QSL\
    \ FR-323-AC-3 requires a checker witness outside this test.\n#[trace(\"FR-379-AC-7\")]\n#[test]\nfn ieee_union_payload_has_no_key_and_sequence_keeps_occurrences()\
    \ {\n    let value = UnionValue::from_admitted(\n        fixture(\"Measured\", 9),\n        vec![Value::Float(IeeeValue::binary64(0x3ff0_0000_0000_0000))],"
  lines: '358'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#ten_thousand_union_levels_clone_compare_format_and_drop_on_small_stack
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-8\")]\n#[test]\nfn ten_thousand_union_levels_clone_compare_format_and_drop_on_small_stack()\
    \ {\n    std::thread::Builder::new()\n        .stack_size(512 * 1024)\n        .spawn(|| {"
  lines: '527'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#shared_union_children_count_each_occurrence_and_clone_shares_root
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-8\")]\n#[test]\nfn shared_union_children_count_each_occurrence_and_clone_shares_root()\
    \ {\n    let child = deep(1_000);\n    let root = UnionValue::from_admitted(fixture(\"Pair\", 7), vec![child.clone(),\
    \ child]);\n    assert_eq!(root.occ(), Integer::from(4_003_u64));"
  lines: '558'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#union_storage_grows_linearly_with_depth
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-8\")]\n#[test]\nfn union_storage_grows_linearly_with_depth() {\n    let retained\
    \ = |levels| {\n        let mut value = None;\n        let allocation = allocation_counter::measure(|| value\
    \ = Some(deep(levels)));"
  lines: '576'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union.rs#mixed_union_composite_collection_paths_walk_and_drop_on_small_stack
  path: tests/union.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-8\")]\n#[test]\nfn mixed_union_composite_collection_paths_walk_and_drop_on_small_stack()\
    \ {\n    std::thread::Builder::new()\n        .stack_size(512 * 1024)\n        .spawn(|| {"
  lines: '596'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/union_declarations.rs#option_of_union_admits_compares_and_keys_through_the_payload
  path: tests/union_declarations.rs
  role: examined
  excerpt: "}\n\n#[trace(\"FR-379-AC-9\")]\n#[test]\nfn option_of_union_admits_compares_and_keys_through_the_payload()\
    \ {\n    let option_type = ValueType::Composite(key(U));\n    let present = |payload: Value| OptionValue::present(option_type.clone(),\
    \ payload).unwrap();\n    let a = present(holding(p100()));"
  lines: '324'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_membership.rs#tests::inclusive_integer_and_reduced_rational_membership_preserves_inputs
  path: src/numeric_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-380-AC-1\n    #[test]\n    fn inclusive_integer_and_reduced_rational_membership_preserves_inputs()\
    \ {\n        let cancel = Cancel::new();\n        let integer_domain = interval(Integer::from(-3_i64), Integer::from(5_i64));\n\
    \        let original_integer_domain = integer_domain.clone();"
  lines: '311'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_membership.rs#tests::exact_ceiling_admits_every_position_and_one_over_names_it
  path: src/numeric_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-380-AC-2\n    #[test]\n    fn exact_ceiling_admits_every_position_and_one_over_names_it()\
    \ {\n        let cancel = Cancel::new();\n        let exact = Integer::one().shifted_left(7);\n        let over\
    \ = Integer::one().shifted_left(8);"
  lines: '361'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_membership.rs#tests::borrowed_inline_and_large_comparisons_stay_within_limb_ceiling_without_allocation
  path: src/numeric_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-380-AC-3, FR-380-AC-5\n    #[test]\n    fn borrowed_inline_and_large_comparisons_stay_within_limb_ceiling_without_allocation()\
    \ {\n        for bits in [32_u64, 33, 128, 129, 257] {\n            let value = Integer::one().shifted_left(bits\
    \ - 1).add(&Integer::one());\n            let integer_domain = interval(value.clone(), value.clone());"
  lines: '474'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_membership.rs#tests::original_handle_cancels_before_work_and_at_a_late_limb_step
  path: src/numeric_membership.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-380-AC-4\n    #[test]\n    fn original_handle_cancels_before_work_and_at_a_late_limb_step()\
    \ {\n        let value = Integer::one().shifted_left(256).add(&Integer::one());\n        let integer_domain\
    \ = interval(value.clone(), value.clone());\n        let rational_value = rational(value.clone(), value.add(&Integer::from(2_i64)));"
  lines: '524'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::arithmetic_endpoint_equality_preserves_borrowed_domains
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-381-AC-1\n    #[test]\n    fn arithmetic_endpoint_equality_preserves_borrowed_domains()\
    \ {\n        let cancel = Cancel::new();\n        let expected = interval(Integer::from(-7_i64), Integer::zero());\n\
    \        let same = interval("
  lines: '358'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::integer_positions_accept_exact_b_and_report_first_one_over_b
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-381-AC-2\n    #[test]\n    fn integer_positions_accept_exact_b_and_report_first_one_over_b()\
    \ {\n        let cancel = Cancel::new();\n        let exact = Integer::one().shifted_left(7);\n        let over\
    \ = Integer::one().shifted_left(8);"
  lines: '420'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::rational_positions_accept_exact_b_and_report_first_one_over_b
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-381-AC-2\n    #[test]\n    fn rational_positions_accept_exact_b_and_report_first_one_over_b()\
    \ {\n        let cancel = Cancel::new();\n        let exact = Integer::one().shifted_left(7);\n        let over\
    \ = Integer::one().shifted_left(8);"
  lines: '485'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::borrowed_limb_work_and_storage_are_bounded_at_repr_boundaries
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-381-AC-3, FR-381-AC-5\n    #[test]\n    fn borrowed_limb_work_and_storage_are_bounded_at_repr_boundaries()\
    \ {\n        for bits in [32_u64, 33, 128, 129, 257] {\n            let value = Integer::one().shifted_left(bits\
    \ - 1).add(&Integer::one());\n            let integer = interval(value.neg(), value.clone());"
  lines: '594'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::original_cancel_preempts_size_and_late_comparison_then_retry_completes
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-381-AC-4, FR-381-AC-5\n    #[test]\n    fn original_cancel_preempts_size_and_late_comparison_then_retry_completes()\
    \ {\n        let value = Integer::one().shifted_left(256).add(&Integer::one());\n        let integer = interval(value.neg(),\
    \ value.clone());\n        let rational = domain(value.neg(), value.clone(), value.clone(), value.clone());"
  lines: '642'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::retained_decimal_fields_decide_equality_without_changing_borrowed_inputs
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-382-AC-1\n    #[test]\n    fn retained_decimal_fields_decide_equality_without_changing_borrowed_inputs()\
    \ {\n        let expected = decimal(\n            Integer::from(-7_i64),\n            Integer::from(7_i64),"
  lines: '691'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::decimal_endpoints_accept_exact_b_and_report_first_one_over_b
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-382-AC-2\n    #[test]\n    fn decimal_endpoints_accept_exact_b_and_report_first_one_over_b()\
    \ {\n        let cancel = Cancel::new();\n        let exact = Integer::one().shifted_left(7);\n        let over\
    \ = Integer::one().shifted_left(8);"
  lines: '759'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::decimal_borrowed_limb_work_and_storage_stay_bounded
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-382-AC-3\n    #[test]\n    fn decimal_borrowed_limb_work_and_storage_stay_bounded()\
    \ {\n        for bits in [32_u64, 33, 128, 129, 257] {\n            let value = Integer::one().shifted_left(bits\
    \ - 1).add(&Integer::one());\n            let expected = decimal(value.neg(), value.clone(), 0, 2, RoundingMode::Exact);"
  lines: '856'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: src/numeric_type_equality.rs#tests::decimal_original_cancel_preempts_size_and_late_comparison
  path: src/numeric_type_equality.rs
  role: examined
  excerpt: "    }\n\n    /// Trace: FR-382-AC-4\n    #[test]\n    fn decimal_original_cancel_preempts_size_and_late_comparison()\
    \ {\n        let value = Integer::one().shifted_left(256).add(&Integer::one());\n        let expected = decimal(value.neg(),\
    \ value, 0, 2, RoundingMode::Exact);\n        for cause in [CancelCause::Requested, CancelCause::Deadline] {"
  lines: '890'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir895_meter_cancel.rs#accessor_reports_only_the_installed_handle
  path: tests/ir895_meter_cancel.rs
  role: examined
  excerpt: "}\n\n#[trace(\"TC-933\", \"FR-383-AC-1\")]\n#[test]\nfn accessor_reports_only_the_installed_handle()\
    \ {\n    let first = Cancel::new();\n    let replacement = Cancel::new();\n    let independent = Cancel::new();"
  lines: '26'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: tests/ir895_meter_cancel.rs#accessor_handle_cancels_the_next_charge_without_changing_accounting
  path: tests/ir895_meter_cancel.rs
  role: examined
  excerpt: "}\n\n#[trace(\"TC-933\", \"FR-383-AC-2\")]\n#[test]\nfn accessor_handle_cancels_the_next_charge_without_changing_accounting()\
    \ {\n    let point = ChargePoint::FunctionCall;\n    let charge = || Charge::new(point).work(Integer::from(3_u64)).results(2);\n\
    \    let installed = Cancel::new();"
  lines: '46'
  basis: Exact declaration selected by computed matrix; mechanical binding only.
- id: normalization-dependency
  path: Cargo.toml
  role: examined
  excerpt: '# The Unicode 17 text definition requires 0.1.25''s tables; 0.1.26 uses Unicode 18.

    unicode-normalization = { version = "=0.1.25", default-features = false }'
  lines: 31-32
- id: fresh-consumer-trace
  path: tests/fresh_unicode_consumer.rs
  role: examined
  excerpt: '/// Trace: FR-141-AC-1'
  lines: 6-6
- id: external-requirement-namespace
  path: src/lib.rs
  role: context_only
  excerpt: '//!

    //! Value-semantics requirements (decimals, text and enumerations, quantities

    //! and units, integer division, IEEE profiles) are owned by

    //! `agent-ix/quire-specification`. A test carries a `QSpec-` tagged AC only

    //! where its own assertions, not merely its module doc''s citation, tell the

    //! AC''s claim from a wrong implementation. The requirements this repository

    //! owns are in `spec/`.'
  lines: 44-50
- id: FR-141-AC-1
  path: /home/peter/dev/quire-specification/spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md
  role: examined
  excerpt: Canonically equivalent text compares according to the selected normalization profile and distinct text
    remains distinct.
  lines: '119'
  artifact_type: FR criterion
- id: FR-141-profile-definition
  path: /home/peter/dev/quire-specification/spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md
  role: context_only
  excerpt: '

    ## Text and enumeration profiles


    `unicode-scalars` compares the decoded scalar sequence without normalization;

    `nfc`, `nfd`, `nfkc` and `nfkd` apply exactly the named Unicode normalization

    form using the `quire.value.text.unicode-17.0.0/v1` tables; `binary-utf8`

    compares the admitted UTF-8 payload bytes. A source string''s payload is the

    canonical UTF-8 encoding of its decoded scalar sequence, so raw `é` and'
  lines: 38-45
- id: TC-186-rust-boundary
  path: /home/peter/dev/quire-specification/spec/test-cases/TC-186-text-and-enum-identity-semantics.md
  role: context_only
  excerpt: '

    Select language `ix:native`, edition `1-draft`, and the

    `quire.value.complete/v1` definition and the

    `quire.value.text.unicode-17.0.0/v1` definition through

    [`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).

    Execute these vectors through the real

    local Rust boundary:'
  lines: 17-23
- id: suspicion-inspection:src/identity.rs#fn equal_digest_bytes_mint_interchangeable_identities()
  path: src/identity.rs
  role: examined
  excerpt: "    fn equal_digest_bytes_mint_interchangeable_identities() {\n        macro_rules! check {\n      \
    \      ($ty:ident) => {\n                let a = $ty::from_digest(digest(1));\n                let b = $ty::from_digest(digest(1));\n\
    \                let c = $ty::from_digest(digest(2));\n                assert_eq!(a, b);\n                assert_eq!(a.as_bytes(),\
    \ b.as_bytes());\n                assert_eq!(std::collections::HashSet::from([a, b, c]).len(), 2);"
  lines: 303-311
- id: suspicion-inspection:tests/ir653_scalar.rs#fn integer_arithmetic_matches_checked_i128_oracle()
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: "fn integer_arithmetic_matches_checked_i128_oracle() {\n    let values = samples();\n    for &a in &values\
    \ {\n        let left = Integer::from(a);\n        if let Some(expected) = a.checked_neg() {\n            assert_eq!(\n\
    \                completed_integer(IntegerArithmetic::Negate(&left)),\n                Integer::from(expected)\n\
    \            );"
  lines: 151-159
- id: suspicion-inspection:tests/ir673_decimal.rs#fn decimal_half_ties_follow_all_six_rounding_modes()
  path: tests/ir673_decimal.rs
  role: examined
  excerpt: "fn decimal_half_ties_follow_all_six_rounding_modes() {\n    for (mode, positive, negative) in [\n  \
    \      (RoundingMode::Exact, None, None),\n        (RoundingMode::TowardZero, Some(0), Some(0)),\n        (RoundingMode::TowardPositive,\
    \ Some(1), Some(0)),\n        (RoundingMode::TowardNegative, Some(0), Some(-1)),\n        (RoundingMode::NearestEven,\
    \ Some(0), Some(0)),\n        (RoundingMode::NearestAway, Some(1), Some(-1)),\n    ] {"
  lines: 105-113
- id: 'suspicion-inspection:tests/composite_declarations.rs#fn key(byte: u8)'
  path: tests/composite_declarations.rs
  role: examined
  excerpt: "fn key(byte: u8) -> NodeKey {\n    NodeKey::from_digest([byte; 32])\n}\n\nconst P: u8 = 1;\nconst C:\
    \ u8 = 2;\nconst G: u8 = 3;\nconst W: u8 = 4;\nconst R: u8 = 5;"
  lines: 33-41
- id: 'suspicion-inspection:tests/union_declarations.rs#fn key(byte: u8)'
  path: tests/union_declarations.rs
  role: examined
  excerpt: "fn key(byte: u8) -> NodeKey {\n    NodeKey::from_digest([byte; 32])\n}\n\nfn meter() -> Meter {\n  \
    \  Meter::new(ScalarLimits {\n        integer_bits: u64::MAX,\n        decimal_digits: u64::MAX,\n        scale_expansion:\
    \ u64::MAX,"
  lines: 23-31
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-359-AC-9
  related_ids: []
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: '37'
  excerpt: 'PLANNED, deferred to IR-844 (u64 Meter redesign). The planned `cfg(kani)` harness, tagged `Trace: FR-359-AC-9`
    only once verified, uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand charge
    with at most two semantic-size requests, arithmetic charge with one, result-retain charge with none) over the
    full `i64` operand range, plus separate repeated-kind charge controls with at most two requests each. Symbolic
    `i64` operands and symbolic limits including `ResultUnits` cover first-short and simultaneous shortages and
    absence of the third admission. Under `'
  finding: The computed matrix has no test binding for the explicitly PLANNED Meter proof obligation; it remains
    deferred to IR-844 and is inherited unchanged from the base. Record that deferred proof gap rather than crediting
    Unicode tests as evidence.
  inherited: true
- fnd: FND-002
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-359-AC-10
  related_ids: []
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: '38'
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their own assertions
    under their original domains, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600
    seconds. Record their individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test or altered
    consumer harness substitutes for it.
  finding: The computed matrix has no binding or current external receipt for the five consumer Kani obligations;
    this inherited IR-705/IR-17 gap requires the genuine consumer qualification and cannot be closed by owner Unicode
    tests.
  inherited: true
- fnd: FND-003
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
  finding: The computed matrix has no binding or delivered healthy/mutant consumer qualification for the inherited
    representation-refinement consumer obligation. Keep its IR-705/IR-17 qualification outstanding; owner normalization
    tests provide no proof credit.
  inherited: true
- fnd: FND-004
  severity: low
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-383-AC-4
  related_ids: []
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  lines: '50'
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std` + `alloc`
    build.
  finding: The computed matrix reports the existing no_std signature criterion as untagged because its documented
    make build-no-std gate has no source Trace binder. This inherited structural evidence gap is distinct from missing
    implementation; reconcile it in the cancellation-accessor assurance scope rather than misreporting matrix strict
    as passing.
  inherited: true
bindings:
- test_id: tests::compare_keys_orders_members_of_one_shape_by_rank
  ac_id: FR-088-AC-11
  trace: correct
  path: src/key.rs
  line: 304
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::compare_keys_refuses_same_rank_different_declaration_members
  ac_id: FR-088-AC-11
  trace: correct
  path: src/key.rs
  line: 331
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::enum_shape_admits_only_its_own_variants
  ac_id: FR-088-AC-11
  trace: correct
  path: src/value.rs
  line: 1990
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::admits_refuses_a_known_variant_at_the_wrong_rank
  ac_id: FR-088-AC-11
  trace: correct
  path: src/value.rs
  line: 2008
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::enum_shape_rank_matches_canonical_position
  ac_id: FR-088-AC-11
  trace: correct
  path: src/value.rs
  line: 2026
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::a_set_visits_enum_values_in_canonical_rank_order
  ac_id: FR-088-AC-11
  trace: correct
  path: src/value.rs
  line: 2089
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes
  ac_id: FR-088-AC-12
  trace: correct
  path: src/identity.rs
  line: 382
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::plan_pairs_refuses_a_population_pair
  ac_id: FR-089-AC-6
  trace: correct
  path: src/equality.rs
  line: 585
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::compare_keys_yields_no_key_for_a_population_pair
  ac_id: FR-089-AC-6
  trace: correct
  path: src/key.rs
  line: 275
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::admits_refuses_a_population_pair
  ac_id: FR-089-AC-6
  trace: correct
  path: src/value.rs
  line: 1977
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_428_every_record_building_refusal_names_code_and_cause
  ac_id: FR-096-AC-8
  trace: correct
  path: src/outcome.rs
  line: 440
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: all_typed_causes_are_distinct_internal_faults_with_required_traits
  ac_id: FR-096-AC-8
  trace: correct
  path: tests/checked_invariant.rs
  line: 45
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_441_an_unbounded_collection_never_refuses_for_cardinality
  ac_id: FR-097-AC-7
  trace: correct
  path: src/collection.rs
  line: 625
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_441_an_unbounded_collection_stops_only_on_the_callers_meter
  ac_id: FR-097-AC-7
  trace: correct
  path: src/collection.rs
  line: 668
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: public_cardinality_query_is_inclusive_and_agrees_with_admission
  ac_id: FR-097-AC-8
  trace: correct
  path: tests/ir673_core.rs
  line: 30
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::a_deep_value_equals_its_clone_and_both_drop
  ac_id: FR-262-AC-2
  trace: correct
  path: src/value.rs
  line: 2697
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::a_deep_value_drops_on_a_small_stack
  ac_id: FR-262-AC-2
  trace: correct
  path: src/value.rs
  line: 2714
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_735_a_deep_option_type_clones_compares_hashes_formats_and_drops
  ac_id: FR-262-AC-2
  trace: correct
  path: src/value/value_type.rs
  line: 483
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_735_value_type_debug_matches_the_derived_format
  ac_id: FR-262-AC-2
  trace: correct
  path: src/value/value_type.rs
  line: 509
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_735_value_types_equal_and_hash_equal_their_clones_only
  ac_id: FR-262-AC-2
  trace: correct
  path: src/value/value_type.rs
  line: 522
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_735_the_debug_worklist_grows_by_a_constant_per_type_node
  ac_id: FR-262-AC-2
  trace: correct
  path: src/value/value_type.rs
  line: 551
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::type_nodes_grow_linearly_with_value_depth
  ac_id: FR-262-AC-3
  trace: correct
  path: src/value.rs
  line: 2861
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::a_deep_value_with_deep_types_is_admitted_in_linear_memory
  ac_id: FR-262-AC-3
  trace: correct
  path: src/value.rs
  line: 2873
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::division_by_zero_is_undefined
  ac_id: FR-357-AC-1
  trace: correct
  path: src/division.rs
  line: 264
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::only_the_exposed_member_must_be_in_domain
  ac_id: FR-357-AC-2
  trace: correct
  path: src/division.rs
  line: 280
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::exposed_member_outside_domain_refuses_with_its_cause
  ac_id: FR-357-AC-3
  trace: correct
  path: src/division.rs
  line: 294
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::profiles_differ_on_negative_operands
  ac_id: FR-357-AC-4
  trace: correct
  path: src/division.rs
  line: 317
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::i64_min_divided_by_minus_one_is_exact
  ac_id: FR-357-AC-5
  trace: correct
  path: src/division.rs
  line: 337
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::division_charges_one_domain_occurrence_and_one_result_unit
  ac_id: FR-357-AC-6
  trace: correct
  path: src/division.rs
  line: 364
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: modulo_is_euclidean_after_every_division_profile
  ac_id: FR-357-AC-7
  trace: correct
  path: tests/ir673_core.rs
  line: 74
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::injected_denial_spends_only_on_its_named_admitted_position
  ac_id: FR-358-AC-1
  trace: correct
  path: src/accounting.rs
  line: 1410
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::injected_plan_denial_spends_after_one_refusal
  ac_id: FR-358-AC-2
  trace: correct
  path: src/accounting.rs
  line: 1470
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::injected_denial_occurrence_requires_nonzero
  ac_id: FR-358-AC-3
  trace: correct
  path: src/accounting.rs
  line: 1699
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::charge_log_retains_only_first_4096_admissions
  ac_id: FR-358-AC-4
  trace: correct
  path: src/accounting.rs
  line: 1712
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::charge_log_retains_only_first_4096_admissions
  ac_id: FR-358-AC-5
  trace: correct
  path: src/accounting.rs
  line: 1712
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::production_meter_size_is_constant_as_charges_grow
  ac_id: FR-358-AC-6
  trace: correct
  path: src/accounting.rs
  line: 1300
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-8
  trace: correct
  path: tests/ir673_core.rs
  line: 105
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-9
  trace: correct
  path: tests/ir673_core.rs
  line: 105
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-10
  trace: correct
  path: tests/ir673_core.rs
  line: 105
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-11
  trace: correct
  path: tests/ir673_core.rs
  line: 105
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::injected_record_uses_precharge_work_and_plan_reservation
  ac_id: FR-358-AC-12
  trace: correct
  path: src/accounting.rs
  line: 1507
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::injected_denial_precedes_ordinary_size_and_plan_shortages
  ac_id: FR-358-AC-13
  trace: correct
  path: src/accounting.rs
  line: 1568
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: records_actual_sizes_coalesces_repeats_and_distinguishes_absent_zero
  ac_id: FR-358-AC-14
  trace: correct
  path: tests/ir667_normalize.rs
  line: 53
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ordinary_injected_and_cancelled_charges_preserve_admission_records
  ac_id: FR-358-AC-14
  trace: correct
  path: tests/ir667_normalize.rs
  line: 136
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: fixed_records_share_the_existing_prefix_and_truncation_boundary
  ac_id: FR-358-AC-15
  trace: correct
  path: tests/ir667_normalize.rs
  line: 217
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::cumulative_work_reaches_max_then_refuses_atomically
  ac_id: FR-359-AC-1
  trace: correct
  path: src/accounting.rs
  line: 975
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::cumulative_results_reach_max_then_refuse_atomically
  ac_id: FR-359-AC-2
  trace: correct
  path: src/accounting.rs
  line: 1006
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::exact_cumulative_amount_above_max_refuses_without_changes
  ac_id: FR-359-AC-3
  trace: correct
  path: src/accounting.rs
  line: 1034
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::exact_cumulative_amount_above_max_refuses_without_changes
  ac_id: FR-359-AC-4
  trace: correct
  path: src/accounting.rs
  line: 1034
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::result_refusal_preserves_prior_size_high_water
  ac_id: FR-359-AC-5
  trace: correct
  path: src/accounting.rs
  line: 1066
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: meter_uses_field_order_and_all_counters_are_queryable
  ac_id: FR-359-AC-6
  trace: correct
  path: tests/ir673_core.rs
  line: 209
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: meter_uses_field_order_and_all_counters_are_queryable
  ac_id: FR-359-AC-7
  trace: correct
  path: tests/ir673_core.rs
  line: 209
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::duplicate_size_requests_preserve_first_refusal_and_are_atomic
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1170
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::duplicate_sizes_coalesce_by_max_and_retain_explicit_zero
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1189
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::first_inserted_failing_size_precedes_later_same_kind_maximum
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1213
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::size_shortages_use_field_order_across_attachment_orders
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1244
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::cancellation_and_injection_precede_size_refusal
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1326
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::injected_denial_precedes_ordinary_size_and_plan_shortages
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1568
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::size_request_naming_a_cumulative_counter_is_validated_in_field_order
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1757
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::request_counts_from_zero_to_many_gain_no_denial
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1787
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::reordered_and_repeated_requests_keep_the_high_water_maximum
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1809
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::ordinary_refusal_leaves_a_pending_injected_occurrence_unspent
  ac_id: FR-359-AC-8
  trace: correct
  path: src/accounting.rs
  line: 1837
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_931_admitted_log_matches_model_ordered_prefix
  ac_id: FR-359-AC-12
  trace: correct
  path: src/accounting.rs
  line: 1918
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::tc_931_admitted_log_matches_model_same_kind
  ac_id: FR-359-AC-12
  trace: correct
  path: src/accounting.rs
  line: 1971
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::minimum_i64_absolute_value_is_positive_two_to_the_63rd
  ac_id: FR-360-AC-1
  trace: correct
  path: src/integer.rs
  line: 776
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: integer_arithmetic_denial_precedes_large_intermediate_even_when_result_cancels
  ac_id: FR-361-AC-1
  trace: correct
  path: tests/ir653_admission.rs
  line: 64
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_arithmetic_denial_precedes_large_unreduced_products
  ac_id: FR-361-AC-2
  trace: correct
  path: tests/ir653_admission.rs
  line: 91
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: division_and_modulus_denial_precedes_large_quotient
  ac_id: FR-361-AC-3
  trace: correct
  path: tests/ir653_admission.rs
  line: 123
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: decimal_scale_denial_precedes_power_of_ten_allocation
  ac_id: FR-361-AC-4
  trace: correct
  path: tests/ir653_admission.rs
  line: 186
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: decimal_arithmetic_denial_precedes_aligned_intermediate
  ac_id: FR-361-AC-5
  trace: correct
  path: tests/ir653_admission.rs
  line: 206
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: decimal_retain_denial_precedes_target_scale_expansion
  ac_id: FR-361-AC-6
  trace: correct
  path: tests/ir653_admission.rs
  line: 229
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: large_floor_quotient_completes_at_exact_operand_bit_limit_and_denies_below
  ac_id: FR-361-AC-7
  trace: correct
  path: tests/ir653_admission.rs
  line: 249
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: large_floor_quotient_completes_at_exact_operand_bit_limit_and_denies_below
  ac_id: FR-361-AC-8
  trace: correct
  path: tests/ir653_admission.rs
  line: 249
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: huge_decimal_target_denies_retain_before_power_allocation
  ac_id: FR-361-AC-9
  trace: correct
  path: tests/ir653_admission.rs
  line: 306
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: integer_arithmetic_matches_checked_i128_oracle
  ac_id: FR-362-AC-1
  trace: correct
  path: tests/ir653_scalar.rs
  line: 151
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: integer_arithmetic_retains_results_beyond_i128
  ac_id: FR-362-AC-2
  trace: correct
  path: tests/ir653_scalar.rs
  line: 182
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_arithmetic_matches_reduced_fraction_oracle
  ac_id: FR-362-AC-3
  trace: correct
  path: tests/ir653_scalar.rs
  line: 215
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: integer_and_rational_ordering_match_checked_oracle
  ac_id: FR-362-AC-4
  trace: correct
  path: tests/ir653_scalar.rs
  line: 294
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: scalar_families_charge_order_and_independent_size_maxima
  ac_id: FR-362-AC-5
  trace: correct
  path: tests/ir653_scalar.rs
  line: 376
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: scalar_families_charge_order_and_independent_size_maxima
  ac_id: FR-362-AC-6
  trace: correct
  path: tests/ir653_scalar.rs
  line: 376
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: three_direct_atoms_consume_seven_work_and_three_results
  ac_id: FR-362-AC-7
  trace: correct
  path: tests/ir653_scalar.rs
  line: 474
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: each_scalar_charge_point_can_stop_before_result
  ac_id: FR-362-AC-8
  trace: correct
  path: tests/ir653_scalar.rs
  line: 498
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: already_decided_boolean_connectives_follow_truth_tables_and_one_charge
  ac_id: FR-362-AC-9
  trace: correct
  path: tests/ir653_scalar.rs
  line: 585
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_normalize_records_unreduced_parts_and_exact_boundary_stops
  ac_id: FR-362-AC-10
  trace: correct
  path: tests/ir667_normalize.rs
  line: 359
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: integer_arithmetic_exact_bits_and_one_under_use_operand_formulas
  ac_id: FR-362-AC-10
  trace: correct
  path: tests/ir667_normalize.rs
  line: 487
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: integer_and_rational_ordering_exact_bits_and_one_under_stop_at_first_short_point
  ac_id: FR-362-AC-10
  trace: correct
  path: tests/ir667_normalize.rs
  line: 588
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: scalar_outcomes_keep_only_admitted_prefixes
  ac_id: FR-362-AC-11
  trace: correct
  path: tests/ir673_scalar.rs
  line: 223
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_constructor_reduces_sign_and_zero
  ac_id: FR-362-AC-12
  trace: correct
  path: tests/ir673_scalar.rs
  line: 348
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_domain_refuses_denominator_outside_interval
  ac_id: FR-362-AC-13
  trace: correct
  path: tests/ir673_scalar.rs
  line: 363
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: direct_countdown_atoms_stop_at_fifth_result
  ac_id: FR-362-AC-14
  trace: correct
  path: tests/ir673_scalar.rs
  line: 403
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: direct_rational_division_charges_four_points
  ac_id: FR-362-AC-15
  trace: correct
  path: tests/ir673_scalar.rs
  line: 515
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: direct_integer_additions_share_meter_without_fold
  ac_id: FR-362-AC-16
  trace: correct
  path: tests/ir673_scalar.rs
  line: 574
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: atom_work_boundaries_stop_on_named_points
  ac_id: FR-362-AC-17
  trace: correct
  path: tests/ir673_scalar.rs
  line: 612
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: powers_of_two_charge_operand_derived_bounds
  ac_id: FR-362-AC-18
  trace: correct
  path: tests/ir673_scalar.rs
  line: 704
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: public_integer_arithmetic_admits_exact_bits_and_denies_one_under
  ac_id: FR-362-AC-19
  trace: correct
  path: tests/ir653_scalar.rs
  line: 35
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_normalize_records_unreduced_parts_and_exact_boundary_stops
  ac_id: FR-362-AC-20
  trace: correct
  path: tests/ir667_normalize.rs
  line: 359
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: temporal_visit_charges_one_work_unit_and_respects_the_boundary
  ac_id: FR-362-AC-21
  trace: correct
  path: tests/ir873_temporal_visit.rs
  line: 53
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: injected_temporal_visit_denial_is_atomic_and_one_shot
  ac_id: FR-362-AC-22
  trace: correct
  path: tests/ir873_temporal_visit.rs
  line: 85
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: decimal_ordering_matches_independent_scaled_integer_oracle
  ac_id: FR-363-AC-1
  trace: correct
  path: tests/ir653_decimal_order.rs
  line: 41
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: decimal_ordering_charges_retained_zero_and_equal_value_representations
  ac_id: FR-363-AC-2
  trace: correct
  path: tests/ir653_decimal_order.rs
  line: 79
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: decimal_ordering_charges_retained_zero_and_equal_value_representations
  ac_id: FR-363-AC-3
  trace: correct
  path: tests/ir653_decimal_order.rs
  line: 79
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: huge_retained_scale_refuses_analytically_before_comparison
  ac_id: FR-363-AC-4
  trace: correct
  path: tests/ir653_decimal_order.rs
  line: 127
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: each_decimal_ordering_point_denies_its_suffix
  ac_id: FR-363-AC-5
  trace: correct
  path: tests/ir653_decimal_order.rs
  line: 178
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: normalized_equal_decimals_keep_distinct_retained_representations
  ac_id: FR-363-AC-6
  trace: correct
  path: tests/ir673_decimal.rs
  line: 37
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: metered_ordering_uses_retained_decimal_sizes
  ac_id: FR-363-AC-7
  trace: correct
  path: tests/ir673_decimal.rs
  line: 67
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ieee_flags_iterate_all_subsets_in_vocabulary_order
  ac_id: FR-364-AC-1
  trace: correct
  path: tests/ir673_ieee_text.rs
  line: 40
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: decimal_half_ties_follow_all_six_rounding_modes
  ac_id: FR-365-AC-1
  trace: correct
  path: tests/ir673_decimal.rs
  line: 105
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: default_decimal_rounding_is_strict_exact
  ac_id: FR-365-AC-2
  trace: correct
  path: tests/ir673_decimal.rs
  line: 147
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ieee_nan_selection_quiets_first_and_reports_later_signaling
  ac_id: FR-366-AC-1
  trace: correct
  path: tests/ir673_ieee_text.rs
  line: 65
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ieee_nan_width_conversion_refuses_unrepresentable_payload
  ac_id: FR-366-AC-2
  trace: correct
  path: tests/ir673_ieee_text.rs
  line: 129
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ieee_signed_zero_converts_to_canonical_rational_with_loss
  ac_id: FR-366-AC-3
  trace: correct
  path: tests/ir673_ieee_text.rs
  line: 168
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ieee_total_order_separates_ten_classes_and_signed_zero
  ac_id: FR-366-AC-4
  trace: correct
  path: tests/ir673_ieee_text.rs
  line: 191
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: text_profile_mismatch_refuses_before_any_comparison_charge
  ac_id: FR-367-AC-1
  trace: correct
  path: tests/ir673_ieee_text.rs
  line: 264
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: charge_point_vocabulary_is_unique_and_round_trips
  ac_id: FR-368-AC-1
  trace: correct
  path: tests/ir673_core.rs
  line: 353
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: limit_kind_vocabulary_matches_scalar_limit_fields
  ac_id: FR-368-AC-2
  trace: correct
  path: tests/ir673_core.rs
  line: 625
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: all_typed_causes_are_distinct_internal_faults_with_required_traits
  ac_id: FR-369-AC-1
  trace: correct
  path: tests/checked_invariant.rs
  line: 45
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: comparison_payloads_retain_the_original_typed_cause
  ac_id: FR-369-AC-1
  trace: correct
  path: tests/checked_invariant.rs
  line: 191
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause
  ac_id: FR-369-AC-1
  trace: correct
  path: tests/checked_invariant.rs
  line: 230
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::evaluated_element_outside_its_type_reports_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
  path: src/collection.rs
  line: 555
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::grouped_float_members_without_keys_report_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
  path: src/collection.rs
  line: 570
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::refused_interval_requires_a_bounded_domain
  ac_id: FR-369-AC-2
  trace: correct
  path: src/division.rs
  line: 225
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::plan_pairs_refuses_a_population_pair
  ac_id: FR-369-AC-2
  trace: correct
  path: src/equality.rs
  line: 585
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::incompatible_pairs_report_specific_causes_without_a_boolean
  ac_id: FR-369-AC-2
  trace: correct
  path: src/equality.rs
  line: 618
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::deferred_record_field_outside_its_type_reports_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
  path: src/value.rs
  line: 1918
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::deferred_tuple_position_outside_its_type_reports_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
  path: src/value.rs
  line: 1947
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: all_typed_causes_are_distinct_internal_faults_with_required_traits
  ac_id: FR-369-AC-5
  trace: correct
  path: tests/checked_invariant.rs
  line: 45
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: comparison_payloads_retain_the_original_typed_cause
  ac_id: FR-369-AC-5
  trace: correct
  path: tests/checked_invariant.rs
  line: 191
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause
  ac_id: FR-369-AC-5
  trace: correct
  path: tests/checked_invariant.rs
  line: 230
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_type_admission_has_no_alias
  ac_id: FR-370-AC-1
  trace: correct
  path: src/value.rs
  line: 1545
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_equality_uses_boolean_leaf_schedule
  ac_id: FR-370-AC-2
  trace: correct
  path: src/value.rs
  line: 1692
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_mixed_kind_plans_refuse_in_both_orders
  ac_id: FR-370-AC-3
  trace: correct
  path: src/value.rs
  line: 1630
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_keys_order_canonical_content_and_sets
  ac_id: FR-370-AC-4
  trace: correct
  path: src/value.rs
  line: 1744
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_type_admission_has_no_alias
  ac_id: FR-370-AC-5
  trace: correct
  path: src/value.rs
  line: 1545
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_mixed_kind_plans_refuse_in_both_orders
  ac_id: FR-370-AC-5
  trace: correct
  path: src/value.rs
  line: 1630
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_equality_uses_boolean_leaf_schedule
  ac_id: FR-370-AC-5
  trace: correct
  path: src/value.rs
  line: 1692
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_keys_order_canonical_content_and_sets
  ac_id: FR-370-AC-5
  trace: correct
  path: src/value.rs
  line: 1744
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::existing_kind_controls_remain_admitted_and_keyed
  ac_id: FR-370-AC-5
  trace: correct
  path: src/value.rs
  line: 1808
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::uuid_canonical_text_preserves_network_octets
  ac_id: FR-370-AC-6
  trace: correct
  path: src/value/native.rs
  line: 138
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::timestamp_canonical_text_preserves_full_signed_nanoseconds
  ac_id: FR-370-AC-7
  trace: correct
  path: src/value/native.rs
  line: 178
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::uuid_canonical_text_preserves_network_octets
  ac_id: FR-370-AC-8
  trace: correct
  path: src/value/native.rs
  line: 138
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::timestamp_canonical_text_preserves_full_signed_nanoseconds
  ac_id: FR-370-AC-8
  trace: correct
  path: src/value/native.rs
  line: 178
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_arithmetic_stays_inline_and_overflow_refines_exactly
  ac_id: FR-371-AC-1
  trace: correct
  path: src/integer.rs
  line: 814
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: inline_signed_bounds_refine_the_explicit_128_bit_domain
  ac_id: FR-371-AC-1
  trace: correct
  path: tests/ir705_bounded_integer.rs
  line: 66
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: construction_boundaries_and_sampled_arithmetic_match_independent_oracles
  ac_id: FR-371-AC-1
  trace: correct
  path: tests/ir705_bounded_integer.rs
  line: 81
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::cache_is_invisible_to_traits_clones_and_concurrent_borrows
  ac_id: FR-371-AC-2
  trace: correct
  path: src/integer.rs
  line: 851
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: metered_native_and_overflow_operations_keep_the_existing_charge_schedule
  ac_id: FR-371-AC-3
  trace: correct
  path: tests/ir705_bounded_integer.rs
  line: 177
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: metered_operation_matrix_preserves_fr362_amounts_work_and_denial_prefixes
  ac_id: FR-371-AC-3
  trace: correct
  path: tests/ir705_bounded_integer.rs
  line: 268
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_arithmetic_stays_inline_and_overflow_refines_exactly
  ac_id: FR-371-AC-4
  trace: correct
  path: src/integer.rs
  line: 814
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::magnitude_limb_stream_does_not_materialize_small_cache
  ac_id: FR-371-AC-4
  trace: correct
  path: src/integer.rs
  line: 953
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: fitting_integer_operations_and_cached_source_clone_allocate_nothing
  ac_id: FR-371-AC-4
  trace: correct
  path: tests/ir705_bounded_integer.rs
  line: 32
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: inline_signed_bounds_refine_the_explicit_128_bit_domain
  ac_id: FR-371-AC-4
  trace: correct
  path: tests/ir705_bounded_integer.rs
  line: 66
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: native_public_metered_call_matches_accounting_only_allocation_observation
  ac_id: FR-371-AC-4
  trace: correct
  path: tests/ir705_bounded_integer.rs
  line: 554
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::cache_is_invisible_to_traits_clones_and_concurrent_borrows
  ac_id: FR-371-AC-5
  trace: correct
  path: src/integer.rs
  line: 851
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::once_box_race_retains_one_candidate_and_releases_losers
  ac_id: FR-371-AC-5
  trace: correct
  path: src/integer.rs
  line: 893
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::inclusive_membership_preserves_representations_and_ignores_rounding
  ac_id: FR-372-AC-1
  trace: correct
  path: src/decimal_membership.rs
  line: 329
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::borrowed_limb_crossings_and_huge_shift_have_bounded_work
  ac_id: FR-372-AC-1
  trace: correct
  path: src/decimal_membership.rs
  line: 407
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::scale_sign_and_zero_paths_request_no_scratch
  ac_id: FR-372-AC-1
  trace: correct
  path: src/decimal_membership.rs
  line: 438
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::longest_alignment_reuses_one_exact_request_within_independent_envelope
  ac_id: FR-372-AC-2
  trace: correct
  path: src/decimal_membership.rs
  line: 382
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::borrowed_limb_crossings_and_huge_shift_have_bounded_work
  ac_id: FR-372-AC-2
  trace: correct
  path: src/decimal_membership.rs
  line: 407
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::scale_sign_and_zero_paths_request_no_scratch
  ac_id: FR-372-AC-2
  trace: correct
  path: src/decimal_membership.rs
  line: 438
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::actual_small_reservation_denial_is_typed_and_recoverable
  ac_id: FR-372-AC-3
  trace: correct
  path: src/decimal_membership.rs
  line: 465
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::native_layout_capacity_failure_is_separate_from_allocator_denial
  ac_id: FR-372-AC-3
  trace: correct
  path: src/decimal_membership.rs
  line: 501
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::original_precancelled_handle_prevents_the_actual_reservation
  ac_id: FR-372-AC-4
  trace: correct
  path: src/decimal_membership.rs
  line: 531
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::reservation_division_comparison_and_publication_cancellation_release_scratch
  ac_id: FR-372-AC-4
  trace: correct
  path: src/decimal_membership.rs
  line: 576
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-1
  trace: correct
  path: src/ieee.rs
  line: 1984
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-2
  trace: correct
  path: src/ieee.rs
  line: 1984
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::nonfinite_values_stop_after_operands
  ac_id: FR-373-AC-3
  trace: correct
  path: src/ieee.rs
  line: 2014
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::numerator_and_denominator_refusals_retain_the_declared_target
  ac_id: FR-373-AC-4
  trace: correct
  path: src/ieee.rs
  line: 2037
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-5
  trace: correct
  path: src/ieee.rs
  line: 1984
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::ordinary_and_injected_denials_preserve_the_admitted_prefix
  ac_id: FR-373-AC-5
  trace: correct
  path: src/ieee.rs
  line: 2054
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::subnormal_intermediates_require_the_exact_denominator_bits
  ac_id: FR-373-AC-5
  trace: correct
  path: src/ieee.rs
  line: 2111
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::original_cancellation_handle_stops_each_poll_and_live_handle_completes
  ac_id: FR-373-AC-6
  trace: correct
  path: src/ieee.rs
  line: 2160
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: rational_target_ieee::already_spent_meter_prefix_survives_success_and_every_retain_stop
  ac_id: FR-373-AC-7
  trace: correct
  path: src/ieee.rs
  line: 2202
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: admission_denies_zero_and_borrows_each_original_nonzero_integer
  ac_id: FR-374-AC-1
  trace: correct
  path: tests/nonzero_integer.rs
  line: 38
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: admitting_an_inline_integer_does_not_materialize_its_bigint
  ac_id: FR-374-AC-1
  trace: correct
  path: tests/nonzero_integer.rs
  line: 64
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: admission_denies_zero_and_borrows_each_original_nonzero_integer
  ac_id: FR-374-AC-2
  trace: correct
  path: tests/nonzero_integer.rs
  line: 38
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: admitted_token_has_no_default_construction
  ac_id: FR-374-AC-2
  trace: correct
  path: tests/nonzero_integer.rs
  line: 78
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: admitted_pair_helpers_preserve_signed_zero_and_large_results
  ac_id: FR-374-AC-3
  trace: correct
  path: tests/nonzero_integer.rs
  line: 91
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: exact_div_keeps_infallible_signed_results_and_nondivisible_truncation
  ac_id: FR-374-AC-4
  trace: correct
  path: tests/nonzero_integer.rs
  line: 125
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::canonical_nonzero_premise_rejects_zero_with_its_own_message
  ac_id: FR-374-AC-5
  trace: correct
  path: src/integer.rs
  line: 770
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: metered_zero_stops_after_operands_and_denial_precedes_zero_classification
  ac_id: FR-374-AC-6
  trace: correct
  path: tests/nonzero_integer.rs
  line: 222
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: metered_nonzero_retains_laws_single_member_domain_and_charge_requests
  ac_id: FR-374-AC-6
  trace: correct
  path: tests/nonzero_integer.rs
  line: 269
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::kernel_result_bearing_evaluation_returns_relation_plan_and_charge_order
  ac_id: FR-375-AC-9
  trace: correct
  path: src/equality.rs
  line: 436
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::pair_and_result_refusals_stop_at_the_first_denied_charge
  ac_id: FR-375-AC-10
  trace: correct
  path: src/equality.rs
  line: 527
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: different_declarations_are_unequal_with_one_pair_in_either_order
  ac_id: FR-376-AC-1
  trace: correct
  path: tests/composite_declarations.rs
  line: 169
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: different_declarations_charge_the_one_pair_schedule
  ac_id: FR-376-AC-2
  trace: correct
  path: tests/composite_declarations.rs
  line: 190
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: skipped_slots_never_report_a_foreign_reference
  ac_id: FR-376-AC-3
  trace: correct
  path: tests/composite_declarations.rs
  line: 217
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: same_declaration_compares_by_slots_as_before
  ac_id: FR-376-AC-4
  trace: correct
  path: tests/composite_declarations.rs
  line: 238
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: different_declarations_inside_wrappers_are_unequal
  ac_id: FR-376-AC-5
  trace: correct
  path: tests/composite_declarations.rs
  line: 256
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: faults_that_remain_still_refuse
  ac_id: FR-376-AC-6
  trace: correct
  path: tests/composite_declarations.rs
  line: 278
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: member_equal_of_different_declarations_is_false
  ac_id: FR-376-AC-7
  trace: correct
  path: tests/composite_declarations.rs
  line: 320
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ordered_set_keeps_supply_order_across_declarations
  ac_id: FR-376-AC-8
  trace: correct
  path: tests/composite_declarations.rs
  line: 364
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_payloads_of_different_declarations_are_unequal_without_a_fault
  ac_id: FR-376-AC-9
  trace: correct
  path: tests/union_declarations.rs
  line: 93
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: skipped_payload_slots_of_other_declarations_never_report_a_foreign_reference
  ac_id: FR-376-AC-9
  trace: correct
  path: tests/union_declarations.rs
  line: 114
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: traversal_does_not_short_circuit_after_a_declaration_mismatch
  ac_id: FR-376-AC-9
  trace: correct
  path: tests/union_declarations.rs
  line: 127
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_payload_pair_inside_another_value_leaves_enclosing_counts
  ac_id: FR-376-AC-9
  trace: correct
  path: tests/union_declarations.rs
  line: 143
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: unlike_union_pairings_stay_checked_invariant_faults
  ac_id: FR-376-AC-9
  trace: correct
  path: tests/union_declarations.rs
  line: 155
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: declaration_key_orders_before_slots
  ac_id: FR-377-AC-1
  trace: correct
  path: tests/composite_declarations.rs
  line: 390
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: first_differing_declaration_byte_decides
  ac_id: FR-377-AC-2
  trace: correct
  path: tests/composite_declarations.rs
  line: 407
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: same_declaration_keeps_slot_order_and_distinct_declarations_never_equal
  ac_id: FR-377-AC-3
  trace: correct
  path: tests/composite_declarations.rs
  line: 435
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: set_and_bag_visit_the_smaller_declaration_key_first
  ac_id: FR-377-AC-4
  trace: correct
  path: tests/composite_declarations.rs
  line: 457
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: equal_keys_exactly_when_values_are_equal
  ac_id: FR-377-AC-5
  trace: correct
  path: tests/composite_declarations.rs
  line: 482
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: pairs_without_a_key_stay_unkeyed_and_refuse
  ac_id: FR-377-AC-6
  trace: correct
  path: tests/composite_declarations.rs
  line: 500
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: declaration_order_holds_inside_wrappers
  ac_id: FR-377-AC-7
  trace: correct
  path: tests/composite_declarations.rs
  line: 540
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_payload_declarations_order_by_declaration_key_bytes
  ac_id: FR-377-AC-8
  trace: correct
  path: tests/union_declarations.rs
  line: 187
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: unlike_unions_have_no_key_and_refuse_equality
  ac_id: FR-377-AC-8
  trace: correct
  path: tests/union_declarations.rs
  line: 215
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_payload_key_order_sorts_set_and_keeps_ordered_set_supply_order
  ac_id: FR-377-AC-8
  trace: correct
  path: tests/union_declarations.rs
  line: 238
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: form_and_form_grouped_keep_both_declarations_unchanged
  ac_id: FR-378-AC-1
  trace: correct
  path: tests/composite_declarations.rs
  line: 561
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: form_over_a_set_charges_one_member_comparison_and_no_element
  ac_id: FR-378-AC-2
  trace: correct
  path: tests/composite_declarations.rs
  line: 586
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: from_admitted_option_stores_a_descendant_payload_unchanged
  ac_id: FR-378-AC-3
  trace: correct
  path: tests/composite_declarations.rs
  line: 616
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: from_admitted_slots_keeps_a_descendant_slot_and_retains_once
  ac_id: FR-378-AC-4
  trace: correct
  path: tests/composite_declarations.rs
  line: 641
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: exact_paths_refuse_a_descendant_and_accept_the_declared_type
  ac_id: FR-378-AC-5
  trace: correct
  path: tests/composite_declarations.rs
  line: 676
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: collections_are_not_covariant_and_stored_collections_behave
  ac_id: FR-378-AC-6
  trace: correct
  path: tests/composite_declarations.rs
  line: 745
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: admitted_union_keeps_a_descendant_payload_and_exact_paths_refuse_it
  ac_id: FR-378-AC-8
  trace: correct
  path: tests/union_declarations.rs
  line: 285
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_construction_keeps_identity_positions_and_occurrences
  ac_id: FR-379-AC-1
  trace: correct
  path: tests/union.rs
  line: 88
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_position_and_arity_refusals_are_typed
  ac_id: FR-379-AC-2
  trace: correct
  path: tests/union.rs
  line: 108
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: evaluated_union_preserves_deferred_fault_and_first_stop
  ac_id: FR-379-AC-3
  trace: correct
  path: tests/union.rs
  line: 403
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: evaluated_union_orders_arguments_and_charges_one_complete_result
  ac_id: FR-379-AC-3
  trace: correct
  path: tests/union.rs
  line: 440
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_equality_compares_members_and_payload_positions
  ac_id: FR-379-AC-4
  trace: correct
  path: tests/union.rs
  line: 145
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: foreign_union_enum_and_composite_kinds_remain_distinct
  ac_id: FR-379-AC-5
  trace: correct
  path: tests/union.rs
  line: 163
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: unlike_unions_have_no_key_and_refuse_equality
  ac_id: FR-379-AC-5
  trace: correct
  path: tests/union_declarations.rs
  line: 215
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_keys_order_identifier_ascii_then_payload_positions
  ac_id: FR-379-AC-6
  trace: correct
  path: tests/union.rs
  line: 199
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: member_declaration_order_and_digest_order_do_not_reach_union_key_order
  ac_id: FR-379-AC-6
  trace: correct
  path: tests/union.rs
  line: 303
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_collections_sort_deduplicate_and_preserve_first_occurrence
  ac_id: FR-379-AC-7
  trace: correct
  path: tests/union.rs
  line: 251
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ieee_union_payload_has_no_key_and_sequence_keeps_occurrences
  ac_id: FR-379-AC-7
  trace: correct
  path: tests/union.rs
  line: 358
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: ten_thousand_union_levels_clone_compare_format_and_drop_on_small_stack
  ac_id: FR-379-AC-8
  trace: correct
  path: tests/union.rs
  line: 527
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: shared_union_children_count_each_occurrence_and_clone_shares_root
  ac_id: FR-379-AC-8
  trace: correct
  path: tests/union.rs
  line: 558
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: union_storage_grows_linearly_with_depth
  ac_id: FR-379-AC-8
  trace: correct
  path: tests/union.rs
  line: 576
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: mixed_union_composite_collection_paths_walk_and_drop_on_small_stack
  ac_id: FR-379-AC-8
  trace: correct
  path: tests/union.rs
  line: 596
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: option_of_union_admits_compares_and_keys_through_the_payload
  ac_id: FR-379-AC-9
  trace: correct
  path: tests/union_declarations.rs
  line: 324
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::inclusive_integer_and_reduced_rational_membership_preserves_inputs
  ac_id: FR-380-AC-1
  trace: correct
  path: src/numeric_membership.rs
  line: 311
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::exact_ceiling_admits_every_position_and_one_over_names_it
  ac_id: FR-380-AC-2
  trace: correct
  path: src/numeric_membership.rs
  line: 361
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::borrowed_inline_and_large_comparisons_stay_within_limb_ceiling_without_allocation
  ac_id: FR-380-AC-3
  trace: correct
  path: src/numeric_membership.rs
  line: 474
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::original_handle_cancels_before_work_and_at_a_late_limb_step
  ac_id: FR-380-AC-4
  trace: correct
  path: src/numeric_membership.rs
  line: 524
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::borrowed_inline_and_large_comparisons_stay_within_limb_ceiling_without_allocation
  ac_id: FR-380-AC-5
  trace: correct
  path: src/numeric_membership.rs
  line: 474
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::arithmetic_endpoint_equality_preserves_borrowed_domains
  ac_id: FR-381-AC-1
  trace: correct
  path: src/numeric_type_equality.rs
  line: 358
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::integer_positions_accept_exact_b_and_report_first_one_over_b
  ac_id: FR-381-AC-2
  trace: correct
  path: src/numeric_type_equality.rs
  line: 420
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::rational_positions_accept_exact_b_and_report_first_one_over_b
  ac_id: FR-381-AC-2
  trace: correct
  path: src/numeric_type_equality.rs
  line: 485
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::borrowed_limb_work_and_storage_are_bounded_at_repr_boundaries
  ac_id: FR-381-AC-3
  trace: correct
  path: src/numeric_type_equality.rs
  line: 594
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::original_cancel_preempts_size_and_late_comparison_then_retry_completes
  ac_id: FR-381-AC-4
  trace: correct
  path: src/numeric_type_equality.rs
  line: 642
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::borrowed_limb_work_and_storage_are_bounded_at_repr_boundaries
  ac_id: FR-381-AC-5
  trace: correct
  path: src/numeric_type_equality.rs
  line: 594
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::original_cancel_preempts_size_and_late_comparison_then_retry_completes
  ac_id: FR-381-AC-5
  trace: correct
  path: src/numeric_type_equality.rs
  line: 642
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::retained_decimal_fields_decide_equality_without_changing_borrowed_inputs
  ac_id: FR-382-AC-1
  trace: correct
  path: src/numeric_type_equality.rs
  line: 691
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::decimal_endpoints_accept_exact_b_and_report_first_one_over_b
  ac_id: FR-382-AC-2
  trace: correct
  path: src/numeric_type_equality.rs
  line: 759
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::decimal_borrowed_limb_work_and_storage_stay_bounded
  ac_id: FR-382-AC-3
  trace: correct
  path: src/numeric_type_equality.rs
  line: 856
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: tests::decimal_original_cancel_preempts_size_and_late_comparison
  ac_id: FR-382-AC-4
  trace: correct
  path: src/numeric_type_equality.rs
  line: 890
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: accessor_reports_only_the_installed_handle
  ac_id: FR-383-AC-1
  trace: correct
  path: tests/ir895_meter_cancel.rs
  line: 26
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: accessor_handle_cancels_the_next_charge_without_changing_accounting
  ac_id: FR-383-AC-2
  trace: correct
  path: tests/ir895_meter_cancel.rs
  line: 46
  basis: Computed declared-owner binding exists; semantic assertion adequacy not assessed in this planless mechanical
    audit.
- test_id: fresh_consumer_resolves_and_runs_unicode17_normalization
  ac_id: FR-141-AC-1
  trace: wrong
  path: tests/fresh_unicode_consumer.rs
  claimed_ac: FR-141-AC-1
  correct_ac: QSpec-FR-141-AC-1
  basis: The asserted kernel normalization behavior belongs to external QSpec FR-141-AC-1; the tag omits the documented
    QSpec namespace.
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Existing source-owned IR-844 proof work, outside this Unicode dependency correction: FR-359-AC-9 and its unchanged Status explicitly remain planned and untagged. No proof or new waiver is supplied. The underlying obligation remains unclosed and the whole-repository verdict remains FAIL. |
| FND-002 | deferred | Existing external IR-705/IR-17 consumer qualification, outside this owner dependency correction: unchanged FR-359 Status calls the original five-test receipt UNRUN and permanently consumer-owned. No local replacement, changed harness or normalization-test proof credit is accepted. The underlying obligation remains unclosed; global FAIL remains. |
| FND-003 | deferred | Existing IR-705/IR-17 consumer representation qualification, outside this Unicode dependency correction: FR-371-AC-6 still requires genuine healthy and substantive mutant outcomes under its original bounds. Those checks are UNRUN by this reviewer and absent from this fix delivery. The criterion remains unclosed; global FAIL remains. |
| FND-004 | deferred | Preexisting FR-383 cancellation-accessor assurance bookkeeping is outside this Unicode correction: its unchanged Status documents make build-no-std evidence with no source Trace binder. Author bare-metal clippy compilation supports implementation availability, not a newly added binder or strict pass. No new exemption is granted; the structural gap and global FAIL remain. |

## Disposition round 1

FAIL remains for whole-repository assurance; four inherited obligations remain unclosed. These dispositions classify them outside the narrow IR-935 correction using their existing ownership, without granting a waiver or claiming implementation/proof delivery.

Reviewed actual ec37d86/d878209 code and source-owned classifications at d878209d5c11b4085963dee17cbb0619dc961110. Source, dependency, public Unicode identity/assertion, fixture normalization vectors and feature boundaries remain unchanged except the reviewed harness fix. Fresh consumers still have independently created source/manifest inputs and no initial Cargo.lock; the shared build cache contains build products only and cannot supply a consumer lockfile. The separate cache has its own Cargo lock, so it does not recurse into the outer owner test target. Parent test process environment is preserved apart from the child target override and explicit serial jobs. RAII cleanup is best-effort with path/error diagnostics on filesystem deletion failure; successful-run and caught-unwind removal are asserted. The current scratch parent is empty. Earlier red/green evidence directories are retained separately and never used as inputs. No new findings were identified in this fix review.

Reviewer actually ran Quire trace and strict matrix only. Trace resolves the canonical external attribute; strict exits 1 for exactly the same four inherited gaps, with zero ignored-only. Author fix1-consumer-default.log and fix1-consumer-test-support.log each report one passing real Cargo consumer test including lifecycle controls. Author fix1-fmt-lint-final.log shows fmt-check and all three clippy lanes passing; existing stable-rustfmt configuration warnings remain visible. Reviewer Cargo/rustc/fmt/Kani checks are UNRUN under the no-heavy grant. Aggregate make ci, downstream CG native qualification and Kani/mutation obligations await their owners; none is credited from the owner normalization test.

Plan completion: not assessed

Runtime model: gpt-6.1-sol; run: ir935-disposition1-e929a750-9cf8-4d45-81ae-71afdbb0a68c. No source edits, commits or PR review comments by this reviewer.

```yaml
round: 1
head: d878209d5c11b4085963dee17cbb0619dc961110
base: aa63308958b7fa3a08142a24ce3694e94c28d62b
current_verdict: FAIL remains for whole-repository assurance; four inherited obligations remain unclosed. These
  dispositions classify them outside the narrow IR-935 correction using their existing ownership, without granting
  a waiver or claiming implementation/proof delivery.
scope:
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: '37'
  role: examined
  excerpt: 'PLANNED, deferred to IR-844 (u64 Meter redesign). The planned `cfg(kani)` harness, tagged `Trace: FR-359-AC-9`
    only once verified, uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand charge
    with at most two semantic-size requests, arithmetic charge with one, result-retain charge with none) over the
    full `i64` operand range, plus separate repeated-kind charge controls with at most two requests each. Symbolic
    `i64` operands and symbolic limits including `ResultUnits` cover first-short and simultaneous shortages and
    absence of the third admission. Under `'
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: '38'
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their own assertions
    under their original domains, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600
    seconds. Record their individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test or altered
    consumer harness substitutes for it.
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '84'
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain
    bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample
    checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation,
    accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing
    mutation execution or missing teardown is inconclusive and does not satisfy this criterion.
- id: FR-383-AC-4
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  lines: '50'
  role: examined
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std` + `alloc`
    build.
- id: FR-359-inherited-status
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: 63-63
  role: context_only
  excerpt: 'AC-8, AC-11 and AC-12 are delivered by IR-705. AC-8 is bound by the `Trace: FR-359-AC-8` unit tests
    in `src/accounting.rs` (including `first_inserted_failing_size_precedes_later_same_kind_maximum`, `size_shortages_use_field_order_across_attachment_orders`
    and `request_counts_from_zero_to_many_gain_no_denial`). AC-11 is verified by inspection: the only semantic-size
    loop in `Meter::charge` is over the charge''s own requests. AC-12 is bound by `tc_931_admitted_log_matches_model_ordered_prefix`
    and `tc_931_admitted_log_matches_model_same_kind`. No Kani result or mutation control is claim'
- id: FR-371-inherited-status
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: 92-92
  role: context_only
  excerpt: PROPOSED / UNIMPLEMENTED / UNRUN. All six criteria are prospective. No new executable Trace bindings,
    proof receipt or memory-improvement claim accompanies this Markdown proposal. The selected mechanism and its
    trade-offs are recorded in [AD-003](../assurance/AD-003-bounded-integer-representation.md); implementation and
    independent review remain outstanding. The original healthy exact-kernel qualification had an unavailable measured
    peak; an older distinct control's peak and a separate compiler-only panic cannot establish the cause or success
    of this proposal. No 16GiB Kani improveme
- id: FR-383-inherited-status
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  lines: 54-59
  role: context_only
  excerpt: 'Implemented for IR-895. [TC-933](../test-cases/TC-933-meter-cancellation-handle-access.md)

    has direct public-API Trace bindings for AC-1 and AC-2. AC-3 is verified by

    inspection of the borrowing accessor; AC-4 is verified by `make build-no-std`.

    The computed `quire matrix` reports AC-1 and AC-2 as tagged, AC-3 as

    method-without-symbol, and AC-4 as untagged because its build gate has no

    source Trace binding.'
- id: ScratchDirectory::new
  path: tests/fresh_unicode_consumer.rs
  lines: 15-30
  role: examined
  excerpt: "    fn new(parent: &Path) -> io::Result<Self> {\n        fs::create_dir_all(parent)?;\n        // Atomic\
    \ directory creation claims ownership; PID reuse and concurrent\n        // invocations skip existing directories\
    \ rather than reading their inputs.\n        for attempt in 0..128 {\n            let path = parent.join(format!(\"\
    consumer-{}-{attempt}\", std::process::id()));\n            match fs::create_dir(&path) {\n                Ok(())\
    \ => return Ok(Self(path)),\n                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,\n\
    \                Err(error) => return Err("
- id: ScratchDirectory::drop
  path: tests/fresh_unicode_consumer.rs
  lines: 34-45
  role: examined
  excerpt: "impl Drop for ScratchDirectory {\n    fn drop(&mut self) {\n        // Also runs if a Cargo invocation\
    \ or assertion panics. Keep diagnostics\n        // without risking a second panic while an assertion is unwinding.\n\
    \        if let Err(error) = fs::remove_dir_all(&self.0) {\n            eprintln!(\n                \"could\
    \ not remove consumer scratch {}: {error}\",\n                self.0.display()\n            );\n        }\n\
    \    }\n}"
- id: canonical-external-test-trace
  path: tests/fresh_unicode_consumer.rs
  lines: 47-50
  role: examined
  excerpt: '/// Provenance: IR-935, QSpec-TC-186

    #[trace("QSpec-FR-141-AC-1")]

    #[test]

    fn fresh_consumer_resolves_and_runs_unicode17_normalization() {'
bindings: []
dispositions:
- fnd: FND-001
  outcome: deferred
  reason: 'Existing source-owned IR-844 proof work, outside this Unicode dependency correction: FR-359-AC-9 and
    its unchanged Status explicitly remain planned and untagged. No proof or new waiver is supplied. The underlying
    obligation remains unclosed and the whole-repository verdict remains FAIL.'
- fnd: FND-002
  outcome: deferred
  reason: 'Existing external IR-705/IR-17 consumer qualification, outside this owner dependency correction: unchanged
    FR-359 Status calls the original five-test receipt UNRUN and permanently consumer-owned. No local replacement,
    changed harness or normalization-test proof credit is accepted. The underlying obligation remains unclosed;
    global FAIL remains.'
- fnd: FND-003
  outcome: deferred
  reason: 'Existing IR-705/IR-17 consumer representation qualification, outside this Unicode dependency correction:
    FR-371-AC-6 still requires genuine healthy and substantive mutant outcomes under its original bounds. Those
    checks are UNRUN by this reviewer and absent from this fix delivery. The criterion remains unclosed; global
    FAIL remains.'
- fnd: FND-004
  outcome: deferred
  reason: 'Preexisting FR-383 cancellation-accessor assurance bookkeeping is outside this Unicode correction: its
    unchanged Status documents make build-no-std evidence with no source Trace binder. Author bare-metal clippy
    compilation supports implementation availability, not a newly added binder or strict pass. No new exemption
    is granted; the structural gap and global FAIL remain.'
new_findings: []
```
