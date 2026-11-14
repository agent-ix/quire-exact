#!/usr/bin/env python3
"""The FR-409 Behavior 17 to 19 ladder: plan harnesses, then each member
combination's symbolic widths in ascending order (the widths above the first
non-VERIFIED width are recorded as not run), then every constant-divisor
harness. Resumable: harnesses already in RESULTS.tsv are skipped.

    python3 scripts/ir1541_ladder.py RESULTS.tsv
"""
import importlib.util
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
spec = importlib.util.spec_from_file_location("runner", os.path.join(ROOT, "scripts", "ir1541_run_kani.py"))
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)

COMBOS = ["trunc_quotient", "floor_quotient", "euclid_quotient",
          "trunc_remainder", "floor_remainder", "euclid_remainder", "modulo"]
PLANS = ["divide_quotient_plan_refuses_min_over_minus_one_and_derives_capacity",
         "divide_remainder_plan_derives_capacity_and_never_refuses",
         "modulo_plan_derives_capacity_and_never_refuses"]


def main():
    results = sys.argv[1]
    done = {}
    if os.path.exists(results):
        for line in open(results):
            cells = line.rstrip("\n").split("\t")
            done[cells[0]] = cells[1]
    logdir = os.path.join(os.path.dirname(os.path.abspath(results)), "ir1541-logs")
    os.makedirs(logdir, exist_ok=True)

    def record(harness, outcome, wall="", peak="", covers="", log=""):
        with open(results, "a") as f:
            f.write(f"{harness}\t{outcome}\t{wall}\t{peak}\t{covers}\t{log}\n")
        done[harness] = outcome
        print(harness, outcome, wall, peak, covers, flush=True)

    def go(harness):
        if harness in done:
            return done[harness]
        log = os.path.join(logdir, f"{harness}.log")
        outcome, wall, peak, covers = runner.run(harness, 600, log)
        record(harness, outcome, f"{float(wall):.1f}", peak, covers, log)
        return outcome

    for h in PLANS:
        go(h)
    for combo in COMBOS:
        failing = None
        for w in (8, 16, 32, 64, 128):
            h = f"{combo}_sym_w{w}"
            if failing is not None:
                if h not in done:
                    record(h, f"not run, covered by {failing}")
                continue
            if go(h) != "SUCCESSFUL":
                failing = h
    for combo in COMBOS:
        for d in ("7", "neg7", "neg1"):
            for w in (64, 128):
                go(f"{combo}_const_{d}_w{w}")


if __name__ == "__main__":
    main()
