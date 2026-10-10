---
id: SR-4887
title: "IR-678 Requirement-to-test trace and PR-scope reverse gaps"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@6757d4392cb753462fe0cb64d7e599a3d613cbdc; tests/ir653_scalar.rs, spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md, spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md"
review_set: subset
---

## Summary

Reviewed the frozen PR #16 diff for IR-678 against FR-362-AC-19 and TC-910 Step 13. The changed file is `tests/ir653_scalar.rs`; no production or workflow file changed.

## Verdict

**PASS** — The computed matrix binds FR-362-AC-19 to the added test; test behavior matches the criterion, and this test-only diff introduces no new unspecified production behavior.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- Examined FR-362-AC-19: Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add `8 + 3` and `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`, without a result bound and with all non-bit limits sufficient. The independently calculated `integer-arithmetic.arithmetic` bit amounts are respectively 5, 5, 5 and 6: add/subtract use `max(B(a), B(c)) + 1`, including the larger right operand and a result that cancels to one bit; multiply uses `B(a) + B(c)`. At each exact `integer_bits` limit, the call completes to respectively 11, 11, 1 and 24, admits operands, arithmetic and result-retain in 
- Examined TC-910 Step 13: 13. For AC-19, construct fresh public meters for both bit limits of each integer call `8 + 3`, `3 + 8`, `8 - 7` and `8 * 3`, with no result bound and sufficient other limits. Independently calculate both operand bit lengths and the arithmetic request from the FR formula. Call the public integer arithmetic entry point; inspect the completed value or all `Incomplete` fields, `Meter::consumed` for integer bits, value occurrences, work and results, and the admitted point sequence under `test-support`. The reversed add detects a left-only maximum, subtraction detects charging from the one-bit resul
- Examined TC-910 Expected Results: AC-19 exact-limit calls complete with respective results 11, 11, 1 and 24 and bit consumption 5, 5, 5 and 6; one-under calls stop at the arithmetic point with requests 5, 5, 5 and 6 after admitting only operands.
- Binding: `public_integer_arithmetic_admits_exact_bits_and_denies_one_under` → FR-362-AC-19; correct.
- Scoped tests: `cargo test --test ir653_scalar public_integer_arithmetic_admits_exact_bits_and_denies_one_under` passed with default features and with `--features test-support`.
- Computed matrix: FR-362-AC-19 tagged to `tests/ir653_scalar.rs:35`; FR-362-AC-10 remains untagged and is outside IR-678 scope.
- Reverse gap: no production code changed.
- Plan completion: not assessed
