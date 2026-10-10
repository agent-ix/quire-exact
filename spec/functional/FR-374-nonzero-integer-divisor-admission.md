---
id: FR-374
title: "Borrowed nonzero Integer divisor admission"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-357
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-371
    type: references
---
# FR-374: Borrowed nonzero Integer divisor admission

## Description

The `quire-exact` kernel SHALL require an admitted borrowed nonzero mathematical
`Integer` divisor at its public unmetered `Integer::exact_div`,
`Integer::div_rem_truncating` and `Integer::div_mod_floor` boundaries.

This requirement owns only those representation helpers and their admission
type. Metered evaluation remains governed by
[FR-357](./FR-357-single-member-integer-division.md). The narrow public-signature
preservation baseline for these three methods is reconciled in
[FR-371 AC-5](./FR-371-bounded-integer-representation-refinement.md#acceptance-criteria).

## Inputs

- A shared borrow of any mathematical `Integer` supplied for divisor admission.
- An `Integer` dividend and successfully admitted divisor supplied to a helper.

## Outputs

- Public admission returns `Result<NonZeroInteger<'a>, ZeroDivisor>`.
- `exact_div` returns `Integer`; the pair helpers return `(Integer, Integer)`.

## Behavior

### Borrowed admission and immutable state

The kernel SHALL export `NonZeroInteger<'a>` and the unit construction error
`ZeroDivisor` from the crate root.

The kernel SHALL expose `TryFrom<&'a Integer>` for `NonZeroInteger<'a>` as its
sole externally available construction route, with associated error
`ZeroDivisor`.

When the supplied mathematical integer is zero, admission SHALL return
`Err(ZeroDivisor)` without producing a token or executing division.

When the supplied mathematical integer is nonzero, admission SHALL return a
token borrowing that same integer for lifetime `'a`.

The token SHALL retain only a private shared `&'a Integer` referent, without
cloning or materializing a `BigInt` for admission.

The token SHALL expose `get(self) -> &'a Integer` as read-only access to that
referent.

The token SHALL implement `Clone`, `Copy`, `Debug`, `Eq` and `PartialEq` without
changing the admitted mathematical value.

The construction error SHALL follow the kernel's existing typed unit-error
convention: `Clone`, `Copy`, `Debug`, `Eq`, `PartialEq` and `thiserror::Error`.
Its type, rather than display-message parsing, identifies zero admission.

The public token API SHALL exclude writable referent access, `Default`, public
field construction and unchecked public construction.

Safe external callers cannot invalidate admission by mutating or replacing the
referent while the borrow is live, and cannot retain a token beyond the referent's
lifetime. Copying a token copies the borrow; it does not copy the mathematical
integer or grant mutation.

### Exact helper signatures and nonzero arithmetic

The kernel SHALL expose exactly these divisor parameter and return types for
the three helpers:

```rust
pub fn exact_div(&self, divisor: NonZeroInteger<'_>) -> Self;
pub fn div_rem_truncating(&self, divisor: NonZeroInteger<'_>) -> (Self, Self);
pub fn div_mod_floor(&self, divisor: NonZeroInteger<'_>) -> (Self, Self);
```

The helpers SHALL return their existing mathematical results infallibly for
every admitted nonzero divisor, including a zero dividend and either divisor
sign.

`div_rem_truncating` returns the quotient rounded toward zero and remainder
satisfying `a = b*q + r`; `div_mod_floor` returns the quotient rounded toward
negative infinity and its corresponding remainder. Both operate on arbitrary
precision mathematical integers, without host-width narrowing.

`exact_div` retains its existing quotient rounded toward zero for all nonzero
divisors. Its intended use remains division known to be exact; this requirement
does not add an exact-divisibility test, rejection or error. In particular,
`5/2` remains `2`, and `-5/2` remains `-2`. This baseline is the behavior of
the existing BigInt `/` operation in
[the helper implementation](../../src/integer.rs), rather than a stronger
interpretation of its existing exact-quotient documentation.

The kernel SHALL remove acceptance of a plain `&Integer` divisor at these three
helper signatures, without a compatibility overload or public bypass.

### Internal canonical callers and ownership

The kernel SHALL adapt its Rational, Decimal and metered division helper
callers to the admitted divisor without adding construction errors to their
existing canonical infallible public APIs.

The existing internal nonzero evidence is mathematical: Rational reduction uses
a gcd with a nonzero denominator, including numerator zero in
`divided_by_power_of_ten`; canonical Rational denominators are positive;
Decimal's ten, two and five literals, squared five-power ladder and powers of
ten are positive. These are the current caller families in
[Rational](../../src/rational.rs), [Decimal](../../src/decimal.rs) and
[metered division](../../src/division.rs).

Where a private kernel path constructs the token without public admission, the
kernel SHALL establish nonzero from its owning mathematical invariant.

The kernel SHALL exclude `unwrap` or `expect` extraction of admission on a
caller-supplied divisor.

A zero supplied at public admission is an ordinary typed construction denial.
A broken internal canonical invariant is an implementation defect, not evidence
that zero admission succeeded. This requirement does not introduce an internal
fault variant, assign an unrelated checked-invariant cause, or authorize a
fabricated quotient, remainder or substitute divisor after such a defect.

The borrowed token can be reused for Rational's paired quotient calls and
Decimal's local powers while its referent remains alive. It need not be stored
inside an owner alongside its own referent. The sole measured downstream direct
caller, QSL's literal-two `Exact::halved` implementation, remains QSL-owned;
its sealed infallible trait need not gain a fallible result to borrow admission
of its locally authored nonzero constant. This exact-owned contract does not
authorize downstream source edits or copied helper implementations.

### Metered evaluation preservation

The metered public `divide` and `modulo` signatures SHALL continue to accept
ordinary borrowed `Integer` operands.

When their operands charge admits and the divisor is zero, metered evaluation
SHALL return the existing `Undefined(DivisionByZero)` with only its existing
operands charge admitted.

When their operands charge denies, metered evaluation SHALL return its existing
`Incomplete` before classifying divisor zero.

For a nonzero divisor, metered evaluation SHALL preserve the existing laws,
exposed-member domain decision, results, refusal/incomplete fields, admitted
charge order and request amounts of
[FR-357](./FR-357-single-member-integer-division.md).

The kernel SHALL exclude divisor admission from semantic meter charges.

`ZeroDivisor` belongs to construction admission, not `Undefined` or the closed
checked-invariant cause catalog. The metered boundary maps its ordinary zero
case to the already specified evaluation outcome; it does not expose a new
construction-error outcome. There is no new arithmetic, domain or result-retain
charge for zero, and no new admission charge for any divisor.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-374-AC-1 | Admission of zero constructed by primitive, canonical parse and zero-producing arithmetic returns exactly `Err(ZeroDivisor)` and no token. Positive and negative values, including values outside i128, admit; `get` borrows the original integer, with no Integer/BigInt clone or materialization for admission. The public exports and typed unit error match the stated contract. | Test |
| FR-374-AC-2 | An external public-API caller can reuse one borrowed copyable token across all three helpers. A plain `&Integer` cannot satisfy any of the three divisor parameters. External field construction, writable access and a token outliving its referent are rejected by the public type/lifetime boundary; no Default or unchecked public constructor permits zero bypass. | Test |
| FR-374-AC-3 | Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)` return respectively truncating `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and floor `(3,1)`, `(-4,-1)`, `(-4,1)`, `(3,-1)`. A zero dividend with either divisor sign returns `(0,0)` in both helpers. Dividing `680564733841876926926749214863536422912` by either `2` or `-2` returns respectively quotient `340282366920938463463374607431768211456` or its negative and remainder zero, without narrowing. | Test |
| FR-374-AC-4 | Direct exact_div on every sign combination of dividend 6 and divisor 2 returns the signed quotient 3; zero dividend with either divisor sign returns zero. The beyond-i128 divisible literals in AC-3 return the same signed quotient. The nondivisible controls `5/2 = 2` and `-5/2 = -2` retain the prior behavior, without divisibility rejection or a fallible helper result. | Test |
| FR-374-AC-5 | Inspection accounts for every exact-owned production caller in Rational, Decimal and division. Canonical callers establish the documented nonzero invariant without forced Integer/BigInt clones, new public errors in existing canonical infallible APIs, caller-divisor admission unwrap/expect, unchecked public admission or fabricated numeric fallback. Any private token construction has an explicit owning nonzero invariant. | Inspection |
| FR-374-AC-6 | With sufficient real meters, zero divisor under all three divide profiles and both members, plus public modulo, returns `Undefined(DivisionByZero)` after exactly the operation's operands charge, with no arithmetic/domain/result-retain or admission charge. Denying operands returns the existing Incomplete first. Nonzero success and exposed-member refusal controls retain the existing laws, charge requests, ordered prefixes, counters and outcome fields; public metered signatures remain unchanged. | Test |

## Dependencies

[TC-922](../test-cases/TC-922-nonzero-integer-divisor-admission.md) specifies the
bounded direct-helper and metered regression controls. The helper admission
boundary and its Rational, Decimal and metered division callers are exact-owned;
QSL owns adoption by its literal-two caller and downstream qualification.

[FR-371 AC-5](./FR-371-bounded-integer-representation-refinement.md#acceptance-criteria)
preserves these admitted-divisor signatures during prospective representation
refinement. All other FR-371 guarantees retain their existing scope.

## Status

PROPOSED / UNIMPLEMENTED / UNRUN. This is the exact-owned IR-734 admission
contract. No executable Trace bindings, build, proof, downstream adoption or
gate result accompany this specification. Exact-divisibility strengthening,
compatibility APIs and downstream pin/lock qualification are outside this
requirement.
