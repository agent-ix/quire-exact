---
id: SR-5161
title: "gap-analysis review of IR-704 native values"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@0df4eb3f9f264d6e67e0ae266320662a8a6f6b9e; src/equality.rs, src/key.rs, src/lib.rs, src/value.rs, src/value/native.rs, src/value/value_type.rs; FR-370-AC-1..8, TC-918"
review_set: subset
---

## Summary

All eight FR-370 criteria have trace-tagged tests, and the changed behavior has FR-370 ownership. Two tagged criteria have incomplete charge assertions; plan completion was not assessed.

## Verdict

**CONDITIONAL** — Two medium evidence gaps remain.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The equal UUID and Timestamp cases assert a true result and one planned pair but pass a temporary meter, so a regression charging equal native pairs differently from the Boolean leaf still passes. Compare the equal-pair meter counters to a Boolean equal-pair control. | src/value.rs:1416-1423 |
| FND-002 | medium | The existing-kind control asserts value occurrences and completed equality but never measures or compares admission/work/result charges for the old kinds. A changed accounting schedule for an existing scalar or composite can pass the new FR-370-AC-5 test. Add representative metered control assertions against the established expected charges. | src/value.rs:1491-1556 |

## Coverage

Reviewed the exact six-file PR diff against FR-370-AC-1 through AC-8 and TC-918. The computed Quire matrix tags all eight FR-370 criteria; repository-wide strict matrix reports unrelated planned criteria. Pre-PR `make ci` and changed-document Quire validation receipts at the reviewed SHA both report exit 0. Plan completion: not assessed.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 3c75079496ed49e720a87c0d6e1ee0bebe3eff59 |
| FND-002 | fixed | 3c75079496ed49e720a87c0d6e1ee0bebe3eff59 |

## Disposition verdict

**PASS at 3c75079496ed49e720a87c0d6e1ee0bebe3eff59** — Both medium charge-evidence findings are fixed. The focused native-equality and existing-kind control tests pass on this head.
