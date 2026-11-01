---
id: SR-2026102769
title: "IR-1021 spec-review-integrity"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact; IR-1021; spec/functional/FR-369-typed-checked-invariant-causes.md; TC-917; src/outcome.rs rustdoc"
review_set: subset
---

## Summary

Scoped independent review of IR-1021 PR121 at6214158. FAIL. No applicable AssuranceProfile exists. Runtime code is unchanged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A blank line separates AC-10 from the Acceptance Criteria table, so the Markdown parser omits FR-369-AC-10 from the computed matrix. Remove that blank line and verify the criterion is emitted by quire matrix. | spec/functional/FR-369-typed-checked-invariant-causes.md:211 |

## Verdict

FAIL: one high structural trace defect.

## Examined scope

```yaml
scope:
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
- id: FR-369-AC-6
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Every typed cause interpreted by QSL''s reference `Machine::stopped` follows
    its existing internal-fault path: no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant`
    at S6a, `CallFailure::Fault` at that public interpreter call, and `Failed` with
    unavailable basis for an internal replay fault. Neither a received interpreter
    stop nor the generated-body carrier in AC-10 becomes an ordinary replay refusal,
    `Inconclusive`, `Incomplete(Cancelled)`, a proved result or a certification rejection;
    an unknown received cause retains fault provenance.'
- id: FR-369-AC-10
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Inspection against CG FR-021-AC-38..40 / FR-042-AC-2/3 finds the same two
    generated-function triggers, checker-issued result type and genuine owner declaration
    inputs. A proven wrong completed value or exact invalid-declaration rejection
    uses the located typed kernel payload in `Ok(Transition::Done(Evaluation))`; undecided
    QSV failures retain `BodyFailure::QsvEnvironment` / `CallFailure::Body`, never
    either checked-invariant cause or `BodyFailure::Fault`. A body or scheduler implementation
    fault retains its distinct fault channel. All nine CG cause assignments and the
    closed thirty-two-cause c
- id: CheckedInvariantCause::GeneratedUnexpectedOutcome
  path: src/outcome.rs
  role: examined
  excerpt: "A generated function's completed value is not admitted by its selected\n\
    \    /// callable's checker-issued result type: QSV `admits` returned `Ok(false)`."
- id: CheckedInvariantCause::GeneratedEnvironmentRejected
  path: src/outcome.rs
  role: examined
  excerpt: "A generated function's genuine owner-declaration environment constructor\n\
    \    /// returned exactly `Refused(InvalidDeclaration)` before body work."
- id: TC-917
  path: spec/test-cases/TC-917-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'This kernel target tests their

    carrier and non-catalog mapping only; CG FR-042 owns the real producer controls

    and the generated-body versus leaf-failure boundary inspection in FR-369-AC-10.'
- id: FR-021-AC-38
  path: agent-ix/quire-contract-codegen/spec/oracle/functional/FR-021-function-application-oracles.md
  role: context_only
  excerpt: 'With a genuine selected callable declaring Boolean result, mutate only
    the test''s copy of the emitted body source to return `Completed(Value::Integer(...))`,
    then compile and run that source through the normal registered `Body`; the unchanged
    shipped result check returns `Done(Evaluation { outcome: FamilyOutcome::Evaluated(Outcome::Refused(CheckedInvariant
    { cause: GeneratedUnexpectedOutcome })), location: Some(result occurrence), ..
    })`, preserving charges already admitted. The unmutated production body emits
    and executes the same check and completes with its admitted Boolean value. The
    mutat'
```

## Coverage

The body carrier and reference interpreter are explicitly separate boundaries: host Done retains the typed payload/location; reference Machine::stopped retains Fault mapping. This resolves the prior universal no-Evaluation wording without turning an internal failure into an ordinary refusal. Ok(false), Ok(true), QSV Err, body Fault and scheduler Fault are distinct; only the intended AC10 table structure is defective.

Parent owns runtime gates. Reviewer ran git diff --check, scoped Quire validation and computed matrix; no Cargo command or Kani proof. Matrix contains FR369 AC1/2/5/6/8 but omits the intended AC10. Existing baseline matrix gaps and unchanged historical inventory statements were not broadened into this scoped review. Gap-analysis is skipped: no production behavior changed. Spec object/risk/full failure-domain audits are not warranted by this narrow alignment.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At4cbe172 AC10 directly follows AC8 in the same table. Independent quire matrix now emits FR-369-AC-10 as method-without-symbol (Inspection). |

## Latest verdict

PASS after disposition round1 at4cbe172. Original finding and initial verdict above remain historical review evidence. No new finding; producer execution remains explicitly CG-owned and Planned.
