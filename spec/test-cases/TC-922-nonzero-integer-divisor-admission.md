---
id: TC-922
title: "Nonzero Integer admission prevents direct zero division"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-374
    type: verifies
---
# TC-922: Nonzero Integer admission prevents direct zero division

## Description

Verify [FR-374](../functional/FR-374-nonzero-integer-divisor-admission.md) through
public borrowed admission and the three genuine unmetered helpers, with bounded
metered preservation controls. The controls distinguish construction zero denial
from evaluator zero Undefined and from an internal canonical invariant defect.
Expected values and charge prefixes are independently specified; the obsolete
plain-divisor signatures are not retained as a comparison implementation.

## Test Procedure

1. Supply zero from a primitive, canonical parse and `one.sub(one)` to
   `NonZeroInteger::try_from`. Assert `Err(ZeroDivisor)` directly. Admit both
   signs of 2 and the beyond-i128 dividend literal in AC-3, asserting the
   `get` borrow identifies the original Integer. Inspect the admission path
   for clone/materialization absence and typed unit-error/public exports.
   Bind an admitted result to `NonZeroInteger<'_>` and its error to
   `ZeroDivisor`; no message parsing or caller-input unwrap is a control.
2. Use the crate-root public API to call all three helpers with one reusable
   borrowed copyable token. Add scoped negative external compilation/doc
   controls for each plain-`&Integer` helper parameter, private-field
   construction, writable referent access and escaping the referent lifetime.
   Inspect the token API for no Default or unchecked public constructor.
   These controls establish type exclusion, rather than attempting division
   on a token that public admission cannot produce.
3. Run each of the four independently enumerated signed `(7,2)` pair variants
   in FR-374 AC-3 through truncating and floor helpers. Assert exact quotient
   and remainder constants. Run zero dividends with both signs of 2 and the
   two signed-divisor beyond-i128 literal controls. Run exact_div on the four
   signs of `(6,2)`, zero dividends with both divisor signs, the two large
   divisible literal controls, `5/2` and `-5/2`. Match every literal result;
   no host-width or dependency division computes the expected oracle.
4. Inspect every exact-owned production call site in Rational, Decimal and
   division against its documented nonzero evidence. Check paired gcd use,
   positive canonical denominators, ten/two/five constants, squared powers
   and powers of ten. Identify the mathematical proof at any private token
   construction. Reject caller-divisor admission unwrap/expect, forced
   validation clones, unchecked public bypass, new construction errors in
   canonical infallible public APIs and fabricated fallback arithmetic.
   Keep QSL's literal-two caller and its pin qualification in QSL's lane.
5. On fresh sufficient real meters, call public `divide` with dividend 1
   and divisor zero under each profile and member, and `modulo` with the
   same operands. Assert exact `Undefined(DivisionByZero)`, zero retained
   results and an admission count of one. With test-support, assert exactly
   `[IntegerDivisionOperands]` or `[IntegerModulusOperands]`. Work consumed
   is 1 and IntegerBits/ValueOccurrences high-water sizes are 1/2;
   result consumption is zero. Repeat with work limit 0, then a real
   occurrence-1 operands-point denial injection: each returns the existing
   Incomplete at operands, with an empty prefix and no consumed work or
   retained result. The ordinary work denial reports limit 0, consumed 0
   and next charge 1. Inspect all incomplete fields against the unchanged
   meter contract, rather than treating a construction error as evaluation.
6. Preserve bounded nonzero regression controls from
   [TC-905](./TC-905-single-member-integer-division.md): negative-operand
   profile laws, quotient-only domain admission for `10/5` in `[1,10]`,
   quotient refusal with a permitted remainder, successful four-point
   charge order and refusal's three-point prefix. Include public modulo
   `(-7,-3) = 2` with its own four named points. For each control assert
   existing charge request amounts, exact outcome/refusal fields, consumed
   counters and ordered prefix, including absence of an admission event.
   Bind implementation tests to the FR-374 criterion they actually assert;
   existing FR-357 bindings remain separate metered-semantic evidence.

## Expected Results

Zero cannot be admitted to the public helper parameter type. Every admitted
divisor borrows its original immutable mathematical integer, and direct helper
results match the stated signed and arbitrary-size constants. `exact_div`
retains its current nondivisible truncation behavior. Canonical internal
callers preserve their public error/signature contracts and establish nonzero
without a caller-input panic or substitute arithmetic. Metered operations
preserve operands-before-zero classification, original outcomes, charge
requests and admitted prefixes.

## Status

PLANNED / UNRUN. This document specifies controls only; it records no executed
test, review verdict, build, proof, allocation benchmark, consumer adoption or
gate result. It does not alter the prospective qualification obligations of
[TC-919](./TC-919-bounded-integer-representation-refinement.md).
