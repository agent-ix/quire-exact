---
id: SR-2026102776
title: IR-709 Exact carrier retirement code-review
type: SpecReview
analysis: code-review
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

The public enum removes only the explicitly retired unit variant. The exhaustive independent matcher and 31-vector control agree with the 23 kernel plus eight generated allocation; four payload-bearing causes retain their original IllTypedCause. The removed-constructor probe uses the real dependency and requires E0599 naming the retired variant, with successful compile controls retained. No fallback, compatibility path, vendoring, new unsafe code, dependency change or gate weakening is introduced. Real trace attributes are recognized by the computed matrix.

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
- id: FR-369 status
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'The IR-1021 cause-table/rustdoc alignment specifies the generated host-body

    boundary separately from the reference interpreter''s received-payload mapping.

    It neither implements CG''s planned producer nor credits its execution or

    storage-failure evidence. IR-709 owns those controls after IR-595.

    The kernel carrier and kernel producer tests exist. `src/outcome.rs` exports

    the thirty-one-cause enum without either `CallDepthExceeded` or the retired

    placeholder variant. `tests/checked_invariant.rs` matches the complete carrier

    and rejects both removed constructors through the Rust compiler. The RT F'
- id: TC-917 status
  path: spec/test-cases/TC-917-typed-checked-invariant-causes.md
  role: examined
  excerpt: '`tests/checked_invariant.rs` exhaustively tests the thirty-one-cause carrier,

    checks that both removed constructors are rejected by the Rust compiler, and

    preserves the four typed nested payload controls. These carrier tests give no

    credit to CG''s generated producer execution. The remaining RT and external

    producer controls retain their own evidence and ownership.'
```
