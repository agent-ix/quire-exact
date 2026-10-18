---
id: SR-12490
title: "Code and Rust review of IR-873 temporal visit charge point"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; src/accounting.rs; tests/ir673_core.rs; tests/ir873_temporal_visit.rs; FR-362-AC-21/22; FR-368-AC-1/2"
review_set: subset
---
## Summary

Reviewed quire-exact PR #54 at 9635f599. The new charge point is the final `ChargePoint::ALL` entry, uses the existing work-unit counter, and has exact public-meter boundary and injected-denial tests.

## Verdict

**PASS** — No code or Rust findings in the PR diff. The new point is at index 62 after `ModelNavigate` at 61; the old 0–61 index assertions and the ten `LimitKind` entries remain intact. `from_code` derives from `ALL` and `as_str`, and the exhaustive test independently spells every variant. Moving the new point fails the index assertion; changing the default charge kind fails the counter and `Incomplete` assertions (reasoned from the test bodies; no mutation runner was used).

## Scope and evidence

| Unit | Role | Evidence |
| --- | --- | --- |
| `src/accounting.rs` | examined | `TemporalVisit`, `ChargePoint::ALL`, `as_str`, `from_code`, `Charge::new`, `Meter::charge`, `LimitKind` |
| `tests/ir673_core.rs` | examined | Exact index 61/62, exhaustive spelling, uniqueness and round trip |
| `tests/ir873_temporal_visit.rs` | examined | Exact/one-under WorkUnits, all counters, no semantic sizes, atomic one-shot denial |
| FR-362-AC-21 and AC-22 | examined | Each bound to one new `#[trace]` test in computed matrix |
| FR-368-AC-1 and AC-2 | examined | AC-1 extended, AC-2 existing limit vocabulary unchanged |
| TC-910 and TC-916 | context_only | Test procedures compared with assertions |

`cargo fmt --all -- --check`, focused default and `test-support` tests (six test executions), `cargo clippy --locked -j2 --all-targets -- -D warnings`, and `cargo deny check` passed. Computed coverage changed from 165/194 backed, 29 unbacked, 0 contradicted on base to 167/194 backed, 27 unbacked, 0 contradicted on PR head. The existing `TC-910` unmatched-row warning occurs on base and head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
