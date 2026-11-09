---
id: SR-202611071713004
title: "IR-1114 Exact receipt-reference spec-review/base review"
type: SpecReview
analysis: base
review_set: base
scope: "agent-ix/quire-exact; IR-1114; PR #141; spec/functional/FR-359-cumulative-meter-boundary.md receipt-entry correction"
relationships:
  - { target: ix://agent-ix/quire-exact/FR-359, type: references }
---
## Summary

Independent spec-review/base review of Exact PR #141. The moved CG healthy-test reference is correct; no introduced or in-scope defect found.

## Assurance Context

No AssuranceProfile document is declared in this repository's spec tree. Ordinary scoped review applies; no enforced profile review, unavailable selected architecture evidence, or profile exception is claimed. The existing FR-359/FR-371 and CG consumer source are the authority.

## Verdict

**PASS** — mergeable after the lead commits these latest artifacts. This review grants no proof execution authority and claims no consumer qualification. Luna retains merge authority.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The changed table header now scopes four unqualified entries to tests/it/kani_obligations.rs and the qualified healthy entry to tests/it/kani_generation.rs. CG current source declares every entry. The ignored healthy wrapper calls verify_routed_scalar_harness_real_kani directly. Merged CG PR #601 preserves the old test body as that helper, retaining Verified, no inconclusive kind, harness path, options, unwind and missing-harness refusal assertions; launcher path, argument count, solver and successful exit checks are added. The bounded_increment_package/BOUNDED_INCREMENT/49_100/Bound::Integer(0,9)/REAL_KANI_TIMEOUT fixture definition is unchanged across that move. This is a reference correction, not a substituted native theorem.

All twelve FR-359 acceptance rows are byte-identical to origin/main, and the other four consumer names and all five expected-verdict cells are unchanged. FR-371 is untouched. Original domains, oracle, completion cover, counterexample, unwind 3, Cadical, 16 GiB and 600-second obligations remain. No new owner vocabulary, copied source/catalog, pin, tracking ledger, execution command, backend verdict, completed receipt, Trace binding or qualification claim is introduced. The descriptive move note is supported by source history.

The edited FR and the full spec/**/*.md set validate; git diff --check passes. quire matrix --strict exits 1: this repository retains open/unqualified criteria including the deliberately external FR-359-AC-10. The pointer-only diff changes neither criteria nor executable tags and cannot resolve that baseline. This scoped PASS is not a repository-wide coverage PASS. No Rust code changed, no test embeds this document, and no Cargo, Kani, CBMC or aggregate gate ran.

## Method Selection

Code-review always applies. Spec-review/base applies to the edited FR support reference. Rust review and gap-analysis are not selected: no Cargo manifest, Rust or production code changed. No requirement statement, structure, domain object or relationship edge changed, so EARS, integrity, object and dependency lenses are not applicable. The code lead selected the minimal applicable direct review; no accepted enforced workflow is represented.

## Examined Units

```json
[
  {
    "id": "FR-359-AC-10",
    "path": "spec/functional/FR-359-cumulative-meter-boundary.md",
    "role": "examined",
    "excerpt": "After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their own assertions under their original domains, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds. Record their individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test or altered consumer harness substitutes for it."
  },
  {
    "id": "FR-359",
    "path": "spec/functional/FR-359-cumulative-meter-boundary.md",
    "role": "examined",
    "excerpt": "The unqualified entries are in CG's `tests/it/kani_obligations.rs`. The healthy\nentry is in `tests/it/kani_generation.rs` and calls\n`kani_obligations::verify_routed_scalar_harness_real_kani`, which retained the\noriginal test body in [CG PR #601](https://github.com/agent-ix/quire-contract-codegen/pull/601).\nThis entry-point move changes no domain, oracle or expected verdict."
  }
]
```
