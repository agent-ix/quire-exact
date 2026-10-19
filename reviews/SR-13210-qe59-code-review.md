---
id: SR-13210
title: "quire-exact PR 59 code review: Meter::cancel accessor (FR-383)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@396a7551c5719005dfeb72844464945fe52400e3 vs origin/main 248ea79; src/accounting.rs, tests/ir895_meter_cancel.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-383
    type: reviews
---

## Summary

Ticket: IR-895. The diff adds one public item, `pub fn cancel(&self) -> Option<&Cancel> { self.cancel.as_ref() }` on `Meter` (src/accounting.rs:707-711). It also adds `tests/ir895_meter_cancel.rs` with two tests, tagged `#[trace("TC-933", "FR-383-AC-1")]` and `#[trace("TC-933", "FR-383-AC-2")]`. The rust-review lane ran in this same pass. No other public item changes. `cargo public-api` is not installed, so the surface was checked by reading the diff: exactly one `pub fn` is added and nothing is removed or re-signed.

The signature matches FR-383 Behavior exactly: `&self`, a borrow tied to the meter's lifetime, `None` when no handle is installed, and no clone, allocation, panic or `unsafe` (the crate has `forbid(unsafe_code)`). The neighbouring read accessors (`limits`, `consumed`, `admission_count`) carry no `#[must_use]` and no `#[inline]`. Only the builder `with_cancel` has `#[must_use]`. Clippy here is `all = "deny"` without pedantic, so the bare accessor follows local convention. The doc comment matches the terse style of its neighbours.

Gates were run at the PR head in a throwaway worktree, toolchain 1.98.1 (equal to the MSRV), own CARGO_TARGET_DIR, `--jobs 2`. All pass: `make fmt-check`; `make lint`, which runs all three clippy runs with `-D warnings` (default, `--features test-support`, `--lib --target thumbv7em-none-eabi`); `make build-no-std`; `make test` (default 177 lib tests, then test-support 181, with all integration suites, including the 2 new tests); and `make docs` (`-D warnings`).

Mutation checks, one mutant at a time in the throwaway worktree. Each mutant was run against `cargo test --test ir895_meter_cancel`, and the source was restored to the PR head afterwards:

- M1, accessor always `None`: killed by AC-1 (line 34) and AC-2 (line 58).
- M2, accessor returns an independent, leaked `Cancel::new()`: killed by AC-1 (line 34, identity) and AC-2 (line 65, the charge is admitted instead of denied).
- M3, accessor returns a leaked clone of the meter's own handle (same `Arc`, new allocation): survives. Through the public API this is behaviourally equivalent, because `Cancel` equality is shared-state identity. It breaks only FR-383-AC-3's "without cloning", and FR-383 assigns AC-3 to Inspection. Inspection of `self.cancel.as_ref()` confirms the borrow, so this is an expected survivor and not a test defect. A "handle from another meter" mutant reduces to M2, since any handle the meter does not poll is independent of it.
- M4, the accessor returns the caller's handle but the meter polls a different one (`with_cancel` stores `Cancel::new()` for polling and the given handle in a separate exposed field): killed by AC-2 (line 65).
- M5, a stale accessor after replacement (the exposed field keeps the first handle while polling moves to the replacement): killed by AC-1 (line 39).

Interior mutability (brief item 3): the returned `&Cancel` lets a holder of `&Meter` call `Cancel::cancel`, `poll` and `count_charges`. `poll`, together with its `tripped` and charge-count side effects, was already reachable from `&Meter`: `Meter: Clone` shares the `Arc`, and charging the clone polls the shared handle. Before this PR, the only party who could call `cancel(cause)` was the one that built the handle, because `with_cancel` takes an owned `Cancel` the caller made. The new reach is cancelling from `&Meter`. That is the exact grant FR-383 states, and its Decision relies on it for the QSL position-search meter. No meter counter, limit, injection or log becomes writable through the accessor.

## Examined criteria

- FR-383-AC-1 (examined): `Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`, `cancel()` is `Some(&Cancel)` equal by shared-state identity to `handle`, and unequal to an independently created handle. Replacing the handle reports only the replacement.
- FR-383-AC-2 (examined): Cancel the handle obtained from `meter.cancel()` with `Requested`, then charge the meter under sufficient limits: the charge is denied at its named point, that handle records `tripped() = Some(Requested)`, and all accounting state stays unchanged. On a separate fresh meter, cancel an independent handle while its installed handle remains live: an otherwise identical charge is admitted.
- FR-383-AC-3 (examined, Inspection): The accessor's implementation borrows the existing optional handle without cloning it or constructing a second `Meter` or `Counters`; calling it changes no meter accounting or injection state.
- FR-383-AC-4 (examined): The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std` + `alloc` build.

Each tagged test asserts its whole AC. The AC-1 test covers `None`, `Some(&first)`, inequality to the independent and the not-yet-installed replacement, and after replacement equality only to the replacement. The AC-2 test admits one charge first and checks the counters. It then cancels a clone taken through the accessor and asserts that `tripped()` is `None` before the charge. It asserts the exact kernel refusal `Incomplete { WorkUnits, limit: 3, consumed: 3, next_charge: 3, charge_point: FunctionCall }` (the `limit = consumed = w` cancel shape from `check_injected`), then `tripped() == Some(Requested)`, then `meter == before`, which covers every counter and, under test-support, the log. The independent-handle half admits the second charge and checks the counters. No `/// Trace:` lines are added. AC-3 and AC-4 are correctly left untagged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS, with no findings. The signature and semantics match FR-383. The change is purely additive with no compatibility layer, and all gates pass. Both tests kill every behavioural mutant. The one survivor (M3) is covered by AC-3's Inspection. Mergeable as-is.
