---
id: SR-4921
title: QSL-500 frozen PR17 gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact@a15c465344b29503b0bbaad85dfdccf3f3fb319c; src/equality.rs,
  src/key.rs, src/lib.rs, src/value.rs, tests/union.rs; QSL-500; public QSL consumer
  criteria at b24dbda01d56843cd14d8cab9233ec3cff67535f
review_set: subset
---

## Summary

Reviewed the five-file frozen kernel companion PR17 diff for QSL-500. Useful public kernel controls exist, but the findings below bind them to the wrong caller boundary.

## Verdict

**CONDITIONAL** — trace tags need correction; no production defect was found by the scoped static review. Runtime gates remain unexecuted by this reviewer. The repository-wide gap claim remains FAIL because five unchanged baseline criteria are untagged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The IEEE payload test tags the S3 ill_typed/operator-ineligible criterion but only exercises trusted kernel runtime CanonicalKeyUnavailable and sequence storage; deleting S3 eligibility would not make this test fail. | tests/union.rs:362 |

## Review Evidence

The reviewed SHA and detached checkout remain frozen. Code-review and its Rust lane ran as one method; gap-analysis ran separately. No spec diff exists, so spec-review and all spec sub-analyses were skipped. No optional whole-repository semantic fanout was requested. This is a PR-diff review, not acceptance of the complete QSL-500/SV-502/SV-503/SV-504 capability.

The public caller requirements were read directly from committed git objects at agent-ix/quire-spec-language@b24dbda01d56843cd14d8cab9233ec3cff67535f. QSpec FR-144 and FR-441 were inspected read-only at the private specification baseline b1da9c84396e776411ccd1474e549f54fbdd4c2e. Their private text and source excerpts are intentionally absent from this publicly committable artifact; only identifiers and the public consumer requirements are retained.

No cargo, build, lint, test, Kani, replay or mutation execution occurred: IR-496 owns the shared host locks. The author's remote gate/mutation receipts are external claims, not reviewer execution evidence. Static adversarial checks followed reversed digest/identifier order, swapped payload positions, distinct declarations and kinds, same-member arity disagreement, unsupported IEEE keys, stopped deferred arguments, exact occurrence counts, shared Arcs and destructive worklist draining. Source-backed controls can detect these changes, but no mutant was executed locally.

No new unsafe, compatibility layer, vendored file, panic on a checked caller path, integer boundary cast, async/locking surface, source/test stub, or gate weakening was found in the diff. The 31-cause carrier and IR-678 tests are byte-unchanged against main; no refreshed proof of their runtime behavior is asserted. The kernel explicitly trusts the caller's admitted member binding and type shape. SV must still verify union membership, identifier/key binding, package identity, references, keyed-type eligibility and transport; those responsibilities are not hidden kernel defects.

Tools: /home/peter/.local/bin/quoin 0.28.3; /home/peter/.local/bin/quire 0.36.2 (engine 0.50.2). The skill shorthand quoin write --types SpecReview failed because it omitted the repo argument; quoin write with the exact repo path returned the installed schema/skeleton contract. Known DuplicateArchetype/ DuplicateInverseEdge first-wins diagnostics and the inline Standard advisory were observed; successful structural validation does not mean warning-clean. Configured model/native session id unavailable. Actual harness identity /root/qsl500_exact_pr17_reviewer.

## Coverage

Plan completion: not assessed

Reconciliation: quire matrix --scope /home/peter/dev/worktrees/exact-qsl500-pr17-review --format json; quire 0.36.2 engine 0.50.2; no run evidence read. Actual static counts: 78 tagged, 5 untagged, 9 method-without-symbol. The untagged baseline criteria remain FR-362-AC-10 and FR-369-AC-3/-4/-8/-9. They are unchanged outside this PR, with preserved prior SR-4881 dispositions; this review cannot issue a repository-wide gap PASS. FR-362-AC-19 remains tagged to the unchanged IR-678 test. Local computed matrix reads only the exact spec root; caller-owned FR-319/321/322/323 criteria were measured manually from the exact public QSL git revision. The local matrix does not independently reconcile those external criteria.

PR reverse-gap inventory: admitted member binding, positional union construction/evaluation, Composite admission, occurrence accounting, member/payload equality, member/payload canonical keys, collection integration, clone/debug/drop; all have public consumer requirement ownership. No changed production stub or orphan behavior found. Fourteen new tests have behavioral assertions; no ignored test or test-only production bypass was added. Optional whole-repository semantic review: skipped. Direct code/test alignment was performed as part of the required Rust review, without subagents.

## Examined Scope

```yaml
scope:
- id: FR-319-AC-2
  path: agent-ix/quire-spec-language/spec/functional/FR-319-key-union-member-and-case-nodes.md
  role: examined
  excerpt: For each member of `Shape`, the `VariantId` computed at check time equals the one computed at argument admission for the same member, and equals that member's FR-441 member key bytes (QSpec FR-441-AC-1's vectors give the expected digests for their inputs). Two unions `A { X }` and `B { X }` give two different `VariantId`s for `X`; an enum `E { X }` gives a third. A `case` over `A` evaluated with a scrutinee whose `VariantId` belongs to `B` or `E` selects no arm (it is refused at admission, FR-321).
- id: FR-319-AC-3
  path: agent-ix/quire-spec-language/spec/functional/FR-319-key-union-member-and-case-nodes.md
  role: examined
  excerpt: '`Shape::Rect(2, 3) = Shape::Rect(2, 3)` is `true`, `Shape::Rect(2, 3) = Shape::Rect(3, 2)` is `false`, and `Shape::Empty = Shape::Circle(0)` is `false`. With `union A { X(Integer) }` and `union B { X(Integer) }`, comparing `A::X(1)` with `B::X(1)` is refused by type checking (QSpec TC-262''s equality cases each give TC-262''s verdict).'
- id: FR-321-AC-1
  path: agent-ix/quire-spec-language/spec/functional/FR-321-admit-supplied-union-values.md
  role: examined
  excerpt: 'Running `area` with a supplied `Shape` argument refuses `invalid_runtime_input`/`wrong-value-kind`, with no evaluation and no charge, for each of: a union value whose key is a union of another package; a `VariantId` that is not a `Shape` member (another union''s, or an enum''s); `Rect` with one payload value; `Circle` with payload `true`. A `Rect(2, 3)` argument is admitted and the run returns 6.'
- id: FR-321-AC-4
  path: agent-ix/quire-spec-language/spec/functional/FR-321-admit-supplied-union-values.md
  role: examined
  excerpt: A supplied `Tree` value 10,000 levels deep is admitted, converted and compared equal to itself under the default run limits without a host stack overflow. With the run's value-occurrence limit set one below the value's occurrence count, admission returns that limit by name with its configured value.
- id: FR-322-AC-2
  path: agent-ix/quire-spec-language/spec/functional/FR-322-evaluate-union-construction-and-case.md
  role: examined
  excerpt: 'Under `CheckMode::Kernel`, with `type Small = Int[0, 3]` and `q` holding `2, 2`: a `case` whose scrutinee is `Shape::Circle(sum<Small>(x in q: x))` returns `Undefined::SumOutOfDomain` and evaluates no arm body, and `Shape::Rect(sum<Small>(x in q: x), f())` returns the same outcome and never calls `f`. An injected scrutinee value whose `VariantId` matches no arm returns `Err(InternalFault)` naming S6a, not a refusal.'
- id: FR-322-AC-3
  path: agent-ix/quire-spec-language/spec/functional/FR-322-evaluate-union-construction-and-case.md
  role: examined
  excerpt: 'Evaluating `Shape::Rect(2, 3)` and `area(Shape::Rect(2, 3))` records one `composite.result-retain` for the construction with `value_occurrences = 3`, and for the `case` exactly the scrutinee''s charges followed by the `Rect` body''s charges, with no charge for arm selection or payload binding and none for the unselected arms; a work budget one unit below the total stops at the last listed point with `incomplete { limit_kind: work_units }`.'
- id: FR-323-AC-1
  path: agent-ix/quire-spec-language/spec/functional/FR-323-key-union-values-in-collections.md
  role: examined
  excerpt: '`Set<Shape>`, `Bag<Shape>` and `OrderedSet<Shape>` are admitted at S3. Building each from `Shape::Rect(2, 3)`, `Shape::Empty`, `Shape::Rect(2, 3)` and `Shape::Circle(1)` gives a set of three elements and a bag of four, each sorted by its FR-144 union key exactly as QSpec''s union-key vectors order them and the same whatever the input order, and an ordered set of the three elements `Rect(2, 3)`, `Empty`, `Circle(1)` in first-occurrence order; from the reversed input the ordered set is `Circle(1)`, `Rect(2, 3)`, `Empty`.'
- id: FR-323-AC-2
  path: agent-ix/quire-spec-language/spec/functional/FR-323-key-union-values-in-collections.md
  role: examined
  excerpt: '`Sequence<Shape>` of the same four values is admitted and keeps all four in input order.'
- id: FR-323-AC-3
  path: agent-ix/quire-spec-language/spec/functional/FR-323-key-union-values-in-collections.md
  role: examined
  excerpt: With `union F { Measured(Float64) }` (an IEEE-bearing payload), `Set<F>` refuses `ill_typed`/`operator-ineligible` at its type reference, and `Sequence<F>` is admitted.
- id: src/value.rs
  path: src/value.rs
  role: examined
  excerpt: UnionMember retains admitted declaration, member key and identifier; UnionValue retains positional payload and occurrence count; union and evaluate_union check positional shape.
- id: src/equality.rs
  path: src/equality.rs
  role: examined
  excerpt: Union pairs with one declaration compare member identity, then all payload positions using the existing iterative occurrence-pair worklist.
- id: src/key.rs
  path: src/key.rs
  role: examined
  excerpt: Union keys compare member identifier ASCII bytes, then positional payload keys using the existing iterative worklist.
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: The crate exports UnionMember, UnionValue, union and evaluate_union.
- id: tests/union.rs
  path: tests/union.rs
  role: examined
  excerpt: Fourteen public-API tests exercise trusted kernel seams; they do not exercise package admission, SV registry eligibility or v2 transport.
- id: AGENTS.md
  path: AGENTS.md
  role: context_only
  excerpt: See CLAUDE.md.
- id: CLAUDE.md
  path: CLAUDE.md
  role: context_only
  excerpt: 'The exact-value kernel: checked identity, outcome and refusal, provenance and bound value types, with no dependency on any other crate in the quire ecosystem.'
- id: src/outcome.rs
  path: src/outcome.rs
  role: context_only
  excerpt: The closed CheckedInvariantCause enum retains 31 variants; this file is unchanged from the reviewed main baseline.
- id: tests/ir653_scalar.rs
  path: tests/ir653_scalar.rs
  role: context_only
  excerpt: public_integer_arithmetic_admits_exact_bits_and_denies_one_under is unchanged from the reviewed main baseline.
- id: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  path: spec/functional/FR-262-deep-values-and-types-in-the-kernel.md
  role: context_only
  excerpt: The quire-exact kernel SHALL handle a Value and a ValueType of any depth without growing the native stack with the depth, in clone, equality, hash, debug format and drop.
bindings:
- test_id: ieee_union_payload_has_no_key_and_sequence_keeps_occurrences
  ac_id: FR-323-AC-3
  trace: wrong
- test_id: union_equality_compares_members_and_payload_positions
  ac_id: FR-319-AC-3
  trace: correct
  extent: kernel subset only; caller admission, S3 and transport remain unverified
- test_id: union_keys_order_identifier_ascii_then_payload_positions
  ac_id: FR-323-AC-1
  trace: correct
  extent: kernel subset only; caller admission, S3 and transport remain unverified
- test_id: union_collections_sort_deduplicate_and_preserve_first_occurrence
  ac_id: FR-323-AC-2
  trace: correct
  extent: kernel subset only; caller admission, S3 and transport remain unverified
- test_id: ten_thousand_union_levels_clone_compare_format_and_drop_on_small_stack
  ac_id: FR-321-AC-4
  trace: correct
  extent: kernel subset only; caller admission, S3 and transport remain unverified
```
