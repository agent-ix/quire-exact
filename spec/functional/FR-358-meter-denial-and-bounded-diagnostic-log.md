---
id: FR-358
title: "A meter denies one named occurrence, bounds diagnostics and captures fixed-size intervals"
type: FR
relationships: []
---
# FR-358: A meter denies one named occurrence, bounds diagnostics and captures fixed-size intervals

## Description

The `quire-exact` meter SHALL deny an injected, named charge exactly once at its selected 1-based occurrence. Where `test-support` is enabled, the meter SHALL bound the admitted-charge log to 4096 entries without changing exact accounting. The public `InjectedDenial::occurrence` SHALL be `NonZeroU64`, making zero unrepresentable. The production meter SHALL expose optional fixed-size interval accounting of successful admissions and optional bounded per-charge event capture, both disabled by default.

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

3. In a production build, `Counters` shall remain fixed-size. It shall own no heap-allocated charge log. The production `Meter` shall contain no admitted-charge log by default; its separately enabled bounded per-charge capture is specified in behavior 10.
4. An injected denial shall report `Incomplete { limit_kind: WorkUnits, limit: w, consumed: w, next_charge: n, charge_point: P }`, where `w` is the admitted work consumed immediately before the denied charge and `P` is the selected point. The record shall be independent of the configured `ScalarLimits.work_units`: `n` is the denied charge's own work amount for `charge`, or the `pairs + 2` availability reservation for `charge_plan` at `equality.plan`. A successful `charge_plan` commits one work unit, but its injected denial reports the reservation it checked, not that eventual commit.
5. At the selected occurrence, an injected denial shall take precedence over an ordinary `ScalarLimits` counter refusal of that same charge, including a plan reservation that exceeds the remaining work. An active cancellation is checked before injection and is outside this ordinary-limit precedence rule.

6. The public, non-generic `Meter` SHALL expose `enable_charge_accounting(&mut self)`, `take_charge_accounting(&mut self) -> Option<ChargeAccounting>` and `disable_charge_accounting(&mut self) -> Option<ChargeAccounting>`.

   Enabling disabled capture SHALL start an empty interval without incorporating earlier admissions.

   Enabling already active capture SHALL preserve its unread amounts.

   Taking active capture SHALL return its current aggregate and reset only the interval to empty while leaving capture enabled.

   Taking disabled capture SHALL return `None`.

   Disabling active capture SHALL return its final aggregate and disable capture.

   Disabling already disabled capture SHALL return `None`.

   An enabled empty interval SHALL return `Some` with ten zeros.
7. The public `ChargeAccounting` SHALL expose `amount(&self, kind: LimitKind) -> u64` for all ten kinds.

   Its private state SHALL contain fixed-size, heap-free counters only, without charge points, ordered history, caller identities or arbitrary metadata.

   For each interval, `WorkUnits` and `ResultUnits` SHALL sum the actual successfully committed increments.

   Each of the eight semantic-size kinds named in behavior 2 SHALL retain the maximum actual validated request admitted during that interval, coalescing repeated same-kind requests.

   An absent request and an explicit zero SHALL both contribute zero.

   Capture SHALL observe actual requests independently of the meter's prior consumed maxima, rather than differences of global high-water values.
8. While capture is active, every successful admission through ordinary `charge`, its unit-only fast path and `charge_plan` SHALL contribute exactly once.

   A successful `charge_plan` SHALL contribute work 1, results 0 and its actual `ValueOccurrences` request, never its `pairs + 2` work reservation or result availability check.

   Ordinary refusals, injected denials, cancellation and failed plan reservations SHALL leave the interval unchanged and preserve the successful prefix.

   Capture SHALL continue aggregating admissions after diagnostic entry 4096, independently of diagnostic-log truncation.
9. Enabling, taking and disabling capture SHALL leave limits, consumed counters, admission counts, cancellation state and injected-denial state unchanged.

   Capture control operations SHALL charge no accounting overhead.

   The fixed-size capture SHALL introduce no allocation, request-count cap, admission cap, callback, trait object, lifetime parameter or generic parameter into `Meter`.

   `Meter` SHALL retain `Clone`, `Debug`, `Eq`, `PartialEq`, `Send`, `Sync` and no_std compatibility.

   Cloning SHALL copy the enabled/disabled state and unread aggregate as independent owned state.

   The production meter SHALL remain a count rather than an admitted-charge log.

10. The production `Meter` SHALL expose `enable_charge_capture(max_events, max_size_requests)`, `take_charge_capture()` and `disable_charge_capture()`. Enabling starts a new interval when disabled and preserves the active interval when already enabled. Taking returns the interval and resets it while keeping capture active with the same capacities; disabling returns the interval and ends capture. A disabled take or disable returns `None`. The event capture is independent of the fixed-size aggregate in behavior 6 and changes no charge outcome, counters, cancellation or injected-denial behavior.

    The meter SHALL produce one ordered event for every actual `Meter::charge` and `Meter::charge_plan` boundary in an active interval, with its charge point, requested work and result increments, every semantic-size request in insertion order including repeated kinds and exact integer amounts, and an admitted or denied decision. For `charge_plan`, the event SHALL record its actual one-work, zero-result commitment request, the exact `ValueOccurrences` pair request, and separate work `pairs + 2` and result one availability reservations; reservations do not become committed increments. The meter SHALL record denied boundaries, including cancellation and injection, without changing accounting.

    Capture SHALL retain only a complete event prefix within both caller-provided capacities. The meter SHALL omit a boundary that exceeds either capacity as a whole event and mark the interval truncated; later boundaries cannot restore completeness. Taking or disabling a truncated interval SHALL return a typed truncation outcome carrying that complete prefix, never a successful result that silently omits events. Event and size-request capacities bound capture storage independently of how many charges or duplicate requests the caller makes. Capture SHALL be available in default-feature and no_std builds, including calls whose internal Exact charge schedule is opaque to the caller.

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
| FR-358-AC-16 | In a default-feature build, a fresh meter returns `None` from take and disable. Admit work before enabling, enable capture, take twice and observe `Some` with all ten amounts zero both times. After an admission, enable again and verify the unread amounts survive. Disable returns that final aggregate, subsequent take/disable return `None`, and re-enable starts with zeros. Each control operation preserves limits, all consumed counters, admission count, cancellation and injected-denial state and consumes no work. | Test |
| FR-358-AC-17 | With sufficient limits and capture enabled, exercise ordinary general and unit-only charges with independently specified work/result increments and all eight semantic-size kinds, including repeated same-kind requests, explicit zeros and absent kinds. The aggregate exposes the sum of actual work/results and the maximum actual request for each size, through `amount` for all ten kinds. Separately admit IntegerBits 9 while disabled, enable, admit IntegerBits 3 and verify the interval reports 3 while global consumption remains 9. | Test |
| FR-358-AC-18 | With an active nonempty interval, independently attempt ordinary work, result and semantic-size refusals, an injected denial, cancellation, and failed plan work/result/size availability checks. Each leaves the entire interval equal to its pre-attempt value. A later successful admission extends the preceding successful prefix exactly once. On a sufficient meter, `charge_plan` with pairs 2 adds exactly work 1, results 0 and ValueOccurrences 2, rather than reservation work 4 or result 1. | Test |
| FR-358-AC-19 | On one shared meter with prior consumption, partition a fixed successful charge schedule across repeated takes, including an empty interval, repeated enable and a final disable. Summing the returned work/results and taking per-kind maxima of the eight sizes equals the independently expected actual schedule usage and an uninterrupted capture of that schedule; prior consumption remains excluded. Perform the same schedule with capture disabled and verify identical admission/refusal outcomes, global consumed counters and admission counts. | Test |
| FR-358-AC-20 | In default-feature and test-support builds, an enabled interval aggregates 4097 one-work/one-result admissions, with an independently specified largest semantic-size request on admission 4097: work/results equal 4097 and the size includes that last request. Under test-support, the existing ordered diagnostic prefix remains exactly 4096 records and its existing truncation flag changes only at 4097. | Test |
| FR-358-AC-21 | Inspect the production capture and `ChargeAccounting` types for private fixed-size heap-free state and absence of history, caller metadata, callbacks, trait objects or new Meter lifetime/generic parameters. Default construction and enabled capture/control operations allocate no heap storage. Verify the unchanged no_std build and Meter's Clone/Debug/Eq/PartialEq/Send/Sync contracts; cloning active capture copies unread amounts independently, so draining one clone does not drain the other. | Inspection |
| FR-358-AC-22 | With bounded production capture active, admit a charge requesting all eight semantic-size kinds, a repeated kind, two work units and one result unit; then deny a different point on its result limit. The two ordered events retain every requested amount, repeated request, point and exact admitted or denied decision; the denial preserves already consumed work. | Test |
| FR-358-AC-23 | With event capacity two and size-request capacity one, the third charge returns a typed truncated interval with the first two complete ordered events while meter admission count reaches three. Taking resets the scope; a later charge appears in a complete new interval. A zero size-request capacity reports truncation when a charge requests one size, including an explicit zero amount. | Test |
| FR-358-AC-24 | With production capture active, a public Exact integer arithmetic call yields its actual ordered operand, arithmetic and result-retain charge events and their requested amounts. A successful `charge_plan` adds a fourth event with work one, results zero, its exact pair-count size request, and separate work `pairs + 2` and result one availability reservations. A native integer ordering call using the direct unit-only path likewise produces all three actual boundary events. | Test |

## Status

AC-16 through AC-21 have public-meter executable bindings in `tests/ir1280_charge_accounting.rs` for the IR-1280 implementation. The kernel interval primitive does not complete QSL-1548 or `ix://agent-ix/quire-spec-language/FR-520` AC-11. QSL owns binder routing, read-presence entries (including zero-charge and Unavailable reads), per-call conservation and the semantic `accounting_version`; none belongs in the kernel capture.

PR #9 adds executable Trace bindings for AC-8 through AC-11. AC-7 was retired to FR-368; its ID is not reused. AC-12 and AC-13 have direct public-meter Trace bindings for the injected record and ordinary-limit precedence, including cancellation remaining first. AC-14 and AC-15 specify the IR-667 enriched test-support log and remain planned until executable bindings land; this specification does not claim runtime conformance. Test-support callers must adapt directly to typed records when implementation lands. Run `quire matrix` for the current criterion-to-test mapping.

## Dependencies

- The `quire-exact` implementation is in `src/accounting.rs`. This requirement adds a one-shot denial, a bounded test-support diagnostic log and optional fixed-size production interval accounting; it does not add a compatibility layer or copy downstream code.
- `ix://agent-ix/quire-spec-language` tests use both `admitted_charges()` and `InjectedDenial` with integer occurrence literals or values, so its callers must construct nonzero occurrences when they adopt this field type.
- `ix://agent-ix/quire-contract-runtime` has a local meter copy and already uses `NonZeroU64`; RT #95 is the downstream pin-and-replace work and must consume the authoritative crate rather than vendor it.
- `ix://agent-ix/quire-contract-codegen` uses the runtime's `NonZeroU64` denial in tests. No direct usage was found in `ix://agent-ix/quire-contract-ir` or the locally available `quire-driver` checkout.
