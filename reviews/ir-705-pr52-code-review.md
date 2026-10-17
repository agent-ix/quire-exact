---
id: SR-11980
title: "IR-705 quire-exact PR 52 code review: single-pass Meter::charge and uncredited Kani harness"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@9eab47cc66fa5fcc6a88ed3e38b4dbed01e8d60d; Cargo.toml; src/accounting.rs; src/accounting/kani_proofs.rs"
review_set: subset
---

## Summary

Ticket: IR-705. Code review with the rust-review lane folded in, over
`git diff origin/main...HEAD` of quire-exact PR 52 (seven commits, head
9eab47cc). Re-measured locally: `cargo test --features test-support` (all
suites pass, 25 accounting tests), `cargo clippy --all-targets --features
test-support -- -D warnings` clean, `cargo fmt --check` clean. Kani was not
run, per the brief.

Checked:

- `Meter::charge` (src/accounting.rs:793-850): one pass over
  `charge.sizes.iter()`. A valid request folds into a local
  `[Option<u64>; 10]` maximum; an invalid one replaces `first_fail` only when
  its kind index is strictly lower, so the lowest field wins and the first
  inserted failing amount wins within a kind. This is the same refusal as the
  old `LimitKind::ALL` x requests double loop. No counter is written before
  the refusal return or the cumulative work/result checks. Size requests
  naming `WorkUnits`/`ResultUnits` are still validated against their limits,
  then superseded by the cumulative totals, as before.
- The test-support `AdmittedCharge::observe` call moved into the
  validation pass. `observe` keeps a per-kind maximum and ignores
  WorkUnits/ResultUnits, so insertion order does not change the record. The
  record is dropped on refusal, so the log is unchanged.
- `Option::is_none_or` is within MSRV 1.98. No `unsafe`, no new panic path;
  `sizes[kind.index()]` is in bounds for all ten kinds.
- `charge_plan` has no loop; `InlineSizes::iter` is a two-part chain.
- New tests (src/accounting.rs:1176-1913): oracles are literal expected
  values or a separately written fold (`expected_record`), not calls back
  into the meter. Each one fails for the mutant the PR names (coalesce
  before refusal, later field wins, request-count cap, refusal spends the
  injection, skipped validation, log overwrite or append on refusal).
- `kani_proofs.rs` is only built under `cfg(kani)`, carries no `Trace` tag,
  and its comment at line 195 says AC-9 is PLANNED on IR-844. The
  `unexpected_cfgs` check-cfg in Cargo.toml keeps lint green.
- SPDX header on the new file; no local paths or private text in the diff
  (public repo).

## Verdict

PASS WITH FINDINGS (one low). The single-pass rewrite keeps the old refusal
order, atomicity and high-water behaviour, and the new tests have independent
oracles that can fail. The one finding is a maintainability nit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The local maxima array uses a literal length (`[None; 10]`) and eight `observe_size(0..7, sizes[0..7])` writes with literal indices. This repeats the `LimitKind::index` mapping by hand: if a kind is added or the order changes, a semantic size is silently not written. `LimitKind::ALL.len()` for the length and named `LimitKind::X.index()` indices would tie it to the enum. | src/accounting.rs:799, src/accounting.rs:834-841 |

## Dispositions

Round 1, checked at head 8ca1796d84188735672352ea609ee62bf1991311 (fix commits 9eab47c..8ca1796).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fix 29cd242: array lengths come from LimitKind::COUNT (= ALL.len()); the eight writes name LimitKind::X.index() and observe_size takes a LimitKind; const assert COUNT == 8 + 2 fails to compile if a counter is added. Straight-line writes kept, as AC-11 forbids an ALL traversal. |
