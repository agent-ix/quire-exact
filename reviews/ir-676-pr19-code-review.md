---
id: SR-5060
title: "Code and Rust review of IR-676 PR 19"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@0fb208955aeeca6d6f7599f2e36e4425ace7b7eb; src/accounting.rs; spec/functional/FR-358; spec/test-cases/TC-906"
review_set: subset
---

## Summary

Independent diff-scoped code and Rust review of the public meter denial record, its new tests, and the status text. The record assertions are field-exact and use separate work limits and ordinary-shortage controls. The cancellation precedence control has an oracle gap.

## Verdict

**CONDITIONAL** — one medium finding. No new production branch or vendored source was added. `git diff --check` passed; this reviewer did not run full gates.

## Examined Units

- `FR-358-AC-12` (examined): Injected ordinary and plan refusals after two admitted work units report exact `Incomplete` fields independently of work limits 10 and 100, with plan `next_charge = pairs + 2`.
- `FR-358-AC-13` (examined): Selected injection precedes ordinary size and plan work shortages while cancellation is checked before injection.
- `TC-906-Step-9` (examined): Compare exact ordinary and plan records and pre-refusal state under both configured limits.
- `TC-906-Step-10` (examined): Establish both ordinary shortages independently, then compare injected records and atomic state; cancellation remains earlier.
- `src/accounting.rs` (examined): `Incomplete.limit` rustdoc and both new tagged tests against the existing `check_injected`, `charge`, `charge_plan`, `Cancel`, and `assert_unchanged` paths.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The cancellation control does not prove that cancellation leaves the selected injection pending. The cancellation and injection branches return identical `Incomplete` fields, while `assert_unchanged` checks only public counters, admission count, and log. A mutant that polls cancellation, spends the injection, then returns the cancellation-shaped record passes `cancel.tripped()` and all current assertions. After the cancelled refusal, replace the meter's handle with a fresh live `Cancel` and retry the named charge; the still-pending injection must refuse once before the next retry admits. | src/accounting.rs:1284-1306; FR-358-AC-13 | correct-requirement-no-evidence |

## Coverage

The two new `Trace:` tags bind FR-358-AC-12 and AC-13 in the computed Quire matrix. The ordinary record, plan reservation, two-limit independence, ordinary size shortage, ordinary plan shortage, and counter/admission atomicity paths are exercised. Plan completion: not assessed.
