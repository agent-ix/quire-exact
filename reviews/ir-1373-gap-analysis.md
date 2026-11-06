---
id: SR-2026110101329
title: IR-1373 PR 134 gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-exact; IR-1373; PR #134; spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md;
  src/accounting.rs; src/accounting/units.rs; src/lib.rs; tests/ir1373_charge_capture.rs'
review_set: subset
---

## Summary

Ticket: IR-1373. Frozen PR #134 head b6ef75a. No applicable AssuranceProfile found.

## Examined Scope

```yaml
scope:
- id: FR-358-AC-22
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With bounded production capture active, admit a charge requesting all eight
    semantic-size kinds, a repeated kind, two work units and one result unit; then
    deny a different point on its result limit. The two ordered events retain every
    requested amount, repeated request, point and exact admitted or denied decision;
    the denial preserves already consumed work.
- id: FR-358-AC-23
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With event capacity two and size-request capacity one, the third charge
    returns a typed truncated interval with the first two complete ordered events
    while meter admission count reaches three. Taking resets the scope; a later charge
    appears in a complete new interval. A zero size-request capacity reports truncation
    when a charge requests one size, including an explicit zero amount.
- id: FR-358-AC-24
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With production capture active, a public Exact integer arithmetic call
    yields its actual ordered operand, arithmetic and result-retain charge events
    and their requested amounts. A successful `charge_plan` adds a fourth event with
    work one, results zero, its exact pair-count size request, and separate work `pairs
    + 2` and result one availability reservations. A native integer ordering call
    using the direct unit-only path likewise produces all three actual boundary events.
```

## Verdict

CONDITIONAL: AC-24 has a trace binding but incomplete value oracles.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-24 tests assert points and selected fields but never assert requested work and result increments or full size vectors for the Exact arithmetic and native ordering events; wrong captured amounts on those boundaries still pass. | tests/ir1373_charge_capture.rs:133-153,193-203 |

## Coverage

Plan completion: not assessed. AC-22, AC-23 and AC-24 have computed nonignored test bindings. Reviewed actual general, unit-only and plan charge boundaries. Coder-reported focused tests, focused Clippy, spec validation and diff check accepted without rerun. Aggregate gates belong to lead.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Commit fed5e14 compares complete expected ChargeEvent values for all three Exact arithmetic and all three native ordering events, including work, results, reservations, ordered sizes and admission. |
