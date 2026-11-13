---
id: SR-2026111319100500000001
title: "IR-262 PR #164 Kani-tractable native enum carrier code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #164 @ 7a219689 on origin/main 8a51569; IR-262; src/native_enum.rs, src/native_enum/kani_proofs.rs, src/lib.rs, tests/ir262_native_enum.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-407
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-955
    type: reviews
---

## Summary

Ticket: IR-262. PR #164 (head 7a219689, branch code/ir-262-native-enum-carrier), scoped to `git diff origin/main...HEAD` (6 files, +741/-2). Methods: code-review with the rust-review lane folded in. Reviewed read-only in /home/peter/dev/worktrees/ir-262-enum-code against FR-407 and TC-955 on origin/main (authoritative) and the owner `compare_enum` in quire-semantic-value origin/main `src/enumeration.rs:292`. Kani was not run and `kani_proofs.rs` was not compiled (read only). No new build: the existing test binaries in `ir262-target` (newer than the sources) were executed: 15/15 pass without `test-support`, 14/14 with it.

## Verdict

**PASS with findings (one medium, one low).** No wrong result, panic or spec violation in the carrier or the comparison. Mergeable once FND-001 is settled; FND-002 is a wording fix for the same round.

Checked and found correct:

- **Owner equivalence.** `compare_native_enum` follows the owner's `compare_enum` exactly: distinct declaration first (`DistinctEnumDeclarations`, here tag, count or ordered flag per FR-407 Behavior 2), then ordering over an unordered declaration (`UnorderedEnumOrdering`), both before any charge; then `EnumIdentityRead` size `ValueOccurrences` 1, `EnumIdentityRead` size `ValueOccurrences` 2, the comparison, then `EnumResultRetain` `results(1)`, each `?`-stopping, wrapped by `Outcome::from_stop` as the owner's `outcome_from_stop`. Rank order replaces the owner's position/member-key order as Behavior 8 states.
- **Constructors.** `from_index` is `const`, Meter-free, refuses `index >= member_count` (zero count included), never clamps. `from_member` refuses `MemberNotInShape` when `shape.rank(member.variant()) != Some(member.rank())`, so wrong rank, foreign variant and a minted `VariantId` (other owner key, same name and rank) all refuse; count and flag come from the shape. The `u32::try_from` count failure is unreachable in practice (`EnumShape::rank` already `expect`s `u32`) but is the checked conversion Behavior 3 asks for.
- **Carrier.** Four private fields, `Copy`, no drop glue, derived `Eq` over all four fields, `parts()` lossless. `NativeEnumRefusal` is `Copy` with `thiserror` (`default-features = false`, no_std-safe, as other modules).
- **lib.rs** adds only `mod native_enum`, the `cfg(kani)` `#[path]` proof module (same pattern as `native_text_kani_proofs`) and one `pub use`. Minimal.
- **Kani harnesses (read only, not compiled).** No compile error found: every name used exists and is crate-visible (`accounting::{Meter, ScalarLimits}`, `comparison::{ComparisonOperator, IllTypedCause}`, `ComparisonOperator::ALL`, `Meter::admission_count` is not `test-support`-gated, `outcome::Outcome`, public `IllTyped::cause`); `kani::cover!(cond, "lit")` form is right; the `ScalarLimits` literal matches the host test's. Assumes are satisfiable (`count > 0`, `index < count`). Covers are on the Completed, Incomplete, unordered-refusal, distinct-refusal and same-declaration paths, all reachable. Mutant sensitivity traced by hand: `Less` as `LessOrEqual`, `Equal` over tags, ordering on unordered, swapped refusal order, ignoring count or flag in the distinct check, a refused call charging, first read at size 2, retain before the reads, retain at 2 results: each makes an assertion fail. No unwind attribute is needed (only the six-element `ALL` loop), matching `native_integer`'s Meter proofs.
- **No ceremony** in the code: no pins, SHAs, digests, version records. `#[trace("TC-955", "FR-407-AC-n")]` tags are on every host test for AC-1..AC-6 and AC-8; Kani proofs untagged, matching the repo's other `kani_proofs.rs`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-8 is verified by a source-text token scan (`include_str!("../src/native_enum.rs")`, substring search after `pub fn compare_native_enum`). A test that scans source text is the antipattern PR #163's review removed from FR-408/TC-956, which made the equivalent FR-408-AC-10 "Inspection (code review)" with no automated step. The scan is also vacuous for what it claims: it passes for a loop written as `.fold(`/`.into_iter()`/recursion, or a body moved into a helper defined above the marker, and it would fail on a doc comment containing a banned word. FR-407-AC-8 as merged says Inspection and TC-955 step 6 says scan, so the test follows the spec; but even by the spec it is incomplete (no seeded violating copies). Fix: delete the test, amend FR-407-AC-8 Verification to "Inspection (code review)", TC-955 step 6 to a code-review inspection with no automated step, and drop the `cfg(kani)` mutant row (the Kani harness running the shipped function is the one-source evidence), as FR-408/TC-956 do. This review performed the inspection: the comparison body names no `cfg(kani)`, loop, iterator adapter, allocation, `num_bigint`, `Integer`, `NodeKey` or `EnumValue` | tests/ir262_native_enum.rs:419, spec/functional/FR-407-kani-tractable-native-enum-carrier.md:69, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:23, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:42 |
| FND-002 | low | `kani_proofs.rs` module doc says "Written and compile-checked only", but nothing compiled it: `cargo test`/`clippy` never build `cfg(kani)` code and no `cargo kani` codegen was run (PR body: "written, not run"). The harness doc at line 48 also says it checks "the consumed counters", but it asserts only `admission_count()`. Fix: say "written, not compiled or run", and either assert `meter.consumed(..)` on the Completed path or drop "with the consumed counters" | src/native_enum/kani_proofs.rs:5, src/native_enum/kani_proofs.rs:48 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-407-AC-1 | examined | NativeEnum is Copy, has no drop glue, holds no NodeKey, VariantId, Arc, String, Box or Vec, and has no public field; construction, comparison and the round-trip conversion perform zero allocations under a counting allocator. |
| FR-407-AC-2 | examined | from_index refuses index == member_count, index == member_count + 1, u32::MAX against a smaller count and any index with a count of zero, each with IndexOutOfRange and no carrier, takes no Meter, never clamps or wraps, and admits 0 and member_count - 1. |
| FR-407-AC-3 | examined | from_member agrees with from_index for every member of an ordered and an unordered EnumShape, refuses a member whose rank differs from the shape's rank for its variant and a foreign variant with MemberNotInShape, and takes the count and ordered flag from the shape. |
| FR-407-AC-4 | examined | For the six operators over every ordered pair of an ordered enum of three members, the Boolean equals the native operator over the indices; for an unordered enum Equal and NotEqual equal the same and the four orderings are refused. |
| FR-407-AC-5 | examined | Operands with different tags, or the same tag but a different count or ordered flag, are refused with DistinctEnumDeclarations, which takes precedence over UnorderedEnumOrdering; a refused call leaves the Meter unchanged. |
| FR-407-AC-6 | examined | Against a literal expected charge sequence on a recording Meter, the charge points, order, limit kinds and amounts of Behavior 7, the denied charge and the incomplete record are as stated at a sufficient, an exact and a one-less limit. |
| FR-407-AC-8 | examined | The comparison source contains no cfg(kani), loop, iterator adapter, allocation, num_bigint item, owner Integer, NodeKey or EnumValue. |
| FR-407-AC-9 | examined | PLANNED: the Kani harness of Behavior 9 returns an executed terminal VERIFIED verdict, its cover paths are reached and its mutant fails. |
| QSV compare_enum | context_only | quire-semantic-value src/enumeration.rs:292: declaration/ordered/members check, unordered check, two EnumIdentityRead sizes 1 and 2, compare, EnumResultRetain results 1. |
