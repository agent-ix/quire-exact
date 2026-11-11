---
id: SR-17940486709564847047707
title: "IR-1022 PR #151 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-exact PR #151 at f3a25430 (base main 1f07457); IR-1022; src/native_integer.rs, src/lib.rs, tests/ir1422_native_entries.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-397
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-944
    type: reviews
---

## Summary

Ticket: IR-1022. Code review with the rust-review lane folded in, scoped to `git diff origin/main...HEAD` (5 files, +78/-16; the two spec files are reviewed in the spec-review artifact).

Coder facts checked against the code:

- `pub fn evaluate_native_boolean_equality(left: bool, right: bool, meter: &mut Meter) -> Outcome<bool>` sits directly after `evaluate_native_integer_equality` in `src/native_integer.rs` and has the same body: `Outcome::from_stop(charge_equality_schedule(meter).map(|()| left == right).map_err(Stop::from))`. It uses the private `charge_equality_schedule` (`equality::charge_plan_form(&one, &one, ..)` then `equality::charge_pairs(&one, ..)`), not `retain_boolean`. No `unwrap`, `expect`, panic, `unsafe` or integer cast. It is re-exported at the crate root in the same `pub use native_integer::{..}` list as the Integer kernel, with no extra `cfg`, so it has the Integer kernel's visibility under `no_std`, thumb and `cfg(kani)`.
- The Integer kernel, `charge_equality_schedule` and `NativeIntegerSchedule` are byte-for-byte unchanged.
- Result retain: `planned_equality` on two `Value::Boolean` runs `charge_plan_form(&left.occ(), &right.occ())` with `occ() == 1` (size `value_occurrences` 1, work 2), `charge_pairs(&1)` (`equality.plan`, work 1, reservation pairs+2 work and 1 result, size 1), one `equality.pair` and `equality.result-retain` with `.results(1)`. The native kernel makes exactly these calls with `Integer::one()`, so the result is retained the same way, as one result unit at `equality.result-retain`, with no separate size charge for the Boolean result in either path. Measured spend for both: `value_occurrences` 1, `work_units` 5, `result_units` 1, `integer_bits` 0.
- The test compares the whole `(Outcome, events, counters)` tuple for native vs reference, so each failing outcome (the `Incomplete` record with its limit, consumed, next charge and charge point) is compared, not only "is refused".

Reviewer runs, own detached worktree at f3a2543 with its own target, under `flock /tmp/cargo-slot.lock`:

- `cargo fmt --check`: clean. `cargo test --locked --test ir1422_native_entries`: 8 passed, 0 failed, including `boolean_equality_matches_planned_equality_over_boolean_values`. `cargo test --locked --lib native_integer`: 1 passed; `--lib equality`: 22 passed.
- `cargo clippy --locked --lib --target thumbv7em-none-eabi -- -D warnings`: clean. `cargo clippy --locked --all-targets -- -D warnings`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --lib` (default features): fails on `Meter::admitted_charges` at src/accounting.rs:1002, identically on origin/main's `src`; pre-existing, not from this PR (`make docs` runs with `--all-features`). The PR's new intra-doc links (`crate::planned_equality`, `evaluate_native_integer_equality`, `crate::retain_boolean`) resolve.
- Limit probe (temporary test file, removed): failing charge point per bound for `(true, false)`: `value_occurrences` 0 -> plan-form; `work_units` 0-1 -> plan-form, 2-4 -> plan; `result_units` 0 -> plan; full spend completes.
- Reviewer mutants, each applied only to the Boolean kernel, file restored after each (`git status --porcelain` empty at the end); every one fails `boolean_equality_matches_planned_equality_over_boolean_values` by name with the other 7 tests passing:
  - R1, pair count off by one for Booleans only (`charge_pairs(&Integer::from(2))`): FAILED at tests/ir1422_native_entries.rs:397 (unlimited comparison).
  - R2, result-retain charged as one work unit instead of one result unit: FAILED at :397.
  - R3, an `Incomplete` from the schedule swallowed and `Completed(left == right)` returned: FAILED at :410 (the sweep), passing the unlimited comparison.
  - R4, plan-form sized as two occurrences per operand: FAILED at :397.

## Verdict

Mergeable on the code. The kernel is correct, minimal and exported consistently with the Integer kernel; it charges exactly what `planned_equality` charges on two Booleans, including the result retain, and the test is a real differential that kills every mutant tried. Rust gates (fmt, both clippy runs including thumb, focused tests) pass. The deviation (differential against `planned_equality` rather than `TypeEnvironment::check_equality(..).evaluate(..)`) is equivalent at the kernel level; see the gap-analysis artifact. One low finding on a test comment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The sweep comment says "Every bound from zero to the full spend of each counted limit lands on every charge boundary of the four-charge schedule", but the probe shows no bound stops at `equality.pair` or `equality.result-retain`: the `equality.plan` charge reserves their work and result units, so only plan-form and plan are ever the denied point. Reword the comment to say the sweep covers every bound of each limit and the two charge points a limit can stop on. | tests/ir1422_native_entries.rs:398-399 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| src/native_integer.rs evaluate_native_boolean_equality | examined | pub fn evaluate_native_boolean_equality(left: bool, right: bool, meter: &mut Meter) -> Outcome<bool> { Outcome::from_stop(charge_equality_schedule(meter).map(\|()\| left == right).map_err(Stop::from)) } |
| src/native_integer.rs evaluate_native_integer_equality, charge_equality_schedule | examined | unchanged; equality::charge_plan_form(&one, &one, meter)?; equality::charge_pairs(&one, meter) |
| src/lib.rs re-export | examined | pub use native_integer::{charge_native_integer_schedule, evaluate_native_boolean_equality, ...} |
| tests/ir1422_native_entries.rs boolean_equality_matches_planned_equality_over_boolean_values | examined | four pairs; reference shape asserts; full-tuple native vs reference at unlimited limits and bounds 0..=spend of integer_bits, value_occurrences, work_units, result_units |
| src/equality.rs planned_equality_stop, charge_plan_form, charge_pairs, plan_pairs | context_only | charge_plan_form(&left.occ(), &right.occ(), meter)?; ... charge_pairs(&plan.pairs, meter)? |
| src/accounting.rs Meter::charge_plan | context_only | EqualityPlan event: work 1, work_reservation pairs+2, result_reservation 1, size value_occurrences pairs |

## Dispositions

Round 1, fix head 28ccedc9 (reviewed head f3a25430); merge-tree with origin/main 5f2745c clean.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Comment at tests/ir1422_native_entries.rs:398-402 now states bounds stop only at plan-form and plan, with pair and result-retain covered by the injected-denial test. |
