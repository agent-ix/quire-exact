---
id: SR-2026110101334
title: "IR-1395 base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; IR-1395; src/integer.rs; src/lib.rs; tests/ir1395_copy_footprint.rs; FR-390; TC-939; existing clone mechanism tests"
review_set: subset
---

## Summary

The added owner footprint contract supplies concrete signed and cold boundaries, with one verification-column formatting violation.

Frozen head: 40c011eb532df95347bc65d87b59925ebcbdac66. Ticket: IR-1395.

## Verdict

**PASS after disposition round 1** — All recorded findings fixed by inspection; merge remains subject to pending focused runtime checks and the lead’s final aggregate gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | New AC6/AC7 Verification cells contain TC-939 instead of only the verification method. | spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md:45 |

## Coverage

Reviewed both copies against the representation branches, signed i128 edge cases and the odd 129-u32-limb control. Public re-export, docs, test traces and retained output lifetimes were examined. No CI or dependency files changed. No applicable AssuranceProfile was found under spec/.

Focused tests were queued under the shared Cargo lock when reviewed; reviewer ran no Cargo and claims no runtime pass. Final aggregate gate belongs to the lead. Deterministic validation: 2/2 changed docs grammar-clean. Optional criterion-strength/Jev and optional semantic-audit fan-out were not selected.

## Examined units

```yaml
scope:
- artifact_id: FR-390-AC-1
  role: context_only
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  excerpt: '`try_clone` returns an equal, independently owned value for zero, a small
    positive and negative value, and positive and negative values larger than one
    machine limb; changing or dropping the returned value leaves the source unchanged.
    Each actual copy reservation is injectable in a crate test: refusal yields `Allocation
    { requested }`, no copy and an unchanged source, with `requested` equal to that
    reservation''s minimum additional payload bytes.'
- artifact_id: FR-390-AC-2
  role: context_only
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  excerpt: '`try_to_decimal_string` returns `0`, `1`, `-1` and independently supplied
    large positive and negative decimal literals byte for byte, matching `Display`
    on success without a leading plus, leading zero or negative zero. The large controls
    cross multiple internal magnitude limbs.'
- artifact_id: FR-390-AC-3
  role: context_only
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  excerpt: 'For each successful spelling in AC-2, a bound equal to its actual UTF-8
    length permits output, and a bound one byte smaller returns `ByteLimit { required:
    actual_length, limit }` with no output. The minus sign contributes one byte. A
    synthetic output count that cannot fit `usize` returns `OutputSizeOverflow` before
    output allocation under both a finite bound and `max_bytes = usize::MAX`; no `ByteLimit`
    result has `required <= limit`. Neither count nor byte-limit refusal is reported
    as allocation failure.'
- artifact_id: FR-390-AC-4
  role: context_only
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  excerpt: With the same prepared integers and a sufficient bound, injected refusal
    at every actual decimal scratch or output reservation yields `Allocation { requested
    }`, no partial output and an unchanged source. Each amount equals the minimum
    additional payload bytes of the failed request, including multiplication by element
    size where applicable; a synthetic checked-count overflow yields `usize::MAX`.
    An independent source inspection confirms that production calls the checked-count
    calculation tested with synthetic overflow inputs and has no hidden infallible
    allocation on either fallible route.
- artifact_id: FR-390-AC-5
  role: context_only
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  excerpt: A consumer can compile a call to both public fallible operations without
    consuming its `Integer`; copy and decimal failures are typed so that allocation
    can be mapped with its actual `requested` amount, decimal byte-limit refusal maps
    separately, and output-size overflow is distinguishable from both. The existing
    `Clone`, `Display`, parsing and mathematical operations retain their public signatures
    and successful numeric behavior.
- artifact_id: FR-390-AC-6
  role: examined
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  excerpt: For zero, `-7`, `i128::MAX`, `i128::MIN`, `2^127`, `-(2^127 + 1)`, `2^4096`
    and `-2^4096`, `copy_footprint` returns `limbs` of 0, 0, 0, 0, 4, 4, 129 and 129
    with `bytes` four times `limbs`, identically for the original and for a fresh
    `try_clone` of it. For each, the total bytes and allocation count observed around
    each actual `try_clone` and `Clone::clone` equal `bytes` and one allocation (none
    for an inline value).
- artifact_id: FR-390-AC-7
  role: examined
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  excerpt: Observing only the call, `copy_footprint` and `magnitude_bits` on a freshly
    copied value perform zero allocations for every AC-6 value, and `magnitude_bits`
    returns 1, 127, 128, 128, 128, 129 and 4097 for zero, `i128::MAX`, `i128::MIN`,
    `2^127`, `-(2^127 + 1)`, `2^128` and `2^4096`, for the original and for a fresh
    copy.
- artifact_id: TC-939
  role: examined
  path: spec/test-cases/TC-939-fallible-integer-copy-and-decimal-output.md
  excerpt: Call copy_footprint for boundary and arbitrary-size controls; observe actual
    try_clone and Clone allocations, prepare cold sources before each observation,
    and run numeric value oracles after copying.
```

## Dispositions

Round 1, head f78ed027b756977a335cea4f0c0c2a859795dc73.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Both new Verification cells now contain only Test; their criterion statements and real TC-939 trace bindings remain intact. |

Fixed text:

```rust
`i128::MIN`, `2^127`, `-(2^127 + 1)`, `2^4096` and `-2^4096`, `copy_footprint` returns `limbs` of 0, 0, 0, 0, 4, 4, 129 and 129 with `bytes` four times `limbs`, identically for the original and for a fresh `try_clone` of it. For each, the total bytes and allocation count observed around each actual `try_clone` and `Clone::clone` equal `bytes` and one allocation (none for an inline value). | Test |
```

No new findings. The boundary fixture now retains the original Large positive 2^127 instead of cloning it during preparation. Runtime checks remain queued under the shared Cargo lock; no runtime pass is claimed.
