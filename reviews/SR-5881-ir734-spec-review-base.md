---
id: SR-5881
title: spec-review/base review of IR-734
type: SpecReview
analysis: base
scope: 'agent-ix/quire-exact@d1a085c1778517b6b52ee87abe8473f8768f60bc; spec/functional/FR-374-nonzero-integer-divisor-admission.md,
  spec/test-cases/TC-922-nonzero-integer-divisor-admission.md, spec/functional/FR-371-bounded-integer-representation-refinement.md;
  source context: src/integer.rs, src/rational.rs, src/decimal.rs, src/division.rs,
  FR-357; IR-734'
review_set: subset
---

## Summary

Ticket: IR-734. Base checklist and scoped structure/link validation applied. Planned TC-922 covers the six new criteria; matrix currently reports five untagged and one method-without-symbol, and FR-371-AC-5 untagged. These are truthful implementation-stage obligations, not conformance claimed by this spec-only PR.

## Verdict

**FAIL** — Defects require dispositions before handoff.

## Review Provenance

Exact runtime model ID unavailable in harness; model=unavailable. Run a15dfc8c-200a-4457-9e6d-c90b1e5b1928. No applicable AssuranceProfile found. Reviewed frozen head only; no Cargo/Kani/build/proof/adoption evidence claimed. Quire 0.36.2 (engine 0.50.2), Quoin 0.28.3. Installed inline-schema/duplicate-archetype/inverse-edge warnings were emitted; changed-doc validation exited 0. GitHub connector independently verified public repository 1404605101 and PR head/base. gh CLI 401 left untouched.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-374 records exact and QSL baseline commit SHAs in normative specification prose. The base checklist prohibits recorded revisions and SHA/pin records; remove both literals and retain the owning source links and behavioral contract. | spec/functional/FR-374-nonzero-integer-divisor-admission.md:176 |

## Examined Scope

```yaml
scope:
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "---\nid: FR-374\ntitle: \"Borrowed nonzero Integer divisor admission\"\
    \ntype: FR\norg: agent-ix\nrelationships:\n  - target: ix://agent-ix/quire-exact/FR-357\n\
    \    type: depends_on\n---\n# FR-374: Borrowed nonzero Integer divisor admission"
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The `quire-exact` kernel SHALL require an admitted borrowed nonzero mathematical

    `Integer` divisor at its public unmetered `Integer::exact_div`,

    `Integer::div_rem_truncating` and `Integer::div_mod_floor` boundaries.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'This requirement owns only those representation helpers and their admission

    type. Metered evaluation remains governed by

    [FR-357](./FR-357-single-member-integer-division.md). The narrow public-signature

    preservation baseline for these three methods is reconciled in

    [FR-371 AC-5](./FR-371-bounded-integer-representation-refinement.md#acceptance-criteria).'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '- A shared borrow of any mathematical `Integer` supplied for divisor admission.

    - An `Integer` dividend and successfully admitted divisor supplied to a helper.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '- Public admission returns `Result<NonZeroInteger<''a>, ZeroDivisor>`.

    - `exact_div` returns `Integer`; the pair helpers return `(Integer, Integer)`.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The kernel SHALL export `NonZeroInteger<''a>` and the unit construction
    error

    `ZeroDivisor` from the crate root.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The kernel SHALL expose `TryFrom<&''a Integer>` for `NonZeroInteger<''a>`
    as its

    sole externally available construction route, with associated error

    `ZeroDivisor`.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'When the supplied mathematical integer is zero, admission SHALL return

    `Err(ZeroDivisor)` without producing a token or executing division.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'When the supplied mathematical integer is nonzero, admission SHALL return
    a

    token borrowing that same integer for lifetime `''a`.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The token SHALL retain only a private shared `&''a Integer` referent,
    without

    cloning or materializing a `BigInt` for admission.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The token SHALL expose `get(self) -> &''a Integer` as read-only access
    to that

    referent.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The token SHALL implement `Clone`, `Copy`, `Debug`, `Eq` and `PartialEq`
    without

    changing the admitted mathematical value.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The construction error SHALL follow the kernel''s existing typed unit-error

    convention: `Clone`, `Copy`, `Debug`, `Eq`, `PartialEq` and `thiserror::Error`.

    Its type, rather than display-message parsing, identifies zero admission.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The public token API SHALL exclude writable referent access, `Default`,
    public

    field construction and unchecked public construction.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'Safe external callers cannot invalidate admission by mutating or replacing
    the

    referent while the borrow is live, and cannot retain a token beyond the referent''s

    lifetime. Copying a token copies the borrow; it does not copy the mathematical

    integer or grant mutation.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The kernel SHALL expose exactly these divisor parameter and return types
    for

    the three helpers:'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '```rust

    pub fn exact_div(&self, divisor: NonZeroInteger<''_>) -> Self;

    pub fn div_rem_truncating(&self, divisor: NonZeroInteger<''_>) -> (Self, Self);

    pub fn div_mod_floor(&self, divisor: NonZeroInteger<''_>) -> (Self, Self);

    ```'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The helpers SHALL return their existing mathematical results infallibly
    for

    every admitted nonzero divisor, including a zero dividend and either divisor

    sign.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '`div_rem_truncating` returns the quotient rounded toward zero and remainder

    satisfying `a = b*q + r`; `div_mod_floor` returns the quotient rounded toward

    negative infinity and its corresponding remainder. Both operate on arbitrary

    precision mathematical integers, without host-width narrowing.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '`exact_div` retains its existing quotient rounded toward zero for all
    nonzero

    divisors. Its intended use remains division known to be exact; this requirement

    does not add an exact-divisibility test, rejection or error. In particular,

    `5/2` remains `2`, and `-5/2` remains `-2`. This baseline is the behavior of

    the existing BigInt `/` operation in

    [the helper implementation](../../src/integer.rs), rather than a stronger

    interpretation of its existing exact-quotient documentation.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The kernel SHALL remove acceptance of a plain `&Integer` divisor at these
    three

    helper signatures, without a compatibility overload or public bypass.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The kernel SHALL adapt its Rational, Decimal and metered division helper

    callers to the admitted divisor without adding construction errors to their

    existing canonical infallible public APIs.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The existing internal nonzero evidence is mathematical: Rational reduction
    uses

    a gcd with a nonzero denominator, including numerator zero in

    `divided_by_power_of_ten`; canonical Rational denominators are positive;

    Decimal''s ten, two and five literals, squared five-power ladder and powers of

    ten are positive. These are the current caller families in

    [Rational](../../src/rational.rs), [Decimal](../../src/decimal.rs) and

    [metered division](../../src/division.rs).'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'Where a private kernel path constructs the token without public admission,
    the

    kernel SHALL establish nonzero from its owning mathematical invariant.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The kernel SHALL exclude `unwrap` or `expect` extraction of admission
    on a

    caller-supplied divisor.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'A zero supplied at public admission is an ordinary typed construction
    denial.

    A broken internal canonical invariant is an implementation defect, not evidence

    that zero admission succeeded. This requirement does not introduce an internal

    fault variant, assign an unrelated checked-invariant cause, or authorize a

    fabricated quotient, remainder or substitute divisor after such a defect.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The borrowed token can be reused for Rational''s paired quotient calls
    and

    Decimal''s local powers while its referent remains alive. It need not be stored

    inside an owner alongside its own referent. The sole measured downstream direct

    caller, QSL''s literal-two `Exact::halved` implementation, remains QSL-owned;

    its sealed infallible trait need not gain a fallible result to borrow admission

    of its locally authored nonzero constant. This exact-owned contract does not

    authorize downstream source edits or copied helper implementations.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'The metered public `divide` and `modulo` signatures SHALL continue to
    accept

    ordinary borrowed `Integer` operands.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'When their operands charge admits and the divisor is zero, metered evaluation

    SHALL return the existing `Undefined(DivisionByZero)` with only its existing

    operands charge admitted.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'When their operands charge denies, metered evaluation SHALL return its
    existing

    `Incomplete` before classifying divisor zero.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'For a nonzero divisor, metered evaluation SHALL preserve the existing
    laws,

    exposed-member domain decision, results, refusal/incomplete fields, admitted

    charge order and request amounts of

    [FR-357](./FR-357-single-member-integer-division.md).'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: The kernel SHALL exclude divisor admission from semantic meter charges.
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '`ZeroDivisor` belongs to construction admission, not `Undefined` or the
    closed

    checked-invariant cause catalog. The metered boundary maps its ordinary zero

    case to the already specified evaluation outcome; it does not expose a new

    construction-error outcome. There is no new arithmetic, domain or result-retain

    charge for zero, and no new admission charge for any divisor.'
- id: FR-374-AC-1
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Admission of zero constructed by primitive, canonical parse and zero-producing
    arithmetic returns exactly `Err(ZeroDivisor)` and no token. Positive and negative
    values, including values outside i128, admit; `get` borrows the original integer,
    with no Integer/BigInt clone or materialization for admission. The public exports
    and typed unit error match the stated contract.
- id: FR-374-AC-2
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: An external public-API caller can reuse one borrowed copyable token across
    all three helpers. A plain `&Integer` cannot satisfy any of the three divisor
    parameters. External field construction, writable access and a token outliving
    its referent are rejected by the public type/lifetime boundary; no Default or
    unchecked public constructor permits zero bypass.
- id: FR-374-AC-3
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)`
    return respectively truncating `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and floor
    `(3,1)`, `(-4,-1)`, `(-4,1)`, `(3,-1)`. A zero dividend with either divisor sign
    returns `(0,0)` in both helpers. Dividing `680564733841876926926749214863536422912`
    by either `2` or `-2` returns respectively quotient `340282366920938463463374607431768211456`
    or its negative and remainder zero, without narrowing.
- id: FR-374-AC-4
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct exact_div on every sign combination of dividend 6 and divisor 2
    returns the signed quotient 3; zero dividend with either divisor sign returns
    zero. The beyond-i128 divisible literals in AC-3 return the same signed quotient.
    The nondivisible controls `5/2 = 2` and `-5/2 = -2` retain the prior behavior,
    without divisibility rejection or a fallible helper result.
- id: FR-374-AC-5
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Inspection accounts for every exact-owned production caller in Rational,
    Decimal and division. Canonical callers establish the documented nonzero invariant
    without forced Integer/BigInt clones, new public errors in existing canonical
    infallible APIs, caller-divisor admission unwrap/expect, unchecked public admission
    or fabricated numeric fallback. Any private token construction has an explicit
    owning nonzero invariant.
- id: FR-374-AC-6
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: With sufficient real meters, zero divisor under all three divide profiles
    and both members, plus public modulo, returns `Undefined(DivisionByZero)` after
    exactly the operation's operands charge, with no arithmetic/domain/result-retain
    or admission charge. Denying operands returns the existing Incomplete first. Nonzero
    success and exposed-member refusal controls retain the existing laws, charge requests,
    ordered prefixes, counters and outcome fields; public metered signatures remain
    unchanged.
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '[TC-922](../test-cases/TC-922-nonzero-integer-divisor-admission.md) specifies
    the

    bounded direct-helper and metered regression controls. The current baseline is

    the three helpers and their callers at exact main

    `56175fc1925549e49ea843798c5c3f3d69b99ece`; the measured QSL constant-two caller
    is

    at QSL main `3e71f94f6e665eca450386e254512cb7770e8b74`.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: '[FR-371 AC-5](./FR-371-bounded-integer-representation-refinement.md#acceptance-criteria)

    preserves these admitted-divisor signatures during prospective representation

    refinement. All other FR-371 guarantees retain their existing scope.'
- id: FR-374
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'PROPOSED / UNIMPLEMENTED / UNRUN. This is the exact-owned IR-734 admission

    contract. No executable Trace bindings, build, proof, downstream adoption or

    gate result accompany this specification. Exact-divisibility strengthening,

    compatibility APIs and downstream pin/lock qualification are outside this

    requirement.

    '
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "---\nid: TC-922\ntitle: \"Nonzero Integer admission prevents direct zero\
    \ division\"\ntype: TC\norg: agent-ix\nrelationships:\n  - target: ix://agent-ix/quire-exact/FR-374\n\
    \    type: verifies\n---\n# TC-922: Nonzero Integer admission prevents direct\
    \ zero division"
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'Verify [FR-374](../functional/FR-374-nonzero-integer-divisor-admission.md)
    through

    public borrowed admission and the three genuine unmetered helpers, with bounded

    metered preservation controls. The controls distinguish construction zero denial

    from evaluator zero Undefined and from an internal canonical invariant defect.

    Expected values and charge prefixes are independently specified; the obsolete

    plain-divisor signatures are not retained as a comparison implementation.'
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "1. Supply zero from a primitive, canonical parse and `one.sub(one)` to\n\
    \   `NonZeroInteger::try_from`. Assert `Err(ZeroDivisor)` directly. Admit both\n\
    \   signs of 2 and the beyond-i128 dividend literal in AC-3, asserting the\n \
    \  `get` borrow identifies the original Integer. Inspect the admission path\n\
    \   for clone/materialization absence and typed unit-error/public exports.\n \
    \  Bind an admitted result to `NonZeroInteger<'_>` and its error to\n   `ZeroDivisor`;\
    \ no message parsing or caller-input unwrap is a control.\n2. Use the crate-root\
    \ public API to call all three helpers with one reusable\n   borro"
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "wed copyable token. Add scoped negative external compilation/doc\n   controls\
    \ for each plain-`&Integer` helper parameter, private-field\n   construction,\
    \ writable referent access and escaping the referent lifetime.\n   Inspect the\
    \ token API for no Default or unchecked public constructor.\n   These controls\
    \ establish type exclusion, rather than attempting division\n   on a token that\
    \ public admission cannot produce.\n3. Run each of the four independently enumerated\
    \ signed `(7,2)` pair variants\n   in FR-374 AC-3 through truncating and floor\
    \ helpers. Assert exact quotient\n   and remainder constants. Ru"
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "n zero dividends with both signs of 2 and the\n   two signed-divisor beyond-i128\
    \ literal controls. Run exact_div on the four\n   signs of `(6,2)`, zero dividends\
    \ with both divisor signs, the two large\n   divisible literal controls, `5/2`\
    \ and `-5/2`. Match every literal result;\n   no host-width or dependency division\
    \ computes the expected oracle.\n4. Inspect every exact-owned production call\
    \ site in Rational, Decimal and\n   division against its documented nonzero evidence.\
    \ Check paired gcd use,\n   positive canonical denominators, ten/two/five constants,\
    \ squared powers\n   and powers of ten. Identif"
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "y the mathematical proof at any private token\n   construction. Reject\
    \ caller-divisor admission unwrap/expect, forced\n   validation clones, unchecked\
    \ public bypass, new construction errors in\n   canonical infallible public APIs\
    \ and fabricated fallback arithmetic.\n   Keep QSL's literal-two caller and its\
    \ pin qualification in QSL's lane.\n5. On fresh sufficient real meters, call public\
    \ `divide` with dividend 1\n   and divisor zero under each profile and member,\
    \ and `modulo` with the\n   same operands. Assert exact `Undefined(DivisionByZero)`,\
    \ zero retained\n   results and an admission count of one. W"
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "ith test-support, assert exactly\n   `[IntegerDivisionOperands]` or `[IntegerModulusOperands]`.\
    \ Work consumed\n   is 1 and IntegerBits/ValueOccurrences high-water sizes are\
    \ 1/2;\n   result consumption is zero. Repeat with work limit 0, then a real\n\
    \   occurrence-1 operands-point denial injection: each returns the existing\n\
    \   Incomplete at operands, with an empty prefix and no consumed work or\n   retained\
    \ result. The ordinary work denial reports limit 0, consumed 0\n   and next charge\
    \ 1. Inspect all incomplete fields against the unchanged\n   meter contract, rather\
    \ than treating a construction error "
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "as evaluation.\n6. Preserve bounded nonzero regression controls from\n\
    \   [TC-905](./TC-905-single-member-integer-division.md): negative-operand\n \
    \  profile laws, quotient-only domain admission for `10/5` in `[1,10]`,\n   quotient\
    \ refusal with a permitted remainder, successful four-point\n   charge order and\
    \ refusal's three-point prefix. Include public modulo\n   `(-7,-3) = 2` with its\
    \ own four named points. For each control assert\n   existing charge request amounts,\
    \ exact outcome/refusal fields, consumed\n   counters and ordered prefix, including\
    \ absence of an admission event.\n   Bind implementation "
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: "tests to the FR-374 criterion they actually assert;\n   existing FR-357\
    \ bindings remain separate metered-semantic evidence."
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'Zero cannot be admitted to the public helper parameter type. Every admitted

    divisor borrows its original immutable mathematical integer, and direct helper

    results match the stated signed and arbitrary-size constants. `exact_div`

    retains its current nondivisible truncation behavior. Canonical internal

    callers preserve their public error/signature contracts and establish nonzero

    without a caller-input panic or substitute arithmetic. Metered operations

    preserve operands-before-zero classification, original outcomes, charge

    requests and admitted prefixes.'
- id: TC-922
  path: spec/test-cases/TC-922-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'PLANNED / UNRUN. This document specifies controls only; it records no
    executed

    test, review verdict, build, proof, allocation benchmark, consumer adoption or

    gate result. It does not alter the prospective qualification obligations of

    [TC-919](./TC-919-bounded-integer-representation-refinement.md).

    '
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'Public signatures and all existing Integer methods remain available, except
    that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating
    and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined
    by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their
    former plain &Integer parameters. Their existing nonzero results and infallible
    return types remain preserved; every other public signature and Integer method
    retains its existing preservation guarantee. Trait checks preserve Send/Sync and
    current panic-unwind traits. Default and '
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: test-support checks and the required thumbv7em-none-eabi build retain no_std
    plus alloc behavior and local unsafe prohibition. Existing IEEE BigInt consumers
    observe the same exact values. If lazy materialization is selected, cache concurrency
    tests demonstrate one retained immutable winner, released losers and the disclosed
    first-access allocation behavior; a safe observer records temporary candidates
    without imposing a fictitious once-only bound.
- id: src/integer.rs
  path: src/integer.rs
  role: context_only
  excerpt: "pub fn exact_div(&self, divisor: &Self) -> Self {\n        Self(&self.0\
    \ / &divisor.0)\n    }\n\n    /// Truncating quotient/remainder; `divisor` is\
    \ nonzero.\n    pub fn div_rem_truncating(&self, divisor: &Self) -> (Self, Self)\
    \ {\n        let (quotient, remainder) = self.0.div_rem(&divisor.0);\n       \
    \ (Self(quotient), Self(remainder))\n    }\n\n    /// Floor quotient/remainder;\
    \ `divisor` is nonzero.\n    pub fn div_mod_floor(&self, divisor: &Self) -> (Self,\
    \ Self) {\n        let (quotient, remainder) = self."
- id: src/rational.rs
  path: src/rational.rs
  role: context_only
  excerpt: "pub fn divided_by_power_of_ten(&self, exponent: u64) -> Self {\n     \
    \   let denominator = self.denominator.mul(&Integer::power_of_ten(exponent));\n\
    \        let divisor = self.numerator.gcd(&denominator);\n        Self {\n   \
    \         numerator: self.numerator.exact_div(&divisor),\n            denominator:\
    \ denominator.exact_div(&divisor),\n        }\n    }\n\n    /// Whether the value\
    \ is zero.\n    pub fn is_zero(&self) -> bool {\n        self.numerator.is_zero()\n\
    \    }\n\n    /// The exact value `self / 2^expon"
- id: src/decimal.rs
  path: src/decimal.rs
  role: context_only
  excerpt: "fn split_factor_five(value: &Integer, limit: u64) -> (Integer, u64) {\n\
    \    let mut remaining = value.clone();\n    let mut count = 0_u64;\n    // `(5^width,\
    \ width)` with `width = 2^j`.\n    let mut powers = vec![(Integer::from(5_i64),\
    \ 1_u64)];\n    loop {\n        let (power, width) = powers.last().expect(\"the\
    \ ladder starts with 5^1\");\n        if *width > limit - count {\n          \
    \  break;\n        }\n        let (quotient, remainder) = remaining.div_rem_truncating(power);\n\
    \        if !remainder.is_zero("
- id: src/division.rs
  path: src/division.rs
  role: context_only
  excerpt: "fn reject_zero_divisor(divisor: &Integer) -> Result<(), Stop> {\n    if\
    \ divisor.is_zero() {\n        Err(Stop::Undefined(Undefined::DivisionByZero))\n\
    \    } else {\n        Ok(())\n    }\n}\n\n/// The bounded consumer interval a\
    \ failed membership decision was made\n/// against. Only a bounded domain can\
    \ refuse a member, so a mathematical\n/// domain here is a checked-program invariant\
    \ failure.\nfn refused_interval(domain: &IntegerDomain) -> Result<Box<IntegerInterval>,\
    \ Stop> {\n    match domain {\n        Int"
- id: spec/functional/FR-357-single-member-integer-division.md
  path: spec/functional/FR-357-single-member-integer-division.md
  role: context_only
  excerpt: 'The public `modulo` operation SHALL return the Euclidean remainder for
    nonzero divisors independently of any `DivisionProfile` selected for a separate
    `divide` call.


    ## Use case


    A caller evaluates `10 div y` over `1..=10` at `y = 5`. The remainder 0 is outside
    the domain, but the expression exposes the quotient 2, so the result is 2.


    ## Behavior


    1. **Typed outcomes.** A zero divisor is `Undefined(DivisionByZero)`. An exposed
    member outside a bounded domain is `Refused(DivisionOutOfDomain { m'
```

## Dispositions

Round 1 independently examined agent-ix/quire-exact@b572f1dfae7b2dddb47b6bf5730cbeedea236201. Original findings remain unchanged. Published fix changes only FR-374 relationship and baseline prose; all nine original SR exports were independently compared byte-for-byte with the committed copies. No scoped regression found in arithmetic, signatures, ownership or metered preservation. No executable evidence claimed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b572f1dfae7b2dddb47b6bf5730cbeedea236201 |
