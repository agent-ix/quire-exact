---
id: TC-955
title: "The native enum carrier matches the owner's compare_enum and verifies"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-407
    type: verifies
---
# TC-955: The native enum carrier matches the owner's compare_enum and verifies

## Description

Verify FR-407-AC-1 through FR-407-AC-9 through the public kernel API, a host differential beside the owner's `compare_enum` and the recorded Kani measurement. The differential runs where both the owner and the kernel are visible. Each mutant names the input or inspection that fails it.

## Test Procedure

1. For AC-1, check `Copy` by a generic bound, no drop glue by `needs_drop`, and zero allocations of construction, comparison and the round-trip conversion in a counting allocator window; check that no field is public (a struct-literal program fails to compile with `E0451`).
2. For AC-2, call `from_index` with `(count - 1, count)`, `(count, count)`, `(count + 1, count)`, `(u32::MAX, 3)` and `(0, 0)` and assert the refusal or the carrier as stated, with a `Meter`-free signature (coerce to a function pointer).
3. For AC-3, for an ordered and an unordered `EnumShape` of three members assert `from_member` equals `from_index` for each member; assert `MemberNotInShape` for a wrong rank and for a foreign variant of the same rank.
4. For AC-4 and AC-5, evaluate the six operators over every ordered pair of three members for an ordered and an unordered declaration with literals written independently of the carrier; then operands with different tags, with the same tag and a different count, and with the same tag and a different ordered flag, and an ordering over an unordered declaration paired with distinct declarations (cause precedence), each against a recording `Meter` that stays unchanged.
5. For AC-6 and AC-7, run the differential of FR-407 Behavior 10 beside the owner's `compare_enum` (an ordered and an unordered declaration of 1, 2, 3 and 16 members, tags assigned one per owner declaration node), comparing outcome, cause, charge points with limit kinds and amounts, the denied charge, counters and the incomplete record at a sufficient, an exact and a one-less limit.
6. For AC-8, scan the comparison source for the banned tokens and callees of FR-407 Behavior 8; run the scan over one seeded violating copy per banned construct.
7. For AC-9, once the code lead authorizes the run, execute the harness of FR-407 Behavior 9 under the authorized Kani runner and record time, peak memory with provenance, tool version and outcome in the evidence record below; confirm every cover is reached and the mutant fails it. Until then this step is not executable.
8. Run each mutant through the step it names.

### Mutants

| Mutant | Fails at |
| --- | --- |
| `Less` is evaluated as `LessOrEqual` | the equal-rank pair of step 4 |
| `Equal` compares the declaration tag instead of the rank | the unequal-rank pair of step 4 |
| An ordering over an unordered declaration returns a Boolean | the unordered case of step 4 |
| The distinct-declaration check runs after the unordered check | the cause-precedence case of step 4 |
| The distinct-declaration check ignores the member count or the ordered flag | the differing-count and differing-flag operands of step 4 |
| The first `EnumIdentityRead` charges size 2 | the charge comparison of step 5 |
| `EnumResultRetain` is charged before the comparison or not at all | the charge comparison of step 5 |
| A refused call charges the `Meter` | the unchanged-`Meter` check of step 4 |
| `from_index` accepts `index == member_count` | step 2 |
| `from_index` wraps a large index modulo the count | the `(u32::MAX, 3)` case of step 2 |
| `from_member` does not check the rank | the wrong-rank member of step 3 |
| The comparison delegates to `compare_enum` | the call scan of step 6 |
| The comparison body is selected by `cfg(kani)` | the token scan of step 6 |
| The carrier owns a `Box<str>` | step 1 |

## Evidence record

No measurement exists. Columns: operation, outcome, wall time, peak memory, Kani version, harness id.

| Operation | Outcome | Wall time | Peak memory | Kani version | Harness id |
| --- | --- | --- | --- | --- | --- |

## Expected Results

Every step passes, each mutant fails at the step it names, and the evidence record holds an executed row for FR-407-AC-9 or the criterion stays planned.

## Status

Specified (IR-262). No step has run.
