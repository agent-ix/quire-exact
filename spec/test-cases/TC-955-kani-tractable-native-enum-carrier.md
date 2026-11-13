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

Verify FR-407-AC-1 through FR-407-AC-9 through the public kernel API, a literal-charge test, the owner differential and the recorded Kani measurement. The differential lives in quire-semantic-value's `tests/` under its own QSV ticket; quire-exact cannot depend on it. Each mutant names the input or inspection that fails it.

## Test Procedure

1. For AC-1, check `Copy` by a generic bound, no drop glue by `needs_drop`, and zero allocations of construction, comparison and the round-trip conversion in a counting allocator window; check that no field is public (a struct-literal program fails to compile with `E0451`).
2. For AC-2, call `from_index` with `(count - 1, count)`, `(count, count)`, `(count + 1, count)`, `(u32::MAX, 3)` and `(0, 0)` and assert the refusal or the carrier as stated, with a `Meter`-free signature (coerce to a function pointer).
3. For AC-3, for an ordered and an unordered `EnumShape` of three members assert `from_member` equals `from_index` for each member; assert `MemberNotInShape` for a wrong rank and for a foreign variant of the same rank.
4. For AC-4 and AC-5, evaluate the six operators over every ordered pair of three members for an ordered and an unordered declaration with literals written independently of the carrier; then operands with different tags, with the same tag and a different count, and with the same tag and a different ordered flag, and an ordering over an unordered declaration paired with distinct declarations (cause precedence), each against a recording `Meter` that stays unchanged.
5. For AC-6, inside quire-exact, run every operator over a three-member ordered declaration under a recording `Meter` at a sufficient, an exact and a one-less limit for each limit kind used, and compare the charge points, limit kinds, amounts, denied charge and incomplete record with literals written independently of the carrier. For AC-7, in quire-semantic-value `tests/` (own QSV ticket, after the carrier is on quire-exact main), run the differential of FR-407 Behavior 10 beside the owner's `compare_enum`: an ordered and an unordered declaration of 1, 2, 3 and 16 members, one tag per distinct (declaration key, ordered flag, member list), two owner values with one key and different same-length member lists, and a `Meter` at the three limits, comparing outcome, cause, charges, denied charge, counters and incomplete record.
6. For AC-8, inspect the comparison source in code review for the banned constructs of FR-407 Behavior 8. There is no automated step.
7. For AC-9, once the code lead authorizes the run, execute the harness of FR-407 Behavior 9 under the authorized Kani runner and record time, peak memory with provenance, tool version and outcome in the evidence record below; confirm every cover is reached and the mutant fails it. Until then this step is not executable.
8. Run each mutant through the step it names.

### Mutants

| Mutant | Fails at |
| --- | --- |
| `Less` is evaluated as `LessOrEqual` | the equal-rank pair of step 4 and the differential of step 5 |
| `Equal` compares the declaration tag instead of the rank | the unequal-rank pair of step 4 |
| An ordering over an unordered declaration returns a Boolean | the unordered case of step 4 |
| The distinct-declaration check runs after the unordered check | the cause-precedence case of step 4 |
| The distinct-declaration check ignores the member count or the ordered flag | the differing-count and differing-flag operands of step 4 and the differential |
| The first `EnumIdentityRead` charges size 2 | the literal charge test of step 5 (AC-6) and the differential |
| `EnumResultRetain` is charged before the comparison or not at all | the literal charge test of step 5 (AC-6) and the differential |
| A refused call charges the `Meter` | the unchanged-`Meter` check of step 4 |
| `from_index` accepts `index == member_count` | step 2 |
| `from_index` wraps a large index modulo the count | the `(u32::MAX, 3)` case of step 2 |
| `from_member` does not check the rank | the wrong-rank member of step 3 |
| The carrier owns a `Box<str>` | step 1 |

## Evidence record

One measurement exists, a superseded harness that did not finish. Columns: operation, outcome, wall time, peak memory, Kani version, harness id.

| Operation | Outcome | Wall time | Peak memory | Kani version | Harness id |
| --- | --- | --- | --- | --- | --- |
| Same-declaration compare over symbolic meter limits (superseded) | did not finish (exit 124) | 1800 s limit | not recorded | not recorded | `native_enum_compare_same_declaration` |

## Expected Results

Every step passes, each mutant fails at the step it names, and the evidence record holds an executed row for FR-407-AC-9 or the criterion stays planned.

## Status

Steps 1 to 4 and the quire-exact half of step 5 (AC-6) are implemented in `tests/ir262_native_enum.rs`; the E0451 check of step 1 is a `compile_fail,E0451` doctest on `NativeEnum` in `src/native_enum.rs`. Step 6 is a code-review inspection with no automated step. The differential half of step 5 (AC-7) is not run here. Step 7 is not executed: the harnesses `native_enum_compare_completes`, `native_enum_compare_denies_second_read`, `native_enum_compare_denies_retain`, `native_enum_refuses_distinct_declarations` and `native_enum_refuses_unordered_ordering` compile under `cargo kani --only-codegen` but have not been run, and no Kani verdict exists. Step 8 (mutants) has not been run.
