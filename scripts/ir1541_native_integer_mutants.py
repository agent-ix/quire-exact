#!/usr/bin/env python3
"""Buildable mutants of FR-409 Behavior 22 / TC-957, host and Kani.

Each mutant is an exact-text edit of a scratch copy of the crate (the working
tree is never touched). A mutant is admitted only if its old text occurs
exactly once and the edit builds.

    python3 scripts/ir1541_native_integer_mutants.py host [mutant ...]
        runs tests/ir1541_native_integer_divide.rs on each mutant; each must fail.
    python3 scripts/ir1541_native_integer_mutants.py kani RESULTS.tsv [mutant:harness ...]
        runs the named harness on the mutant (default: every pair of the
        TC-957 table and of the assigned-mutant table), requires FAILED,
        decodes the concrete playback and replays it on the healthy crate and
        on the mutant through `replay_a_decoded_playback`.

Run Kani only through the authorized runner of ir1541_run_kani.py, one job at a
time.
"""
import importlib.util
import os
import re
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SCRATCH = os.environ.get("IR1541_SCRATCH", "/tmp/ir1541-mutants")
NI = "src/native_integer.rs"
TESTS = "ir1541_native_integer_divide"

spec = importlib.util.spec_from_file_location("runner", os.path.join(ROOT, "scripts", "ir1541_run_kani.py"))
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)

SWAP = "(DivisionSchedule::Division, {}, member),"
SWAP_OLD = "(DivisionSchedule::Division, profile, member),"
MOD_OLD = """            DivisionSchedule::Modulus,
            DivisionProfile::Euclidean,
            DivisionMember::Remainder,
        ),
        _ => return None,"""
MOD_NEW = """            DivisionSchedule::Modulus,
            DivisionProfile::{},
            DivisionMember::Remainder,
        ),
        _ => return None,"""
ZERO_OLD = """    if divisor.0 == 0 {
        return Err(Stop::Undefined(Undefined::DivisionByZero));
    }
    division::charge_arithmetic(schedule, bits, meter)?;
    let (quotient, remainder)"""
GUARD = """    if divisor.0 == 0 {
        return Err(Stop::Undefined(Undefined::DivisionByZero));
    }
"""
M = {}


def swap(frm, to):
    cond = f"if profile == DivisionProfile::{frm} {{ DivisionProfile::{to} }} else {{ profile }}"
    return SWAP.format(cond)


for frm, to in [("Truncating", "Floor"), ("Truncating", "Euclidean"), ("Floor", "Truncating"),
                ("Floor", "Euclidean"), ("Euclidean", "Truncating"), ("Euclidean", "Floor")]:
    M[f"swap-{frm.lower()}-{to.lower()}"] = (NI, SWAP_OLD, swap(frm, to))
M["modulo-truncating"] = (NI, MOD_OLD, MOD_NEW.format("Truncating"))
M["modulo-floor"] = (NI, MOD_OLD, MOD_NEW.format("Floor"))
M["zero-guard-removed"] = (NI, ZERO_OLD, ZERO_OLD.replace(GUARD, ""))
M["zero-guard-moved"] = (NI, ZERO_OLD, ZERO_OLD.replace(GUARD, "").replace(
    "    let (quotient, remainder)", GUARD + "    let (quotient, remainder)"))
M["min-over-minus-one-unguarded"] = (NI, """                if dividend.lower == i128::MIN && divisor.lower <= -1 && divisor.upper >= -1 {
                    return None;
                }
""", "")
M["remainder-capacity-widened"] = (NI, "                (-reach, reach)\n",
                                   "                (-reach.saturating_add(1), reach.saturating_add(1))\n")
M["modulo-capacity-lowered"] = (NI, "Self::Modulo { divisor, .. } => (0, remainder_reach(divisor)),",
                                "Self::Modulo { divisor, .. } => (-1, remainder_reach(divisor)),")
M["result-interval-skipped"] = (NI, "(Some(value), Some(domain)) if domain.contains(NativeInteger(value)) => {",
                                "(Some(value), Some(_)) => {")
M["remainder-returns-quotient"] = (NI, "DivisionMember::Remainder => Some(remainder),",
                                   "DivisionMember::Remainder => quotient,")

HOST_ONLY = {"zero-guard-moved"}

TCS = ["trunc", "floor", "euclid"]
LAW = {"trunc": "truncating", "floor": "floor", "euclid": "euclidean"}


def pairs():
    out = []
    w8 = lambda tag: f"{tag}_sym_w8"
    # TC-957 mutant table.
    for kind in ("quotient", "remainder"):
        for tag, swaps in (("trunc", ["swap-truncating-floor", "swap-truncating-euclidean"]),
                           ("floor", ["swap-floor-truncating", "swap-floor-euclidean"]),
                           ("euclid", ["swap-euclidean-truncating", "swap-euclidean-floor"])):
            for s in swaps:
                out.append((s, w8(f"{tag}_{kind}")))
    out += [("modulo-truncating", w8("modulo")), ("modulo-floor", w8("modulo"))]
    for tag in TCS:
        out.append(("zero-guard-removed", w8(f"{tag}_quotient")))
    out += [("min-over-minus-one-unguarded", "divide_quotient_plan_refuses_min_over_minus_one_and_derives_capacity"),
            ("remainder-capacity-widened", "divide_remainder_plan_derives_capacity_and_never_refuses"),
            ("modulo-capacity-lowered", "modulo_plan_derives_capacity_and_never_refuses"),
            ("result-interval-skipped", w8("trunc_quotient")),
            ("remainder-returns-quotient", w8("trunc_remainder"))]
    # The assigned mutant of every evaluation harness (Behavior 22 table).
    for tag, kind in [(t, k) for t in TCS for k in ("quotient", "remainder")] + [("modulo", None)]:
        name = tag if kind is None else f"{tag}_{kind}"
        if kind is None:
            assigned = "modulo-truncating"
        else:
            assigned = "swap-truncating-floor" if tag == "trunc" else \
                ("swap-floor-truncating" if tag == "floor" else "swap-euclidean-truncating")
        for w in (8, 16, 32, 64, 128):
            out.append((assigned, f"{name}_sym_w{w}"))
        for d in ("7", "neg7"):
            for w in (64, 128):
                out.append((assigned, f"{name}_const_{d}_w{w}"))
        for w in (64, 128):
            out.append(("result-interval-skipped", f"{name}_const_neg1_w{w}"))
    seen, uniq = set(), []
    for p in out:
        if p not in seen:
            seen.add(p)
            uniq.append(p)
    return uniq


def prepare(mutant):
    """A scratch copy with the mutant applied; the edit must land."""
    work = os.path.join(SCRATCH, mutant)
    if os.path.exists(work):
        shutil.rmtree(work)
    os.makedirs(work)
    for entry in ["Cargo.toml", "Cargo.lock", "src", "tests", "clippy.toml", "rustfmt.toml",
                  "rust-toolchain.toml", "spec", "docs", "proofs"]:
        src = os.path.join(ROOT, entry)
        if os.path.exists(src):
            (shutil.copytree if os.path.isdir(src) else shutil.copy)(src, os.path.join(work, entry))
    file, old, new = M[mutant]
    path = os.path.join(work, file)
    text = open(path).read()
    assert text.count(old) == 1, f"{mutant}: old text occurs {text.count(old)} times"
    assert old != new
    open(path, "w").write(text.replace(old, new))
    assert new in open(path).read()
    return work


def host(ids):
    bad = 0
    for mutant in ids or M:
        work = prepare(mutant)
        proc = subprocess.run(["cargo", "test", "--test", TESTS], cwd=work, capture_output=True, text=True)
        out = proc.stdout + proc.stderr
        failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", out, re.M)))
        built = "could not compile" not in out
        killed = proc.returncode != 0 and built and failed
        print(f"{mutant:32} {'KILLED by ' + ','.join(failed) if killed else 'SURVIVED/BUILD-FAIL'}", flush=True)
        bad += not killed
        shutil.rmtree(work, ignore_errors=True)
    sys.exit(1 if bad else 0)


def playback_values(text):
    block = text.split("concrete_vals", 1)[1]
    vals = []
    for m in re.finditer(r"vec!\[\s*([\d,\s]*)\s*\]", block.split("let mut", 1)[0] if "let mut" in block else block):
        nums = [int(x) for x in re.findall(r"\d+", m.group(1))]
        vals.append(nums)
    return vals


def le(bytes_):
    return int.from_bytes(bytes(bytes_), "little", signed=True)


def replay_spec(harness, vals):
    """The IR1541_REPLAY line for a decoded playback."""
    tag = re.match(r"(trunc|floor|euclid|modulo)", harness).group(1)
    if harness.startswith("modulo_plan"):
        d = [le(v) for v in vals[:4]]
        present = vals[4][0] != 0
        r = [le(v) for v in vals[5:7]]
        return f"plan|M|{min(d[0], d[1])}|{max(d[0], d[1])}|{min(d[2], d[3])}|{max(d[2], d[3])}|" \
               f"{int(present)}|{min(r)}|{max(r)}"
    if harness.startswith("divide_"):
        d = [le(v) for v in vals[:4]]
        present = vals[4][0] != 0
        r = [le(v) for v in vals[5:7]]
        kind = "Q" if "quotient" in harness else "R"
        return f"plan|{kind}:Truncating|{min(d[0], d[1])}|{max(d[0], d[1])}|{min(d[2], d[3])}|{max(d[2], d[3])}|" \
               f"{int(present)}|{min(r)}|{max(r)}"
    present = vals[0][0] != 0
    r = [le(vals[1]), le(vals[2])]
    a, b = le(vals[3]), le(vals[4])
    m = re.match(r"(trunc|floor|euclid|modulo)(?:_(quotient|remainder))?_(sym|const)_(?:(neg)?(\d+)_)?w(\d+)", harness)
    _, kind, shape, neg, digits, w = m.groups()
    w = int(w)
    law = {"trunc": "Truncating", "floor": "Floor", "euclid": "Euclidean", "modulo": "Euclidean"}[tag]
    combo = "M" if tag == "modulo" else f"{'Q' if kind == 'quotient' else 'R'}:{law}"
    lo, hi = (-2 ** (w - 1), 2 ** (w - 1) - 1)
    dlo = lo + 1 if (w == 128 and kind == "quotient") else lo
    if shape == "sym":
        vlo, vhi = lo, hi
    else:
        d = int(digits) * (-1 if neg else 1)
        vlo = vhi = d
    return f"eval|{combo}|{a}|{b}|{dlo}|{hi}|{vlo}|{vhi}|{int(present)}|{min(r)}|{max(r)}"


def replay(cwd, spec_line):
    env = dict(os.environ, IR1541_REPLAY=spec_line)
    proc = subprocess.run(["cargo", "test", "--test", TESTS, "replay_a_decoded_playback", "--",
                           "--ignored", "--nocapture"], cwd=cwd, capture_output=True, text=True, env=env)
    out = proc.stdout + proc.stderr
    lines = [l for l in out.splitlines() if l.startswith(("plan=", "exact="))]
    panicked = "panicked" in out
    return lines, panicked


def kani(results, selected):
    for mutant, harness in selected or pairs():
        if mutant in HOST_ONLY:
            continue
        work = prepare(mutant)
        log_root = os.path.dirname(os.path.abspath(results))
        os.makedirs(os.path.join(log_root, "ir1541-logs"), exist_ok=True)
        log = os.path.join(log_root, "ir1541-logs", f"mutant-{mutant}-{harness}.log")
        runner.ROOT = work
        outcome, wall, peak, covers = runner.run(
            harness, 600, log, "-Z concrete-playback --concrete-playback=print")
        row = [mutant, harness, outcome, f"{float(wall):.1f}", peak, "", "", "", ""]
        if outcome == "FAILED":
            text = open(log, errors="replace").read()
            try:
                vals = playback_values(text)
                line = replay_spec(harness, vals)
                healthy, _ = replay(ROOT, line)
                mutated, mpanic = replay(work, line)
                if harness.startswith(("divide_", "modulo_plan")):
                    differs = healthy != mutated
                else:
                    healthy_ok = len(healthy) == 2 and healthy[0].split("=", 1)[1] == healthy[1].split("=", 1)[1]
                    mutant_differs = mpanic or len(mutated) < 2 or mutated[0].split("=", 1)[1] != mutated[1].split("=", 1)[1]
                    differs = healthy_ok and mutant_differs
                row[5:] = [line, " ; ".join(healthy), " ; ".join(mutated) + (" PANIC" if mpanic else ""),
                           "REPLAY-DIFFERS" if differs else "REPLAY-SAME"]
            except Exception as exc:  # noqa: BLE001
                row[5:] = ["", "", "", f"DECODE-ERROR {exc!r}"]
        with open(results, "a") as f:
            f.write("\t".join(row) + "\n")
        print("\t".join(row), flush=True)
        shutil.rmtree(work, ignore_errors=True)


def main():
    mode = sys.argv[1]
    if mode == "host":
        host(sys.argv[2:])
    elif mode == "kani":
        selected = [tuple(a.split(":", 1)) for a in sys.argv[3:]]
        kani(sys.argv[2], selected)
    elif mode == "pairs":
        for p in pairs():
            print(*p)


if __name__ == "__main__":
    main()
