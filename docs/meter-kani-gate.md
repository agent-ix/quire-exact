# Meter proof gate

`make kani-meter-gate` runs the real standalone Meter proof and its production
`LimitKind::ALL` trip-count mutation control. Its ignored integration test is the
executable FR-359-AC-9 binding; it invokes the gate and asserts its result. The
Kani proof itself stays untagged. This lane is separate from `make ci`.

Run from a clean, committed checkout on a Linux host with cgroup v2, a systemd
user manager with memory-controller delegation, Python 3, and at least 20 GiB
free disk. Install Kani and its backend first (`cargo kani --version` must report
both Kani and CBMC), and provision the committed Rust toolchain and locked Cargo
dependencies in the local cache. Proof workers use offline Cargo resolution.
The regular-output parser is grounded in Kani 0.68.0 / CBMC 6.11.0 output and
fails closed if the check records or verdict format change.

Reserve the shared host's proof slot before starting. The runner rejects other
visible Cargo, Rust compiler, Kani and CBMC jobs, while allowing its own native
test and Cargo ancestors. This is an additional preflight check; operators must
still serialize work across lanes and avoid starting another job during a run.

```sh
make kani-meter-gate
```

The Make target starts the native test in a 16 GiB, zero-swap systemd scope and
checks its enforced limits before starting Cargo. The test runs the healthy
proof followed by the ALL-loop control, each in a fresh 16 GiB, zero-swap scope.
Each worker verifies its actual `memory.max` and `memory.swap.max` before any
Cargo invocation, with one build job, one proof job and one Rayon thread. Each
proof command has a 600-second wall timeout, followed by TERM and a ten-second
KILL grace period for its process group. The fixed proof command is:

```sh
cargo kani --harness accounting::kani_proofs::meter_integer_arithmetic_schedule_matches_reference --exact --unwind 3 --solver cadical --output-format regular --jobs 1
```

The runner creates two private detached Git worktrees from the candidate's exact
commit, each with a separate target directory. Only the mutant worktree changes:
it wraps the production request-validation loop in `for requested_kind in
LimitKind::ALL`, processing requests matching that kind. Both run the unchanged
six-attempt harness, independent model, full operand/selector/limit/request
amount domains, public `Meter::charge` calls and arithmetic `exact_size` path.
No production `cfg(kani)` branch or harness substitution is installed.

A healthy run must return success with every reachable check and every
unwinding check successful. The mutation must return Kani's failure code with
exactly one failing check, `accounting::Meter::charge.unwind.1`, located at the
inserted production ALL loop. A model assertion failure, different loop failure,
compiler error, missing checks, timeout, memory event or exhausted resource does
not pass the gate. The expensive refusal-order counterexample observation is
already recorded in TC-931 and is not repeated by this persistent lane.

Reports live under `target/meter-kani-reports/<run>/`. Keep the top-level
`result.json` and both proof directories when archiving a receipt. They contain
the Git commit, input hashes, temporary mutation patch, actual installed
Kani/CBMC versions, command, verdict and check counts, individual verification
and wall times, per-proof cgroup peak bytes, swap and memory events, plus the
full Kani and scope logs. Reports from a failed run remain for diagnosis. Private
worktrees and their targets are removed after success, failure or a handled
interrupt; the original checkout and its existing target are never mutated or
cleaned. SIGKILL or host loss can require removing the invocation's registered
worktrees with `git worktree remove --force <reported-private-path>` after
confirming no worker remains.

For the documented scheduled lane, the kernel maintainer runs this command
weekly on the reserved proof host and before accepting changes to Meter
accounting or its proof. Run against the committed candidate, retain the report
with that revision's review evidence, and treat any nonzero exit as a failed
scheduled gate requiring investigation. Provision the environment and reserve
the host before each scheduled invocation. This repository installs no timer or
hosted CI workflow; the maintainer's scheduler should invoke the same Make
target without changing proof arguments or resource limits.
