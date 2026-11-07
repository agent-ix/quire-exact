---
id: SR-202611071713001
title: "Code review of quire-exact PR 136 reservation-charged integer entries"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; PR 136 head 1220f681c4241d61fd62e16e05a8205cef4f2468 diff against origin/main a08e1b2: src/integer.rs, src/integer/reserved.rs, src/reservation.rs, src/lib.rs, tests/ir1404_exact_reservation.rs, spec FR-396, TC-943"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-396
    type: reviews
---

## Summary

Ticket: IR-1404. PR: quire-exact#136. This is a code review with the
rust-review lane folded in. It covers the local `Reservation` trait, the five
reserved entries (`parse_reserved`, `add_reserved`, `mul_reserved`,
`exact_div_reserved`, `gcd_reserved`), the routing of the unmetered entries
through them with `Unlimited`, the allocation-free `Ord`, and the cloneable
`MagnitudeLimbs`.

Evidence:

- `cargo test --release --test ir1404_exact_reservation` passes (7 tests).
- `cargo clippy --all-targets` is clean.
- A throwaway differential test (removed after the run) compared `add`, `mul`,
  `gcd`, `cmp`, `exact_div` and `FromStr` with num-bigint on 20,000 random pairs
  of up to 9 limbs. Both storage forms were used, and limbs were biased to 0,
  1, 0x8000_0000 and u32::MAX so the Knuth D correction steps run. There were
  no mismatches.
- The same harness timed the unmetered entries against num-bigint, in release
  mode, with operands built by `from_big`:

| digits | mul (num-bigint / PR) | exact_div | gcd | parse |
| --- | --- | --- | --- | --- |
| 300 | 1.18 / 5.58 us | 2.97 / 3.63 us | 36 / 49 us | 0.97 / 1.03 us |
| 3,000 | 36 / 521 us | 124 / 541 us | 4.6 / 10.4 ms | 21 / 167 us |
| 30,000 | 2.97 / 73.0 ms | 2.81 / 18.5 ms | 102 / 180 ms | 1.27 / 3.61 ms |

Answers to the brief's questions:

- (1) See SR-202611071713002 for the call-site inventory. This PR meets the
  FR-038 owner-prerequisite list exactly: parse, add, mul, exact division,
  gcd, plus allocation-free comparison and sign tests. The read path needs
  no `sub`, `neg` or `abs`. Reducing a value is gcd followed by exact_div,
  and both are covered.
- (2) Unmetered callers are slower: mul 4.7x to 25x, exact_div 1.2x to 6.6x,
  gcd 1.3x to 2.3x, parse up to 8x (FND-001). `Rational::add`, `mul`, `cmp`
  and `reduce` in this crate, which back QSL exact_probabilistic, RT and CG,
  all route through these entries. A Rational step is gcd-dominated, so it
  runs about 1.4x to 2.3x slower at 300 to 3,000 digits. Keeping num-bigint
  for the unmetered entries while sharing only the representation would be
  two arithmetic implementations. That contradicts FR-396 behaviour 5 and the
  one-core directive, so it is not the remedy. The remedy is to speed up the
  single implementation (FND-001).
- (3) No public behaviour changes. `Debug` and `Display` print through
  `as_big` and give the same text. `Hash` hashes `as_big`, so equal values in
  either storage form hash equally. Equality across forms agreed with
  num-bigint in the differential run. `copy_footprint` reads `limbs.len()`,
  and `from_limbs` trims, so it equals the old `iter_u32_digits().len()`. The
  crate has no serde impl. One side effect does change: every large result is
  now `LargeDigits`, so `as_big` builds a second, retained copy of the value
  (FND-002).
- (4) The parse formula is sound. log2(10) = 3.3219 < 10/3, so n digits fit
  in ceil(10n/3) bits. The `used` index never reaches `len`, because the
  prefix value is below 10^k < 2^(32*len). The per-chunk carry stays below
  2^32, since scale <= 10^9 < 2^30. `checked_mul(10)` guards overflow. At the
  first heap value (39 digits, 2^127) the block is 5 limbs for a 4-limb value.
  The charge equals the allocation, so the accounting is right; the surplus
  capacity is retained.

## Verdict

**PASS with findings**: no high findings, one medium, one low.

What is sound:

- Every limb block is charged at its full capacity before `try_reserve_exact`.
  A denial returns before the request and drops the blocks already held.
- The block formulas match FR-396 behaviour 2.
- Knuth D normalization, the q-hat bound and the add-back step are correct.
- Stein gcd keeps its result inside the first operand's capacity.
- `compare_magnitudes` relies on trimmed limbs, and every `LargeDigits`
  constructor guarantees that.
- `MagnitudeLimbs::clone` restarts a BigInt digit iterator correctly.
- No unsafe code, and the reserved entries cannot panic.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The unmetered `add`, `mul`, `exact_div`, `gcd` and `FromStr` now run the reserved code, which uses schoolbook multiplication over the `MagnitudeLimbs` enum iterator and an index-checked Knuth D. num-bigint uses Karatsuba and Toom-3 above 32 limbs. Measured in release mode: mul 4.7x slower at 300 digits (below the Karatsuba threshold, so this is per-limb overhead) and 25x at 30,000 digits; exact_div 6.6x and parse 2.8x to 8x slower at 3,000 to 30,000 digits. Every `Rational` operation in this crate goes through these entries, so existing QSL exact_probabilistic, RT and CG rational paths get slower with no change on their side. Fix: run the inner loops over slices (copy a `Large` operand's limbs once, or read `LargeDigits` directly), and if large operands matter, add Karatsuba with a reserved scratch block. Do not restore a second num-bigint path. | src/integer/reserved.rs:199-227, src/integer/reserved.rs:237-282, src/integer/reserved.rs:506-545, src/integer.rs:808-810, src/integer.rs:837-839 |
| FND-002 | low | Results of `FromStr`, `add`, `mul`, `gcd` and `exact_div` outside i128 used to be `Large(BigInt)`. They are now `LargeDigits`, so every `as_big`-based entry builds a full `BigInt` image on first use and retains it for the value's lifetime, outside any reservation. Those entries are `is_even`, `sub`, `neg`, `abs`, `div_rem_*`, `pow`, `Display`, `Debug`, `Hash`, `to_u64` and `decimal_digits`. `Rational::sub` calls `Integer::neg` and so takes this path. Resident memory doubles for long-lived large values. On the IR read path, `is_even` on a parsed multi-limb dimension exponent would be an unmetered allocation, against FR-038-AC-516. The rustdoc on `as_big` documents this, but `is_even` could read the low limb directly instead of building the image. | src/integer.rs:801-806, src/integer.rs:814-835, src/integer.rs:842-856, src/integer.rs:1184-1197, src/integer/reserved.rs:121-142 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | `from_big_normalized` now keeps both the `BigInt` it was given (as the cached image) and a fresh `to_u32_digits` limb copy. On main a value outside i128 from `from_big`, `sub`, `neg`, `abs`, `pow`, `div_rem_*`, `div_mod_floor`, `shifted_left`, `split_factor_two` or `power_of_ten` held one `BigInt`. It now holds two full copies, and each construction does an extra O(n) unmetered allocation. Resident memory doubles for long-lived values built this way, for example a pow result or a QSL value from `from_big`. Fix: drop the image after taking the limbs and let `as_big` rebuild it on demand, or document the trade on `from_big` | src/integer.rs:201-212, src/integer.rs:1110-1112 |

## Dispositions

Round 1, fix commit 998fe5bcaa3fcdb817006d35faab466b5d7c332b. Checked as follows:

- The 11 PR tests pass.
- Clippy is clean.
- A throwaway reviewer differential test was run on this head and then removed. It covered 20,000 random small pairs and both storage forms, all limb-length pairs over {1..5, 31-33, 63-66, 127-130, 200, 255-257, 511, 513, 700}, Karatsuba balanced and unbalanced splits, runs of 2^k and 2^k-1 up to k=20,000, and add, mul, gcd, cmp, is_even, exact_div and a product divided back by a factor. Every case matched num-bigint.
- On a separate allocation_counter check, charged equals allocated for Karatsuba products including the scratch block (64x64 up to 700x700, and 700x130 and 300x64 unbalanced). A provision one byte short is denied with exactly the admitted bytes allocated.

`IntegerRepr` is private (`pub struct Integer(IntegerRepr)`), so removing the `Large` variant changes no public API.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Rewritten as one implementation with slice loops on 64-bit digits, Karatsuba from 64 limbs with one charged scratch block, and Knuth D in base 2^64. The PR's own perf test on this head reads quire/num-bigint: mul 0.89 / 0.97 / 1.29 at 300 / 3,000 / 30,000 digits, exact_div 1.36 / 1.01 / 2.48, parse 1.11 / 0.81 / 0.77, gcd 0.87 / 1.5 at 300 / 3,000 over three runs on a loaded host. That is within FR-396 behaviour 5's 1.5x claim, and no second num-bigint path was added |
| FND-002 | fixed | `is_even` reads the low limb of the magnitude slice and no longer builds the image (src/integer.rs:738-743). FR-396-AC-6 now covers it with a zero-allocation test. The other `as_big`-based entries remain documented on `as_big`, as the finding allowed |
