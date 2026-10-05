---
id: TC-292
title: "Kernel Value::Population carries PopulationId only, with no model dependency"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-089
    type: verifies
---
# TC-292: Kernel Value::Population carries PopulationId only, with no model dependency

## Description

Verify FR-089-AC-2 by inspection: the kernel `Value::Population` variant's payload type is `PopulationId`, and no file under `src/` can import a population binding or any other caller's model type, because the crate's `Cargo.toml` names no crate that owns one. This is a structural fact about the dependency direction (every edge points into `quire-exact`, never out of it), not a runtime behaviour, so it is an inspection, not a test.

It catches an implementation that moves a population binding into the kernel instead of adding an opaque identity, which a test that only checks that `Value` admits a population value cannot tell from the correct shape.

Scope: FR-089-AC-2.

## Test Procedure

1. Locate the `Value` enum in `src/value.rs` and read its `Population` variant's payload type.
2. Read the `[dependencies]` table of `Cargo.toml` and confirm that it names no crate that owns a population binding or any other caller's model type.

## Expected Results

- Step 1 shows `Value::Population(PopulationId)`, never a binding or a reproduction of its fields defined locally.
- Step 2 finds only the numeric, text and error crates (`num-*`, `thiserror`, `unicode-normalization`).

## Status

Inspected.
