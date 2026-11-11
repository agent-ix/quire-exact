---
id: TC-947
title: "Non-metered Text constructor from admitted parts"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-399
    type: verifies
---
# TC-947: Non-metered Text constructor from admitted parts

## Description

Verify FR-399-AC-1 through FR-399-AC-6 through the public `quire_exact` API: `Text::from_admitted`, `admit_text`, `compare_text`, `NormalizationForm::apply` and `TextType::admits`. Operands, retained sequences and the counting allocator are prepared outside each measurement window.

## Test Procedure

1. Coerce `Text::from_admitted` to `fn(TextType, TextPayload, Box<str>) -> Result<Text, TextNotAdmitted>`. For every profile and each of `""`, `"a"`, `"e\u{301}"`, `"\u{212B}"`, `"\u{FB03}"`, `"🦀"`, build `retained` with `NormalizationForm::apply` (the payload for the two non-normalizing profiles), declare `Text[L,L]` with L the profile length of `retained`, call `from_admitted` and `admit_text` (generous meter), and compare type, payload, retained sequence and length.
2. For `nfc` (`"e\u{301}"` against `"\u{E9}"`) and `binary-utf8` (`"a"` against `"ab"`), build each operand both ways and run all six operators over every budget from zero to exact, with the recorded charge log under `test-support`. Compare typed outcome, ordered charges, denied next charge and consumed counters.
3. Call `from_admitted` with the mismatches of FR-399-AC-3 (unnormalized `nfc` payload kept as retained, composed `nfd` retained, wrong `unicode-scalars` retained, last-scalar difference, proper prefix, one extra scalar), each under a type that would admit the wrong retained sequence, and assert `RetainedNotNormalForm`.
4. Call it with the length cases of FR-399-AC-4 at the minimum and maximum edges and the byte-against-scalar split `"\u{E9}"` under `Text[0,1]` and `Text[0,2]` for `binary-utf8` and `unicode-scalars`; assert the cause, and assert `RetainedNotNormalForm` when both defects are present.
5. Wrap steps 1, 3 and 4 in a counting global allocator window around the call only, and assert zero allocations.
6. When the IR-1445 carrier exists, run its host-callable counterpart and the host constructor over the FR-052-AC-2 enumeration and the C - 1, C and C + 1 payloads and compare decision and cause.

## Expected Results

- Constructor-built and admitted texts are indistinguishable by accessor and by every `compare_text` observation at every budget.
- Each mismatch and each out-of-domain length is refused with its stated cause, retained mismatch first.
- A mutant that adds a `Meter`, skips the retained check, measures the payload instead of the retained sequence, swaps the byte and scalar measures, moves a bound by one, reports the cause in the other order or allocates or clones an argument (the control payloads are non-empty, so a clone allocates) fails at least one step.
- The proof-path counterpart agrees on the enumeration and refuses C + 1.

## Status

Planned (IR-1499).
