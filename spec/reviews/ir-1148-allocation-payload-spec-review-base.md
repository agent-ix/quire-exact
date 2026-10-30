---
id: SR-16681
title: "IR-1148 allocation payload spec review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; FR-389-AC-11/12 and TC-938 step 8"
review_set: subset
---

## Summary

Independent review of PR #104 at head `fc0da10ac5fe8d94fa008c897fac792057e65016`, including disposition of the one finding raised at `9ba24b015c22906f5d678648a7b87078f2aed620`. The changed contract specifies reservation-local byte amounts for `Allocation { requested }`, preserves shape refusals and earlier FR-389 criteria, and leaves QSV's downstream `IdentityRefusal` mapping to QSV. No applicable active AssuranceProfile was found in this repository.

## Examined scope

- FR-389 Behavior 9 and AC-11/12: `String` and flat-path `Vec` requested-byte rules, count overflow, empty text, shape-refusal precedence, source preservation.
- TC-938 steps and expected results 8 and 9: injected allocation failures, exact byte oracles, and synthetic boundary inputs to a private production-shared checked-count helper.
- Earlier FR-389 AC-1 through AC-10: checked for accidental narrowing; none found.

## Verdict

**PASS — FND-001 is resolved by TC-938 step 9's production-shared checked-count helper and synthetic boundary cases.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved at `fc0da10`: TC-938 step 9 uses synthetic `usize::MAX` and near-overflow inputs against the same private checked-count helper used by actual reservations, with a representable boundary oracle and inspection of real sites. | FR-389-AC-11, FR-389-AC-12, TC-938 steps 8-9 |

## Coverage

The existing TC-938 trace covers both changed ACs. The requested-byte oracles for ordinary text and path reservations are explicit and can distinguish an incorrect enclosing-identity estimate. The synthetic helper controls make count and byte overflow testable without impossible-size allocations. The downstream QSV byte mapping is outside this exact-only PR and requires separate QSV evidence.
