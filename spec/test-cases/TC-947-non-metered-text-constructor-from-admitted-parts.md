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

Verify FR-399-AC-1 through FR-399-AC-7 through the public `quire_exact` API: `Text::from_admitted`, `admit_text`, `compare_text`, `NormalizationForm::apply` and `TextType::admits`. Operands, retained sequences and the counting allocator are prepared outside each measurement window.

## Test Procedure

1. Coerce `Text::from_admitted` to `fn(TextType, TextPayload, Box<str>) -> Result<Text, TextNotAdmitted>`. For every profile and each of `""`, `"a"`, `"e\u{301}"`, `"\u{212B}"`, `"\u{FB03}"`, `"🦀"`, build `retained` with `NormalizationForm::apply` (the payload for the two non-normalizing profiles), declare `Text[L,L]` with L the profile length of `retained`, call `from_admitted` and `admit_text` (generous meter), and compare type, payload, retained sequence and length.
2. For `nfc` (`"e\u{301}"` against `"\u{E9}"`) and `binary-utf8` (`"a"` against `"ab"`), build each operand both ways and run all six operators over every budget from zero to exact, with the recorded charge log under `test-support`. Compare typed outcome, ordered charges, denied next charge and consumed counters.
3. Call `from_admitted` with the mismatches of FR-399-AC-3 (unnormalized `nfc` payload kept as retained, composed `nfd` retained, wrong `unicode-scalars` retained, last-scalar difference, proper prefix, one extra scalar), each under a type that would admit the wrong retained sequence, and assert `RetainedNotNormalForm`.
4. Call it with the length cases of FR-399-AC-4 at the minimum and maximum edges and the byte-against-scalar split `"\u{E9}"` under `Text[0,1]` and `Text[0,2]` for `binary-utf8` and `unicode-scalars`; assert the cause, and assert `RetainedNotNormalForm` when both defects are present.
5. Under a global allocator whose counter is a const-initialized `thread_local!` `Cell`, assert zero allocations around the call only for the cases named in FR-399-AC-5.
6. When the IR-1445 carrier exists, run its host-callable counterpart and the host constructor over the FR-052-AC-2 enumeration, each string with the correct, last-scalar-replaced and empty retained sequences, and over the A - 1 and A boundary payloads of FR-399-AC-6 (final byte a one-byte scalar or a continuation byte); compare decision and cause; assert the carrier refuses a payload of C + 1 bytes.
7. Apply the scanner and allocation window of FR-401-AC-3 and FR-401-AC-4 to the counterpart and its module over the step 6 inputs.

## Expected Results

- Constructor-built and admitted texts are indistinguishable by accessor and by every `compare_text` observation at every budget.
- Each mismatch and each out-of-domain length is refused with its stated cause, retained mismatch first.
- A mutant that adds a `Meter`, skips the retained check, measures the payload instead of the retained sequence, swaps the byte and scalar measures, moves a bound by one, reports the cause in the other order or allocates or clones an argument (the control payloads are non-empty, so a clone allocates) fails at least one step.
- The proof-path counterpart agrees on the enumeration and boundary family, refuses C + 1, and passes the allocation and scan checks.

## Status

AC-1 to AC-5: Planned (IR-1499). AC-6 and AC-7: Executed (IR-1445, planner ruling) in `tests/ir1445_native_text.rs` by `proof_path_constructor_decides_like_the_owner_over_the_enumeration`, `proof_path_constructor_refuses_c_plus_one_and_allocates_nothing` and `proof_path_constructor_module_scans_clean`. Deviations: the host `Text::from_admitted` does not exist, so `admit_text` is the host counterpart of the decision; the zero-allocation measurement covers the supported profiles only (FR-399-AC-7 carries the same carve-out); the enumeration is every UTF-8 byte string of at most two bytes plus every string of at most three scalars over a four-scalar alphabet, with C - 1 and C boundary payloads, not the full enumeration of CG FR-052-AC-2.
