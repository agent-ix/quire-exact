# Meter proof gate

`make kani-meter-gate` runs the ignored test `units_kani_proofs_verify`
(`tests/meter_kani_gate.rs`, bound to FR-359-AC-9). The test runs real Kani on
the three `accounting::units_proofs` harnesses with `-Z function-contracts
--exact --unwind 3 --jobs 1 --solver cadical` and requires each to report
`VERIFICATION:- SUCCESSFUL` with every cover satisfied.

The gate is not part of `make ci`. Run one heavy job at a time. A timeout,
memory exhaustion or compilation failure is inconclusive, not a verdict.

The earlier monolithic harness `src/accounting/kani_proofs.rs` stays excluded
from compilation and is not claimed by FR-359-AC-9.

Harness rules that keep the proofs tractable:

- Compare `Incomplete::next_charge` through `Integer::native()`, never
  `to_u64()`: its `LargeDigits` arm owns a `Vec<u32>` that CBMC would execute
  symbolically.
- Reach the common path through `Meter::charge_units` and the private
  `push_units`/`set_units` setters, not the `Integer`-typed `Charge::work`.
- Name each check with `kani::assert(condition, "name")`; a formatted
  `assert!` message is dropped and a mutant failure would not identify its check.
