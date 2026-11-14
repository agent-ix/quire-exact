---
id: SR-17940486709564847047950
title: "IR-1446 PR #159 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-exact PR #159 at 62fefed0, git diff origin/main...HEAD of code/ir-1446-native-float-carrier; IR-1446; src/native_float.rs, src/native_float/kani_proofs.rs, src/ieee.rs, src/lib.rs, tests/ir1446_native_float.rs, scripts/ir1446_native_float_mutants.py"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-405
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-953
    type: reviews
---

## Summary

Ticket: IR-1446. The PR adds `NativeFloat` (a width tag and a `u64`, `Copy`, no drop glue), `evaluate_native_float` and `compare_native_float`. These are a `u128`/`i32` reimplementation of `evaluate_ieee`/`compare_ieee`. It also adds 20 Kani harnesses, a 20-test host file and a 49-mutant script, and widens `IeeeFlags::with` to `pub(crate)`.

Reviewer reproduction at 62fefed0. Every cargo step ran under `flock /tmp/cargo-slot.lock`, and Kani ran in `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, foreground, one job:

- `cargo test --release --test ir1446_native_float`: 19 passed, 1 ignored (the mutant runner). With `--features test-support`: 18 passed, 1 ignored. The counting-allocator test is `cfg(not(test-support))`, and `make test` runs both feature sets.
- Scratch-copy host probes (not in the PR):
  - FMA crossed over the full 44-pattern grid in a, b and c, times 6 spellings and 2 widths, against the owner: pass, 0.5 s in release.
  - 300,000 biased random triples per width: native `NearestEven` add, sub, mul, div, sqrt and fma equal the host `f32`/`f64` bits on every non-NaN result.
- Scratch-copy Kani probes (Kani 0.68.0, CBMC 6.11.0, cadical). Each copies the PR's harness unchanged, except that the rounding is fixed to `NearestEven` instead of `any_rounding()`. Each probe hit exactly 1 failed check of 3345 (the host-side NaN check, FND-001), and every other check, the native-vs-host assertions included, was SUCCESS. Covers were 4 of 4 each.
  - `float32_add`: 51.5 s, `memory.peak` 1,044,566,016 B.
  - `float32_multiply`: 40.0 s, 986,763,264 B.
  - `float64_add`: 345.7 s, 993,665,024 B.

  The PR records all three combinations as 600 s timeouts with peaks of 22.7-23.3 GB.

Question (b), independence: accepted. The arithmetic expected side is CBMC's own IEEE model of `f32`/`f64` `+ - * / sqrt mul_add`. The comparison side is host `==`, bit equality and a separately written signed fold for `TotalOrder` (checked: it is the `total_cmp` rule). Neither shares a function with the native integer body, not even the decode. The NaN rule (leftmost operand NaN quieted, else canonical) is a restatement of the owner's policy, not taken from the native code. The harness module is `cfg(kani)` only at its mount in `src/lib.rs`, and the native bodies have no `cfg(kani)` token. `evaluate_native_float` and `compare_native_float` are `pub` and host-called by every differential.

Question (c): `IeeeFlags::with` becoming `pub(crate)` is a bitmask constructor, not owner arithmetic. It widens no public API and does not touch the Rational path, so it is fine. All 20 tests carry `#[trace("TC-953", "FR-405-AC-n")]` (ix-trace-rs); the AC-12 gap is in the gap-analysis artifact. The 49 mutants match the script and use exact-text edits in scratch copies with a target dir under `target/mutants`. The boundary grid is from independent literals, both signs, as Behavior 17 lists. The gaps are FND-004 and FND-005.

Arithmetic body: no defect found. Shift amounts are bounded (`distance >= 128` and `shift >= 128` are guarded, and `-shift` is below 128 because `quantum >= log2 - precision`). The jammed-subtract branch cannot reach `mag = 0` with `sticky`, because a jam needs a distance above the trailing zeros of a normalised significand, which are at least 19. The overflow test `index + 1 >= exponent_ones` matches the biased exponent. The differential and the probes above agree.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Every arithmetic harness computes its expected side with a host float operation (`a.1 + b.1`, `*`, `/`, `sqrt`, `mul_add`) over unconstrained operands. Kani's default `--nan-check` then fails on the NaN-producing inputs (inf + -inf, 0 * inf, 0/0, sqrt of a negative). Reproduced: the scratch probes report `VERIFICATION:- FAILED` with 'NaN on addition' / 'NaN on multiplication' at the `host` line as the only failed check. The recorded runner (`cargo kani ... --unwind 57 --jobs 1 --solver cadical`) does not turn that check off. So none of the 12 arithmetic harnesses as written can ever return VERIFIED: given time, their terminal verdict is FAILED from a harness defect, not timeout. Compute the expected value without a NaN-producing host operation (decide the invalid cases first and take a NaN constant), or justify and record a runner flag that disables only the float NaN check. Then re-run. | src/native_float/kani_proofs.rs:160-176, src/native_float/kani_proofs.rs:178-268 |
| FND-002 | high | The 12 'table row: timeout' rows measure this harness formulation, not the operation. Each harness draws the rounding symbolically over all six spellings (`any_rounding()`), but it checks result bits only for `NearestEven`/`Exact`. With the spelling fixed to `NearestEven` and nothing else changed, the probes finish in 40-346 s at about 1 GB: `float32_add` 51.5 s, `float32_multiply` 40.0 s, `float64_add` 345.7 s. Every native-vs-host assertion holds, against 600 s killed at 22.7-23.3 GB in the record. IR-1526's seam will load these rows and refuse CG's float add/mul rows that are tractable. Behavior 19 is met by per-spelling harnesses that together cover the six spellings. Split each arithmetic harness by spelling (at least `NearestEven`/`Exact` apart from the directed modes), fix FND-001, re-measure every arithmetic combination under the 600 s cap and replace the rows. Rows that still time out after the split are honest table rows. Do this before merge: it is this PR's AC-11 evidence, not a follow-up. | src/native_float/kani_proofs.rs:44-48, src/native_float/kani_proofs.rs:160-176, spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:79-98 |
| FND-003 | medium | TC-953's mutant table has buildable sites with no mutant in the script. TC-953 Status says 'one per row of the table above that has a buildable form', and FR-405-AC-10 needs 'each mutant of TC-953'. Missing: (1) 'the charge of the operand points is ... reordered' (for example swapping the `ieee.exact-intermediate`/`ieee.round` charges); (2) 'the denied charge is followed by a later charge' / 'first charge made after the width check' (for example `let _ =` in place of `?` on a charge); (3) 'Negate raises a flag'; (4) 'TotalOrder ignores ... the payload of a NaN'; (5) '`divide_by_zero` is raised for zero over zero' (the script mutates the one-over-zero site instead). Add one mutant per row, each with the test that kills it, or mark a row non-buildable with a reason. | scripts/ir1446_native_float_mutants.py:37-139, spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:33-71 |
| FND-004 | medium | The FMA differential does not cross the full boundary grid. `grid_operations` takes a and b from `step_by(3)` (15 of 44 patterns). That drops, among others, +infinity, both signalling NaNs, the positive largest finite and smallest subnormal. The comment says 'then the full grid in the host differential below', but no test does that. FR-405-AC-10 and Behavior 17 require each operation on the full grid. The full cross passes in 0.5 s (reviewer probe), so cost is no reason to reduce it. In the same way, the AC-6 charge test (`charges_denials_and_incomplete_records...`) uses `step_by(2)` picks and three spellings, while AC-6 says 'for every boundary case'. Cross the full grid, or state the reduction and its reason in TC-953. | tests/ir1446_native_float.rs:249-276, tests/ir1446_native_float.rs:644-660 |
| FND-005 | low | The AC-11/AC-13 evidence test is weaker than it reads. The harness-id check strips the width prefix and asserts `harness_source.contains("add")` (or `"negate"`, `"divide"` ...), which any such word in the file satisfies. The peak check for a non-VERIFIED row accepts any row whose text says 'above the 16 GiB ceiling' and never compares the number. Match the harness name itself (`float32_add,` in the macro call) and assert `peak > 16 << 30` whenever the row claims it is above. | tests/ir1446_native_float.rs:1036-1040, tests/ir1446_native_float.rs:1030-1033 |

## Verdict

Not mergeable as is. The carrier and native operations look correct: the boundary-grid, random and literal differentials pass, and reviewer probes against host floats pass. Host-callability, separation and the absence of `cfg(kani)` bodies are as FR-405 requires. The four VERIFIED comparison and Negate rows stand. The arithmetic Kani evidence does not. FND-001 means the harnesses cannot reach VERIFIED as written. FND-002 shows the recorded timeouts come from the symbolic six-spelling formulation: three probed combinations verify in at most 346 s at about 1 GB once the spelling is fixed. Fix FND-001 and FND-002 and re-measure in this PR. FND-003 and FND-004 are test gaps, and FND-005 is low.

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-405-AC-1 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:77 | The carrier is `Copy`, has no drop glue and holds no owner `Integer`, `Rational`, `IeeeValue` or heap type, and the conversions to and from `IeeeValue` round-trip every boundary pattern. |
| FR-405-AC-2 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:78 | The constructor refuses a `Binary32` pattern above `u32::MAX` ... with `OutsideCarrierDomain`, never masks it to `0`, takes no `Meter` ... |
| FR-405-AC-3 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:79 | The native evaluation supports exactly Negate, Add, Subtract, Multiply, Divide, SquareRoot and FusedMultiplyAdd ...; a cross-width call is `ill_typed` and a `Meter` presented to it is unchanged. |
| FR-405-AC-4 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:80 | For each of the five rounding modes and `Exact`, the native result bits equal the owner's on the tie and one-ulp-off-tie operands ... |
| FR-405-AC-5 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:81 | The flag set equals the owner's on the boundary grid ...; and the NaN result equals the owner's in sign, quietness and payload. |
| FR-405-AC-6 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:82 | The charge points, their order, limit kinds and amounts, the denied charge and the incomplete record equal the owner's at a sufficient, an exact and a one-less limit for every boundary case ... |
| FR-405-AC-7 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:83 | The proof-path source contains no `num_bigint` item, ... no `cfg(kani)` token, no iterator adapter and no data-dependent loop ... |
| FR-405-AC-8 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:84 | A counting allocator around the native operations over the boundary grid observes zero allocations ... and a host test calls each native operation and comparison. |
| FR-405-AC-9 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:70 | The expected side of every Kani harness shares no function with the native body, except the bit-pattern decode, and the harness docstring states the outputs it determines (Behavior 19). |
| FR-405-AC-10 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:86 | The host differential passes on the full boundary grid, each of the six rounding spellings and each operation and comparison, and each mutant of TC-953 fails it at the step TC-953 names ... |
| FR-405-AC-11 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:87 | For each combination of operation and width, either a Kani harness of Behavior 19 returns an executed terminal VERIFIED verdict, or a measured row exists whose outcome is one of timeout, memory exhaustion or unreliable ... A FAILED verdict ... fails this criterion ... |
| FR-405-AC-12 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:88 | Each VERIFIED harness has a vacuity cover that is reached and a harness mutant that it fails ... |
| FR-405-AC-13 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:89 | The peak memory of every executed harness, VERIFIED or not, is recorded with its provenance and is at most the 16 GiB ceiling of CG FR-028 ... |
| FR-405-AC-14 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:90 | The refusal of a combination in the table-row state is that of IR-1526's seam: the kernel defines no refusal variant ... |
| src/ieee.rs IeeeFlags::with | examined | src/ieee.rs:247 | pub(crate) fn with(self, flag: IeeeFlag) -> Self { Self(self.0 \| flag.mask()) } |
