#!/usr/bin/env python3
"""Run TC-931's healthy Meter proof and production ALL-loop control.

Only the Python standard library is used. Each proof gets its own systemd scope,
Git worktree and target directory; the checked-out crate is never mutated.
"""

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time
import uuid

MEMORY_BYTES = 16 * 1024**3
TIMEOUT_SECONDS = 600
HARNESS = "accounting::kani_proofs::meter_integer_arithmetic_schedule_matches_reference"
COMMAND = [
    "cargo", "kani", "--harness", HARNESS, "--exact", "--unwind", "3",
    "--solver", "cadical", "--output-format", "regular", "--jobs", "1",
]
CRATE_FILES = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml")


class GateError(Exception):
    """An absent prerequisite or inconclusive/unexpected verification result."""


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def input_bytes(root):
    paths = [root / name for name in CRATE_FILES]
    paths.extend(sorted((root / "src").rglob("*")))
    return {
        str(path.relative_to(root)): path.read_bytes()
        for path in paths if path.is_file()
    }


def git(root, *arguments):
    return subprocess.check_output(
        ["git", "-C", str(root), *arguments], text=True, stderr=subprocess.STDOUT,
    ).strip()


def require_clean(root):
    if git(root, "status", "--porcelain", "--untracked-files=normal"):
        raise GateError("freeze and commit the candidate before running the proof gate")
    return git(root, "rev-parse", "HEAD")


def reject_unrelated_heavy_processes():
    """The native test and its Cargo ancestors belong to this same gate job."""
    ancestors = set()
    current = os.getpid()
    while current > 0:
        ancestors.add(current)
        status = Path(f"/proc/{current}/status").read_text()
        current = int(re.search(r"^PPid:\s+(\d+)$", status, re.MULTILINE)[1])
    heavy = {"cargo", "cargo-kani", "kani-driver", "kani-compiler", "rustc",
             "clippy-driver", "rustfmt", "cbmc", "goto-instrument", "goto-cc"}
    for entry in Path("/proc").iterdir():
        if not entry.name.isdigit() or int(entry.name) in ancestors:
            continue
        try:
            words = (entry / "cmdline").read_bytes().split(b"\0")
            command = Path(os.fsdecode(words[0])).name
            executable = (entry / "exe").resolve()
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            continue
        if command in heavy or ("target" in executable.parts and "deps" in executable.parts):
            raise GateError(f"unrelated heavy process {entry.name} ({command}); reserve the host first")


def mutate_all_loop(root):
    """Wrap the actual request validation in a ten-kind production traversal."""
    path = root / "src/accounting.rs"
    source = path.read_text()
    start_marker = "        for (kind, amount) in charge.sizes.iter() {\n"
    end_marker = "        if let Some((kind, amount)) = first_fail {\n"
    if source.count(start_marker) != 1 or source.count(end_marker) != 1:
        raise GateError("production request loop changed; review the ALL-loop mutation")
    start = source.index(start_marker)
    end = source.index(end_marker, start)
    original = source[start:end]
    if not original.endswith("        }\n"):
        raise GateError("cannot locate the production request-loop boundary")
    nested = "".join("    " + line if line.strip() else line for line in original.splitlines(True))
    nested = nested.replace(
        "            for (kind, amount) in charge.sizes.iter() {\n",
        "            for (kind, amount) in charge.sizes.iter() {\n"
        "                if *kind != requested_kind {\n"
        "                    continue;\n"
        "                }\n",
        1,
    )
    replacement = "        for requested_kind in LimitKind::ALL {\n" + nested + "        }\n"
    path.write_text(source[:start] + replacement + source[end:])
    return source[:start].count("\n") + 1


def classify(log, returncode, mutant_line=None):
    """Fail closed over Kani's regular check records, not its exit code alone."""
    if log.count("Checking harness " + HARNESS + "...") != 1:
        raise GateError("expected exactly one execution of the Meter harness")
    records = re.findall(
        r"^Check (\d+): ([^\n]+)\n[ \t]+- Status: ([A-Z_]+)\n"
        r'[ \t]+- Description: "([^\n]*)"\n(?:[ \t]+- Location: ([^\n]+))?',
        log, re.MULTILINE,
    )
    if not records or [int(row[0]) for row in records] != list(range(1, len(records) + 1)):
        raise GateError("missing or malformed Kani check records")
    if len(re.findall(r"^Check \d+:", log, re.MULTILINE)) != len(records):
        raise GateError("unparsed Kani checks")
    times = re.findall(r"^Verification Time: ([0-9.]+)s$", log, re.MULTILINE)
    if len(times) != 1:
        raise GateError("missing individual verification time")
    statuses = {row[2] for row in records}
    failures = [row for row in records if row[2] == "FAILURE"]
    if mutant_line is None:
        if returncode != 0 or statuses - {"SUCCESS", "UNREACHABLE"}:
            raise GateError("healthy proof did not verify every reachable check")
        unwinds = [row for row in records if ".unwind." in row[1]]
        if not unwinds or any(row[2] != "SUCCESS" for row in unwinds):
            raise GateError("healthy proof lacks successful unwinding checks")
        expected = "Complete - 1 successfully verified harnesses, 0 failures, 1 total."
        verdict = "Verified"
        marker = "VERIFICATION:- SUCCESSFUL"
    else:
        if returncode != 1 or statuses - {"SUCCESS", "UNREACHABLE", "UNDETERMINED", "FAILURE"}:
            raise GateError("ALL-loop control was inconclusive")
        if len(failures) != 1:
            raise GateError("ALL-loop control must fail exactly one assertion")
        _, name, _, description, location = failures[0]
        if (name != "accounting::Meter::charge.unwind.1"
                or description != "unwinding assertion loop 1"
                or not location.startswith(f"src/accounting.rs:{mutant_line}:")
                or not location.endswith(" in function accounting::Meter::charge")):
            raise GateError("control did not fail at the mutated production ALL loop")
        expected = "Complete - 0 successfully verified harnesses, 1 failures, 1 total."
        verdict = "ExpectedProductionUnwindFailure"
        marker = "VERIFICATION:- FAILED"
    if log.count(expected) != 1 or log.count(marker) != 1:
        raise GateError("missing or inconsistent Kani harness verdict")
    return {
        "verdict": verdict,
        "checks": len(records),
        "statuses": {status: sum(row[2] == status for row in records) for status in sorted(statuses)},
        "failed_checks": [row[1] for row in failures],
        "verification_seconds": float(times[0]),
    }


def cgroup_path():
    entries = Path("/proc/self/cgroup").read_text().splitlines()
    unified = [line[3:] for line in entries if line.startswith("0::")]
    if len(unified) != 1:
        raise GateError("a unified cgroup v2 hierarchy is required")
    path = Path("/sys/fs/cgroup") / unified[0].lstrip("/")
    if not path.name.startswith("meter-kani-") or path.suffix != ".scope":
        raise GateError("proof worker must run in its own meter-kani systemd scope")
    if (path / "memory.max").read_text().strip() != str(MEMORY_BYTES):
        raise GateError("proof scope must enforce exactly 16 GiB memory.max")
    if (path / "memory.swap.max").read_text().strip() != "0":
        raise GateError("proof scope must disable swap")
    return path


def memory_record(cgroup):
    return {
        "peak_bytes": int((cgroup / "memory.peak").read_text()),
        "swap_bytes": int((cgroup / "memory.swap.current").read_text()),
        "events": dict(
            (key, int(value)) for key, value in
            (line.split() for line in (cgroup / "memory.events").read_text().splitlines())
        ),
    }


def observe_scope(unit, observation):
    """Sample from the parent, which survives a proof scope's OOM kill."""
    if observation["cgroup"] is None:
        query = subprocess.run(
            ["systemctl", "--user", "show", unit + ".scope", "--property=ControlGroup", "--value"],
            capture_output=True, text=True, check=False,
        )
        relative = query.stdout.strip()
        if query.returncode == 0 and relative.startswith("/"):
            observation["cgroup"] = "/sys/fs/cgroup" + relative
    if observation["cgroup"] is not None:
        try:
            cgroup = Path(observation["cgroup"])
            sample = memory_record(cgroup)
            sample["memory_max"] = (cgroup / "memory.max").read_text().strip()
            sample["swap_max"] = (cgroup / "memory.swap.max").read_text().strip()
            observation["last_cgroup_sample"] = sample
        except OSError:
            pass  # Scope may have been removed; retain the last actual sample.


def finish_observation(unit, observation):
    observe_scope(unit, observation)
    query = subprocess.run(
        ["systemctl", "--user", "show", unit + ".scope", "--property=MemoryPeak",
         "--property=MemoryCurrent", "--property=MemorySwapCurrent", "--property=Result"],
        capture_output=True, text=True, check=False,
    )
    observation["systemd_final"] = {
        "returncode": query.returncode,
        "properties": dict(line.split("=", 1) for line in query.stdout.splitlines() if "=" in line),
        "stderr": query.stderr.strip(),
    }
    peak = observation["systemd_final"]["properties"].get("MemoryPeak", "")
    # UINT64_MAX is systemd's unavailable/infinity sentinel, not a measured peak.
    observation["final_peak_bytes"] = (
        int(peak) if query.returncode == 0 and peak.isdigit() and int(peak) < 2**64 - 1 else None
    )
    sample = observation["last_cgroup_sample"]
    observation["sampled_peak_lower_bound_bytes"] = sample["peak_bytes"] if sample else None
    observation["final_peak_unavailable"] = observation["final_peak_bytes"] is None


def proof_worker(args):
    report = {"accepted": False, "command": COMMAND, "timeout_seconds": TIMEOUT_SECONDS}
    try:
        cgroup = cgroup_path()  # Verify enforced caps before invoking Cargo or Kani.
        reject_unrelated_heavy_processes()
        report.update(cgroup=str(cgroup), memory_max_bytes=MEMORY_BYTES, swap_max_bytes=0)
        initial = memory_record(cgroup)
        if any(initial["events"].values()) or initial["swap_bytes"]:
            raise GateError("proof scope has pre-existing memory events or swap")
        # Discard ambient compiler/Kani flags and toolchain overrides. The fixture's
        # committed toolchain and lockfile, fixed command and private target own this run.
        allowed = ("PATH", "HOME", "USER", "LOGNAME", "CARGO_HOME", "RUSTUP_HOME")
        env = {key: os.environ[key] for key in allowed if key in os.environ}
        env.update(CARGO_TARGET_DIR=str(args.fixture / "target"), CARGO_BUILD_JOBS="1",
                   RUST_TEST_THREADS="1", RAYON_NUM_THREADS="1", CARGO_NET_OFFLINE="true",
                   CARGO_TERM_COLOR="never", TERM="dumb")
        version = subprocess.run(["cargo", "kani", "--version"], cwd=args.fixture,
                                 env=env, capture_output=True, text=True, check=True)
        report["versions"] = version.stdout.strip()
        if not re.search(r"^Kani Rust Verifier \S+", version.stdout, re.MULTILINE) or not re.search(
                r"^CBMC \S+", version.stdout, re.MULTILINE):
            raise GateError("Kani did not report both installed Kani and CBMC versions")
        before = input_bytes(args.fixture)
        expected_status = git(args.fixture, "status", "--porcelain", "--untracked-files=normal")
        expected_patch = git(args.fixture, "diff", "--binary")
        started = time.monotonic()
        with (args.report / "kani.log").open("w") as output:
            process = subprocess.Popen(COMMAND, cwd=args.fixture, env=env,
                                       stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
            try:
                process.wait(timeout=TIMEOUT_SECONDS)
            except BaseException:
                # All backend/compiler children share this process group. A timeout
                # or interrupted wait must not leave a live CBMC behind.
                os.killpg(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()
                raise
        report.update(wall_seconds=time.monotonic() - started, returncode=process.returncode)
        if (before != input_bytes(args.fixture)
                or expected_status != git(args.fixture, "status", "--porcelain", "--untracked-files=normal")
                or expected_patch != git(args.fixture, "diff", "--binary")):
            raise GateError("Kani changed its source inputs or lockfile")
        report["memory"] = memory_record(cgroup)
        if (report["memory"]["peak_bytes"] > MEMORY_BYTES or report["memory"]["swap_bytes"]
                or any(report["memory"]["events"].values())):
            raise GateError("proof encountered memory pressure or exhaustion")
        report.update(classify((args.report / "kani.log").read_text(), process.returncode,
                               args.mutant_line))
        report["accepted"] = True
    except (GateError, OSError, subprocess.SubprocessError) as error:
        report["error"] = str(error)
    finally:
        if "cgroup" in report:
            report["memory"] = memory_record(Path(report["cgroup"]))
        write_json(args.report / "result.json", report)
    return 0 if report["accepted"] else 1


def run_gate(args):
    root = Path(__file__).resolve().parent.parent
    args.report.mkdir(parents=True, exist_ok=False)
    original = input_bytes(root)
    report = {"accepted": False, "proofs": {}}
    owned_worktrees = []
    try:
        commit = require_clean(root)
        reject_unrelated_heavy_processes()
        if shutil.disk_usage(root).free < 20 * 1024**3:
            raise GateError("at least 20 GiB free disk is required")
        # Each input is a registered detached worktree of this exact commit.
        # Persistent reports survive removal of the private inputs and targets.
        temporary = tempfile.mkdtemp(prefix="meter-kani-", dir=root / "target")
        try:
            for name in ("healthy", "all-loop"):
                fixture = Path(temporary) / name
                git(root, "worktree", "add", "--detach", str(fixture), commit)
                owned_worktrees.append(fixture)
                if require_clean(fixture) != commit:
                    raise GateError("proof input does not match the frozen candidate")
                mutant_line = mutate_all_loop(fixture) if name == "all-loop" else None
                lane = args.report / name
                lane.mkdir()
                (lane / "source.patch").write_text(git(fixture, "diff", "--", "src/accounting.rs") + "\n")
                unit = "meter-kani-" + uuid.uuid4().hex
                command = [
                    "systemd-run", "--user", "--scope", "--quiet", "--unit=" + unit,
                    "-p", "MemoryMax=" + str(MEMORY_BYTES), "-p", "MemorySwapMax=0",
                    sys.executable, str(Path(__file__).resolve()), "--worker",
                    "--fixture", str(fixture), "--report", str(lane),
                ]
                if mutant_line is not None:
                    command.extend(["--mutant-line", str(mutant_line)])
                observation = {"unit": unit + ".scope", "cgroup": None, "last_cgroup_sample": None}
                with (lane / "scope.log").open("w") as output:
                    process = subprocess.Popen(command, stdout=output, stderr=subprocess.STDOUT)
                    try:
                        while process.poll() is None:
                            observe_scope(unit, observation)
                            try:
                                process.wait(timeout=0.05)
                            except subprocess.TimeoutExpired:
                                pass
                        returncode = process.returncode
                    except BaseException:
                        # Terminate this invocation's whole proof scope before its
                        # private worktree can be removed underneath a live backend.
                        subprocess.run(["systemctl", "--user", "kill", "--signal=TERM", unit + ".scope"],
                                       stdout=output, stderr=subprocess.STDOUT, check=False)
                        try:
                            process.wait(timeout=30)
                        except subprocess.TimeoutExpired:
                            subprocess.run(["systemctl", "--user", "kill", "--signal=KILL", unit + ".scope"],
                                           stdout=output, stderr=subprocess.STDOUT, check=False)
                            process.wait()
                        raise
                    finally:
                        finish_observation(unit, observation)
                        write_json(lane / "resource-observation.json", observation)
                report["proofs"][name] = {"accepted": False, "resource_observation": observation}
                result_path = lane / "result.json"
                if not result_path.is_file():
                    raise GateError(f"{name}: no worker result; inspect scope.log (scope exit {returncode})")
                report["proofs"][name].update(json.loads(result_path.read_text()))
                if returncode or not report["proofs"][name]["accepted"]:
                    raise GateError(f"{name}: proof/control failed; inspect result.json and kani.log")
        finally:
            for fixture in reversed(owned_worktrees):
                # Only paths registered by this invocation may be force-removed.
                git(root, "worktree", "remove", "--force", str(fixture))
            Path(temporary).rmdir()
        if require_clean(root) != commit:
            raise GateError("candidate commit changed during the gate")
        report["accepted"] = True
    except (GateError, OSError, subprocess.SubprocessError, KeyboardInterrupt) as error:
        report["error"] = str(error) or "interrupted"
    finally:
        if input_bytes(root) != original:
            report.update(accepted=False, error="checked-out crate inputs changed during the gate")
        write_json(args.report / "result.json", report)
    print(f"Meter Kani gate: {'PASS' if report['accepted'] else 'FAIL'}; report: {args.report}")
    return 0 if report["accepted"] else 1


def interrupted(_signum, _frame):
    raise KeyboardInterrupt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", type=Path)
    parser.add_argument("--preflight", action="store_true", help="check clean inputs and exclusive host before Cargo")
    parser.add_argument("--worker", action="store_true", help=argparse.SUPPRESS)
    parser.add_argument("--fixture", type=Path, help=argparse.SUPPRESS)
    parser.add_argument("--mutant-line", type=int, help=argparse.SUPPRESS)
    args = parser.parse_args()
    # Make TERM take the same child/worktree cleanup path as an interrupted wait.
    signal.signal(signal.SIGTERM, interrupted)
    if args.preflight:
        try:
            require_clean(Path(__file__).resolve().parent.parent)
            reject_unrelated_heavy_processes()
        except (GateError, OSError, subprocess.SubprocessError) as error:
            print(str(error), file=sys.stderr)
            return 1
        return 0
    if args.report is None:
        parser.error("--report is required")
    args.report = args.report.resolve()
    if args.worker:
        if args.fixture is None:
            parser.error("worker requires --fixture")
        return proof_worker(args)
    (Path(__file__).resolve().parent.parent / "target").mkdir(exist_ok=True)
    return run_gate(args)


if __name__ == "__main__":
    sys.exit(main())
