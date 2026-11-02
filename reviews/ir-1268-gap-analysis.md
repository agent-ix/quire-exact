---
id: SR-2026110101301
title: "IR-1268 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; IR-1268; src/, spec/; PR 125 frozen 4c1df8e"
review_set: subset
---

## Summary

Independent gap-analysis of agent-ix/quire-exact PR #125, ticket IR-1268, frozen head 4c1df8eeb896e470d7192ceb6a0020041f0e288b against 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae. No applicable AssuranceProfile exists. Source is read-only; no Cargo or backend execution was performed by this reviewer.

## Verdict

**FAIL (baseline repository gaps; no introduced gap)**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Existing baseline criterion FR-359-AC-9 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-359-cumulative-meter-boundary.md:39 |
| FND-002 | high | Existing baseline criterion FR-359-AC-10 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-359-cumulative-meter-boundary.md:40 |
| FND-003 | high | Existing baseline criterion FR-371-AC-6 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-371-bounded-integer-representation-refinement.md:90 |
| FND-004 | high | Existing baseline criterion FR-384-AC-5 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-384-admitted-native-integer-refinement.md:81 |
| FND-005 | high | Existing baseline criterion FR-385-AC-1 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md:86 |
| FND-006 | high | Existing baseline criterion FR-385-AC-2 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md:87 |
| FND-007 | high | Existing baseline criterion FR-385-AC-3 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md:88 |
| FND-008 | high | Existing baseline criterion FR-385-AC-4 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md:89 |
| FND-009 | high | Existing baseline criterion FR-385-AC-5 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md:90 |
| FND-010 | high | Existing baseline criterion FR-385-AC-6 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md:91 |
| FND-011 | high | Existing baseline criterion FR-385-AC-7 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md:92 |
| FND-012 | high | Existing baseline criterion FR-388-AC-3 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-388-read-closed-limit-kind-spellings.md:34 |
| FND-013 | high | Existing baseline criterion FR-391-AC-4 has no test tagged with its ID; this PR does not introduce or resolve that repository assurance gap. | spec/functional/FR-391-fallible-effective-identity-copy.md:46 |

## Coverage

Repository-wide computed matrix: 256 criteria, 221 tagged, 13 untagged, 22 method-without-symbol, zero ignored-only. quoin matrix --repo . --json was also read: evidence is no-run-evidence, not proof or passing execution evidence. quire coverage diagnostics include unused archetypes and property-shape advisory; these are not silently counted as coverage. Its 103 untracked symbol entries include external QSpec criterion/TC references and local TC references; these are baseline contextual references, not automatically defective local AC bindings. No new test tag names an absent criterion.

All 13 untagged rows predate this PR: spec files are unchanged and no existing test/source lines or tags were removed. FR-359 AC9/10 include deliberately parked/external proof/consumer obligations. Record baseline gaps rather than fabricate local tags or retry prohibited proofs. Repository FAIL is not a new defect in the approved two-helper slice. No unrelated source/spec fix is requested by this child.

Public facade and module inventory spans identity/provenance, refusal/outcome, bounds/accounting, native numerics, collections, values and scalar operation families (502 public declaration lines inventoried); existing functional specs and computed bindings own these surfaces. Changed helpers remain private under accounting/IEEE requirements. Source scan found no new TODO, dummy return, no-op or production stub; no test replaces the changed implementation. No introduced unowned behavior or hollow evidence found. This is a structural repository audit, not an exhaustive semantic proof of every unchanged implementation. Semantic review skipped (not opted in). Plan completion: not assessed.

Reconciliation: quoin matrix 0.29.1 and quire matrix — no run evidence.
Plan completion: not assessed

## Examined Scope

```yaml
scope:
- id: FR-088-AC-11
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: For an enum shape whose canonical member list is `b`, `a`, `c`, the ranks
    of the variants are `b` 0, `a` 1, `c` 2, and a set of all three values visits
    `b`, `a`, `c`. For an unordered enum whose canon
- id: FR-088-AC-12
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: A declared unit `UnitId` and a compound unit `UnitId` are unequal when
    their domains or admitted identities differ. A declared unit's `UnitId` carries
    its admitted node identity in the declared domain
- id: FR-089-AC-2
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: The kernel `Value::Population` variant's payload type is `PopulationId`;
    no kernel source file imports a population binding or any other caller's model
    type to define, construct or match this variant,
- id: FR-089-AC-6
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`,
    kernel `ValueType::admits` returns false; given two `Value::Population` operands,
    kernel `equality::plan_pairs` r
- id: FR-090-AC-7
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Refusal` has no variant whose payload is a wrong-snapshot cause.'
- id: FR-090-AC-11
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Undefined` has no `PreconditionFalse` variant.'
- id: FR-090-AC-12
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Undefined` has no `AbsentKey` variant.'
- id: FR-090-AC-13
  path: spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md
  role: examined
  excerpt: '`Refusal::CheckedInvariant` carries only FR-369''s closed typed cause;
    it has no arbitrary caller message, QSL model/checker type, catalog code or generic
    caller-cause escape hatch.'
- id: FR-096-AC-8
  path: spec/functional/FR-096-kernel-refusal-code-and-cause.md
  role: examined
  excerpt: Each catalogued kernel refusal below returns the code and cause spelling
    named for it from `Refusal::code()` and `Refusal::cause()`, and every FR-369 `CheckedInvariant
    { cause }` returns neither. `Ine
- id: FR-097-AC-7
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: An unbounded collection type admits a collection value of any size with
    no cardinality refusal; it still charges `collection.bound`, and stops with `Incomplete`
    at that charge point only when the call
- id: FR-097-AC-8
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: A caller outside the crate can call `CardinalityBound::violation(count)`
    and receive the existing `Option<BoundViolation>` type. For `[2, 4]`, counts 1,
    2, 3, 4 and 5 return below-minimum, none, none,
- id: FR-262-AC-2
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s
    around `Boolean` clones, compares equal to its clone, hashes equal to its clone,
    formats for debug and drops. A `Value` of t
- id: FR-262-AC-3
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: A `Value` nested N levels deep, alternating `Option` and `Sequence` levels
    around an `Integer` with each level declared with the type of its depth, holds
    at least N - 10 and at most 2N distinct `Optio
- id: FR-357-AC-1
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a zero divisor under any profile and either member, `divide` returns
    `Undefined(DivisionByZero)`.
- id: FR-357-AC-2
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `10 div y` over `1..=10` at `y = 5`, `divide` of the quotient returns
    2; given `x rem -1` over `-10..=5` at `x = -10`, `divide` of the remainder returns
    0.
- id: FR-357-AC-3
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a quotient outside the domain, `divide` refuses with cause `quotient-outside-domain`;
    given a remainder outside the domain, it refuses with `remainder-outside-domain`.
- id: FR-357-AC-4
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given -7 and 2, truncating returns (-3, -1), floor (-4, 1) and Euclidean
    (-4, 1); given -7 and -2, truncating returns (3, -1), floor (3, -1) and Euclidean
    (4, 1).
- id: FR-357-AC-5
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `i64::MIN` and -1 under any profile, `divide` of the quotient returns
    2^63 in a domain that holds it. In a signed 64-bit domain the quotient refuses
    with `quotient-outside-domain` and the remain
- id: FR-357-AC-6
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a successful `div` or `rem`, the meter admits `integer-division.operands`,
    `integer-division.arithmetic`, `integer-division.domain` (one value occurrence)
    and `integer-division.result-retain` (o
- id: FR-357-AC-7
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: For dividend/divisor pairs `(7, 3)`, `(7, -3)`, `(-7, 3)` and `(-7, -3)`,
    public `modulo` in the mathematical domain completes to 1, 1, 2 and 2 respectively.
    Running a `divide` under each of Truncatin
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit
    P charge under an ordinary limit, then admit P once and an unrelated point once.
    The next P charge returns `Incomplete` a
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan`
    returns `Incomplete` at `equality.plan` without changing any consumed counter,
    admission count, or test-support log;
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction
    with zero is rejected by that type, while a nonzero occurrence constructs a denial.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor
    returns all 4096 records with their points in order and `charge_log_truncated()`
    is false. On the 4097th admission the slice st
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()`
    and consumed work units reflect all 4097 admissions, including the one omitted
    from the diagnostic log.
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge
    log and its `Counters` accounting state remains fixed-size and heap-free.
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one
    result unit. The log retains the first 4096 points and reports truncation, while
    `admission_count()`, consumed work and consume
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`,
    admit 4097 one-work-unit charges and request one more work unit. The charge returns
    `Incomplete` at its requested point with `W
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`,
    admit 4097 charges each consuming one result unit, then request one more result
    unit. The charge returns `Incomplete` with `Res
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial
    for one named point. A charge at that point returns `Incomplete` with `WorkUnits`
    and leaves all counters, admission count, pr
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After admitting two work units, inject occurrence 1 at point P and attempt
    a three-work-unit `charge` at P under otherwise sufficient limits whose configured
    work limits are 10 and 100 on separate met
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With an occurrence-1 injected denial selected, make the same `charge` independently
    fail an ordinary semantic-size counter and make a `charge_plan` with `pairs =
    2` independently fail its work reserva
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request
    of 9 followed by 3 and then a charge with no size request. The records expose
    respectively 9, 3 and no IntegerBits request whil
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently
    chosen semantic-size requests, including repeated same-kind requests and distinct
    amounts at admissions 4096 and 4097. The sin
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX -
    1` followed by one work unit reaches `u64::MAX` exactly. The next one-unit charge
    returns `Incomplete` at its named point with `
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting
    `u64::MAX` result units. Its work consumption is 1. A second charge requesting
    one result unit returns `Incomplete` at its name
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount
    is `u64::MAX + 1` returns `Incomplete` with `limit_kind = WorkUnits` and `next_charge
    = u64::MAX + 1`; no counter or admissi
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit
    and `u64::MAX + 1` result units returns `Incomplete` with `limit_kind = ResultUnits`
    and `next_charge = u64::MAX + 1`; no counter
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit
    under limits of at least 16 bits, `u64::MAX` work units and one result unit. Then
    attempt a charge carrying `integer_bits = 16` a
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters
    the first unavailable counter of a charge, including several simultaneously short
    counters and semantic sizes attached in eithe
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers
    without panic for every member of `LimitKind::ALL`: untouched counters are zero,
    semantic sizes are admitted high-water maxima and'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum
    observable through `consumed()`; the diagnostic log record is verified under FR-359-AC-12.
    With `IntegerBits` limit 5 and reque
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference`
    uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand
    charge with at most two semantic-size '
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests
    below pass their own assertions under their original domains, oracle, completion
    and counterexample checks, unwind 3, Cadica
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and
    called helpers finds no traversal with a trip count greater than that charge's
    semantic-size request count, including no traver
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362
    schedule contains exactly the admitted prefix in charge order, with each record's
    point and absent-or-present per-admission se
- id: FR-360-AC-1
  path: spec/functional/FR-360-integer-minimum-magnitude.md
  role: examined
  excerpt: '`Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808
    (`2^63`), is nonnegative, and differs from the original negative value. It neither
    overflows nor returns `i64::MIN'
- id: FR-361-AC-1
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For an integer multiplication with a large operand, set `integer_bits`
    to one below the exact `integer-arithmetic.arithmetic` amount while permitting
    the operand charge. The outcome is `Incomplete` at
- id: FR-361-AC-2
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a rational operation with a large numerator or denominator, set `integer_bits`
    to one below the exact `rational-arithmetic.arithmetic` amount while permitting
    the operand charge. The outcome is `I
- id: FR-361-AC-3
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Inject a one-shot denial at each of `integer-division.arithmetic` and `integer-modulus.arithmetic`
    for large nonzero operands. Each returns `Incomplete` at the selected point with
    `limit_kind = WorkUn
- id: FR-361-AC-4
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For valid decimal operands requiring a `2^20`-place scale expansion, set
    `scale_expansion` to one below the planned shift and leave other limits sufficient.
    The outcome is `Incomplete` at `decimal.sca
- id: FR-361-AC-5
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For decimal addition of small coefficients at scales 0 and `2^20`, set
    `integer_bits` to the admitted scale-expansion bit amount, one below the planned
    arithmetic bit amount, with other limits suffici
- id: FR-361-AC-6
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a decimal value retained at a target scale requiring a `2^20`-place
    power of ten, set `decimal_digits` one below the exact result-retain amount while
    permitting preceding charges. Evaluation retur
- id: FR-361-AC-7
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: On a fresh meter, floor division of a positive at-least-65,536-bit dividend
    by 3, exposing the quotient in the mathematical domain, completes to the independently
    calculated floor quotient with `integ
- id: FR-361-AC-8
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For that floor quotient fixture, lower the `integer_bits` limit by one.
    The call returns `Incomplete` at `integer-division.operands` with `limit_kind
    = IntegerBits`, `consumed = 0` and `next_charge` e
- id: FR-361-AC-9
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Multiply retained decimals `(1, 0)` and `(1, 0)` into an Exact target with
    coefficient bounds 0 through 1 and scales 0 through `2^20`. At `decimal_digits
    = 2^20`, the `decimal.result-retain` charge re
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`,
    -1, 0, 1 and `i128::MAX`; for negate, use each member as a unary operand. Compare
    integer add, subtract, multiply and negate on
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes
    to the independently authored decimal value `340282366920938463463374607431768211454`;
    negating `Integer::from(i128::MIN)`
- id: FR-362-AC-3
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For fixed and reproducibly sampled rational pairs whose checked `i128`
    reference intermediates fit and whose denominators are nonzero, rational add,
    subtract, multiply, divide by nonzero and negate eq
- id: FR-362-AC-4
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Integer and rational comparisons over fixed and reproducibly sampled pairs
    equal independently checked numeric comparisons for equality and order. Decimal
    comparison is specified by [FR-363](./FR-363-
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each
    rational arithmetic operation, integer ordering and rational ordering, the admitted
    point sequence equals its family row in B
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate
    the maximum size amount across their points from the formulas in Behavior, using
    the unreduced numerator and denominator for ra
- id: FR-362-AC-7
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter, directly add integers 5 and 7, compare rationals 1/2 and
    2/3 for less-than, then retain an already-decided Boolean `true`. The admitted
    atom sequence consumes 7 work units and 3 result u
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer
    arithmetic, rational arithmetic, integer ordering, rational ordering and already-decided
    Boolean retention calls. Each returns `I
- id: FR-362-AC-9
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For every already-decided Boolean connective, the completed value matches
    its truth table and the kernel admits `boolean.result-retain` once, consuming
    one work and one result unit.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer add, subtract, multiply and negate, rational add, subtract,
    multiply, divide and negate, and integer/rational ordering, independently calculate
    the required bit high-water amount from Beha
- id: FR-362-AC-11
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`,
    typed domain `Refused`, and `Incomplete` with no partial value; `completed()`
    returns a value only for `Completed`, inc
- id: FR-362-AC-12
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '`Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator
    1 and denominator 2, proving reduction to lowest terms and cancellation of two
    negative signs. `Rational::new(1, -2)` exposes -1'
- id: FR-362-AC-13
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)`
    is false. Dividing `1/1` by `3/1` under that domain returns `Refused(RationalOutOfDomain
    { target })`
- id: FR-362-AC-14
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Call the five scalar atoms `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0`
    directly on one meter. They complete to true, 1, true, 0, false in order, consuming
    15 work and 5 result units. With work limit 1
- id: FR-362-AC-15
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Directly divide rational `3/1` by `2/1` on a fresh meter. The result is
    `3/2`, with high-water `integer_bits = 3`, value occurrences 2, work 4 and results
    1; work limit 3 stops at `rational-arithmetic
- id: FR-362-AC-16
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter directly add integers `0 + 1` and then `1 + 2`. The results
    are 1 and 3; the integer point sequence occurs twice, high-water `integer_bits
    = 3`, occurrences 2, work 6 and results 2. Calle
- id: FR-362-AC-17
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain`
    after operands and arithmetic, with zero result units; work limit 3 completes
    to 12 with three work and one result unit. F
- id: FR-362-AC-18
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For `k` in 63, 64, 65, 127, 128, 200 and 511, compare exact results and
    charged bit high-water sizes around `2^k`: `(2^k - 1)(2^k + 1)` charges `2k +
    1` bits for a `2k`-bit result, `(2^k)^2` charges `'
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Through public `evaluate_integer_arithmetic` and a fresh public `Meter`
    for each call, add `8 + 3` and `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`,
    without a result bound and with all non-bit limi
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On fresh sufficient public meters under `test-support`, evaluate the six
    rational calls listed below with no result domain. Each completes to the listed
    canonical value and admits operands, arithmetic
- id: FR-362-AC-21
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On a fresh public meter, a direct default charge at `TemporalVisit` (`temporal.visit`)
    succeeds with `work_units = 1`, `result_units = 0` and no semantic-size request
    when the work limit is exactly 1.
- id: FR-362-AC-22
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'A one-shot injected denial of the first direct charge at `TemporalVisit`
    (`temporal.visit`) returns `Incomplete` naming `TemporalVisit` and `WorkUnits`,
    with `limit = consumed = 0` and `next_charge = '
- id: FR-363-AC-1
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: 'For decimal pairs with equal values but different retained scales, and
    for positive, negative and zero values at differing scales, `order_numbers` agrees
    with an independent exact rational comparison '
- id: FR-363-AC-2
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For a successful decimal comparison, the meter admits `ordering.operands`,
    `ordering.arithmetic`, `ordering.result-retain` in that order. On a fresh meter,
    its size high-water counters equal the maxim
- id: FR-363-AC-3
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: Comparing retained `(0, 5) >= (0, 0)` completes `true` and consumes `integer_bits
    = 18`, `decimal_digits = 6`, `scale_expansion = 5`, `value_occurrences = 2`, `work_units
    = 3`, `result_units = 1`. Com
- id: FR-363-AC-4
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: With all limits at `u64::MAX` except `decimal_digits = 64`, testing retained
    `(1, 0) < (1, u32::MAX)` returns `Incomplete` at `ordering.arithmetic` with `limit_kind
    = DecimalDigits`, `limit = 64`, `co
- id: FR-363-AC-5
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: 'On separate fresh meters with generous limits, inject a one-shot denial
    at occurrence 1 of each decimal-ordering point: `ordering.operands`, `ordering.arithmetic`,
    and `ordering.result-retain`. For ea'
- id: FR-363-AC-6
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: 'For retained decimals `(110, 2)` and `(11, 1)`, public unmetered `Decimal::compare`
    returns `Equal`, `numerically_equal` returns true and both normalized representations
    are `(11, 1)`; their retained '
- id: FR-363-AC-7
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)`
    both complete false. The mixed pair consumes integer bits, decimal digits and
    scale expansion 8/3/1; the narrow pair consumes 4/2
- id: FR-364-AC-1
  path: spec/functional/FR-364-ieee-flag-iteration.md
  role: examined
  excerpt: For every subset of the five public flags, collect it into `IeeeFlags`
    in forward and reverse vocabulary order. Both `iter()` results equal that subset
    filtered from `IeeeFlag::ALL`, without duplicate
- id: FR-365-AC-1
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: Round retained `(5, 1)` and `(-5, 1)` to a scale-zero target with coefficient
    bounds -10 through 10 under each of the six modes. Values or `Refused(InexactDecimal
    { target })` match the tie table in B
- id: FR-365-AC-2
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: '`RoundingMode::default()` and the mode read back from a target constructed
    with it are `Exact`. Rounding retained `(1, 1)` to that scale-zero target returns
    `Refused(InexactDecimal { target })`, rathe'
- id: FR-366-AC-1
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For same-width Add with a positive quiet NaN of payload 5 and a negative
    signaling NaN of payload 9 in either operand order, the result bits are the first
    NaN quieted and Invalid is present. Two quiet
- id: FR-366-AC-2
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: 'Converting a binary64 NaN with payload `2^22` or `2^30` to binary32 returns
    `Refused(IeeeNanPayloadNotRepresentable { source: Binary64, target: Binary32 })`
    without truncating the payload or retaining'
- id: FR-366-AC-3
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: Converting either signed binary32 zero to the rational domain with numerator
    interval `[-1, 1]` and denominator interval `[1, 1]` completes to canonical `0/1`.
    Positive zero has no loss; negative zero
- id: FR-366-AC-4
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For each width, distinct bit patterns representing the ten classes in Behavior
    have strictly increasing `total_order_key()` values in that stated order, including
    quiet/signaling NaNs of both signs an
- id: FR-367-AC-1
  path: spec/functional/FR-367-text-profile-mismatch-before-charging.md
  role: examined
  excerpt: 'Admit the same UTF-8 text "a" once with Nfc and once with BinaryUtf8 using
    separate sufficient meters. On a fresh comparison meter whose ten limits are zero,
    public `compare_text(Equal, left, right)` '
- id: FR-368-AC-1
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`,
    has a unique documented spelling from `as_str()` and round-trips through `from_code()`;
    an unknown spelling returns `None`
- id: FR-368-AC-2
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 'Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`,
    has a unique documented `as_str()` spelling, and the sequence is exactly the ten
    `ScalarLimits` fields: integer bits, decimal '
- id: FR-368-AC-3
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: The public typed variants `FrameWrite` and `FrameRedefinesLink` spell exactly
    `frame.write` and `frame.redefines-link`; each round-trips through `from_code()`,
    while misspellings return `None`. `Charg
- id: FR-368-AC-4
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Inspection of the public kernel API and the caller's frame path identifies
    the two typed names in the kernel vocabulary and the caller's independent frame
    event, meter and `ModelNormalizationLimits.wo
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause`
    payload whose closed variant set and four typed `IllTypedCause` payloads equal
    the thirty-one table rows; `CheckedInvarian
- id: FR-369-AC-2
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Every kernel producer in collection, value, division and equality returns
    exactly its table cause; two population values return `PopulationPair`, a population
    paired with another kind returns `ValueKi
- id: FR-369-AC-5
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()`
    return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ.
    Ordinary kernel refusal codes and '
- id: FR-369-AC-6
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Every typed cause interpreted by QSL''s reference `Machine::stopped` follows
    its existing internal-fault path: no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant`
    a'
- id: FR-369-AC-8
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public carrier contains no QSL model/checker/catalog type or dependency
    on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and
    an exhaustive source inventory finds every ke
- id: FR-369-AC-10
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Inspection against CG FR-021-AC-38..40 / FR-042-AC-2/3 finds the same two
    generated-function triggers, checker-issued result type and genuine owner declaration
    inputs. A proven wrong completed value o
- id: FR-370-AC-1
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'A UUID value is admitted only by `ValueType::Uuid` and a Timestamp value
    only by `ValueType::Timestamp`; crossed native kinds and Integer, Text and Reference
    values are rejected by both native types, '
- id: FR-370-AC-2
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'Each same-kind native pair follows the existing one-terminal-pair equality
    schedule: an equal pair yields `true`, an unequal pair yields `false`, and the
    planned pair count and charges equal the corre'
- id: FR-370-AC-3
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'A direct equality plan pairing UUID or Timestamp with another value kind
    returns `Refusal::CheckedInvariant { cause: ValueKindMismatch }`, while a direct
    mixed-kind `compare_keys` call returns `None`;'
- id: FR-370-AC-4
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: '`compare_keys` gives each native type a total key consistent with its
    equality relation and ordered by canonical payload ASCII bytes; Timestamp `"10"`
    precedes `"2"` and `"-1"` precedes `"-2"` in the '
- id: FR-370-AC-5
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Existing scalar and composite admission, equality, key comparison and accounting
    vectors retain their prior results.
- id: FR-370-AC-6
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: UUID values preserve exactly 16 network-order octets, including Nil, Max
    and non-RFC-version bit patterns; only the lowercase ASCII `8-4-4-4-12` caller
    payload admits, and uppercase, braces, URN form,
- id: FR-370-AC-7
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'Timestamp values preserve signed `i128` POSIX-epoch nanoseconds, including
    inclusive bounds `-170141183460469231731687303715884105728` and `170141183460469231731687303715884105727`
    and `-1`, `0`, `1` '
- id: FR-370-AC-8
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Kernel constructors accept canonical UUID or Timestamp text supplied after
    the caller has validated its tagged wire envelope. Each malformed text returns
    `ConstructionRefusal` at `Component::Value` wi
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate
    preserve the exact value for fixed boundaries and reproducibly sampled inputs.
    Fixed cases include i128 MIN/MAX, -1
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and
    arithmetic routes remain equal, identically ordered, identically hashed with the
    same hasher and identically formatted before an
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362
    schedule and counters on native, native-overflow and arbitrary-size cases. Test
    sufficient limits, independently calculated exact
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native
    Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt
    construction/materialization. A safe extern
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Public signatures and all existing Integer methods remain available, except
    that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating
    and Integer::div_mod_floor preserve the admit
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic
    mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second
    native ceiling, 16GiB memory ceiling, comple
- id: FR-371-AC-7
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Private mechanism checks distinguish Small64 and Small128 construction,
    normalization and clone routes according to AD-003 without exposing a public tag
    API. Source inspection identifies storage match
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the authoritative membership rule for inclusive
    lower/upper equality, adjacent outsiders, negative and zero values, scale refusal,
    all rounding spellings and equivalent retaine
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Production helper observations on admitted `B,S` fixtures stay within the
    one-request `4 × L_B`-byte scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step
    bound, including a longest-pass endpoint fi
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Denying the actual scratch reservation reports its actual native request
    as typed allocation failure. A separate checked-capacity failure retains its own
    classification. Removing the allocation denial
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation
    observed during the longest admitted division/comparison path returns the original
    cause before another limb step or result; can
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder
    and sign invariants, exact reservation boundary, checked layout conversions, allocation-free
    inline unsigned-magnitude stream and
- id: FR-373-AC-1
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: The public crate-root `ieee_to_exact` has the exact direct signature in
    Behavior. `IeeeExactTarget` and its re-export are absent; no compatibility entry
    point or copied conversion algorithm remains. A
- id: FR-373-AC-2
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Binary32 `0x3fc00000` and binary64 `0x3ff8000000000000` complete as `3/2`
    with no loss in numerator domain `[-3,3]`, denominator domain `[1,2]`. Both signed
    zeros at both widths complete as `0/1`; onl
- id: FR-373-AC-3
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Quiet NaN, signaling NaN and positive/negative infinity at each width return
    exactly `Undefined(IeeeNotFinite)` after one operand charge, one work unit, source-width
    IntegerBits and one ValueOccurrenc
- id: FR-373-AC-4
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Converting `3/2` to numerator domain `[-1,1]`, denominator domain `[1,2]`,
    and separately to numerator domain `[-3,3]`, denominator domain `[1,1]`, returns
    `Refused(IeeeRationalOutOfDomain { target })
- id: FR-373-AC-5
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Successful finite conversion admits exactly operands, intermediate and
    retain in that order, consuming three work units and one result unit. Ordinary
    work limits 0, 1 and 2 deny the respective next po
- id: FR-373-AC-6
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: For Requested and Deadline, pre-cancelling the original handle attached
    to the supplied meter returns WorkUnits incomplete at operands with limit = consumed
    = 0 and next charge 1. Deterministic cancel
- id: FR-373-AC-7
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Precharge the supplied meter once at FunctionCall with work 2, result 1,
    IntegerBits 80, ValueOccurrences 5 and TextScalars 7. Direct conversion of either
    width's `3/2` under sufficient limits ends wi
- id: FR-374-AC-1
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'Admission of zero constructed by primitive, canonical parse and zero-producing
    arithmetic returns exactly `Err(ZeroDivisor)` and no token. Positive and negative
    values, including values outside i128, '
- id: FR-374-AC-2
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: An external public-API caller can reuse one borrowed copyable token across
    all three helpers. A plain `&Integer` cannot satisfy any of the three divisor
    parameters. External field construction, writab
- id: FR-374-AC-3
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)`
    return respectively truncating `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and floor
    `(3,1)`, `(-4,-1)`, `(-4,1)`, `(3,-1)`. A zer
- id: FR-374-AC-4
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct exact_div on every sign combination of dividend 6 and divisor 2
    returns the signed quotient 3; zero dividend with either divisor sign returns
    zero. The beyond-i128 divisible literals in AC-3 re
- id: FR-374-AC-5
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Inspection accounts for every exact-owned production caller in Rational,
    Decimal and division. Canonical callers establish the documented nonzero invariant
    without forced Integer/BigInt clones, new pu
- id: FR-374-AC-6
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: 'With sufficient real meters, zero divisor under all three divide profiles
    and both members, plus public modulo, returns `Undefined(DivisionByZero)` after
    exactly the operation''s operands charge, with '
- id: FR-375-AC-7
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: No cross-crate Kani contract, stub, or assumed postcondition is treated
    as checked unless a focused tool control demonstrates both a passing and a deliberately
    failing dependency-contract case through
- id: FR-375-AC-9
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: 'On equal and first-head-unequal bounded kernel operands, `planned_equality_with_plan`
    returns the Boolean relation and 17 planned pair events from the same formed plan;
    `plan_equality` agrees on that '
- id: FR-375-AC-10
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: When `Meter::charge` refuses the 17th pair, the result-bearing kernel operation
    returns the typed stop with exactly the prior 16 pair charges admitted and no
    result-retain charge; it does not report a
- id: FR-376-AC-1
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`plan_equality`, `planned_equality` and `planned_equality_with_plan` on
    two composites whose declaration keys differ return `false`, with a pair count
    of 1, and no `Refusal`. The result is the same in'
- id: FR-376-AC-2
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: 'On that pair, `planned_equality` admits `equality.plan-form`, `equality.plan`,
    exactly one `equality.pair` and `equality.result-retain`, in that order, with
    the plan-form charge computed from the two '
- id: FR-376-AC-3
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite pair of different declarations whose slots hold `Reference`
    values of different universes returns `false` and does not report `foreign_reference`,
    because the slots are not visited. As a c
- id: FR-376-AC-4
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: Two composites with equal declaration keys and equal slots give `true`.
    Equal declaration keys with a differing slot give `false`. The pair counts are
    those of the same plan before this requirement.
- id: FR-376-AC-5
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A pair of different declarations that sits as the payload of two present
    `Option`s, in a one-slot record, as the element of two one-element `Sequence`,
    `Set` and `Bag`, gives `false`, with pair counts
- id: FR-376-AC-6
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, `Option`, `Collection` or `Reference`,
    in either order, still refuses `ValueKindMismatch`. Two composites of one declaration
    key and different slot counts still refus
- id: FR-376-AC-7
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`member_equal` of a composite candidate against a member of a different
    declaration returns `false`, charging `collection.member-walk` then `collection.member-test`
    with one pair.'
- id: FR-376-AC-8
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`form` over an `OrderedSet` type whose element type is a declaration P,
    with occurrences of one composite of declaration C then one of P, completes with
    the two members in that supply order, charging '
- id: FR-376-AC-9
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A union member whose one position holds a `P` composite in one value and
    a `C` composite in the other (also `C` against `G`), in both orders, gives `false`
    with a pair count of 2 and no refusal; refer
- id: FR-377-AC-1
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: For admitted declaration keys `P` and `C` with `P < C` under the `NodeKey`
    order, `compare_keys(a, b)` for composites of `P` and `C` is `Some(Less)` and
    `compare_keys(b, a)` is `Some(Greater)`, whatev
- id: FR-377-AC-2
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'For three distinct admitted declaration keys `K0 < K1 < K2` under the
    `NodeKey` total order, composites whose single integer slots descend in the opposite
    order (`9`, `5`, `0`) compare `K0 < K1`, `K1 '
- id: FR-377-AC-3
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'Two composites of one declaration key keep the slot order of the rule
    before this requirement; `compare_keys(x, x)` is `Some(Equal)`; two composites
    of different declarations are never `Some(Equal)`, '
- id: FR-377-AC-4
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'A `Set` and a `Bag` of element type `Composite(P)` formed through `form`
    from one composite of each of two declarations, in each of the two supply orders,
    complete and visit the member of the smaller '
- id: FR-377-AC-5
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'Equal keys occur exactly when values are equal: for a set of composites
    of two declarations with equal and unequal slots, every pair has `compare_keys
    == Some(Equal)` exactly when `planned_equality` g'
- id: FR-377-AC-6
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'A composite against an `Integer`, two composites of one declaration key
    and different slot counts, and a pair of `Float` members each return `None` from
    `compare_keys`. As the two members of a `Set`, '
- id: FR-377-AC-7
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: The key order of a pair of different declarations holds as the payload
    of two `Option`s, in a record slot and as an element of two `Sequence` values,
    ordering the enclosing values by the same declarat
- id: FR-377-AC-8
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: '`compare_keys` on unions of one member that hold a `P`, `C` or `G` composite
    at the payload position returns `Some(Less)` for `P` against `C` and `C` against
    `G`, whatever the slots, and `Some(Equal)`'
- id: FR-378-AC-1
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` and `form_grouped` over an element type `Composite(P)` and one
    `P` member and one `C` member complete for `Sequence`, `Set`, `Bag` and `OrderedSet`,
    keep both members unchanged (declaration key'
- id: FR-378-AC-2
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` over a `Set` of those two members charges, in order, `collection.member-walk`,
    `collection.member-test` with one pair, `collection.bound` and `collection.result-retain`,
    and no `collection.elem'
- id: FR-378-AC-3
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted(Composite(P), Some(c))` returns an `Option` whose `payload_type()`
    is `Composite(P)` and whose payload has declaration `C` and its slots unchanged.'
- id: FR-378-AC-4
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted_slots` of a declaration whose declared field type is `Composite(P)`
    and whose slot holds a `C` composite keeps the slot unchanged; `retain_composite`
    charges `composite.result-retain` o'
- id: FR-378-AC-5
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: 'For the same `C` composite at element type `Composite(P)`: `construct_collection`
    refuses `CollectionElementNotAdmitted`; `form_collection` refuses `Element(index)`
    `TypeMismatch`; `ValueType::admits`'
- id: FR-378-AC-6
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: A collection of element type `Composite(C)` is not admitted by `ValueType::admits`
    at the collection type of element type `Composite(P)`, nor the reverse. A stored
    collection clones, compares equal to
- id: FR-378-AC-7
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: The rustdoc of the seven items in Behavior 8 carries the stated text.
- id: FR-378-AC-8
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`UnionValue::from_admitted` of a position typed `Composite(P)` and a `C`
    composite keeps the payload unchanged (declaration `C`, slots unchanged). For
    that `C` composite `union` refuses `Position(0)` '
- id: FR-379-AC-1
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A nullary member has an occurrence count of 1 and a two-position member
    over two integers has 3. The value reports its declaration, `VariantId`, identifier
    and payload in position order. `ValueType::C
- id: FR-379-AC-2
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`union` refuses an arity of 0 against 1 supplied and 2 against 1 supplied
    with `Value` `WrongArity`, and a `Boolean` at an `Integer` position with `Position(0)`
    `TypeMismatch`.'
- id: FR-379-AC-3
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`evaluate_union` returns `Undefined` for the first argument that is undefined,
    runs no later argument and admits no charge. A completed `Boolean` at an `Integer`
    position refuses `DeferredResultNotAdm'
- id: FR-379-AC-4
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Unions of one declaration and member with equal payloads are equal, with
    differing payload positions or members are not, and the pair count is one plus
    the payload pairs.
- id: FR-379-AC-5
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A union against a union of another declaration, an enum member and a composite
    refuses `ValueKindMismatch` in `plan_equality` and gives `None` from `compare_keys`.
    One member with payload lengths 1 an
- id: FR-379-AC-6
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Members `Alpha` and `Zulu` of one declaration order by identifier regardless
    of their payloads; one member orders by the payload positions in order. Declaration
    order does not reach the within-declara
- id: FR-379-AC-7
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`Set`, `Bag`, `OrderedSet` and `Sequence` over four unions, from a supply
    order and its reverse, hold the stated elements. A union with an IEEE payload
    refuses `CanonicalKeyUnavailable` through `form_'
- id: FR-379-AC-8
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: 'Clone, equality, key comparison, `Debug` and drop of a union nested ten
    thousand levels complete on a 512 KiB stack, a mixed union, composite and collection
    nest does the same, storage grows linearly '
- id: FR-379-AC-9
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A present `Option` of declared type `Composite(K)` over a union of declaration
    `K` is admitted, two such `Option` values with equal unions are equal and with
    different members are not, and they compar
- id: FR-380-AC-1
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: With an authenticated B, Integer and reduced Rational membership agree
    with the inclusive existing rule at both endpoints, one outside each endpoint
    and a denominator boundary, without mutating operan
- id: FR-380-AC-2
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: 'Exact-B value and every exact-B endpoint are accepted for comparison;
    one-over-B in each of the three Integer or six Rational operand positions returns
    typed unsupported size naming that position and '
- id: FR-380-AC-3
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Observed production comparison steps do not exceed 2L for Integer or 4L
    for Rational, with fixed-width per-step work and constant auxiliary storage on
    the allocation-free path; source inspection confi
- id: FR-380-AC-4
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: A pre-cancelled original handle prevents input-sized work. Deterministically
    cancelling during a longest admitted comparison returns that cause before the
    next step/result; a fresh-handle mutant fails
- id: FR-380-AC-5
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Inspection identifies whether the production comparison actually performs
    a fallible native conversion or storage request. For each reachable boundary,
    a denied checked conversion is typed capacity/re
- id: FR-380-AC-7
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: 'Inspection of both direct exact-kernel `IntegerInterval::try_contains_bounded`
    and `RationalDomain::try_contains_bounded` signatures verifies that authenticated
    finite B is a required `magnitude_bits:'
- id: FR-381-AC-1
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: With authenticated B, equal and unequal Integer intervals and Rational
    domains return the same Boolean equality as independent endpoint arithmetic, including
    negative, zero, and denominator endpoints,
- id: FR-381-AC-2
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: At B >= 1, every one of the four Integer and eight Rational endpoint positions
    completes at exact B and returns typed unsupported size when made one bit over
    B, before endpoint comparison. Two oversiz
- id: FR-381-AC-3
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: 'Observed production comparisons use no more than 2L Integer or 4L Rational
    limb-pair steps, fixed-width work per step, and the measured temporary-storage
    bound, including inline/Large and 32-bit limb '
- id: FR-381-AC-4
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: A pre-cancelled original handle stops before bound validation, including
    when the first endpoint exceeds B; cancellation on a late observed limb step stops
    before the next step or equality result with
- id: FR-381-AC-5
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: Source inspection establishes whether production performs native-size conversion
    or allocation. For each reachable boundary, an executed denial/retry test establishes
    the typed measured request and re
- id: FR-382-AC-1
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: Equal distinct Decimal types complete `true`; a difference in either signed
    endpoint, either scale or rounding mode completes `false` under authenticated
    `B`. Endpoint equality is mathematical integer
- id: FR-382-AC-2
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: At `B >= 1`, every endpoint position completes at exact `B` and returns
    typed `UnsupportedSize` with the correct position, measured `B+1` bits and `B`
    when one bit over. Two oversized endpoints report
- id: FR-382-AC-3
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: 'Observed production comparisons at inline/Large and 32-bit limb boundaries
    perform at most `2 ceil(B/32)` limb-pair steps of fixed-width work and no input-sized
    clone or allocation; auxiliary storage '
- id: FR-382-AC-4
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: 'A pre-cancelled original handle stops before bound validation, including
    when expected lower exceeds `B`; cancellation observed after a late limb step
    stops before another step or Boolean publication '
- id: FR-383-AC-1
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: '`Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`,
    `cancel()` is `Some(&Cancel)` equal by shared-state identity to `handle`, and
    unequal to an independently created handle. '
- id: FR-383-AC-2
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: 'Cancel the handle obtained from `meter.cancel()` with `Requested`, then
    charge the meter under sufficient limits: the charge is denied at its named point,
    that handle records `tripped() = Some(Request'
- id: FR-383-AC-3
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The accessor's implementation borrows the existing optional handle without
    cloning it or constructing a second `Meter` or `Counters`; calling it changes
    no meter accounting or injection state.
- id: FR-383-AC-4
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles
    in the crate's `no_std` + `alloc` build.
- id: FR-384-AC-1
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Admission accepts every valid Add/Subtract/Negate domain whose complete
    interval image fits signed i128, retains full-i128 ordering, and accepts every
    valid Multiply interval pair whose four operand e
- id: FR-384-AC-2
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: A machine-checked refinement against an independently expressed mathematical
    abstraction of the actual BigInt semantics covers every admitted native payload/operation
    under the operator-specific capac
- id: FR-384-AC-3
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Fresh and cumulative real Meter comparisons against the normative formulas
    and general exact calls agree on completed values, domain refusal, cancellation,
    every named injected denial, all counters, a
- id: FR-384-AC-4
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Source inspection and ordinary generated-client execution demonstrate native
    carriers persist through a multi-operation expression without successful-path
    BigInt/cache/general Integer construction. In
- id: FR-384-AC-5
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Genuine CG TC-027 healthy, TC-027 bound mutant, TC-025 arithmetic mutant,
    IR-917 clause implication and IR-948 V2 contract profiles execute the reviewed
    production admitted API over their original inp
- id: FR-385-AC-1
  path: spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: examined
  excerpt: A Bytes value is admitted only by `ValueType::Bytes` and a JsonObject value
    only by `ValueType::JsonObject`; crossed kinds and Text, Integer, Reference, UUID
    and Timestamp values are rejected by both,
- id: FR-385-AC-2
  path: spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: examined
  excerpt: Bytes payloads the empty text, `00`, `00ff` and `deadbeef` construct; `00FF`,
    `0`, `0x00`, `00 ff`, ` 00` and `gg` return `BytesNoncanonical` with no value
    and no `CheckedInvariant` fault.
- id: FR-385-AC-3
  path: spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: examined
  excerpt: 'JsonObject payloads `{}`, `[]`, `3`, `"x"`, `null`, `{"a":1,"b":[true,null]}`
    and `{"a":9007199254740992}`, the literal-character strings `"é"` (U+00E9), `"\n"`,
    `"\u001f"`, `"\\"`, `"\""` and `"/"`, '
- id: FR-385-AC-4
  path: spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: examined
  excerpt: 'Each same-kind pair follows the one-terminal-pair equality schedule: Bytes
    `00ff` against `00ff` is `true`, against `00fe` and `00ff00` `false`; the JsonObject
    `{"a":1,"b":[true,null]}` against itself'
- id: FR-385-AC-5
  path: spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: examined
  excerpt: 'A direct equality plan pairing Bytes or JsonObject with another kind returns
    `Refusal::CheckedInvariant { cause: ValueKindMismatch }`, and a direct mixed-kind
    `compare_keys` returns `None`.'
- id: FR-385-AC-6
  path: spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: examined
  excerpt: '`compare_keys` over Bytes `02` and `0a` orders `02` first, over `0a` and
    `0a00` orders `0a` first (proper prefix), and over `0200` and `10` orders `0200`
    first, equal payloads have equal keys, and nei'
- id: FR-385-AC-7
  path: spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md
  role: examined
  excerpt: 'Bytes `deadbeef` accounts four variable octets and the JsonObject `{"a":1}`
    accounts seven; existing scalar, composite, UUID and Timestamp admission, equality,
    key and accounting vectors retain their '
- id: FR-386-AC-1
  path: spec/functional/FR-386-map-value-type-with-text-keys.md
  role: examined
  excerpt: '`ValueType::map(MapType::new(V, bound))` returns a `ValueType::Map` whose
    `MapType::values()` equals `V` and whose `MapType::bound()` equals `bound`, for
    `bound` both `None` and `Some`.'
- id: FR-386-AC-2
  path: spec/functional/FR-386-map-value-type-with-text-keys.md
  role: examined
  excerpt: Two map types are equal exactly when both value type and bound are equal.
    Maps differing only in value type, only in bound, and only in bound presence (`None`
    against `Some([0, u64::MAX])`) are unequa
- id: FR-386-AC-3
  path: spec/functional/FR-386-map-value-type-with-text-keys.md
  role: examined
  excerpt: 'A map whose value type is a map, an `Option` or a `Collection` compares
    by its whole nested chain: two such types differing only at the innermost link
    are unequal, and equal chains hash equal.'
- id: FR-386-AC-4
  path: spec/functional/FR-386-map-value-type-with-text-keys.md
  role: examined
  excerpt: On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Map`
    links around `Boolean`, and one of 100,000 links cycling `Map`, `Option` and `Sequence`,
    clone, compare equal to their clones, h
- id: FR-386-AC-5
  path: spec/functional/FR-386-map-value-type-with-text-keys.md
  role: examined
  excerpt: '`Debug` of `Map<Text, Boolean>` and of a bounded `Map<Text, Option<Boolean>>[1,
    3]` prints exactly `Map(MapType { values: Boolean, bound: None })` and `Map(MapType
    { values: Option(Boolean), bound: So'
- id: FR-386-AC-6
  path: spec/functional/FR-386-map-value-type-with-text-keys.md
  role: examined
  excerpt: '`ValueType::admits` on a `Map` type returns `false` for a collection value,
    a text value and an option value.'
- id: FR-386-AC-7
  path: spec/functional/FR-386-map-value-type-with-text-keys.md
  role: examined
  excerpt: '`Debug` in alternate mode (`{:#?}`) of `Map<Text, Boolean>` and of the
    bounded `Map<Text, Option<Boolean>>[1, 3]` prints exactly what `#[derive(Debug)]`
    pretty-printing prints: `Map(\n    MapType {\n '
- id: FR-387-AC-1
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`BoundedQuantityType::new(U, min, max)` returns a type whose `unit()`,
    `min()` and `max()` equal the arguments, for bounds `None`/`None`, `Some`/`None`,
    `None`/`Some` and `Some`/`Some`; `ValueType::bo'
- id: FR-387-AC-2
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`new` returns `Err` with cause `MalformedBoundedQuantityType` for a `min`
    quantity in a unit other than `U` and for a `max` quantity in a unit other than
    `U`, and for no other input; it returns `Ok` f'
- id: FR-387-AC-3
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: Two bounded quantity types are equal exactly when unit, minimum and maximum
    are all equal, and equal types hash equal. Types differing only in unit, only
    in minimum, only in maximum, and only in bound
- id: FR-387-AC-4
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: 'A bounded quantity type nested in `Option`, `Collection` and `Map` compares
    by its whole chain: two chains differing only in the innermost bound are unequal
    and equal chains hash equal.'
- id: FR-387-AC-5
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`
    links around a bounded quantity type clones, compares equal to its clone, hashes
    equal to its clone, formats for debug with b
- id: FR-387-AC-6
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`Debug` of a bounded quantity type over unit `U` with `min` 0 and `max`
    100 prints exactly what `#[derive(Debug)]` prints for `BoundedQuantity(BoundedQuantityType
    { unit: .., min: Some(..), max: Some('
- id: FR-387-AC-7
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: 'At a `BoundedQuantity` type over `U` with effective bounds `[0, 100]`,
    `admits` returns `true` (admitted) for in-range magnitudes: for quantities of
    `U` with magnitudes 0, 50 and 100 (both bounds incl'
- id: FR-387-AC-9
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`BoundedQuantityType::new` is the only constructor refusal: it raises
    no `invalid_model_binding`, `ill_typed`/`type-mismatch` or run-time invariant
    cause. Its sole cause is `MalformedBoundedQuantityTy'
- id: FR-387-AC-10
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: A `BoundedQuantity` type over `U` with `min` 5 and `max` 3 is constructed
    and `admits` returns `false` for quantities of `U` with magnitudes 2, 3, 4, 5
    and 100, for a magnitude 4 quantity in another u
- id: FR-387-AC-8
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: At a `BoundedQuantity` type over `U`, `admits` returns `false` for a quantity
    of magnitude 50 in a different unit, for a `Text` value, a `Boolean` value and
    an `Option` value; `ValueType::Quantity(U)`
- id: FR-388-AC-1
  path: spec/functional/FR-388-read-closed-limit-kind-spellings.md
  role: examined
  excerpt: For every member of `LimitKind::ALL`, the public reader returns `Some(kind)`
    for `kind.as_str()`; each member maps back to itself, including the first and
    last members.
- id: FR-388-AC-2
  path: spec/functional/FR-388-read-closed-limit-kind-spellings.md
  role: examined
  excerpt: The public reader returns `None` for an empty string, an unknown name,
    a case-changed member name, a member name with leading or trailing whitespace,
    and a member name with a malformed separator; no i
- id: FR-388-AC-3
  path: spec/functional/FR-388-read-closed-limit-kind-spellings.md
  role: examined
  excerpt: Against the authoritative QSpec `ScalarLimitsV1` full-vocabulary vector,
    `LimitKind::ALL` has exactly the same members in the same field order; each vector
    spelling is returned by `as_str()` and accep
- id: FR-388-AC-4
  path: spec/functional/FR-388-read-closed-limit-kind-spellings.md
  role: examined
  excerpt: Inspection of every successful `from_code` return path shows that it returns
    `Some(kind)` only after the input exactly equals `kind.as_str()` for a member
    of `LimitKind::ALL`; there is no catch-all su
- id: FR-389-AC-1
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: Two `NodeKey`s built independently from equal admitted `{package, path,
    ordinal}` components compare equal, hash equally under one hasher and have `cmp
    == Equal`; changing only the package, a path seg
- id: FR-389-AC-2
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: A non-empty package and path with an ordinal of 0 or 9007199254740991 construct
    a `NodeKey` even if the caller has not asked the kernel to resolve that key against
    a package. An empty package, empty p
- id: FR-389-AC-3
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: Two `EffectiveId`s made from equal `{owner_effective_type, original}` values
    compare and hash equally, including a `null` owner for an effective type and a
    recursive owning `EffectiveId` for a member.
- id: FR-389-AC-4
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: 'Enum, union and model-variant `VariantId`s each carry only the admitted
    `{owner: NodeId, case: identifier}` pair. Equal pairs compare, hash and order
    equally; changing the owner or case distinguishes '
- id: FR-389-AC-5
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: '`MemberId`s with equal `{owner, member}` components compare, hash and
    order equally; changing the owner arm, owner value or exact member identifier
    distinguishes them, with `node_id` arm before `effec'
- id: FR-389-AC-6
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: A declared `UnitId` constructed from a unit node and a compound `UnitId`
    constructed from caller-admitted canonical terms are unequal across their domains,
    with `declared` ordered before `terms`. Equa
- id: FR-389-AC-7
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: 'Every identity constructor and public read path of this requirement uses
    admitted structural components; the public API has no `from_digest`, `NodeKey::decode_admitted`,
    fixed digest byte accessor or '
- id: FR-389-AC-8
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: For values built from the admitted structural identities, FR-088-AC-11/12,
    FR-089-AC-6, FR-377-AC-1 through AC-8 and FR-379-AC-1 through AC-9 keep their
    stated result, refusal, pair count and collecti
- id: FR-389-AC-9
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: For a `NodeKey` with mixed text and nested-node path segments, a public
    borrowed view yields the exact package and ordinal and each direct segment once
    in component order. A text item borrows its orig
- id: FR-389-AC-10
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: A consumer can traverse at least 10,000 nested node levels using the borrowed
    views and an explicit fallibly reserved stack, producing the same component sequence
    as the structural identity without ca
- id: FR-389-AC-11
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: Given an already owned `String` package and `Vec<PathSegment>` path, `NodeKey::try_new`
    accepts the same package, path and ordinal shape as `NodeKey::new`, including
    the boundary ordinals, and produce
- id: FR-389-AC-12
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: '`NodeKey::try_clone` and `PathSegment::try_clone` reproduce every component
    of text and nested-node inputs, including 10,000-level paths, with equal hash
    and order where applicable and bounded call-st'
- id: FR-389-AC-13
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: For independently constructed equal and differing `NodeKey`s, `cmp` and
    `partial_cmp` each perform zero heap allocations on the calling thread and return
    the same QSpec FR-322 order in both directions
- id: FR-389-AC-14
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: Given a prebuilt operand `NodeKey`, constructing a derived key with `PathSegment::Node(operand)`
    and cloning either key at 1,000 and 10,000 operand levels performs work and additional
    allocation bound
- id: FR-390-AC-1
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '`try_clone` returns an equal, independently owned value for zero, a small
    positive and negative value, and positive and negative values larger than one
    machine limb; changing or dropping the returned '
- id: FR-390-AC-2
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '`try_to_decimal_string` returns `0`, `1`, `-1` and independently supplied
    large positive and negative decimal literals byte for byte, matching `Display`
    on success without a leading plus, leading zero'
- id: FR-390-AC-3
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: 'For each successful spelling in AC-2, a bound equal to its actual UTF-8
    length permits output, and a bound one byte smaller returns `ByteLimit { required:
    actual_length, limit }` with no output. The m'
- id: FR-390-AC-4
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: With the same prepared integers and a sufficient bound, injected refusal
    at every actual decimal scratch or output reservation yields `Allocation { requested
    }`, no partial output and an unchanged sou
- id: FR-390-AC-5
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: A consumer can compile a call to both public fallible operations without
    consuming its `Integer`; copy and decimal failures are typed so that allocation
    can be mapped with its actual `requested` amoun
- id: FR-391-AC-1
  path: spec/functional/FR-391-fallible-effective-identity-copy.md
  role: examined
  excerpt: A root-only `EffectiveId` and a member identity with at least 10,000 owner
    links each copy successfully through the public API. The result is separately
    owned, equals and hashes like the source, retai
- id: FR-391-AC-2
  path: spec/functional/FR-391-fallible-effective-identity-copy.md
  role: examined
  excerpt: A crate-internal reservation-failure control, after the caller policy has
    permitted the request, denies each actual reservation in a prepared `EffectiveId`
    whose chain, package, and node strings are n
- id: FR-391-AC-3
  path: spec/functional/FR-391-fallible-effective-identity-copy.md
  role: examined
  excerpt: 'The same checked-count calculation used by the production `EffectiveId`
    chain reservation is exercised in a crate test with synthetic counts at the exact
    representable multiplication boundary and one '
- id: FR-391-AC-4
  path: spec/functional/FR-391-fallible-effective-identity-copy.md
  role: examined
  excerpt: 'A consumer can match the public copy error without inspecting text: a
    representable denied request maps to `EnvironmentFailure::Allocation(StorageRequest::new(requested,
    StorageUnit::Bytes))`, and ove'
- id: FR-391-AC-5
  path: spec/functional/FR-391-fallible-effective-identity-copy.md
  role: examined
  excerpt: A public policy-aware `EffectiveId` clone and a public policy-aware `NodeKey`
    clone each invoke a caller callback immediately before every actual representable
    chain, path, package, or text reservatio
- id: FR-392-AC-1
  path: spec/functional/FR-392-typed-collection-state-boundary.md
  role: examined
  excerpt: From outside the crate, a caller builds `Sequence<Integer>[1,3]` from `[2,2,1]`
    with the checked path and reads the same three elements in the same order; `OrderedSet<Integer>[1,3]`
    from `[2,2,1]` rea
- id: FR-392-AC-2
  path: spec/functional/FR-392-typed-collection-state-boundary.md
  role: examined
  excerpt: For reachable counts 1, 2, 3, 4 and 5 under the inclusive bound `[2,4]`,
    checked prestate construction counts occurrences for `Sequence` and retained members
    for `OrderedSet`; counts 1 and 5 refuse be
- id: FR-392-AC-3
  path: spec/functional/FR-392-typed-collection-state-boundary.md
  role: examined
  excerpt: A typed prestate with an element outside the exact declared element type
    refuses at `Component::Element(index)` with `ConstructionCause::TypeMismatch`
    before collection membership or bound work; an el
- id: FR-392-AC-4
  path: spec/functional/FR-392-typed-collection-state-boundary.md
  role: examined
  excerpt: With a caller-authenticated schema-aware admission authority, the public
    poststate inspector yields a borrowed view of an exactly typed sequence or ordered
    set, exposing the declared type and elements
- id: FR-392-AC-5
  path: spec/functional/FR-392-typed-collection-state-boundary.md
  role: examined
  excerpt: Public `from_admitted` values with a wrong element, out-of-bound count,
    or duplicate ordered-set member are refused by poststate inspection at the offending
    element or bound, without silent normalizat
- id: FR-392-AC-6
  path: spec/functional/FR-392-typed-collection-state-boundary.md
  role: examined
  excerpt: Inspection of an admitted sequence preserves stored duplicates and order;
    inspection of an admitted ordered set checks pairwise uniqueness and preserves
    stored order, while the builder establishes fir
- id: FR-392-AC-7
  path: spec/functional/FR-392-typed-collection-state-boundary.md
  role: examined
  excerpt: The public API exposes one collection representation (`Value::Collection`
    and `CollectionType`) for the prestate and poststate; no parallel RT collection
    type or generated-function ABI is defined by t
```

## Dispositions

Round 1: dispatched lead explicitly authorized deferral of the unchanged baseline obligations. None is accepted as satisfied. All original finding text remains unchanged. Spec statements are identical at base and head; this PR only adds test code and removes no pre-existing tags, so no untagged criterion is introduced.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-002 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-003 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-004 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-005 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-006 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-007 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-008 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-009 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-010 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-011 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-012 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
| FND-013 | deferred | Unchanged baseline 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae obligation outside the user/Luna-approved IR-1268 native IEEE helper slice. Deferral is scope allocation, not satisfaction, waiver or repository assurance. Lead will check existing tracked owners; no ticket or coverage is fabricated. Parked proofs remain prohibited and IR-722 root proof remains unresolved. |
