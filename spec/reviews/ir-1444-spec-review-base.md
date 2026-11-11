---
id: SR-17940486709564847047632
title: "IR-1444 PR #146 FR-402 base spec review"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #146 @ 1343d2aa on origin/main 9403a5c; IR-1444; spec/functional/FR-402-kani-tractable-native-decimal-carrier.md, spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-950
    type: reviews
---

## Summary

Ticket: IR-1444. Spec-only PR #146 (head 1343d2aa, branch spec/ir-1444-decimal-carrier, on origin/main 9403a5c) adding FR-402 and TC-950, a Kani-tractable native Decimal carrier built from the owner's Decimal bindings. Method: quoin spec-review/base. Reviewed against quire-exact origin/main 9403a5c src/decimal.rs (DecimalRepresentation::normalized, Decimal::new, DecimalType::new, DecimalType::contains, compare_shifted, shifted_digits), src/decimal_membership.rs, src/native_integer.rs, src/integer.rs (From<i64>/From<i128>), Cargo.toml (no profile.dev override); merged FR-398/TC-946 and spec/reviews/ir-1443-*; quire-contract-codegen origin/main spec/kani/functional FR-051, FR-025 (AC-15), FR-028, spec/oracle/functional/FR-014; quire-specification FR-140; Linear IR-1444, IR-1508. Read-only: no cargo, no Kani, no edits to the author's worktree.

## Verdict

CHANGES REQUIRED (see the sibling SR files for the remaining medium findings; this file holds the base-review ones). Verified clean: FR-402 and TC-950 are free on origin/main and in the other open PR (#141); `quire validate` on both files exits 0; `quire matrix` lists FR-402-AC-1..11 (untagged, as a planned spec should be); every relationship target resolves (quire-exact FR-372, FR-384, FR-398; QSpec FR-140; CG FR-051, FR-025 incl. AC-15, FR-028; FR-014 cited in prose); no hash, pin or version string; no memory budget or fit claim in normative text (B10 and AC-11 record measurement only). Owner semantics checked against source: Decimal::new forms no power of ten (normalized() is a trailing-zero loop bounded by the coefficient's digits); DecimalType::new refuses lo > hi, smin > smax and smax > u32::MAX with MalformedDecimalType, matching B1/AC-1; the owner route runs for every u32 scale, because shifted_digits saturates and compare_shifted only materializes a power when the shifted digit count equals the bound's, i.e. at most about 10^39 for i128 bounds. The total constructor mirrors NativeInteger::new (const, total over i128, grants no capability). All AC numeric literals were recomputed (10^38 < i128::MAX < 2*10^38, 10^128 mod 2^128 = 0, 10^18/10^5 = 10^13, 9*10^18 < 2^63).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Behavior 8 justifies the owner route by 'contains never materializes the lifted coefficient and Decimal::new forms no power of ten', but DecimalType::contains calls compare_shifted (src/decimal.rs:439-440), which on equal digit counts computes value.abs().mul(&Integer::power_of_ten(shift)) (src/decimal.rs:464): the lifted magnitude is materialized whenever it has the bound's digit count. The conclusion (the route runs for every u32 scale) still holds because that power is bounded by the bound's digits, so state that reason instead. TC-950 step 2's 'run the owner contains on (1, 4294967295) in Decimal[0, 1; 0, 4294967295] to confirm the owner route needs no power of ten there' is wrong on that very input (digits 1 = 1, so it calls power_of_ten(0)) and names no observation that could fail; drop it or make it an observable check. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:55 |
| FND-002 | low | Merged FR-398 Dependencies still says 'IR-1444 and this constructor share the Decimal-to-fraction mapping and must not diverge' (and its out-of-scope paragraph 'IR-1444 may reuse that mapping'), while FR-402 retains the pair, never forms c/10^s and says the two share only the membership rule. Edit FR-398's two sentences in this PR so the sibling specs agree (half-applied wording, the IR-1443 class). | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md (Dependencies) |
| FND-003 | low | Behavior 10 quotes ticket prose in a normative item ('the label "K-DR budget" in the text of IR-1444 names no figure in this repository'). Ticket text is mutable and not a spec artifact, and quire-exact does contain the label (FR-401 line 22). The sentence 'This requirement states no memory budget and claims no fit' already carries the obligation; drop the ticket-label clause. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:57 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-402 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:26 | The kernel SHALL provide a native Decimal carrier, a native Decimal type and a constructor that builds a carried value of that type from the two `i64` physical bindings of a Decimal operand, a coefficient then a scale (Behavior 1 to 7). The constructor SHALL use only fixed-width native arithmetic... |
| FR-402-OOS | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:32 | Four things are out of scope. Operations over the carrier (arithmetic, rounding and ordering, with their charges) are not specified here, and CG FR-051 cannot be credited with a native expression until they exist. Decoding retained counterexample playback into bindings is CG's (CG FR-051, FR-025-... |
| FR-402-B1 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:42 | 1. The native Decimal type SHALL hold two signed 128-bit inclusive coefficient bounds, a minimum and a maximum membership scale (`u32`) and the owner's `RoundingMode`, and SHALL be `Copy` with no drop glue. Its checked constructor SHALL take `(lower: i128, upper: i128, min_scale: u64, max_scale: ... |
| FR-402-B2 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:43 | 2. The native Decimal type SHALL be convertible from an owner `DecimalType` by an ingress function. If a bound of the owner type lies outside signed 128 bits, then the ingress SHALL return `TypeNotRepresentable` and no native type. The owner `DecimalType` is well formed, so the ingress has no oth... |
| FR-402-B3 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:47 | 3. The carried Decimal SHALL hold the retained representation only: a signed 128-bit coefficient and a `u32` scale. It SHALL be `Copy`, SHALL have no drop glue and SHALL carry no `DecimalType`, `Integer`, `Decimal` or cached normalized form. Two carried Decimals that are numerically equal with di... |
| FR-402-B7 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:51 | 7. The carried Decimal SHALL provide a normalized view returning the pair `(c', s')` of Behavior 5 as a signed 128-bit coefficient and a `u32` scale, with no loop, no allocation and no `Integer`, and SHALL provide a read-out that yields the owner `Decimal` whose `representation()` equals `Decimal... |
| FR-402-B8 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:55 | 8. The kernel SHALL carry a host differential. For every boundary binding of TC-950 whose scale is in `0..=4294967295`, the oracle SHALL be the shipped owner route: `Decimal::new(Integer::from(coefficient), scale)` and `DecimalType::contains` on the owner type, with the owner's `normalized()` for... |
| FR-402-B10 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:57 | 10. When CG's decimal row (CG FR-051, ticket IR-259) consumes the delivered carrier, the memory and time of the proof harness that draws its operands through the carrier under the 16 GiB per-request ceiling of CG FR-028 SHALL be measured and recorded under IR-1444 with the peak provenance and the... |
| FR-402-DEPS | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:81 | - FR-372 and QSpec FR-140 own Decimal membership, on which the constructor depends. FR-384 is the native-width and host-availability precedent. FR-398 (IR-1443) is the sibling constructor and the precedent for the boundary vocabulary, the refusal names and the differential. |
| TC-950 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:14 | Verify FR-402-AC-1 through FR-402-AC-11 through the public kernel API, the host differential, the Kani proofs and the recorded measurement. Every mutant below is a real buildable variant of the shared body in a scratch copy. Each names the input, or the inspection, that fails it; a mutant that no... |
| TC-950-S2 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:31 | 2. For AC-6 and AC-8, build every boundary binding with scale in `0..=4294967295` a second time by the owner route: `Decimal::new(Integer::from(coefficient), scale)` and `DecimalType::contains` on an owner `DecimalType` built from the same bounds, and the owner's `normalized()`. Compare admission... |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified in b5c7fa1e: The owner route runs for every such scale: `Decimal::new` forms no power of ten, and `DecimalType::contains` compares digit counts first (the shifted digit count saturates) and materializes a power of ten only when the lifted coefficient has the bound's digit count, so the power is bounded by the di |
| FND-002 | fixed | Verified in b5c7fa1e: IR-1444 (FR-402) retains the Decimal pair rather than the fraction; it and this constructor share the owner Decimal membership rule and must not diverge in it. |
| FND-003 | fixed | Verified in b5c7fa1e: This requirement states no memory budget and claims no fit: a configured ceiling or a prediction is not a measurement. |
