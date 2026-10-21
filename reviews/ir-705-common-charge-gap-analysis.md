---
id: SR-15258
title: IR-705 PR76 private common charge gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact; PR76 frozene1411c6; whole spec/src/tests/scripts inventory
  and full computed matrix
review_set: subset
---

## Summary

Ticket: IR-705; product PR quire-exact#76. Sole independent nonauthor reviewer. Exact clean frozen heade1411c620d91137737c53a1b71ba655bd641c32d; published base949ba2cca08d820b1c9fa566997ce02ea6b3354a. Four changed files: src/accounting.rs, tests/ir705_common_charge.rs, scripts/meter_kani_gate.py and scripts/test_meter_kani_gate.py. All422 frozen tracked-source hashes independently match. Cargo.toml/Cargo.lock and complete original src/accounting/kani_proofs.rs (harness/model/domain/limits/schedule) byte-compare to949. No author/source, protected qualification case/target, other worktree or frozen review was changed. Repo AGENTS/CLAUDE/.agent Rust conventions were freshly read. No applicable AssuranceProfile found. No spec/plan edits: spec-base and sub-analyses are inapplicable; code-review includes Rust plus the changed Python gate lane. No broad optional semantic fanout. Only read-only files/Git/tracker and Quire/Quoin commands ran; NO Cargo/fmt/metadata/native/Python tests/backend/Kani/proof/aggregate. IDs were checked against reviewed base/main artifacts, alllocal private review artifacts (maxSR15256) and actual open PR inventory (only76), allocating collision-free SR15257/15258.

## Verdict

**FAIL — four inherited local assurance gaps plus one introduced wrong trace binding.** Candidate backend/persistent gate is unmeasured; no product PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The dedicated Meter proof wrapper remains ignored-only in the local matrix despite its separately delivered gate; the strict local assurance status remains non-green. | spec/functional/FR-359-cumulative-meter-boundary.md:39 |
| FND-002 | medium | The current five unchanged CG consumer tests and external proof/receipt qualification remain without a local binding or delivered current acceptance; native common-charge tests do not substitute. | spec/functional/FR-359-cumulative-meter-boundary.md:40 |
| FND-003 | medium | The criterion remains locally untagged despite newly successful original-artifact healthy and canonical bound/arithmetic controls; these external results do not create a local trace binding. | spec/functional/FR-371-bounded-integer-representation-refinement.md:90 |
| FND-004 | low | The no_std build criterion retains its inherited Test-method/no-symbol binding discrepancy, separately from actual supported no_std build success. | spec/functional/FR-383-meter-cancellation-handle-access.md:50 |
| FND-005 | medium | The new builder-history test claims injection-precedence AC13 but installs no injected denial and never calls charge_plan; it adds a false criterion binding. | src/accounting.rs:1429-1474 |

## Coverage

Fresh explicit-scope quire matrix0.36.2/engine0.50.2 and quoin matrix0.28.4 were compared with separately Git-extracted949 base. All189 criterion ids/statements/methods/statuses remain identical:167tagged,18Inspection method-without-symbol,3untagged,1ignored-only. Bindings275→281 with six new resolved bindings and none removed. Five new bindings contribute to common/promotion/ordinary-refusal/log behavior. The sixth, FR358AC13 on the builder-history test, is a newly introduced false trace binding (FND005); static tagged status does not make it valid. Quoin no-run-evidence rows do not establish execution. No whole AC13 or general acceptance is inferred from these tests.

Full reverse inventory covers27src Rust files,20test Rust files and threescripts. Existing identity/node/location/reference/outcome, value/type/composite/union/collection/equality/key, numeric/domain and accounting/cancellation surfaces map to local requirements or cited external QSpec ownership. Changed storage/admission is owned by FR359/FR358; no new unowned production behavior or TODO/FIXME/XXX/todo!/unimplemented! stub found. The common copy/no-drop/source-anchor checks establish mechanism/native validation only, never a replacement proof. Whole-repository optional semantic fanout was not run; listed189scope entries are mechanical criterion examinations, not blanket certification of all281bindings. Inherited coverage diagnostics/heuristics remain separate from the concrete false new trace finding.

Four prior local assurance gaps remain: IR810 Done's delivered prior Meter gate versus ignored-only local binder (actual handoff5b08d0b8); IR705/IR17 current externalfive/receipt acceptance blocked on QSL937 (actual IR17 In Progress handoff7d0df838); FR371AC6 local untagged binding despite actual original-artifact healthy plus canonical bound/arithmetic PASS16GiB atf28d (IR7056ce5acee), not old OOM-only controls; and IR895 Done's no_std Test-method/binder reconciliation (actual handoffc41187f2). Prior owner decisions79c56958/c17b5325 are context, not invented PR76 deferrals. New common-charge tests cannot close current consumer acceptance. Actual authorization616340b5 permits private option1 only, requires current native/persistent gate before merge, and reserves option2/new bounded API for the owner; no exception or new scope is inferred from ticket prose.

Published949 genuineIR91716GiB attempt still OOMed (IR70505b7ea70); separately authorized40GiB diagnostic also OOMed (IR705fdb27527), not acceptance or causal attribution. No IR917healthy/helper mutant, current CGfive/native-attested receipts, IR17 delivery, wholeIR705 completion or memory-fit claim is made. Candidate persistent Meter baseline+real ALL/refusal-order controls and final reviewed-head aggregate remain pending. Existing prior Meter delivery is distinguished from this changed candidate's unmeasured proof behavior.

Supplied focused evidence inspected, not rerun: initial compile failed because the private opaque iterator did not expose Clone; archived red source/log retained. Correct private bound, retry1 fmt/fmt-check/lint(default/test-support/thumbv7em-none-eabi), accounting26/28,cancel7/7,numeric4/4, five integration binaries35/41, three VerdictTests passed. Final mechanism assertion strengthened exact third-request promotion; confined fmt/lint/accounting26/28 passed again. Actual16GiB/swap0/jobs1/testthreads1/QSPECfd7, peaks685441024 and395071488bytes, memoryevents0. Heavy-release records allthree scopes inactive/hierarchiesgone/TENguardsempty and targetretained. No reviewer heavy work, backend or aggregate; those remaining gates cannot be inferred from native receipts.

Plan completion: not assessed

## Review data

```yaml
clean: false
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
  excerpt: 'A declared unit `UnitId` and a compound unit `UnitId` are unequal when
    their domains or bytes differ. A declared unit''s `UnitId` carries its node key
    under the `quire.checked-semantic-node/v1` label, '
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
- id: FR-369-AC-1
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause`
    payload whose closed variant set and four typed `IllTypedCause` payloads equal
    the thirty table rows; `CheckedInvariantCau
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
  excerpt: 'Every received typed cause follows QSL''s existing internal-fault path:
    no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant`
    at S6a, `CallFailure::Fault` at the publ'
- id: FR-369-AC-8
  path: spec/functional/FR-369-typed-checked-invariant-causes.md
  role: examined
  excerpt: The public carrier contains no QSL model/checker/catalog type or dependency
    on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and
    an exhaustive source inventory finds every ke
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
  excerpt: "Production helper observations on admitted `B,S` fixtures stay within\
    \ the one-request `4 \xD7 L_B`-byte scratch envelope and `2 \xD7 (min(S,B)+2)\
    \ \xD7 L_B` limb-step bound, including a longest-pass endpoint fi"
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
  excerpt: '`compare_keys(a, b)` for composites of declaration keys `[1; 32]` and
    `[2; 32]` is `Some(Less)` and `compare_keys(b, a)` is `Some(Greater)`, whatever
    their slots: slots equal, slots that would order t'
- id: FR-377-AC-2
  path: spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: Declaration keys that differ only in the last byte (`[1; 31]` followed
    by `0`, against the same followed by `1`) order by that byte; keys that differ
    in the first byte order by the first byte even whe
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
  excerpt: Members `Alpha` and `Zulu` order by identifier whatever their digests and
    payloads. One member orders by the payload positions in order. Declaration order
    and digest order do not reach the order. Equa
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
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference`
    uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand
    charge with at most two semantic-size requests, arithmetic charge with one, result-retain
    charge with none) over the full `i64` operand range, plus separate repeated-kind
    charge controls with at most two requests each. Symbolic `i64` operands and symbolic
    limits including `ResultUnits` cover first-short and simultaneous shortages and
    absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and
    600 seconds, the basel
  finding: The dedicated Meter proof wrapper remains ignored-only in the local matrix
    despite its separately delivered gate; the strict local assurance status remains
    non-green.
- fnd: FND-002
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-359-AC-10
  related_ids: []
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  lines: '40'
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests
    below pass their own assertions under their original domains, oracle, completion
    and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds. Record their
    individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test
    or altered consumer harness substitutes for it.
  finding: The current five unchanged CG consumer tests and external proof/receipt
    qualification remain without a local binding or delivered current acceptance;
    native common-charge tests do not substitute.
- fnd: FND-003
  severity: medium
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-371-AC-6
  related_ids: []
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  lines: '90'
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic
    mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second
    native ceiling, 16GiB memory ceiling, completion cover and counterexample checks.
    Healthy verification and both substantive falsifications are required. Existing
    all-four-operation, accounting/denial and refinement obligations remain required.
    A compiler error, MemoryExhausted, timeout, missing mutation execution or missing
    teardown is inconclusive and does not satisfy this criterion.
  finding: The criterion remains locally untagged despite newly successful original-artifact
    healthy and canonical bound/arithmetic controls; these external results do not
    create a local trace binding.
- fnd: FND-004
  severity: low
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-383-AC-4
  related_ids: []
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  lines: '50'
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles
    in the crate's `no_std` + `alloc` build.
  finding: The no_std build criterion retains its inherited Test-method/no-symbol
    binding discrepancy, separately from actual supported no_std build success.
- fnd: FND-005
  severity: medium
  confidence: high
  method: gap-analysis
  check: trace
  artifact_id: FR-358-AC-13
  related_ids:
  - FR-359-AC-8
  path: src/accounting.rs
  lines: 1429-1474
  excerpt: 'With an occurrence-1 injected denial selected, make the same `charge`
    independently fail an ordinary semantic-size counter and make a `charge_plan`
    with `pairs = 2` independently fail its work reservation under a configured work
    limit of 3 after two admitted work units, with all other limits sufficient. In
    each case, the injected `WorkUnits` record is returned before the ordinary refusal:
    `limit = consumed` equals the pre-charge work, `next_charge` is the charge''s
    work amount or the plan''s reservation respectively, and every consumed counter
    and admission count remain unchanged. The criterion '
  finding: The new builder-history test claims injection-precedence AC13 but installs
    no injected denial and never calls charge_plan; it adds a false criterion binding.
  test_id: src/accounting.rs::tests::common_and_promoted_builder_histories_have_identical_observations
  claimed_ac: FR-358-AC-13
  correct_ac: FR-359-AC-8
  scenario: The entire test only compares common/promoted/fitting Eq/Clone/Debug and
    ordinary successful or IntegerBits-refused charge calls. A regression that checks
    ordinary size/work refusal before injection in charge and charge_plan cannot fail
    this test. Remove AC13 from this test or exercise its actual two injected-precedence
    obligations; retain the independently meaningful storage/refusal assertions.
bindings:
- test_id: src/accounting.rs::tests::charge_size_requests_keep_two_inline_and_overflow_in_order
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::common_and_promoted_builder_histories_have_identical_observations
  ac_id: FR-359-AC-8
  trace: correct
- test_id: src/accounting.rs::tests::common_and_promoted_builder_histories_have_identical_observations
  ac_id: FR-358-AC-13
  trace: wrong
- test_id: tests/ir705_common_charge.rs::public_charge_transitions_preserve_literal_denials_and_admitted_prefixes
  ac_id: FR-359-AC-6
  trace: correct
- test_id: tests/ir705_common_charge.rs::public_charge_transitions_preserve_literal_denials_and_admitted_prefixes
  ac_id: FR-359-AC-8
  trace: correct
- test_id: tests/ir705_common_charge.rs::public_charge_transitions_preserve_literal_denials_and_admitted_prefixes
  ac_id: FR-358-AC-14
  trace: correct
```

## Dispositions

Separate same-reviewer disposition round1 at clean frozen fix546eadb4a5cd540c8bfc1f3efd9c9e2e47cdc413. Original Findings and review bytes remain unchanged as a prefix. The fix removes only UNDETERMINED from the mutant companion-status allow-list, adds one resolved-vs-unresolved adverse parser control, and removes only the false FR358AC13 tag. Exact expected failure/function/file/line/loop checks remain unchanged. No new finding.

Independently checked preserved initial-review hashes, exact source diff, archived old-control classifier byte-equal toe1411c6 and its new test byte-equal to fixed test source. Supplied negative execution genuinely fails the old classifier (GateError not raised), while fixed four VerdictTests pass. Focused fmt/fmt-check/lint(three lanes), accounting26/28 pass; actual16GiB/swap0/QSPECfd7, peak397017088bytes, allmemoryevents0, hierarchy gone/TENguards empty, targetretained. These are inspected receipts, not reviewer execution. Supplied matrix compared narrowly: exactly wrongAC13 binding removed (281→280), no added binding or189criterion/status change; no full matrix repeated. No reviewer heavy/native/backend/proof/aggregate work.

Actual current root decisione437a3cc-7298-4e8e-b12e-75971dd8c2c2 and existing owner handoffs were read back. Code/Rust latest disposition PASS, with both findings FIXED. Full gap remains FAIL for four inherited local gaps, explicitly DEFERRED; introduced trace FND005 FIXED. Persistent Meter baseline, actual ALL/refusal-order controls and backend property mapping remain PENDING and required at reviewed head, plus final aggregate and exact artifact custody before merge. No IR917 fit or acceptance inferred.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Current PR76 root decisione437a3cc-7298-4e8e-b12e-75971dd8c2c2 defers unchanged FR359AC9 local ignored-only binder reconciliation to existing IR810; actual owner handoff5b08d0b8-40ba-4497-978c-476e8119236a preserves delivered prior gate. The REQUIRED real current-head persistent Meter baseline/ALL/refusal-order controls before this merge are not deferred or inferred from that prior delivery. |
| FND-002 | deferred | Current PR76 root decisione437a3cc-7298-4e8e-b12e-75971dd8c2c2 defers unchanged FR359AC10 current five genuine CG consumer proofs/receipt to open IR17/IR705; actual IR17 owner handoff7d0df838-11c4-4027-a7d7-ae368a8e0549 retains QSL937 blocker. New common/native tests cannot substitute. |
| FND-003 | deferred | Current PR76 root decisione437a3cc-7298-4e8e-b12e-75971dd8c2c2 defers FR371AC6 local evidence binding and current TC027 receipt to open IR705. Original healthy/bound/arithmetic16GiB results atf28d, actual IR7056ce5acee-ac3c-47e2-9f09-c17376e80e14, remain successful historical evidence; they are not called unexecuted/OOM-only or new current-driver acceptance. |
| FND-004 | deferred | Current PR76 root decisione437a3cc-7298-4e8e-b12e-75971dd8c2c2 defers unchanged FR383AC4 no_std verification-method/local Test-binder reconciliation to existing IR895, actual handoffc41187f2-aae7-4f5a-a34f-c3f63b9b1208. Delivered accessor/no_std behavior is not reopened; real no_std checks do not manufacture a Test-symbol binder. |
| FND-005 | fixed | At546eadb4, inspected fixed source:     #[trace("FR-359-AC-8")]     #[test]     fn common_and_promoted_builder_histories_have_identical_observations() { |

## Review update (disposition pass 2)

Same sole reviewer, PR76 disposition/source review round2. New exact clean frozen headcbe9e680a5a6574858e27bca8b6d86180bf72c10; source baseaf4abc4acfdb28f69e29679fc290b8056d3260f3 (round1 source546eadb4 plus byte-exact review custody). Only src/accounting.rs source changes65insertions/26deletions. Original source review/findings and entire round1 artifacts remain byte-unchanged as a prefix. No new finding; no previously resolved/deferred finding reopened. Their prior outcomes remain current; dispositions is empty because no FND had latest still-open or lacked an outcome before this pass.

Latest code/Rust source verdict: PASS. Full repository gap verdict: FAIL, four inherited local discrepancies still DEFERRED under actual root decisione437a3cc and existing IR810 / IR17+IR705 / IR705 / IR895 owners. Original code FND001/002 and gapFND005 stay FIXED, and stricter status classifier/false-tag fixes are byte-unchanged.

The new private CommonRequests has named first/second Option<(LimitKind,u64)> slots and derives Copy/Clone/Eq. next takes first when present, otherwise second; each take clears exactly that slot. Zero/one/two requests, same-kind duplicates, explicit zero, repeated exhaustion and copying the remaining cursor preserve ordered values. No caller-mutated state/aliasing, new allocation, indexed search, owned-array IntoIter/Flatten or additional production loop appears. The new test directly exercises order, duplicate zero, exhausted state and an independently copied remaining cursor and truthfully binds FR359AC8.

Builder storage invariants remain first-then-second; an admitted iterator consumes its owned copy, never a retained builder. push_units now assigns the named empty slot directly; third request still promotes and appends. Common-to-general promotion maps both Option slots in place to ChargeAmount::Units and empty overflow Vec, preserving request count/order/multiplicity, zeros, native work/results and exact subsequent spill without a loop. Manual Eq/Clone/Debug still normalize to identical general contents, including overwritten exceptional work/results. General path and borrowed iterator are unchanged. Private cursor implements the existing Clone request bound with no baseline runtime clone. The ONE charge_fields body/check_injected/first-failure/cumulative/allten atomic commit/admitted log is unchanged; cancellation/injection before traversal and exact negative/above-u64/duplicate refusal behavior remain unchanged. No new public API, Integer/as_big, representation/domain, real arithmetic, proof/model or budget change.

Cargo.toml/lock, complete original kani_proofs.rs, both gate scripts, public literal-oracle test and all other production files byte-compare toaf4. The unique mutation anchor for (kind, amount) in requests remains in sharedcharge_fields. Actual future backend specialization/loop mapping is still unconfirmed; unchanged strict parser/source-anchor tests are not backend evidence or permission to accept a different failure.

Fresh complete explicit-scope quire matrix0.36.2/engine0.50.2 plus quoin matrix0.28.4 compared with a separately Git-extractedaf4base: all189criterion ids/statements/methods/static statuses identical (167tagged/18Inspection-method-without-symbol/3untagged/1ignored-only). Bindings280→281, exactly one correct FR359AC8 binding on common_cursor_keeps_request_order_and_clones_the_remaining_prefix; no removed binding, no return of false AC13. This281count differs in membership from initiale141's281: wrongAC13 remains removed and truthful cursorAC8 is added. Quoin static/no-run-evidence is not a new proof result. Fullgap stays FAIL; original four local owned gaps remain deferred with no newly introduced gap or stale trace.

Reverse changed-code check maps named native storage/iterator/promotion to FR359AC8 and existing FR358/359 semantic requests, denials and log obligations; no new unowned behavior, stub or copied second Meter. Whole-source/requirements inventory remains as previously reviewed because only this private accounting surface changed. No spec/plan changes or new profile: spec-base/subanalyses remain inapplicable; no optional broad semantic fanout.

Inspected actual fix-round2 receipts: fmt/fmt-check/lint(default/support/thumb) exit0, accounting27/29, public integration18/23 and four VerdictTests PASS. Scope enforced16GiB/swap0/jobs1/QSPECfd7; actual peak534257664bytes, memoryevents0; hierarchy gone/TENexactguards empty/targetretained at explicit release. No reviewer Cargo/fmt/metadata/native/Python-test/backend/goto/Kani/aggregate ran. These native receipts do not measure proof fit.

Actual root commentb432fea4-7813-492a-b71b-6cb84dea5b28 records prioraf4 aggregate PASS then real persistent healthy OOM at16GiB exact terminal peak17179869184bytes, no logical verdict; ALL never started. That failure remains a merge blocker, not a waived local binder issue. Removing array/search encoding is a confined option1 hypothesis, not causal attribution or accepted tractability. Currentcbe9 aggregate, actual persistent healthy plus ALL/refusal-order controls and exact property mapping must pass at reviewed/custody head before merge; only after publication unchanged genuineIR91716GiB qualification can support fit. No option2/public admitted API or domain/model/solver/memory weakening is authorized.

Plan completion: not assessed

All previously final outcomes remain current; no new Dispositions table row is needed this round.
