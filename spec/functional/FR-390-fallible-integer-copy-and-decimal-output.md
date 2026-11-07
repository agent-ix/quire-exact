---
id: FR-390
title: "Copy and render arbitrary-precision integers with typed allocation refusal"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-389
    type: references
---
# FR-390: Copy and render arbitrary-precision integers with typed allocation refusal

## Description

When a consumer copies an admitted `Integer`, learns what that copy will reserve, or renders its decimal spelling for a bounded identity preimage, the exact kernel SHALL expose fallible operations that preserve the mathematical value and distinguish a failed storage reservation from a byte-limit refusal. The consumer remains responsible for its enclosing identity's admission, ordering, encoding and total byte limit.

## Inputs

- A borrowed, already constructed `Integer` of arbitrary magnitude and either sign.
- For decimal output, a caller-supplied maximum UTF-8 output byte count. The bound applies to the decimal text including a minus sign, before any enclosing JSON escaping or punctuation.

## Outputs

- An owned, numerically equal `Integer` from a fallible copy, or a typed `Allocation { requested: usize }` refusal.
- The exact canonical decimal text in an owned `String`, or a typed decimal-output refusal distinguishing `Allocation { requested: usize }`, `ByteLimit { required: usize, limit: usize }`, and `OutputSizeOverflow`. An output count that cannot fit `usize` reports `OutputSizeOverflow` before attempting allocation; it is not a byte-limit refusal with a fabricated or saturated required count.
- An `IntegerCopyFootprint { limbs, bytes }` from `Integer::copy_footprint`, and the `u64` magnitude bit length from `Integer::magnitude_bits`.

## Behavior

1. **Fallible copy.** `Integer::try_clone` SHALL produce a separate owned value numerically equal to its source for zero, inline values and arbitrary-precision positive and negative values. Every heap reservation performed by this method SHALL be fallible. An allocation refusal SHALL leave the source unchanged and return no copy. The existing `Clone` trait retains its public signature and successful mathematical value; both copy paths retain the same base-2^32 payload. Only `try_clone` reports typed allocation refusal.
2. **Bounded decimal output.** `Integer::try_to_decimal_string(max_bytes)` SHALL emit the same ordinary base-ten spelling as `Integer`'s `Display`: `0` for zero, a minus sign only for a negative nonzero value, and no leading zero. It SHALL count the complete output length with checked arithmetic and refuse `ByteLimit { required, limit }` if a representable length exceeds `max_bytes`, before reserving the output; every such refusal has `required > limit`. If the output length cannot fit `usize`, the method SHALL instead return `OutputSizeOverflow` for every bound, including `usize::MAX`. Consequently, `max_bytes = usize::MAX` cannot produce a `ByteLimit` refusal. At the exact representable bound it SHALL succeed if storage is available. It SHALL not construct an infallibly allocated intermediate radix string, BigInt copy or unbounded output before the bound check. The retained source remains unchanged on every refusal.
3. **Reservation reporting.** Every allocation performed within either fallible operation, including temporary magnitude or conversion scratch and final decimal storage, SHALL be preceded by a fallible reservation. A failed reservation SHALL report `Allocation { requested }`, where `requested` is the minimum additional payload bytes requested at that reservation, computed with checked count and byte arithmetic. A reservation count or byte product that cannot fit `usize` SHALL report `requested = usize::MAX` without attempting it. The amount SHALL not be a proxy for the entire integer or allocator capacity. These paths use no infallible `Clone`, radix formatting, `collect`, `push` without established capacity, or equivalent hidden reservation.
4. **Consumer boundary.** A QSV caller can pass its canonical preimage byte limit as `max_bytes`, convert an exact `ByteLimit` refusal to its input-byte-limit refusal only while preserving `required > bound`, and convert `Allocation { requested }` without changing the amount. `OutputSizeOverflow` requires a separate typed refusal at that boundary; it cannot become `InputBytes` with `required <= bound`, `Allocation` without a failed reservation, or malformed-input refusal. QSV's total canonical encoding limit and RFC 8785 string encoding still apply after this operation. The exact kernel SHALL not mint a compound-unit identity or encode JSON.
5. **Copy footprint.** `Integer::copy_footprint` SHALL return, without allocating, the storage `Integer::try_clone` and `Integer::clone` reserve for the same value. `limbs` is the count of 32-bit magnitude limbs of the single exact-capacity reservation the copy performs, and is zero for a value that fits `i128`, which reserves nothing. `bytes` is that reservation's minimum additional payload bytes, computed with the checked arithmetic of behavior 3, so it equals the `requested` amount a refused copy reports and is `usize::MAX` if the product does not fit `usize`. The result is the same for an inline value, a value whose `BigInt` is retained, a value produced by `try_clone`, and a value whose lazy `BigInt` image has not been built; it SHALL not build that image, and it covers values just outside `i128` such as `2^127` and `-(2^127 + 1)`. Both paths copy directly from the retained representation, without materializing a lazy source image. A consumer charges `bytes` before calling either copying operation and needs no layout assumption of its own.
6. **Allocation-free bit length.** `Integer::magnitude_bits` SHALL return the magnitude bit length, with zero reported as one, for every representation without allocating or building the lazy `BigInt` image.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-390-AC-1 | `try_clone` returns an equal, independently owned value for zero, a small positive and negative value, and positive and negative values larger than one machine limb; changing or dropping the returned value leaves the source unchanged. Each actual copy reservation is injectable in a crate test: refusal yields `Allocation { requested }`, no copy and an unchanged source, with `requested` equal to that reservation's minimum additional payload bytes. | Test |
| FR-390-AC-2 | `try_to_decimal_string` returns `0`, `1`, `-1` and independently supplied large positive and negative decimal literals byte for byte, matching `Display` on success without a leading plus, leading zero or negative zero. The large controls cross multiple internal magnitude limbs. | Test |
| FR-390-AC-3 | For each successful spelling in AC-2, a bound equal to its actual UTF-8 length permits output, and a bound one byte smaller returns `ByteLimit { required: actual_length, limit }` with no output. The minus sign contributes one byte. A synthetic output count that cannot fit `usize` returns `OutputSizeOverflow` before output allocation under both a finite bound and `max_bytes = usize::MAX`; no `ByteLimit` result has `required <= limit`. Neither count nor byte-limit refusal is reported as allocation failure. | Test |
| FR-390-AC-4 | With the same prepared integers and a sufficient bound, injected refusal at every actual decimal scratch or output reservation yields `Allocation { requested }`, no partial output and an unchanged source. Each amount equals the minimum additional payload bytes of the failed request, including multiplication by element size where applicable; a synthetic checked-count overflow yields `usize::MAX`. An independent source inspection confirms that production calls the checked-count calculation tested with synthetic overflow inputs and has no hidden infallible allocation on either fallible route. | Test |
| FR-390-AC-5 | A consumer can compile a call to both public fallible operations without consuming its `Integer`; copy and decimal failures are typed so that allocation can be mapped with its actual `requested` amount, decimal byte-limit refusal maps separately, and output-size overflow is distinguishable from both. The existing `Clone`, `Display`, parsing and mathematical operations retain their public signatures and successful numeric behavior. | Test |
| FR-390-AC-6 | For zero, `-7`, `i128::MAX`, `i128::MIN`, `2^127`, `-(2^127 + 1)`, `2^4096` and `-2^4096`, `copy_footprint` returns `limbs` of 0, 0, 0, 0, 4, 4, 129 and 129 with `bytes` four times `limbs`, identically for the original and for a fresh `try_clone` of it. For each, the total bytes and allocation count observed around each actual `try_clone` and `Clone::clone` equal `bytes` and one allocation (none for an inline value). | Test |
| FR-390-AC-7 | Observing only the call, `copy_footprint` and `magnitude_bits` on a freshly copied value perform zero allocations for every AC-6 value, and `magnitude_bits` returns 1, 127, 128, 128, 128, 129 and 4097 for zero, `i128::MAX`, `i128::MIN`, `2^127`, `-(2^127 + 1)`, `2^128` and `2^4096`, for the original and for a fresh copy. | Test |

## Status

Specified. Copy footprint and allocation-free bit length are implemented; QSV consumption is pending IR-1149.

## Dependencies

- [FR-389](./FR-389-carry-admitted-structural-identities.md) owns structural node copying; this requirement owns only `Integer` copying and decimal text.
- QSV uses these operations for compound-unit term exponents and remains responsible for fallible term collection, structural key copying and canonical encoding. QSL FR-259 owns the compound-unit identity interpretation.
