---
id: SR-202611071713002
title: "Gap analysis of quire-exact PR 136 against FR-396 and IR FR-038-AC-516"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; PR 136 head 1220f681c4241d61fd62e16e05a8205cef4f2468 diff against origin/main a08e1b2: spec FR-396, TC-943, src/integer/reserved.rs, tests/ir1404_exact_reservation.rs; consumer context agent-ix/quire-contract-ir main 1d8c102 (FR-038 owner prerequisites and AC-516, checked_package/v2 structural.rs, timed_model.rs, natural.rs, identity.rs, operations.rs)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-396
    type: reviews
---

## Summary

Ticket: IR-1404. PR: quire-exact#136. This review is planless. Plan
completion: not assessed.

Requirement to test coverage. Each of FR-396-AC-1 to AC-7 is tagged
`#[trace("TC-943", ...)]` on one test in tests/ir1404_exact_reservation.rs
(lines 129, 170, 186, 202, 220, 255 and 279), and all seven pass. Each test
asserts the requirement's real content:

- the bytes charged equal the bytes allocated (`allocation_counter`);
- one request per block;
- the exact provision is admitted;
- every smaller provision is denied with nothing allocated beyond what it
  admitted;
- the value matches a num-bigint oracle, over both storage forms.

Untraced code: none. All new code serves FR-396. No stubs were found and
coverage is not inflated.

Consumer inventory: the operations the IR strict read performs at origin/main
1d8c102.

- structural.rs:1117 `canonical_rational`: signed parse, a
  `to_string() == text` round trip, `> 1`, `gcd` and `is_one`. AC-516 moves
  the canonical-spelling decision to text. What remains is parse, cmp, gcd
  and Eq, all covered.
- operations.rs:542 calls `canonical_rational`, as above. The Timestamp
  check at line 518 parses only an i128.
- timed_model.rs:1515 `quantity_key`: two unsigned parses, and
  `OrderPart::Ratio` cmp by cross multiplication (two `mul` and one `cmp`).
  Covered.
- natural.rs `coprime` and `compare_rationals`: parse, binary gcd (halve,
  subtract, cmp, is_one), and multiply plus cmp. `gcd_reserved` plus Eq, and
  `mul_reserved` plus Ord, cover both.
- identity.rs `ExactRatio` (dimension equivalence and unit scale):
  - `parse`, `multiply` and `add` need parse, mul, add, gcd and exact_div,
    all covered.
  - `inverse` and the `power` seed need a clone. FR-390's
    `copy_footprint` plus `try_clone` cover this.
  - The `power` loop needs the exponent's parity and halving:
    `exponent.bit(0)` and `>>= 1`.
  - `normalized_dimension` needs a signed exponent parse, `+=` (signed add),
    `is_zero` and map Eq. All covered.

No `sub`, `neg` or `abs` is needed: the signed exponent sum uses `add`.
Reducing a value is gcd followed by exact_div, and both are covered. The one
operation without an allocation-free or reserved entry is parity and halving
of a multi-limb exponent. Halving can use `exact_div_reserved` by 2 (one-limb
route, charged). Parity can come from the exponent text's last digit, which
needs no new owner entry. `Integer::is_even` on a parsed value allocates
unmetered (SR-202611071713001 FND-002). That stays a latent risk for the IR
consumer, not a gap in FR-396, which matches the FR-038 owner-prerequisite
list word for word.

## Verdict

**PASS**: there are no coverage gaps against FR-396 and no missing owner
entry against the FR-038 owner-prerequisite list. The single file required
by the schema carries one placeholder row.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
