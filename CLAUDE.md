# quire-exact

The exact-value kernel: checked identity, outcome and refusal, provenance and bound value types, with no dependency on any other crate in the quire ecosystem.

## Commands

DO NOT RUN: exhausts fixed 16 GiB ceiling, returns Inconclusive MemoryExhausted, proves nothing, blocked work for a week; Peter ordered it never run until all feature work is done. `make kani-meter-gate`, the Python and shell runners, and the ignored native test print this warning; the Kani harness is excluded from compilation. FR-359-AC-9 remains unqualified.

```bash
make fmt            # format with rustfmt
make fmt-check      # verify formatting (CI gate)
make lint           # clippy with -D warnings
make test           # cargo test (default features, then test-support)
make build          # release build
make clean          # cargo clean
make deny           # cargo deny check
make ci             # fmt-check + lint + test + no_std build + deny + docs
```

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` caps cognitive complexity and argument count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `rustfmt.toml` uses 100-char width and `StdExternalCrate` import grouping. CI fails on drift.
- `Cargo.toml` declares the minimum supported Rust version; the no-std gate uses the `thumbv7em-none-eabi` target.

## Layout

```
src/lib.rs             # crate root; unit tests sit beside the code
spec/                  # the requirements this crate owns
```
