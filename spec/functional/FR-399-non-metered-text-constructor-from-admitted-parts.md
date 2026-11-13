---
id: FR-399
title: "Construct an owner Text from already-admitted parts without a Meter"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-404
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-367
    type: references
  - target: ix://agent-ix/quire-exact/FR-400
    type: references
  - target: ix://agent-ix/quire-exact/FR-401
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: references
---
# FR-399: Construct an owner Text from already-admitted parts without a Meter

## Description

When a consumer requests a `Text` from a `TextPayload`, the retained sequence of that payload under a `TextType` and that type, the kernel SHALL return a `Text` from those parts without a `Meter`, a charge or a clone. The constructor refuses parts that `admit_text` would not have produced. The only public way to obtain a `Text` today is the metered `admit_text`, so a consumer that builds an operand as setup (a Kani harness drawing a symbolic operand, ticket IR-1499) pays an admission per operand. This requirement is prerequisite P2 of the Contract Codegen text row (CG FR-052). It is not a way to skip validation: the constructor decides what `admit_text` decides about the retained sequence and the declared bounds, and it spends nothing.

The representation of `Text` on the Kani proof path is the text carrier of [FR-404](./FR-404-kani-tractable-native-text-carrier.md), which owns its capacity C, its refusal of an oversize payload and its memory evidence. This requirement states the behaviour that every representation has, and the proof-path counterpart (FR-399-AC-6) is conditional on the carrier.

## Inputs

- A `TextType` (`min`, `max`, `profile`).
- A `TextPayload`, owned, with its provenance.
- The retained sequence, owned (`Box<str>`): the sequence `admit_text` would retain for that payload under that profile.

## Outputs

`Text::from_admitted(text_type: TextType, payload: TextPayload, retained: Box<str>) -> Result<Text, TextNotAdmitted>`.

`TextNotAdmitted` is a struct with a public field `cause: TextNotAdmittedCause`, a closed enum with the variants `TextNotAdmittedCause::RetainedNotNormalForm` and `TextNotAdmittedCause::LengthOutOfDomain`. It is a refusal beside `Outcome`, not an evaluator outcome, because nothing is metered.

## Behavior

1. **No meter.** The constructor takes no `Meter` and no `Reservation`, charges no point and moves its three arguments into the `Text` without copying them. The work it does is linear in the lengths of values the caller already holds and is unmetered by design; it materializes no sequence of its own.
2. **Same decision as admission.** The constructor refuses with `RetainedNotNormalForm` when `retained` is not the retained sequence of `payload` under the profile: for `unicode-scalars` and `binary-utf8` the payload itself, and for `nfc`, `nfd`, `nfkc` and `nfkd` exactly the scalars `NormalizationForm::apply` emits for the payload, compared scalar by scalar in order and by length, without building the normalized `String`. It then refuses with `LengthOutOfDomain` when `TextType::admits(retained)` is false, so the length is the profile length of the retained sequence and not of the payload. The first refusing check decides the cause.
3. **Equal to an admitted value.** When it returns `Ok`, the `Text` has the given type, payload and retained sequence, and `length()`, `retained()`, `payload()` and `text_type()` return what the same accessors return for the `Text` that `admit_text` produces from the same payload and type. `compare_text` over constructor-built operands returns the same typed outcome, ordered charges, denied next charge and consumed counters as over admitted operands, at every budget, because `compare_text` prepares from the payloads and charges its own points.
4. **Carrier-safe on the proof path.** The proof-path counterpart takes the carrier's fixed-capacity forms in place of `Box<str>`, allocates nothing, owns no drop glue, uses no `chars()` count, SWAR step or iterator adapter, and decides with indexed byte loops bounded by the carrier capacity C, in the sense of IR-1436 design D4. Its normalization is [FR-401](./FR-401-loop-free-proof-path-text-normalization.md)'s. Its signature and representation are those of [FR-404](./FR-404-kani-tractable-native-text-carrier.md); its accept or refuse decision and its cause are this requirement's for every input within C.
5. **Host-callable replacement.** The proof-path counterpart is callable from a host build, so that the host differential of AC-6 can run. This is a prerequisite on [FR-404](./FR-404-kani-tractable-native-text-carrier.md), which states it (its Behavior 7 and FR-404-AC-4). The carrier's refusal of a payload of C + 1 bytes (AC-6) is likewise stated by [FR-404](./FR-404-kani-tractable-native-text-carrier.md) (its Behavior 4 and FR-404-AC-3).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-399-AC-1 | The constructor is a safe public associated function whose type is exactly `fn(TextType, TextPayload, Box<str>) -> Result<Text, TextNotAdmitted>`, bound in a test by coercing `Text::from_admitted` to that function-pointer type (no `Meter` parameter). For each of the six profiles and the payloads `""`, `"a"`, `"e\u{301}"`, `"\u{212B}"`, `"\u{FB03}"` and `"🦀"`, with `retained` the sequence `NormalizationForm::apply` emits (the payload itself for the two non-normalizing profiles) and the type `Text[L,L]` for the profile length L of that sequence, `from_admitted` returns `Ok`, and its `text_type()`, `payload()`, `retained()` and `length()` equal those of `admit_text` over the same payload and type under a generous meter. | Test (TC-947) |
| FR-399-AC-2 | `compare_text` for all six operators over a constructor-built pair and over an admitted pair of the same payloads, for `nfc` over `"e\u{301}"` against `"\u{E9}"` and for `binary-utf8` over `"a"` against `"ab"`, returns the same typed outcome, ordered charges, denied next charge and consumed counters at zero, exact and one-short budgets for each point the profile reaches. | Test (TC-947) |
| FR-399-AC-3 | Retained mismatch: `nfc` with payload `"e\u{301}"` and retained `"e\u{301}"` (payload not normalized), `nfd` with payload `"\u{E9}"` and retained `"\u{E9}"`, `unicode-scalars` with payload `"ab"` and retained `"a"`, a retained sequence that equals the normalization except in its last scalar, one that is a proper prefix of it and one that adds a scalar at its end each return `Err(TextNotAdmitted { cause: RetainedNotNormalForm })`, including when the retained sequence would be admitted by the type. | Test (TC-947) |
| FR-399-AC-4 | Length is measured on the retained sequence by the profile measure: `nfc` payload `"e\u{301}"` with retained `"\u{E9}"` is admitted by `Text[1,1]` (payload has two scalars, retained one); `unicode-scalars` payload `"ab"` is refused by `Text[0,1]` with `LengthOutOfDomain`; `binary-utf8` payload `"\u{E9}"` (two bytes, one scalar) is refused by `Text[0,1]` and admitted by `Text[0,2]`, while `unicode-scalars` admits it under `Text[0,1]`; `Text[2,2]` refuses a one-scalar sequence and `Text[0,1]` refuses a two-scalar one, at the minimum and the maximum edges. When both defects are present, the cause is `RetainedNotNormalForm`. | Test (TC-947) |
| FR-399-AC-5 | A counting global allocator observes zero allocations between entry and return of `from_admitted`, over payloads and retained sequences built before the measurement, for these cases only: the success cases of AC-1 for `unicode-scalars`, `binary-utf8` and `nfc` with `"e\u{301}"`; the AC-3 refusals `unicode-scalars` `"ab"` against `"a"` and `nfc` `"e\u{301}"` against `"e\u{301}"`; and the AC-4 refusals `unicode-scalars` `"ab"` under `Text[0,1]` and `binary-utf8` `"\u{E9}"` under `Text[0,1]`. (At most one combining mark per starter, so the normalizing crate's inline buffer is not exceeded; `nfd` and other `nfc` refusals are not claimed allocation-free.) The counter is per thread (a `thread_local!` `Cell` initialized by a `const` block), so allocations by concurrently running tests do not enter the window; a process-wide counter is not acceptable. | Test (TC-947) |
| FR-399-AC-6 | When the carrier of [FR-404](./FR-404-kani-tractable-native-text-carrier.md) exists, its host-callable proof-path counterpart of the constructor and the host `from_admitted` return the same accept or refuse decision and the same cause for every string that the correspondence enumeration of CG FR-052-AC-2 covers (every UTF-8 string up to the stated small length, per profile), each with three retained sequences: the correct one, the correct one with its last scalar replaced by a different scalar, and the empty sequence (the missing normalization; empty is also the correct sequence for the empty payload). It also agrees for the boundary family of CG FR-052-AC-12 as amended by FR-400: for each supported profile, payloads of byte length A - 1 and A, where A is min(A(profile, max), C), A(profile, max) being the attained payload length of FR-400-AC-6 at the operand's declared `max` (the payload `nfd(c)` repeated `max` times, for any `max`, 12 bytes per retained scalar on the Unicode 17.0.0 data for `nfc` and `nfkc`), whose last scalar is a one-byte scalar or a multibyte scalar's continuation byte, each with the three retained sequences. A payload of C + 1 bytes returns the carrier's encoding refusal, which AC-6 reads and does not compare. The check ties the counterpart to the host constructor for the enumerated and boundary inputs only. | Test (TC-947) |
| FR-399-AC-7 | When the carrier of [FR-404](./FR-404-kani-tractable-native-text-carrier.md) exists, the proof-path counterpart of the constructor is checked by the methods of FR-401-AC-3 and FR-401-AC-4: zero allocations under a per-thread counting allocator over the AC-6 inputs, and a comment-stripped source scan of its module for the banned constructs and the loop-header form of FR-401-AC-4. | Test (TC-947) |

## Status

Planned (IR-1499). No Rust, test or proof exists. AC-6 and AC-7 are also conditional on [FR-404](./FR-404-kani-tractable-native-text-carrier.md).

## Dependencies

- [FR-367](./FR-367-text-profile-mismatch-before-charging.md) owns `compare_text` and the distinct-profile refusal this constructor leaves unchanged.
- [FR-400](./FR-400-text-payload-byte-bound-per-profile.md) states the payload-byte bound that decides whether a payload fits the carrier.
- [FR-401](./FR-401-loop-free-proof-path-text-normalization.md) is the proof-path normalization the counterpart uses.
- [FR-404](./FR-404-kani-tractable-native-text-carrier.md) (ticket IR-1445, Kani-tractable text carrier) owns the carrier, C and the memory evidence; IR-1500 and IR-1501 are the sibling prerequisites.
- [TC-947](../test-cases/TC-947-non-metered-text-constructor-from-admitted-parts.md) is the verification home.
