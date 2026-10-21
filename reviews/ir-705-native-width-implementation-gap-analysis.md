---
id: SR-14801
title: IR-705 native-width implementation gap-analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; IR-705; whole repository spec/, src/, tests/"
review_set: subset
---

## Summary

Whole-repository planless matrix, reverse ownership, and stub audit of all33 FR owners and188 acceptance criteria, including clean units. Candidate and separately archived published7ff baseline were computed by the same installed engine. Criterion IDs, full statements, methods and statuses are identical. Three new correctly resolved native-width test bindings add evidence without changing the acceptance set.

Ticket: IR-705. Frozen source head: 888d5a5c8ce590b75954d11d1b972e0ad0d3d23c; published base: 7ff882c27849dc52a92e88cc8f54bc8b0f3c5bac. Independent reviewer, model gpt-6.1-sol, invocation 9491716e-a8e9-421b-9a5a-ff8f4e7cea9f. Read AGENTS/CLAUDE and applicable code-review/Rust/gap skills. No applicable AssuranceProfile under spec/. This review performs static source analysis and Quire computations only; no Cargo/rustc/formatter/metadata/native/backend or aggregate execution. No product edits or push. Findings go only to private Linear, not the public GitHub repository. No spec/plan change in PR70, so spec-review and sub-analyses do not apply.

## Verdict

**FAIL** for whole-repository strict traceability: three untagged criteria and one ignored-only criterion. All four findings are inherited unchanged from the actual published7ff baseline. There are no new gaps or wrong bindings established by this change. Earlier PR68/69 authority does not dispose findings on this PR70: a current scoped owner decision and explicit dispositions are still required. This review does not close IR-705/IR-17 or claim any memory improvement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The standalone Meter criterion is bound only to its explicitly ignored dedicated Kani wrapper. Strict traceability remains non-green despite separate gate delivery; native-width tests do not supply this proof evidence. | spec/functional/FR-359-cumulative-meter-boundary.md:39 |
| FND-002 | medium | The computed matrix has no local binding or delivered current five-test external consumer receipt. Genuine IR-17/IR-705 consumer qualification remains outstanding; native-width refinement tests cannot replace it. | spec/functional/FR-359-cumulative-meter-boundary.md:40 |
| FND-003 | medium | The computed matrix has no binding or completed original consumer healthy and both substantive mutant qualification. Existing OOM measurements are inconclusive; native-width success is not consumer Kani acceptance. | spec/functional/FR-371-bounded-integer-representation-refinement.md:86 |
| FND-004 | low | The no_std build criterion has no executable-symbol binding in the computed matrix. This inherited verification-method discrepancy remains separate from implementation failure and the native-width change. | spec/functional/FR-383-meter-cancellation-handle-access.md:50 |

## Coverage

Plan completion: not assessed

Actual Quire candidate and independent git-archive baseline matrix:188 criteria across33 requirement owners;166 tagged,18 declared Inspection method-without-symbol,3 untagged,1 ignored-only. All188 full statements, IDs, methods and statuses compare identical. Resolved unique test/AC bindings rise253 to256, exactly the three new imported-attribute test functions; no binding removed. Strict exits1 with exactly3 untagged and1 ignored-only. Scope records all188 criteria mechanically examined. Binding entries record declaration-to-existing-criterion resolution, not an unsupported repository-wide semantic or execution certification. No coverage percentage is claimed.

Whole-tree inventory covers27 source Rust files and19 test/fixture Rust files, their module ownership docs and curated root facade, public declaration inventory and TODO/FIXME/todo!/unimplemented!/ignored-marker scans. Kernel foundational QSpec-owned identity/value/outcome/location/reference/collection behavior has explicit module/spec ownership; local FRs own accounting, cancellation, division, exact scalar operations, type/membership/equality, typed refusal, native representations, enum/unit and composite/union paths. All four changed operations map to FR-371's existing reviewed native-width architecture and FR-362 exact arithmetic. No newly unspecified behavior, confirmed concrete source stub or confirmed test stub was found. This is ownership/mechanical inspection, not the optional full semantic audit.

Actual coverage reports86 untracked symbols,34 unmatched tags and6 heuristic suspicions. These are inspected candidates, not86 established missing requirements. Cross-repo QSpec/TC and UTF-8 references do not mint local ACs. All3 added trace functions bind correctly; no new unmatched qualified attribute. The six suspicions were inspected: identity macro executes five unconditional instantiations; fitting checked-i128 test assertions are limited intentionally to fitting references and paired with actual beyond-i128 tests; Decimal Some/None branches both assert outcomes; three key-helper comparisons concern fixture identity construction, not copied production equality. No actual hollow evidence was established. Existing ignored Meter test names its real dedicated lane and checks the real runner status; delivered separate gate does not erase strict ignored-only status.

The four inherited findings remain owned by IR-810 (dedicated Meter lane), IR-17/IR-705 (five consumer tests and original healthy/two-mutant qualification), and IR-895 (no_std verification-method discrepancy). Ownership context is not a current deferral decision; no current dispositions are silently imported. No tests, ignored markers, input/solver/resource ceilings or criteria are weakened. No plan selected, created or audited. Optional whole-repository semantic review was not requested and is skipped; focused changed spec/code/test agreement is in SR-14800. Ambient duplicate-archetype/inverse-edge diagnostics did not prevent JSON computation. Full aggregate, genuine original consumer controls and separate IR-917 fixture measurement remain outstanding. Original76516GiB and40GiB OOM results remain inconclusive and protected.

## Review data

```yaml
clean: false
scope:
- id: FR-088-AC-11
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: For an enum shape whose canonical member list is `b`, `a`, `c`, the ranks of the variants are
    `b` 0, `a` 1, `c` 2, and a set of all three values visit
- id: FR-088-AC-12
  path: spec/functional/FR-088-enum-and-unit-identity-in-the-kernel.md
  role: examined
  excerpt: 'A declared unit `UnitId` and a compound unit `UnitId` are unequal when their domains or bytes
    differ. A declared unit''s `UnitId` carries its node key '
- id: FR-089-AC-2
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: The kernel `Value::Population` variant's payload type is `PopulationId`; no kernel source file
    imports a population binding or any other caller's mode
- id: FR-089-AC-6
  path: spec/functional/FR-089-kernel-refuses-a-population-pair.md
  role: examined
  excerpt: Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`, kernel
    `ValueType::admits` returns false; given two `Value::Pop
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
  excerpt: '`Refusal::CheckedInvariant` carries only FR-369''s closed typed cause; it has no arbitrary
    caller message, QSL model/checker type, catalog code or gene'
- id: FR-096-AC-8
  path: spec/functional/FR-096-kernel-refusal-code-and-cause.md
  role: examined
  excerpt: 'Each catalogued kernel refusal below returns the code and cause spelling named for it from
    `Refusal::code()` and `Refusal::cause()`, and every FR-369 '
- id: FR-097-AC-7
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: An unbounded collection type admits a collection value of any size with no cardinality refusal;
    it still charges `collection.bound`, and stops with `I
- id: FR-097-AC-8
  path: spec/functional/FR-097-unbounded-collection-in-the-kernel.md
  role: examined
  excerpt: 'A caller outside the crate can call `CardinalityBound::violation(count)` and receive the existing
    `Option<BoundViolation>` type. For `[2, 4]`, counts '
- id: FR-262-AC-2
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: 'On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s around `Boolean`
    clones, compares equal to its clone, hashes equal to its '
- id: FR-262-AC-3
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: examined
  excerpt: A `Value` nested N levels deep, alternating `Option` and `Sequence` levels around an `Integer`
    with each level declared with the type of its depth, ho
- id: FR-357-AC-1
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a zero divisor under any profile and either member, `divide` returns `Undefined(DivisionByZero)`.
- id: FR-357-AC-2
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `10 div y` over `1..=10` at `y = 5`, `divide` of the quotient returns 2; given `x rem
    -1` over `-10..=5` at `x = -10`, `divide` of the remainder
- id: FR-357-AC-3
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a quotient outside the domain, `divide` refuses with cause `quotient-outside-domain`;
    given a remainder outside the domain, it refuses with `rem
- id: FR-357-AC-4
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given -7 and 2, truncating returns (-3, -1), floor (-4, 1) and Euclidean (-4, 1); given -7
    and -2, truncating returns (3, -1), floor (3, -1) and Eucli
- id: FR-357-AC-5
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given `i64::MIN` and -1 under any profile, `divide` of the quotient returns 2^63 in a domain
    that holds it. In a signed 64-bit domain the quotient ref
- id: FR-357-AC-6
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: Given a successful `div` or `rem`, the meter admits `integer-division.operands`, `integer-division.arithmetic`,
    `integer-division.domain` (one value o
- id: FR-357-AC-7
  path: spec/functional/FR-357-single-member-integer-division.md
  role: examined
  excerpt: For dividend/divisor pairs `(7, 3)`, `(7, -3)`, `(-7, 3)` and `(-7, -3)`, public `modulo` in
    the mathematical domain completes to 1, 1, 2 and 2 respec
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit P charge under
    an ordinary limit, then admit P once and an unrelated po
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan` returns
    `Incomplete` at `equality.plan` without changing any consu
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction with zero
    is rejected by that type, while a nonzero occurrence const
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor returns all 4096 records
    with their points in order and `charge_log_truncated
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()` and consumed work
    units reflect all 4097 admissions, including the one omi
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge log and its `Counters`
    accounting state remains fixed-size and heap-free.
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one result unit. The log
    retains the first 4096 points and reports truncation, wh
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`, admit 4097 one-work-unit
    charges and request one more work unit. The charge r
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`, admit 4097 charges
    each consuming one result unit, then request one more resu
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial for one named point.
    A charge at that point returns `Incomplete` with `WorkU
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After admitting two work units, inject occurrence 1 at point P and attempt a three-work-unit
    `charge` at P under otherwise sufficient limits whose con
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With an occurrence-1 injected denial selected, make the same `charge` independently fail an
    ordinary semantic-size counter and make a `charge_plan` wi
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request of 9 followed by 3
    and then a charge with no size request. The records expose
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently chosen semantic-size
    requests, including repeated same-kind requests and dis
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one
    work unit reaches `u64::MAX` exactly. The next one-unit cha
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting `u64::MAX` result
    units. Its work consumption is 1. A second charge requestin
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount is `u64::MAX +
    1` returns `Incomplete` with `limit_kind = WorkUnits` and `
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit and `u64::MAX + 1` result
    units returns `Incomplete` with `limit_kind = ResultU
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least
    16 bits, `u64::MAX` work units and one result unit. Th
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters the first unavailable
    counter of a charge, including several simultaneously s
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers without panic for every
    member of `LimitKind::ALL`: untouched counters are zero,'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'Reordered and repeated requests preserve the admitted high-water maximum observable through
    `consumed()`; the diagnostic log record is verified under '
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production
    `Meter::charge` for the exact three-charge FR-362 schedu
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their
    own assertions under their original domains, oracle, comp
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and called helpers finds
    no traversal with a trip count greater than that charge'
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362 schedule contains
    exactly the admitted prefix in charge order, with each rec
- id: FR-360-AC-1
  path: spec/functional/FR-360-integer-minimum-magnitude.md
  role: examined
  excerpt: '`Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808 (`2^63`),
    is nonnegative, and differs from the original negative'
- id: FR-361-AC-1
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: 'For an integer multiplication with a large operand, set `integer_bits` to one below the exact
    `integer-arithmetic.arithmetic` amount while permitting '
- id: FR-361-AC-2
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a rational operation with a large numerator or denominator, set `integer_bits` to one below
    the exact `rational-arithmetic.arithmetic` amount whil
- id: FR-361-AC-3
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Inject a one-shot denial at each of `integer-division.arithmetic` and `integer-modulus.arithmetic`
    for large nonzero operands. Each returns `Incomplet
- id: FR-361-AC-4
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For valid decimal operands requiring a `2^20`-place scale expansion, set `scale_expansion`
    to one below the planned shift and leave other limits suffi
- id: FR-361-AC-5
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For decimal addition of small coefficients at scales 0 and `2^20`, set `integer_bits` to the
    admitted scale-expansion bit amount, one below the planne
- id: FR-361-AC-6
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: For a decimal value retained at a target scale requiring a `2^20`-place power of ten, set `decimal_digits`
    one below the exact result-retain amount wh
- id: FR-361-AC-7
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: On a fresh meter, floor division of a positive at-least-65,536-bit dividend by 3, exposing
    the quotient in the mathematical domain, completes to the i
- id: FR-361-AC-8
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: 'For that floor quotient fixture, lower the `integer_bits` limit by one. The call returns `Incomplete`
    at `integer-division.operands` with `limit_kind '
- id: FR-361-AC-9
  path: spec/functional/FR-361-admission-before-large-exact-work.md
  role: examined
  excerpt: Multiply retained decimals `(1, 0)` and `(1, 0)` into an Exact target with coefficient bounds
    0 through 1 and scales 0 through `2^20`. At `decimal_dig
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`;
    for negate, use each member as a unary operand. Com
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently
    authored decimal value `340282366920938463463374607
- id: FR-362-AC-3
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For fixed and reproducibly sampled rational pairs whose checked `i128` reference intermediates
    fit and whose denominators are nonzero, rational add, s
- id: FR-362-AC-4
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Integer and rational comparisons over fixed and reproducibly sampled pairs equal independently
    checked numeric comparisons for equality and order. Dec
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For a successful representative of each integer arithmetic operation, each rational arithmetic
    operation, integer ordering and rational ordering, the '
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate the maximum size amount
    across their points from the formulas in Behavior, us
- id: FR-362-AC-7
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'On one meter, directly add integers 5 and 7, compare rationals 1/2 and 2/3 for less-than,
    then retain an already-decided Boolean `true`. The admitted '
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer arithmetic, rational
    arithmetic, integer ordering, rational ordering and alread
- id: FR-362-AC-9
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For every already-decided Boolean connective, the completed value matches its truth table and
    the kernel admits `boolean.result-retain` once, consumin
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and
    negate, and integer/rational ordering, independently calc
- id: FR-362-AC-11
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`, typed domain
    `Refused`, and `Incomplete` with no partial value; `comp
- id: FR-362-AC-12
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: '`Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator 1 and denominator
    2, proving reduction to lowest terms and cancellation of two'
- id: FR-362-AC-13
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)`
    is false. Dividing `1/1` by `3/1` under that domain
- id: FR-362-AC-14
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Call the five scalar atoms `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0` directly on one meter.
    They complete to true, 1, true, 0, false in order, consu
- id: FR-362-AC-15
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Directly divide rational `3/1` by `2/1` on a fresh meter. The result is `3/2`, with high-water
    `integer_bits = 3`, value occurrences 2, work 4 and res
- id: FR-362-AC-16
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On one meter directly add integers `0 + 1` and then `1 + 2`. The results are 1 and 3; the integer
    point sequence occurs twice, high-water `integer_bit
- id: FR-362-AC-17
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain` after operands
    and arithmetic, with zero result units; work limit 3 comp
- id: FR-362-AC-18
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For `k` in 63, 64, 65, 127, 128, 200 and 511, compare exact results and charged bit high-water
    sizes around `2^k`: `(2^k - 1)(2^k + 1)` charges `2k + '
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add
    `8 + 3` and `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On fresh sufficient public meters under `test-support`, evaluate the six rational calls listed
    below with no result domain. Each completes to the list
- id: FR-362-AC-21
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On a fresh public meter, a direct default charge at `TemporalVisit` (`temporal.visit`) succeeds
    with `work_units = 1`, `result_units = 0` and no seman
- id: FR-362-AC-22
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: A one-shot injected denial of the first direct charge at `TemporalVisit` (`temporal.visit`)
    returns `Incomplete` naming `TemporalVisit` and `WorkUnits
- id: FR-363-AC-1
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For decimal pairs with equal values but different retained scales, and for positive, negative
    and zero values at differing scales, `order_numbers` agr
- id: FR-363-AC-2
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For a successful decimal comparison, the meter admits `ordering.operands`, `ordering.arithmetic`,
    `ordering.result-retain` in that order. On a fresh m
- id: FR-363-AC-3
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: Comparing retained `(0, 5) >= (0, 0)` completes `true` and consumes `integer_bits = 18`, `decimal_digits
    = 6`, `scale_expansion = 5`, `value_occurrenc
- id: FR-363-AC-4
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: With all limits at `u64::MAX` except `decimal_digits = 64`, testing retained `(1, 0) < (1,
    u32::MAX)` returns `Incomplete` at `ordering.arithmetic` wi
- id: FR-363-AC-5
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: 'On separate fresh meters with generous limits, inject a one-shot denial at occurrence 1 of
    each decimal-ordering point: `ordering.operands`, `ordering'
- id: FR-363-AC-6
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: For retained decimals `(110, 2)` and `(11, 1)`, public unmetered `Decimal::compare` returns
    `Equal`, `numerically_equal` returns true and both normali
- id: FR-363-AC-7
  path: spec/functional/FR-363-metered-decimal-ordering.md
  role: examined
  excerpt: On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` both complete false.
    The mixed pair consumes integer bits, decimal digits and s
- id: FR-364-AC-1
  path: spec/functional/FR-364-ieee-flag-iteration.md
  role: examined
  excerpt: For every subset of the five public flags, collect it into `IeeeFlags` in forward and reverse
    vocabulary order. Both `iter()` results equal that subse
- id: FR-365-AC-1
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: Round retained `(5, 1)` and `(-5, 1)` to a scale-zero target with coefficient bounds -10 through
    10 under each of the six modes. Values or `Refused(In
- id: FR-365-AC-2
  path: spec/functional/FR-365-decimal-rounding-ties-and-default.md
  role: examined
  excerpt: '`RoundingMode::default()` and the mode read back from a target constructed with it are `Exact`.
    Rounding retained `(1, 1)` to that scale-zero target r'
- id: FR-366-AC-1
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For same-width Add with a positive quiet NaN of payload 5 and a negative signaling NaN of payload
    9 in either operand order, the result bits are the f
- id: FR-366-AC-2
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: 'Converting a binary64 NaN with payload `2^22` or `2^30` to binary32 returns `Refused(IeeeNanPayloadNotRepresentable
    { source: Binary64, target: Binary'
- id: FR-366-AC-3
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: Converting either signed binary32 zero to the rational domain with numerator interval `[-1,
    1]` and denominator interval `[1, 1]` completes to canonic
- id: FR-366-AC-4
  path: spec/functional/FR-366-ieee-exceptional-value-semantics.md
  role: examined
  excerpt: For each width, distinct bit patterns representing the ten classes in Behavior have strictly
    increasing `total_order_key()` values in that stated orde
- id: FR-367-AC-1
  path: spec/functional/FR-367-text-profile-mismatch-before-charging.md
  role: examined
  excerpt: Admit the same UTF-8 text "a" once with Nfc and once with BinaryUtf8 using separate sufficient
    meters. On a fresh comparison meter whose ten limits ar
- id: FR-368-AC-1
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`, has a unique
    documented spelling from `as_str()` and round-trips through
- id: FR-368-AC-2
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`, has a unique documented
    `as_str()` spelling, and the sequence is exactly the
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose
    closed variant set and four typed `IllTypedCause` payloads
- id: FR-369-AC-2
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: Every kernel producer in collection, value, division and equality returns exactly its table
    cause; two population values return `PopulationPair`, a po
- id: FR-369-AC-5
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`,
    and `Eq` distinguishes otherwise equal refusals whose
- id: FR-369-AC-6
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: 'Every received typed cause follows QSL''s existing internal-fault path: no refusal record
    or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-i'
- id: FR-369-AC-8
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT
    and QSV own no second refusal enum or compatibility shim,
- id: FR-370-AC-1
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: A UUID value is admitted only by `ValueType::Uuid` and a Timestamp value only by `ValueType::Timestamp`;
    crossed native kinds and Integer, Text and Re
- id: FR-370-AC-2
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'Each same-kind native pair follows the existing one-terminal-pair equality schedule: an equal
    pair yields `true`, an unequal pair yields `false`, and '
- id: FR-370-AC-3
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: 'A direct equality plan pairing UUID or Timestamp with another value kind returns `Refusal::CheckedInvariant
    { cause: ValueKindMismatch }`, while a dir'
- id: FR-370-AC-4
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: '`compare_keys` gives each native type a total key consistent with its equality relation and
    ordered by canonical payload ASCII bytes; Timestamp `"10"`'
- id: FR-370-AC-5
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Existing scalar and composite admission, equality, key comparison and accounting vectors retain
    their prior results.
- id: FR-370-AC-6
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: UUID values preserve exactly 16 network-order octets, including Nil, Max and non-RFC-version
    bit patterns; only the lowercase ASCII `8-4-4-4-12` calle
- id: FR-370-AC-7
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Timestamp values preserve signed `i128` POSIX-epoch nanoseconds, including inclusive bounds
    `-170141183460469231731687303715884105728` and `1701411834
- id: FR-370-AC-8
  path: spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md
  role: examined
  excerpt: Kernel constructors accept canonical UUID or Timestamp text supplied after the caller has validated
    its tagged wire envelope. Each malformed text retu
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve
    the exact value for fixed boundaries and reproducibly sa
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain
    equal, identically ordered, identically hashed with t
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters
    on native, native-overflow and arbitrary-size cases. Test
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate,
    magnitude, to_u64 and interval paths without '
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Public signatures and all existing Integer methods remain available, except that the divisor
    parameters of Integer::exact_div, Integer::div_rem_trunca
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original
    input/domain bounds, oracle, unwind3, Cadical, 600-s
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the authoritative membership rule for inclusive lower/upper equality,
    adjacent outsiders, negative and zero values, scale refu
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "Production helper observations on admitted `B,S` fixtures stay within the one-request `4 \xD7\
    \ L_B`-byte scratch envelope and `2 \xD7 (min(S,B)+2) \xD7 L_B` lim"
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: 'Denying the actual scratch reservation reports its actual native request as typed allocation
    failure. A separate checked-capacity failure retains its '
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation observed during the
    longest admitted division/comparison path returns the orig
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder and sign invariants,
    exact reservation boundary, checked layout conversions, al
- id: FR-373-AC-1
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: The public crate-root `ieee_to_exact` has the exact direct signature in Behavior. `IeeeExactTarget`
    and its re-export are absent; no compatibility ent
- id: FR-373-AC-2
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'Binary32 `0x3fc00000` and binary64 `0x3ff8000000000000` complete as `3/2` with no loss in
    numerator domain `[-3,3]`, denominator domain `[1,2]`. Both '
- id: FR-373-AC-3
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Quiet NaN, signaling NaN and positive/negative infinity at each width return exactly `Undefined(IeeeNotFinite)`
    after one operand charge, one work uni
- id: FR-373-AC-4
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Converting `3/2` to numerator domain `[-1,1]`, denominator domain `[1,2]`, and separately to
    numerator domain `[-3,3]`, denominator domain `[1,1]`, re
- id: FR-373-AC-5
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'Successful finite conversion admits exactly operands, intermediate and retain in that order,
    consuming three work units and one result unit. Ordinary '
- id: FR-373-AC-6
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: For Requested and Deadline, pre-cancelling the original handle attached to the supplied meter
    returns WorkUnits incomplete at operands with limit = co
- id: FR-373-AC-7
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Precharge the supplied meter once at FunctionCall with work 2, result 1, IntegerBits 80, ValueOccurrences
    5 and TextScalars 7. Direct conversion of ei
- id: FR-374-AC-1
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Admission of zero constructed by primitive, canonical parse and zero-producing arithmetic returns
    exactly `Err(ZeroDivisor)` and no token. Positive an
- id: FR-374-AC-2
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: An external public-API caller can reuse one borrowed copyable token across all three helpers.
    A plain `&Integer` cannot satisfy any of the three divis
- id: FR-374-AC-3
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct truncating/floor calls on `(7,2)`, `(7,-2)`, `(-7,2)` and `(-7,-2)` return respectively
    truncating `(3,1)`, `(-3,1)`, `(-3,-1)`, `(3,-1)` and f
- id: FR-374-AC-4
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Direct exact_div on every sign combination of dividend 6 and divisor 2 returns the signed quotient
    3; zero dividend with either divisor sign returns z
- id: FR-374-AC-5
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: Inspection accounts for every exact-owned production caller in Rational, Decimal and division.
    Canonical callers establish the documented nonzero inva
- id: FR-374-AC-6
  path: spec/functional/FR-374-nonzero-integer-divisor-admission.md
  role: examined
  excerpt: With sufficient real meters, zero divisor under all three divide profiles and both members,
    plus public modulo, returns `Undefined(DivisionByZero)` af
- id: FR-375-AC-7
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a
    focused tool control demonstrates both a passing and a del
- id: FR-375-AC-9
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: On equal and first-head-unequal bounded kernel operands, `planned_equality_with_plan` returns
    the Boolean relation and 17 planned pair events from the
- id: FR-375-AC-10
  path: spec/functional/FR-375-composable-equality-effect-seam.md
  role: examined
  excerpt: When `Meter::charge` refuses the 17th pair, the result-bearing kernel operation returns the
    typed stop with exactly the prior 16 pair charges admitted
- id: FR-376-AC-1
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`plan_equality`, `planned_equality` and `planned_equality_with_plan` on two composites whose
    declaration keys differ return `false`, with a pair count'
- id: FR-376-AC-2
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: On that pair, `planned_equality` admits `equality.plan-form`, `equality.plan`, exactly one
    `equality.pair` and `equality.result-retain`, in that order
- id: FR-376-AC-3
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite pair of different declarations whose slots hold `Reference` values of different
    universes returns `false` and does not report `foreign_ref
- id: FR-376-AC-4
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: Two composites with equal declaration keys and equal slots give `true`. Equal declaration keys
    with a differing slot give `false`. The pair counts are
- id: FR-376-AC-5
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A pair of different declarations that sits as the payload of two present `Option`s, in a one-slot
    record, as the element of two one-element `Sequence`
- id: FR-376-AC-6
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, `Option`, `Collection` or `Reference`, in either order, still
    refuses `ValueKindMismatch`. Two composites of one dec
- id: FR-376-AC-7
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`member_equal` of a composite candidate against a member of a different declaration returns
    `false`, charging `collection.member-walk` then `collectio'
- id: FR-376-AC-8
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: '`form` over an `OrderedSet` type whose element type is a declaration P, with occurrences of
    one composite of declaration C then one of P, completes wi'
- id: FR-376-AC-9
  path: spec/functional/FR-376-composite-equality-across-declarations.md
  role: examined
  excerpt: A union member whose one position holds a `P` composite in one value and a `C` composite in
    the other (also `C` against `G`), in both orders, gives `f
- id: FR-377-AC-1
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: '`compare_keys(a, b)` for composites of declaration keys `[1; 32]` and `[2; 32]` is `Some(Less)`
    and `compare_keys(b, a)` is `Some(Greater)`, whatever '
- id: FR-377-AC-2
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'Declaration keys that differ only in the last byte (`[1; 31]` followed by `0`, against the
    same followed by `1`) order by that byte; keys that differ '
- id: FR-377-AC-3
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: Two composites of one declaration key keep the slot order of the rule before this requirement;
    `compare_keys(x, x)` is `Some(Equal)`; two composites o
- id: FR-377-AC-4
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A `Set` and a `Bag` of element type `Composite(P)` formed through `form` from one composite
    of each of two declarations, in each of the two supply ord
- id: FR-377-AC-5
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'Equal keys occur exactly when values are equal: for a set of composites of two declarations
    with equal and unequal slots, every pair has `compare_keys'
- id: FR-377-AC-6
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, two composites of one declaration key and different slot
    counts, and a pair of `Float` members each return `None` fr
- id: FR-377-AC-7
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: 'The key order of a pair of different declarations holds as the payload of two `Option`s, in
    a record slot and as an element of two `Sequence` values, '
- id: FR-377-AC-8
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: '`compare_keys` on unions of one member that hold a `P`, `C` or `G` composite at the payload
    position returns `Some(Less)` for `P` against `C` and `C` '
- id: FR-378-AC-1
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` and `form_grouped` over an element type `Composite(P)` and one `P` member and one `C`
    member complete for `Sequence`, `Set`, `Bag` and `Ordered'
- id: FR-378-AC-2
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`form` over a `Set` of those two members charges, in order, `collection.member-walk`, `collection.member-test`
    with one pair, `collection.bound` and `'
- id: FR-378-AC-3
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted(Composite(P), Some(c))` returns an `Option` whose `payload_type()` is `Composite(P)`
    and whose payload has declaration `C` and its slot'
- id: FR-378-AC-4
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`from_admitted_slots` of a declaration whose declared field type is `Composite(P)` and whose
    slot holds a `C` composite keeps the slot unchanged; `ret'
- id: FR-378-AC-5
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: 'For the same `C` composite at element type `Composite(P)`: `construct_collection` refuses
    `CollectionElementNotAdmitted`; `form_collection` refuses `E'
- id: FR-378-AC-6
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: A collection of element type `Composite(C)` is not admitted by `ValueType::admits` at the collection
    type of element type `Composite(P)`, nor the reve
- id: FR-378-AC-7
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: The rustdoc of the seven items in Behavior 8 carries the stated text.
- id: FR-378-AC-8
  path: spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md
  role: examined
  excerpt: '`UnionValue::from_admitted` of a position typed `Composite(P)` and a `C` composite keeps the
    payload unchanged (declaration `C`, slots unchanged). For'
- id: FR-379-AC-1
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A nullary member has an occurrence count of 1 and a two-position member over two integers has
    3. The value reports its declaration, `VariantId`, ident
- id: FR-379-AC-2
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`union` refuses an arity of 0 against 1 supplied and 2 against 1 supplied with `Value` `WrongArity`,
    and a `Boolean` at an `Integer` position with `Po'
- id: FR-379-AC-3
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`evaluate_union` returns `Undefined` for the first argument that is undefined, runs no later
    argument and admits no charge. A completed `Boolean` at a'
- id: FR-379-AC-4
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Unions of one declaration and member with equal payloads are equal, with differing payload
    positions or members are not, and the pair count is one plu
- id: FR-379-AC-5
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A union against a union of another declaration, an enum member and a composite refuses `ValueKindMismatch`
    in `plan_equality` and gives `None` from `c
- id: FR-379-AC-6
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Members `Alpha` and `Zulu` order by identifier whatever their digests and payloads. One member
    orders by the payload positions in order. Declaration o
- id: FR-379-AC-7
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: '`Set`, `Bag`, `OrderedSet` and `Sequence` over four unions, from a supply order and its reverse,
    hold the stated elements. A union with an IEEE payloa'
- id: FR-379-AC-8
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: Clone, equality, key comparison, `Debug` and drop of a union nested ten thousand levels complete
    on a 512 KiB stack, a mixed union, composite and coll
- id: FR-379-AC-9
  path: spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md
  role: examined
  excerpt: A present `Option` of declared type `Composite(K)` over a union of declaration `K` is admitted,
    two such `Option` values with equal unions are equal a
- id: FR-380-AC-1
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: With an authenticated B, Integer and reduced Rational membership agree with the inclusive existing
    rule at both endpoints, one outside each endpoint a
- id: FR-380-AC-2
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Exact-B value and every exact-B endpoint are accepted for comparison; one-over-B in each of
    the three Integer or six Rational operand positions return
- id: FR-380-AC-3
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Observed production comparison steps do not exceed 2L for Integer or 4L for Rational, with
    fixed-width per-step work and constant auxiliary storage on
- id: FR-380-AC-4
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: A pre-cancelled original handle prevents input-sized work. Deterministically cancelling during
    a longest admitted comparison returns that cause before
- id: FR-380-AC-5
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Inspection identifies whether the production comparison actually performs a fallible native
    conversion or storage request. For each reachable boundary
- id: FR-380-AC-7
  path: spec/functional/FR-380-bounded-cancellable-numeric-membership.md
  role: examined
  excerpt: Inspection of both direct exact-kernel `IntegerInterval::try_contains_bounded` and `RationalDomain::try_contains_bounded`
    signatures verifies that aut
- id: FR-381-AC-1
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: With authenticated B, equal and unequal Integer intervals and Rational domains return the same
    Boolean equality as independent endpoint arithmetic, in
- id: FR-381-AC-2
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: At B >= 1, every one of the four Integer and eight Rational endpoint positions completes at
    exact B and returns typed unsupported size when made one b
- id: FR-381-AC-3
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: Observed production comparisons use no more than 2L Integer or 4L Rational limb-pair steps,
    fixed-width work per step, and the measured temporary-stor
- id: FR-381-AC-4
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: 'A pre-cancelled original handle stops before bound validation, including when the first endpoint
    exceeds B; cancellation on a late observed limb step '
- id: FR-381-AC-5
  path: spec/functional/FR-381-bounded-retained-numeric-type-equality.md
  role: examined
  excerpt: 'Source inspection establishes whether production performs native-size conversion or allocation.
    For each reachable boundary, an executed denial/retry '
- id: FR-382-AC-1
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: Equal distinct Decimal types complete `true`; a difference in either signed endpoint, either
    scale or rounding mode completes `false` under authentica
- id: FR-382-AC-2
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: At `B >= 1`, every endpoint position completes at exact `B` and returns typed `UnsupportedSize`
    with the correct position, measured `B+1` bits and `B`
- id: FR-382-AC-3
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: Observed production comparisons at inline/Large and 32-bit limb boundaries perform at most
    `2 ceil(B/32)` limb-pair steps of fixed-width work and no i
- id: FR-382-AC-4
  path: spec/functional/FR-382-bounded-retained-decimal-type-equality.md
  role: examined
  excerpt: A pre-cancelled original handle stops before bound validation, including when expected lower
    exceeds `B`; cancellation observed after a late limb step
- id: FR-383-AC-1
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: '`Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`, `cancel()` is
    `Some(&Cancel)` equal by shared-state identity to `handle`'
- id: FR-383-AC-2
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: 'Cancel the handle obtained from `meter.cancel()` with `Requested`, then charge the meter under
    sufficient limits: the charge is denied at its named po'
- id: FR-383-AC-3
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The accessor's implementation borrows the existing optional handle without cloning it or constructing
    a second `Meter` or `Counters`; calling it chang
- id: FR-383-AC-4
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std`
    + `alloc` build.
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-359-AC-9
  related_ids: []
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: '39'
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production
    `Meter::charge` for the exact three-charge FR-362 schedule (operand charge with at most two semantic-size
    requests, arithmetic charge with one, result-retain charge with none) over the full `i64` operand
    range, plus separate repeated-kind charge controls with at most two requests each. Symbolic `i64`
    operands and symbolic limits including `ResultUnits` cover first-short and simultaneous shortages
    and absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and 600 seconds, the
    basel
  finding: The standalone Meter criterion is bound only to its explicitly ignored dedicated Kani wrapper.
    Strict traceability remains non-green despite separate gate delivery; native-width tests do not supply
    this proof evidence.
- fnd: FND-002
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-359-AC-10
  related_ids: []
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: '40'
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their
    own assertions under their original domains, oracle, completion and counterexample checks, unwind
    3, Cadical, 16 GiB and 600 seconds. Record their individual Kani verdicts as an external IR-705/IR-17
    receipt; no quire-exact test or altered consumer harness substitutes for it.
  finding: The computed matrix has no local binding or delivered current five-test external consumer receipt.
    Genuine IR-17/IR-705 consumer qualification remains outstanding; native-width refinement tests cannot
    replace it.
- fnd: FND-003
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-6
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '86'
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original
    input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion
    cover and counterexample checks. Healthy verification and both substantive falsifications are required.
    Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler
    error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and
    does not satisfy this criterion.
  finding: The computed matrix has no binding or completed original consumer healthy and both substantive
    mutant qualification. Existing OOM measurements are inconclusive; native-width success is not consumer
    Kani acceptance.
- fnd: FND-004
  severity: low
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-383-AC-4
  related_ids: []
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  lines: '50'
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std`
    + `alloc` build.
  finding: The no_std build criterion has no executable-symbol binding in the computed matrix. This inherited
    verification-method discrepancy remains separate from implementation failure and the native-width
    change.
bindings:
- test_id: src/key.rs::tests::compare_keys_orders_members_of_one_shape_by_rank
  ac_id: FR-088-AC-11
  trace: correct
- test_id: src/key.rs::tests::compare_keys_refuses_same_rank_different_declaration_members
  ac_id: FR-088-AC-11
  trace: correct
- test_id: src/value.rs::tests::enum_shape_admits_only_its_own_variants
  ac_id: FR-088-AC-11
  trace: correct
- test_id: src/value.rs::tests::admits_refuses_a_known_variant_at_the_wrong_rank
  ac_id: FR-088-AC-11
  trace: correct
- test_id: src/value.rs::tests::enum_shape_rank_matches_canonical_position
  ac_id: FR-088-AC-11
  trace: correct
- test_id: src/value.rs::tests::a_set_visits_enum_values_in_canonical_rank_order
  ac_id: FR-088-AC-11
  trace: correct
- test_id: src/identity.rs::tests::tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes
  ac_id: FR-088-AC-12
  trace: correct
- test_id: src/equality.rs::tests::plan_pairs_refuses_a_population_pair
  ac_id: FR-089-AC-6
  trace: correct
- test_id: src/key.rs::tests::compare_keys_yields_no_key_for_a_population_pair
  ac_id: FR-089-AC-6
  trace: correct
- test_id: src/value.rs::tests::admits_refuses_a_population_pair
  ac_id: FR-089-AC-6
  trace: correct
- test_id: src/outcome.rs::tests::tc_428_every_record_building_refusal_names_code_and_cause
  ac_id: FR-096-AC-8
  trace: correct
- test_id: tests/checked_invariant.rs::all_typed_causes_are_distinct_internal_faults_with_required_traits
  ac_id: FR-096-AC-8
  trace: correct
- test_id: src/collection.rs::tests::tc_441_an_unbounded_collection_never_refuses_for_cardinality
  ac_id: FR-097-AC-7
  trace: correct
- test_id: src/collection.rs::tests::tc_441_an_unbounded_collection_stops_only_on_the_callers_meter
  ac_id: FR-097-AC-7
  trace: correct
- test_id: tests/ir673_core.rs::public_cardinality_query_is_inclusive_and_agrees_with_admission
  ac_id: FR-097-AC-8
  trace: correct
- test_id: src/value.rs::tests::a_deep_value_equals_its_clone_and_both_drop
  ac_id: FR-262-AC-2
  trace: correct
- test_id: src/value.rs::tests::a_deep_value_drops_on_a_small_stack
  ac_id: FR-262-AC-2
  trace: correct
- test_id: src/value/value_type.rs::tests::tc_735_a_deep_option_type_clones_compares_hashes_formats_and_drops
  ac_id: FR-262-AC-2
  trace: correct
- test_id: src/value/value_type.rs::tests::tc_735_value_type_debug_matches_the_derived_format
  ac_id: FR-262-AC-2
  trace: correct
- test_id: src/value/value_type.rs::tests::tc_735_value_types_equal_and_hash_equal_their_clones_only
  ac_id: FR-262-AC-2
  trace: correct
- test_id: src/value/value_type.rs::tests::tc_735_the_debug_worklist_grows_by_a_constant_per_type_node
  ac_id: FR-262-AC-2
  trace: correct
- test_id: src/value.rs::tests::type_nodes_grow_linearly_with_value_depth
  ac_id: FR-262-AC-3
  trace: correct
- test_id: src/value.rs::tests::a_deep_value_with_deep_types_is_admitted_in_linear_memory
  ac_id: FR-262-AC-3
  trace: correct
- test_id: src/division.rs::tests::division_by_zero_is_undefined
  ac_id: FR-357-AC-1
  trace: correct
- test_id: src/division.rs::tests::only_the_exposed_member_must_be_in_domain
  ac_id: FR-357-AC-2
  trace: correct
- test_id: src/division.rs::tests::exposed_member_outside_domain_refuses_with_its_cause
  ac_id: FR-357-AC-3
  trace: correct
- test_id: src/division.rs::tests::profiles_differ_on_negative_operands
  ac_id: FR-357-AC-4
  trace: correct
- test_id: src/division.rs::tests::i64_min_divided_by_minus_one_is_exact
  ac_id: FR-357-AC-5
  trace: correct
- test_id: src/division.rs::tests::division_charges_one_domain_occurrence_and_one_result_unit
  ac_id: FR-357-AC-6
  trace: correct
- test_id: tests/ir673_core.rs::modulo_is_euclidean_after_every_division_profile
  ac_id: FR-357-AC-7
  trace: correct
- test_id: src/accounting.rs::tests::injected_denial_spends_only_on_its_named_admitted_position
  ac_id: FR-358-AC-1
  trace: correct
- test_id: src/accounting.rs::tests::injected_plan_denial_spends_after_one_refusal
  ac_id: FR-358-AC-2
  trace: correct
- test_id: src/accounting.rs::tests::injected_denial_occurrence_requires_nonzero
  ac_id: FR-358-AC-3
  trace: correct
- test_id: src/accounting.rs::tests::charge_log_retains_only_first_4096_admissions
  ac_id: FR-358-AC-4
  trace: correct
- test_id: src/accounting.rs::tests::charge_log_retains_only_first_4096_admissions
  ac_id: FR-358-AC-5
  trace: correct
- test_id: src/accounting.rs::tests::production_meter_size_is_constant_as_charges_grow
  ac_id: FR-358-AC-6
  trace: correct
- test_id: tests/ir673_core.rs::truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-8
  trace: correct
- test_id: tests/ir673_core.rs::truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-9
  trace: correct
- test_id: tests/ir673_core.rs::truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-10
  trace: correct
- test_id: tests/ir673_core.rs::truncated_meter_log_preserves_accounting_and_denial_atomicity
  ac_id: FR-358-AC-11
  trace: correct
- test_id: src/accounting.rs::tests::injected_record_uses_precharge_work_and_plan_reservation
  ac_id: FR-358-AC-12
  trace: correct
- test_id: src/accounting.rs::tests::injected_denial_precedes_ordinary_size_and_plan_shortages
  ac_id: FR-358-AC-13
  trace: correct
- test_id: tests/ir667_normalize.rs::records_actual_sizes_coalesces_repeats_and_distinguishes_absent_zero
  ac_id: FR-358-AC-14
  trace: correct
- test_id: tests/ir667_normalize.rs::ordinary_injected_and_cancelled_charges_preserve_admission_records
  ac_id: FR-358-AC-14
  trace: correct
- test_id: tests/ir667_normalize.rs::fixed_records_share_the_existing_prefix_and_truncation_boundary
  ac_id: FR-358-AC-15
  trace: correct
- test_id: src/accounting.rs::tests::cumulative_work_reaches_max_then_refuses_atomically
  ac_id: FR-359-AC-1
  trace: correct
- test_id: src/accounting.rs::tests::cumulative_results_reach_max_then_refuse_atomically
  ac_id: FR-359-AC-2
  trace: correct
- test_id: src/accounting.rs::tests::exact_cumulative_amount_above_max_refuses_without_changes
  ac_id: FR-359-AC-3
  trace: correct
- test_id: src/accounting.rs::tests::exact_cumulative_amount_above_max_refuses_without_changes
  ac_id: FR-359-AC-4
  trace: correct
- test_id: src/accounting.rs::tests::result_refusal_preserves_prior_size_high_water
  ac_id: FR-359-AC-5
  trace: correct
- test_id: tests/ir673_core.rs::meter_uses_field_order_and_all_counters_are_queryable
  ac_id: FR-359-AC-6
  trace: correct
- test_id: tests/ir673_core.rs::meter_uses_field_order_and_all_counters_are_queryable
  ac_id: FR-359-AC-7
  trace: correct
- test_id: src/accounting.rs::tests::duplicate_size_requests_preserve_first_refusal_and_are_atomic
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::duplicate_sizes_coalesce_by_max_and_retain_explicit_zero
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::first_inserted_failing_size_precedes_later_same_kind_maximum
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::size_shortages_use_field_order_across_attachment_orders
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::cancellation_and_injection_precede_size_refusal
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::injected_denial_precedes_ordinary_size_and_plan_shortages
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::size_request_naming_a_cumulative_counter_is_validated_in_field_order
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::request_counts_from_zero_to_many_gain_no_denial
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::reordered_and_repeated_requests_keep_the_high_water_maximum
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::ordinary_refusal_leaves_a_pending_injected_occurrence_unspent
  ac_id: FR-359-AC-8
  trace: correct
- test_id: tests/meter_kani_gate.rs::meter_proof_and_production_all_loop_control
  ac_id: FR-359-AC-9
  trace: correct
- test_id: src/accounting.rs::tests::tc_931_admitted_log_matches_model_ordered_prefix
  ac_id: FR-359-AC-12
  trace: correct
- test_id: src/accounting.rs::tests::tc_931_admitted_log_matches_model_same_kind
  ac_id: FR-359-AC-12
  trace: correct
- test_id: src/integer.rs::tests::minimum_i64_absolute_value_is_positive_two_to_the_63rd
  ac_id: FR-360-AC-1
  trace: correct
- test_id: tests/ir653_admission.rs::integer_arithmetic_denial_precedes_large_intermediate_even_when_result_cancels
  ac_id: FR-361-AC-1
  trace: correct
- test_id: tests/ir653_admission.rs::rational_arithmetic_denial_precedes_large_unreduced_products
  ac_id: FR-361-AC-2
  trace: correct
- test_id: tests/ir653_admission.rs::division_and_modulus_denial_precedes_large_quotient
  ac_id: FR-361-AC-3
  trace: correct
- test_id: tests/ir653_admission.rs::decimal_scale_denial_precedes_power_of_ten_allocation
  ac_id: FR-361-AC-4
  trace: correct
- test_id: tests/ir653_admission.rs::decimal_arithmetic_denial_precedes_aligned_intermediate
  ac_id: FR-361-AC-5
  trace: correct
- test_id: tests/ir653_admission.rs::decimal_retain_denial_precedes_target_scale_expansion
  ac_id: FR-361-AC-6
  trace: correct
- test_id: tests/ir653_admission.rs::large_floor_quotient_completes_at_exact_operand_bit_limit_and_denies_below
  ac_id: FR-361-AC-7
  trace: correct
- test_id: tests/ir653_admission.rs::large_floor_quotient_completes_at_exact_operand_bit_limit_and_denies_below
  ac_id: FR-361-AC-8
  trace: correct
- test_id: tests/ir653_admission.rs::huge_decimal_target_denies_retain_before_power_allocation
  ac_id: FR-361-AC-9
  trace: correct
- test_id: tests/ir653_scalar.rs::integer_arithmetic_matches_checked_i128_oracle
  ac_id: FR-362-AC-1
  trace: correct
- test_id: tests/ir653_scalar.rs::integer_arithmetic_retains_results_beyond_i128
  ac_id: FR-362-AC-2
  trace: correct
- test_id: tests/ir653_scalar.rs::rational_arithmetic_matches_reduced_fraction_oracle
  ac_id: FR-362-AC-3
  trace: correct
- test_id: tests/ir653_scalar.rs::integer_and_rational_ordering_match_checked_oracle
  ac_id: FR-362-AC-4
  trace: correct
- test_id: tests/ir653_scalar.rs::scalar_families_charge_order_and_independent_size_maxima
  ac_id: FR-362-AC-5
  trace: correct
- test_id: tests/ir653_scalar.rs::scalar_families_charge_order_and_independent_size_maxima
  ac_id: FR-362-AC-6
  trace: correct
- test_id: tests/ir653_scalar.rs::three_direct_atoms_consume_seven_work_and_three_results
  ac_id: FR-362-AC-7
  trace: correct
- test_id: tests/ir653_scalar.rs::each_scalar_charge_point_can_stop_before_result
  ac_id: FR-362-AC-8
  trace: correct
- test_id: tests/ir653_scalar.rs::already_decided_boolean_connectives_follow_truth_tables_and_one_charge
  ac_id: FR-362-AC-9
  trace: correct
- test_id: tests/ir667_normalize.rs::rational_normalize_records_unreduced_parts_and_exact_boundary_stops
  ac_id: FR-362-AC-10
  trace: correct
- test_id: tests/ir667_normalize.rs::integer_arithmetic_exact_bits_and_one_under_use_operand_formulas
  ac_id: FR-362-AC-10
  trace: correct
- test_id: tests/ir667_normalize.rs::integer_and_rational_ordering_exact_bits_and_one_under_stop_at_first_short_point
  ac_id: FR-362-AC-10
  trace: correct
- test_id: tests/ir673_scalar.rs::scalar_outcomes_keep_only_admitted_prefixes
  ac_id: FR-362-AC-11
  trace: correct
- test_id: tests/ir673_scalar.rs::rational_constructor_reduces_sign_and_zero
  ac_id: FR-362-AC-12
  trace: correct
- test_id: tests/ir673_scalar.rs::rational_domain_refuses_denominator_outside_interval
  ac_id: FR-362-AC-13
  trace: correct
- test_id: tests/ir673_scalar.rs::direct_countdown_atoms_stop_at_fifth_result
  ac_id: FR-362-AC-14
  trace: correct
- test_id: tests/ir673_scalar.rs::direct_rational_division_charges_four_points
  ac_id: FR-362-AC-15
  trace: correct
- test_id: tests/ir673_scalar.rs::direct_integer_additions_share_meter_without_fold
  ac_id: FR-362-AC-16
  trace: correct
- test_id: tests/ir673_scalar.rs::atom_work_boundaries_stop_on_named_points
  ac_id: FR-362-AC-17
  trace: correct
- test_id: tests/ir673_scalar.rs::powers_of_two_charge_operand_derived_bounds
  ac_id: FR-362-AC-18
  trace: correct
- test_id: tests/ir653_scalar.rs::public_integer_arithmetic_admits_exact_bits_and_denies_one_under
  ac_id: FR-362-AC-19
  trace: correct
- test_id: tests/ir667_normalize.rs::rational_normalize_records_unreduced_parts_and_exact_boundary_stops
  ac_id: FR-362-AC-20
  trace: correct
- test_id: tests/ir873_temporal_visit.rs::temporal_visit_charges_one_work_unit_and_respects_the_boundary
  ac_id: FR-362-AC-21
  trace: correct
- test_id: tests/ir873_temporal_visit.rs::injected_temporal_visit_denial_is_atomic_and_one_shot
  ac_id: FR-362-AC-22
  trace: correct
- test_id: tests/ir653_decimal_order.rs::decimal_ordering_matches_independent_scaled_integer_oracle
  ac_id: FR-363-AC-1
  trace: correct
- test_id: tests/ir653_decimal_order.rs::decimal_ordering_charges_retained_zero_and_equal_value_representations
  ac_id: FR-363-AC-2
  trace: correct
- test_id: tests/ir653_decimal_order.rs::decimal_ordering_charges_retained_zero_and_equal_value_representations
  ac_id: FR-363-AC-3
  trace: correct
- test_id: tests/ir653_decimal_order.rs::huge_retained_scale_refuses_analytically_before_comparison
  ac_id: FR-363-AC-4
  trace: correct
- test_id: tests/ir653_decimal_order.rs::each_decimal_ordering_point_denies_its_suffix
  ac_id: FR-363-AC-5
  trace: correct
- test_id: tests/ir673_decimal.rs::normalized_equal_decimals_keep_distinct_retained_representations
  ac_id: FR-363-AC-6
  trace: correct
- test_id: tests/ir673_decimal.rs::metered_ordering_uses_retained_decimal_sizes
  ac_id: FR-363-AC-7
  trace: correct
- test_id: tests/ir673_ieee_text.rs::ieee_flags_iterate_all_subsets_in_vocabulary_order
  ac_id: FR-364-AC-1
  trace: correct
- test_id: tests/ir673_decimal.rs::decimal_half_ties_follow_all_six_rounding_modes
  ac_id: FR-365-AC-1
  trace: correct
- test_id: tests/ir673_decimal.rs::default_decimal_rounding_is_strict_exact
  ac_id: FR-365-AC-2
  trace: correct
- test_id: tests/ir673_ieee_text.rs::ieee_nan_selection_quiets_first_and_reports_later_signaling
  ac_id: FR-366-AC-1
  trace: correct
- test_id: tests/ir673_ieee_text.rs::ieee_nan_width_conversion_refuses_unrepresentable_payload
  ac_id: FR-366-AC-2
  trace: correct
- test_id: tests/ir673_ieee_text.rs::ieee_signed_zero_converts_to_canonical_rational_with_loss
  ac_id: FR-366-AC-3
  trace: correct
- test_id: tests/ir673_ieee_text.rs::ieee_total_order_separates_ten_classes_and_signed_zero
  ac_id: FR-366-AC-4
  trace: correct
- test_id: tests/ir673_ieee_text.rs::text_profile_mismatch_refuses_before_any_comparison_charge
  ac_id: FR-367-AC-1
  trace: correct
- test_id: tests/ir673_core.rs::charge_point_vocabulary_is_unique_and_round_trips
  ac_id: FR-368-AC-1
  trace: correct
- test_id: tests/ir673_core.rs::limit_kind_vocabulary_matches_scalar_limit_fields
  ac_id: FR-368-AC-2
  trace: correct
- test_id: tests/checked_invariant.rs::all_typed_causes_are_distinct_internal_faults_with_required_traits
  ac_id: FR-369-AC-1
  trace: correct
- test_id: tests/checked_invariant.rs::comparison_payloads_retain_the_original_typed_cause
  ac_id: FR-369-AC-1
  trace: correct
- test_id: tests/checked_invariant.rs::rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause
  ac_id: FR-369-AC-1
  trace: correct
- test_id: src/collection.rs::tests::evaluated_element_outside_its_type_reports_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
- test_id: src/collection.rs::tests::grouped_float_members_without_keys_report_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
- test_id: src/division.rs::tests::refused_interval_requires_a_bounded_domain
  ac_id: FR-369-AC-2
  trace: correct
- test_id: src/equality.rs::tests::plan_pairs_refuses_a_population_pair
  ac_id: FR-369-AC-2
  trace: correct
- test_id: src/equality.rs::tests::incompatible_pairs_report_specific_causes_without_a_boolean
  ac_id: FR-369-AC-2
  trace: correct
- test_id: src/value.rs::tests::deferred_record_field_outside_its_type_reports_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
- test_id: src/value.rs::tests::deferred_tuple_position_outside_its_type_reports_exact_cause
  ac_id: FR-369-AC-2
  trace: correct
- test_id: tests/checked_invariant.rs::all_typed_causes_are_distinct_internal_faults_with_required_traits
  ac_id: FR-369-AC-5
  trace: correct
- test_id: tests/checked_invariant.rs::comparison_payloads_retain_the_original_typed_cause
  ac_id: FR-369-AC-5
  trace: correct
- test_id: tests/checked_invariant.rs::rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause
  ac_id: FR-369-AC-5
  trace: correct
- test_id: src/value.rs::tests::native_type_admission_has_no_alias
  ac_id: FR-370-AC-1
  trace: correct
- test_id: src/value.rs::tests::native_equality_uses_boolean_leaf_schedule
  ac_id: FR-370-AC-2
  trace: correct
- test_id: src/value.rs::tests::native_mixed_kind_plans_refuse_in_both_orders
  ac_id: FR-370-AC-3
  trace: correct
- test_id: src/value.rs::tests::native_keys_order_canonical_content_and_sets
  ac_id: FR-370-AC-4
  trace: correct
- test_id: src/value.rs::tests::native_type_admission_has_no_alias
  ac_id: FR-370-AC-5
  trace: correct
- test_id: src/value.rs::tests::native_mixed_kind_plans_refuse_in_both_orders
  ac_id: FR-370-AC-5
  trace: correct
- test_id: src/value.rs::tests::native_equality_uses_boolean_leaf_schedule
  ac_id: FR-370-AC-5
  trace: correct
- test_id: src/value.rs::tests::native_keys_order_canonical_content_and_sets
  ac_id: FR-370-AC-5
  trace: correct
- test_id: src/value.rs::tests::existing_kind_controls_remain_admitted_and_keyed
  ac_id: FR-370-AC-5
  trace: correct
- test_id: src/value/native.rs::tests::uuid_canonical_text_preserves_network_octets
  ac_id: FR-370-AC-6
  trace: correct
- test_id: src/value/native.rs::tests::timestamp_canonical_text_preserves_full_signed_nanoseconds
  ac_id: FR-370-AC-7
  trace: correct
- test_id: src/value/native.rs::tests::uuid_canonical_text_preserves_network_octets
  ac_id: FR-370-AC-8
  trace: correct
- test_id: src/value/native.rs::tests::timestamp_canonical_text_preserves_full_signed_nanoseconds
  ac_id: FR-370-AC-8
  trace: correct
- test_id: src/integer.rs::tests::native_arithmetic_stays_inline_and_overflow_refines_exactly
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::inline_signed_bounds_refine_the_explicit_128_bit_domain
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::construction_boundaries_and_sampled_arithmetic_match_independent_oracles
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::signed_i64_extremes_refine_all_four_widened_operations
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::failed_width_guards_retain_full_i128_and_arbitrary_precision_results
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::canonical_parsing_preserves_native_boundaries_and_arbitrary_precision
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::fitting_canonical_parsing_requests_no_allocations
  ac_id: FR-371-AC-1
  trace: correct
- test_id: src/integer.rs::tests::cache_is_invisible_to_traits_clones_and_concurrent_borrows
  ac_id: FR-371-AC-2
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::parsed_values_keep_observations_and_borrows_across_materialization_and_clone
  ac_id: FR-371-AC-2
  trace: correct
- test_id: src/numeric.rs::tests::integer_bit_counts_preserve_exact_amounts_above_the_meter_limit
  ac_id: FR-371-AC-3
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::metered_native_and_overflow_operations_keep_the_existing_charge_schedule
  ac_id: FR-371-AC-3
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::metered_operation_matrix_preserves_fr362_amounts_work_and_denial_prefixes
  ac_id: FR-371-AC-3
  trace: correct
- test_id: src/integer.rs::tests::native_arithmetic_stays_inline_and_overflow_refines_exactly
  ac_id: FR-371-AC-4
  trace: correct
- test_id: src/integer.rs::tests::magnitude_limb_stream_does_not_materialize_small_cache
  ac_id: FR-371-AC-4
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::fitting_integer_operations_and_cached_source_clone_allocate_nothing
  ac_id: FR-371-AC-4
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::inline_signed_bounds_refine_the_explicit_128_bit_domain
  ac_id: FR-371-AC-4
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::widened_i64_extreme_operations_request_no_allocations
  ac_id: FR-371-AC-4
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::native_public_metered_call_matches_accounting_only_allocation_observation
  ac_id: FR-371-AC-4
  trace: correct
- test_id: src/integer.rs::tests::cache_is_invisible_to_traits_clones_and_concurrent_borrows
  ac_id: FR-371-AC-5
  trace: correct
- test_id: src/integer.rs::tests::once_box_race_retains_one_candidate_and_releases_losers
  ac_id: FR-371-AC-5
  trace: correct
- test_id: src/decimal_membership.rs::tests::inclusive_membership_preserves_representations_and_ignores_rounding
  ac_id: FR-372-AC-1
  trace: correct
- test_id: src/decimal_membership.rs::tests::borrowed_limb_crossings_and_huge_shift_have_bounded_work
  ac_id: FR-372-AC-1
  trace: correct
- test_id: src/decimal_membership.rs::tests::scale_sign_and_zero_paths_request_no_scratch
  ac_id: FR-372-AC-1
  trace: correct
- test_id: src/decimal_membership.rs::tests::longest_alignment_reuses_one_exact_request_within_independent_envelope
  ac_id: FR-372-AC-2
  trace: correct
- test_id: src/decimal_membership.rs::tests::borrowed_limb_crossings_and_huge_shift_have_bounded_work
  ac_id: FR-372-AC-2
  trace: correct
- test_id: src/decimal_membership.rs::tests::scale_sign_and_zero_paths_request_no_scratch
  ac_id: FR-372-AC-2
  trace: correct
- test_id: src/decimal_membership.rs::tests::actual_small_reservation_denial_is_typed_and_recoverable
  ac_id: FR-372-AC-3
  trace: correct
- test_id: src/decimal_membership.rs::tests::native_layout_capacity_failure_is_separate_from_allocator_denial
  ac_id: FR-372-AC-3
  trace: correct
- test_id: src/decimal_membership.rs::tests::original_precancelled_handle_prevents_the_actual_reservation
  ac_id: FR-372-AC-4
  trace: correct
- test_id: src/decimal_membership.rs::tests::reservation_division_comparison_and_publication_cancellation_release_scratch
  ac_id: FR-372-AC-4
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-1
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-2
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::nonfinite_values_stop_after_operands
  ac_id: FR-373-AC-3
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::numerator_and_denominator_refusals_retain_the_declared_target
  ac_id: FR-373-AC-4
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-5
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::ordinary_and_injected_denials_preserve_the_admitted_prefix
  ac_id: FR-373-AC-5
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::subnormal_intermediates_require_the_exact_denominator_bits
  ac_id: FR-373-AC-5
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::original_cancellation_handle_stops_each_poll_and_live_handle_completes
  ac_id: FR-373-AC-6
  trace: correct
- test_id: src/ieee.rs::rational_target_ieee::already_spent_meter_prefix_survives_success_and_every_retain_stop
  ac_id: FR-373-AC-7
  trace: correct
- test_id: tests/nonzero_integer.rs::admission_denies_zero_and_borrows_each_original_nonzero_integer
  ac_id: FR-374-AC-1
  trace: correct
- test_id: tests/nonzero_integer.rs::admitting_an_inline_integer_does_not_materialize_its_bigint
  ac_id: FR-374-AC-1
  trace: correct
- test_id: tests/nonzero_integer.rs::admission_denies_zero_and_borrows_each_original_nonzero_integer
  ac_id: FR-374-AC-2
  trace: correct
- test_id: tests/nonzero_integer.rs::admitted_token_has_no_default_construction
  ac_id: FR-374-AC-2
  trace: correct
- test_id: tests/nonzero_integer.rs::admitted_pair_helpers_preserve_signed_zero_and_large_results
  ac_id: FR-374-AC-3
  trace: correct
- test_id: tests/nonzero_integer.rs::exact_div_keeps_infallible_signed_results_and_nondivisible_truncation
  ac_id: FR-374-AC-4
  trace: correct
- test_id: src/integer.rs::tests::canonical_nonzero_premise_rejects_zero_with_its_own_message
  ac_id: FR-374-AC-5
  trace: correct
- test_id: tests/nonzero_integer.rs::metered_zero_stops_after_operands_and_denial_precedes_zero_classification
  ac_id: FR-374-AC-6
  trace: correct
- test_id: tests/nonzero_integer.rs::metered_nonzero_retains_laws_single_member_domain_and_charge_requests
  ac_id: FR-374-AC-6
  trace: correct
- test_id: src/equality.rs::tests::kernel_result_bearing_evaluation_returns_relation_plan_and_charge_order
  ac_id: FR-375-AC-9
  trace: correct
- test_id: src/equality.rs::tests::pair_and_result_refusals_stop_at_the_first_denied_charge
  ac_id: FR-375-AC-10
  trace: correct
- test_id: tests/composite_declarations.rs::different_declarations_are_unequal_with_one_pair_in_either_order
  ac_id: FR-376-AC-1
  trace: correct
- test_id: tests/composite_declarations.rs::different_declarations_charge_the_one_pair_schedule
  ac_id: FR-376-AC-2
  trace: correct
- test_id: tests/composite_declarations.rs::skipped_slots_never_report_a_foreign_reference
  ac_id: FR-376-AC-3
  trace: correct
- test_id: tests/composite_declarations.rs::same_declaration_compares_by_slots_as_before
  ac_id: FR-376-AC-4
  trace: correct
- test_id: tests/composite_declarations.rs::different_declarations_inside_wrappers_are_unequal
  ac_id: FR-376-AC-5
  trace: correct
- test_id: tests/composite_declarations.rs::faults_that_remain_still_refuse
  ac_id: FR-376-AC-6
  trace: correct
- test_id: tests/composite_declarations.rs::member_equal_of_different_declarations_is_false
  ac_id: FR-376-AC-7
  trace: correct
- test_id: tests/composite_declarations.rs::ordered_set_keeps_supply_order_across_declarations
  ac_id: FR-376-AC-8
  trace: correct
- test_id: tests/union_declarations.rs::union_payloads_of_different_declarations_are_unequal_without_a_fault
  ac_id: FR-376-AC-9
  trace: correct
- test_id: tests/union_declarations.rs::skipped_payload_slots_of_other_declarations_never_report_a_foreign_reference
  ac_id: FR-376-AC-9
  trace: correct
- test_id: tests/union_declarations.rs::traversal_does_not_short_circuit_after_a_declaration_mismatch
  ac_id: FR-376-AC-9
  trace: correct
- test_id: tests/union_declarations.rs::union_payload_pair_inside_another_value_leaves_enclosing_counts
  ac_id: FR-376-AC-9
  trace: correct
- test_id: tests/union_declarations.rs::unlike_union_pairings_stay_checked_invariant_faults
  ac_id: FR-376-AC-9
  trace: correct
- test_id: tests/composite_declarations.rs::declaration_key_orders_before_slots
  ac_id: FR-377-AC-1
  trace: correct
- test_id: tests/composite_declarations.rs::first_differing_declaration_byte_decides
  ac_id: FR-377-AC-2
  trace: correct
- test_id: tests/composite_declarations.rs::same_declaration_keeps_slot_order_and_distinct_declarations_never_equal
  ac_id: FR-377-AC-3
  trace: correct
- test_id: tests/composite_declarations.rs::set_and_bag_visit_the_smaller_declaration_key_first
  ac_id: FR-377-AC-4
  trace: correct
- test_id: tests/composite_declarations.rs::equal_keys_exactly_when_values_are_equal
  ac_id: FR-377-AC-5
  trace: correct
- test_id: tests/composite_declarations.rs::pairs_without_a_key_stay_unkeyed_and_refuse
  ac_id: FR-377-AC-6
  trace: correct
- test_id: tests/composite_declarations.rs::declaration_order_holds_inside_wrappers
  ac_id: FR-377-AC-7
  trace: correct
- test_id: tests/union_declarations.rs::union_payload_declarations_order_by_declaration_key_bytes
  ac_id: FR-377-AC-8
  trace: correct
- test_id: tests/union_declarations.rs::unlike_unions_have_no_key_and_refuse_equality
  ac_id: FR-377-AC-8
  trace: correct
- test_id: tests/union_declarations.rs::union_payload_key_order_sorts_set_and_keeps_ordered_set_supply_order
  ac_id: FR-377-AC-8
  trace: correct
- test_id: tests/composite_declarations.rs::form_and_form_grouped_keep_both_declarations_unchanged
  ac_id: FR-378-AC-1
  trace: correct
- test_id: tests/composite_declarations.rs::form_over_a_set_charges_one_member_comparison_and_no_element
  ac_id: FR-378-AC-2
  trace: correct
- test_id: tests/composite_declarations.rs::from_admitted_option_stores_a_descendant_payload_unchanged
  ac_id: FR-378-AC-3
  trace: correct
- test_id: tests/composite_declarations.rs::from_admitted_slots_keeps_a_descendant_slot_and_retains_once
  ac_id: FR-378-AC-4
  trace: correct
- test_id: tests/composite_declarations.rs::exact_paths_refuse_a_descendant_and_accept_the_declared_type
  ac_id: FR-378-AC-5
  trace: correct
- test_id: tests/composite_declarations.rs::collections_are_not_covariant_and_stored_collections_behave
  ac_id: FR-378-AC-6
  trace: correct
- test_id: tests/union_declarations.rs::admitted_union_keeps_a_descendant_payload_and_exact_paths_refuse_it
  ac_id: FR-378-AC-8
  trace: correct
- test_id: tests/union.rs::union_construction_keeps_identity_positions_and_occurrences
  ac_id: FR-379-AC-1
  trace: correct
- test_id: tests/union.rs::union_position_and_arity_refusals_are_typed
  ac_id: FR-379-AC-2
  trace: correct
- test_id: tests/union.rs::evaluated_union_preserves_deferred_fault_and_first_stop
  ac_id: FR-379-AC-3
  trace: correct
- test_id: tests/union.rs::evaluated_union_orders_arguments_and_charges_one_complete_result
  ac_id: FR-379-AC-3
  trace: correct
- test_id: tests/union.rs::union_equality_compares_members_and_payload_positions
  ac_id: FR-379-AC-4
  trace: correct
- test_id: tests/union.rs::foreign_union_enum_and_composite_kinds_remain_distinct
  ac_id: FR-379-AC-5
  trace: correct
- test_id: tests/union_declarations.rs::unlike_unions_have_no_key_and_refuse_equality
  ac_id: FR-379-AC-5
  trace: correct
- test_id: tests/union.rs::union_keys_order_identifier_ascii_then_payload_positions
  ac_id: FR-379-AC-6
  trace: correct
- test_id: tests/union.rs::member_declaration_order_and_digest_order_do_not_reach_union_key_order
  ac_id: FR-379-AC-6
  trace: correct
- test_id: tests/union.rs::union_collections_sort_deduplicate_and_preserve_first_occurrence
  ac_id: FR-379-AC-7
  trace: correct
- test_id: tests/union.rs::ieee_union_payload_has_no_key_and_sequence_keeps_occurrences
  ac_id: FR-379-AC-7
  trace: correct
- test_id: tests/union.rs::ten_thousand_union_levels_clone_compare_format_and_drop_on_small_stack
  ac_id: FR-379-AC-8
  trace: correct
- test_id: tests/union.rs::shared_union_children_count_each_occurrence_and_clone_shares_root
  ac_id: FR-379-AC-8
  trace: correct
- test_id: tests/union.rs::union_storage_grows_linearly_with_depth
  ac_id: FR-379-AC-8
  trace: correct
- test_id: tests/union.rs::mixed_union_composite_collection_paths_walk_and_drop_on_small_stack
  ac_id: FR-379-AC-8
  trace: correct
- test_id: tests/union_declarations.rs::option_of_union_admits_compares_and_keys_through_the_payload
  ac_id: FR-379-AC-9
  trace: correct
- test_id: src/numeric_membership.rs::tests::inclusive_integer_and_reduced_rational_membership_preserves_inputs
  ac_id: FR-380-AC-1
  trace: correct
- test_id: src/numeric_membership.rs::tests::exact_ceiling_admits_every_position_and_one_over_names_it
  ac_id: FR-380-AC-2
  trace: correct
- test_id: src/numeric_membership.rs::tests::borrowed_inline_and_large_comparisons_stay_within_limb_ceiling_without_allocation
  ac_id: FR-380-AC-3
  trace: correct
- test_id: src/numeric_membership.rs::tests::original_handle_cancels_before_work_and_at_a_late_limb_step
  ac_id: FR-380-AC-4
  trace: correct
- test_id: src/numeric_membership.rs::tests::borrowed_inline_and_large_comparisons_stay_within_limb_ceiling_without_allocation
  ac_id: FR-380-AC-5
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::arithmetic_endpoint_equality_preserves_borrowed_domains
  ac_id: FR-381-AC-1
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::integer_positions_accept_exact_b_and_report_first_one_over_b
  ac_id: FR-381-AC-2
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::rational_positions_accept_exact_b_and_report_first_one_over_b
  ac_id: FR-381-AC-2
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::borrowed_limb_work_and_storage_are_bounded_at_repr_boundaries
  ac_id: FR-381-AC-3
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::original_cancel_preempts_size_and_late_comparison_then_retry_completes
  ac_id: FR-381-AC-4
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::borrowed_limb_work_and_storage_are_bounded_at_repr_boundaries
  ac_id: FR-381-AC-5
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::original_cancel_preempts_size_and_late_comparison_then_retry_completes
  ac_id: FR-381-AC-5
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::retained_decimal_fields_decide_equality_without_changing_borrowed_inputs
  ac_id: FR-382-AC-1
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::decimal_endpoints_accept_exact_b_and_report_first_one_over_b
  ac_id: FR-382-AC-2
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::decimal_borrowed_limb_work_and_storage_stay_bounded
  ac_id: FR-382-AC-3
  trace: correct
- test_id: src/numeric_type_equality.rs::tests::decimal_original_cancel_preempts_size_and_late_comparison
  ac_id: FR-382-AC-4
  trace: correct
- test_id: tests/ir895_meter_cancel.rs::accessor_reports_only_the_installed_handle
  ac_id: FR-383-AC-1
  trace: correct
- test_id: tests/ir895_meter_cancel.rs::accessor_handle_cancels_the_next_charge_without_changing_accounting
  ac_id: FR-383-AC-2
  trace: correct
```

## Dispositions

Round1 independently read back the actual current PR70 root decision and all three continuing owner comments. This authority is specific to PR70, not imported from PR67/68. Frozen source remains888d5a5; no product changes or new findings. The original findings and **FAIL** verdict above remain unchanged. Scoped implementation acceptance does not close IR-705/IR-17, discharge any original consumer control or establish IR-917 fit.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Current PR70 root scope decision a7ea23a4-d203-4b1d-8938-d9ea20041314 defers the inherited ignored-only Meter binding to existing IR-810 assurance owner, actual continuing comment5b08d0b8-40ba-4497-978c-476e8119236a. Separate gate delivery does not turn the strict matrix green; no native-width test replaces the Meter proof. |
| FND-002 | deferred | Current PR70 root scope decision a7ea23a4-d203-4b1d-8938-d9ea20041314 defers the five genuine current-CG consumer tests to IR-17/IR-705, actual continuing owner comment7d0df838-11c4-4027-a7d7-ae368a8e0549. QSL-937 and the foreign parent stack remain adoption blockers. Kernel/native tests supply no routed or attested receipt acceptance. |
| FND-003 | deferred | Current PR70 root scope decision a7ea23a4-d203-4b1d-8938-d9ea20041314 defers original healthy and both substantive mutant qualification to IR-17/IR-705, owner comment7d0df838-11c4-4027-a7d7-ae368a8e0549. Original16GiB/600s/unwind3/Cadical/cover/witness obligations remain; prior76516GiB/40GiB OOMs have no verdict. Separate genuine IR-917 fit is unmeasured pending owner input9d43e8ff; no tractability claim. |
| FND-004 | deferred | Current PR70 root scope decision a7ea23a4-d203-4b1d-8938-d9ea20041314 defers inherited no_std Test-symbol/method reconciliation to existing IR-895 verification owner, actual continuing commentc41187f2-aae7-4f5a-a34f-c3f63b9b1208. Actual required no_std build remains mandatory; accessor behavior is not reopened and no artificial test trace is added. |
