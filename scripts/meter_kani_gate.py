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
    heavy = {"cargo", "kani", "cargo-kani", "kani-driver", "kani-compiler", "rustc",
             "clippy-driver", "rustfmt", "cbmc", "goto-instrument", "goto-cc"}
    for entry in Path("/proc").iterdir():
        if not entry.name.isdigit() or int(entry.name) in ancestors:
            continue
        try:
            command = (entry / "comm").read_text().strip()
        except (FileNotFoundError, ProcessLookupError):
            continue
        names = {command}
        try:
            executable = (entry / "exe").resolve(strict=True)
            names.add(executable.name)
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            executable = None
        if names & heavy or (executable and "target" in executable.parts and "deps" in executable.parts):
            raise GateError(f"unrelated heavy process {entry.name} ({command}); reserve the host first")


def mutate_all_loop(root):
    """Wrap the actual request validation in a ten-kind production traversal."""
    path = root / "src/accounting.rs"
    source = path.read_text()
    start_marker = "        for (kind, amount) in requests {\n"
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
        "            for (kind, amount) in requests {\n",
        "            for (kind, amount) in requests.clone() {\n"
        "                if kind != requested_kind {\n"
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
        if returncode != 1 or statuses - {"SUCCESS", "UNREACHABLE", "FAILURE"}:
            raise GateError("ALL-loop control was inconclusive")
        if len(failures) != 1:
            raise GateError("ALL-loop control must fail exactly one assertion")
        _, name, _, description, location = failures[0]
        function = name.removesuffix(".unwind.1")
        shared = "accounting::Meter::charge_fields"
        # Bind the exact production function/loop/line and its same named
        # monomorphization in both property and source location.
        if (not name.endswith(".unwind.1")
                or not (function == shared or
                        (function.startswith(shared + "::<") and function.endswith(">")))
                or description != "unwinding assertion loop 1"
                or not location.startswith(f"src/accounting.rs:{mutant_line}:")
                or not location.endswith(" in function " + function)):
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


def live_group_members(group):
    members = []
    for entry in Path("/proc").iterdir():
        if not entry.name.isdigit():
            continue
        try:
            fields = (entry / "stat").read_text().rsplit(")", 1)[1].split()
        except (FileNotFoundError, ProcessLookupError):
            continue
        if int(fields[2]) == group and fields[0] not in {"Z", "X"}:
            members.append(int(entry.name))
    return members


def wait_until_empty(is_empty, seconds):
    deadline = time.monotonic() + seconds
    while not is_empty():
        if time.monotonic() >= deadline:
            return False
        time.sleep(0.02)
    return True


def stop_group(process, grace=10):
    """Escalate surviving group members even when the leader already exited."""
    for signum in (signal.SIGTERM, signal.SIGKILL):
        if not live_group_members(process.pid):
            break
        try:
            os.killpg(process.pid, signum)
        except ProcessLookupError:
            pass
        if wait_until_empty(lambda: not live_group_members(process.pid), grace):
            break
    else:
        raise GateError(f"process group {process.pid} remains live; preserve proof inputs")
    process.wait(timeout=grace)


def wait_group(process, timeout, grace=10):
    try:
        returncode = process.wait(timeout=timeout)
    except BaseException:
        stop_group(process, grace)
        raise
    if live_group_members(process.pid):
        stop_group(process, grace)
        raise GateError("proof leader exited with live descendants")
    return returncode


def scope_empty(unit, observation):
    cgroup = observation["cgroup"]
    if cgroup:
        try:
            events = dict(line.split() for line in (Path(cgroup) / "cgroup.events").read_text().splitlines())
            return events["populated"] == "0"
        except FileNotFoundError:
            pass
    query = subprocess.run(
        ["systemctl", "--user", "show", unit + ".scope", "--property=ControlGroup",
         "--property=LoadState", "--property=ActiveState"],
        capture_output=True, text=True, check=False,
    )
    fields = dict(line.split("=", 1) for line in query.stdout.splitlines() if "=" in line)
    if fields.get("LoadState") == "not-found":
        return True
    relative = fields.get("ControlGroup", "")
    if relative.startswith("/"):
        observation["cgroup"] = "/sys/fs/cgroup" + relative
        try:
            events = dict(line.split() for line in
                          (Path(observation["cgroup"]) / "cgroup.events").read_text().splitlines())
            return events["populated"] == "0"
        except FileNotFoundError:
            return False
    if query.returncode == 0 and fields.get("ActiveState") in {"inactive", "failed"}:
        return True
    raise GateError(f"cannot confirm {unit}.scope is empty; preserve proof inputs")


def stop_scope(unit, observation, grace=10):
    for signum in ("TERM", "KILL"):
        if scope_empty(unit, observation):
            return
        subprocess.run(["systemctl", "--user", "kill", "--signal=" + signum, unit + ".scope"],
                       capture_output=True, check=False)
        if wait_until_empty(lambda: scope_empty(unit, observation), grace):
            return
    raise GateError(f"{unit}.scope remains populated; preserve proof inputs")


def wait_scope(process, unit, observation, timeout=None, grace=10):
    started = time.monotonic()
    try:
        while process.poll() is None:
            observe_scope(unit, observation)
            if timeout is not None and time.monotonic() - started >= timeout:
                raise subprocess.TimeoutExpired(process.args, timeout)
            try:
                process.wait(timeout=0.05)
            except subprocess.TimeoutExpired:
                pass
        if not scope_empty(unit, observation):
            raise GateError("scope leader exited with live descendants")
        return process.returncode
    finally:
        # Stop an interrupted launcher before checking for its scope, including
        # interruption during startup. Its exit never substitutes for quiescence.
        if process.poll() is None:
            process.terminate()
        try:
            process.wait(timeout=grace)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=grace)
        stop_scope(unit, observation, grace)


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
            wait_group(process, TIMEOUT_SECONDS)
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
    unsafe_inputs = set()
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
                unsafe_inputs.add(fixture)
                with (lane / "scope.log").open("w") as output:
                    process = subprocess.Popen(command, stdout=output, stderr=subprocess.STDOUT)
                    try:
                        returncode = wait_scope(process, unit, observation)
                    finally:
                        try:
                            if scope_empty(unit, observation):
                                unsafe_inputs.remove(fixture)
                        except (GateError, OSError) as error:
                            observation["cleanup_error"] = str(error)
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
                if fixture in unsafe_inputs:
                    report.setdefault("preserved_inputs", []).append(str(fixture))
                else:
                    git(root, "worktree", "remove", "--force", str(fixture))
            if not unsafe_inputs:
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
