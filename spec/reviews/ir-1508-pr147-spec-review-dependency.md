---
id: SR-17940486709564847047666
title: "IR-1508 PR #147 FR-403/TC-951 dependency analysis"
type: SpecReview
analysis: dependency
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #147 @ 36f59e5f on origin/main 7200065; IR-1508; spec/functional/FR-403-native-decimal-operations-refine-owner-path.md, spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md, spec/functional/FR-402-kani-tractable-native-decimal-carrier.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-951
    type: reviews
---

## Summary

Ticket: IR-1508. Spec-only PR #147 (head 36f59e5f, branch spec/ir-1508-native-decimal-ops, on origin/main 7200065) adding FR-403 and TC-951 (native Decimal operations over the FR-402 carrier) and a one-line pointer in FR-402. Reviewed against the owner source at origin/main 7200065: src/decimal.rs (evaluate, plan, Plan::expansion, arithmetic_sizes, intermediate, in_target_units, round, ExactLossValue::scaled, split_factor_five, Coefficient::lift/retain_charge/retain, reject_zero_divisor, expand_one, sbits, sdigits, Decimal::compare, compare_shifted), src/numeric.rs (order_numbers), src/integer.rs (magnitude_bits, decimal_digits, power_product_bits); FR-402, TC-950, FR-363, FR-362; Linear IR-1508. The literal AC values and charge amounts of AC-1 to AC-6 and AC-8 and the mutant inputs were recomputed with an independent Python model of the owner path (not the crate): all literal values agree with the owner. quire validate (spec/**/*.md) reports the same two FR-390 warnings as main and no new one; quire matrix adds FR-403-AC-1..12 as untagged (planned). No cargo, no Kani, no edits to the PR. Method: quoin spec-review/dependency.

## Verdict

CHANGES REQUIRED (low only). The prerequisites are named with ticket numbers: IR-1444 merged (quire-exact PR 146, FR-402, depends_on), CG FR-051 consumer with CG's ticket IR-259 and CG's credit, and CG FR-028 owning the 16 GiB ceiling. Behavior 12, AC-12 and TC step 10 record the measurement under IR-1508 only after IR-259 consumes, with no fit claimed. The Kani proofs stay unexecuted per the repository's CLAUDE.md (FR Dependencies and TC step 8). FR-402's pointer now names FR-403. Enablement (the carrier, merged) is separated from this feature. Gap: two owner FRs are missing from the relationships.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-403 refines order_numbers (Behavior 7) and the owner's membership rule (Behavior 4) but references neither FR-363 ('Decimal ordering compares values and charges retained representations', which owns the ordering charges and the no-materialization rule) nor FR-372 (fallible bounded decimal membership). The relationships cover FR-402, FR-384, FR-362, QSpec FR-140, CG FR-051 and CG FR-028. Add references to FR-363 and FR-372, and list them in Dependencies. | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:6 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-403-FM | examined | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:6 | relationships: - target: ix://agent-ix/quire-exact/FR-402 type: depends_on - target: ix://agent-ix/quire-exact/FR-384 type: references - target: ix://agent-ix/quire-exact/FR-362 type: references - target: ix://agent-ix/quire-specification/FR-140 type: references - target: ix://agent-ix/quire-contract-codegen/FR-051 type: references - target: ix://agent-ix/quire-contract-codegen/FR-028 type: references |
| FR-403-DEPS | examined | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:90 | - IR-1444 (merged, quire-exact PR 146) delivered the carrier, the native type and the constructor of [FR-402](./FR-402-kani-tractable-native-decimal-carrier.md), on which this requirement depends. FR-362 and QSpec FR-140 own the arithmetic and the charge schedule that this requirement refines. FR-384 is the precedent for admission before charge and for the typed capacity refusal. |
| FR-403-DEPS-KANI | examined | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:92 | - The seven proofs of Behavior 11 are run only when the repository's standing instruction in CLAUDE.md on the Kani runners permits it; until then they stay unexecuted. |
| FR-402-DEPS-1508 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:84 | - IR-1508 (native Decimal operations over the carrier, specified in [FR-403](./FR-403-native-decimal-operations-refine-owner-path.md)) owns the native expression that CG FR-051's prerequisite P1 also needs; this requirement is the operand half of P1 and IR-1508 depends on it. |
| FR-403-B12 | examined | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:65 | 12. When CG's decimal row (CG FR-051, ticket IR-259) consumes the delivered operations, the memory and time of the composed proof under the 16 GiB per-request ceiling of CG FR-028 SHALL be measured and recorded under IR-1508 with the peak provenance and the terminal outcome. This requirement states no memory budget and claims no fit; a prediction is not a measurement. |
| TC-951-S8 | examined | spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md:31 | 8. For AC-11, run the seven proofs `native_decimal_add_refines_owner`, `native_decimal_subtract_refines_owner`, `native_decimal_multiply_refines_owner`, `native_decimal_divide_refines_owner`, `native_decimal_negate_refines_owner`, `native_decimal_round_refines_owner` and `native_decimal_ordering_refines_owner` with no unwind attribute and their covers, and run the host test that binds each independent reference to the owner route over the boundary set. A proof is credited only on `VERIFIED` with the covers satisfied. CLAUDE.md in this repository forbids running the Kani runners until all fe... |
| TC-951-S10 | examined | spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md:33 | 10. For AC-12, once CG's decimal row (IR-259) consumes the delivered operations, run it under the authorized runner and record the peak memory and its provenance, time, tool version and terminal outcome under IR-1508. Until then this step is not executable and AC-12 stays unexecuted; no fit is claimed. |
| FR-403-B4 | context_only | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:51 | 4. For an admitted operation the kernel SHALL return the owner's typed outcome. The completed value's retained representation SHALL equal the owner's `(v x 10^T, T)`, not only its normalized value. `Undefined(DivisionByZero)` SHALL be returned for a divisor whose coefficient is zero, after the `decimal.operands` charge and before the `decimal.scale-expansion` charge. Under `exact` rounding, `Refused(InexactDecimal)` SHALL be returned, after the `decimal.arithmetic` charge and before any `decimal.rounding` charge or loss record, whenever the exact result is not an integer multiple of `10^-T`... |
| FR-403-B7 | context_only | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:57 | 7. For an ordering the kernel SHALL charge `ordering.operands` with `integer_bits` the maximum `bits` and `decimal_digits` the maximum `digits` of the two coefficients and `value_occurrences` 2; then `ordering.arithmetic` with `scale_expansion` the larger of the two alignment shifts (the difference to the larger scale), `integer_bits` the maximum of the two `sbits(c, shift)` and `decimal_digits` the maximum of the two `sdigits(c, shift)`; then `ordering.result-retain` with one result unit. An ordering SHALL order by mathematical value, not by retained representation or raw coefficient. |
