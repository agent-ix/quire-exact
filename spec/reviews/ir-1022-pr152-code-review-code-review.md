---
id: SR-17940486709564847047710
title: "IR-1022 PR #152 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
date: 2026-11-12
scope: "agent-ix/quire-exact PR #152 at e88fc800 (base main e20118a); IR-1022; src/accounting/units.rs, src/native_integer.rs, src/equality.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-397
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-944
    type: reviews
---

## Summary

Ticket: IR-1022. Code review with the rust-review lane folded in, scoped to `git diff origin/main...HEAD` (5 files, +196/-9; the two spec files are reviewed in the spec-review artifact). `tests/ir1422_native_entries.rs` is not in the diff, so the 9 differential tests are unchanged and unweakened.

Behavior-preservation check, `Meter::charge_plan_one_pair` against `Meter::charge_plan(&Integer::one())`, line by line:

- Capture request: identical `ChargeEvent` (point `EqualityPlan`, work 1, results 0, work reservation 3 = `pairs + 2`, result reservation 1, size `value_occurrences` 1, `can_record(1)`), built before the inner call, recorded with `record_capture(request, result.is_ok())` in both.
- Check order: injected denial / cancel (`injected_denial`, shared) first, `Incomplete { WorkUnits, limit = consumed = w, next_charge 3, EqualityPlan }` in both (`check_injected` with `ChargeAmount::exact(3)` gives the same record); then the `value_occurrences` size (1 > limit gives `incomplete(ValueOccurrences, 1)`); then work reservation 3 against remaining work; then one remaining result unit. Same order, same `Incomplete` fields (limit, consumed, next charge, charge point).
- Remaining: the Integer path computes `limit - consumed` as a signed `Integer`; the u64 path uses `saturating_sub`. They can differ only when consumed > limit, and there both deny (`3 > negative` vs `3 > 0`; `negative < 1` vs `0 < 1`), with the same `incomplete(...)` record, since `incomplete` reads the u64 counters, not the remaining value. No overflow divergence: the only additions are `consumed(WorkUnits) + 1` (identical in both, and safe because 3 <= limit - consumed was just checked) and `saturating_add(1)` (identical).
- Commit: same `capture_charge`, same high-water `max(1)`, same work `+1`, same `admit` (admission count, denied-point admission count, test-support `AdmittedCharge { value_occurrences: Some(1) }`).
- The other three requests are `charge_units` calls; `Meter::charge` routes a `Common` `Charge` through `UnitsCharge::from_charge` into the same `charge_units`, so equal `UnitsCharge` fields mean identical behavior.
- No `unwrap`, panic, `unsafe` or narrowing cast in production code; the test module uses `expect`/`panic!` only in assertions. `Integer` values still built on the capture and `Incomplete` paths are `Integer::from(u64)` small inline values (`IntegerRepr::Small64`), no bignum arithmetic.

Reviewer runs, own detached worktree at e88fc80 with its own `CARGO_TARGET_DIR`, under `flock /tmp/cargo-slot.lock`:

- `cargo fmt --check`: clean. `cargo clippy --all-targets -- -D warnings`, `... --features test-support`, `cargo clippy --lib --target thumbv7em-none-eabi -- -D warnings`: all clean.
- `cargo test --test ir1422_native_entries`: 9 passed (also 9 with `--features test-support`). `cargo test --lib accounting`: 42 passed (45 with test-support), including both new FR-397-AC-6 tests. `cargo test --lib native_integer`: 1 passed.
- Reviewer mutants (each alone, file restored after, `git status --porcelain` empty at the end):
  - M1, result-unit reservation `< 1` to `< 0` in `charge_plan_one_pair_inner` only: FAILS `one_pair_plan_in_u64_equals_the_integer_plan_charge`, `boolean_equality_matches_planned_equality_over_boolean_values`, `equality_matches_planned_equality_over_integer_values`.
  - M2, `equality.pair` and `equality.result-retain` swapped in `charge_equality_schedule`: FAILS the three ir1422 equality tests (both Boolean tests and the Integer one); the accounting unit tests all pass.
  - M3, work reservation checked as 2 instead of 3 in the u64 plan only: FAILS the grid test plus the two ir1422 equality tests.
  - M4, production plan-form work `.work(2)` to `.work(1)`: FAILS the three ir1422 equality tests; `the_equality_schedule_requests_are_native_common_charges` PASSES (see FND-001).
  - M5, `next_charge` of the `value_occurrences` denial in the u64 plan off by one: FAILS only the grid test (in the kernel, plan-form's identical size check denies first, so this arm is reachable only through the unit test).

## Verdict

Behavior-preserving: the u64 plan charge matches `charge_plan(&1)` in amounts, units, check order, counters, admissions, capture event and every `Incomplete` field, including the consumed > limit saturation edge, and no u64 overflow can occur where the Integer path would not. All gates pass and every mutant is killed by a named test. One medium finding: the AC-6 "Common, not General" test asserts a copy of the requests written in the test, not the requests `charge_equality_schedule` actually makes (M4 survives it). Two low findings. Mergeable on behavior; FND-001 is a small refactor worth doing in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `the_equality_schedule_requests_are_native_common_charges` builds its own `UnitsCharge` literals (`UnitsCharge::one(EqualityPlanForm, ValueOccurrences, 1).work(2)`, `UnitsCharge::new(EqualityPair)`, `...results(1)`) and compares them with `Charge` adapters; it never reads the requests `charge_equality_schedule` makes. Mutant M4 (production plan-form `.work(2)` to `.work(1)`) passes this test; only the ir1422 differential tests catch it. So the AC-6 claim "each request of the Boolean equality schedule equals a Common Charge" is asserted of a duplicate, not of the schedule. Factor the three requests into `pub(crate)` constructors (or a const array) that both `charge_equality_schedule` and the test use. | src/accounting/units.rs:255-288, src/native_integer.rs:644-658 |
| FND-002 | low | The `charge_equality_schedule` comment says the charges are made "with no `Integer` arithmetic", but the capture event and every `Incomplete` on this path still construct `Integer` values (`Integer::from(RESERVATION)`, `Integer::from(PAIRS)`, `Integer::one()`). They are small inline values with no bignum arithmetic, so the property holds in spirit; say "no arbitrary-precision arithmetic on the admission path" or similar. | src/native_integer.rs:645-646 |
| FND-003 | low | `PAIRS` and `RESERVATION` are declared twice, in `charge_plan_one_pair` and `charge_plan_one_pair_inner`; a later edit to one copy (the event's reservation vs the checked reservation) would desynchronize them silently. Hoist them to associated or module consts. | src/accounting/units.rs:151-152, :170-171 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| src/accounting/units.rs Meter::charge_plan_one_pair, charge_plan_one_pair_inner | examined | the equality.plan charge of a plan with exactly one pair, in u64 arithmetic: same denial, reservation, size and consumption as Meter::charge_plan with pairs = 1 |
| src/accounting/units.rs UnitsCharge::work | examined | pub(crate) fn work(mut self, amount: u64) -> Self { self.work = amount; self } |
| src/accounting/units.rs tests (AC-6) | examined | the_equality_schedule_requests_are_native_common_charges; one_pair_plan_in_u64_equals_the_integer_plan_charge (grid work 0-5, results 0-2, occurrences 0-2, plus injected denial) |
| src/native_integer.rs charge_equality_schedule | examined | charge_units(plan-form one VO 1 work 2)?; charge_plan_one_pair()?; charge_units(EqualityPair)?; charge_units(EqualityResultRetain results 1) |
| src/equality.rs charge_plan_form doc comment | examined | The native equality entries charge the same amounts in u64 units (charge_equality_schedule), pinned equal by their differential tests. |
| src/accounting.rs Meter::charge_plan, charge_plan_inner, charge, check_injected, injected_denial, incomplete, admit | context_only | reference Integer path the u64 plan mirrors |
| tests/ir1422_native_entries.rs (unchanged) | context_only | 9 differential tests, AC-1 to AC-5 |
