---
id: SR-5603
title: integrity review of Rational-target IEEE outcome
type: SpecReview
analysis: integrity
scope: agent-ix/quire-exact@5471de5801f55372a1f49915325acfc2c489f3f1; spec/functional/FR-373-rational-target-ieee-outcome.md;
  spec/test-cases/TC-921-rational-target-ieee-outcome.md; context src/ieee.rs, src/accounting.rs,
  src/cancel.rs, src/lib.rs, src/rational.rs, FR-366, FR-358, TC-914; ticket IR-728
review_set: subset
---

## Summary

Ticket: IR-728. Single change obligation: replace the target-selector API with a Rational-only direct Outcome interface while preserving owner behavior. FR-373 -> TC-921 supplies planned verification, with FR-366/TC-914 signed-zero and FR-358 denial context. Source-context facts agree: operands -> reduced-size intermediate -> uncharged membership -> retain. Ordinary limits, injection, result denial and cancellation are distinct stops. Repository existing FR convention uses direct requirement/TC relationships; no invented US/StR artifact is required for this API slice.

## Verdict

**PASS** — No defects found within the stated method and frozen spec scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

Every changed body statement, all six acceptance criteria and the complete planned TC procedure were examined. Contiguous excerpts of long units are split at 600 characters; together they preserve the full text. Source entries denote named context inspected, not runtime evidence.

```yaml
scope:
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'When a caller converts an IEEE value to a declared Rational domain, the exact

    kernel SHALL return the conversion''s typed `Outcome<IeeeExact>` directly through

    its public `ieee_to_exact` operation.'
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: "- An `IeeeValue`, retaining its binary32 or binary64 bit pattern.\n- A borrowed, well-formed `RationalDomain`.\n- The caller's borrowed mutable `Meter`, including its already-spent counters,\n  injected denial and original shared cancellation handle when installed."
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: '`Outcome<IeeeExact>`: a completed reduced rational and optional negative-zero

    loss, typed undefined, typed refusal or the original meter''s incomplete record.

    There is no outer `Result` or cancellation variant.'
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: "1. The kernel SHALL expose and re-export\n   `ieee_to_exact(value: IeeeValue, domain: &RationalDomain, meter: &mut Meter)\n   -> Outcome<IeeeExact>` in place of the target-selector signature.\n2. The kernel SHALL remove `IeeeExactTarget` and its re-export without retaining\n   a wrapper, alias or second conversion algorithm. The existing owner conversion\n   and charging implementation remains the single authority.\n3. The kernel SHALL preserve finite conversion, negative-zero loss, non-finite\n   undefined and target-domain refusal semantics of the existing conversion.\n   [FR-366](./FR-366-ieee-exce"
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: "ptional-value-semantics.md) owns signed-zero\n   semantics. NaN and either infinity return `Undefined::IeeeNotFinite`;\n   finite nonmembers return `Refusal::IeeeRationalOutOfDomain` carrying the exact\n   declared target, without retaining the refused value.\n4. The kernel SHALL use the supplied meter in the existing order:\n   `IeeeOperands` at source width with one value occurrence; for finite values,\n   `IeeeExactIntermediate` at the reduced rational's maximum numerator or\n   denominator magnitude bit length; uncharged domain membership; then\n   `IeeeResultRetain` with one result unit. Zero's i"
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: "ntermediate size is one.\n   Each admitted named charge consumes one work unit. Non-finite values stop\n   after the operand charge; domain refusal stops after the intermediate charge.\n5. When a charge is denied, the kernel SHALL return that meter's `Incomplete`\n   unchanged without a partial result or consumption of the denied charge.\n   [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md) owns injected\n   denial behavior. When installed cancellation is observed by the meter,\n   the operation preserves its `WorkUnits` incomplete record and the original\n   shared handle's observed cause"
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: "; caller cancellation reporting remains outside\n   the kernel outcome. The operation never replaces the supplied meter or handle."
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'The direct signature cannot express a Decimal, Integer or bounded Integer target.

    [QSpec FR-148](ix://agent-ix/quire-specification/FR-148) owns the language''s

    uncharged `ill_typed` rejection of those conversions at its type boundary.

    Removing or changing the shared `IllTypedCause::IeeeToNonRationalExact` vocabulary

    is outside this requirement. QSL and RT API adoption belongs to their owning

    lanes; this requirement claims neither consumer migration nor deletion of QSL''s

    temporary defensive outer-error mapping. No numeric default changes.'
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: "- [TC-921](../test-cases/TC-921-rational-target-ieee-outcome.md) defines the\n  direct public API controls using independently specified bits, outcomes and\n  charge prefixes, rather than a retained old API as oracle.\n- Existing [TC-914](../test-cases/TC-914-ieee-exceptional-value-semantics.md)\n  signed-zero evidence remains required; adopting the direct signature does not\n  weaken its value/loss assertions."
- id: FR-373
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'SPEC DRAFT. Code changes and all runtime controls are PLANNED/UNRUN. No consumer

    migration, runtime qualification or full-gate success is claimed.'
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'Verify [FR-373](../functional/FR-373-rational-target-ieee-outcome.md) through

    the public crate-root operation and real meter. Specify expected bit patterns,

    reduced rational values and charge prefixes independently. Do not retain a

    legacy conversion API as a parity oracle or replace the conversion algorithm.'
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: "1. Inspect the public signature, re-export and conversion implementation for\n   the direct borrowed-domain interface, absence of the target enum and wrapper,\n   and one conversion/charging authority. Bind a public call's returned value\n   directly to `Outcome<IeeeExact>`; inspect that non-Rational target types\n   cannot satisfy the domain parameter. Keep downstream language rejection\n   and the shared ill-typed vocabulary outside this kernel change.\n2. Call the direct API on the two independently specified `3/2` bit patterns\n   and both zeros at each width in the FR's admitting domain. Match c"
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: "ompleted\n   values and assert numerator, denominator and exact loss. Preserve the existing\n   [TC-914](./TC-914-ieee-exceptional-value-semantics.md) binary32 signed-zero\n   assertions when its call adopts the new signature.\n3. Use binary32 quiet/signaling NaNs `0x7fc00001`, `0x7f800001` and infinities\n   `0x7f800000`, `0xff800000`; use binary64 `0x7ff8000000000001`,\n   `0x7ff0000000000001`, `0x7ff0000000000000`, `0xfff0000000000000`.\n   Assert the exact undefined variant and the one-charge prefix, counters and\n   absence of retained results. Independently tighten numerator and denominator\n   d"
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: "omains as in AC-4 and inspect exact refusal targets and two-charge prefixes.\n4. On fresh meters, assert successful finite charge order and counters. Deny\n   each next point with ordinary work limits 0, 1 and 2, then with a real\n   occurrence-1 injected denial at each named point. Compare every incomplete\n   field to the independently enumerated point and prefix. Deny final retention\n   with result limit 0 and assert work remains 2. For subnormal patterns 1 at\n   each width, set IntegerBits to 32 or 64: operands admit, but denominators\n   `2^149` and `2^1074` require 150 and 1075 bits at interm"
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: "ediate. Assert that\n   precise size refusal. Check all consumed counters and admission counts after\n   every denial, and admitted logs when test-support is enabled. Sufficient-limit\n   companion calls complete; no refusal assertion may be guarded and skipped.\n5. Attach a clone of the original caller-owned cancellation handle to the real\n   meter. For each supported cause, pre-cancel it and inspect operands denial.\n   Use its deterministic observer to cancel on the second and third charge\n   polls, then assert the exact intermediate/retain incomplete record, original\n   handle's `tripped()` cau"
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: "se, admitted prefix and zero retained results. Repeat\n   with a live handle and assert completion. These controls must fail if the\n   operation substitutes a fresh meter or cancellation handle; use no sleep or\n   elapsed-time oracle. Caller projection to its cancellation result is separate."
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'The direct API expresses only a Rational target and returns the exact completed,

    undefined, refused or incomplete variant. Values, zero loss, refusal payloads,

    ordered accounting, denial atomicity and original-handle cancellation observation

    match the independently stated controls. No outer-error mapping, copied algorithm

    or host-floating-point oracle is required.'
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'PLANNED/UNRUN. This document specifies controls; it records no executable test,

    consumer adoption or gate result.'
- id: src/ieee.rs
  path: src/ieee.rs
  role: context_only
  excerpt: ieee_to_exact; to_exact; Finite::max_part_bits; Finite::to_rational; decode; charge_operands; charge_exact_result; charge_result
- id: src/accounting.rs
  path: src/accounting.rs
  role: context_only
  excerpt: Charge::new; Meter::check_injected; Meter::charge; consumed; admission_count; admitted_charges
- id: src/cancel.rs
  path: src/cancel.rs
  role: context_only
  excerpt: Cancel::observing; Cancel::poll; Cancel::tripped; Cancel::count_charges
- id: src/lib.rs
  path: src/lib.rs
  role: context_only
  excerpt: crate-root IEEE and rational re-exports
- id: src/rational.rs
  path: src/rational.rs
  role: context_only
  excerpt: RationalDomain construction and membership; reduced rational representation
- id: FR-366
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: context_only
  excerpt: Exact rational zero has no sign; conversion records the loss of a negative IEEE zero.
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: An active cancellation is checked before injection and is outside this ordinary-limit precedence rule.
- id: TC-914
  path: spec/test-cases/TC-914-ieee-exceptional-value-semantics.md
  role: context_only
  excerpt: Convert each signed binary32 zero to the stated rational domain and inspect numerator, denominator and loss.
```

## Limitations

Spec-only review at frozen revision. Code and every runtime control, Cargo/CI/Kani/replay, and consumer adoption are UNRUN. Exact model identifier unavailable; marker model=unavailable (configured Codex reviewer). No applicable AssuranceProfile was found under this repository spec tree. Jev criterion-strength unavailable per installed skill (no client); manual falsifier scrutiny is not that lens. Gap-analysis skipped: no production diff. Object/security/architecture/app methods skipped: no domain-object, security, architecture or application-spec edits. GitHub CLI metadata lookup failed HTTP 401; repo/PR visibility PUBLIC, PR25 frozen head/base were measured by dispatching root via its connector. Local origin independently confirms repo identity. Installed tools: quoin 0.28.3, quire 0.36.2 engine 0.50.2. Scoped validation emits existing module inline-schema/duplicate-archetype advisories; no artifact validation failure. The documented quoin write invocation needs a repo argument here; quoin write . --types SpecReview succeeded.
