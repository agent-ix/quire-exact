---
id: SR-5890
title: gap-analysis of IR-734 nonzero Integer admission
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-exact@acb577ac11ecdd6afe02583f7dbb578f6be3469b (base d18927e849e8b575113fbeffb9041f5f8812d97a); FR-374, TC-922, FR-371-AC-5 against src/integer.rs, src/division.rs, src/decimal.rs, src/rational.rs, src/lib.rs, tests/nonzero_integer.rs; branch ir734-nonzero-integer; Ticket IR-734'
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-374
    type: references
  - target: ix://agent-ix/quire-exact/TC-922
    type: references
---

## Summary

Ticket: IR-734. Planless gap analysis scoped to the IR-734 diff, with test-oracle strength measured by mutation. `quire matrix --scope . --strict` at the frozen head binds FR-374-AC-1 and AC-2 to tests/nonzero_integer.rs:38, AC-3 to :64, AC-4 to :98, AC-6 to :195 and :242; AC-5 is Inspection (method-without-symbol), which this review performed: all nine production `from_proven_nonzero` sites and both metered admission sites are accounted for, each with an owning nonzero premise, with no forced clone, no new public error, no caller-input unwrap/expect and no fallback arithmetic. FR-371-AC-5's amended signatures match `src/integer.rs:421-436`. Reverse gap: every new public item (`NonZeroInteger`, `ZeroDivisor`, `NonZeroInteger::get`) and every changed caller is owned by FR-374; no stub or placeholder code was found. The 14 strict-matrix untagged criteria reported repository-wide (FR-369, FR-371-AC-6, FR-372-AC-5, FR-375, FR-380-AC-6) predate and lie outside this diff and are not findings here. Plan completion: not assessed.

Test-oracle strength: 12 of 14 mutants are killed (table in SR-5889). The two survivors are recorded below.

## Verdict

PASS with four low findings. Every Test-verified FR-374 criterion has a binder whose assertions are independent literal oracles (no host-width or dependency division computes the expected values), and the order-sensitive AC-6 controls kill every admission-reordering mutant. Gaps are confined to two unpinned clauses, matrix visibility of the compile-fail controls, and stale spec status prose.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No test pins the private canonical-invariant assert; deleting it survives every test | src/integer.rs:252-255 |
| FND-002 | low | AC-1 no-BigInt-materialization clause is unpinned; admission via as_big().is_zero() survives every test | tests/nonzero_integer.rs:38-60 |
| FND-003 | low | AC-2 compile-fail exclusion controls are doctests the matrix does not see; AC-2's only binder shows reuse, not exclusion | src/integer.rs:152-222 |
| FND-004 | low | FR-374 and TC-922 Status still say unimplemented/unrun with no Trace bindings, contradicting this branch | spec/functional/FR-374-nonzero-integer-divisor-admission.md:188 |

FND-001 failure scenario: mutant M4a removes `assert!(!value.is_zero(), ...)`; all suites pass. Impact is small because BigInt division by zero still panics, but FR-374 says a broken canonical invariant is a defect and not a fabricated value, and nothing guards that the defect surfaces at admission with the stated message. A crate-internal `#[should_panic(expected = "canonical divisor must be nonzero")]` unit test would kill it.

FND-002 failure scenario: mutant M10 makes `try_from` test `value.as_big().is_zero()`, which initializes the retained BigInt cache of an inline divisor on every admission; all tests pass. TC-922 step 1 assigns this clause to inspection (and the current code is correct), but AC-1's verification column says Test and the repo already carries the `allocation-counter` dev-dependency that could assert zero allocations during admission of an inline value.

FND-003 failure scenario: the `Trace: FR-374-AC-2` line on the struct rustdoc is not registered as a binder, so the matrix reports AC-2 covered by a runtime test that cannot fail on a public field, a public unchecked constructor or a `Default` impl; those are caught only by doctests (M1, M9) invisible to `quire matrix`.

FND-004 failure scenario: FR-374 Status reads "PROPOSED / UNIMPLEMENTED / UNRUN ... No executable Trace bindings ... accompany this specification" and TC-922 reads "PLANNED / UNRUN" after this branch adds the implementation and tagged tests, so a reader of the spec gets a false status. FR-357's status wording ("Run `quire matrix` for the current criterion-to-test mapping") is the existing alternative.

## Coverage

Commands: `quoin matrix --json` and `quire matrix --scope . --strict` (exit 1 on 14 out-of-scope untagged criteria; all FR-374 rows tagged or Inspection). Focused cargo runs and the mutation table are recorded in SR-5889. No AssuranceProfile applies; Kani and full gates not run by reviewer.

```yaml
clean: false
scope:
- {id: FR-374-AC-1, path: spec/functional/FR-374-nonzero-integer-divisor-admission.md, role: examined}
- {id: FR-374-AC-2, path: spec/functional/FR-374-nonzero-integer-divisor-admission.md, role: examined}
- {id: FR-374-AC-3, path: spec/functional/FR-374-nonzero-integer-divisor-admission.md, role: examined}
- {id: FR-374-AC-4, path: spec/functional/FR-374-nonzero-integer-divisor-admission.md, role: examined}
- {id: FR-374-AC-5, path: spec/functional/FR-374-nonzero-integer-divisor-admission.md, role: examined}
- {id: FR-374-AC-6, path: spec/functional/FR-374-nonzero-integer-divisor-admission.md, role: examined}
- {id: TC-922, path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md, role: examined}
- {id: FR-371-AC-5, path: spec/functional/FR-371-bounded-integer-representation-refinement.md, role: examined}
- {id: FR-357, path: spec/functional/FR-357-single-member-integer-division.md, role: context_only}
bindings:
- {test_id: admission_denies_zero_and_borrows_each_original_nonzero_integer, ac_id: FR-374-AC-1, trace: correct}
- {test_id: admission_denies_zero_and_borrows_each_original_nonzero_integer, ac_id: FR-374-AC-2, trace: correct}
- {test_id: admitted_pair_helpers_preserve_signed_zero_and_large_results, ac_id: FR-374-AC-3, trace: correct}
- {test_id: exact_div_keeps_infallible_signed_results_and_nondivisible_truncation, ac_id: FR-374-AC-4, trace: correct}
- {test_id: metered_zero_stops_after_operands_and_denial_precedes_zero_classification, ac_id: FR-374-AC-6, trace: correct}
- {test_id: metered_nonzero_retains_laws_single_member_domain_and_charge_requests, ac_id: FR-374-AC-6, trace: correct}
```

## Dispositions

Round 1, verified at 838d544eeb485c04af7b0e6bf25c5b0cda31baee (`git diff acb577a..838d544`).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | New crate-internal `integer::tests::canonical_nonzero_premise_rejects_zero_with_its_own_message` (`should_panic(expected = "canonical divisor must be nonzero")`); mutant M4a (assert deleted) now fails it. |
| FND-002 | fixed | New `admitting_an_inline_integer_does_not_materialize_its_bigint` measures admission of a fresh inline `2` with allocation-counter (thread-local counters, safe under parallel tests) and asserts zero allocations plus pointer identity; mutant M10 (admission via `as_big().is_zero()`) now fails it. The inline divisor is the representation where materialization allocates, so the test is not passing by representation luck. |
| FND-003 | fixed | FR-374 and TC-922 Status now state that AC-2's exclusions are checked by error-code-pinned compile_fail doctests which `quire matrix` does not bind; new tagged `admitted_token_has_no_default_construction` uses a trait-ambiguity check that fails to compile (E0283) when `Default` is added (mutant M11 verified). |
| FND-004 | fixed | FR-374 and TC-922 Status replaced with implemented-status prose that matches the strict matrix (AC-1..4 and AC-6 tagged, AC-5 inspection plus the internal premise test) and points to `quire matrix` for the live mapping. |
