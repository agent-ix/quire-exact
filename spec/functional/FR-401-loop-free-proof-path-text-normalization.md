---
id: FR-401
title: "Normalize text on the Kani proof path without allocation or iterator loops"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-399
    type: references
  - target: ix://agent-ix/quire-exact/FR-400
    type: references
  - target: ix://agent-ix/quire-exact/FR-367
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: references
---
# FR-401: Normalize text on the Kani proof path without allocation or iterator loops

## Description

When the Kani text carrier (ticket IR-1445) needs the retained sequence of a payload under `nfc`, `nfd`, `nfkc` or `nfkd`, the kernel SHALL provide a proof-path normalization that allocates nothing, owns no drop glue, uses no iterator adapter, closure or recursion, and emits exactly the scalars, in order and in number, that the shipped `NormalizationForm::apply` emits. Host tests check that agreement. The shipped `apply` streams through `unicode-normalization` iterators and `prepare` collects into `String`s, which the IR-1436 design review (D4) bans from a Kani carrier. Contract Codegen (CG FR-052, prerequisite P4, ticket IR-1501) gates every normalizing-profile criterion on this requirement, and without it those profiles are `unsupported`.

**Reading of "loop-free".** The code lead ruled (ticket IR-1501) that "loop-free" means no iterator adapter, no `chars()`, no allocation, no recursion and no data-dependent loop bound. Indexed loops with a constant bound are permitted, because Kani unwinds them to a stated finite bound. Every loop of the proof-path normalization is such a loop (bound the carrier capacity C or the constant size of a decomposition buffer), and the proof-path module states the unwind bound its loops need (FR-401-AC-6). This is an interpretation of the wording of IR-1501 and CG FR-052, not a narrowing of it.

## Inputs

- A payload in the carrier's fixed-capacity form, a length at most C bytes, and a normalization form.
- An output capacity R in bytes, with R at least `4 * max` for the declared `max` of the operand's `TextType`, which is the byte length of every retained sequence that `TextType` admits ([FR-400](./FR-400-text-payload-byte-bound-per-profile.md)).

## Outputs

The retained scalars in an output buffer of capacity R, their scalar count and byte length, or the refusal `OutputExceedsCapacity`. The function is callable from a host build, as the carrier's other proof-path replacements are (the requirement recorded on IR-1445).

## Behavior

1. **Same data.** The normalization reads the Unicode 17.0.0 data from the one source `NormalizationForm::apply` reads (the pinned `unicode-normalization` tables, or tables derived from that crate's table-generation inputs), so the existing build-time version assertion applies. No second hand-copied table exists. Composition exclusions, the singleton and non-starter decompositions, Hangul algorithmic composition and decomposition, canonical reordering by combining class and the blocked-composition rule are those of the pinned crate.
2. **Same result.** For every payload within C, form and R, the emitted scalars and their count equal the scalars and count of `NormalizationForm::apply` on that payload when the output fits R. The count is the running emitted count of `text.normalize-output`, so the carrier's `compare_text` replacement charges the same points as the shipped `compare_text`.
3. **Capacity.** When the normalized output would exceed R bytes the function returns `OutputExceedsCapacity` and no partial result is used. With R at least `4 * max`, an exceeded output has more than `max` scalars, so the sequence is not admitted by the operand's `TextType`, and the refusal is a correct negative for the draw constraint of CG FR-052.
4. **No hidden cost.** The function charges no `Meter` (metering is the replacement `compare_text`'s) and allocates nothing.
5. **Proof-path module is checkable.** The proof-path normalization sits in its own module so that the absence of allocation and iterator adapters can be inspected.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-401-AC-1 | For each of `nfc`, `nfd`, `nfkc` and `nfkd`, the proof-path normalization equals `NormalizationForm::apply` (scalars, order and count) for every Unicode scalar value alone, for every ordered pair drawn from the scalars U+0061, U+0075, U+0041, U+0300, U+0301, U+0307, U+0308, U+0310, U+0323, U+0915, U+093C, U+1100, U+1161, U+11A8, U+AC00, U+D7A3, U+1F82, U+1F600, U+212B, U+FB03, U+FDFA, and for the sequences `u` U+0308 U+0301 (composes to U+01D8), `a` U+0307 U+0323 (reorders, `nfc` gives U+1EA1 U+0307), `a` U+0310 U+0300 (second mark blocked, unchanged in `nfc`), U+0915 U+093C (excluded composite, unchanged in `nfc`), U+1100 U+1161 U+11A8 (Hangul, composes to U+AC01 in `nfc`) and the empty payload. | Test (TC-949) |
| FR-401-AC-2 | The output-capacity boundary is exact: `nfd` of U+1F82 (8 bytes) succeeds at R of 8 and 9 and returns `OutputExceedsCapacity` at R of 7; `nfkd` of U+FDFA succeeds at its exact byte length and fails one byte short; an empty payload succeeds at R of 0. For every Unicode scalar value alone and `max` from 1 to 4, every payload whose `apply` output `TextType::admits` under `Text[0,max]` normalizes successfully at R of `4 * max`. | Test (TC-949) |
| FR-401-AC-3 | A counting global allocator observes zero allocations in the proof-path normalization over every input of AC-1 for the four forms, with input and output buffers built before the measurement. The proof-path module's source, read by the test, contains none of the tokens `String`, `Vec`, `Box`, `alloc::`, `.chars()`, `.iter()`, `.map(`, `.collect(`, `impl Iterator`, `dyn ` and `Iterator`. | Inspection (TC-949) |
| FR-401-AC-4 | For every ordered pair of AC-1 over the four profiles, the lexicographic order of the two proof-path normalized outputs (by scalar, as bytes of UTF-8) gives, for each of the six operators, the Boolean that the shipped `compare_text` returns over the same two payloads admitted under a type that admits both (`nfc` over `"e\u{301}"` against `"\u{E9}"` is `eq` true), and `binary-utf8` payloads are compared by their bytes. | Test (TC-949) |
| FR-401-AC-5 | The emitted count of the proof-path normalization equals the number of `text.normalize-output` charges that the shipped `compare_text` records for each operand pair of AC-4 under `test-support`, with the second operand's charge amounts continuing the running count of the first. | Test (TC-949) |
| FR-401-AC-6 | The proof-path module exposes a constant `UNWIND_BOUND`, and the test computes from the module's loop bounds (C, the decomposition-buffer size and the output capacity R) the largest trip count of any loop and asserts `UNWIND_BOUND` is greater than it by exactly the unwind margin the harness needs (one). A loop whose bound grows with the data, or a constant lowered by one, fails the equality. | Test (TC-949) |

## Status

Planned (IR-1501). No Rust or test exists.

Open premises, not claimed:

- Kani tractability. That Kani verifies the pinned table lookups and the composition within the carrier's memory budget is not established here. The memory evidence is IR-1445's, and a table form that does not fit makes the normalizing profiles stay `unsupported` in CG FR-052 with this requirement still met on the host.
- The reading of "loop-free" above is the code lead's ruling on IR-1501, recorded here as an interpretation of the IR-1501 and CG FR-052 wording.

## Dependencies

- [FR-399](./FR-399-non-metered-text-constructor-from-admitted-parts.md) uses this normalization in the proof-path constructor; [FR-400](./FR-400-text-payload-byte-bound-per-profile.md) states the capacity argument.
- [FR-367](./FR-367-text-profile-mismatch-before-charging.md) owns the shipped `compare_text` the AC-4 and AC-5 differential reads.
- IR-1445 owns the carrier, C and the `cfg(kani)` replacement of `compare_text`, which CG FR-052-AC-12 ties to the shipped function.
- [TC-949](../test-cases/TC-949-loop-free-proof-path-text-normalization.md) is the verification home.
