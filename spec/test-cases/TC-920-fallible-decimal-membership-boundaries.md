---
id: TC-920
title: "Fallible Decimal membership boundaries and real scratch denial"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-372
    type: verifies
---
# TC-920: Fallible Decimal membership boundaries and real scratch denial

## Description

Verify [FR-372](../functional/FR-372-fallible-bounded-decimal-membership.md)
through the real fallible kernel operation and its actual supplied-admission
caller. Prepare types, Decimal normalization and observer/cancellation handles
before observing the helper. Do not replace membership logic, copy the Decimal
algorithm into QSV or infer an expected count from the counter under test.

## Test Procedure

1. Check interval `[0,100]` with scales `2,2` and supplied `(1,0)`;
   independently calculate the lifted coefficient 100 and exact upper equality.
   Repeat at exact lower equality with `[100,200]`, then at adjacent outside
   values, signed endpoints, zero and normalized scale beyond `smax`. Use
   all six rounding spellings and equivalent representations `(1,0)` and
   `(100,2)`, observing that retained representations remain unchanged.
2. Include a nonzero-discarded-remainder boundary such as `[0,99]`, scales
   `2,2`, `(1,0)` and a negative counterpart. For a lower-bound remainder
   control, check `[199,299]`, scales `2,2`, `(1,0)`: the divided lower
   endpoint's quotient is 1 with a nonzero remainder, but the lifted coefficient
   100 is below 199 and membership is false. Include its signed counterpart.
   Calculate the expected result independently with exact rational arithmetic
   on bounded fixtures. A mutant ignoring the remainder flag or negative sign
   reversal must fail these comparisons.
3. Independently choose admitted `B,S` and compute the FR-372 scratch and
   limb-step bounds. Include an endpoint requiring every available division
   pass, coefficient equality requiring the full comparison, native 32-bit
   limb crossings, and a shift exceeding the endpoint's decimal length.
   Observe production scratch requests and limb steps through a seam that
   does not replace helper arithmetic. Assert the formula bound, one reusable
   reservation, no scale-sized scratch and early quotient-zero termination.
4. Deny the actual fallible scratch reservation on a small otherwise-valid
   fixture, including the shifted upper-equality fixture from step 1. Compare
   its typed allocation request with the observed request, including additional
   elements, element width and layout bytes. Separately exercise checked native
   layout/capacity overflow through the production checked-layout calculation
   with boundary-sized measure inputs, without constructing an impossibly
   large Decimal; this is not an allocator-denial fixture. Remove the
   denial and assert the same completed membership as a fresh call. Observe
   scratch release and absence of a completed/partially accepted result on
   refusal. Keep unsafe allocator instrumentation outside this forbidding crate;
   use a maintained safe test seam rather than a copied allocator implementation.
5. Pre-cancel the original caller handle with each supported cause and observe
   no scratch request. At deterministic observed polling boundaries, cancel
   immediately after successful reservation and during the longest admitted
   division/comparison path. Assert the original cause, no following limb step,
   no completed result, and scratch release. A fresh-handle mutant must fail.
   Do not substitute wall-clock timing assertions for the polling oracle.
6. Through the actual supplied-admission caller, independently enumerate the
   FR-109 event order and N for a fixture with Decimal membership and another
   argument sharing an initially spent budget. Run N-1 and N. Observe denial
   before the real denied event/helper, successful cumulative spend and the
   owning phase/locus. Repeat native scratch denial, capacity refusal and
   cancellation through that caller; assert no descriptor is partially admitted,
   no helper limb debit or fifth logical event appears, and an external
   evaluator meter remains unchanged. Exercise replay's final membership
   through the same admission owner and retain conversion ownership where
   that shared operation is initiated by conversion.
7. Analyze the final production helper and maintained borrowed iterator against
   the Euclidean decomposition, remainder accumulation, carry width, sign
   reversal, checked scratch layout and per-step polling claims. Account for
   every allocation and every input-sized loop. Record requested layout bytes
   separately from unknown allocator rounding/overhead, and arithmetic work
   response separately from indivisible allocator-call elapsed time. Refuse
   qualification if any premise is absent; a small source-predicted event count
   or a bool-only helper is insufficient evidence.

## Expected Results

Membership agrees with the independent mathematical oracle, preserving all
accepted forms and inclusive boundaries. Observed helper work and scratch
requests satisfy the symbolic admitted envelope. Actual allocation denial,
checked capacity refusal and original-handle cancellation have distinct typed
causes; stopped calls release scratch and expose no accepted result. Integrated
N/N-1 controls preserve existing phase ownership, cumulative spend and event
order without evaluator debit. The qualification record establishes the
production arithmetic/storage/polling mechanism and states allocator limitations.

## Status

PLANNED/UNRUN. IR-719 owns executable implementation and qualification. The
integrated phase controls await the owning QSV/QSL interfaces; no kernel-only
test or source inspection claims QSL-503 full-size acceptance.
