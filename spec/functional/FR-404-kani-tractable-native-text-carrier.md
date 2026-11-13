---
id: FR-404
title: "A Kani-tractable native Text carrier with fixed capacity and a host-callable compare_text replacement"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-399
    type: references
  - target: ix://agent-ix/quire-exact/FR-400
    type: references
  - target: ix://agent-ix/quire-exact/FR-401
    type: references
  - target: ix://agent-ix/quire-exact/FR-367
    type: references
  - target: ix://agent-ix/quire-exact/FR-402
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---
# FR-404: A Kani-tractable native Text carrier with fixed capacity and a host-callable compare_text replacement

## Description

The kernel SHALL provide a native Text carrier, a fixed-capacity value that holds a payload of at most C bytes, with no allocation and no drop glue, and a proof-path replacement of `compare_text` over two such carriers. The carrier is the representation on which [FR-399](./FR-399-non-metered-text-constructor-from-admitted-parts.md)'s proof-path constructor and [FR-401](./FR-401-loop-free-proof-path-text-normalization.md)'s normalization operate, and the operand of the Contract CodeGen text row (CG FR-052). No requirement owns the carrier itself today: FR-399 and FR-401 each defer its capacity C, its refusal of an oversize payload and its memory evidence to it. This requirement owns them, and states none of the behavior of FR-399, FR-400 or FR-401 again.

The consumer draws an operand symbolically, constructs the carrier through FR-399's proof-path counterpart, and compares it with the replacement of `compare_text`. A Kani proof over `std` `chars()` or UTF-8 validation is not tractable (measured), so the carrier decides with indexed byte loops bounded by C (IR-1436 design D4). The measured scope of that choice is recorded in the repository's evidence table (indexed byte-loop equality up to 128 bytes verifies in seconds; `std` UTF-8 validation does not return at 32 bytes). The table is evidence for the choice and is not an acceptance number: the capacity is fixed by the probe of Behavior 2.

**Out of scope.** The accept or refuse decision of the constructor (FR-399), the payload-byte bound per profile (FR-400), the normalization (FR-401) and the shipped `compare_text` (FR-367) are not restated. Choosing a smaller domain than the owner's so that a proof fits is not done here: where the carrier cannot serve a profile or a payload, it refuses, and the consumer reports the case unsupported.

## Behavior

### Capacity

1. The carrier SHALL have a single capacity C in bytes, a named constant of the carrier module, with C a candidate of 32 or 64. The carrier SHALL hold the payload bytes and their length, SHALL be `Copy`, SHALL have no drop glue and SHALL own no heap type.
2. C SHALL be fixed by a probe recorded as acceptance evidence: for each candidate, the proof harness of Behavior 6 is run under Kani, and the recorded peak memory, wall time, tool version and terminal outcome decide C. C is the largest candidate whose harness returns VERIFIED within the K-DR budget. The probe is bounded to ten minutes of wall time per candidate. A candidate that does not return is recorded as such and is not chosen. No evidence number is hard-coded in this requirement.

### Construction and refusal

3. A payload of C + 1 bytes or more SHALL be refused by the carrier with an encoding refusal that carries no carrier, and a payload of C bytes or fewer SHALL be held without change. The refusal SHALL be reachable from the proof-path constructor of FR-399 and SHALL be the refusal that FR-399-AC-6 reads and does not compare.

### Replacement of compare_text

4. The carrier SHALL provide a proof-path replacement of `compare_text` over two carriers. It SHALL be an implementation separate from the shipped `compare_text` and from every expected-side computation of the host differential (no shared helper that decides equality or order), SHALL use indexed byte loops bounded by C, and SHALL NOT use `chars()`, an iterator adapter, a SWAR step, recursion or a data-dependent loop bound.
5. The replacement SHALL be callable from a host build and SHALL NOT be selected by `cfg(kani)` in its body, so that the host differential runs the code Kani proves. The differential SHALL cover payload lengths C - 1, C and C + 1 (the last returns the refusal of Behavior 3 and is not compared), as CG FR-052-AC-12 requires, and SHALL compare the replacement with the shipped `compare_text` on typed outcome, for every operator.

### Kani evidence

6. The CG text row SHALL consume the carrier as its operand: the row's harness draws each operand's payload as the carrier, constructs it through FR-399's proof-path counterpart, and compares with the replacement of Behavior 4. The harness SHALL return VERIFIED under Kani, and its peak memory, time, tool version and terminal outcome SHALL be recorded with their provenance. The peak memory SHALL be within the K-DR budget and under the 16 GiB per-request ceiling of CG FR-028; a ceiling or a prediction is not a measurement.

### Normalizing profiles

7. Whether the constant normalization tables of FR-401 fit the K-DR budget when instantiated at the carrier's C SHALL be measured by the same method as Behavior 6 and recorded. If they fit, the normalizing profiles are supported. If they do not fit, the carrier SHALL support only the non-normalizing profiles, the consumer SHALL leave `nfc`, `nfd`, `nfkc` and `nfkd` unsupported with the measurement as the stated reason, and the type domain SHALL NOT be narrowed to make them fit.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-404-AC-1 | The carrier is `Copy`, has no drop glue and no field of an owner or heap type (a generic `Copy` bound and a `!needs_drop` assertion compile), and constructing it from a payload of C bytes performs zero heap allocations under a per-thread counting allocator. | Test (TC-952) |
| FR-404-AC-2 | C is a named constant equal to 32 or 64, and the probe record of Behavior 2 exists for each candidate: peak memory, wall time, tool version, terminal outcome and the harness run. The chosen C is the largest candidate recorded VERIFIED within the K-DR budget; a record that shows the larger candidate VERIFIED within budget while C is the smaller fails the criterion. | Inspection (TC-952) |
| FR-404-AC-3 | Payloads of C - 1 and C bytes are held and read back byte for byte, and a payload of C + 1 bytes is refused with the encoding refusal and no carrier, for payloads whose last byte is one-byte and whose last scalar is a multibyte scalar's continuation byte. | Test (TC-952) |
| FR-404-AC-4 | The replacement of `compare_text` is host-callable, and over every operator and the payload pairs at lengths C - 1 and C it returns the typed outcome of the shipped `compare_text`, including pairs equal except in the last byte and pairs that are a proper prefix of one another. A C + 1 payload returns the refusal and is not compared. | Test (TC-952) |
| FR-404-AC-5 | A comment-stripped source scan of the replacement and of the carrier module finds no `cfg(kani)` token, no `chars()`, no iterator adapter, no recursion, no `while` or `loop` and only `for <index> in 0..<NAME>` loops with `NAME` a named constant, and finds no call of the shipped `compare_text` or of a helper the host differential uses as its expected side. | Test (TC-952) |
| FR-404-AC-6 | The CG text row's harness over the carrier at the chosen C returns VERIFIED under Kani, with the peak memory, wall time, tool version, terminal outcome and harness run recorded, the peak memory within the K-DR budget and under 16 GiB. A TIMEOUT, an out-of-memory or a missing cover is recorded as such and fails the criterion. | Inspection (TC-952) |
| FR-404-AC-7 | The measurement of Behavior 7 is recorded: the FR-401 normalization instantiated at the chosen C, run under Kani, with peak memory, time and outcome. Either the normalizing profiles are supported with that record, or the consumer's profile list excludes them citing it; in the second case no `TextType` bound of the owner is narrowed. | Inspection (TC-952) |
| FR-404-AC-8 | Executed verdict required: this requirement is not Done until AC-2, AC-6 and AC-7 each have an executed run with a recorded outcome. A configured limit, a prediction or an unexecuted harness does not satisfy them. | Inspection (TC-952) |

## Status

Planned (IR-1445). No Rust, test, probe or proof exists. C is not yet fixed. FR-399-AC-6 and FR-399-AC-7 and the carrier-dependent parts of FR-401 are conditional on this requirement; their text already names the carrier of IR-1445 and needs no change.

## Dependencies

- [FR-399](./FR-399-non-metered-text-constructor-from-admitted-parts.md) owns the proof-path constructor decision on this carrier; [FR-400](./FR-400-text-payload-byte-bound-per-profile.md) the payload bound that decides what fits C; [FR-401](./FR-401-loop-free-proof-path-text-normalization.md) the normalization; [FR-367](./FR-367-text-profile-mismatch-before-charging.md) the shipped `compare_text`.
- CG FR-052 is the consumer (the text row, its AC-12 boundary family) and CG FR-028 the 16 GiB ceiling.
- [FR-402](./FR-402-kani-tractable-native-decimal-carrier.md) is the sibling Decimal carrier and the precedent for host availability and for recording a measurement rather than a prediction.
- [TC-952](../test-cases/TC-952-kani-tractable-native-text-carrier.md) is the verification home.
