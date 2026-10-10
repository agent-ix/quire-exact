---
id: TC-921
title: "Direct Rational IEEE outcomes preserve charge and stop boundaries"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-373
    type: verifies
---
# TC-921: Direct Rational IEEE outcomes preserve charge and stop boundaries

## Description

Verify [FR-373](../functional/FR-373-rational-target-ieee-outcome.md) through
the public crate-root operation and real meter. Specify expected bit patterns,
reduced rational values and charge prefixes independently. Do not retain a
legacy conversion API as a parity oracle or replace the conversion algorithm.

## Test Procedure

1. Inspect the public signature, re-export and conversion implementation for
   the direct borrowed-domain interface, absence of the target enum and wrapper,
   and one conversion/charging authority. Bind a public call's returned value
   directly to `Outcome<IeeeExact>`; inspect that non-Rational target types
   cannot satisfy the domain parameter. Keep downstream language rejection
   and the shared ill-typed vocabulary outside this kernel change.
2. Call the direct API on the two independently specified `3/2` bit patterns
   and both zeros at each width in the FR's admitting domain. Match completed
   values and assert numerator, denominator and exact loss. Preserve the existing
   [TC-914](./TC-914-ieee-exceptional-value-semantics.md) binary32 signed-zero
   assertions when its call adopts the new signature.
3. Use binary32 quiet/signaling NaNs `0x7fc00001`, `0x7f800001` and infinities
   `0x7f800000`, `0xff800000`; use binary64 `0x7ff8000000000001`,
   `0x7ff0000000000001`, `0x7ff0000000000000`, `0xfff0000000000000`.
   Assert the exact undefined variant and the one-charge prefix, counters and
   absence of retained results. Independently tighten numerator and denominator
   domains as in AC-4 and inspect exact refusal targets and two-charge prefixes.
4. On fresh meters, assert successful finite charge order and counters. Deny
   each next point with ordinary work limits 0, 1 and 2, then with a real
   occurrence-1 injected denial at each named point. Compare every incomplete
   field to the independently enumerated point and prefix. Deny final retention
   with result limit 0 and assert work remains 2. For subnormal patterns 1 at
   each width, set IntegerBits to 32 or 64: operands admit, but denominators
   `2^149` and `2^1074` require 150 and 1075 bits at intermediate. Assert that
   precise size refusal. Check all consumed counters and admission counts after
   every denial, and admitted logs when test-support is enabled. Sufficient-limit
   companion calls complete; no refusal assertion may be guarded and skipped.
5. Attach a clone of the original caller-owned cancellation handle to the real
   meter. For each supported cause, pre-cancel it and inspect operands denial.
   Use its deterministic observer to cancel on the second and third charge
   polls, then assert the exact intermediate/retain incomplete record, original
   handle's `tripped()` cause, admitted prefix and zero retained results. Repeat
   with a live handle and assert completion. These controls must fail if the
   operation substitutes a fresh meter or cancellation handle; use no sleep or
   elapsed-time oracle. Caller projection to its cancellation result is separate.
6. Prepare an independently specified already-spent prefix through one real
   `Charge` at `FunctionCall`: work 2, result 1, IntegerBits 80,
   ValueOccurrences 5 and TextScalars 7; all other counters remain zero.
   Check these constants before conversion, rather than deriving the oracle
   from measured counters. For each width's `3/2`, a sufficient-limit call
   must end at work 5, results 2 and four admissions, with the three size
   high-water values unchanged and, under test-support, exactly the log
   `[FunctionCall, IeeeOperands, IeeeExactIntermediate, IeeeResultRetain]`.
   On separately precharged meters, set work limit 4 or result limit 1, install
   an occurrence-1 `IeeeResultRetain` injection, or attach the original shared
   cancellation handle and cancel at the third conversion poll with each cause.
   Attach that observer after precharging so its poll positions are unambiguous.
   Assert the exact AC-7 records, work 4, results 1, three admissions and log
   `[FunctionCall, IeeeOperands, IeeeExactIntermediate]`. All other counters
   equal their independent prefix constants. Check the supplied limits remain
   exactly configured; injected denial is spent according to the existing meter
   contract and cancellation trips the original handle. These success/shortage
   controls must fail a mutant that resets already-spent counters or refunds
   the prefix while retaining the limits, injected denial and cancellation
   handle. No denied retain work or result unit is consumed.

## Expected Results

The direct API expresses only a Rational target and returns the exact completed,
undefined, refused or incomplete variant. Values, zero loss, refusal payloads,
ordered accounting, denial atomicity and original-handle cancellation observation
match the independently stated controls. No outer-error mapping, copied algorithm
or host-floating-point oracle is required.

## Status

PLANNED/UNRUN. This document specifies controls; it records no executable test,
consumer adoption or gate result.
