---
id: FR-400
title: "State the payload-byte bound P of every text profile"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-399
    type: references
  - target: ix://agent-ix/quire-exact/FR-401
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: references
---
# FR-400: State the payload-byte bound P of every text profile

## Description

When a consumer needs a finite capacity for every payload that `admit_text` admits under a declared `TextType`, the kernel SHALL state, as a pure function of the profile and the declared `max`, an upper bound P on the byte length of those payloads, derived from the normalization expansion limits of the Unicode 17.0.0 data the kernel uses. Tests show that the bound is sound and that its scalar factor is attained. Contract Codegen (CG FR-052, prerequisite P3, ticket IR-1500) derives P as `max` for `binary-utf8` and `4 * max` for `unicode-scalars` and has no bound for `nfc`, `nfd`, `nfkc` and `nfkd`, which therefore stay unsupported in its Kani row. The declared `max` is a profile length (Unicode scalars of the retained sequence for every profile but `binary-utf8`, bytes for `binary-utf8`) and is not a payload length, because the payload of a normalizing profile can have more scalars than its retained sequence.

## Inputs

A `TextProfile` and a `max` (`u64`).

## Outputs

- `TextProfile::payload_scalar_ratio(self) -> u64`: the largest number of payload scalars that one retained scalar can come from.
- `TextProfile::payload_byte_bound(self, max: u64) -> Option<u64>`: P, or `None` when P exceeds `u64`.

## Behavior

**Statement.** `payload_scalar_ratio` is 1 for `unicode-scalars`, `binary-utf8`, `nfd` and `nfkd`, K_D for `nfc` and K_KD for `nfkc`, where

- K_D is the maximum, over every Unicode scalar value c with `nfc(c) = c`, of the number of scalars in `nfd(c)`;
- K_KD is the maximum, over every Unicode scalar value c with `nfkc(c) = c`, of the number of scalars in `nfkd(c)`.

`payload_byte_bound` is `max` for `binary-utf8`, and `4 * ratio * max` with checked multiplication for the other five profiles (four bytes is the longest UTF-8 scalar).

**Derivation.** The retained sequence R of a payload X is `nfd(X)` for `nfd`, `nfkd(X)` for `nfkd`, `nfc(X)` for `nfc`, `nfkc(X)` for `nfkc` and X itself for the two non-normalizing profiles. Let `|.|` be the scalar count. A decomposition maps every scalar to at least one scalar and canonical reordering permutes, so `|nfd(X)| >= |X|` and `|nfkd(X)| >= |X|`. For `nfd` and `nfkd` this gives `|X| <= |R|`, so the ratio is 1. For `nfc`, normalization is idempotent on the decomposition (`nfd(nfc(X)) = nfd(X)`), and the length of a full decomposition is the sum over scalars of the length of each scalar's own decomposition, because reordering changes no count. With R = r_1 ... r_n, `|X| <= |nfd(X)| = sum |nfd(r_i)|`. Every scalar r_i of a normalized string satisfies `nfc(r_i) = r_i`, so each term is at most K_D and `|X| <= K_D * n`. The `nfkc` case is the same with `nfkd(nfkc(X)) = nfkd(X)` and K_KD. The payload is therefore at most `ratio * max` scalars of at most four bytes each. The ratio is attained: for a scalar c achieving K_D, the payload `nfd(c)` has K_D scalars and `nfc(nfd(c)) = c` has one (likewise `nfkd(c)` and K_KD).

**Data.** An independent computation over the Unicode 17.0.0 `UnicodeData.txt` and `CompositionExclusions.txt` (full decomposition, Hangul algorithmic decomposition, singleton and non-starter decompositions excluded from composition) gives K_D = 4 and K_KD = 4, attained by U+1F82 and 35 further scalars (36 in all), and a longest single-scalar canonical decomposition of 4 (U+1FAF). The scalar U+FDFA, whose `nfkd` has 18 scalars, does not satisfy `nfkc(c) = c` and does not enter K_KD. So P is `4 * max` for `nfd` and `nfkd` and `16 * max` for `nfc` and `nfkc`. The constants are stated in the kernel as the values the exhaustive enumeration of FR-400-AC-1 recomputes from the pinned `unicode-normalization` tables on every test run, so a table change changes the test result, not a hidden pin.

**Looseness.** P is an upper bound, not the supremum of payload bytes. The scalar factor is attained. The byte factor is attained for `binary-utf8`, `unicode-scalars`, `nfd` and `nfkd` by a four-byte scalar that is its own normal form, but for `nfc` and `nfkc` the payload that attains `ratio` scalars does not consist of four-byte scalars (the payload of U+1F82 is 8 bytes against the bound of 16 for `max` of 1). A consumer whose capacity C is below P for such a profile treats the profile as unsupported, which narrows its supported domain and never admits a payload outside it. A byte supremum for `nfc` and `nfkc` is not derived here.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-400-AC-1 | For every Unicode scalar value (surrogates excluded), the test computes `nfd` and `nfkd` through `NormalizationForm::apply`, keeps the scalars c with `nfc(c) = c` (respectively `nfkc(c) = c`) as one-scalar strings, and takes the maximum output length. `payload_scalar_ratio` for `nfc` equals that K_D and for `nfkc` that K_KD. The test records one scalar that attains each maximum and asserts its `nfd` (respectively `nfkd`) has exactly that many scalars. It asserts that the `nfkd` maximum over all scalars without the stability filter (U+FDFA, 18 scalars) is strictly greater than K_KD, so a filter-free implementation fails. `payload_scalar_ratio` is 1 for `nfd`, `nfkd`, `unicode-scalars` and `binary-utf8`. | Test (TC-948) |
| FR-400-AC-2 | `payload_byte_bound` equals `max` for `binary-utf8` and `4 * ratio * max` for the other five profiles, at `max` of 0, 1, 2 and 10 for every profile. | Test (TC-948) |
| FR-400-AC-3 | At the overflow edge, `payload_byte_bound` returns `Some` at `max = u64::MAX / (4 * ratio)` and `None` at that value plus one for each of the five scaled profiles, and returns `Some(u64::MAX)` for `binary-utf8` at `u64::MAX`. No result wraps or saturates. | Test (TC-948) |
| FR-400-AC-4 | Soundness: for every normalizing profile, every Unicode scalar value used alone as a payload, every ordered pair drawn from the scalars U+0061, U+0075, U+0041, U+0300, U+0301, U+0307, U+0308, U+0310, U+0323, U+0915, U+093C, U+1100, U+1161, U+11A8, U+1F82, U+1F600, U+212B, U+FB03, U+FDFA, and the payloads `"u\u{308}\u{301}"`, `"a\u{307}\u{323}"`, `"a\u{310}\u{300}"`, `"\u{1100}\u{1161}\u{11A8}"`, `nfd(c)` and `nfkd(c)` for the attaining scalar c of AC-1: with L the profile length of the retained sequence (`NormalizationForm::apply`), `admit_text` under `Text[L,L]` admits the payload and the payload's byte length is at most `payload_byte_bound(L)`. | Test (TC-948) |
| FR-400-AC-5 | Tightness of the scalar factor: for `nfc` the payload `"u\u{308}\u{301}"` (three scalars, retained `"\u{1D8}"`, one scalar, five bytes) exceeds `payload_byte_bound(1)` computed with ratio 1, the attaining payload `nfd(c)` of AC-1 has exactly `payload_scalar_ratio` scalars and is admitted by `Text[1,1]` under `nfc`, and the payload `nfkd(c)` is admitted by `Text[1,1]` under `nfkc` with its scalar count equal to the ratio. A four-byte scalar that is its own normal form (`"\u{1F600}"`) has byte length equal to `payload_byte_bound(1)` for `unicode-scalars`, `nfd` and `nfkd`. | Test (TC-948) |

## Status

Planned (IR-1500). No Rust or test exists. The numeric values 4 and 4 are derived above from the Unicode 17.0.0 data files; the owner test (AC-1) is what binds them to the pinned `unicode-normalization` 0.1.25 tables. Because K_D equals K_KD, `nfc` and `nfkc` share a formula and no control distinguishes a swap between them; none is claimed.

## Dependencies

- [FR-399](./FR-399-non-metered-text-constructor-from-admitted-parts.md) and [FR-401](./FR-401-loop-free-proof-path-text-normalization.md) are the sibling prerequisites; FR-401 uses `4 * max` bytes as the output capacity of admitted retained sequences.
- QSpec FR-141 owns the six text profiles; the module documentation of `text.rs` selects Unicode 17.0.0.
- [TC-948](../test-cases/TC-948-text-payload-byte-bound-per-profile.md) is the verification home.
