---
id: SR-17940486709564847047936
title: "IR-1445 PR #160 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact PR #160 at 75efd007, git diff origin/main...HEAD; IR-1445; src/native_text.rs, src/native_text/normalize.rs, src/native_text/normalize/tables.rs, src/native_text/normalize/trips.rs, src/native_text/normalize/trip_tests.rs, src/native_text/kani_proofs.rs, src/lib.rs, tests/ir1501_normalize/*, tests/ir1445_native_text.rs, Makefile"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-401
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-404
    type: reviews
---

## Summary

Ticket: IR-1445. This is the code-review method with the rust-review lane folded in. It covers `normalize_text`, which decodes, decomposes (Hangul is done by arithmetic, everything else by binary search over the tables), reorders by stable bubble passes over D, recomposes in place with the blocked-mark rule and encodes. It also covers the four Kani harnesses, the scanner, the trip counters and the `test-exhaustive` Makefile target.

What the reviewer ran, under `flock /tmp/cargo-slot.lock` with a 24G scope:

- `cargo test --test ir1501_normalize`: 11 passed, 1 ignored.
- `cargo test --lib normalize`: both trip tests passed.
- `cargo test --test ir1445_native_text normalizing`: passed.
- `make test-exhaustive`: the pair sweep passed in 0.78 s release.

Correctness was checked by reading the code:

- Decode refuses overlong forms, surrogates, scalars above U+10FFFF and truncated sequences.
- The reorder swaps only when `after != 0 && before > after`, which is a stable canonical sort.
- The recompose condition `written == starter + 1 || last_class < class` is the UAX #15 blocking rule.
- The step counts are large enough for their tables: 12 for 2081 and 3849 entries, 9 for 403, 10 for 961.
- The output tail stays zero.

Nothing allocates, and every refusal is exact.

The Kani OOM was checked against its evidence:

- The four journal scopes run-p3141497, -p3150443, -p3159057 and -p3168251 match the recorded wall times (402.5, 400.7, 429.5 and 426.3 s). Each ends `Failed with result 'oom-kill'` with `24G memory peak`.
- The run logs stop in symbolic execution, inside `class_of` under the `push_decomposed` copy loop, at DECODE iteration 58 or 59 of 64.
- The line numbers in the logs (138, 247, 278, 304, 331) match the committed source.

Clippy was re-run with the module `#[allow]` removed, on a `git archive` copy. Only `needless_range_loop` (1), `manual_range_contains` (6), `manual_is_multiple_of` (1) and `manual_swap` (1) fire. Each of those is a rewrite to a callee that the AC-4 allow-list bans, so those four allows are justified.

`static` rather than `const` for the tables is the better choice for code: one copy, no inlining at each use. It is a spec-wording issue only (see the spec-review SR).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The OOM measures this implementation's shape, not the cost of normalizing at C = 64. The D-bounded COPY loop sits inside the N-bounded DECODE loop, and every copied member calls `class_of`, a 9-step symbolic binary search over the 403-entry CCC tables. The logs show about 2,100 `class_of` instances unwound before the OOM at DECODE slot 58 or 59 of 64. A change the spec allows was not tried: the generator could emit the class of each pool member, so the copy path does no class search, and `class_of` would run only for a scalar with no table entry. That change cuts the symbolic search work by roughly D times. AC-7 and AC-9 are still met (see the spec-review SR). Decision for the lead: try the change once in this PR and re-run at C = 64, or record a follow-up. | src/native_text/normalize.rs:240-256, src/native_text/normalize.rs:206-217, src/native_text/normalize.rs:327-347 |
| FND-002 | low | Two of the six module-level clippy allows are dead. `manual_div_ceil` and `collapsible_if` do not fire when the attribute is removed. The comment justifies all six by the AC-4 scan form, so the dead two read as justified when they are not. Drop them. | src/native_text.rs:23-30 |
| FND-003 | low | The scanner's test-only check on macros is vacuous: `body.contains("cfg(test)") \|\| source.contains("#[cfg(test)]")`. The second clause is always true for this module, because `#[cfg(test)] mod trips;` exists. `macro_rules!` bodies are also removed before any other check, so a construct added outside `cfg(test)` to the body of `trip!` or `entry!` (a `while`, an iterator, an allocation) escapes the scan. Check each macro body alone and scan the parts that are not test-only. | tests/ir1501_normalize/scan.rs:118-130 |
| FND-004 | low | The Hangul clause of the AC-8 pair sweep is dead: `COMPOSE_RIGHT` holds no jamo, so b never falls in 0x1161..0x1176 or 0x11A8..0x11C3. The clause is also wrong if it is ever reached, because it accepts any syllable AC00..=D7A3 with a T jamo, while only LV syllables compose with a T jamo. Hangul composition is covered elsewhere (AC-1 sequence, every scalar alone). Remove the clause or correct it to LV syllables only. | tests/ir1501_normalize/main.rs:500-512 |

## Verdict

Mergeable as code. The normalization is correct against `apply`. The checks are exhaustive per scalar (and per pair in `test-exhaustive`), the tables are generated and compared byte for byte, and the code allocates nothing. The OOM record is genuine and corroborated by the journal and the run logs. FND-001 is the substantive question for the lead. It does not block the AC verdict, but the support decision for CG FR-052's normalizing profiles currently rests on a design whose cost is dominated by an avoidable nested class search. FND-002 to FND-004 are cleanups.
