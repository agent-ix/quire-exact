---
id: SR-14220
title: code-review review of IR-810 Meter proof gate
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; Makefile, README.md, docs/meter-kani-gate.md, scripts/meter_kani_gate.py,
  scripts/meter_kani_scope.sh, scripts/test_meter_kani_gate.py, tests/meter_kani_gate.rs,
  spec/functional/FR-359-cumulative-meter-boundary.md, spec/test-cases/TC-931-single-pass-meter-accounting.md
review_set: subset
---

## Summary

Frozen candidate b4a47bf8aec7805d42627561544564cb7fd5773c; base 204fb0c3637947b87d885eaaa325126f8d75016d. Ticket: IR-810. PR: agent-ix/quire-exact#66. Reviewer model: GPT-6; exact deployment SKU is unavailable, so markers use gpt-6. No applicable AssuranceProfile was found. No source edits, commits, pushes, Cargo/native/Kani executions or aggregate gate were performed by this reviewer. Scope excerpts over 600 characters use a verbatim prefix; the complete units were read.

## Verdict

CONDITIONAL — two medium process-lifecycle/exclusivity defects require fixes before acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Timeout/interruption cleanup waits only for the process leader: if it exits on TERM while a backend descendant ignores TERM, the KILL branch is skipped and a live descendant can survive private-worktree removal. The parent scope cleanup makes the analogous leader-only decision. | scripts/meter_kani_gate.py:265-270 |
| FND-002 | medium | The host-exclusivity guard omits the required kani process name and uses caller-controlled argv[0] instead of OS process identity; an unrelated required-name tool can be present without rejection, so the gate can start alongside a competing heavy job. | scripts/meter_kani_gate.py:68-79 |

## Coverage

Independently reclassified both archived raw Kani logs using the frozen production classifier: healthy Verified, 2,565 checks (2,534 SUCCESS, 31 UNREACHABLE), all six unwinds successful, 130.15448 s verification; mutant sole accounting::Meter::charge.unwind.1 failure at production line 862, 1/2,586 failures, 4.272985 s verification. Worker receipts and parent observations agree on memory.max 17,179,869,184, swap.max 0, zero memory events, peaks 8,196,722,688 / 613,203,968 bytes, wall 135.6143 / 9.8273 seconds. Final systemd peaks are explicitly unavailable/null. The native log records one actual ignored test executed and passing in 145.80 seconds. fmt-check/focused locked Clippy logs pass; reviewer reran three lightweight Python operational tests and Bash syntax. Production src, Cargo.toml, Cargo.lock and toolchain byte-match the published base. No CI workflow change, proof/harness replacement, vendored implementation, source hash catalog or tool-version pin machinery was introduced. The internal Git commit is used to check out exact private inputs. Reserved-host weekly maintainer execution is the ticket-authorized documented scheduled option.

Rust review is folded into this artifact; reviewed actual #[test], canonical imported #[trace], meaningful subprocess-result assertion, explicit ignore reason and Make invocation with --ignored --exact. Panic/expect is confined to test setup/assertion; no production Rust change, unsafe, wire conversion, new public type or feature change. Python/Bash automation was reviewed as production operations, including parser rejection cases, private registered worktrees, separate targets, sequential execution, pre-Cargo caps and outside-scope sampling. The no-heavy Python reproduction cleanup-reproduction.py uses the exact worker termination sequence with a leader and TERM-resistant descendant; the leader returned -15 while the descendant remained live (state S). The reproduction finally killed its entire process group. The existing negative tests cover output classification and mutation anchoring, but not this process-lifecycle failure. Require whole-group and whole-scope quiescence, escalating remaining members even after leader exit, before deleting private inputs. Controlled lightweight Python-only probes also demonstrated that a child with literal argv[0]=kani was accepted and a child with OS comm=cargo but harmless-helper argv[0] was accepted. Both helper process groups were explicitly killed and reaped. This independently confirms the missing name and argv identity issue; no real Kani/Cargo/backend was launched. Guard checks should cover the complete required name set using verified OS identity while preserving the actual ancestor exemption.

## Examined Scope

```yaml
scope:
- id: Makefile
  path: Makefile
  role: examined
  excerpt: "# =============================================================================\n# quire-exact\
    \ Makefile\n# =============================================================================\n\nCARGO\
    \ ?= cargo\n# A target with no `std` at all: building for it proves the crate pulls in\n# nothing\
    \ from `std`.\nNO_STD_TARGET ?= thumbv7em-none-eabi\n\n.PHONY: help\nhelp:\n\t@echo \"Available targets:\"\
    \n\t@echo \"  make fmt              - Format with rustfmt\"\n\t@echo \"  make fmt-check        - Verify\
    \ formatting (CI gate)\"\n\t@echo \"  make lint             - Clippy with -D warnings\"\n\t@echo \"\
    \  make test             - cargo te"
- id: README.md
  path: README.md
  role: examined
  excerpt: "# quire-exact\n\nquire-exact is a Rust library of exact values: integers and fractions of\
    \ any\nsize, decimals, text, IEEE floating-point numbers, quantities and collections,\nplus the checked\
    \ identities and result types that go with them.\n\nIt is `no_std` (it needs only `alloc`), contains\
    \ no `unsafe` code, and depends\non no other Quire crate. The layers above it use it for arithmetic\
    \ with no\nrounding error. When an operation has no value (dividing by zero, a result\noutside its\
    \ declared range, a budget that ran out), it returns a closed, named\noutcome.\n\n<picture>\n  <source\
    \ media=\"(prefers-color-sche"
- id: docs/meter-kani-gate.md
  path: docs/meter-kani-gate.md
  role: examined
  excerpt: '# Meter proof gate


    `make kani-meter-gate` runs the real standalone Meter proof and its production

    `LimitKind::ALL` trip-count mutation control. Its ignored integration test is the

    executable FR-359-AC-9 binding; it invokes the gate and asserts its result. The

    Kani proof itself stays untagged. This lane is separate from `make ci`.


    Run from a clean, committed checkout on a Linux host with cgroup v2, a systemd

    user manager with memory-controller delegation, Python 3, and at least 20 GiB

    free disk. Install Kani and its backend first (`cargo kani --version` must report

    both Kani and CBMC), and pr'
- id: scripts/meter_kani_gate.py
  path: scripts/meter_kani_gate.py
  role: examined
  excerpt: "#!/usr/bin/env python3\n\"\"\"Run TC-931's healthy Meter proof and production ALL-loop control.\n\
    \nOnly the Python standard library is used. Each proof gets its own systemd scope,\nGit worktree and\
    \ target directory; the checked-out crate is never mutated.\n\"\"\"\n\nimport argparse\nimport json\n\
    import os\nfrom pathlib import Path\nimport re\nimport shutil\nimport signal\nimport subprocess\n\
    import sys\nimport tempfile\nimport time\nimport uuid\n\nMEMORY_BYTES = 16 * 1024**3\nTIMEOUT_SECONDS\
    \ = 600\nHARNESS = \"accounting::kani_proofs::meter_integer_arithmetic_schedule_matches_reference\"\
    \nCOMMAND = [\n    \"cargo\", \"kani\", \"--"
- id: scripts/meter_kani_scope.sh
  path: scripts/meter_kani_scope.sh
  role: examined
  excerpt: '#!/usr/bin/env bash

    # Enforce the native wrapper''s ceiling before it can start Cargo.

    set -euo pipefail

    cgroup="/sys/fs/cgroup$(awk -F: ''$1 == "0" {print $3}'' /proc/self/cgroup)"

    [[ "$(cat "$cgroup/memory.max")" == 17179869184 ]]

    [[ "$(cat "$cgroup/memory.swap.max")" == 0 ]]

    python3 scripts/meter_kani_gate.py --preflight

    export CARGO_TARGET_DIR="$PWD/target/meter-kani-native"

    export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1

    exec "${1:-cargo}" test --locked --test meter_kani_gate -- --ignored --exact meter_proof_and_production_all_loop_control

    '
- id: scripts/test_meter_kani_gate.py
  path: scripts/test_meter_kani_gate.py
  role: examined
  excerpt: "\"\"\"Focused operational checks for the gate's fail-closed verdict classifier.\n\nThese do\
    \ not bind FR-359-AC-9: the ignored Rust test executes the real proofs.\n\"\"\"\n\nfrom pathlib import\
    \ Path\nimport tempfile\nimport unittest\n\nimport meter_kani_gate as gate\n\n\ndef check(number,\
    \ name, status=\"SUCCESS\", description=\"assertion\", location=None):\n    text = (f\"Check {number}:\
    \ {name}\\n\\t - Status: {status}\\n\"\n            f'\\t - Description: \"{description}\"\\n')\n\
    \    if location is not None:\n        text += f\"\\t - Location: {location}\\n\"\n    return text\
    \ + \"\\n\"\n\n\ndef log(mutant=False):\n    text = \"Checking harn"
- id: tests/meter_kani_gate.rs
  path: tests/meter_kani_gate.rs
  role: examined
  excerpt: "// SPDX-License-Identifier: AGPL-3.0-or-later\n//! Executable binding for the standalone Meter\
    \ proof and its trip-count control.\n\nuse ix_trace_rs::trace;\nuse std::{path::PathBuf, process::Command,\
    \ time::SystemTime};\n\n#[trace(\"FR-359-AC-9\")]\n#[test]\n#[ignore = \"requires Kani, cgroup v2\
    \ and systemd user scopes; run make kani-meter-gate\"]\nfn meter_proof_and_production_all_loop_control()\
    \ {\n    let root = PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"));\n    let stamp = SystemTime::now()\n\
    \        .duration_since(SystemTime::UNIX_EPOCH)\n        .expect(\"clock is after Unix epoch\")\n\
    \        .as_nanos();\n    le"
- id: spec/functional/FR-359-cumulative-meter-boundary.md
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: '---

    id: FR-359

    title: "Cumulative meter boundaries and request-proportional charging"

    type: FR

    relationships: []

    ---

    # FR-359: Cumulative meter boundaries and request-proportional charging


    ## Description


    When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete`
    record for the first unavailable counter without wrapping or changing any consumed counter or admission
    count. The meter SHALL keep semantic-size charge traversal proportional to that charge''s request
    count while preserving refusal order and admitted accounting. A standalone meter proof '
- id: spec/test-cases/TC-931-single-pass-meter-accounting.md
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: "---\nid: TC-931\ntitle: \"Request-proportional meter accounting and bounded integer-arithmetic\
    \ proof\"\ntype: TC\nrelationships:\n  - target: ix://agent-ix/quire-exact/FR-359\n    type: verifies\n\
    ---\n# TC-931: Request-proportional meter accounting and bounded integer-arithmetic proof\n\n## Description\n\
    \nVerify FR-359-AC-8, AC-11 and AC-12 (delivered), AC-9 (IR-844 standalone proof and mutation controls,\
    \ with the IR-810 persistent gate) and record the permanently external AC-10 receipt. Kernel accounting\
    \ and its Kani obligation belong to quire-exact. The five unchanged CG scalar tests are a separate\
    \ consum"
- id: scripts/meter_kani_gate.py::proof_worker
  path: scripts/meter_kani_gate.py
  role: examined
  excerpt: "                os.killpg(process.pid, signal.SIGTERM)\n                try:\n           \
    \         process.wait(timeout=10)\n                except subprocess.TimeoutExpired:\n          \
    \          os.killpg(process.pid, signal.SIGKILL)\n                    process.wait()"
- id: scripts/meter_kani_gate.py::reject_unrelated_heavy_processes
  path: scripts/meter_kani_gate.py
  role: examined
  excerpt: "    heavy = {\"cargo\", \"cargo-kani\", \"kani-driver\", \"kani-compiler\", \"rustc\",\n \
    \            \"clippy-driver\", \"rustfmt\", \"cbmc\", \"goto-instrument\", \"goto-cc\"}"
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum observable through
    `consumed()`; the diagnostic log record is verified under FR-359-AC-12. With `IntegerBits` limit 5
    and requests 7 then 9 for that kind in one charge, refusal reports `next_charge = 7`, the first-inserted
    failing amount; the existing 9 then 7 case alone cannot distinguish maximum coalescing before refusal.
    Simultaneous shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters,
    high-water, admissions, log and a pending injected occurrence. Cancellation and injection precedence,
    c
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production
    `Meter::charge` for the exact three-charge FR-362 schedule (operand charge with at most two semantic-size
    requests, arithmetic charge with one, result-retain charge with none) over the full `i64` operand
    range, plus separate repeated-kind charge controls with at most two requests each. Symbolic `i64`
    operands and symbolic limits including `ResultUnits` cover first-short and simultaneous shortages
    and absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and 600 seconds, the
    basel
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their
    own assertions under their original domains, oracle, completion and counterexample checks, unwind
    3, Cadical, 16 GiB and 600 seconds. Record their individual Kani verdicts as an external IR-705/IR-17
    receipt; no quire-exact test or altered consumer harness substitutes for it.
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and called helpers finds
    no traversal with a trip count greater than that charge's semantic-size request count, including no
    traversal of `LimitKind::ALL` or another independently sized collection. The FR-362 proof schedule
    has at most two requests per charge, while the public `Charge` API retains no request-count ceiling.
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362 schedule contains
    exactly the admitted prefix in charge order, with each record's point and absent-or-present per-admission
    semantic-size maxima matching an independent model. Reordered and repeated requests of the same kind,
    including zero and distinct amounts, coalesce to the per-admission maximum in that record. A refused
    charge appends no record and leaves the existing log unchanged. The named `tc_931_admitted_log_matches_model_ordered_prefix`
    and `tc_931_admitted_log_matches_model_same_kind` tests verify these prope
```
