---
id: SR-16635
title: code-review review of IR-844 Meter Kani proof disablement
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@9c267346025f643856c4d29ae3a9d08e5727569e; CLAUDE.md, Makefile, docs/meter-kani-gate.md, scripts/meter_kani_gate.py, scripts/meter_kani_scope.sh, src/accounting.rs, src/accounting/kani_proofs.rs, tests/meter_kani_gate.rs"
review_set: subset
---

## Summary

Ticket: IR-844. Reviewed quire-exact draft PR #87 at frozen head 9c267346025f643856c4d29ae3a9d08e5727569e, diff `origin/main...HEAD` only (one commit, 8 files). The rust-review lane is folded in, using the rust-style baseline and the repository's `rustfmt.toml` and `clippy.toml`.

The operator ordered the FR-359-AC-9 u64 Meter Kani proof never to run until all feature work is done. That order reached this review through the planner and the dispatch brief. The PR is checked against it here.

All validation is static. The reviewer did not run the Meter Kani proof, `make kani-meter-gate`, any Kani harness, any Cargo build, or `make ci`. A shared build lock is held for CG #592. These checks did run:
- `python3 -m py_compile scripts/meter_kani_gate.py` passed.
- `bash -n scripts/meter_kani_scope.sh` passed.
- `make -n kani-meter-gate` shows only `echo ... >&2; exit 2`.
- `git diff --check` is clean.
- `rustfmt --check --edition 2021` (the crate's edition) is clean on `tests/meter_kani_gate.rs` and `src/accounting.rs`.

Entry points:
- **Source cfg.** `#[cfg(all(kani, any()))]` on `mod kani_proofs` is always false, because the empty `any()` is false. The module is therefore left out of every build, `cargo kani` included. A direct `cargo kani --harness accounting::kani_proofs::meter_integer_arithmetic_schedule_matches_reference --exact` has no harness to select. This is stronger than `#[ignore]`.
- **Other Kani modules.** `units_proofs` and `native_integer::kani_proofs` stay `cfg(kani)`. No other item in `accounting.rs` is gated for the removed module only, so no dead-code warnings appear.
- **`check-cfg`.** `check-cfg` already admits `cfg(kani)`, and the empty `any()` needs no declaration.
- **Make, shell and Python runners.** All three print the warning to stderr and exit 2 before any setup. `meter_kani_gate.py` exits through `sys.exit(main())`, which also covers `--preflight` and `--worker`.
- **Native test.** It stays `#[ignore]` with the warning as its reason, and it panics if selected with `--ignored`.

No entry point can report success. `make test` does not run ignored tests, and `ci.yml` (manual dispatch only) runs no Kani step.

Removing `#[trace("FR-359-AC-9")]` is truthful. The test no longer executes anything, and TC-931 forbids a dummy binding. `quire matrix --scope . --strict` at this head reports FR-359-AC-9 as `untagged`, so the criterion stays visibly unqualified. The AC row in `spec/functional/FR-359-cumulative-meter-boundary.md` is unchanged.

## Verdict

**CHANGES REQUESTED.** The disablement is correct and fails closed at every named entry point, and nothing can report a misleading success. However, two documents outside the diff still say the gate runs and that a tagged executable binding exists:
- the FR-359 evidence paragraph and the TC-931 procedure;
- the README.

The brief requires the docs to describe the disabled state accurately, and these two do not. Two low findings cover stale descriptive comments and the retained proof source, which no build now compiles.

## Examined scope

- `src/accounting.rs` (examined): `#[cfg(all(kani, any()))] mod kani_proofs;` with the DO NOT RUN comment, plus the unchanged `cfg(kani)` `units_proofs` and `cfg(any(test, kani))` `units_reference`.
- `src/accounting/kani_proofs.rs` (examined): DO NOT RUN comment above the untagged `#[kani::proof] fn meter_integer_arithmetic_schedule_matches_reference`.
- `tests/meter_kani_gate.rs` (examined): `#[ignore = "DO NOT RUN: ..."]` test whose body is a `panic!` carrying the warning, with the `#[trace("FR-359-AC-9")]` binding and the gate invocation removed.
- `scripts/meter_kani_gate.py` (examined): `main()` prints the warning and returns 2 before argparse; it runs under `sys.exit(main())`.
- `scripts/meter_kani_scope.sh` (examined): prints the warning and runs `exit 2` before the cgroup checks, preflight and `cargo test`.
- `Makefile` (examined): `kani-meter-gate` echoes the warning to stderr and exits 2; it no longer runs `systemd-run`.
- `docs/meter-kani-gate.md` (examined): rewritten to describe the disabled state and the unqualified AC.
- `CLAUDE.md` (examined): DO NOT RUN paragraph under Commands.
- `FR-359-AC-9` (context_only): The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production `Meter::charge` for the exact three-charge FR-362 schedule ... For the baseline, any `UnwindBoundExhausted`, `MemoryExhausted`, timeout, compile error or missing assertion fails acceptance.
- `README.md` (context_only): line 143 describes the Meter Kani gate.
- `spec/test-cases/TC-931-single-pass-meter-accounting.md` (context_only): step 3 describes the gate and the tagged binding.
- `.github/workflows/ci.yml` (context_only): manual dispatch only, with no Kani step.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-359's evidence paragraph and TC-931 step 3 still say `make kani-meter-gate` runs the ignored test tagged `#[trace("FR-359-AC-9")]`, and they record a passing run, which is false at this head | spec/functional/FR-359-cumulative-meter-boundary.md:71, spec/test-cases/TC-931-single-pass-meter-accounting.md:19 |
| FND-002 | medium | README still says the Meter Kani gate "runs the accounting proof and its production-loop mutation control with `make kani-meter-gate`" | README.md:143 |
| FND-003 | low | Descriptive headers still describe live execution: the test module says "Executable binding for the standalone Meter proof", and the Python docstring says "Run TC-931's healthy Meter proof" | tests/meter_kani_gate.rs:2, scripts/meter_kani_gate.py:2 |
| FND-004 | low | The retained `kani_proofs.rs` now compiles under no cfg at all, so Meter API drift will break it silently until it is re-enabled; nothing records that it must be rebuilt first | src/accounting.rs:24 |
