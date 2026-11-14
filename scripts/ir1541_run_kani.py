#!/usr/bin/env python3
"""Run FR-409 Kani harnesses one job at a time and append a results TSV.

Each harness runs in its own memory-limited scope behind the shared cargo
slot lock, under a wall-clock limit, and its cgroup memory.peak is read inside
the scope. Usage:

    python3 scripts/ir1541_run_kani.py RESULTS.tsv TIMEOUT_SECONDS HARNESS...

HARNESS is a harness name under native_integer::division_proofs. The
environment variable IR1541_ROOT selects the crate directory to verify (a
mutant scratch copy); it defaults to the repository.
"""
import os
import re
import subprocess
import sys
import time

ROOT = os.environ.get("IR1541_ROOT", os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
INNER = r"""
cd "$1"
start=$(date +%s.%N)
timeout "$2" cargo kani --harness "native_integer::division_proofs::$3" --exact --jobs 1 --solver cadical $5 > "$4" 2>&1
code=$?
end=$(date +%s.%N)
cg=/sys/fs/cgroup$(cut -d: -f3 /proc/self/cgroup)
echo "EXIT $code" >> "$4"
echo "WALL $(echo "$end - $start" | bc)" >> "$4"
echo "PEAK $(cat $cg/memory.peak 2>/dev/null || echo NA)" >> "$4"
"""


def required_covers(harness):
    """The covers a harness's shape must satisfy (FR-409 Behavior 16)."""
    admission = {"admitted pair reached", "rejected pair reached"}
    if harness.startswith("divide_quotient_plan"):
        return admission | {"MIN / -1 refused", "full-width quotient capacity"}
    if harness.startswith("divide_remainder_plan"):
        return admission | {"full-width remainder capacity"}
    if harness.startswith("modulo_plan"):
        return admission | {"full-width modulo capacity"}
    kind = "quotient" if "quotient" in harness else "remainder"
    if harness.startswith("modulo"):
        kind = "modulo"
    need = {f"completed {kind if kind != 'modulo' else 'remainder'} reached",
            f"{kind} out-of-domain refusal reached"}
    if "_sym_" in harness:
        need.add("zero divisor refusal reached")
    return need


def cover_status(text):
    """description -> status for every cover property in a Kani log."""
    found = {}
    for block in re.split(r"\n(?=Check \d+:)", text):
        if ".cover." not in block.split("\n", 1)[0]:
            continue
        status = re.search(r"Status: (\w+)", block)
        desc = re.search(r'Description: "([^"]*)"', block)
        if status and desc:
            found[desc.group(1)] = status.group(1)
    return found


def run(harness, timeout, log, extra=""):
    cmd = ["flock", "/tmp/cargo-slot.lock", "systemd-run", "--user", "--scope", "-q",
           "-p", "MemoryMax=24G", "-p", "MemorySwapMax=0",
           "bash", "-c", INNER, "ir1541", ROOT, str(timeout), harness, log, extra]
    subprocess.run(cmd, check=False)
    text = open(log, errors="replace").read()
    verdict = re.search(r"VERIFICATION:- (\w+)", text)
    covers = re.search(r"\*\* (\d+) of (\d+) cover properties satisfied", text)
    exit_code = re.search(r"^EXIT (\d+)", text, re.M).group(1)
    wall = re.search(r"^WALL ([\d.]+)", text, re.M).group(1)
    peak = re.search(r"^PEAK (\w+)", text, re.M).group(1)
    if verdict:
        outcome = verdict.group(1)
    elif exit_code == "124":
        outcome = "TIMEOUT"
    elif "out of memory" in text.lower() or exit_code == "137":
        outcome = "OOM"
    else:
        outcome = "NO-VERDICT"
    peak_gib = f"{int(peak) / 2**30:.2f}" if peak.isdigit() else peak
    status = cover_status(text)
    missing = sorted(c for c in required_covers(harness) if status.get(c) != "SATISFIED")
    cover_note = f"{covers.group(1)}/{covers.group(2)}" if covers else "-"
    cover_note += " ok" if not missing else " MISSING:" + "|".join(missing)
    return outcome, wall, peak_gib, cover_note


def main():
    results, timeout, harnesses = sys.argv[1], sys.argv[2], sys.argv[3:]
    logdir = os.path.join(os.path.dirname(os.path.abspath(results)), "ir1541-logs")
    os.makedirs(logdir, exist_ok=True)
    for h in harnesses:
        log = os.path.join(logdir, f"{h}.{int(time.time())}.log")
        outcome, wall, peak, covers = run(h, timeout, log)
        with open(results, "a") as f:
            f.write(f"{h}\t{outcome}\t{float(wall):.1f}\t{peak}\t{covers}\t{log}\n")
        print(h, outcome, wall, peak, covers, flush=True)


if __name__ == "__main__":
    main()
