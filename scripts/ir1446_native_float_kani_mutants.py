#!/usr/bin/env python3
"""Re-runnable harness mutants of FR-405-AC-12 / TC-953.

For each VERIFIED Kani harness, a buildable mutant of the native body (one of
the exact-text edits of ir1446_native_float_mutants.py, by id) is built in a
scratch copy and the harness is run on it; the harness must report
`VERIFICATION:- FAILED`. The harness list and the mutant it is checked
against are the table in TC-953 ("Vacuity and harness mutants").

Run one harness at a time under the authorized runner, for example:

    flock /tmp/cargo-slot.lock systemd-run --user --scope \\
      -p MemoryMax=24G -p MemorySwapMax=0 \\
      python3 scripts/ir1446_native_float_kani_mutants.py float32_negate

With no argument, every harness in the TC-953 table is run in turn.
"""
import importlib.util
import os
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
spec = importlib.util.spec_from_file_location(
    "host_mutants", os.path.join(ROOT, "scripts", "ir1446_native_float_mutants.py"))
host = importlib.util.module_from_spec(spec)
spec.loader.exec_module(host)
MUTANTS = {m[0]: m for m in host.M}
TC = os.path.join(ROOT, "spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md")


def table():
    text = open(TC).read().split("### Vacuity and harness mutants")[1].split("\n## ")[0].split("\n### ")[0]
    rows = []
    for line in text.splitlines():
        cells = [c.strip().strip("`") for c in line.strip().strip("|").split("|")]
        if line.startswith("| ") and len(cells) == 4 and cells[0] not in ("Harness id", "---"):
            rows.append(cells)
    return rows


def run(harness, mutant_id):
    harness = harness.removeprefix("native_float_kani_proofs::")
    mid, site, file, old, new, _ = MUTANTS[mutant_id]
    work = tempfile.mkdtemp(prefix=f"ir1446-kani-{harness}-")
    try:
        for entry in ["Cargo.toml", "Cargo.lock", "src", "clippy.toml", "rustfmt.toml"]:
            src = os.path.join(ROOT, entry)
            (shutil.copytree if os.path.isdir(src) else shutil.copy)(src, os.path.join(work, entry))
        path = os.path.join(work, file)
        text = open(path).read()
        assert text.count(old) == 1, f"{mutant_id}: {text.count(old)} occurrences"
        open(path, "w").write(text.replace(old, new))
        proc = subprocess.run(
            ["cargo", "kani", "--harness", f"native_float_kani_proofs::{harness}", "--exact",
             "--jobs", "1", "--solver", "cadical"],
            cwd=work, capture_output=True, text=True, timeout=700)
        verdict = re.search(r"VERIFICATION:- (\w+)", proc.stdout + proc.stderr)
        return verdict.group(1) if verdict else "NO-VERDICT"
    finally:
        shutil.rmtree(work, ignore_errors=True)


def main():
    only = set(sys.argv[1:])
    bad = 0
    for harness, _covers, mutant_id, _verdict in table():
        if only and harness.removeprefix("native_float_kani_proofs::") not in only:
            continue
        verdict = run(harness, mutant_id)
        print(f"{harness:48} {mutant_id:28} {verdict}", flush=True)
        bad += verdict != "FAILED"
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
