---
id: SR-2438
title: "Code review of quire-exact PR #10: QSL-639 Arc-shared nested value types"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@9ea9b679e10e63ec7fabc9d042afd3fbaa9bff02; PR #10 diff vs origin/main: src/value.rs, src/value/value_type.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-262
    type: reviews
---
# Code review of quire-exact PR #10: QSL-639 Arc-shared nested value types

## Summary

Ticket: QSL-639. PR: quire-exact#10, head `9ea9b679e10e63ec7fabc9d042afd3fbaa9bff02`. Reviewer model `claude-opus-5-5`, run `9367b1b3-0e93-4a8a-8a58-801a174e21eb`. Code review with its Rust lane (`rust-review`) folded in.

The change moves the payloads of `ValueType::Option` and `ValueType::Collection` from `Box` to `Arc`. `Clone` becomes an `Arc` clone. `Drop` detaches the chain with `Arc::get_mut`, and `PartialEq`, `Hash` and `Debug` keep their iterative walks. Two tests are added in `src/value.rs`: a node count at N = 1,000 against 2N = 2,000, and a 100,000-deep value with types as deep as the value, run on a 512 KiB thread.

Drop was checked by hand against three cases. In the unique chain, every `get_mut` succeeds and the loop takes each link apart with a leaf below it. In the shared top, `get_mut` fails at once, the field drop only decrements, and the remaining holder detaches the chain later. In the shared middle, the loop stops at the first shared link, which is released by decrement, and its last holder later detaches the rest iteratively, either through its own `ValueType::drop` or through `Arc`'s drop glue running `ValueType::drop` on the freed inner type. No `Arc` cycle can form, because a link is mutated only through `get_mut` (the handle is unique) and is otherwise immutable, so shared links cannot leak. The crate stays `#![forbid(unsafe_code)]`. There is no `Box` alias, no shim and no deprecated constructor: the variants change type in place and `option()`/`collection()` keep their signatures.

The N-against-2N oracle discriminates. With the old `Box` payloads, each level's `payload_type.clone()` copied the chain below it, so `type_nodes` would count about N²/2 distinct allocations (about 500,000 at N = 1,000 and 2,000,000 at 2N). `large <= 2 * small + 2` and the deep test's `type_nodes(&value) <= 2 * DEEP` both fail under that behaviour. `small >= 990` stops the count being vacuous.

## Verdict

**Mergeable with two low findings.** The Arc sharing is correct, Drop stays iterative for the unique and shared cases a single thread can produce, nothing leaks, and the memory test would fail on the old representation. The deep test's drop phase is ordered so that it never drops a long unique chain (FND-001). That path is still covered by the existing `tc_735_a_deep_option_type_clones_compares_hashes_formats_and_drops` (copy dropped last, unique 100,000-link chain) and by the mixed chain dropped at the end of each loop iteration in `tc_735_the_debug_worklist_grows_by_a_constant_per_type_node`. The `get_mut`-based Drop departs from the crate's own `Arc::into_inner` drop idiom in ways only concurrent drops or an external `Weak` can reach (FND-002). Neither finding blocks the batch. Gates: see `## Gates`.

## Gates

`make ci` on this head through `locked-build.sh` (fmt-check, clippy -D warnings, test default and test-support, no_std build, deny, docs): green on 9ea9b67, exit 0: fmt-check, clippy -D warnings (default, test-support, thumbv7em), cargo test 111+45 and test-support 112+47 passed with 0 failed (both new tests and all TC-735 tests pass), no_std build, cargo deny ok.

## Coverage

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| src/value/value_type.rs Drop | examined | src/value/value_type.rs:186-196 | `let mut next = self.take_unshared_child(); while let Some(mut link) = next { next = link.take_unshared_child(); }` |
| src/value/value_type.rs take_unshared_child | examined | src/value/value_type.rs:45-68 | `Self::Option(payload) => Arc::get_mut(payload).map(take_type)` |
| src/value/value_type.rs Clone | examined | src/value/value_type.rs:134-155 | `Self::Option(payload) => Self::Option(Arc::clone(payload))` |
| src/value.rs ValueType variants | examined | src/value.rs:196-200 | `Option(Arc<ValueType>)`, `Collection(Arc<CollectionType>)` |
| type_nodes_grow_linearly_with_value_depth | examined | src/value.rs:2161-2168 | `assert!(small >= 990, "{small}"); assert!(large <= 2 * small + 2, ...)` |
| a_deep_value_with_deep_types_is_admitted_in_linear_memory | examined | src/value.rs:2173-2199 | built, admitted, cloned, compared, hashed, formatted and dropped on a 512 KiB stack |
| FR-262-AC-2 | context_only | spec/functional/FR-262-deep-values-and-types-in-the-kernel.md | On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s around `Boolean` clones, compares equal to its clone, hashes equal to its clone, formats for debug and drops. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The drop phase of `a_deep_value_with_deep_types_is_admitted_in_linear_memory` passes even with plain derived drop glue, so it does not test the iterative Drop it claims to. It drops `value_type`, then `clone`, then `value`. `value_type` and `clone` share the top link, so the first drop only decrements. `clone` then owns the top link alone, but the link below it is shared with the outermost value level's `payload_type`, so the loop stops after one link. `drop(value)` then releases one type node per value level, because each level's chain is shared with the next level's. No step ever drops a long uniquely owned chain, so a recursive `Drop` would also pass. Unique-chain drop at depth is still covered by tc_735 (which drops the copy last) and by the mixed chain dropped in the debug-worklist test. What stays untested is the case this PR adds: a deep chain shared in the middle whose last holder must detach the rest without recursion. Fix: drop `value` first, then `value_type`, then `clone`. The last drop then detaches a mixed 100,000-link chain whose lower links were shared until a moment before. | src/value.rs:2195-2197 |
| FND-002 | low | `Drop` decides ownership with `Arc::get_mut`. The crate's own value drop uses `Arc::into_inner` and states why at src/value.rs:356-359: "`Arc::into_inner` decides that atomically: exactly one dropping handle receives the node." `get_mut` is not atomic across handles, and it also fails whenever a `Weak` exists. Scenario 1: two threads drop the last two handles to a shared link at the same time. Both see strong = 2 and stop. The handle that decrements last frees the link through `Arc`'s drop glue, which runs `ValueType::drop` one frame set deeper. Each race lost further down the chain adds another frame set. Scenario 2: the payload `Arc` is public, so a consumer can `Arc::downgrade` it. A chain with a `Weak` on every link fails `get_mut` at strong = 1 at every level and drops recursively, O(depth) frames. The doc comment at value_type.rs:187-189 ("the stack stays a fixed few frames deep") claims more than the code guarantees in either scenario. A fix without `unsafe` is not one line, because an `Arc` cannot be moved out of a field of a type that implements `Drop` without a replacement allocation. Accepting this with a narrowed doc comment is reasonable. | src/value/value_type.rs:49-55, 186-196 |

## Dispositions

Round 1, reviewed at `7a7ed05534354d89e2f6c6d9112046f5a2c1fc46`.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 899a7f8 |
| FND-002 | fixed | 899a7f8 |
