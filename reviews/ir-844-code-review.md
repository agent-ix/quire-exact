---
id: SR-14200
title: code-review review of IR-844 meter completion
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; src/accounting.rs, src/accounting/kani_proofs.rs, spec/functional/FR-359-cumulative-meter-boundary.md,
  spec/test-cases/TC-931-single-pass-meter-accounting.md
review_set: subset
---

## Summary

Frozen head a4f22a9a5ee9d05dfcb5ab5ca3b8836c89bd59b0; base 441407020d76b8e9b63d1985701bbe78caaf2bfe. Ticket: IR-844. PR: agent-ix/quire-exact#65.

Independent source inspection found no new Rust correctness, panic, narrowing, production-fork, request-cap, ordering or atomicity defect.

## Verdict

PASS — Independent source inspection found no new Rust correctness, panic, narrowing, production-fork, request-cap, ordering or atomicity defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Reviewer model identity: GPT-6; the harness does not expose a more specific SKU. Markers use `gpt-6`.

The Rust lane examined all changed code and complete existing accounting tests, with Integer conversion semantics and FR-362 schedule as context. Admissible amounts retain u64 representation; negative and oversized exact values retain their complete boxed Integer for refusal. Two named slots preserve insertion order and spill without a public bound. The only semantic-size traversal follows requests. Cancellation/injection precede size checks; field order and first failing duplicate are preserved; cumulative checks finish before any counter/admission/log commit. Borrowed refusal amounts stay alive until converted; the temporary charge_plan amount is valid for the call. The independent oracle names limits and counters separately, keeps all six attempts, four symbolic limits, full i64/u8 domains and five independent duplicate/result quantities, public charge and arithmetic exact_size. B(0)=1 matches normative Integer::magnitude_bits. No production cfg(kani) alternative exists. The model is intentionally independent verification code, not an alternate production implementation.

Existing raw native logs independently show 25+27+8+7+6=73 passing test executions and six non-test format/lint/build phases (eleven phases total). All three Clippy lanes and actual bare-metal build completed. Both Rust files byte-match the actual qualified c824270 source; only documentation changed afterward. Raw healthy log and terminal evidence establish 0/2565 failed, all six unwinds successful, 131.5852 s verification and 8,183,332,864 bytes cgroup peak at 16 GiB with swap disabled. The actual ALL mutant changes the production request traversal and fails accounting::Meter::charge.unwind.1. The order mutant changes only the production comparison and fails the independent model assertion; raw generated playback executes one test, reproducing ValueOccurrences versus IntegerBits. The 40 GiB observer result is distinct from its preserved 16 GiB OOM and healthy slicing protocol. No cause of historical OOM is inferred.

New trace attributes are on actual tests only. The installed matrix currently omits these two namespaced supplemental test attributes, while existing AC-6/8 bindings remain. Their attribute IDs and assertions are reviewed as correct semantic bindings, not claimed new computed coverage. No proof or production trace tag was added. Existing raw logs suffice for this source; no new Cargo, rustc, fmt, native or Kani invocation was run. Root retains the once-only aggregate gate. No CI/gate changes, vendoring, unsafe, wire/persistence change, async, lock or new public API surface is present.

## Examined Scope

```yaml
scope:
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one work unit
    reaches `u64::MAX` exactly. The next one-unit charge returns `Incomplete` at its named point with `limit_kind
    = WorkUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission count
    stay at their pre-refusal values.
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting `u64::MAX` result units. Its
    work consumption is 1. A second charge requesting one result unit returns `Incomplete` at its named point with
    `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission
    count remains 1.
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount is `u64::MAX + 1` returns
    `Incomplete` with `limit_kind = WorkUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes.
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit and `u64::MAX + 1` result units
    returns `Incomplete` with `limit_kind = ResultUnits` and `next_charge = u64::MAX + 1`; no counter or admission
    count changes.
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least 16 bits,
    `u64::MAX` work units and one result unit. Then attempt a charge carrying `integer_bits = 16` and one result
    unit. The result refusal leaves `integer_bits = 8`, work units, result units and admission count at their pre-refusal
    values; under `test-support`, the admitted-charge log is unchanged too.
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters the first unavailable counter
    of a charge, including several simultaneously short counters and semantic sizes attached in either order. The
    reported `limit_kind` follows `ScalarLimits` field order, with the exact limit, previously consumed amount,
    requested amount and point. A refusal after an earlier size check succeeds changes no counter, admission count
    or test-support log. A separate meter with a pending later named denial proves that this ordinary refusal does
    not advance its occurrence.
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers without panic for every member
    of `LimitKind::ALL`: untouched counters are zero, semantic sizes are admitted high-water maxima and work/results
    are admitted cumulative totals.'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum observable through `consumed()`;
    the diagnostic log record is verified under FR-359-AC-12. With `IntegerBits` limit 5 and requests 7 then 9 for
    that kind in one charge, refusal reports `next_charge = 7`, the first-inserted failing amount; the existing
    9 then 7 case alone cannot distinguish maximum coalescing before refusal. Simultaneous shortages refuse in `ScalarLimits`
    field order; ordinary refusal preserves counters, high-water, admissions, log and a pending injected occurrence.
    Cancellation and injection precedence, c
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production `Meter::charge`
    for the exact three-charge FR-362 schedule (operand charge with at most two semantic-size requests, arithmetic
    charge with one, result-retain charge with none) over the full `i64` operand range, plus separate repeated-kind
    charge controls with at most two requests each. Symbolic `i64` operands and symbolic limits including `ResultUnits`
    cover first-short and simultaneous shortages and absence of the third admission. Under `--unwind 3 --solver
    cadical`, 16 GiB and 600 seconds, the basel
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their own assertions
    under their original domains, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600
    seconds. Record their individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test or altered
    consumer harness substitutes for it.
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and called helpers finds no traversal
    with a trip count greater than that charge's semantic-size request count, including no traversal of `LimitKind::ALL`
    or another independently sized collection. The FR-362 proof schedule has at most two requests per charge, while
    the public `Charge` API retains no request-count ceiling.
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362 schedule contains exactly the
    admitted prefix in charge order, with each record's point and absent-or-present per-admission semantic-size
    maxima matching an independent model. Reordered and repeated requests of the same kind, including zero and distinct
    amounts, coalesce to the per-admission maximum in that record. A refused charge appends no record and leaves
    the existing log unchanged. The named `tc_931_admitted_log_matches_model_ordered_prefix` and `tc_931_admitted_log_matches_model_same_kind`
    tests verify these prope
- id: ChargeAmount
  path: src/accounting.rs
  role: examined
  excerpt: "/// Amounts admissible under a `u64` limit stay in machine width. An exact\n/// amount outside that\
    \ domain is retained out of line only for refusal.\n#[derive(Clone, Eq, PartialEq)]\nenum ChargeAmount {\n \
    \   Units(u64),\n    Exact(Box<Integer>),\n}\n\nimpl ChargeAmount {\n    fn exact(amount: Integer) -> Self {\n\
    \        match amount.to_u64() {\n            Some(units) => Self::Units(units),\n            None => Self::Exact(Box::new(amount)),\n\
    \        }\n    }\n\n    fn units(&self) -> Option<u64> {\n        match self {\n            Self::Units(units)\
    \ => Some(*units),\n            Self::Exact(_) => None,\n       "
- id: InlineSizes
  path: src/accounting.rs
  role: examined
  excerpt: "\nimpl InlineSizes {\n    fn new() -> Self {\n        Self {\n            first: None,\n           \
    \ second: None,\n            overflow: Vec::new(),\n        }\n    }\n\n    fn push(&mut self, kind: LimitKind,\
    \ amount: ChargeAmount) {\n        if self.first.is_none() {\n            self.first = Some((kind, amount));\n\
    \        } else if self.second.is_none() {\n            self.second = Some((kind, amount));\n        } else\
    \ {\n            self.overflow.push((kind, amount));\n        }\n    }\n\n    fn iter(&self) -> impl Iterator<Item\
    \ = &(LimitKind, ChargeAmount)> {\n        self.first\n            .iter()\n           "
- id: Charge
  path: src/accounting.rs
  role: examined
  excerpt: "impl Charge {\n    /// A charge at `point` with no size yet, a default one work unit, and\n    /// no\
    \ result units.\n    pub fn new(point: ChargePoint) -> Self {\n        Self {\n            point,\n        \
    \    sizes: InlineSizes::new(),\n            work_units: ChargeAmount::Units(1),\n            result_units:\
    \ ChargeAmount::Units(0),\n        }\n    }\n\n    /// A semantic-size charge against `kind`.\n    pub fn size(mut\
    \ self, kind: LimitKind, amount: u64) -> Self {\n        self.sizes.push(kind, ChargeAmount::Units(amount));\n\
    \        self\n    }\n\n    /// A semantic-size amount that may exceed `u64::MAX`.\n "
- id: Meter::check_injected
  path: src/accounting.rs
  role: examined
  excerpt: "        &mut self,\n        point: ChargePoint,\n        work_units: &ChargeAmount,\n    ) -> Result<(),\
    \ Incomplete> {\n        if self.cancel.as_ref().is_some_and(Cancel::poll) {\n            let consumed = self.consumed(LimitKind::WorkUnits);\n\
    \            return Err(Incomplete {\n                limit_kind: LimitKind::WorkUnits,\n                limit:\
    \ consumed,\n                consumed,\n                next_charge: work_units.integer(),\n               \
    \ charge_point: point,\n            });\n        }\n        match self.counters.denial {\n            Some(denial)\n\
    \                if denial.point == poin"
- id: Meter::charge
  path: src/accounting.rs
  role: examined
  excerpt: "    pub fn charge(&mut self, charge: Charge) -> Result<(), Incomplete> {\n        let point = charge.point;\n\
    \        self.check_injected(point, &charge.work_units)?;\n        // Keep validation and per-admission maxima\
    \ local until every counter\n        // is known to fit. An earlier field wins even when requested later;\n\
    \        // an equal field keeps its first failing request.\n        let mut sizes = [None; LimitKind::COUNT];\n\
    \        let mut first_fail: Option<(LimitKind, &ChargeAmount)> = None;\n        #[cfg(feature = \"test-support\"\
    )]\n        let mut record = AdmittedCharge::new(point);\n       "
- id: Meter::charge_plan
  path: src/accounting.rs
  role: examined
  excerpt: "    pub fn charge_plan(&mut self, pairs: &Integer) -> Result<(), Incomplete> {\n        let point =\
    \ ChargePoint::EqualityPlan;\n        let reservation = pairs.add(&Integer::from(2_u64));\n        self.check_injected(point,\
    \ &ChargeAmount::exact(reservation.clone()))?;\n        let kind = LimitKind::ValueOccurrences;\n        let\
    \ size = pairs\n            .to_u64()\n            .filter(|size| *size <= kind.limit(&self.counters.limits))\n\
    \            .ok_or_else(|| self.incomplete(kind, pairs.clone(), point))?;\n        let remaining = |kind: LimitKind|\
    \ {\n            Integer::from(kind.limit(&self.counter"
- id: Model::step
  path: src/accounting/kani_proofs.rs
  role: examined
  excerpt: "        requests: &Requests,\n        results: u64,\n    ) -> Result<(), Refusal> {\n        // Each\
    \ branch names the field and counter directly. The oracle does\n        // not reconstruct a field by dispatching\
    \ a merged symbolic kind.\n        macro_rules! size {\n            ($kind:ident, $field:ident, $index:literal)\
    \ => {\n                if let Some(amount) = Self::size_short(limits.$field, requests, LimitKind::$kind) {\n\
    \                    return Err(Refusal {\n                        kind: LimitKind::$kind,\n               \
    \         limit: limits.$field,\n                        consumed: self.cons"
- id: Model::check
  path: src/accounting/kani_proofs.rs
  role: examined
  excerpt: "        assert_eq!(meter.consumed(LimitKind::IntegerBits), self.consumed[0]);\n        assert_eq!(meter.consumed(LimitKind::DecimalDigits),\
    \ self.consumed[1]);\n        assert_eq!(meter.consumed(LimitKind::ScaleExpansion), self.consumed[2]);\n   \
    \     assert_eq!(meter.consumed(LimitKind::TextInputBytes), self.consumed[3]);\n        assert_eq!(meter.consumed(LimitKind::TextScalars),\
    \ self.consumed[4]);\n        assert_eq!(\n            meter.consumed(LimitKind::NormalizedScalars),\n     \
    \       self.consumed[5]\n        );\n        assert_eq!(meter.consumed(LimitKind::UnitEdges), self.consumed[6]);\n\
    \        "
- id: step
  path: src/accounting/kani_proofs.rs
  role: examined
  excerpt: "    let mut charge = Charge::new(point).results(results);\n    for (kind, amount) in requests.into_iter().flatten()\
    \ {\n        charge = if exact {\n            charge.exact_size(kind, Integer::from(amount))\n        } else\
    \ {\n            charge.size(kind, amount)\n        };\n    }\n    let actual = meter.charge(charge);\n    match\
    \ (&actual, &expected) {\n        (Ok(()), Ok(())) => {}\n        (Err(actual), Err(expected)) => {\n      \
    \      assert_eq!(actual.limit_kind, expected.kind);\n            assert_eq!(actual.limit, expected.limit);\n\
    \            assert_eq!(actual.consumed, expected.consumed);\n       "
- id: bits
  path: src/accounting/kani_proofs.rs
  role: examined
  excerpt: '

    // FR-359-AC-9 remains unqualified until the healthy proof and both controls pass.

    #[kani::proof]

    fn meter_integer_arithmetic_schedule_matches_reference() {'
- id: meter_integer_arithmetic_schedule_matches_reference
  path: src/accounting/kani_proofs.rs
  role: examined
  excerpt: "    let limits = ScalarLimits {\n        integer_bits: kani::any(),\n        value_occurrences: kani::any(),\n\
    \        work_units: kani::any(),\n        result_units: kani::any(),\n        decimal_digits: u64::MAX,\n \
    \       scale_expansion: u64::MAX,\n        text_input_bytes: u64::MAX,\n        text_scalars: u64::MAX,\n \
    \       normalized_scalars: u64::MAX,\n        unit_edges: u64::MAX,\n    };\n\n    // The three-charge schedule:\
    \ operands (up to two size requests),\n    // arithmetic (one) and result-retain (none).\n    let operand_bits\
    \ = bits(left).max(bits(right));\n    let (operands, count, arithmetic_bit"
```

## Disposition Round 1

PASS retained. Inspected frozen fix head `8cacb30872e724094260d8bc92ec6432a58c3232`: only two Status wording changes plus original review-file custody. Both Rust files byte-match qualified source. No new finding, no source change, no heavy rerun. No dispositions comment is required for this artifact because it had no findings.

## New findings (disposition pass 2)

Root's actual aggregate at `301ccea9a388715f005e59d07408761a6d34b7fa` reached the complete default integration suite and exposed this obsolete expectation, missed by the initial focused review. `root-final-gate.log` records five bounded-integer tests passing and this sixth test failing at line 591. No new production defect is established.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | The native metered allocation observer still requires a positive size-vector allocation; two inline request slots correctly eliminate that allocation, causing the unchanged integration test and make ci to fail. | tests/ir705_bounded_integer.rs:591-594 |

The required fix must retain independently measured public arithmetic versus explicit accounting-only charge schedules, the actual arithmetic result, refusal path and accounting checks, and a nontrivial assertion that native arithmetic adds no allocations. The old positive-allocation premise is not required by FR-371-AC-4. A concrete zero-allocation expectation for this specific native schedule is grounded in two inline slots, machine-width charge amounts and fitting arithmetic; it does not assert every public metered operation is allocation-free. The original clean placeholder FND-001 is preserved and is not a defect/disposition obligation.

```yaml
new_findings:
- fnd: FND-002
  severity: high
  confidence: high
  method: code-review
  check: test-intent
  artifact_id: FR-371-AC-4
  related_ids:
  - FR-359-AC-8
  path: tests/ir705_bounded_integer.rs
  lines: 591-594
  excerpt: Observation of public metered calls separately identifies ordinary accounting/refusal allocations and
    does not misreport the entire call as allocation-free.
  finding: The native metered allocation observer still requires a positive size-vector allocation; two inline request
    slots correctly eliminate that allocation, causing the unchanged integration test and make ci to fail.
scope:
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Observation of public metered calls separately identifies ordinary accounting/refusal allocations and
    does not misreport the entire call as allocation-free.
- id: native_public_metered_call_matches_accounting_only_allocation_observation
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: "assert!(\n        public_allocations.bytes_total > 0,\n        \"metered size-vector allocation is expected\"\
    \n    );"
```

## Dispositions

Round 2: independently inspected actual fix head `c769905d7946697d9814eb7cac61a3fd4ca81384`, one integration-test file only. FND-002 is the only real pending finding; FND-001 is the original clean placeholder and is excluded.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | The independent accounting-only schedule now must allocate zero bytes, and the unchanged separately measured public path must equal it. Both observations are exercised; equality is not self-comparison or deletion of the allocation gate. Completed Integer 11, actual public refusal, separately executed refusal baseline and their allocation equality remain unchanged. Other existing arithmetic/counter/denial assertions remain intact. The trace attribute is confined to the actual default-only test, with a matching cfg-gated import. |

```yaml
dispositions:
- fnd: FND-002
  outcome: fixed
  after_excerpt: "    assert_eq!(\n        baseline_allocations.bytes_total, 0,\n        \"the two-request arithmetic\
    \ charge schedule stays inline\"\n    );\n    assert_eq!(\n        public_allocations.bytes_total,\n       \
    \ baseline_allocations.bytes_total\n    );"
```

## Current Disposition Verdict

PASS. No unresolved source/test finding blocks this PR. The independent accounting-only schedule now must allocate zero bytes, and the unchanged separately measured public path must equal it. Both observations are exercised; equality is not self-comparison or deletion of the allocation gate. Completed Integer 11, actual public refusal, separately executed refusal baseline and their allocation equality remain unchanged. Other existing arithmetic/counter/denial assertions remain intact. The trace attribute is confined to the actual default-only test, with a matching cfg-gated import.

Actual raw allocation-fix-exact.log records the repaired observer test passing; allocation-fix-default.log records all six integration tests passing; allocation-fix-support.log records all five applicable tests passing, with the default-only allocation observer excluded as before. The existing test-support diagnostic log legitimately allocates and is not folded into this no-extra native arithmetic measurement. Both production accounting and proof files remain byte-identical to the healthy qualified source. No new production behavior, reduced input domain, changed oracle, dummy test, disabled assertion or heavy reviewer run was introduced. These focused checks do not replace the root final aggregate rerun.
