---
id: SR-17940486709564847047706
title: "IR-844 PR #150 FR-359 and TC-931 spec review"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-exact PR #150 at 307bf131 (base main 1f07457); IR-844; spec/functional/FR-359-cumulative-meter-boundary.md (Behavior IR-844 paragraphs, AC-9 row, Notes), spec/test-cases/TC-931-single-pass-meter-accounting.md (steps 3-4, Expected Results, Status)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-359
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-931
    type: reviews
---

## Summary

Ticket: IR-844. Spec review of the FR-359 and TC-931 edits that move FR-359-AC-9 from the excluded monolithic harness to the three `accounting::units_proofs` harnesses.

Measured numbers checked against the coder's run logs (`k/final-*.log`, `k/mutant-*.log`, `.done` and `.peak` files in the dispatching session's scratchpad): `units_charge_matches_independent_transition` VERIFIED, 4 of 4 covers, Verification Time 18.802639 s, wall 20.85 s, peak 1,179,090,944 bytes; `units_adapter_maps_common_charge_to_units_charge` VERIFIED, 2 of 2 covers, 0.28825054 s, wall 1.74 s, no peak sample; `units_injection_cancellation_preserves_occurrence_and_saturation` VERIFIED, 4 of 4 covers, 12.467963 s, wall 14.96 s, peak 690,024,448 bytes; mutant max->min FAILED (`Failed Checks: consumed[0..7] matches`), 18.973938 s, wall 21.05 s, peak 1,299,247,104 bytes; mutant `second: common.sizes.first` FAILED at `adapter keeps the second request`, 0.27522984 s, wall 1.68 s. Every figure in FR-359 Notes and TC-931 Status matches, after rounding. The reviewer's own runs reproduce the two healthy verdicts (see the code-review SR).

No "DO NOT RUN" text remains in either file. The decimal proofs (FR-402, FR-403) are not touched by this PR. `quire validate` passes on both files. `quire matrix` reports FR-359-AC-9 as `tagged-by-ignored-test` via `tests/meter_kani_gate.rs:27`.

## Verdict

Mergeable, with low findings. AC-9 is now a precise, falsifiable criterion: it names the three harnesses, the exact command, the cover requirement and two named mutant checks, and the coder's logs and the reviewer's reruns support every number. The monolithic harness is consistently stated to be excluded and unclaimed. The criterion is narrower than before: it covers one `charge_units` transition from an arbitrary symbolic state, not the FR-362 three-charge schedule over `i64` operands through public `Meter::charge`, and it no longer has an ALL-traversal control (gap-analysis FND-001). Because every prior state is symbolic, the single-step proof covers any schedule's per-charge transition, so this is a reasonable re-scope; it follows the IR-844 research comment b101b940 relayed by the planner. Nothing about the decimal proofs is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Stale "disabled" wording survives in the historical paragraphs and contradicts the new Status: FR-359 Notes says "The current head excludes the Meter harness; the healthy proof and both controls must be requalified after the disable is lifted."; TC-931 Status says those results "do not qualify this disabled head" and "Its former command is disabled at this head." Reword to "the monolithic harness stays excluded and unclaimed; AC-9 is carried by the units proofs (see below)". | spec/functional/FR-359-cumulative-meter-boundary.md:69, spec/test-cases/TC-931-single-pass-meter-accounting.md:39, spec/test-cases/TC-931-single-pass-meter-accounting.md:43 |
| FND-002 | low | FR-359 `## Status` records AC-1 to AC-8, AC-11 and AC-12 but not AC-9. The AC-9 delivery and its binding appear only in Behavior and Notes, while TC-931 Status records them. Add one line to FR-359 Status: "AC-9 is delivered by IR-844 (three `units_proofs` harnesses, bound by `units_kani_proofs_verify`)". | spec/functional/FR-359-cumulative-meter-boundary.md:61-65 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-359-AC-9 | examined | The three `cfg(kani)` harnesses ... each report `VERIFICATION:- SUCCESSFUL` with every cover satisfied under `-Z function-contracts --exact --unwind 3 --jobs 1 --solver cadical`, within 16 GiB and 600 seconds ... A mutant that coalesces same-kind requests with `min` ... fails the first harness at a named `consumed[k] matches` check, and a mutant that reads the first request into the second slot of `UnitsCharge::from_charge` fails the second at `adapter keeps the second request`. |
| FR-359 Behavior (IR-844 obligation paragraph) | examined | The IR-844 Kani obligation is the `cfg(kani)` module `accounting::units_proofs`, which checks production `Meter::charge_units` ... The `Integer`-typed `Charge::work` entry and the general charge storage are host-test obligations (`units_tests.rs`). |
| FR-359 Behavior (receipts paragraph) | examined | That harness, `meter_integer_arithmetic_schedule_matches_reference` ... stays excluded from compilation and is not claimed by AC-9. ... AC-9 is bound by the ignored test `units_kani_proofs_verify` |
| FR-359 Notes (IR-844 paragraphs) | examined | AC-9 is delivered by IR-844 as the three `accounting::units_proofs` harnesses ... |
| FR-359 Status | examined | AC-8, AC-11 and AC-12 are delivered by IR-705 ... |
| FR-359-AC-11 | context_only | Inspection of semantic-size processing in production `Meter::charge` and called helpers finds no traversal with a trip count greater than that charge's semantic-size request count |
| TC-931 step 3 | examined | For AC-9, run `make kani-meter-gate` ... |
| TC-931 step 4 | examined | For AC-9 mutation controls, replace `previous.max(amount)` with `previous.min(amount)` ... The `LimitKind::ALL` traversal concern is covered by FR-359-AC-11 inspection. |
| TC-931 Expected Results | examined | The three `units_proofs` harnesses must verify with every cover satisfied within 16 GiB and 600 seconds |
| TC-931 Status | examined | AC-9 is bound by `units_kani_proofs_verify` and delivered as the three `accounting::units_proofs` harnesses ... |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Two stale sentences survive in the FR-359 Notes historical-baseline paragraph (the reviewer missed them in the review pass): "it does not qualify the current disabled head or identify the cause of historical memory exhaustion" and "that binding is removed at this head and no computed Covered status is claimed". Both contradict the new Status and the matrix, which now shows AC-9 `tagged`. Reword them to "it qualifies no current source" and "that earlier binding is historical; AC-9 is now bound by `units_kani_proofs_verify` (see Status)". | spec/functional/FR-359-cumulative-meter-boundary.md:69 |

## Dispositions

Round 1, fix head bf19b58a (reviewed head 307bf131); `git merge-tree` against origin/main 1f07457 is clean. Verified against `git diff 307bf131..bf19b58a` and the code. Reviewer focused job in a private worktree under the shared cargo lock: `cargo fmt --check` clean; `cargo test --locked --lib accounting` 40/40 and 43/43 with `test-support`; reviewer mutant R2 (a `LimitKind::ALL` loop in `charge_units_inner`) re-run: `units_charge_matches_independent_transition` FAILED at `unwinding assertion loop 0` (`charge_units_inner.unwind.0`, units.rs:101), 3.01 s verification, 4.2 s wall, 0.36 GB peak; source restored, worktree clean. `quire validate` passes on FR-359, TC-931 and the three committed SR files; `quire matrix` reports FR-359-AC-9 `tagged` (three host tests plus the ignored gate test). The three committed `spec/reviews/ir-844-pr150-code-review-*.md` files are byte-identical (`cmp`) to the reviewer's round-0 copies.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Every cited sentence is gone. FR-359 Notes now reads "... The monolithic harness stays excluded and unclaimed; AC-9 is carried by the units proofs (see Status)." TC-931 Status reads "they do not qualify the current source" and "... removed; `make kani-meter-gate` now runs the units proofs." Two other stale sentences in FR-359:69 are recorded as new finding FND-003. |
| FND-002 | fixed | FR-359 `## Status` now has "AC-9 is delivered by IR-844 (three `units_proofs` harnesses, bound by `units_kani_proofs_verify`; measurements below)." |
| FND-003 | still-open | New in disposition pass 1 and routed to the coder. FR-359:69 still says "does not qualify the current disabled head" and "that binding is removed at this head and no computed Covered status is claimed". |
