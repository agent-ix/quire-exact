---
id: TC-948
title: "Payload-byte bound per text profile"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-400
    type: verifies
---
# TC-948: Payload-byte bound per text profile

## Description

Verify FR-400-AC-1 through FR-400-AC-6 through `TextProfile::payload_scalar_ratio`, `TextProfile::payload_byte_bound`, `NormalizationForm::apply` and `admit_text`. The enumeration builds its one-scalar strings once, outside the assertions, and runs under the default `cargo test` profile (about 1.1 million short normalizations per form).

## Test Procedure

1. Enumerate every Unicode scalar value except surrogates. For each, compute the `nfd` and `nfkd` scalar counts with `NormalizationForm::apply`, and decide `nfc(c) = c` and `nfkc(c) = c` by collecting `apply` of the one-scalar string. Take the maximum count over the stable scalars for each pair and over all scalars for `nfkd`. Assert the two ratios, the attaining scalar and the filter-free maximum strictly above K_KD, and assert in a `const` context that `payload_scalar_ratio` is a constant.
2. Evaluate `payload_byte_bound` for each profile at `max` of 0, 1, 2, 10 and compare with the formula; assert `payload_scalar_ratio` for the four ratio-1 profiles.
3. Evaluate it at `u64::MAX / (4 * ratio)` and one above for the five scaled profiles, and at `u64::MAX` for `binary-utf8`.
4. For each normalizing profile build the payload families of FR-400-AC-4: every scalar alone, every ordered pair from the listed scalars, the listed sequences, and `nfd(c)` and `nfkd(c)` of the attaining scalar. Compute L from `apply`, admit under `Text[L,L]` with a generous meter, assert admission, and assert the payload byte length against `payload_byte_bound(L)`.
5. For `nfc` admit `"u\u{308}\u{301}"` (five bytes, retained one scalar) under `Text[1,1]` and assert `payload_byte_bound(1)` is at least 5; assert `nfd(c)` and `nfkd(c)` of the attaining scalar are admitted by `Text[1,1]` under `nfc` and `nfkc` with scalar counts equal to the ratio; assert `"\u{1F600}"` has byte length 4 equal to `payload_byte_bound(1)` under `unicode-scalars`, `nfd` and `nfkd`.
6. Find the first (in scalar order) stable scalar with the most `nfd` and `nfkd` UTF-8 bytes, build its payload repeated `max` times for `max` 1, 2, 3, 4 and 8, admit it under `nfc` and `nfkc`, and compare its byte length with `payload_byte_bound(max)`; compare the four-byte-scalar payloads of the other profiles with their bound.

## Expected Results

- Ratios are the enumerated maxima and the bound follows the formula, with `None` exactly past `u64`.
- No admitted payload exceeds the bound, and the bound's scalar factor is reached.
- A ratio computed by enumerating `apply` at run time fails the `const` assertion of step 1; a mutant with ratio 1 for `nfc` fails step 5; a recorded attained length above `payload_byte_bound` fails step 6; a ratio one above the maximum fails step 1; one below fails steps 1 and 5 (the attaining payload has more scalars than the ratio); a byte factor of 3 fails step 5 (`"\u{1F600}"`) and step 4; a byte factor of 5 fails step 2; a wrapping or saturating multiply fails step 3; a filter-free `nfkc` ratio fails step 1; using the declared `max` as the payload bound for the normalizing profiles fails step 4.

## Status

Planned (IR-1500).
