---
id: SR-17940486709564847047704
title: "IR-844 PR #150 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-exact PR #150 at 307bf131 (base main 1f07457); IR-844; src/accounting/units_proofs.rs, tests/meter_kani_gate.rs, Makefile, CLAUDE.md, README.md, docs/meter-kani-gate.md, scripts/meter_kani_gate.py (deleted), scripts/meter_kani_scope.sh (deleted), src/accounting.rs, src/accounting/kani_proofs.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-359
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-931
    type: reviews
---

## Summary

Ticket: IR-844. Code review with the rust-review lane folded in, scoped to `git diff origin/main...HEAD` (12 files, +307/-154). The PR rewrites the units Kani harnesses so they verify, replaces the monolithic adapter harness with the `charge_units` core proof plus an adapter lemma, adds an executable FR-359-AC-9 binding and lifts the repo's Kani DO NOT RUN text.

Coder facts checked against the code:

- `units_proofs.rs` compares `next_charge` through `.native()` at all three sites (`refused next_charge` in `check`, `injected next_charge` and `refused next_charge` in the injection proof). No `.to_u64()` remains in the file.
- Every check is named through the local `named!` macro, which expands to `kani::assert(cond, "literal")`. The old `assert_eq!` calls are gone. The only unnamed failures are the two `panic!` arms on a production/model Ok-vs-Err disagreement and the adapter's `let ... else panic!`; each carries a literal message, which Kani reports verbatim.
- `public_units_adapter_matches_independent_transition` is removed. `units_adapter_maps_common_charge_to_units_charge` builds `Charge::new(point).set_units(work,false).set_units(results,true)` plus zero, one or two `push_units`, and checks `UnitsCharge::from_charge` field by field (point, first, second, work, results) with covers for zero and two requests.
- `tests/meter_kani_gate.rs::units_kani_proofs_verify` (`#[trace("TC-931","FR-359-AC-9")]`, ignored) runs `cargo kani --harness accounting::units_proofs::<name> -Z function-contracts --exact --unwind 3 --jobs 1 --solver cadical --output-format regular` for each of the three harnesses. It requires exit success, `VERIFICATION:- SUCCESSFUL` on stdout, and a ` ** N of M cover properties satisfied` line with N == M. `make kani-meter-gate` runs `cargo test --locked --test meter_kani_gate -- --ignored`.

Reviewer runs, in a private detached worktree with its own target, under `flock /tmp/cargo-slot.lock`:

- `cargo fmt --check`: clean. `cargo test --locked --lib accounting`: 40 passed, 0 failed; with `--features test-support`: 43 passed, 0 failed.
- Kani 0.68.0 / CBMC 6.11.0, `cargo kani --harness accounting::units_proofs::<name> -Z function-contracts --exact --unwind 3 --jobs 1 --solver cadical --output-format regular`, each inside `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0` with `timeout 1800`, peak sampled every 0.5 s from the scope's `memory.peak` (includes the incremental compile):
  - healthy `units_charge_matches_independent_transition`: VERIFICATION:- SUCCESSFUL, 4 of 4 covers, 18.77 s verification, 22.2 s wall, 1.83 GB peak.
  - healthy `units_adapter_maps_common_charge_to_units_charge`: VERIFICATION:- SUCCESSFUL, 2 of 2 covers, 0.25 s verification, 1.6 s wall, 0.30 GB peak.
  - reviewer mutant R1 (distinct from the coder's two): in `charge_units_inner` (`src/accounting/units.rs:111`), `kind.index() < first_kind.index()` replaced by `>` (first-failing size chosen in reverse field order). `units_charge_matches_independent_transition` FAILED with `Failed Checks: refused kind` (check.assertion.1, units_proofs.rs:131), 18.92 s verification, 20.8 s wall, 1.90 GB peak.
  - reviewer mutant R2: a `for kind in LimitKind::ALL { core::hint::black_box(kind); }` loop inserted at the start of size validation in `charge_units_inner`. The same harness FAILED with `Failed Checks: unwinding assertion loop 0` at `charge_units_inner.unwind.0` (units.rs:101), 2.94 s verification, 4.2 s wall, 0.40 GB peak.
  - `units.rs` was restored after each mutant; `git status --porcelain` was empty after the batch.

## Verdict

Mergeable, with low findings only. The harness change is correct and minimal: `.native()` is strictly stronger than the old `to_u64()` comparison, the assertion set of the core proof is unchanged apart from naming, and named `kani::assert` checks make mutant failures identify their check (both of the reviewer's mutants and both of the coder's did). The gate test cannot pass vacuously: exact harness names with `--exact`, exit status AND the `VERIFICATION:- SUCCESSFUL` line AND an N-of-N cover line are all required, and failures print stdout and stderr.

Repo-instruction change (reported separately for the lead and Peter): it is minimal (one CLAUDE.md line, one README sentence, Makefile help, comment and recipe, the 13-line docs page rewritten, two exit-2 stub scripts deleted, two code comments), and nothing else in those files changed. It is accurate: the monolithic `kani_proofs.rs` schedule harness is still stated to be excluded from compilation and unclaimed by AC-9 in docs/meter-kani-gate.md, the `src/accounting.rs` and `kani_proofs.rs` comments, FR-359 and TC-931; `#[cfg(all(kani, any()))]` is unchanged. No safety information that still applies was lost: the base text held only the DO NOT RUN order, and its cgroup/scope runner and memory-cap advice had already been removed by #87 (ce027b3). What remains is "Run one heavy job at a time"; FND-001 suggests adding a memory cap and timeout.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-359-AC-9 and TC-931 Expected Results say the harnesses verify "within 16 GiB and 600 seconds", but `units_kani_proofs_verify` runs `cargo kani` with no memory cap and no timeout, and the docs and CLAUDE.md give only "Run one heavy job at a time". The recorded receipts used a 24 GiB cap and a 30-minute timeout. Measured peaks (1.2 to 1.9 GB) make the 16 GiB bound plausible, but the gate does not check it. Wrap each run in a 600-second timeout and document the `systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0` invocation in docs/meter-kani-gate.md, or state the bound as a measurement rather than a gate condition. | tests/meter_kani_gate.rs:27-53, Makefile:94, docs/meter-kani-gate.md:9-10 |
| FND-002 | low | docs/meter-kani-gate.md says "A timeout, memory exhaustion or compilation failure is inconclusive, not a verdict", while FR-359-AC-9 says these fail the baseline (and are inconclusive only for a mutant). The test does fail in those cases, but the doc reads as if they were neutral. Say "fails the gate (inconclusive, never acceptance)". | docs/meter-kani-gate.md:9-10 |
| FND-003 | low | The PR body says the IR-1444/IR-1508 decimal run of `native_decimal_from_bindings_refines_owner_membership` left an "empty log". The coder's log (`k/dec-native_decimal_from_bindings_refines_owner_membership.log`, 126 lines) shows CBMC in repeated incremental CaDiCaL solves (about 349,600 variables and 1,756,450 clauses each, `SATISFIABLE`, 0.2 to 6.1 s per solve) when the 30-minute cap hit; every line carries the timestamp 13:03:12 because the timestamping `awk` flushed only at exit. The run was making progress, not silent. Correct the PR description (no code change; the PR changes no decimal code or evidence rows, and it correctly notes the `--unwind 3` vs "no unwind bound" mismatch). | PR #150 body, section "IR-1444 / IR-1508 decimal proofs" |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| src/accounting/units_proofs.rs | examined | named! macro; check() with charge_units vs reference::transition; units_charge_matches_independent_transition; units_adapter_maps_common_charge_to_units_charge; units_injection_cancellation_preserves_occurrence_and_saturation |
| tests/meter_kani_gate.rs | examined | PROOFS, covers_all_satisfied, units_kani_proofs_verify |
| Makefile | examined | kani-meter-gate: cargo test --locked --test meter_kani_gate -- --ignored |
| CLAUDE.md | examined | DO NOT RUN paragraph replaced by one line about make kani-meter-gate |
| README.md | examined | final sentence about the Meter Kani gate |
| docs/meter-kani-gate.md | examined | rewritten gate description and harness rules |
| scripts/meter_kani_gate.py, scripts/meter_kani_scope.sh | examined | deleted DO NOT RUN stubs |
| src/accounting.rs | examined | comment above #[cfg(all(kani, any()))] mod kani_proofs |
| src/accounting/kani_proofs.rs | examined | comment above meter_integer_arithmetic_schedule_matches_reference |
| src/accounting/units.rs | context_only | UnitsCharge::from_charge, Meter::charge_units, charge_units_inner |
| src/accounting.rs Charge builder and Meter::charge | context_only | push_units, set_units, size, work, results, Meter::charge dispatch |
| src/accounting/units_tests.rs | context_only | check(), units_charge_matches_independent_boundary_model, units_charge_matches_independent_mixed_state_model |

## Dispositions

Round 1, fix head bf19b58a (reviewed head 307bf131); `git merge-tree` against origin/main 1f07457 is clean. Verified against `git diff 307bf131..bf19b58a` and the code. Reviewer focused job in a private worktree under the shared cargo lock: `cargo fmt --check` clean; `cargo test --locked --lib accounting` 40/40 and 43/43 with `test-support`; reviewer mutant R2 (a `LimitKind::ALL` loop in `charge_units_inner`) re-run: `units_charge_matches_independent_transition` FAILED at `unwinding assertion loop 0` (`charge_units_inner.unwind.0`, units.rs:101), 3.01 s verification, 4.2 s wall, 0.36 GB peak; source restored, worktree clean. `quire validate` passes on FR-359, TC-931 and the three committed SR files; `quire matrix` reports FR-359-AC-9 `tagged` (three host tests plus the ignored gate test). The three committed `spec/reviews/ir-844-pr150-code-review-*.md` files are byte-identical (`cmp`) to the reviewer's round-0 copies.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | tests/meter_kani_gate.rs now runs each harness through `run_proof` with `const TIMEOUT: Duration = Duration::from_secs(30 * 60);`, `recv_timeout(TIMEOUT)` and, on expiry, `pkill -TERM -P <pid>`, `kill <pid>`, then `panic!("{proof} did not finish within {TIMEOUT:?}")`. Failure is not swallowed. Reviewer kill-tree check: the spawned pid is `cargo-kani` (cargo exec-replaces), with child `kani-driver` and grandchild `cbmc`; issuing the same `pkill -TERM -P; kill` 12 s into CBMC left no cargo-kani, kani-driver or cbmc process after 3 s. AC-9 and TC-931 now state the 30-minute per-harness timeout and that receipts ran under a 24 GiB cgroup with swap disabled; docs/meter-kani-gate.md documents `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0 make kani-meter-gate` and says the test enforces no memory cap. |
| FND-002 | fixed | docs/meter-kani-gate.md: "The test fails a harness that does not finish within 30 minutes. A timeout, memory exhaustion or compilation failure fails the gate; it is never a verdict." This agrees with FR-359-AC-9 ("... timeout, compile error or a missing cover fails the baseline"). |
| FND-003 | fixed | PR #150 body now reads: "`native_decimal_from_bindings_refines_owner_membership` had NO VERDICT: the log is not empty (10 SAT solves, then the final proof query did not return within the 30-minute cap), 0.6 GB peak; not the to_u64/as_big memory cause. Diagnosis is owned by titan:IR:proofs; decimal evidence rows are unchanged." This also supersedes the reviewer's earlier reading of the log as "repeated SATISFIABLE solves" in progress: per titan:IR:proofs, the 11th query (the real assertion proof) never returned. The PR still claims nothing else about decimals. |
