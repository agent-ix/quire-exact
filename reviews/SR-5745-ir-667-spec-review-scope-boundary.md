---
id: SR-5745
title: "scope-boundary review of IR-667 normalize observability"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact@36a6f5dd7d031b87142dd855e5726657aab85a47; IR-667; spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md, spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md, spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md, spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md; src/accounting.rs, src/numeric.rs, tests/ir653_scalar.rs"
review_set: subset
---

## Summary

Ticket: IR-667. Frozen spec-only PR29; FR358 owns meter diagnostics (cross-cutting); FR362 owns rational arithmetic amounts (core). TC906 owns meter checks; TC910 owns independent arithmetic observations. Public callers build charge requests; the meter guarantees validated observations. Test-support callers must directly adapt; expression evaluation stays caller-owned. Production Meter/Counters contain no log; no production observability API, dependency or compatibility channel is introduced.

## Verdict

**PASS** — No defects within this method scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope Examined

```yaml
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: The `quire-exact` meter SHALL deny an injected, named charge exactly once at its selected 1-based occurrence. Where `test-support` is enabled, the meter SHALL bound the admitted-charge log to 4096 entries without changing exact accounting. The public `InjectedDenial::occurrence` SHALL be `NonZeroU64`, making zero unrepresentable.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 2. Where `test-support` is enabled, the meter SHALL retain the first 4096 admissions in one ordered diagnostic log of typed records, each carrying its `ChargePoint` and the actual size requests validated for that admission.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The test-support admitted-charge accessor SHALL expose that record prefix, replacing its point-only slice contract.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Each recorded size SHALL retain its `LimitKind` and exact admitted amount, including repeated requests for the same kind.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: An absent request SHALL remain distinguishable from an explicit zero.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The meter SHALL capture these requests from the actual admitted charge rather than reconstruct them from counter maxima or operation results.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: A successful `charge_plan` record SHALL carry its actual `value_occurrences` request, not its work availability reservation as a size.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Ordinary refusals, injected denials and cancellation SHALL append no record.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The existing `charge_log_truncated() -> bool` SHALL be false through the 4096th admission.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: When the 4097th charge is admitted, the flag SHALL become true without extending the record prefix or changing exact accounting.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The enriched records SHALL share the existing admission-entry cap and truncation flag.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The meter SHALL add neither a second diagnostic log nor an execution limit.
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit P charge under an ordinary limit, then admit P once and an unrelated point once. The next P charge returns `Incomplete` at P with the work counter as its limit kind, proving the ordinary refusal did not advance the named occurrence; all consumed counters, admission count, and test-support log equal their values immediately before each refusal. A subsequent P charge succeeds and updates ordinary accounting and the log exactly once.
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan` returns `Incomplete` at `equality.plan` without changing any consumed counter, admission count, or test-support log; a subsequent valid `charge_plan` succeeds and records its ordinary plan charge.
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction with zero is rejected by that type, while a nonzero occurrence constructs a denial.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor returns all 4096 records with their points in order and `charge_log_truncated()` is false. On the 4097th admission the slice still contains exactly that prefix and the flag becomes true.
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()` and consumed work units reflect all 4097 admissions, including the one omitted from the diagnostic log.
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge log and its `Counters` accounting state remains fixed-size and heap-free.
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one result unit. The log retains the first 4096 points and reports truncation, while `admission_count()`, consumed work and consumed results each equal 4097.
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`, admit 4097 one-work-unit charges and request one more work unit. The charge returns `Incomplete` at its requested point with `WorkUnits`, `limit = consumed = 4097`, `next_charge = 1`; every counter, admission count, prefix log and truncation flag are unchanged.
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`, admit 4097 charges each consuming one result unit, then request one more result unit. The charge returns `Incomplete` with `ResultUnits`, `limit = consumed = 4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation flag remain unchanged, including the work counter.
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial for one named point. A charge at that point returns `Incomplete` with `WorkUnits` and leaves all counters, admission count, prefix log and truncation flag unchanged. Retrying that charge under sufficient ordinary limits succeeds once, increments work and admission count once, and does not extend the 4096-entry log.
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 'After admitting two work units, inject occurrence 1 at point P and attempt a three-work-unit `charge` at P under otherwise sufficient limits whose configured work limits are 10 and 100 on separate meters. Both refusals report exactly `limit_kind = WorkUnits`, `charge_point = P`, `limit = consumed = 2`, and `next_charge = 3`, independently of either configured limit. Repeat with an injected `equality.plan` denial, `pairs = 2`, the same pre-consumed work and limits, and sufficient value-occurrence and result limits: both `charge_plan` refusals report `WorkUnits`, `charge_point = equality.plan`, '
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 'With an occurrence-1 injected denial selected, make the same `charge` independently fail an ordinary semantic-size counter and make a `charge_plan` with `pairs = 2` independently fail its work reservation under a configured work limit of 3 after two admitted work units, with all other limits sufficient. In each case, the injected `WorkUnits` record is returned before the ordinary refusal: `limit = consumed` equals the pre-charge work, `next_charge` is the charge''s work amount or the plan''s reservation respectively, and every consumed counter and admission count remain unchanged. The criterion '
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request of 9 followed by 3 and then a charge with no size request. The records expose respectively 9, 3 and no IntegerBits request while the consumed IntegerBits maximum remains 9. Separate admitted explicit-zero and repeated same-kind requests preserve zero and every requested amount respectively. A successful `charge_plan` with pairs 2 records ValueOccurrences 2 and no size request for its four-unit work reservation. Independently attempt an ordinary size refusal, an ordinary work refusal, an injected denial and cancellation; e
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently chosen size requests, including distinct amounts at admissions 4096 and 4097. The single record log contains exactly the first 4096 points and their actual requests in order; truncation is false at admission 4096 and true at 4097. Work consumption and admission count reach 4097, including the omitted record; existing size maxima still include the 4097th charge.
- id: FR-362
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: When evaluating kernel-owned integer or rational arithmetic, integer or rational ordering, or a Boolean connective over already-decided operands, the `quire-exact` kernel SHALL return the exact outcome and charge the point order and amounts defined below.
- id: FR-362
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: The kernel SHALL construct rational values with a positive denominator, numerator and denominator reduced to lowest terms, and a unique zero representation `0/1`.
- id: FR-362
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Where `test-support` is enabled, the kernel SHALL expose each admitted rational normalize `integer_bits` request through the actual size observations defined by [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md), independently of the final high-water counter and the reduced result.
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`; for negate, use each member as a unary operand. Compare integer add, subtract, multiply and negate only when the corresponding checked `i128` operation returns a value. Repeat over reproducibly sampled signed inputs. Every compared kernel result equals the independent checked result.
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently authored decimal value `340282366920938463463374607431768211454`; negating `Integer::from(i128::MIN)` completes to `170141183460469231731687303715884105728`. Neither result is narrowed to `i128`.
- id: FR-362-AC-3
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: For fixed and reproducibly sampled rational pairs whose checked `i128` reference intermediates fit and whose denominators are nonzero, rational add, subtract, multiply, divide by nonzero and negate equal an independently reduced-fraction oracle. A zero rational divisor returns `Undefined(DivisionByZero)` after only the operand charge.
- id: FR-362-AC-4
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: Integer and rational comparisons over fixed and reproducibly sampled pairs equal independently checked numeric comparisons for equality and order. Decimal comparison is specified by [FR-363](./FR-363-metered-decimal-ordering.md).
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: For a successful representative of each integer arithmetic operation, each rational arithmetic operation, integer ordering and rational ordering, the admitted point sequence equals its family row in Behavior. A successful already-decided Boolean retention admits only `boolean.result-retain`.
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate the maximum size amount across their points from the formulas in Behavior, using the unreduced numerator and denominator for rational normalization. The observed final `integer_bits` and `value_occurrences` equal those maxima; work consumption equals the number of admitted points and result consumption is one.
- id: FR-362-AC-7
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: On one meter, directly add integers 5 and 7, compare rationals 1/2 and 2/3 for less-than, then retain an already-decided Boolean `true`. The admitted atom sequence consumes 7 work units and 3 result units, derived from the three family rows in Behavior.
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: Inject a one-shot denial at each named point in representative integer arithmetic, rational arithmetic, integer ordering, rational ordering and already-decided Boolean retention calls. Each returns `Incomplete` naming the denied point, with no completed value, no result unit for the denied charge and no later point admitted by that call.
- id: FR-362-AC-9
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: For every already-decided Boolean connective, the completed value matches its truth table and the kernel admits `boolean.result-retain` once, consuming one work and one result unit.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and negate, and integer/rational ordering, independently calculate the required bit high-water amount from Behavior. On fresh meters with other limits sufficient, that exact bit limit completes with the expected value, while a limit one below stops at the first point whose amount exceeds it, with exact prior consumption, request and admitted prefix and no result unit. Include cancellation `1000 - 999`, `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate `-5/8`, and `7/3 < 5/8`. '
- id: FR-362-AC-11
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`, typed domain `Refused`, and `Incomplete` with no partial value; `completed()` returns a value only for `Completed`, including false. On fresh meters, rational division by zero retains only operands; integer `2 + 2` outside the bound `[0, 3]` retains operands and arithmetic but no result unit; a rational result outside its domain retains operands, arithmetic and normalize but no result unit. A denied first operand charge admits nothing. Each stop preserves exactly the admitted prefix, work totals and size maxima, w
- id: FR-362-AC-12
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: '`Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator 1 and denominator 2, proving reduction to lowest terms and cancellation of two negative signs. `Rational::new(1, -2)` exposes -1/2. For either denominator 5 or -5, a zero numerator exposes exactly 0/1.'
- id: FR-362-AC-13
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: For numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)` is false. Dividing `1/1` by `3/1` under that domain returns `Refused(RationalOutOfDomain { target })` after normalization with no result unit; the identical call without a domain completes to `1/3`.
- id: FR-362-AC-14
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: Call the five scalar atoms `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0` directly on one meter. They complete to true, 1, true, 0, false in order, consuming 15 work and 5 result units. With work limit 14, the final ordering stops at `ordering.result-retain` with `limit = consumed = 14`, `next_charge = 1` and four result units.
- id: FR-362-AC-15
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: Directly divide rational `3/1` by `2/1` on a fresh meter. The result is `3/2`, with high-water `integer_bits = 3`, value occurrences 2, work 4 and results 1; work limit 3 stops at `rational-arithmetic.result-retain` after the first three points and consumes no result unit.
- id: FR-362-AC-16
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: On one meter directly add integers `0 + 1` and then `1 + 2`. The results are 1 and 3; the integer point sequence occurs twice, high-water `integer_bits = 3`, occurrences 2, work 6 and results 2. Caller fold evaluation is outside this direct-atom criterion.
- id: FR-362-AC-17
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain` after operands and arithmetic, with zero result units; work limit 3 completes to 12 with three work and one result unit. For rational `(2/3) * (3/2)`, work limit 2 stops at `rational-arithmetic.normalize`, work limit 3 stops at `rational-arithmetic.result-retain`, and work limit 4 completes to `1/1`; each denied call preserves exactly its admitted prefix and consumes zero result units.
- id: FR-362-AC-18
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 'For `k` in 63, 64, 65, 127, 128, 200 and 511, compare exact results and charged bit high-water sizes around `2^k`: `(2^k - 1)(2^k + 1)` charges `2k + 1` bits for a `2k`-bit result, `(2^k)^2` charges `2k + 2` for a `2k + 1`-bit result, and adding `(2^k - 1) + 1` charges `k + 1` for a `k + 1`-bit result. Subtracting `(2^k + 1) - 2^k` or `2^k - 2^k` still charges `k + 2` bits despite a one-bit result.'
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add `8 + 3` and `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`, without a result bound and with all non-bit limits sufficient. The independently calculated `integer-arithmetic.arithmetic` bit amounts are respectively 5, 5, 5 and 6: add/subtract use `max(B(a), B(c)) + 1`, including the larger right operand and a result that cancels to one bit; multiply uses `B(a) + B(c)`. At each exact `integer_bits` limit, the call completes to respectively 11, 11, 1 and 24, admits operands, arithmetic and result-retain in '
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On fresh sufficient public meters under `test-support`, evaluate the six rational calls listed below with no result domain. Each completes to the listed canonical value and admits operands, arithmetic, normalize and result-retain in order, consuming four work units and one result unit. The actual normalize record contains exactly one size request, IntegerBits with the listed amount independently derived from the unreduced parts and `B(0) = 1`. The recorded arithmetic amount equals its separate listed bound. The normalize observation is not replaced by that bound, the final bit maximum or the r
- id: TC-906:line-17
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 1. Set `integer_bits` limit to 8 and other limits generously, then install a denial for the second `FunctionCall` charge. Snapshot every counter, the admission count and the test-support log; attempt a `FunctionCall` charge with an `integer_bits` size of 9, and compare state to the snapshot. Admit a valid `FunctionCall` and one unrelated charge; snapshot again. Attempt the next valid `FunctionCall`, then retry it under the same limits.
- id: TC-906:line-18
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 2. On a fresh meter, install a denial for the first `EqualityPlan`; snapshot its state, call `charge_plan` with a valid pair count, and call it again.
- id: TC-906:line-19
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 3. Inspect the public `InjectedDenial` field type and construct an occurrence using `NonZeroU64::new(1)`; verify that `NonZeroU64::new(0)` yields `None`.
- id: TC-906:line-20
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 4. Under `test-support`, admit 4096 charges with distinguishable points near the boundary, inspect the log and truncation flag, then admit one more and inspect the log, flag, admission count and work units.
- id: TC-906:line-21
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 5. Without `test-support`, inspect production `Meter` for absence of an admitted-charge log and `Counters` for fixed-size, heap-free accounting state.
- id: TC-906:line-23
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 6. For AC-8, repeat all 4097 admissions with one result unit each; read both cumulative counters and the bounded log.
- id: TC-906:line-24
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 7. For AC-9 and AC-10, construct separate meters with work and result limits exactly 4097, admit 4097 matching charges, then attempt one more matching unit and compare every counter and diagnostic field to the snapshot.
- id: TC-906:line-25
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 8. For AC-11, after 4097 admissions inject an occurrence-1 named denial, compare state before/after its refusal, then retry with ordinary limits sufficient.
- id: TC-906:line-26
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 9. For AC-12, on each of two fresh meters with work limits 10 and 100 and all other limits sufficient, admit two work units at an unrelated point, install an occurrence-1 denial at P, and request a three-work-unit `charge` at P. Compare every `Incomplete` field and the pre-refusal state across the two limits. Repeat on two fresh meters after admitting two work units, this time denying `charge_plan` at `equality.plan` with `pairs = 2`; compare its recorded reservation with the one work unit a successful plan would commit.
- id: TC-906:line-27
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 10. For AC-13, first make a selected `charge` with a semantic size greater than its configured limit and independently confirm that the ordinary charge would refuse at that size counter, with other limits sufficient. Then inject occurrence 1 at the same point and repeat the charge. Separately, after admitting two work units under a configured work limit of 3 and otherwise sufficient limits, confirm that `charge_plan` with `pairs = 2` would refuse its four-unit work reservation, then inject occurrence 1 at `equality.plan` and repeat. Compare each returned record and every consumed counter and a
- id: TC-906:line-29
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 11. For AC-14 under `test-support`, independently submit sizes 9 then 3, an absent-size charge, an explicit-zero charge and a charge with repeated same-kind requests; inspect each typed admission record and the separate high-water counter. Call `charge_plan` with pairs 2 on a sufficient meter and inspect its actual size record. On separate meters snapshot the records, counters, admission count and flag before an ordinary size refusal, ordinary work refusal, occurrence-1 injection and active cancellation, then compare those snapshots after each denial.
- id: TC-906:line-30
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 12. For AC-15, admit 4097 one-work-unit charges whose size requests are independently specified; use IntegerBits 8 at admission 4096 and 9 at admission 4097, with earlier IntegerBits requests at most 7 and sufficient limits. Inspect records and flag at both admissions and read exact accounting after the omitted record.
- id: TC-906:line-34
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 1. The over-limit charge refuses without changing the first snapshot or advancing the named occurrence. The selected valid charge returns `Incomplete` at `FunctionCall` with `WorkUnits` as limit kind and preserves the second snapshot; its retry succeeds and updates accounting and the log once.
- id: TC-906:line-35
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 2. The first plan charge returns `Incomplete` at `EqualityPlan` with the snapshot unchanged; the second succeeds and records the ordinary plan charge.
- id: TC-906:line-36
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 3. Zero cannot be represented as an `InjectedDenial::occurrence`; the nonzero value is accepted.
- id: TC-906:line-37
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 4. The log holds the first 4096 admissions in order and remains length 4096. The flag changes from false to true only after admission 4097, while the count and work units reach 4097.
- id: TC-906:line-38
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 5. Production `Meter` contains no admitted-charge log; its `Counters` accounting state remains fixed-size and heap-free.
- id: TC-906:line-40
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 6. Both cumulative counters and admissions reach 4097 while the ordered diagnostic prefix stays at 4096.
- id: TC-906:line-41
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 7. Each ordinary shortage reports its own limit kind and leaves all admitted state unchanged.
- id: TC-906:line-42
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 8. The injected refusal is atomic and spent; retry updates exact accounting once without extending the prefix.
- id: TC-906:line-43
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: '9. The ordinary-point refusals under work limits 10 and 100 are identical: `WorkUnits`, point P, `limit = consumed = 2`, and `next_charge = 3`. Both plan refusals are `WorkUnits` at `equality.plan` with `limit = consumed = 2` and `next_charge = 4` (`pairs + 2`), rather than the plan''s successful one-unit work commit. Neither configured limit appears in an injected record, and none of the refusals changes admitted state.'
- id: TC-906:line-44
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 10. The selected semantic-size charge returns its injected `WorkUnits` record rather than the otherwise first short size counter. The selected plan returns its injected `WorkUnits` record with `limit = consumed = 2` and `next_charge = 4`, rather than the ordinary work-refusal record whose `limit` would be the configured 3. No counter or admission count changes. This result does not assert precedence over cancellation.
- id: TC-906:line-46
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 11. Recorded requests remain 9 then 3 despite a counter maximum of 9; absent size, zero and repeated requests remain distinct. The admitted plan records ValueOccurrences 2 and no size entry derived from its work reservation. Each denied charge leaves all snapshotted admitted state unchanged.
- id: TC-906:line-47
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 12. Exactly the first 4096 records are retained, including IntegerBits 8 on record 4096; no record for admission 4097 is retained. The flag changes only at admission 4097, work and admissions both reach 4097 and the IntegerBits maximum becomes 9.
- id: TC-910:line-17
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 1. Compare integer operations with checked `i128` results on fixed boundary inputs and reproducibly sampled pairs only where each reference operation fits. Check the two separately specified out-of-range values against their decimal literals.
- id: TC-910:line-18
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 2. Compare rational operations with independently reduced checked `i128` fractions, including the zero-divisor stop. Compare integer and rational orderings with checked reference comparisons.
- id: TC-910:line-19
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 3. With `test-support`, inspect the admitted point order and every limit counter for one success of each listed operation. Compute size amounts from the FR's formulas and unreduced rational parts without calling the kernel's amount helpers.
- id: TC-910:line-20
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 4. Call the direct three-atom sequence in AC-7 on one meter. Inject a denial at each named point in the family rows and inspect each stopped outcome and meter state.
- id: TC-910:line-21
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 5. Check the truth table, single admitted point and work/result consumption for already-decided Boolean connectives.
- id: TC-910:line-23
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 6. For AC-13, check `RationalDomain::contains(1/3)` against the denominator-excluding domain and divide under that domain, then repeat without a domain.
- id: TC-910:line-24
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 7. Independently exercise AC-14, AC-15 and AC-16 as three direct atom schedules on separate meters, including their work-short cases.
- id: TC-910:line-25
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 8. For AC-10, independently calculate exact-bit and one-under-bit requests from the normative formulas and unreduced rational parts. Check completed values, incomplete fields and admitted prefixes.
- id: TC-910:line-26
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 9. For AC-17, use integer `5 + 7` at work limits 2 and 3, then rational `(2/3) * (3/2)` at work limits 2, 3 and 4; inspect the first unavailable point and unchanged result count. A normalize bit-only denial is not claimed when arithmetic already admitted a larger amount.
- id: TC-910:line-27
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 10. For AC-18, use independently constructed powers of two and compare exact result bit lengths with the specified charged amounts at each exponent and cancellation fixture.
- id: TC-910:line-28
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 11. For AC-11, exercise the four public dispositions and extraction behavior, checking every counter and admitted prefix.
- id: TC-910:line-29
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 12. For AC-12, check exposed numerator and denominator of `4/8`, `-4/-8`, `1/-2`, `0/5` and `0/-5`. A constructor that leaves an unreduced `4/8` fails this criterion.
- id: TC-910:line-30
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 13. For AC-19, construct fresh public meters for both bit limits of each integer call `8 + 3`, `3 + 8`, `8 - 7` and `8 * 3`, with no result bound and sufficient other limits. Independently calculate both operand bit lengths and the arithmetic request from the FR formula. Call the public integer arithmetic entry point; inspect the completed value or all `Incomplete` fields, `Meter::consumed` for integer bits, value occurrences, work and results, and the admitted point sequence under `test-support`. The reversed add detects a left-only maximum, subtraction detects charging from the one-bit resul
- id: TC-910:line-32
  path: spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '14. For AC-20, call public `evaluate_rational_arithmetic` with fresh sufficient meters and no domain for each of the six fixed calls in the criterion''s table. Independently form the unreduced numerator and denominator and count their magnitude bits with `B(0) = 1`, without calling kernel amount helpers. Inspect the actual typed admission records: require the four-point sequence, the arithmetic bound, exactly one normalize size request and its amount, then compare the completed canonical value and work/result consumption. The multiply, subtract and zero-cancellation fixtures discriminate unredu'
- id: FR-362-AC-20:fixture-64
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '| `(1/2) + (2/3)` | `7/6` | 5 | 3 | `7/6` |'
- id: FR-362-AC-20:fixture-65
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '| `(5/7) - (4/7)` | `7/49` | 7 | 6 | `1/7` |'
- id: FR-362-AC-20:fixture-66
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '| `(2/3) * (3/2)` | `6/6` | 4 | 3 | `1/1` |'
- id: FR-362-AC-20:fixture-67
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '| `(3/4) / (5/7)` | `21/20` | 6 | 5 | `21/20` |'
- id: FR-362-AC-20:fixture-68
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '| negate `-5/8` | `5/8` | 4 | 4 | `5/8` |'
- id: FR-362-AC-20:fixture-69
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '| `(1/2) - (1/2)` | `0/4` | 4 | 3 | `0/1` |'
- id: pub fn exact_size
  path: src/accounting.rs
  role: context_only
  excerpt: "pub fn exact_size(mut self, kind: LimitKind, amount: Integer) -> Self {\n        self.sizes.push((kind, amount));\n        self\n    }\n\n    /// A `work_units` addition other than the default one.\n    pub fn work(mut self, amount: Integer) -> Self {\n        self.work_units = amount;\n        self\n    }\n\n"
- id: pub fn charge(
  path: src/accounting.rs
  role: context_only
  excerpt: "pub fn charge(&mut self, mut charge: Charge) -> Result<(), Incomplete> {\n        let point = charge.point;\n        self.check_injected(point, charge.work_units.clone())?;\n        // Every semantic-size counter precedes `work_units` and `result_units`\n        // in field order.\n        charge.sizes.s"
- id: pub fn admitted_charges
  path: src/accounting.rs
  role: context_only
  excerpt: "pub fn admitted_charges(&self) -> &[ChargePoint] {\n        &self.admitted\n    }\n\n    /// Whether an admitted charge fell beyond the diagnostic log's first\n    /// 4096 entries.\n    #[cfg(feature = \"test-support\")]\n    pub fn charge_log_truncated(&self) -> bool {\n        self.charge_log_truncated\n   "
- id: ChargePoint::RationalArithmeticNormalize
  path: src/numeric.rs
  role: context_only
  excerpt: "ChargePoint::RationalArithmeticNormalize).size(\n        LimitKind::IntegerBits,\n        numerator.magnitude_bits().max(denominator.magnitude_bits()),\n    ))?;\n    let result = Rational::new(numerator, denominator)\n        .map_err(|_| Stop::Undefined(Undefined::DivisionByZero))?;\n    if let Some(dom"
- id: fn scalar_families_charge_order_and_independent_size_maxima
  path: tests/ir653_scalar.rs
  role: context_only
  excerpt: "fn scalar_families_charge_order_and_independent_size_maxima() {\n    for (left, right) in [(5_i64, 128_i64), (128, 5)] {\n        let (a, b) = (Integer::from(left), Integer::from(right));\n        let operand_max = bit_len(i128::from(left)).max(bit_len(i128::from(right)));\n        let ints = [\n        "

```

## Tool and Evidence Limits

No applicable AssuranceProfile found in spec/. Reviewer read only the diff and relevant unchanged source/tests; no runtime build/test/Cargo/Kani claims. quire changed-doc validation exit 0; grammar 4/4 clean. quire matrix --scope . --format json exit 0; quoin advise --json exit 0. Initial quoin write --types SpecReview exit 1: Error: write requires <repo_dir>; corrected current-CLI command quoin write . --types SpecReview exit 0. Initial matrix --json exit 2 (unexpected argument); current --format json exit 0. Known ambient inline-data-schema, five DuplicateArchetype and part_of DuplicateInverseEdge diagnostics retained in tool logs; no configuration edits/fallback. Exact deployment model ID is unavailable; configured Codex GPT-6, not Opus.
