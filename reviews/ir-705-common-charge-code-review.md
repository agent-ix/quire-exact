---
id: SR-15257
title: IR-705 PR76 private common charge code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; PR76 frozene1411c6; four changed Rust/Python files and
  unchanged Meter proof seam
review_set: subset
---

## Summary

Ticket: IR-705; product PR quire-exact#76. Sole independent nonauthor reviewer. Exact clean frozen heade1411c620d91137737c53a1b71ba655bd641c32d; published base949ba2cca08d820b1c9fa566997ce02ea6b3354a. Four changed files: src/accounting.rs, tests/ir705_common_charge.rs, scripts/meter_kani_gate.py and scripts/test_meter_kani_gate.py. All422 frozen tracked-source hashes independently match. Cargo.toml/Cargo.lock and complete original src/accounting/kani_proofs.rs (harness/model/domain/limits/schedule) byte-compare to949. No author/source, protected qualification case/target, other worktree or frozen review was changed. Repo AGENTS/CLAUDE/.agent Rust conventions were freshly read. No applicable AssuranceProfile found. No spec/plan edits: spec-base and sub-analyses are inapplicable; code-review includes Rust plus the changed Python gate lane. No broad optional semantic fanout. Only read-only files/Git/tracker and Quire/Quoin commands ran; NO Cargo/fmt/metadata/native/Python tests/backend/Kani/proof/aggregate. IDs were checked against reviewed base/main artifacts, alllocal private review artifacts (maxSR15256) and actual open PR inventory (only76), allocating collision-free SR15257/15258.

## Verdict

**CONDITIONAL — two medium assurance defects require same-PR correction.** No production storage/admission behavior defect identified by source review. Native/Persistent/aggregate limitations remain explicit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The ALL-loop classifier allows unrelated UNDETERMINED checks alongside the one intended unwind failure and can report an accepted control despite unresolved properties. | scripts/meter_kani_gate.py:142-159 |
| FND-002 | medium | The new builder-history test claims injection-precedence AC13 but installs no injected denial and never calls charge_plan; it adds a false criterion binding. | src/accounting.rs:1429-1474 |

## Coverage

Private CommonCharge is Copy with two Option<(LimitKind,u64)> slots and native work/results; it owns no Integer/Vec/drop glue. Public Charge retains the unchanged API over Common/General storage; no bounded public API, new Integer/as_big contract, domain/representation/real arithmetic or Meter state layout change. Fitting exact builders normalize to common units. A third size or any negative/above-u64 amount promotes losslessly, preserving both previous requests, order, duplicate/zero requests, cumulative amounts and arbitrary later spill. Once general, later fitting overwrite preserves general storage but restores the same mathematical contents. Manual Eq/Debug normalize common content to the same InlineSizes/work/results view, so storage history is unobservable in those traits; Clone preserves contents. Promotion/debug/equality use at most two inline General slots, no general-container heap allocation merely for holding the variant. Existing public Clone/Eq/Debug availability and textual field layout remain intact.

Storage-first dispatch passes the common iterator/u64 values or general borrowed ChargeAmount values into ONE private generic charge_fields algorithm. MeterAmount is a private Copy trait, not a public seam; references stay borrowed for the whole call, with no aliasing mutation of the charge. Shared cancellation/injection runs first, then one request-ordered validation loop selecting earliest ScalarLimits field and first inserted failure of equal kind; cumulative work/results, allten atomic writes, admission/log/truncation are still shared. Native common requests cannot allocate/drop Integer data during successful admission; general exact requests retain genuine unbounded refusal. The private Clone iterator bound serves the actual ALL-loop mutation only; baseline does not clone requests. No second Meter algorithm, copied renderer/kernel, unsafe, async/lock or hidden cfg bypass was found. Public literal-oracle tests and existing independent model cover refusals/prefixes beyond common/general self-comparison.

Gate adaptation targets the one production requests loop in charge_fields; its ALL traversal is introduced only into the separate control worktree. It preserves byte-exact original proof/model and checks exact failed property suffix.unwind.1, loop1 description, sourcefile/line and same fully printed charge_fields specialization in property/location. No different/lookalike function, wrong line/file, extra FAILURE, compile/resource exit or absent expected summary is accepted. Actual backend property naming/baseline/ALL/refusal-order outcomes are still PENDING; gate-adaptation.md discloses this and native parser/source-anchor checks do not substitute. However its accepted-status list retains the unresolved-companion loophole in FND001. The new history test's AC13 tag is independently wrong (FND002); correct storage tests do not establish injection precedence. Both require same-PR correction before acceptance.

Supplied focused evidence inspected, not rerun: initial compile failed because the private opaque iterator did not expose Clone; archived red source/log retained. Correct private bound, retry1 fmt/fmt-check/lint(default/test-support/thumbv7em-none-eabi), accounting26/28,cancel7/7,numeric4/4, five integration binaries35/41, three VerdictTests passed. Final mechanism assertion strengthened exact third-request promotion; confined fmt/lint/accounting26/28 passed again. Actual16GiB/swap0/jobs1/testthreads1/QSPECfd7, peaks685441024 and395071488bytes, memoryevents0. Heavy-release records allthree scopes inactive/hierarchiesgone/TENguardsempty and targetretained. No reviewer heavy work, backend or aggregate; those remaining gates cannot be inferred from native receipts.

Plan completion: not assessed

## Review data

```yaml
clean: false
scope:
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters
    the first unavailable counter of a charge, including several simultaneously short
    counters and semantic sizes attached in eithe
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum
    observable through `consumed()`; the diagnostic log record is verified under FR-359-AC-12.
    With `IntegerBits` limit 5 and reque
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference`
    uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand
    charge with at most two semantic-size '
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and
    called helpers finds no traversal with a trip count greater than that charge's
    semantic-size request count, including no traver
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With an occurrence-1 injected denial selected, make the same `charge` independently
    fail an ordinary semantic-size counter and make a `charge_plan` with `pairs =
    2` independently fail its work reserva
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request
    of 9 followed by 3 and then a charge with no size request. The records expose
    respectively 9, 3 and no IntegerBits request whil
- id: Charge/Common/General
  path: src/accounting.rs
  role: examined
  excerpt: "enum ChargeStorage {\n    Common(CommonCharge),\n    General(GeneralCharge),\n\
    }"
- id: gate-classifier
  path: scripts/meter_kani_gate.py
  role: examined
  excerpt: 'if returncode != 1 or statuses - {"SUCCESS", "UNREACHABLE", "UNDETERMINED",
    "FAILURE"}:'
- id: gate-parser-controls
  path: scripts/test_meter_kani_gate.py
  role: examined
  excerpt: 'def test_only_actual_production_all_loop_unwind_satisfies_control(self):'
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: test-intent
  artifact_id: FR-359-AC-9
  related_ids: []
  path: scripts/meter_kani_gate.py
  lines: 142-159
  excerpt: A `LimitKind::ALL`-traversal mutant reports an unwinding-assertion failure
    at its mutated production loop; a refusal-order mutant yields a model-equivalence
    counterexample. For the baseline, any `UnwindBoundExhausted`, `MemoryExhausted`,
    timeout, compile error or missing assertion fails acceptance. For either mutation
    control, an unwinding assertion is expected only for the ALL-traversal mutant;
    resource exhaustion, timeout or compile error is inconclusive and does not satisfy
    the control. Test-only code changes no production path.
  finding: The ALL-loop classifier allows unrelated UNDETERMINED checks alongside
    the one intended unwind failure and can report an accepted control despite unresolved
    properties.
  scenario: A sequential two-check transcript has the correctly located charge_fields
    unwind.1 FAILURE plus an unrelated safety property UNDETERMINED, exit1 and the
    expected failed summary/time. The allow-list at142 permits both; len(failures)==1
    and the exact failure-location checks pass. classify returns ExpectedProductionUnwindFailure.
    Remove the unresolved status from accepted companion checks and add this mixed-status
    negative control. This permissive allow-list predates PR76, but the current changed
    gate and its explicit inconclusive-rejection claim still retain it.
- fnd: FND-002
  severity: medium
  confidence: high
  method: code-review
  check: trace
  artifact_id: FR-358-AC-13
  related_ids:
  - FR-359-AC-8
  path: src/accounting.rs
  lines: 1429-1474
  excerpt: 'With an occurrence-1 injected denial selected, make the same `charge`
    independently fail an ordinary semantic-size counter and make a `charge_plan`
    with `pairs = 2` independently fail its work reservation under a configured work
    limit of 3 after two admitted work units, with all other limits sufficient. In
    each case, the injected `WorkUnits` record is returned before the ordinary refusal:
    `limit = consumed` equals the pre-charge work, `next_charge` is the charge''s
    work amount or the plan''s reservation respectively, and every consumed counter
    and admission count remain unchanged. The criterion '
  finding: The new builder-history test claims injection-precedence AC13 but installs
    no injected denial and never calls charge_plan; it adds a false criterion binding.
  test_id: src/accounting.rs::tests::common_and_promoted_builder_histories_have_identical_observations
  claimed_ac: FR-358-AC-13
  correct_ac: FR-359-AC-8
  scenario: The entire test only compares common/promoted/fitting Eq/Clone/Debug and
    ordinary successful or IntegerBits-refused charge calls. A regression that checks
    ordinary size/work refusal before injection in charge and charge_plan cannot fail
    this test. Remove AC13 from this test or exercise its actual two injected-precedence
    obligations; retain the independently meaningful storage/refusal assertions.
bindings:
- test_id: src/accounting.rs::tests::charge_size_requests_keep_two_inline_and_overflow_in_order
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::common_and_promoted_builder_histories_have_identical_observations
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::common_and_promoted_builder_histories_have_identical_observations
  ac_id: FR-358-AC-13
  trace: wrong
- test_id: tests/ir705_common_charge.rs::public_charge_transitions_preserve_literal_denials_and_admitted_prefixes
  ac_id: FR-359-AC-6
  trace: correct
- test_id: tests/ir705_common_charge.rs::public_charge_transitions_preserve_literal_denials_and_admitted_prefixes
  ac_id: FR-359-AC-8
  trace: correct
- test_id: tests/ir705_common_charge.rs::public_charge_transitions_preserve_literal_denials_and_admitted_prefixes
  ac_id: FR-358-AC-14
  trace: correct

```

## Dispositions

Separate same-reviewer disposition round1 at clean frozen fix546eadb4a5cd540c8bfc1f3efd9c9e2e47cdc413. Original Findings and review bytes remain unchanged as a prefix. The fix removes only UNDETERMINED from the mutant companion-status allow-list, adds one resolved-vs-unresolved adverse parser control, and removes only the false FR358AC13 tag. Exact expected failure/function/file/line/loop checks remain unchanged. No new finding.

Independently checked preserved initial-review hashes, exact source diff, archived old-control classifier byte-equal toe1411c6 and its new test byte-equal to fixed test source. Supplied negative execution genuinely fails the old classifier (GateError not raised), while fixed four VerdictTests pass. Focused fmt/fmt-check/lint(three lanes), accounting26/28 pass; actual16GiB/swap0/QSPECfd7, peak397017088bytes, allmemoryevents0, hierarchy gone/TENguards empty, targetretained. These are inspected receipts, not reviewer execution. Supplied matrix compared narrowly: exactly wrongAC13 binding removed (281→280), no added binding or189criterion/status change; no full matrix repeated. No reviewer heavy/native/backend/proof/aggregate work.

Actual current root decisione437a3cc-7298-4e8e-b12e-75971dd8c2c2 and existing owner handoffs were read back. Code/Rust latest disposition PASS, with both findings FIXED. Full gap remains FAIL for four inherited local gaps, explicitly DEFERRED; introduced trace FND005 FIXED. Persistent Meter baseline, actual ALL/refusal-order controls and backend property mapping remain PENDING and required at reviewed head, plus final aggregate and exact artifact custody before merge. No IR917 fit or acceptance inferred.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At546eadb4, inspected fixed source:         if returncode != 1 or statuses - {"SUCCESS", "UNREACHABLE", "FAILURE"}:             raise GateError("ALL-loop control was inconclusive") |
| FND-002 | fixed | At546eadb4, inspected fixed source:     #[trace("FR-359-AC-8")]     #[test]     fn common_and_promoted_builder_histories_have_identical_observations() { |
