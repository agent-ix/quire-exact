---
id: FR-406
title: "The kernel exports a public KERNEL_VERSION constant that is bumped when operation semantics change"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: references
---
# FR-406: The kernel exports a public KERNEL_VERSION constant that is bumped when operation semantics change

## Description

The kernel SHALL export `quire_exact::KERNEL_VERSION`, a public `u32` constant naming the semantic epoch of the kernel's operations. The crate version in `Cargo.toml` is `0.1.0` on every commit of a `branch = main` dependency and so cannot tell a consumer that the kernel's behaviour changed; this constant can (Behavior 1 to 6).

The consumer is Contract CodeGen's measured Kani tractability table (IR-1526, kernel-version half of its stale-row rule). A row records the `KERNEL_VERSION` it was measured against; a row whose recorded value differs from the constant a build exports is stale. Reading the constant, storing it in the table and the staleness comparison are CG's credit and not this requirement's acceptance.

### Chosen rule (documented bump, not a content hash)

The constant is a hand-bumped integer, not a digest of source files. A content hash would change on every refactor and so mark every measured row stale for no semantic reason, and deriving it needs a build script or file tracking; neither carries value. The cost is one line of discipline: whoever changes what a kernel operation returns bumps the constant in the same change. A golden outcome grid catches an unintended change inside the grid (Behavior 5); the rest is review discipline, stated in Behavior 6.

## Behavior

1. The kernel SHALL export `KERNEL_VERSION: u32` from the crate root as an integer-literal `const` available in a `no_std` build, with a doc comment that states the bump rule of Behavior 3.
2. `KERNEL_VERSION` SHALL be a function of the source only, so two builds of the same source with any feature set export the same value, and it is not derived from the build time, a git revision or `Cargo.toml`'s version.
3. The kernel's author SHALL bump `KERNEL_VERSION` by at least one in any change that can alter, for some input, the result of a public kernel operation (a returned value, a flag, a refusal or its cause, an outcome kind, or any charge: its point, order, limit kind or amount), or the cost or size of the verification problem that a public operation presents to a Kani harness (CG FR-056 rows record wall time and peak memory for harnesses that run the kernel's `Value`, `Integer`, `Meter`, `build_typed_prestate` and collection types). A semantics-preserving refactor of a carrier representation (FR-402 to FR-405) that changes that cost or size SHALL therefore bump. A change that affects neither (a rename, a comment, a new operation or type that no existing operation uses, a refactor with no effect on results or verification cost) does not require a bump.
4. `KERNEL_VERSION` SHALL never decrease.
5. The kernel SHALL carry a golden test that evaluates a fixed grid of public operations and compares each outcome, with its ordered charges, with a recorded expected outcome. The grid SHALL include integer add, subtract, multiply, negate, non-zero divide and modulo, division by zero, the six comparisons, decimal multiply and rounding at a tie, the native integer and decimal carriers (FR-384, FR-402, FR-403), `build_typed_prestate`, and one case for each of: rational reduction, IEEE add with inexact and overflow flags, text comparison at a prefix, quantity unit mismatch, collection insert past a bound, identity construction, and a meter denied at the exact limit. The test fails when any outcome or charge in the grid differs from the record. It needs the `test-support` feature to read the charge log.
6. The golden test catches an unintended change of an outcome or charge within the grid. It does not detect a change outside the grid, a change to verification cost, or a re-recording of the expected strings without a bump; the reviewer of a change that edits the recorded strings or the verification cost is the enforcement of Behavior 3 in those cases.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-406-AC-1 | `quire_exact::KERNEL_VERSION` is a public `u32` const reachable from the crate root in default, `test-support` and `no_std` builds, defined as an integer literal, usable in a const context, with a doc comment stating the bump rule. | Test (TC-954) |
| FR-406-AC-2 | The golden test evaluates the grid of Behavior 5, with the `test-support` feature enabled, and passes on the unchanged source. | Test (TC-954) |
| FR-406-AC-3 | A fixture in which one expected outcome, and separately one expected charge, is altered makes the golden comparison report a difference. | Test (TC-954) |
| FR-406-AC-4 | The doc comment names the change classes of Behavior 3 that do and do not require a bump, including the verification-cost class. | Inspection (TC-954) |
| FR-406-AC-5 | In the history of a change that alters an operation outcome or charge, a bump of `KERNEL_VERSION` accompanies it, and no change lowers the value. | Inspection (TC-954) |

## Status

Implemented (IR-1532): `KERNEL_VERSION` in `src/lib.rs`, golden grid in `tests/ir1532_kernel_version.rs`. AC-1 to AC-3 are test-backed; AC-4 and AC-5 are checked by inspection at review.

## Dependencies

- No existing version mechanism: `Cargo.toml` is `0.1.0`, `publish = false`, and nothing in `src/` exports a version or identity.
- IR-1526 (Contract CodeGen) consumes the constant in its stale-row rule. CG FR-056 owns the measured rows (Kani wall time and peak memory) that the constant keeps honest.
