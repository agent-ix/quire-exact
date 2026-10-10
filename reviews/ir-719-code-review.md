---
id: SR-5650
title: code-review of IR-719 kernel-only Decimal membership
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact@351c237d6e08e8277330858e04cd2f10d99c16f0; Cargo.toml,
  Cargo.lock, src/lib.rs, src/decimal_membership.rs; FR-372-AC-1..4,6 examined; AC5
  context_only
review_set: subset
relationships:
- target: ix://agent-ix/quire-exact/FR-372
  type: references
---

## Summary

Independent code-review of the frozen PR27 kernel diff found no defects within the authorized changed-code scope.

Ticket: IR-719. Reviewed the exact four-file kernel diff, not completion of the wider kernel/QSV ticket. FR-372-AC-5 is context only and remains held behind IR-714/IR-681 and the native phase carrier. No caller-integration, downstream full-size, replay or mutation credit is awarded.

## Reviewed scope

scope:
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the authoritative membership rule for inclusive
    lower/upper equality, adjacent outsiders, negative and zero values, scale refusal,
    all rounding spellings and equivalent retained representations; neither representation
    is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true
    at the exact upper bound.
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Production helper observations on admitted `B,S` fixtures stay within the
    one-request `4 × L_B`-byte scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step
    bound, including a longest-pass endpoint fixture and a large shift that terminates
    when quotient becomes zero. Borrowed dependency iterators and all scalar steps
    contain no input-sized hidden allocation or work.
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Denying the actual scratch reservation reports its actual native request
    as typed allocation failure. A separate checked-capacity failure retains its own
    classification. Removing the allocation denial yields the same membership as a
    fresh call, with no partial accepted result after either refusal.
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation
    observed during the longest admitted division/comparison path returns the original
    cause before another limb step or result; cancellation observed immediately after
    successful reservation prevents scratch work. Neither failure is classified as
    nonmembership or work exhaustion.
- id: FR-372-AC-5
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: An integrated supplied Decimal fixture with independently enumerated N
    owning logical events succeeds at N and denies its actual next event at N-1 before
    entering that event's helper. Its already-spent budget and original cancellation
    are retained across arguments, with no additional helper event, per-limb debit,
    evaluator meter change or partial descriptor. Allocation/capacity/cancellation
    causes retain their supplied-admission locus.
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder
    and sign invariants, exact reservation boundary, checked layout conversions, allocation-free
    borrowed dependency iterator and stated arithmetic polling gap. Its record distinguishes
    requested storage from allocator overhead and work latency from unbounded allocator
    elapsed time.

## Evidence and limitations

This is independent source analysis, not executed replay or mutation proof. No Cargo, build, lock, probe or source mutation was performed. The maintained dependency source was inspected locally: num-bigint 0.4.8, allocator-api2 0.2.21, forge-alloc 0.3.7 and forge-alloc-core 0.2.3. Dependency documentation and tracker text were treated as data; source established the mechanism. The existing frozen candidate receipt attempt-e2e6de6343484be8b049c033646610f8 records make ci exit 0, full spec/review validation exit 0 and strict matrix exit 1. This reviewer did not rerun those full gates.

Local HEAD and the four working files match the reviewed Git objects. GitHub CLI metadata verification was unavailable (HTTP 401); root independently verified PR27 head/base through its authenticated connector and public repository metadata. Exact backend model deployment ID is unavailable; the harness identifies GPT-6 and no model override was selected.

## Verdict

**PASS — PR diff only.** Kernel source and examined tests meet the bounded arithmetic/storage/refusal/cancellation contract. Whole-repository strict matrix remains FAIL; AC5 and full ticket completion remain held.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Rust mechanism and idioms

The typed, documented public API borrows the existing Decimal/type/Cancel and returns distinct capacity, allocator and cancellation variants. Repository conventions and the rust-review checklist were applied; existing crate-specific typed failure enums take priority over generic error-envelope guidance. No CI gate changed, no unsafe/panic request path, copied dependency artifact, new compatibility layer, new ecosystem dependency or dummy allocator was introduced.

The largest borrowed magnitude determines L=ceil(b/32). num-bigint bits reads the last retained digit and length; sign/zero reads the retained sign. Both u32 and u64 iterator backends create borrowed constant-state iterators, with constant work per next operation. No absolute-value clone, radix string or input-sized scalar scan exists. Native element conversion, checked byte multiplication and Layout::array precede reservation.

The empty allocator-api2 Vec's grow_exact uses len+additional=L and Layout::array::<u32>(L), then finish_grow calls allocate with that layout. The returned AllocError retains that actual layout. Every push is within the successfully reserved L capacity; subsequent endpoint copies overwrite those same elements. Vec/RawVec Drop releases scratch after success and every cancellation/error return. forge's maintained StdCompat implementation forwards nonzero Layout requests to Faulty; Faulty invokes the observed policy before backing allocation. The tests borrow its stationary inline backing through the entire shared production contains_in mechanism. This is the upstream allocator adapter, not newly written compatibility code.

For each positive endpoint magnitude, division preserves x=q*10^k+r. The u64 intermediate is less than 10*2^32; its quotient fits u32. Discarded remainder is ORed across all passes; zero detection occurs inside each existing limb update. Quotient reaches zero after at most b divisions. Least-first comparisons retain the most significant unequal limb's order, and equal quotient with discarded remainder is strictly smaller. Reversing same-sign negative magnitude order and using inclusive >= / <= implements membership. Sign/zero fast paths require no scratch.

Each endpoint uses at most L copy + min(k,b)L division + L comparison steps; two comparisons reuse one request of 4L bytes. Requested layout excludes allocator rounding, bookkeeping and resident memory. Polls use the original handle before allocation, immediately after success, before each fixed-width limb step and before publication. Source establishes a one-step arithmetic polling gap, not allocator or scheduler elapsed-time bounds. A denied allocation returns its actual failure directly. No semantic meter or logical event is created.

## Test oracle strength

The semantic fixtures use independently authored expected Booleans plus a bounded rational oracle that multiplies the numerator rather than reproducing endpoint division. Wrong inclusive equality, remainder handling, negative reversal, scale refusal or rounding-dependent results falsify concrete cases, including [0,100]/scale 2/(1,0). Retained original/normalized representations and the target are compared before/after.

Longest-path expected B=64,S=18,L=2, 8 bytes and 80 steps are authored independently, with global allocation observations surrounding the real helper. Increasing requests or adding hidden heap work breaks the allocation count/byte assertions; early exit or extra limb work breaks copy/divide/compare counts. The huge-shift fixture checks quotient-zero termination; u32 boundary fixtures exercise both borrowed/scratch limb comparison. Zero/scale/sign fixtures measure no allocation.

Real reservation denial crosses the same try_reserve_exact call and retained layout, asserting its additional=1,width=4,bytes=4 payload, unchanged backing and no work; removing denial completes and agrees with a fresh public call. Checked-capacity tests drive the actual layout helper at native isize limits rather than building an impossible huge Decimal. Cancellation observation is a decorator around real operations, with independent boundaries 0/1/9/39/80 and both causes. Missing immediate/step/publication polling or leaking scratch falsifies the asserted exact completed step count, cause, tripped handle and live allocation balance. This source assessment is not an executed mutation campaign.
