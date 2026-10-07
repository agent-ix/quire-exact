---
id: FR-358
title: "A meter denies one named occurrence and bounds its diagnostic charge log"
type: FR
relationships: []
---
# FR-358: A meter denies one named occurrence and bounds its diagnostic charge log

## Description

The `quire-exact` meter SHALL deny an injected, named charge exactly once at its selected 1-based occurrence. Where `test-support` is enabled, the meter SHALL bound the admitted-charge log to 4096 entries without changing exact accounting. The public `InjectedDenial::occurrence` SHALL be `NonZeroU64`, making zero unrepresentable.

## Behavior

1. When `with_injected_denial` installs a denial, the meter shall count subsequent admitted charges at that denial's `ChargePoint`, independently of admissions at other points. The selected next occurrence shall return `Incomplete` for that point as a work-limit denial. The refusal shall change no consumed counter, admission count, or admitted-charge log. The injection shall then be spent, so a later charge at the same point can be admitted under the ordinary limits. This applies to both `charge` and `charge_plan`.
2. Where `test-support` is enabled, the meter shall retain the first 4096 admitted charge points in order. The existing `admitted_charges() -> &[ChargePoint]` shall expose that prefix. An additive `charge_log_truncated() -> bool` shall be false through the 4096th admission. When the 4097th charge is admitted, the flag shall become true without changing admitted-charge accounting or counter consumption.
3. In a production build, `Counters` shall remain fixed-size. It shall own no heap-allocated charge log. The production `Meter` shall contain no admitted-charge log.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-358-AC-1 | With a denial installed at occurrence 2 of point P, first refuse an over-limit P charge under an ordinary limit, then admit P once and an unrelated point once. The next P charge returns `Incomplete` at P with the work counter as its limit kind, proving the ordinary refusal did not advance the named occurrence; all consumed counters, admission count, and test-support log equal their values immediately before each refusal. A subsequent P charge succeeds and updates ordinary accounting and the log exactly once. | Test |
| FR-358-AC-2 | With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan` returns `Incomplete` at `equality.plan` without changing any consumed counter, admission count, or test-support log; a subsequent valid `charge_plan` succeeds and records its ordinary plan charge. | Test |
| FR-358-AC-3 | The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction with zero is rejected by that type, while a nonzero occurrence constructs a denial. | Inspection |
| FR-358-AC-4 | Under `test-support`, after 4096 admissions `admitted_charges()` returns all 4096 points in order and `charge_log_truncated()` is false. On the 4097th admission the slice still contains exactly that prefix and the flag becomes true. | Test |
| FR-358-AC-5 | Under `test-support`, after the 4097th admitted charge, `admission_count()` and consumed work units reflect all 4097 admissions, including the one omitted from the diagnostic log. | Test |
| FR-358-AC-6 | Without `test-support`, production `Meter` contains no admitted-charge log and its `Counters` accounting state remains fixed-size and heap-free. | Inspection |
| FR-358-AC-8 | Under `test-support`, admit 4097 charges each consuming one work and one result unit. The log retains the first 4096 points and reports truncation, while `admission_count()`, consumed work and consumed results each equal 4097. | Test |
| FR-358-AC-9 | With work limit 4097 and a sufficient result limit under `test-support`, admit 4097 one-work-unit charges and request one more work unit. The charge returns `Incomplete` at its requested point with `WorkUnits`, `limit = consumed = 4097`, `next_charge = 1`; every counter, admission count, prefix log and truncation flag are unchanged. | Test |
| FR-358-AC-10 | With result limit 4097 and a sufficient work limit under `test-support`, admit 4097 charges each consuming one result unit, then request one more result unit. The charge returns `Incomplete` with `ResultUnits`, `limit = consumed = 4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation flag remain unchanged, including the work counter. | Test |
| FR-358-AC-11 | After 4097 admissions under `test-support`, inject an occurrence-1 denial for one named point. A charge at that point returns `Incomplete` with `WorkUnits` and leaves all counters, admission count, prefix log and truncation flag unchanged. Retrying that charge under sufficient ordinary limits succeeds once, increments work and admission count once, and does not extend the 4096-entry log. | Test |

## Status

AC-1, AC-2, AC-4 and AC-5 have tagged pre-IR-673 tests; AC-3 and AC-6 require inspection. AC-7 was moved to FR-368 without reusing its ID. AC-8 through AC-11 are planned and have no executable binders.

## Dependencies

- The `quire-exact` implementation is in `src/accounting.rs`. This requirement adds a one-shot denial and a bounded test-support diagnostic log; it does not add a compatibility layer or copy downstream code.
- `ix://agent-ix/quire-spec-language` tests use both `admitted_charges()` and `InjectedDenial` with integer occurrence literals or values, so its callers must construct nonzero occurrences when they adopt this field type.
- `ix://agent-ix/quire-contract-runtime` has a local meter copy and already uses `NonZeroU64`; RT #95 is the downstream pin-and-replace work and must consume the authoritative crate rather than vendor it.
- `ix://agent-ix/quire-contract-codegen` uses the runtime's `NonZeroU64` denial in tests. No direct usage was found in `ix://agent-ix/quire-contract-ir` or the locally available `quire-driver` checkout.
