---
id: SR-20261109172912000081
title: QSL-1826 decimal ears-conformance review
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-exact; spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md;
  spec/test-cases/TC-944-budgeted-decimal-conversion.md
review_set: subset
---

## Summary

Ticket: QSL-1826. PR: quire-exact#142. Frozen head: 7a0bf1771c5287ae352459df7b01155257e52264. Base: 7f1c7d7d89d87569dad89a33ff5d92cb9bf0a887.

Behavior 7 follows an API-triggered SHALL requirement with explicit denial response and ownership. AC-8/9 are measurable acceptance criteria rather than new requirement sentences. No EARS defect found.

## Verdict

PASS within specification review scope. No method-specific defects found; cross-method findings still block overall merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

```yaml
- id: FR-390-Behavior-1
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: 1. **Fallible copy.** `Integer::try_clone` SHALL produce a separate owned
    value numerically equal to its source for zero, inline values and arbitrary-precision
    positive and negative values. Every heap reservation performed by this method
    SHALL be fallible. An allocation refusal SHALL leave the source unchanged and
    return no copy. The existing `Clone` trait retains its public signature and successful
    mathematical value; both copy paths retain the same base-2^32 payload. Only `try_clone`
    reports typed allocation refusal.
- id: FR-390-Behavior-2
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '2. **Bounded decimal output.** `Integer::try_to_decimal_string(max_bytes)`
    SHALL emit the same ordinary base-ten spelling as `Integer`''s `Display`: `0`
    for zero, a minus sign only for a negative nonzero value, and no leading zero.
    It SHALL count the complete output length with checked arithmetic and refuse `ByteLimit
    { required, limit }` if a representable length exceeds `max_bytes`, before reserving
    the output; every such refusal has `required > limit`. If the output length cannot
    fit `usize`, the method SHALL instead return `OutputSizeOverflow` for every bound,
    including `usize::MAX`. Conseq'
- id: FR-390-Behavior-3
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: 3. **Reservation reporting.** Every allocation performed within either
    fallible operation, including temporary magnitude or conversion scratch and final
    decimal storage, SHALL be preceded by a fallible reservation. A failed reservation
    SHALL report `Allocation { requested }`, where `requested` is the minimum additional
    payload bytes requested at that reservation, computed with checked count and byte
    arithmetic. A reservation count or byte product that cannot fit `usize` SHALL
    report `requested = usize::MAX` without attempting it. The amount SHALL not be
    a proxy for the entire integer or alloca
- id: FR-390-Behavior-4
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '4. **Consumer boundary.** A QSV caller can pass its canonical preimage
    byte limit as `max_bytes`, convert an exact `ByteLimit` refusal to its input-byte-limit
    refusal only while preserving `required > bound`, and convert `Allocation { requested
    }` without changing the amount. `OutputSizeOverflow` requires a separate typed
    refusal at that boundary; it cannot become `InputBytes` with `required <= bound`,
    `Allocation` without a failed reservation, or malformed-input refusal. QSV''s
    total canonical encoding limit and RFC 8785 string encoding still apply after
    this operation. The exact kernel SHALL '
- id: FR-390-Behavior-5
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '5. **Copy footprint.** `Integer::copy_footprint` SHALL return, without
    allocating, the storage `Integer::try_clone` and `Integer::clone` reserve for
    the same value. `limbs` is the count of 32-bit magnitude limbs of the single exact-capacity
    reservation the copy performs, and is zero for a value that fits `i128`, which
    reserves nothing. `bytes` is that reservation''s minimum additional payload bytes,
    computed with the checked arithmetic of behavior 3, so it equals the `requested`
    amount a refused copy reports and is `usize::MAX` if the product does not fit
    `usize`. The result is the same for an '
- id: FR-390-Behavior-6
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: 6. **Allocation-free bit length.** `Integer::magnitude_bits` SHALL return
    the magnitude bit length, with zero reported as one, for every representation
    without allocating or building the lazy `BigInt` image.
- id: FR-390-Behavior-7
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: 7. **Composable decimal admission.** `Integer::try_to_decimal_string_with_budget(max_bytes,
    budget)` SHALL use the same conversion algorithm and produce the same text and
    decimal-output refusals as `try_to_decimal_string`. Before each scratch or output
    storage growth it SHALL call the budget's existing `Reservation::reserve` with
    the full requested capacity in bytes, then use a fallible exact-capacity reservation.
    These admissions are cumulative; releasing or replacing storage does not refund
    a charge. It SHALL admit one setup work unit, one unit before visiting each binary
    limb, one unit befo
- id: FR-390-AC-1
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '`try_clone` returns an equal, independently owned value for zero, a small
    positive and negative value, and positive and negative values larger than one
    machine limb; changing or dropping the returned value leaves the source unchanged.
    Each actual copy reservation is injectable in a crate test: refusal yields `Allocation
    { requested }`, no copy and an unchanged source, with `requested` equal to that
    reservation''s minimum additional payload bytes.'
- id: FR-390-AC-2
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '`try_to_decimal_string` returns `0`, `1`, `-1` and independently supplied
    large positive and negative decimal literals byte for byte, matching `Display`
    on success without a leading plus, leading zero or negative zero. The large controls
    cross multiple internal magnitude limbs.'
- id: FR-390-AC-3
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: 'For each successful spelling in AC-2, a bound equal to its actual UTF-8
    length permits output, and a bound one byte smaller returns `ByteLimit { required:
    actual_length, limit }` with no output. The minus sign contributes one byte. A
    synthetic output count that cannot fit `usize` returns `OutputSizeOverflow` before
    output allocation under both a finite bound and `max_bytes = usize::MAX`; no `ByteLimit`
    result has `required <= limit`. Neither count nor byte-limit refusal is reported
    as allocation failure.'
- id: FR-390-AC-4
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: With the same prepared integers and a sufficient bound, injected refusal
    at every actual decimal scratch or output reservation yields `Allocation { requested
    }`, no partial output and an unchanged source. Each amount equals the minimum
    additional payload bytes of the failed request, including multiplication by element
    size where applicable; a synthetic checked-count overflow yields `usize::MAX`.
    An independent source inspection confirms that production calls the checked-count
    calculation tested with synthetic overflow inputs and has no hidden infallible
    allocation on either fallible route.
- id: FR-390-AC-5
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: A consumer can compile a call to both public fallible operations without
    consuming its `Integer`; copy and decimal failures are typed so that allocation
    can be mapped with its actual `requested` amount, decimal byte-limit refusal maps
    separately, and output-size overflow is distinguishable from both. The existing
    `Clone`, `Display`, parsing and mathematical operations retain their public signatures
    and successful numeric behavior.
- id: FR-390-AC-6
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: For zero, `-7`, `i128::MAX`, `i128::MIN`, `2^127`, `-(2^127 + 1)`, `2^4096`
    and `-2^4096`, `copy_footprint` returns `limbs` of 0, 0, 0, 0, 4, 4, 129 and 129
    with `bytes` four times `limbs`, identically for the original and for a fresh
    `try_clone` of it. For each, the total bytes and allocation count observed around
    each actual `try_clone` and `Clone::clone` equal `bytes` and one allocation (none
    for an inline value).
- id: FR-390-AC-7
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: Observing only the call, `copy_footprint` and `magnitude_bits` on a freshly
    copied value perform zero allocations for every AC-6 value, and `magnitude_bits`
    returns 1, 127, 128, 128, 128, 129 and 4097 for zero, `i128::MAX`, `i128::MIN`,
    `2^127`, `-(2^127 + 1)`, `2^128` and `2^4096`, for the original and for a fresh
    copy.
- id: FR-390-AC-8
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: Budgeted decimal conversion returns independently supplied zero, positive,
    negative, inline and multi-limb literals byte for byte. Exact output-byte, cumulative
    storage and work bounds succeed; one-short output bytes report the existing byte
    refusal independently of the budget. Refusing each actual scratch or output reservation
    reports `Denied` with the caller's storage cause and no later reservation, preserving
    the source.
- id: FR-390-AC-9
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: A zero work budget refuses before scratch or output allocation. A denied
    scratch reservation leaves the actual conversion vector empty with zero capacity.
    Cancellation triggered after admitted conversion scratch refuses at the next conversion
    work admission with the caller's cancellation cause; an uncancelled control follows
    the same budgeted algorithm and returns the literal bytes. One-short work refuses
    distinctly from output bytes and storage. No test depends on a fixed poll number,
    elapsed time or an allocation threshold.
- id: FR-390
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: When a consumer copies an admitted `Integer`, learns what that copy will
    reserve, or renders its decimal spelling for a bounded identity preimage, the
    exact kernel SHALL expose fallible operations that preserve the mathematical value
    and distinguish a failed storage reservation from a byte-limit refusal. The consumer
    remains responsible for its enclosing identity's admission, ordering, encoding
    and total byte limit.
- id: TC-944
  path: spec/test-cases/TC-944-budgeted-decimal-conversion.md
  role: examined
  excerpt: 'Exercise the real `Integer::try_to_decimal_string_with_budget` conversion
    on

    independently supplied zero, signed inline and multi-limb decimal literals.

    Keep the source prepared before the conversion and preserve each caller''s

    typed denial cause.'
```

## Qualification

No applicable active AssuranceProfile was found in the exact-head spec tree. Normative FR/TC schema validation is unavailable for the repository module configuration; quire parse is structural parsing only. SpecReview validation uses the installed process module explicitly and is recorded separately. Advice/matrix catalog results are unavailable; manual assessment does not replace them. No code implementation or runtime behavior was qualified. Merge authority remains the IR owner; this reviewer never merges.
