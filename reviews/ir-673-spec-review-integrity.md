---
id: SR-2420
title: "Integrity review of quire-exact IR-673 spec candidate 05ffdbe"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3; origin/main...05ffdbe: spec/functional/FR-097, FR-358, FR-359, FR-361, FR-362, FR-363, FR-364, FR-365, FR-366, FR-367; spec/test-cases/TC-441, TC-906, TC-907, TC-909, TC-910, TC-911, TC-912, TC-913, TC-914, TC-915"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-097
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-358
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-359
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-361
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-362
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-363
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-364
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-365
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-366
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-367
    type: reviews
---
# Integrity review of quire-exact IR-673 spec candidate 05ffdbe

## Summary

Ticket: IR-673. No PR exists; this reviews the frozen pre-PR candidate `05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3` on base `f321f993371b3aa5226f4b36f547f1fc3dcb825e`. Reviewer model `claude-opus-5-5`, run `720faf90-5d3c-40d8-be4d-1aaae6430466`. Spec-only: no Cargo, gate, Kani or mutation run. Nothing here infers a runtime pass from planned TC text.

Method: `quire validate --scope .` on all 20 changed documents (exit 0, 20/20 grammar-clean); `quire matrix --scope .` for the changed requirements; every new or amended clause read against owner source at the reviewed SHA; the RT `TC-034` test file (`quire-contract-runtime` origin/main `75df34a`, `tests/exact_semantics.rs`) read as data and compared with the candidate.

Checked and clean:

- **`CardinalityBound::violation`.** The FR-097 contract matches the existing private method exactly (`src/collection.rs:120`): same signature, inclusive both ends, `BelowMinimum` before `AboveMaximum`. Collection admission calls that same function (`src/collection.rs:391`), so the FR-097-AC-8 agreement check is meaningful. The change only makes the method `pub`.
- **API claims.** All exist as named: `IeeeFlags::iter` and `FromIterator`, `IeeeFlag::ALL`, `RoundingMode::default() == Exact`, `DecimalType::rounding`, `Refused(InexactDecimal { target })`, `RationalOutOfDomain { target }`, `IeeeNanPayloadNotRepresentable { source, target }`, `IeeeExactLoss::NegativeZeroSign`, `IeeeValue::total_order_key`, `IeeeComparison::{NumericEqual, TotalOrder}`, `compare_text`, `IllTypedCause::DistinctTextProfiles`, `ChargePoint::{ALL, from_code}`, and `LimitKind::ALL` in `ScalarLimits` field order.
- **Fixture arithmetic.** Checked against the FR-362 Behavior formulas and the source: FR-362-AC-7 (15/5 work/results, stop at 14; `3/1 ÷ 2/1` high-water bits 3; two additions high-water bits 3), FR-361-AC-3 (division operands and arithmetic both charge `max(B(a), B(b))`), FR-361-AC-6 (lift `2^20`, digits `1 + 2^20`, prior digits 2), and FR-363-AC-3 (8/3/1 and 4/2/0, matching RT's measured values).
- **Relationships.** TC-912..TC-915 each `verifies` their new FR, and TC-441 extends its scope to AC-8.
- **TC executability.** Every procedure is executable through the public API.
- **Dropped TC-034 step.** Step 10 (`DivisionPairOutOfDomain`) is correctly not ported: that refusal no longer exists in quire-exact, where single-member division replaced it (FR-357).

Scope note for the owner, not a defect: the IR-673 description says "no API change", but FR-097 now requires a new `pub` method. Only ticket comment `8ec12bc6` brings it into scope, and that comment is untrusted ticket text.

## Verdict

**CONDITIONAL**: three medium findings and two low findings. None blocks the spec structurally. FND-001 and FND-002 need fixing before RT relies on these binders to delete its copy.

## Examined scope

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| FR-097 | examined | spec/functional/FR-097-unbounded-collection-in-the-kernel.md:11 | The kernel SHALL expose the pure count check as `pub fn violation(self, count: u64) -> Option<BoundViolation>` as an inherent method of `CardinalityBound` for callers that form their own collection values. |
| FR-097-AC-8 | examined | spec/functional/FR-097-unbounded-collection-in-the-kernel.md:28 | A caller outside the crate can call `CardinalityBound::violation(count)` ... for `[2, 4]`, collection admission agrees with its decisions at counts 1 through 5. |
| FR-358-AC-5 | examined | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:27 | ... Repeat with one result unit per admission ... After truncation, separately exhaust work and result limits and inject a named one-shot denial beyond the cap ... |
| FR-358-AC-7 | examined | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:29 | Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL` ... |
| FR-359-AC-6, FR-359-AC-7 | examined | spec/functional/FR-359-cumulative-meter-boundary.md:26 | first-unavailable counter in `ScalarLimits` field order; `Meter::consumed(kind)` total over `LimitKind::ALL` |
| FR-361-AC-3, FR-361-AC-6 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:31 | floor division exact/one-under bit limit; Multiply `(1, 0)` by `(1, 0)` retain-upscale |
| FR-362 (second SHALL), AC-3, AC-7, AC-10, AC-11, AC-12 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:15 | The kernel SHALL construct rational values with a positive denominator and a unique zero representation `0/1`. |
| FR-363-AC-1, FR-363-AC-3 | examined | spec/functional/FR-363-metered-decimal-ordering.md:35 | `(110, 2)` vs `(11, 1)` normalized equal; retained charges 8/3/1 vs 4/2/0 |
| FR-364 .. FR-367 | examined | spec/functional/FR-364-ieee-flag-iteration.md:11 | new owner FRs for flag iteration, rounding ties/default, IEEE exceptional values, text-profile IllTyped |
| TC-441, TC-906, TC-907, TC-909 .. TC-915 | examined | spec/test-cases/ | procedures, expected results, status |
| FR-357 | context_only | spec/functional/FR-357-single-member-integer-division.md:11 | The `quire-exact` kernel SHALL offer `divide` ... |
| RT TC-034 | context_only | quire-contract-runtime@75df34a tests/exact_semantics.rs | steps 1-10 kernel cases (data) |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Seven already-tagged ACs were extended in place with new obligations: FR-358-AC-5, FR-361-AC-3, FR-361-AC-6, FR-362-AC-3, FR-362-AC-7, FR-363-AC-1 and FR-363-AC-3. `quire matrix` still reports each one `tagged` by the IR-653 test, which predates the new clauses. For example, `tests/ir653_decimal_order.rs` has no `Decimal::compare(110,2 vs 11,1)`, and `tests/ir653_admission.rs` has no floor-division or Multiply-retain case. Statuses still read "Planned" or "Planned for the IR-653 CODE stage". Nothing marks the new clauses as unbacked, which is the same alias masking IR-673 exists to remove. FR-097 handled this correctly with a new AC-8 and a split status. Fix: give the new obligations new AC ids, or state per AC in Status that the existing tag covers only the original clause until it is extended. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:43 | correct-requirement-no-evidence |
| FND-002 | medium | The new FR-362 SHALL and FR-362-AC-12 require a positive denominator and zero as `0/1`, but not reduction to lowest terms. TC-910 step 9 hands the unreduced (`4/8`) and two-negative (`-4/-8`) cases of RT TC-034 step 7 to "existing owner reduction and two-negative sign tests". The only such test, `new_reduces_and_normalizes_sign` (`src/rational.rs:347`), carries no trace tag, so once RT deletes its copy no AC binds lowest-terms construction. Fix: add lowest terms to the SHALL, and add `4/8 -> 1/2` and `-4/-8 -> 1/2` to AC-12 or tag the existing test. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:48 | missing-requirement |
| FND-003 | medium | RT TC-034 step 9 (`modulo` equals the Euclidean remainder for all four sign combinations, whatever `div`/`rem` profile ran earlier) is kernel-owned: `pub fn modulo`, `src/division.rs:93`. No owner AC covers it. FR-357 covers `divide` only, and the owner test `euclidean_modulo_is_nonnegative` (`src/division.rs:420`) checks only that `-7 mod 3` is non-negative, with no exact value and no trace tag. The candidate neither ports this case nor records why it is excluded. Fix: add a `modulo` AC under FR-357 (or a new FR) with exact values for `(+-7, +-3)` and profile independence, or record why it is out of scope. | spec/functional/FR-357-single-member-integer-division.md:11 | missing-requirement |
| FND-004 | low | FR-097 is titled and scoped "unbounded collection", yet its new second SHALL defines a public inclusive count check that only matters for bounded types. The bounded-decision contract is now owned by an FR whose title readers will not search for it under. Fix: move the SHALL and AC-8 to their own FR (for example "Public inclusive cardinality check"), or retitle FR-097. | spec/functional/FR-097-unbounded-collection-in-the-kernel.md:11 | wrong-requirement |
| FND-005 | low | FR-358-AC-7, a census of the `ChargePoint` and `LimitKind` vocabularies, has no link to FR-358's statement (one-shot denial and the bounded diagnostic log). It is a separate obligation filed under an unrelated requirement. Fix: give the vocabulary census its own FR, or attach it to the FR that owns the charge vocabulary. | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:29 | wrong-requirement |

## Dispositions

Round 1, reviewed `41d35ea4741dcbe9a5df33961a9e6358c5357222` against prior `05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3`. Model `claude-opus-5-5`, run `720faf90-5d3c-40d8-be4d-1aaae6430466`, session `5600b5d7-c445-43b9-8eba-c1cdee75cf9f`. Each outcome was checked against the 05ffdbe..41d35ea diff, owner source and `quire matrix`; the author's report was not relied on. Spec-only: nothing was built or run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 41d35ea4741dcbe9a5df33961a9e6358c5357222 |
| FND-002 | fixed | 41d35ea4741dcbe9a5df33961a9e6358c5357222 |
| FND-003 | fixed | 41d35ea4741dcbe9a5df33961a9e6358c5357222 |
| FND-004 | fixed | 41d35ea4741dcbe9a5df33961a9e6358c5357222 |
| FND-005 | fixed | 41d35ea4741dcbe9a5df33961a9e6358c5357222 |

### Round 1 after-excerpts

- FND-001: FR-362-AC-7 restored to its original tagged text; added schedules moved to untagged FR-362-AC-14..16. On one meter, directly add integers 5 and 7, compare rationals 1/2 and 2/3 for less-than, then retain an already-decided Boolean `true`. The admitted atom sequence consumes 7 work units and 3 result units, derived from the three family rows in Behavior. Status: AC-1 through AC-9 have tagged pre-IR-673 tests. AC-10 through AC-18 are planned and untagged; the existing tags do not bind those additions.
- FND-002: The kernel SHALL construct rational values with a positive denominator, numerator and denominator reduced to lowest terms, and a unique zero representation `0/1`. `Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator 1 and denominator 2, proving reduction to lowest terms and cancellation of two negative signs. `Rational::new(1, -2)` exposes -1/2. For either denominator 5 or -5, a zero numerator exposes exactly 0/1.
- FND-003: The public `modulo` operation SHALL return the Euclidean remainder for nonzero divisors independently of any `DivisionProfile` selected for a separate `divide` call. For dividend/divisor pairs `(7, 3)`, `(7, -3)`, `(-7, 3)` and `(-7, -3)`, public `modulo` in the mathematical domain completes to 1, 1, 2 and 2 respectively. Running a `divide` under each of Truncating, Floor and Euclidean before each `modulo` call does not change those results; `modulo` has no profile argument.
- FND-004: FR-097 title: title: "Collection cardinality bounds and unbounded construction"
- FND-005: FR-358-AC-7 retired (ID not reused); moved to FR-368. The `quire-exact` meter SHALL expose a complete, uniquely spelled public `ChargePoint` vocabulary and a `LimitKind` vocabulary in `ScalarLimits` field order.
