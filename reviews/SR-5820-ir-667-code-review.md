---
id: SR-5820
title: code-review of IR-667 PR30 frozen implementation
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact@33bcd5cfd4a2d0f9cccdad09a77697cf9b00d99a; src/accounting.rs,
  src/division.rs, src/ieee.rs, src/lib.rs, tests/ir653_admission.rs, tests/ir653_decimal_order.rs,
  tests/ir653_scalar.rs, tests/ir667_normalize.rs, tests/ir673_core.rs, tests/ir673_scalar.rs
review_set: subset
---

## Summary

Independent code/Rust review of the frozen IR-667 implementation diff. No changed-code defects found.

## Verdict

**PASS** for the exact PR diff only. This is not a whole-repository assurance verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

Base: `6359cce0f7d58c738f41ea0d0d37ed64461d0b54`. Reviewed frozen head: `33bcd5cfd4a2d0f9cccdad09a77697cf9b00d99a`. Ticket: IR-667. Scope is the PR diff only; unchanged surrounding numeric logic was read as context. No source edits, builds, tests, locks or mutation runs were performed by this reviewer. No applicable AssuranceProfile exists in this repository's spec tree.

The ten changed files are: `src/accounting.rs`, `src/division.rs`, `src/ieee.rs`, `src/lib.rs`, `tests/ir653_admission.rs`, `tests/ir653_decimal_order.rs`, `tests/ir653_scalar.rs`, `tests/ir667_normalize.rs`, `tests/ir673_core.rs`, `tests/ir673_scalar.rs`.

## Examined Scope

Scope units, including clean units, are listed below. Excerpts are verbatim criterion prefixes when a criterion exceeds 600 characters. Source-file entries identify the diff units examined.

```yaml
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: The `quire-exact` meter SHALL deny an injected, named charge exactly once
    at its selected 1-based occurrence. Where `test-support` is enabled, the meter
    SHALL bound the admitted-charge log to 4096 entries without changing exact accounting.
    The public `InjectedDenial::occurrence` SHALL be `NonZeroU64`, making zero unrepresentable.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor
    returns all 4096 records with their points in order and `charge_log_truncated()`
    is false. On the 4097th admission the slice still contains exactly that prefix
    and the flag becomes true.
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
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request
    of 9 followed by 3 and then a charge with no size request. The records expose
    respectively 9, 3 and no IntegerBits request while the consumed IntegerBits maximum
    remains 9. A separate charge with IntegerBits requests 9, 3 and zero exposes present
    9; a charge with any positive number of IntegerBits zero requests exposes present
    zero. Both have the same fixed payload size as a single-request record, with no
    heap-owned request collection and no retained request list, order or multiplicity.
    A successful `charge_plan` with pai
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently
    chosen semantic-size requests, including repeated same-kind requests and distinct
    amounts at admissions 4096 and 4097. The single record log contains exactly the
    first 4096 fixed-size records with their points and per-admission semantic-size
    maxima in admission order; truncation is false at admission 4096 and true at 4097.
    Work consumption and admission count reach 4097, including the omitted record;
    existing size maxima still include the 4097th charge.
- id: FR-362
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: When evaluating kernel-owned integer or rational arithmetic, integer or
    rational ordering, or a Boolean connective over already-decided operands, the
    `quire-exact` kernel SHALL return the exact outcome and charge the point order
    and amounts defined below.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract,
    multiply, divide and negate, and integer/rational ordering, independently calculate
    the required bit high-water amount from Behavior. On fresh meters with other limits
    sufficient, that exact bit limit completes with the expected value, while a limit
    one below stops at the first point whose amount exceeds it, with exact prior consumption,
    request and admitted prefix and no result unit. Include cancellation `1000 - 999`,
    `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate
    `-5/8`, and `7/3 < 5/8`. '
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On fresh sufficient public meters under `test-support`, evaluate the six
    rational calls listed below with no result domain. Each completes to the listed
    canonical value and admits operands, arithmetic, normalize and result-retain in
    order, consuming four work units and one result unit. The actual normalize record
    has only IntegerBits present, with the listed amount independently derived from
    the unreduced parts and `B(0) = 1`. The recorded arithmetic amount equals its
    separate listed bound. The normalize observation is not replaced by that bound,
    the final bit maximum or the reduced value's pa
- id: src/accounting.rs
  path: src/accounting.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: src/division.rs
  path: src/division.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: src/ieee.rs
  path: src/ieee.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir653_admission.rs
  path: tests/ir653_admission.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir653_decimal_order.rs
  path: tests/ir653_decimal_order.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir653_scalar.rs
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir667_normalize.rs
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: 'Six public API tests: actual request projections, refusal atomicity, prefix
    truncation, rational normalize, integer arithmetic and ordering boundaries.'
- id: tests/ir673_core.rs
  path: tests/ir673_core.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir673_scalar.rs
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
```

## Execution Evidence

Independently read receipt, subject events and stdout/stderr at `/tmp/ix-handoff/ir667-normalize-focused/full-candidate/attempt-2c7de383903d46e0975b9e6c3aa9ea42`: head matches the frozen candidate; make-ci and Quire validation both exit 0; scope teardown CONFIRMED. Logged make-ci commands cover fmt, default/support all-target Clippy, thumbv7em-none-eabi library Clippy/build, both test configurations, deny and documentation with warnings denied. The logs report 190 default and 199 test-support test executions, zero failed/ignored. Six new ir667_normalize tests pass in the support lane. Quire reports 164/164 documents grammar-clean, zero grammar findings. This is reviewed author-run evidence, not reviewer execution. Stable-rustfmt unsupported-option warnings and ambient duplicate archetype/inverse warnings are disclosed; no gate was weakened by the diff. No CI, Cargo, lockfile or specification file changed.

Normalize-undercharge proposal was read as source-derived reasoning only. No executed counterfactual or mutant kill is claimed.

## Coverage

Code review and the Rust lane cover repository idioms, test trace/naming/placement/determinism, real public API seams, source/test completeness and oracle discrimination, lint integrity, panic/unsafe surface, numeric conversion safety, async/blocking, state lifecycle, external contracts and resource bounds. No new wire/async/unsafe boundary is present. Repository AGENTS/CLAUDE, Cargo lint settings, clippy, rustfmt and deny conventions were loaded before analysis and the Rust checklist was applied again for final pre-handoff review.

`AdmittedCharge` carries the existing closed ChargePoint and eight Option<u64> fields; an exhaustive LimitKind match separates semantic requests from cumulative kinds. observe coalesces actual validated amounts within the current admission. charge constructs/commits its record only after every size and cumulative work/result admission succeeds. charge_plan records actual pairs, excluding its pairs+2 work reservation. Cancellation and injection return before record creation. The single existing Vec retains at most 4096 records, with truncation at admitted record 4097; production cfg removes the entire record/log surface. Payload has no heap-owned member. Existing charge staging allocations and accounting logic are unchanged.

All actual local admitted_charges callers were inventoried. Existing point comparisons project `.point`; equality/length/empty callers remain meaningful. No compatibility layer or copied external source is added. All six new tests have real trace tags and exercise public production operations with test-support observing those operations rather than replacing them.

The rational table has six literal controls matching AC20, separate arithmetic and actual normalize request assertions, canonical outcomes and exact work/results. The multiply 6/6 -> 1/1 case discriminates unreduced normalization from reduced-value accounting. Replacing that normalize amount with 1 would leave the arithmetic high-water at 4 but fail the direct record assertion expecting 3; this is a source inference, not an executed mutation result. AC10 spans all integer/rational arithmetic and both orderings at formula-derived exact/one-under boundaries, including unary negate whose first short point is operands. Additional operand-limit controls in integer/rational arithmetic directly verify first-stop metadata and atomic admitted prefix. The corrected refusal helper expects work equal to the number of prior admitted points, so it can fail for undercharging, incorrect requests/points, or committing later results.
