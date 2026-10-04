---
id: SR-002
title: "Gap analysis of quire-exact PR #1 (FR-088-AC-11/12, FR-089-AC-6, FR-096-AC-8, FR-097-AC-7, FR-262-AC-2)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@0992d0d198b0ef0dbd79f86849626bc0503b9b03; spec/functional/FR-088, FR-089, FR-096, FR-097, FR-262; spec/test-cases/TC-297, TC-409, TC-411, TC-428, TC-441, TC-735; the tests that carry their trace tags in src/value.rs, src/key.rs, src/equality.rs, src/identity.rs, src/outcome.rs, src/collection.rs, src/value/value_type.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-089
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-096
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-097
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-262
    type: reviews
---
# Gap analysis of quire-exact PR #1

## Summary

Ticket: IR-582. `quire coverage --scope .` reports 5/6 rows backed:
FR-088 1/2, the others 1/1. 72 of 94 Rust evidence symbols are bound.

- FR-089-AC-6: three tests, one for each clause (`admits`, `plan_pairs`,
  `compare_keys`). Backed.
- FR-097-AC-7: two TC-441 tests cover 1000 elements unbounded, `[0, 1]`
  refused `AboveMaximum`, `Incomplete` at `collection.bound` under 999
  occurrences, and the unbounded type differing from `[0, u64::MAX]`. Backed.
- FR-088-AC-12: `tc_411_...` covers different domains over the same bytes,
  one domain over different bytes, and both labels. Backed, apart from
  FND-005.
- FR-262-AC-2: the `ValueType` half is backed on a 512 KiB thread. The
  `Value` half is not (FND-002).
- FR-096-AC-8: every code and cause is asserted, but not the AC's named
  targets (FND-003).
- FR-088-AC-11: unbacked (FND-001).

Every `#[trace]` id resolves, to a local spec row or to a `QSpec-`
prefixed agent-ix/quire-specification id. The QSL tests that carry these
TC ids outside the crate (qsl-eval, qsl-foundation, qsl-semantics,
tests/it) test the caller's half and import `qsl_*` crates. They rightly
stay in QSL.

## Verdict

Changes requested, no high. FND-001 to FND-003 are ACs that claim more
than this repo's tests check. Fix each one by adding the kernel-level test,
or by narrowing the AC to what is tested. FND-004 and FND-005 are low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-088-AC-11 is unbacked (`quire coverage`: FR-088 1/2). The four TC-409 tests check rank lookup, wrong-rank refusal and that `compare_keys` gives `None` for two variants at the same rank. No test forms a set and checks the visiting order of either enum. No test checks that `compare_keys` orders two members of one shape by rank. No test checks the two-shapes clause. The set-order tests stayed in QSL. The tags name TC-409 only, and two doc comments cite "TC-409 step 5" and "step 3", which do not match this TC's four steps. | src/value.rs:1285-1333; src/key.rs:224-238; spec/test-cases/TC-409-enum-value-identity-and-rank-key.md |
| FND-002 | medium | FR-262-AC-2's `Value` half has no test. No TC-735-tagged test builds a deep `Value`. The deep-`Value` tests in value.rs are untagged and run on a 2 MiB stack, not 512 KiB. Their value is a composite, sequence and option chain, not "a `Value` of the same depth". TC-735 step 2 has no test behind it. | src/value.rs:1840-1967; src/value/value_type.rs:300-310,458-530; spec/test-cases/TC-735-deep-values-and-value-types-evaluate-key-and-drop.md |
| FND-003 | medium | FR-096-AC-8 names targets that TC-428's test does not use. The AC names `Int[-5, 9]`, `Decimal[-100, 100; 0, 2]`, `Rational[-9, 9; 1, 9]`, `Int[0, 9]` and a binary32 `IeeeNotExact` with inexact and overflow flags. The test uses `Int[0, 1]`, `Decimal[0, 1; 0, 0]`, a `[0,1]/[1,1]` rational and `IeeeFlags::EMPTY`. It never asserts that a variant carries its target, so a refusal that drops its target still passes. | src/outcome.rs:357-500; spec/functional/FR-096-kernel-refusal-code-and-cause.md:26 |
| FND-004 | low | About 57 tests are named `tc_300_...` to `tc_356_...`, and `quire coverage` reads each name as a trace to TC-300 to TC-356. Those ids exist neither here nor in QSL's spec, so 74 traces match no row. The `QSpec-` ones are external and expected. This came from QSL. Renaming is a follow-up, not a reason to break src identity in this PR. | src/node.rs; src/numeric.rs; src/outcome.rs; src/quantity.rs; src/rational.rs; src/text.rs; src/value.rs; src/collection.rs; src/accounting.rs |
| FND-005 | low | FR-088-AC-12 names a "scaled declared unit `UnitId`", which the kernel has no notion of. A scaled unit is just another declared node key. TC-411 builds no such id. Fix: drop the phrase. | spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md:27; src/identity.rs:380-399 |
