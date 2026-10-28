#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Check the mathematical enclosure lemma and its substantive controls.

Run in the lead's canonical 16 GiB/no-swap proof scope with an explicit Z3
executable and a fresh output directory. This protocol verifies only the
mathematical lemma. Rust bridge receipts and reviewed composition are required
before applying it to NativeIntegerPlan; it never reports a Kani enclosure PASS.
"""

import argparse
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
PROOFS = ROOT / "proofs/native_integer"
VARIABLES = ("a", "b", "c", "d", "x", "y")
PREMISES = (
    "(<= a b)", "(<= c d)", "(<= a x)",
    "(<= x b)", "(<= c y)", "(<= y d)",
)
SIGNS = {
    "negative": "(< {upper} 0)",
    "nonnegative": "(>= {lower} 0)",
    "straddling": "(and (< {lower} 0) (>= {upper} 0))",
}
MIN32, MAX32 = -(2**31), 2**31 - 1


def integer_literal(value):
    return str(value) if value >= 0 else f"(- {-value})"


def digest(path):
    result = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            result.update(block)
    return result.hexdigest()


def outside(product="(* x y)", omit_upper_bd=False):
    corners = ("(* a c)", "(* a d)", "(* b c)", "(* b d)")
    lower = " ".join(f"(< {product} {corner})" for corner in corners)
    upper = " ".join(f"(> {product} {corner})" for corner in corners[:3 if omit_upper_bd else 4])
    return f"(or (and {lower}) (and {upper}))"


def formula(extra=(), omitted=(), predicate=None, model=False):
    # SAT controls use the full signed32 integer population embedded in Real.
    # The healthy theorem above has no is_int or signed32 restrictions.
    lines = ["(set-logic ALL)"]
    lines.extend(f"(declare-const {name} Real)" for name in VARIABLES)
    lines.extend(f"(assert {term})" for term in PREMISES if term not in omitted)
    for name in VARIABLES:
        lines.append(f"(assert (and (is_int {name}) (<= {integer_literal(MIN32)} {name}) (<= {name} {MAX32})))")
    lines.extend(f"(assert {term})" for term in extra)
    if predicate is not None:
        lines.append(f"(assert {predicate})")
    lines.append("(check-sat)")
    if model:
        lines.append("(get-value (a b c d x y))")
    return "\n".join(lines) + "\n"


def sexpressions(text):
    text = re.sub(r";[^\n]*", "", text)
    tokens = re.findall(r"\(|\)|[^\s()]+", text)
    if len(tokens) > 4096:
        raise ValueError("unexpectedly large model")
    cursor = 0

    def take(depth=0):
        nonlocal cursor
        if depth > 32 or cursor >= len(tokens):
            raise ValueError("malformed model")
        token = tokens[cursor]
        cursor += 1
        if token == "(":
            result = []
            while cursor < len(tokens) and tokens[cursor] != ")":
                result.append(take(depth + 1))
            if cursor >= len(tokens):
                raise ValueError("unclosed model expression")
            cursor += 1
            return result
        if token == ")":
            raise ValueError("unexpected model parenthesis")
        return token

    values = []
    while cursor < len(tokens):
        values.append(take())
    return values


def validate_theorem_schema(text):
    expected = [["set-logic", "QF_NRA"]]
    expected.extend(["declare-const", name, "Real"] for name in VARIABLES)
    expected.extend(["assert", sexpressions(term)[0]] for term in PREMISES)
    expected.extend([["assert", sexpressions(outside())[0]], ["check-sat"]])
    if sexpressions(text) != expected:
        raise ValueError("healthy theorem must have exactly the declared six premises and four-corner negation; controls must use the same premises")


def number(value):
    if isinstance(value, str):
        return Fraction(value)
    if len(value) == 2 and value[0] == "-":
        return -number(value[1])
    if len(value) == 3 and value[0] == "/":
        return number(value[1]) / number(value[2])
    raise ValueError(f"unsupported non-rational model value: {value!r}")


def decode_model(text):
    forms = sexpressions(text)
    if len(forms) != 1 or not isinstance(forms[0], list):
        raise ValueError("expected one get-value response")
    values = {}
    for pair in forms[0]:
        if not isinstance(pair, list) or len(pair) != 2 or pair[0] in values:
            raise ValueError("unexpected model binding")
        values[pair[0]] = number(pair[1])
    if set(values) != set(VARIABLES):
        raise ValueError("model must bind exactly the six theorem variables")
    if any(value.denominator != 1 or not MIN32 <= value <= MAX32 for value in values.values()):
        raise ValueError("control witness is not in the original signed32 integer population")
    return values


def observations(values):
    a, b, c, d, x, y = (values[name] for name in VARIABLES)
    corners = (a * c, a * d, b * c, b * d)
    return {
        "premises": (a <= b, c <= d, a <= x, x <= b, c <= y, y <= d),
        "corners": corners,
        "product": x * y,
        "sum": x + y,
        "low": min(corners),
        "high": max(corners),
    }


def sign(lower, upper):
    if upper < 0:
        return "negative"
    return "nonnegative" if lower >= 0 else "straddling"


def validate_control(kind, values, expected=None):
    obs = observations(values)
    if kind == "missing-membership":
        if obs["premises"][2] or not all(value for index, value in enumerate(obs["premises"]) if index != 2):
            raise ValueError("missing-premise mutant did not expose the omitted lower membership")
        if obs["low"] <= obs["product"] <= obs["high"]:
            raise ValueError("missing-premise model does not falsify enclosure")
    else:
        if not all(obs["premises"]):
            raise ValueError("countermodel violates the original declared premises")
        if not obs["low"] <= obs["product"] <= obs["high"]:
            raise ValueError("exact mathematical product itself escaped its correct image")
        if kind == "weakened-corner":
            if not obs["product"] > max(obs["corners"][:3]):
                raise ValueError("omitting the fourth corner did not change the bound")
        elif kind == "arithmetic-mutant":
            if obs["low"] <= obs["sum"] <= obs["high"]:
                raise ValueError("Add-for-Multiply mutant did not falsify enclosure")
        elif kind == "sign-case":
            actual = (sign(values["a"], values["b"]), sign(values["c"], values["d"]))
            if actual != expected:
                raise ValueError("sign-case witness belongs to a different case")
        elif kind == "extreme":
            if obs["product"] != expected["product"] or any(values[name] != expected[name] for name in VARIABLES):
                raise ValueError("extreme witness has the wrong full interval, payload or exact product")
    return {
        "values": {name: str(value) for name, value in values.items()},
        "product": str(obs["product"]), "sum": str(obs["sum"]),
        "corners": [str(value) for value in obs["corners"]],
        "low": str(obs["low"]), "high": str(obs["high"]),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--solver", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--timeout", type=float, default=600)
    args = parser.parse_args()
    if not 0 < args.timeout <= 600:
        parser.error("protocol timeout must be positive and at most 600 seconds")
    output = args.output_dir.resolve()
    output.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    cgroup_line = next(line for line in Path("/proc/self/cgroup").read_text().splitlines() if line.startswith("0:"))
    cgroup = Path("/sys/fs/cgroup" + cgroup_line.split(":", 2)[2])
    caps = {name: (cgroup / name).read_text().strip() for name in ("memory.max", "memory.swap.max")}
    if caps != {"memory.max": "17179869184", "memory.swap.max": "0"}:
        raise RuntimeError("run this protocol in its canonical 16 GiB/no-swap proof scope")
    if subprocess.check_output(["git", "status", "--porcelain"], cwd=ROOT):
        raise RuntimeError("the reviewed proof candidate must be committed and clean")
    solver = args.solver.resolve(strict=True)
    version = subprocess.run([str(solver), "--version"], capture_output=True, text=True, timeout=10, check=True)
    (output / "solver-version.txt").write_text(version.stdout + version.stderr)
    subjects = [
        "src/native_integer.rs", "src/native_integer/kani_proofs.rs", "src/native_integer/reference.rs",
        "Cargo.toml", "Cargo.lock", "scripts/native_integer_enclosure.py",
        "proofs/native_integer/interval_enclosure.smt2", "proofs/native_integer/integer_real_embedding.smt2",
        "proofs/native_integer/composition.json",
    ]
    provenance = {
        "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "tree": subprocess.check_output(["git", "rev-parse", "HEAD^{tree}"], cwd=ROOT, text=True).strip(),
        "solver": str(solver), "solver_sha256": digest(solver), "caps": caps,
        "subjects": {name: digest(ROOT / name) for name in subjects},
    }
    (output / "provenance.json").write_text(json.dumps(provenance, indent=2) + "\n")
    results = []
    theorem = (PROOFS / "interval_enclosure.smt2").read_text()
    validate_theorem_schema(theorem)

    def run(name, text, expected, kind=None, witness_expected=None):
        case = output / name
        case.mkdir()
        (case / "query.smt2").write_text(text)
        command = [str(solver), "-in"]
        (case / "argv.json").write_text(json.dumps(command) + "\n")
        remaining = args.timeout - (time.monotonic() - started)
        if remaining <= 0:
            raise TimeoutError("whole SMT protocol exceeded its 600-second ceiling")
        before = time.monotonic()
        try:
            process = subprocess.run(command, input=text, capture_output=True, text=True, timeout=remaining)
        except subprocess.TimeoutExpired as error:
            def decoded(value):
                return value.decode(errors="replace") if isinstance(value, bytes) else (value or "")
            (case / "stdout.txt").write_text(decoded(error.stdout))
            (case / "stderr.txt").write_text(decoded(error.stderr))
            (case / "result.json").write_text(json.dumps({"name": name, "status": "TIMEOUT", "acceptance": False}) + "\n")
            raise
        (case / "stdout.txt").write_text(process.stdout)
        (case / "stderr.txt").write_text(process.stderr)
        lines = process.stdout.strip().splitlines()
        if process.returncode != 0 or process.stderr.strip() or not lines or lines[0] != expected:
            raise RuntimeError(f"{name}: expected {expected}; exit={process.returncode}; stdout={process.stdout!r}; stderr={process.stderr!r}")
        if expected == "unsat" and len(lines) != 1:
            raise RuntimeError("an UNSAT response must contain no extra result or error")
        record = {"name": name, "expected": expected, "actual": lines[0], "exit": process.returncode,
                  "elapsed_seconds": time.monotonic() - before, "query_sha256": digest(case / "query.smt2")}
        if expected == "sat":
            record["witness"] = validate_control(kind, decode_model("\n".join(lines[1:])), witness_expected)
        (case / "result.json").write_text(json.dumps(record, indent=2) + "\n")
        results.append(record)
        print(f"{name}: {expected}", flush=True)

    run("healthy-real-enclosure", theorem, "unsat")
    run("integer-real-embedding", (PROOFS / "integer_real_embedding.smt2").read_text(), "unsat")
    run("nonvacuity", formula(model=True), "sat", "nonvacuity")
    for left, left_term in SIGNS.items():
        for right, right_term in SIGNS.items():
            conditions = (left_term.format(lower="a", upper="b"), right_term.format(lower="c", upper="d"))
            run(f"sign-{left}-{right}", formula(conditions, model=True), "sat", "sign-case", (left, right))
    for name, x, y, product in (
        ("min-squared", MIN32, MIN32, 2**62),
        ("negative-extreme", MIN32, MAX32, -(2**62) + 2**31),
        ("zero", 0, MIN32, 0),
    ):
        equalities = tuple(f"(= {variable} {integer_literal(value)})" for variable, value in zip(VARIABLES, (MIN32, MAX32, MIN32, MAX32, x, y)))
        expected = dict(zip(VARIABLES, (MIN32, MAX32, MIN32, MAX32, x, y)))
        expected["product"] = product
        run("extreme-" + name, formula(equalities, model=True), "sat", "extreme", expected)
    run("contradictory-membership", formula(("(< x a)",)), "unsat")
    run("contradictory-sign-case", formula(("(< b 0)", "(>= a 0)")), "unsat")
    run("weakened-corner-mutant", formula(predicate=outside(omit_upper_bd=True), model=True), "sat", "weakened-corner")
    run("arithmetic-mutant", formula(predicate=outside("(+ x y)"), model=True), "sat", "arithmetic-mutant")
    run("missing-membership-mutant", formula(omitted=("(<= a x)",), predicate=outside(), model=True), "sat", "missing-membership")
    report = {
        "method": "standalone mathematical SMT enclosure with explicit Rust bridge obligations",
        "mathematical_protocol_passed": True,
        "native_enclosure_qualified": False,
        "native_qualification_requires": "all source-bound Rust bridges and independently reviewed composition.json",
        "kani_C2_verified": False,
        "elapsed_seconds": time.monotonic() - started, "results": results, "provenance": provenance,
    }
    (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print("Mathematical protocol passed; native composition remains a separate reviewed obligation.", flush=True)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, TimeoutError) as error:
        print(f"SMT protocol inconclusive/failed: {error}", file=sys.stderr)
        sys.exit(1)
