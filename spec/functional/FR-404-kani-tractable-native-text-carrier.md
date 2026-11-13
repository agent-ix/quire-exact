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

The kernel SHALL provide a native Text carrier, a fixed-capacity value that holds a payload of at most C bytes, with no allocation and no drop glue, and a proof-path replacement of `compare_text` over two such carriers. The carrier is the representation on which [FR-399](./FR-399-non-metered-text-constructor-from-admitted-parts.md)'s proof-path constructor and [FR-401](./FR-401-loop-free-proof-path-text-normalization.md)'s normalization operate, and the operand of the Contract CodeGen text row (CG FR-052). No other requirement owns the carrier itself: FR-399 and FR-401 defer its capacity C, its refusal of an oversize payload and its memory evidence to this one. This requirement owns them, and states none of the behavior of FR-399, FR-400 or FR-401 again.

A Kani proof over `std` `chars()` or UTF-8 validation is not tractable (measured), so the carrier decides with indexed byte loops bounded by C (IR-1436 design D4). The measured scope of that choice is recorded in the Kani tractability table (the rows of the evidence-record section of [TC-952](../test-cases/TC-952-kani-tractable-native-text-carrier.md), copied from a local, unversioned tractability table: indexed byte-loop equality up to 128 bytes verifies in seconds; `std` UTF-8 validation does not return at 32 bytes). The table is evidence for the choice and is not an acceptance number: C is fixed by the probe of Behavior 5.

**Budget.** The only memory budget this requirement uses is the 16 GiB per-request ceiling of CG FR-028, as FR-402-AC-11 does. No other budget is stated or assumed.

**Consumer.** CG's text row (CG FR-052) consumes the carrier; that consumption and CG's credit for it are CG's, as CG's consumption of the Decimal carrier is under FR-402. This requirement is met by the kernel alone, through a harness local to this repository (Behavior 9).

**Out of scope.** The accept or refuse decision of the constructor (FR-399), the payload-byte bound per profile (FR-400), the normalization (FR-401) and the shipped `compare_text` (FR-367) are not restated. Choosing a smaller domain than the owner's so that a proof fits is not done here: where the carrier cannot serve a profile or a payload, it refuses, and the case is reported unsupported through the refusal seam of IR-1526, which this requirement does not duplicate.

## Behavior

### Types and signatures

1. The carrier `NativeText` SHALL have a single capacity C in bytes, a named constant of the carrier module, with C a candidate of 32 or 64. It SHALL hold the owner's `TextProfile`, the payload bytes with their length and the retained-sequence bytes with their length (`usize` lengths, each at most C), SHALL be `Copy`, SHALL have no drop glue and SHALL own no heap type.
2. The carrier SHALL hold every byte at an index at or after its length as zero. Construction SHALL zero-fill, so that two carriers of one text are equal bytewise, and the replacement of Behavior 6 SHALL read no index at or after a length.
3. The proof-path constructor SHALL have the signature `NativeText::from_admitted(text_type: TextType, payload: &[u8], retained: &[u8]) -> Result<NativeText, NativeTextRefusal>`, where `NativeTextRefusal` is a closed enum with the variants `PayloadExceedsCapacity` and `NotAdmitted(TextNotAdmittedCause)`. The accept or refuse decision and the cause `NotAdmitted` carries are FR-399's. Its host-callable form is Behavior 7.

### Capacity and refusal

4. When the constructor is given a payload or a retained sequence longer than C bytes, it SHALL return `Err(NativeTextRefusal::PayloadExceedsCapacity)` and no carrier, before any decision of FR-399, for C + 1 bytes and for every longer length. When both are at most C bytes it SHALL NOT return `PayloadExceedsCapacity`. This is the refusal that FR-399-AC-6 reads and does not compare.
5. The carrier SHALL set C to the largest candidate (64, then 32) whose local probe harness of Behavior 9 returns VERIFIED under Kani within the ten-minute wall bound and the 16 GiB ceiling, with the peak memory, wall time, tool version and terminal outcome recorded for each candidate. If neither candidate returns VERIFIED, then C SHALL be 32, both runs SHALL be recorded in the evidence-record section of TC-952 with their terminal outcome. In that case the text row is unsupported at that capacity, the carrier is still delivered at C = 32 for the host, and the recorded rows are the measured end state; IR-1526's refusal seam consumes them as a consumer, and no second refusal mechanism is built here. A configured limit or a prediction is not a measurement.

### Replacement of compare_text

6. The carrier SHALL provide `compare_native_text(operator: ComparisonOperator, left: &NativeText, right: &NativeText, meter: &mut Meter) -> Result<Outcome<bool>, IllTyped>`, mirroring `compare_text` (`src/text.rs`), with the same ill-typed refusal for distinct profiles and the same charges. It SHALL be an implementation separate from the shipped `compare_text` and from every expected-side computation of the host differential (no shared helper that decides equality or order), SHALL decide with indexed byte loops bounded by C, and SHALL NOT use `chars()`, an iterator adapter, a SWAR step, recursion or a data-dependent loop bound.
7. The replacement and the constructor SHALL be callable from a host build and SHALL NOT have a body selected by `cfg(kani)`, so that the host differential runs the code Kani proves. For context, the Kani build substitutes the replacement for `compare_text` at the consumer: CG's generated harness calls `compare_native_text` in place of `compare_text` and builds its operands with the constructor, a selection made by CG's generation and not by a conditional body in this repository; this requirement places no obligation on CG.
8. The differential SHALL compare the replacement with the shipped `compare_text` on the typed outcome, the ordered charges, the denied next charge and the consumed counters, at a zero budget, an exact budget and a one-short budget for each point the profile reaches, for all six operators, as CG FR-052-AC-12 and FR-401-Behavior 3 require. Its payload lengths SHALL be the per-profile boundary family of CG FR-052-AC-12 and AC-14: for each supported profile, L - 1 and L with L = min(A, C), and L_b, where A and L_b are as those criteria define them. When no payload of a length exists that the owner admits under the profile (as for `nfc` and `nfkc` at L - 1), the case SHALL be listed with that reason and replaced by the nearest admitted length, never silently dropped. A payload of C + 1 bytes returns the refusal of Behavior 4 and is not compared.

### Kani evidence

9. The kernel SHALL carry a local probe harness under Kani for the replacement: it draws two payloads of symbolic bytes and symbolic lengths up to C, with every byte at or after the length assumed zero, builds both carriers with the zero-padding step of Behavior 2 (no UTF-8 decision, which is FR-399's), uses the `binary-utf8` profile, and asserts, for each of the six operators, that the returned boolean equals the comparison of the two byte slices by the language's slice `==` and `cmp`, and that the call neither panics nor returns an ill-typed refusal. It SHALL pass a meter whose budget affords every point that operands of C bytes can reach (so that no call is denied and every call returns the value variant of the outcome). It SHALL return VERIFIED, with its peak memory recorded.
10. If the recorded run of the FR-401 normalization instantiated at the chosen C is not VERIFIED within the 16 GiB ceiling, then the carrier SHALL support only the non-normalizing profiles, the normalizing profiles `nfc`, `nfd`, `nfkc` and `nfkd` SHALL be reported unsupported with that record as the stated reason (through the seam of Behavior 5), and the type domain SHALL NOT be narrowed. The normalizing profiles SHALL be reported supported only when that record shows VERIFIED.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-404-AC-1 | The carrier is `Copy`, has no drop glue and no field of an owner or heap type other than `TextProfile` (a generic `Copy` bound and a `!needs_drop` assertion compile), and constructing it from a payload of C bytes performs zero heap allocations under a per-thread counting allocator. The constructor and the replacement coerce to function pointers of exactly the signatures of Behaviors 3 and 6. | Test (TC-952) |
| FR-404-AC-2 | Every carrier built over the AC-3 and AC-4 payloads has all bytes at or after its length equal to zero, and two carriers built from the same text are equal bytewise. | Test (TC-952) |
| FR-404-AC-3 | Payloads of C - 1 and C bytes are held and read back byte for byte. A payload of C + 1 bytes and one of 2C bytes each return `Err(PayloadExceedsCapacity)` and no carrier, with the last byte a one-byte scalar and with the last byte a continuation byte of a multibyte scalar. A retained sequence of C + 1 bytes under a payload within C returns `PayloadExceedsCapacity`, and a payload and retained sequence of at most C bytes never does. | Test (TC-952) |
| FR-404-AC-4 | `compare_native_text` is host-callable and, for all six operators, agrees with the shipped `compare_text` on the typed outcome, the ordered charges, the denied next charge and the consumed counters at zero, exact and one-short budgets for each point reached, over the per-profile payload family of Behavior 8, including pairs equal except in the last byte, proper prefixes of one another, the pair `x` and `x` followed by one U+0000 byte (a proper prefix whose extra byte is zero, which fails a replacement that reads past a length or ignores it) and operands of distinct profiles (the ill-typed refusal, nothing consumed). Each skipped length is listed with its reason. | Test (TC-952) |
| FR-404-AC-5 | A comment-stripped source scan of the carrier module and the replacement, using the method of FR-401-AC-4, finds no `cfg(kani)` token, only callees on a stated allow-list, no cycle in the call graph (the recursion ban), no `while` or `loop`, only `for <index> in 0..<NAME>` loops with `NAME` a named constant, no iterator adapter (any call of `iter`, `bytes`, `chars`, `map`, `zip`, `enumerate`, `fold`, `all`, `any`, `position`, `windows`, `chunks`, `chunks_exact` or `cmp` on an iterator, or a `for` over a range not of the form above), no SWAR step (a word-sized load from bytes such as `from_le_bytes`, `read_unaligned` or a `u64` cast of a slice, or a multiply-and-mask over several bytes), and no call of the shipped `compare_text` or of a helper the host differential uses as its expected side. The scanner rejects a seeded violating copy for each banned construct. | Test (TC-952) |
| FR-404-AC-6 | For a VERIFIED candidate (the chosen C is then that candidate), the local probe harness of Behavior 9 returns VERIFIED under Kani at the chosen C, with peak memory, wall time, tool version, terminal outcome and harness run recorded and the peak memory under 16 GiB. A TIMEOUT, an out-of-memory or a missing cover on the chosen candidate is recorded as such and fails this criterion. When neither candidate is VERIFIED, this criterion does not apply, the text row is unsupported at C = 32 as Behavior 5 states, and AC-8 and AC-9 carry the verdict. | Inspection (TC-952) |
| FR-404-AC-7 | The measurement of Behavior 10 is recorded, with the FR-401 normalization instantiated at the chosen C and run under Kani. The normalizing profiles are reported supported only if the record shows VERIFIED within 16 GiB; a TIMEOUT, an out-of-memory or a missing record with the profiles marked supported fails the criterion. Otherwise they are excluded citing the record, and no owner `TextType` bound is narrowed. | Inspection (TC-952) |
| FR-404-AC-8 | C is a named constant equal to 32 or 64 and the probe records of Behavior 5 exist for each candidate in the evidence-record section of TC-952. The chosen C is the largest candidate recorded VERIFIED within 16 GiB; a record showing 64 VERIFIED while C is 32 fails the criterion. If neither is VERIFIED, C is 32 and both rows are present in that section with their terminal outcomes. | Inspection (TC-952) |
| FR-404-AC-9 | Executed verdict required: this requirement is Done only with AC-6, AC-7 and AC-8 each citing an executed run with a recorded outcome, or, if neither candidate returned VERIFIED, with the measured rows of Behavior 5 recorded in the evidence-record section of TC-952 (AC-6 then does not apply). A configured limit, a prediction or an unexecuted harness does not satisfy it. | Inspection (TC-952) |

## Status

Implemented except AC-7 and AC-9 (open, owner pending planner ruling): C = 64, the largest candidate VERIFIED by the local probe (TC-952 evidence record). Not Done: no Behavior 10 record exists because the FR-401 normalization is not in the kernel, so the four normalizing profiles are unsupported on the proof path meanwhile. FR-399-AC-6 and FR-399-AC-7 are active now that the carrier exists and are untagged (owner pending planner ruling). The constructor takes UTF-8 parts and refuses anything else with `RetainedNotNormalForm`. The host-only constructor decisions in `src/native_text/normal_form.rs` are outside the AC-5 clean scan and covered by an exact listed-violations assertion (TC-952). FR-399 (Description, Behaviors 4 and 5, AC-6, AC-7) and FR-401 (Description, Relation, Dependencies) now point to this requirement as the owner of the carrier and carry a `depends_on` edge to it; their criteria are unchanged. The substitution wording of CG FR-052 is CG's to align.

## Dependencies

- [FR-399](./FR-399-non-metered-text-constructor-from-admitted-parts.md) owns the constructor decision on this carrier; [FR-400](./FR-400-text-payload-byte-bound-per-profile.md) the payload bound that decides what fits C; [FR-401](./FR-401-loop-free-proof-path-text-normalization.md) the normalization; [FR-367](./FR-367-text-profile-mismatch-before-charging.md) the shipped `compare_text`.
- CG FR-052 is the consumer (the text row, its AC-12 and AC-14 boundary family) and CG FR-028 the 16 GiB ceiling. Its credit is CG's, not this requirement's acceptance.
- IR-1526 owns the refusal seam that consumes a measured non-fit.
- [FR-402](./FR-402-kani-tractable-native-decimal-carrier.md) is the sibling Decimal carrier and the precedent for host availability, for recording a measurement rather than a prediction and for leaving the consumer's credit to the consumer.
- [TC-952](../test-cases/TC-952-kani-tractable-native-text-carrier.md) is the verification home.
