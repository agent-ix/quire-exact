# IR-705 Kani arithmetic investigation, round 2

Measured on 2026-10-16 with Kani 0.68.0, CBMC 6.11.0, CaDiCaL, `--unwind 3`, one Kani process at a time, `timeout 700`, and `/usr/bin/time -v`. The control used contract-runtime `3cc88c48`; the quire-exact baseline used `55d7e36`. Each probe made CG's full-range `kani::any::<i64>()` declarations and assumptions, converted the operands with `Integer::from`, created a meter with every limit at `u64::MAX`, and called `evaluate_integer_arithmetic(..., None, &mut meter)`. It asserted only that the call returns. CG's actual generated proof additionally checks the native `i128` result and coverage; this probe does not substitute for that receipt.

## Control and ladder

| Rung | Kernel and operands | Verdict | Wall | Peak RSS |
| --- | --- | --- | ---: | ---: |
| Control | old runtime, Add, two symbolic `i64` | Failed: `big_from_i128` array conversion unwind check | 24.76 s | 0.68 GiB |
| R0 | quire-exact, Add(4, 1) | Verified | 33.16 s | 1.52 GiB |
| R1 | quire-exact, Add(symbolic, 1) | Interrupted during propositional reduction when host free memory reached 1.6 GiB and swap was full; no `/usr/bin/time` final record | At least 90 s of CBMC work | Unmeasured |
| R2 | quire-exact, Add(two symbolic `i64`) | Memory exhausted in round 1; trusted supplied result, not rerun here | 162 s | 15.2 GiB |
| R3 | quire-exact, Negate(symbolic `i64`) | Failed with BigInt conversion unwind error under a 10,000,000 KiB virtual-address cap | 42.96 s | 9.32 GiB |

The control is **not a valid passing reference at unwind 3**: its probe fails, rather than matching the reported CG receipt of about 7 GiB within 434 seconds. Its failure is an unwind check, not a semantic counterexample. The generated CG harness has an additional oracle wrapper and assertions, so the old kernel's 0.68 GiB return-only result does not establish a same-cost comparison with CG's five passing tests.

## Path departures

| Path part | Old runtime `3cc88c48` | quire-exact `55d7e36` | Relevance |
| --- | --- | --- | --- |
| `Integer` representation | Plain struct: inline `i64` plus `Option<Box<BigInt>>`; promoted values live in `big` | `Small { i128, OnceBox<BigInt> }` or `Large(BigInt)` | The current inline range admits values whose addition can overflow `i128`. |
| `From<i64>` | Stores the `i64` inline | Widens to inline `i128` | Both inputs are small in the CG probe. |
| Add/Sub small path | Widens `i64` operands to `i128` and uses wrapping arithmetic; exact because the sum/difference fits | `checked_add`/`checked_sub` on inline `i128`, then `from_big_normalized(self.as_big() ± other.as_big())` on overflow | CBMC symbolically explores the BigInt fallback even for `i64`-origin operands. Negate has the analogous checked overflow branch. |
| `magnitude_bits` / `to_u64` | Small `i64` via `unsigned_abs`/`checked_ilog2` and `u64::try_from` | Small `i128` via `unsigned_abs`/`leading_zeros` and `u64::try_from` | Affects charge amounts but charge-only probes were cheap in round 1. |
| Clone and BigInt view | Derived clone of inline/boxed representation; `as_big` returns a borrowed or owned `Cow` | Inline clone starts with an empty cache; `as_big` initializes a retained `OnceBox` | Cache bypass alone did not reduce R2 below 8 GiB. |
| Outcome carrier | Constructs `Outcome` directly, with a source comment explaining Kani's `Result` niche cost | Builds `Result<Integer, Stop>` then `Outcome::from_stop` | Direct construction, even paired with `#[repr(u64)]`, did not reduce R2 below 8 GiB. |
| Meter | Three charges; sorts requested sizes and retains a bounded log | Same three charge points; single-pass inline-size validation, test-support-only log, optional cancel poll | Round-1 charge-only L0–L6 verified in 5–15 s at at most 771 MiB. The CG meter has no cancel handle. |

## Isolating scratch experiments

These edits were confined to a detached scratch worktree, never put into the production branch. Runs with a 10,000,000 or 15,000,000 KiB virtual-address cap are diagnostic failures, not verification counterexamples. A failed run at the cap establishes only that it did not verify below the target memory budget.

| Only changed part from baseline | R2 verdict | Wall | Peak RSS | Interpretation |
| --- | --- | ---: | ---: | --- |
| Small `i128` Add uses wrapping addition with no overflow fallback (deliberately incorrect for general `i128`) | Verified | 54.28 s | 2.05 GiB | Removing the symbolic overflow/BigInt branch cuts peak RSS by at least 13.15 GiB versus round 1. This scratch variant is not shippable. |
| Guard wrapping addition to operands fitting `i64`; retain checked/BigInt path for all others (exact semantics) | Failed under 15,000,000 KiB address cap, with BigInt unwind checks | 141.04 s | 13.82 GiB | The guard did not prune the expensive path for CBMC. |
| Construct fresh `BigInt`s in the Small overflow fallback, bypassing `OnceBox::as_big` | Failed under 10,000,000 KiB address cap | 120.38 s | 9.23 GiB | Cache construction is not sufficient to explain the cost. |
| Direct `Outcome` instead of `Result<Integer, Stop>` carrier | Failed under 10,000,000 KiB address cap | 129.53 s | 9.23 GiB | Carrier alone is not sufficient. |
| Direct `Outcome` plus `#[repr(u64)]` as in PR #11 | Failed under 10,000,000 KiB address cap | 127.03 s | 9.26 GiB | The combined carrier change is not sufficient. |

## Decision

No production code was changed in round 2. The only sub-8-GiB R2 variant removes exact behavior for inline `i128` overflow. A semantics-preserving local guard did not make that path tractable. The evidence points to the current inline-`i128` representation and its unbounded overflow path as the architectural proof-cost boundary, not the new `Meter::charge` implementation. Integer redesign or a change to FR-371/FR-362 criteria needs an explicit decision.

Options for the owner:

1. Keep `Integer` and the current criteria. Assign a separate bounded proof-engineering investigation to find a sound Kani tractability technique for the overflow branch, then rerun actual CG receipts.
2. Revisit the inline representation or the formal proof obligation in FR-371/FR-362 through spec review, then implement the approved design and rerun the CG receipts.

The real generated CG oracle assertions and all five scalar CG receipts remain unproved on this branch. Round 1 verified charge-only L0–L6, including up to three charges; the full schedule harness with diagnostic-log assertions remains unproved. The requested post-fix R2 and three-charge reruns do not apply because no production fix was made. `make test`, lint, and spec validation were not rerun in this round. The pre-handoff Rust review found no new production diff to review; the WIP branch's earlier changes remain subject to their normal gates.
