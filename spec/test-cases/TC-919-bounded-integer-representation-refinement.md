---
id: TC-919
title: "Bounded integer execution preserves the mathematical kernel contract"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-371
    type: verifies
---
# TC-919: Bounded integer execution preserves the mathematical kernel contract

## Description

This planned case checks [FR-371](../functional/FR-371-bounded-integer-representation-refinement.md) at real kernel entry points. It separates exact semantic refinement, implementation resource observations and genuine consumer proof qualification. This document does not execute or replace consumer-owned CG TC025/TC027.

## Test Procedure

1. For AC-1, construct each fixed boundary through primitive, canonical decimal and from_big routes where available. Calculate fitting Add/Subtract/Multiply/Negate results through independent checked i128 arithmetic and non-fitting results from independent decimal literals. Include large positive/negative cancellation into the native range; compare magnitude bits, zero/sign, to_u64 and canonical parsing refusal with their independently derived expected values.
2. For AC-2, compare mathematical equality, order, hash input behavior for the same hasher/target, Display and Debug across construction routes, cache access and cloning. Obtain as_big borrows and retain them during synchronized concurrent read-only accesses. Check exact values and stable retained-object identity. Join owned threads before dropping their owner; test clone/drop with permitted surviving borrows without introducing a lifetime violation as the oracle.
3. For AC-3, independently derive all operand/arithmetic bit requests and work/result totals from FR-362. Exercise native, overflow and arbitrary-size Add/Subtract/Multiply/Negate, including cancellation, at exact and one-less limits on fresh meters and on a shared cumulative meter. Inject denial at every family point. Check the actual outcome, all incomplete fields, admitted prefix and counters; test an optional domain excluding the exact result. No separate copy of the implementation's amount helper supplies the oracle.
4. For AC-4, inspect the actual native branches and use a safe external allocator observer around only standalone Integer constructors, fitting operations, bits, to_u64, native interval checks and clone. Repeat clone after source cache materialization. Keep operands/observer setup outside the window as appropriate and record each included call. Separately observe the metered facade to distinguish accounting/refusal allocations; never infer BigInt absence merely from an aggregate peak.
5. For AC-5, compile the existing public method and trait checks under default/test-support and the required no_std target in the eventual implementation lane. Exercise real IEEE consumers. If lazy materialization is selected, use synchronized owned concurrent readers and safe allocation/deallocation observations to check exact retained value, one retained winner and released temporary candidates. Record actual candidate behavior and first-access allocation; no timing assertion or once-only allocation count is the oracle.
6. For AC-6, the consumer owner selects the original genuine healthy TC027, TC027 bound mutant and TC025 arithmetic mutant against the actual reviewed/adopted kernel identity. Preflight the exact selection without execution, then use the separately allocated runner/resource envelope. Preserve original domains, oracle, unwind3, Cadical, 600s, 16GiB, completion cover and falsification witnesses. Record each actual result and cleanup separately. Run the existing all-four-operation and refinement/accounting/denial obligations; do not accept a healthy-only or split-subset verdict as whole qualification.

## Expected Results

Exact values and every public observation match the mathematical contract. Native arithmetic overflow reaches genuine arbitrary-precision execution. Meter prefixes, exact requests, counters and domain refusal remain unchanged. The bounded standalone path makes no allocation, while lazy BigInt access, if selected, exhibits its explicitly disclosed allocation behavior and stable retained borrow. Local unsafe/std leakage, lost traits, temporary borrow, cache-state equality/hash, wrapping, extra charge or unobserved discarded candidate falsifies the corresponding criterion.

Healthy proof verification and both substantive mutant falsifications are separately observed under the unchanged ceilings. Missing mutation execution, MemoryExhausted, compiler panic, timeout or unconfirmed cleanup is inconclusive. Any memory/phase observations are attributed to the actual measured run and harness; a configured ceiling or prediction is not measured peak memory.

## Status

PLANNED / UNRUN. No implementation, runtime allocation observation, test execution, Trace binding or proof result is claimed by this proposal. The kernel and consumer checks have different owners and must produce their own evidence before AC-6 or IR-705 is declared complete.
