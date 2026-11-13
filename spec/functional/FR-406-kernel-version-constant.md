---
id: FR-406
title: "The kernel exports a public KERNEL_VERSION constant that is bumped when operation semantics change"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: references
---
# FR-406: The kernel exports a public KERNEL_VERSION constant that is bumped when operation semantics change

## Description

The kernel SHALL export `quire_exact::KERNEL_VERSION`, a public `u32` constant naming the semantic epoch of the kernel's operations. The crate version in `Cargo.toml` is `0.1.0` on every commit of a `branch = main` dependency and so cannot tell a consumer that the kernel's behaviour changed; this constant can (Behavior 1 to 6).

The consumer is Contract CodeGen's measured Kani tractability table (IR-1526, kernel-version half of its stale-row rule). A row records the `KERNEL_VERSION` it was measured against; a row whose recorded value differs from the constant a build exports is stale. Reading the constant, storing it in the table and the staleness comparison are CG's credit and not this requirement's acceptance.

### Chosen rule (documented bump, not a content hash)

The constant is a hand-bumped integer, not a digest of source files. A content hash would change on every refactor and so mark every measured row stale for no semantic reason, and deriving it needs a build script or file tracking; neither carries value. The cost is one line of discipline: whoever changes what a kernel operation returns bumps the constant in the same change. A test over a golden outcome grid makes forgetting fail the build (Behavior 5). The limit is stated honestly in Behavior 6.

## Behavior

1. The kernel SHALL export `KERNEL_VERSION: u32` from the crate root, as a `const` available in a `no_std` build, with a doc comment that states the bump rule of Behavior 3.
2. `KERNEL_VERSION` SHALL be a function of the source only, so two builds of the same source with any feature set export the same value, and it is not derived from the build time, a git revision, the host or `Cargo.toml`'s version.
3. A change that can change the result of any public kernel operation for some input: a returned value, a flag, a refusal or its cause, an outcome kind, or any charge (its point, order, limit kind or amount) SHALL bump `KERNEL_VERSION` by at least one. A change that alters none of these (a refactor, a rename, a new operation or type that no existing operation uses, a performance change, a comment) does not require a bump.
4. `KERNEL_VERSION` SHALL never decrease.
5. The kernel SHALL carry a golden test that evaluates a fixed grid of public operations (at least one case per family: integer, decimal, rational, IEEE, text, quantity, collection, identity, and the meter's charge log) and compares each outcome, with its ordered charges, with a recorded expected outcome stored beside the `KERNEL_VERSION` value those outcomes were recorded at. The test fails if any outcome differs or if the recorded version differs from `KERNEL_VERSION`. Updating the expected outcomes therefore requires editing the recorded version in the same change, and bumping the constant requires re-recording.
6. The golden grid SHALL cover the cases named in the test case, not every input. A semantic change outside the grid is not detected by Behavior 5 and is covered by Behavior 3 alone; a reviewer, not the test, catches it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-406-AC-1 | `quire_exact::KERNEL_VERSION` is a public `u32` const reachable from the crate root in default, `test-support` and `no_std` builds, usable in a const context, and its doc comment states the bump rule. | Test (TC-954) |
| FR-406-AC-2 | The value is identical across two separate builds of the same source and across the feature sets, and no `build.rs`, `env!`, `option_env!` or git lookup contributes to it. | Test (TC-954) |
| FR-406-AC-3 | The golden test evaluates the grid of Behavior 5 and passes on the unchanged source. | Test (TC-954) |
| FR-406-AC-4 | A deliberately altered outcome (a fixture in which one expected outcome, and separately one expected charge, is changed) makes the golden comparison report a difference, and a fixture whose recorded version differs from `KERNEL_VERSION` makes the version check fail, so an outcome change without a matching bump cannot pass. | Test (TC-954) |
| FR-406-AC-5 | The doc comment and the crate docs state that the constant is bumped by the author of a semantic change and name the change classes of Behavior 3 that do and do not require a bump. | Inspection (TC-954) |

## Status

Specified (IR-1532). Not implemented; no Rust accompanies this requirement.

## Dependencies

- No existing version mechanism: `Cargo.toml` is `0.1.0`, `publish = false`, and nothing in `src/` exports a version or identity.
- IR-1526 (Contract CodeGen) consumes the constant in its stale-row rule. CG FR-053 owns the float row that table gates.
