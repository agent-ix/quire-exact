---
id: SR-2026102777
title: IR-709 Exact carrier retirement gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact; IR-709; src/outcome.rs, tests/checked_invariant.rs and
  FR-369/TC-917 Status delta
review_set: subset
---

## Summary

Scoped independent review of Exact PR123 at 239a78dc67db9b434a40f27772d2f522b7ca2f62.

## Verdict

**PASS** — no findings in this retirement implementation delta.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Review evidence

Computed quire matrix output marks FR-369-AC-1, FR-369-AC-5 and FR-096-AC-8 tagged; checked_invariant.rs supplies three, three and one binders respectively. Read the assertions against the closed carrier, four original nested payloads and absence of the retired constructor. No new reverse-spec gap, stub, ignored-only binder or fabricated producer credit occurs in this scoped delta. Plan completion: not assessed. No repository-wide or downstream producer acceptance is claimed; optional semantic audit was not requested.

No applicable AssuranceProfile was found under spec/. Repository CLAUDE.md and Rust API/testing conventions were read.

The dispatching coder's executed logs were inspected: full checked_invariant integration target 3 passed; cargo fmt check passed (existing stable-format configuration warnings only); clippy --locked --all-targets -D warnings passed; focused Quire validation passed. git diff --check and computed quire matrix ran during this review. No Cargo/Kani/proof run was repeated by this reviewer, honoring the scoped cleanup speed rule; cargo-deny and broad suite reruns were not justified by unchanged dependencies/runtime paths.

The downstream QSL cfg(test) cause list is an owner handoff, not a reason to restore the deleted variant. Its code is outside this PR and ordinary CG dependency builds do not compile it.

## Examined scope

```yaml
scope:
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause`
    payload whose closed variant set and four typed `IllTypedCause` payloads equal
    the thirty-one table rows; `CheckedInvariantCause` implements the stated traits,
    and the old unit constructor does not compile.
- id: FR-369-AC-5
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()`
    return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ.
    Ordinary kernel refusal codes and causes are unchanged.
- id: FR-096-AC-8
  path: spec/functional/FR-096-kernel-refusal-code-and-cause.md
  role: examined
  excerpt: 'Each catalogued kernel refusal below returns the code and cause spelling
    named for it from `Refusal::code()` and `Refusal::cause()`, and every FR-369 `CheckedInvariant
    { cause }` returns neither. `InexactDecimal`: `inexact_decimal`, `nonzero-discarded-digit`.
    `DecimalOutOfDomain`, `ModuloOutOfDomain`, `TextLengthOutOfDomain`, `IntegerOutOfDomain`,
    `RationalOutOfDomain`, `IeeeRationalOutOfDomain`: `<name in snake case>`, `outside-domain`.
    `DivisionOutOfDomain`: `division_out_of_domain` with `quotient-outside-domain`
    for the quotient member or `remainder-outside-domain` for the remainder member.'
- id: CheckedInvariantCause retirement
  path: src/outcome.rs
  role: examined
  excerpt: "    /// Scale-zero placement retained a value other than an integer.\n\
    \    ExpectedIntegerPlacement,\n    /// A generated body received the wrong argument\
    \ count or shape.\n    GeneratedArgumentShapeMismatch,\n"
- id: retired-placeholder-negative-probe
  path: tests/checked_invariant.rs
  role: examined
  excerpt: "    let removed = compile(\n        \"removed_placeholder_cause\",\n \
    \       \"use quire_exact::CheckedInvariantCause;\\n\\\n         pub fn cause()\
    \ -> CheckedInvariantCause { CheckedInvariantCause::GeneratedBodyPlaceholderInvoked\
    \ }\",\n        false,\n    );\n    let error = String::from_utf8_lossy(&removed.stderr);\n\
    \    assert!(!removed.status.success());\n    assert!(\n        error.contains(\"\
    E0599\") && error.contains(\"GeneratedBodyPlaceholderInvoked\"),\n        \"{error}\"\
    \n    );\n\n"
```

## Coverage

Plan completion: not assessed

Scope is the changed retirement behavior only; unrelated repository criteria and external producer implementation remain outside this verdict.
