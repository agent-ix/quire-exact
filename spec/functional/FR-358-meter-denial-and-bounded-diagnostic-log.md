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
2. Where `test-support` is enabled, the meter SHALL retain the first 4096 admissions in one ordered diagnostic log of fixed-size typed records, each carrying its `ChargePoint` and a semantic-size projection of that admission.

   The test-support admitted-charge accessor SHALL expose that record prefix, replacing its point-only slice contract.

   Each record SHALL contain one absent-or-present `u64` amount for each of the eight non-cumulative semantic-size kinds: `IntegerBits`, `DecimalDigits`, `ScaleExpansion`, `TextInputBytes`, `TextScalars`, `NormalizedScalars`, `UnitEdges` and `ValueOccurrences`.

   For each semantic-size kind present in the admitted charge, the recorded amount SHALL equal the maximum of the actual validated requests for that kind within that admission.

   The record SHALL coalesce repeated same-kind requests into that per-admission maximum, retaining neither their list, order nor multiplicity.

   The record payload SHALL remain fixed-size with no heap-owned request collection, regardless of the number of requests in a charge.

   An absent kind SHALL remain distinguishable from a present maximum of zero.

   The meter SHALL record fixed-width amounts only after the actual admission validates that every request fits its configured `u64` limit.

   This semantic-size projection excludes cumulative `WorkUnits` and `ResultUnits`; it does not alter the charge API's accepted kinds or their existing accounting behavior.

   The meter SHALL derive this projection from the actual validated requests in that admission rather than reconstruct them from counter maxima or operation results.

   A successful `charge_plan` record SHALL carry its actual `value_occurrences` request, not its work availability reservation as a size.

   Ordinary refusals, injected denials and cancellation SHALL append no record.

   The existing `charge_log_truncated() -> bool` SHALL be false through the 4096th admission.

   When the 4097th charge is admitted, the flag SHALL become true without extending the record prefix or changing exact accounting.

   The enriched records SHALL share the existing admission-entry cap and truncation flag.

   The meter SHALL add neither a second diagnostic log nor an execution limit.

   Arbitrarily many repeated or zero requests SHALL remain subject only to existing admission rules, without a new request-count denial or limit.

3. In a production build, `Counters` shall remain fixed-size. It shall own no heap-allocated charge log. The production `Meter` shall contain no admitted-charge log.
4. An injected denial shall report `Incomplete { limit_kind: WorkUnits, limit: w, consumed: w, next_charge: n, charge_point: P }`, where `w` is the admitted work consumed immediately before the denied charge and `P` is the selected point. The record shall be independent of the configured `ScalarLimits.work_units`: `n` is the denied charge's own work amount for `charge`, or the `pairs + 2` availability reservation for `charge_plan` at `equality.plan`. A successful `charge_plan` commits one work unit, but its injected denial reports the reservation it checked, not that eventual commit.
5. At the selected occurrence, an injected denial shall take precedence over an ordinary `ScalarLimits` counter refusal of that same charge, including a plan reservation that exceeds the remaining work. An active cancellation is checked before injection and is outside this ordinary-limit precedence rule.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-358-AC-1 | With a denial installed at occurrence 2 of point P, first refuse an over-limit P charge under an ordinary limit, then admit P once and an unrelated point once. The next P charge returns `Incomplete` at P with the work counter as its limit kind, proving the ordinary refusal did not advance the named occurrence; all consumed counters, admission count, and test-support log equal their values immediately before each refusal. A subsequent P charge succeeds and updates ordinary accounting and the log exactly once. | Test |
| FR-358-AC-2 | With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan` returns `Incomplete` at `equality.plan` without changing any consumed counter, admission count, or test-support log; a subsequent valid `charge_plan` succeeds and records its ordinary plan charge. | Test |
| FR-358-AC-3 | The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction with zero is rejected by that type, while a nonzero occurrence constructs a denial. | Inspection |
| FR-358-AC-4 | Under `test-support`, after 4096 admissions the admitted-charge accessor returns all 4096 records with their points in order and `charge_log_truncated()` is false. On the 4097th admission the slice still contains exactly that prefix and the flag becomes true. | Test |
| FR-358-AC-5 | Under `test-support`, after the 4097th admitted charge, `admission_count()` and consumed work units reflect all 4097 admissions, including the one omitted from the diagnostic log. | Test |
| FR-358-AC-6 | Without `test-support`, production `Meter` contains no admitted-charge log and its `Counters` accounting state remains fixed-size and heap-free. | Inspection |
| FR-358-AC-8 | Under `test-support`, admit 4097 charges each consuming one work and one result unit. The log retains the first 4096 points and reports truncation, while `admission_count()`, consumed work and consumed results each equal 4097. | Test |
| FR-358-AC-9 | With work limit 4097 and a sufficient result limit under `test-support`, admit 4097 one-work-unit charges and request one more work unit. The charge returns `Incomplete` at its requested point with `WorkUnits`, `limit = consumed = 4097`, `next_charge = 1`; every counter, admission count, prefix log and truncation flag are unchanged. | Test |
| FR-358-AC-10 | With result limit 4097 and a sufficient work limit under `test-support`, admit 4097 charges each consuming one result unit, then request one more result unit. The charge returns `Incomplete` with `ResultUnits`, `limit = consumed = 4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation flag remain unchanged, including the work counter. | Test |
| FR-358-AC-11 | After 4097 admissions under `test-support`, inject an occurrence-1 denial for one named point. A charge at that point returns `Incomplete` with `WorkUnits` and leaves all counters, admission count, prefix log and truncation flag unchanged. Retrying that charge under sufficient ordinary limits succeeds once, increments work and admission count once, and does not extend the 4096-entry log. | Test |
| FR-358-AC-12 | After admitting two work units, inject occurrence 1 at point P and attempt a three-work-unit `charge` at P under otherwise sufficient limits whose configured work limits are 10 and 100 on separate meters. Both refusals report exactly `limit_kind = WorkUnits`, `charge_point = P`, `limit = consumed = 2`, and `next_charge = 3`, independently of either configured limit. Repeat with an injected `equality.plan` denial, `pairs = 2`, the same pre-consumed work and limits, and sufficient value-occurrence and result limits: both `charge_plan` refusals report `WorkUnits`, `charge_point = equality.plan`, `limit = consumed = 2`, and `next_charge = 4` (`pairs + 2`), rather than the one work unit a successful plan would commit. | Test |
| FR-358-AC-13 | With an occurrence-1 injected denial selected, make the same `charge` independently fail an ordinary semantic-size counter and make a `charge_plan` with `pairs = 2` independently fail its work reservation under a configured work limit of 3 after two admitted work units, with all other limits sufficient. In each case, the injected `WorkUnits` record is returned before the ordinary refusal: `limit = consumed` equals the pre-charge work, `next_charge` is the charge's work amount or the plan's reservation respectively, and every consumed counter and admission count remain unchanged. The criterion does not override cancellation, which is checked before injection. | Test |
| FR-358-AC-14 | Under `test-support` and sufficient limits, admit an IntegerBits request of 9 followed by 3 and then a charge with no size request. The records expose respectively 9, 3 and no IntegerBits request while the consumed IntegerBits maximum remains 9. A separate charge with IntegerBits requests 9, 3 and zero exposes present 9; a charge with any positive number of IntegerBits zero requests exposes present zero. Both have the same fixed payload size as a single-request record, with no heap-owned request collection and no retained request list, order or multiplicity. A successful `charge_plan` with pairs 2 records ValueOccurrences 2 and no size request for its four-unit work reservation. Independently attempt an ordinary size refusal, an ordinary work refusal, an injected denial and cancellation; each preserves the preceding record prefix, every counter, admission count and truncation flag. | Test |
| FR-358-AC-15 | Under `test-support`, admit 4097 one-work-unit charges with independently chosen semantic-size requests, including repeated same-kind requests and distinct amounts at admissions 4096 and 4097. The single record log contains exactly the first 4096 fixed-size records with their points and per-admission semantic-size maxima in admission order; truncation is false at admission 4096 and true at 4097. Work consumption and admission count reach 4097, including the omitted record; existing size maxima still include the 4097th charge. | Test |

## Status

PR #9 adds executable Trace bindings for AC-8 through AC-11. AC-7 was retired to FR-368; its ID is not reused. AC-12 and AC-13 have direct public-meter Trace bindings for the injected record and ordinary-limit precedence, including cancellation remaining first. AC-14 and AC-15 specify the IR-667 enriched test-support log and remain planned until executable bindings land; this specification does not claim runtime conformance. Test-support callers must adapt directly to typed records when implementation lands. Run `quire matrix` for the current criterion-to-test mapping.

## Dependencies

- The `quire-exact` implementation is in `src/accounting.rs`. This requirement adds a one-shot denial and a bounded test-support diagnostic log; it does not add a compatibility layer or copy downstream code.
- `ix://agent-ix/quire-spec-language` tests use both `admitted_charges()` and `InjectedDenial` with integer occurrence literals or values, so its callers must construct nonzero occurrences when they adopt this field type.
- `ix://agent-ix/quire-contract-runtime` has a local meter copy and already uses `NonZeroU64`; RT #95 is the downstream pin-and-replace work and must consume the authoritative crate rather than vendor it.
- `ix://agent-ix/quire-contract-codegen` uses the runtime's `NonZeroU64` denial in tests. No direct usage was found in `ix://agent-ix/quire-contract-ir` or the locally available `quire-driver` checkout.
