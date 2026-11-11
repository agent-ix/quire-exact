---
id: SR-17940486709564847047705
title: "IR-844 PR #150 FR-359-AC-9 gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-exact PR #150 at 307bf131 (base main 1f07457); IR-844; FR-359-AC-9 and TC-931 steps 3-4 against src/accounting/units_proofs.rs, tests/meter_kani_gate.rs, src/accounting/units_tests.rs, src/accounting/units.rs and Meter::charge / Charge builder in src/accounting.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-359
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-931
    type: reviews
---

## Summary

Ticket: IR-844. Planless gap analysis of FR-359-AC-9 and TC-931 steps 3-4 against the proofs, the gate test and the host tests. Plan completion: not assessed. Matrix (`quire matrix --scope .` at 307bf131): FR-359-AC-9 `tagged-by-ignored-test` by `tests/meter_kani_gate.rs:27` (`units_kani_proofs_verify`); AC-10 untagged (external, by design); AC-11 `method-without-symbol` (Inspection).

Old harness vs new pair. The removed `public_units_adapter_matches_independent_transition` called `check(true)`: it built `Charge::new(point).work(Integer::from(work)).results(results)` plus `.size(kind, amount)` for zero to two requests, called public `Meter::charge`, and asserted against `reference::transition` the ten consumed counters and the admission count on admission, and kind, limit, consumed, `next_charge.to_u64()`, point, unchanged counters and unchanged admissions on refusal, with four covers. That obligation factors into three links:

1. `Meter::charge_units(u)` agrees with the model for every `UnitsCharge`. Proved by `units_charge_matches_independent_transition`: the same symbolic domain, the same assertion set, now named, with `next_charge.native() == Some(i128::from(next))`, which is stricter than `to_u64() == Some(next)` because it also fixes the inline representation.
2. A `Charge` built by `size`/`results`/`work` maps to that `UnitsCharge`. `size` is `push_units` and `results` is `set_units(_, true)` verbatim, so the lemma `units_adapter_maps_common_charge_to_units_charge` proves this link for them over all `u64`. `work(Integer)` is `match amount.to_u64() { Some(u) => set_units(u, false), None => set_exact(..) }`; for `Integer::from(u64)` the first arm is taken, but that is not proved; it is sampled.
3. `Meter::charge(c) == charge_units(from_charge(c))` when the storage is `Common`: a two-arm `match` at `src/accounting.rs:1249-1252`, checked by reading and sampled on the host.

Host coverage of links 2 (work) and 3: `units_tests.rs::check` builds `Charge::new(point).work(Integer::from(work)).results(results)` plus `.size(..)`, unwraps `UnitsCharge::from_charge` (so the charge must stay `Common`), and asserts `adapter.charge(charge) == direct.charge_units(units)`, `general.charge_general(charge) == actual` and equal meter state. It runs over 5 edge values (0, 1, 2, `u64::MAX - 1`, `u64::MAX`) for work and results and 4,096 pseudo-random draws in `units_charge_matches_independent_mixed_state_model`. So the `Integer`-typed path is genuinely covered by host tests, sampled rather than exhaustive. Because the removed harness never produced a verdict (OOM at 16 and 48 GiB, ticket comments 76a187ff and 2ed397cd), no verified FR-359-AC-9 coverage is lost. The spec states the host-test split openly, so this is not a silent loss.

Gate test vacuity check: harness names in `PROOFS` match the three `#[kani::proof]` functions exactly, and `--exact` makes a misspelt name a Kani error (nonzero exit). The test requires `output.status.success()` and `VERIFICATION:- SUCCESSFUL` on stdout. `covers_all_satisfied` requires a ` ** N of M cover properties satisfied` line with N == M; every harness has covers, so a missing line fails the test. Stdout and stderr are captured and printed on failure; nothing is swallowed. A failed spawn panics through `expect`. The reviewer's runs show the healthy logs carry exactly these lines.

## Verdict

Mergeable for AC-9 coverage, with one medium finding the lead may take in this PR or defer. The new pair covers everything the removed monolithic harness asserted: link 1 is proved with the same domain and a stricter `next_charge` check, link 2 is proved for `size`/`results`, and the `Integer`-typed `work` path and the `Meter::charge` dispatch are covered by real host tests (sampled, not exhaustive), which the spec now states openly. The removed harness never produced a verdict, so no verified coverage is lost. The gate test is a real, non-vacuous binding (see Summary). The reviewer's mutants R1 (field-order reversal -> `refused kind`) and R2 (`LimitKind::ALL` loop -> `unwinding assertion loop 0`) were both killed by the core proof.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-9 and TC-931 step 4 drop the old mechanical `LimitKind::ALL`-traversal control and leave it to the AC-11 inspection. The FR-359 statement still requires that the charge path "SHALL NOT traverse `LimitKind::ALL`". The reviewer's mutant R2 shows that the healthy core proof catches such a loop in `charge_units_inner` in 4.2 s (`unwinding assertion loop 0`, `charge_units_inner.unwind.0`, units.rs:101), so the control costs almost nothing to keep. Add it as a third TC-931 step 4 control, with that expected check, and name it in AC-9. It guards only the common path; `charge_general` stays covered by AC-11 inspection. | spec/test-cases/TC-931-single-pass-meter-accounting.md:20, spec/functional/FR-359-cumulative-meter-boundary.md:39 |
| FND-002 | low | AC-9 (Behavior paragraph) and TC-931 step 3 hand the `Integer`-typed `Charge::work` entry to "host-test obligations (`units_tests.rs`)", but the tests that discharge it (`units_charge_matches_independent_boundary_model`, `units_charge_matches_independent_mixed_state_model`) are tagged FR-359-AC-1/2/5/6/8 only, so the matrix cannot show that part of AC-9. The text also does not name the `Meter::charge` -> `charge_units(from_charge(..))` dispatch (accounting.rs:1249-1252), which the same host `check` covers (`adapter.charge(charge) == direct.charge_units(units)`). Add FR-359-AC-9 to those two tests' `#[trace]` and name the dispatch beside `Charge::work`. | src/accounting/units_tests.rs:102, src/accounting/units_tests.rs:139, spec/functional/FR-359-cumulative-meter-boundary.md:19 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-359-AC-9 | examined | The three `cfg(kani)` harnesses `units_charge_matches_independent_transition`, `units_adapter_maps_common_charge_to_units_charge` and `units_injection_cancellation_preserves_occurrence_and_saturation` in `accounting::units_proofs` each report `VERIFICATION:- SUCCESSFUL` with every cover satisfied ... |
| TC-931 step 3 | examined | For AC-9, run `make kani-meter-gate`, which runs the ignored test `units_kani_proofs_verify` ... |
| TC-931 step 4 | examined | For AC-9 mutation controls, replace `previous.max(amount)` with `previous.min(amount)` ... The `LimitKind::ALL` traversal concern is covered by FR-359-AC-11 inspection. |
| tests/meter_kani_gate.rs::units_kani_proofs_verify | examined | #[trace("TC-931", "FR-359-AC-9")] ignored test running cargo kani per harness |
| src/accounting/units_proofs.rs | examined | three harnesses and check() |
| src/accounting/units_tests.rs::check | examined | Charge::new(point).work(Integer::from(work)).results(results) ... assert_eq!(adapter.charge(charge.clone()), actual) |
| FR-359-AC-11 | context_only | Inspection of semantic-size processing in production `Meter::charge` and called helpers ... |
| FR-359-AC-12 | context_only | admitted-charge log under test-support |
