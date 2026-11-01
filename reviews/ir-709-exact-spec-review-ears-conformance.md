---
id: SR-2026102775
title: "IR-709 Exact retirement spec-review-ears-conformance"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact; IR-709; FR-369 and TC-917 retirement delta"
review_set: subset
---

## Summary

Scoped independent review of IR-709 Exact PR122 at 4a99f24e7a063abf005d3e14d11726cdd82772e0. PASS. Markdown-only retirement of a producer removed by IR-595; no implementation acceptance is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS: no findings in this scoped method.

## Examined scope

```yaml
scope:
- id: FR-369 carrier
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'The public `CheckedInvariantCause` SHALL be a closed enum of the thirty-one
    variants in the two tables. `ScheduledComparisonRefused`, `EqualityQuantityConversionRejected`,
    `GeneratedTypeCheckRejected` and `GeneratedEqualityCheckRejected` each SHALL carry
    `{ cause: IllTypedCause }`; the other twenty-seven variants are unit variants.
    The existing typed refusal returned by a scheduled comparator, an equality quantity
    conversion, or CG''s semantic-value type/equality check SHALL be retained in that
    payload without string conversion. `IllTypedCause` is already owned and exported
    by `quire-exact::com'
- id: FR-369 allocation status
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'The following eight causes specify CG''s generated-oracle assignments.

    Six rows have current typed checked-invariant constructor sites at CG main

    `f935378e56cde1e7aaabe3d073669d9e58085bde`. The two result-membership and

    owner-declaration construction rows remain planned under IR-709. These causes

    name shared checked-oracle failures, not CG package or model types.


    `GeneratedBodyPlaceholderInvoked` is retired. IR-595 replaced the pre-check

    placeholder with concrete registered bodies; no live trigger remains. The

    carrier SHALL omit this retired variant, with no replacement or reallocation.

    CG SHA'
- id: FR-369 retirement
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: '`GeneratedBodyPlaceholderInvoked` is retired. IR-595 replaced the pre-check

    placeholder with concrete registered bodies; no live trigger remains. The

    carrier SHALL omit this retired variant, with no replacement or reallocation.

    CG SHALL NOT restore a placeholder solely to produce a cause. An unexpected

    body resume remains a genuine `InternalFault`, never this retired cause or an

    ordinary refusal. This retirement changes the generated allocation from nine

    to eight causes and the complete carrier from thirty-two to thirty-one; the

    other cause identities, payloads and triggers are unchanged.


    '
- id: FR-369 GeneratedArgumentShapeMismatch
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'A generated scalar or equality function body receives the wrong number
    or shape of arguments: `src/oracle/function/owner_render.rs`, `scalar_item`/`equality_item`.'
- id: FR-369 GeneratedOperandKindUnsupported
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'A generated equality body receives a left operand outside its classified
    Boolean/Integer kinds: `src/oracle/function/owner_render.rs`, `equality_item`.'
- id: FR-369 GeneratedUnexpectedOutcome
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: A generated function body produces `Completed(value)` and the admitted
    QSV environment's `admits` returns exactly `Ok(false)` for the selected callable's
    checker-issued `CallableSelection::signature().result()`. Emit the typed checked-invariant
    payload at the generated result occurrence. `Ok(true)` retains the original completed
    value; `Err(EnvironmentFailure)` decided no membership and is not this cause.
    `Undefined`, `Refused` and `Incomplete` keep their original arms, payloads, locations
    and admitted charges. This result-membership producer and its test-only wrong-completed-value
    mutation ar
- id: FR-369 GeneratedIntervalInvalid
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'A generated native integer bound cannot form a nonempty interval: `src/oracle/boolean_v1.rs`,
    `NATIVE_BOUND_HELPER`.'
- id: FR-369 GeneratedEnvironmentRejected
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Generated function preparation constructs the QSV environment from the
    selected checked-closure owner's actual nominal declarations and receives exactly
    `EnvironmentFailure::Refused(InvalidDeclaration)`. Emit this typed cause at the
    declaration-construction occurrence before body operations or body charges, retaining
    the original rejection in diagnostic evidence and any previously admitted charge
    prefix. Healthy production submits each authentic declaration once; its test-only
    emitted-source mutation submits one unchanged owner declaration twice to the real
    constructor. Default/empty environme
- id: FR-369 GeneratedTypeCheckRejected { cause }
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'A generated equality oracle''s declared comparison type fails shared semantic-value
    `check_type`; retain its kernel-owned `IllTypedCause`: `src/oracle/equality/mod.rs`,
    `oracle_{symbol}` type checks.'
- id: FR-369 GeneratedEqualityCheckRejected { cause }
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'A generated equality body or oracle fails shared semantic-value `check_equality`;
    retain its kernel-owned `IllTypedCause`: `src/oracle/equality/mod.rs`, `oracle_{symbol}`;
    `src/oracle/function/owner_render.rs`, `equality_item`. The Boolean `NATIVE_EQUALITY_HELPER`
    uses planned kernel equality and carries its stop unchanged.'
- id: FR-369 GeneratedDescriptorReconstructionFailed
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'A generated equality oracle receives a prepared descriptor with the wrong
    binding, expected package identity or context requirement, or an admitted context
    with the wrong package identity: `src/oracle/equality/mod.rs`, `oracle_{symbol}`
    prepared-environment check. Descriptor reconstruction errors while preparing that
    environment retain their separate typed `EnvironmentError` path.'
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause`
    payload whose closed variant set and four typed `IllTypedCause` payloads equal
    the thirty-one table rows; `CheckedInvariantCause` implements the stated traits,
    and the old unit constructor does not compile.
- id: FR-369-AC-8
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public carrier contains no QSL model/checker/catalog type or dependency
    on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and
    an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant
    constructor assigned exactly one cause from the two tables.
- id: FR-369-AC-10
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Inspection against CG FR-021-AC-38..40 / FR-042-AC-2/3 finds the same
    two generated-function triggers, checker-issued result type and genuine owner
    declaration inputs. A proven wrong completed value or exact invalid-declaration
    rejection uses the located typed kernel payload in `Ok(Transition::Done(Evaluation))`;
    undecided QSV failures retain `BodyFailure::QsvEnvironment` / `CallFailure::Body`,
    never either checked-invariant cause or `BodyFailure::Fault`. A body or scheduler
    implementation fault retains its distinct fault channel. All eight CG cause assignments
    and the closed thirty-one-cause '
- id: FR-369 Status
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'The IR-1021 cause-table/rustdoc alignment specifies the generated host-body

    boundary separately from the reference interpreter''s received-payload mapping.

    It neither implements CG''s planned producer nor credits its execution or

    storage-failure evidence. IR-709 owns those controls after IR-595.

    The kernel carrier and kernel producer tests exist. `src/outcome.rs` exports

    the pre-retirement thirty-two-cause enum without `CallDepthExceeded`.

    Removing the retired placeholder variant and updating the exhaustive

    `tests/checked_invariant.rs` controls remain implementation work under IR-709;

    this speci'
- id: TC-917 procedure
  path: spec/test-cases/TC-917-typed-checked-invariant-causes.md
  role: examined
  excerpt: "1. Exhaustively match `CheckedInvariantCause` against thirty-one independently\n\
    \   named expectations, including `ScheduledComparisonRefused`,\n   `EqualityQuantityConversionRejected`,\
    \ `GeneratedTypeCheckRejected` and\n   `GeneratedEqualityCheckRejected` with\n\
    \   concrete kernel-owned `IllTypedCause` payloads. Reject a\n   `CallDepthExceeded`\
    \ variant: RT FR-273's final explicit-frame scheduler\n   has no depth-specific\
    \ outcome, and denied `work_units` is `Incomplete`.\n   Reject the retired `GeneratedBodyPlaceholderInvoked`\
    \ variant: IR-595 removed\n   its owning placeholder, and no other trigger inher"
- id: TC-917 status
  path: spec/test-cases/TC-917-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'The target is the thirty-one-cause carrier after retirement of

    `GeneratedBodyPlaceholderInvoked`. The current carrier and exhaustive test

    still include that variant; their removal is implementation work under IR-709,

    not acceptance supplied by this specification-only correction. The remaining

    kernel carrier controls and the RT explicit-frame/work-fuel cutover and external

    producer controls retain their own evidence and ownership.

    '
```

## Coverage

Scoped Quire validation reported 2/2 documents grammar-clean and zero grammar findings. New ubiquitous retirement obligations name the carrier/CG actors and measurable responses (omit the retired variant; do not restore the placeholder); no failure is remapped and no vague operational response is introduced.

No applicable AssuranceProfile exists under the repository spec tree. No cargo, Kani or mutation commands were run. Scope validation and git diff --check pass. Rust review and gap-analysis are inapplicable because no Rust or production implementation changes. Full risk/object/dependency/integrity audits are outside this focused cause-retirement delta.

## Latest verdict

PASS after disposition round 1 at 4e7e9a4991b7c3e039212d049d18f1d52cff15b0. The only delta removes the embedded CG revision. Original findings and initial scope remain historical evidence; no new finding. Other reviewed retirement, count and ownership obligations are unchanged.
