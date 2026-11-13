---
id: FR-407
title: "A Kani-tractable native enum carrier and a compare_enum replacement"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: references
  - target: ix://agent-ix/quire-exact/FR-402
    type: references
  - target: ix://agent-ix/quire-exact/FR-404
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---
# FR-407: A Kani-tractable native enum carrier and a compare_enum replacement

## Description

The kernel SHALL provide a native enum carrier, a small `Copy` value holding a member's rank, its declaration's member count and ordered flag, and an opaque declaration tag, with no `NodeKey`, `Arc`, `String`, `Box` or other heap type, and a replacement of the owner's `compare_enum` over two such carriers (Behavior 1 to 10).

The owner's `EnumValue` (quire-semantic-value) holds a `NodeKey` (an `Arc`), a member `NodeKey` and a `Box<str>`. Comparing two of them walks heap structure, which is the shape that made the general `Integer` route time out or exhaust memory under Kani. A comparison of two enum members needs only the member rank, whether the declaration is ordered, and whether the two operands share a declaration: [FR-088](./FR-088-enum-and-unit-identity-in-the-kernel.md) already fixes the rank as the member's zero-based index in the canonical member list, and the owner orders by that position and equates by member identity, which within one declaration is the same relation as rank equality.

The consumer is the enum row of Contract CodeGen (CG FR-049, IR-262). CG's harness, its measurements and its credit are CG's and are not this requirement's acceptance (as for FR-402 and FR-404). Kani execution here is limited to the proofs of Behavior 9, whose verdict the code lead records. The Integer, Decimal, Text, quantity and float carriers are FR-384, FR-402, FR-404, FR-398 and FR-405.

## Use case

A generated harness for `a < b` over an `ordered enum` of three members draws `a` and `b` as symbolic `i64` indices constrained to `0..3`, builds two carriers with `NativeEnum::from_index`, calls `compare_native_enum` under a `Meter` and compares the Boolean and the charges with the native expression `a < b`.

## Behavior

### Carrier

1. A `NativeEnum` SHALL hold a rank (`u32`), a member count (`u32`), an `ordered` flag and a declaration tag (`u64`), SHALL be `Copy`, SHALL have no drop glue and SHALL hold no `NodeKey`, `VariantId`, `Arc`, `String`, `Box` or `Vec`. Its fields SHALL be private, and a value SHALL exist only if `rank < count`.
2. The declaration tag SHALL be an opaque value supplied by the caller. Two operands are of one declaration exactly when their tags, member counts and ordered flags are all equal. The kernel SHALL NOT derive a tag from a `NodeKey` and SHALL NOT claim that distinct declarations have distinct tags: that mapping belongs to the caller, which assigns one tag per admitted declaration node.

### Construction

3. `NativeEnum::from_index(declaration, ordered, index, member_count)` SHALL be a total constructor that takes no `Meter` and charges nothing. It SHALL return the refusal `IndexOutOfRange` when `index >= member_count` (a `member_count` of zero included, so a zero-member declaration yields no carrier), and SHALL NOT clamp, wrap or mask the index. `index` and `member_count` are `u32`; a caller holding a wider index or count SHALL convert with a checked conversion, whose failure is the caller's `IndexOutOfRange`.
4. `NativeEnum::from_member(declaration, &EnumShape, &EnumMember)` SHALL build the carrier from the kernel's own admitted shape and member (FR-088) and SHALL return `MemberNotInShape` when `shape.rank(member.variant())` is not `Some(member.rank())`, and `CountExceedsCarrier` when the shape holds more than `u32::MAX` variants. It SHALL take the ordered flag and the count from the shape and SHALL NOT mint, rename or reorder a variant.
5. Structural equality of two carriers SHALL be equality of all four fields. The conversion from a carrier back to `(index, member_count, ordered)` SHALL be lossless and total.

### Comparison

6. `compare_native_enum(operator, &left, &right, &mut Meter)` SHALL return `Result<Outcome<bool>, IllTyped>` and SHALL follow the owner's `compare_enum` in this order: if the operands are not of one declaration (Behavior 2), return `Err(IllTyped)` with `IllTypedCause::DistinctEnumDeclarations`; else if the operator is an ordering operator and the declaration is not ordered, return `Err(IllTyped)` with `IllTypedCause::UnorderedEnumOrdering`; in both cases consume nothing from the `Meter`.
7. Otherwise the comparison SHALL charge the `Meter` exactly as the owner does: `EnumIdentityRead` with size `ValueOccurrences` 1, then `EnumIdentityRead` with size `ValueOccurrences` 2, then, after the comparison and before returning, `EnumResultRetain` with `results` 1. A denied charge SHALL stop at that charge with the owner's incomplete record, and a later charge SHALL NOT be made.
8. The Boolean SHALL be `left.rank OP right.rank` for the six operators, which for `Equal` and `NotEqual` is equality of rank and for the four orderings is the order of rank (the declared position of an `ordered enum`). The body SHALL be loop-free, allocation-free and independent of the owner: no call of `compare_enum`, `EnumValue` or `NodeKey`, no `cfg(kani)` and one source for host and Kani builds.

### Proof evidence

9. A Kani harness SHALL prove `compare_native_enum` over symbolic `u32` ranks and counts constrained by Behavior 1, a symbolic ordered flag and either one shared declaration tag or two symbolic tags, for all six operators and for a `Meter` with symbolic sufficient and insufficient limits, against an expected side written separately (the native Rust operators over the ranks, the cause per Behavior 6, the charge sequence of Behavior 7). Each harness SHALL carry a cover that the Completed path and each refusal path are reached, SHALL fail on a mutant of the comparison and SHALL run under the 10-minute wall clock and 16 GiB ceiling of CG FR-028 and return a terminal verdict. No Kani verdict is recorded by this requirement until the code lead records one.
### Differential

10. A host differential, run where both the owner and the kernel are visible, SHALL run `compare_native_enum` beside the owner's `compare_enum` for an ordered and an unordered declaration of one, two, three and sixteen members, every operand pair, every operator, operands of two distinct declarations, and a `Meter` at a sufficient, an exact and a one-less limit for each limit kind the charges use. It SHALL compare the typed outcome, the refusal cause, every charge point with its limit kinds and amounts, the denied charge and the counters. The differential SHALL fail on buildable mutants of Behavior 6, 7 and 8.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-407-AC-1 | `NativeEnum` is `Copy`, has no drop glue, holds no `NodeKey`, `VariantId`, `Arc`, `String`, `Box` or `Vec`, and has no public field; construction, comparison and the round-trip conversion perform zero allocations under a counting allocator. | Test (TC-955) |
| FR-407-AC-2 | `from_index` refuses `index == member_count`, `index == member_count + 1`, `u32::MAX` against a smaller count and any index with a count of zero, each with `IndexOutOfRange` and no carrier, takes no `Meter`, never clamps or wraps, and admits `0` and `member_count - 1`. | Test (TC-955) |
| FR-407-AC-3 | `from_member` agrees with `from_index` for every member of an ordered and an unordered `EnumShape`, refuses a member whose rank differs from the shape's rank for its variant and a foreign variant with `MemberNotInShape`, and takes the count and ordered flag from the shape. | Test (TC-955) |
| FR-407-AC-4 | For the six operators over every ordered pair of an `ordered enum` of three members, the Boolean equals the native operator over the indices; for an unordered enum `Equal` and `NotEqual` equal the same and the four orderings are refused. | Test (TC-955) |
| FR-407-AC-5 | Operands with different tags, or the same tag but a different count or ordered flag, are refused with `DistinctEnumDeclarations`, which takes precedence over `UnorderedEnumOrdering`; an ordering over an unordered declaration is refused with `UnorderedEnumOrdering`; a refused call leaves the `Meter` unchanged. | Test (TC-955) |
| FR-407-AC-6 | The charge points, order, limit kinds and amounts of Behavior 7, the denied charge and the incomplete record equal the owner's at a sufficient, an exact and a one-less limit, and equality and ordered comparison are charged identically on one `Meter`. | Test (TC-955) |
| FR-407-AC-7 | The host differential of Behavior 10 passes beside the owner's `compare_enum`, and each mutant of TC-955 fails it at the step TC-955 names. | Test (TC-955) |
| FR-407-AC-8 | The comparison source contains no `cfg(kani)`, loop, iterator adapter, allocation, `num_bigint` item, owner `Integer`, `NodeKey` or `EnumValue`, and no call of `compare_enum`. | Inspection (TC-955) |
| FR-407-AC-9 | PLANNED: the Kani harness of Behavior 9 returns an executed terminal VERIFIED verdict, its cover paths are reached and its mutant fails, with time, peak memory, tool version and harness id recorded in TC-955's evidence record. A FAILED verdict fails the criterion; a timeout, memory exhaustion or unreliable outcome is recorded as the measured row and claims no proof. | Test (TC-955) |

## Relationship to CG FR-049

CG FR-049 draws each operand as a symbolic `i64` index in `0..member_count` and says the harness builds each operand "through the owner's checked member identity" (FR-049 Behavior, AC-3). With this carrier CG consumes `NativeEnum::from_index` (or `from_member`) for the harness operands and `compare_native_enum` as the native expression of the oracle, and the assertion of FR-049-AC-4 is the equality of the oracle's outcome with the carrier's. CG owns the mapping from the admitted declaration node to the declaration tag, the symbolic index encoding, the refusal dispositions and the incomplete-member-list `unsupported` item (the kernel has no zero or partial member list to refuse beyond `IndexOutOfRange`). This requirement edits no CG specification. Whether FR-049-AC-3's minted-identity mutant is still detectable when operands are rank-only is a question for CG, listed in the pull request.

## Status

Specified (IR-262). Not implemented; no Rust accompanies this requirement, no Kani harness has run and no measurement exists.

## Dependencies

- quire-semantic-value `compare_enum` and `EnumValue` own enum comparison and charges and are the oracle of the differential; FR-088 owns rank, `EnumShape` and `EnumMember`.
- FR-402 and FR-404 are the precedent for the carrier and evidence structure. CG FR-049 is the consumer and CG FR-028 owns the 16 GiB ceiling.
