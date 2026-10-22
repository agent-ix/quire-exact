"""Focused operational checks for the gate's fail-closed verdict classifier.

These do not bind FR-359-AC-9: the ignored Rust test executes the real proofs.
"""

from pathlib import Path
import tempfile
import unittest

import meter_kani_gate as gate


def check(number, name, status="SUCCESS", description="assertion", location=None):
    text = (f"Check {number}: {name}\n\t - Status: {status}\n"
            f'\t - Description: "{description}"\n')
    if location is not None:
        text += f"\t - Location: {location}\n"
    return text + "\n"


def log(mutant=False):
    text = "Checking harness " + gate.HARNESS + "...\n"
    if mutant:
        text += check(1, gate.MUTANT_FUNCTION + ".unwind.0", "FAILURE",
                      "unwinding assertion loop 0",
                      "src/accounting/units.rs:101:9 in function " + gate.MUTANT_FUNCTION)
        text += "VERIFICATION:- FAILED\n"
        text += "Complete - 0 successfully verified harnesses, 1 failures, 1 total.\n"
    else:
        text += check(1, "pointer_dereference.1")  # Some Kani records lack locations.
        text += check(2, "accounting::Meter::charge.unwind.0", description="unwinding assertion loop 0")
        text += "VERIFICATION:- SUCCESSFUL\n"
        text += "Complete - 1 successfully verified harnesses, 0 failures, 1 total.\n"
    return text + "Verification Time: 4.296773s\n"


class VerdictTests(unittest.TestCase):
    def test_healthy_requires_complete_successful_checks_and_unwind(self):
        self.assertEqual(gate.classify(log(), 0)["verdict"], "Verified")
        for corrupted in (
            log().replace("Check 2:", "Check 3:"),
            log().replace("Status: SUCCESS", "Status: FAILURE", 1),
            log().replace("Status: SUCCESS", "Status: UNDETERMINED", 1),
            log().replace("accounting::Meter::charge.unwind.0", "ordinary.assertion.1"),
            log().replace("Check 2:", "Check missing:"),
            log().replace("Verification Time: 4.296773s\n", ""),
            log().replace("1 successfully verified harnesses", "0 successfully verified harnesses"),
        ):
            with self.subTest(log=corrupted), self.assertRaises(gate.GateError):
                gate.classify(corrupted, 0)
        for returncode in (1, 124, 137, -9):
            with self.subTest(returncode=returncode), self.assertRaises(gate.GateError):
                gate.classify(log(), returncode)

    def test_only_actual_production_all_loop_unwind_satisfies_control(self):
        self.assertEqual(gate.classify(log(True), 1, 101)["verdict"],
                         "ExpectedProductionUnwindFailure")
        name = gate.MUTANT_FUNCTION + ".unwind.0"
        for corrupted in (
            log(True).replace(name, "kani_proofs::step.assertion.1"),
            log(True).replace(name, gate.MUTANT_FUNCTION + ".unwind.1"),
            log(True).replace(name, "accounting::Meter::charge_general.unwind.0"),
            log(True).replace("src/accounting/units.rs:101:", "src/accounting.rs:101:"),
            log(True).replace("src/accounting/units.rs:101:", "src/accounting/units.rs:102:"),
            log(True).replace(":101:9 in function", ":101:13 in function"),
            log(True).replace("in function " + gate.MUTANT_FUNCTION,
                              "in function accounting::Meter::charge_general"),
            log(True).replace("unwinding assertion loop 0", "assertion failed"),
            log(True).replace("Status: FAILURE", "Status: UNDETERMINED"),
            log(True).replace("Status: FAILURE", "Status: ERROR"),
            log(True) + check(2, "model.assertion.1", "FAILURE"),
            log(True) + log(True),
            "error: could not compile quire-exact\n",
        ):
            with self.subTest(log=corrupted), self.assertRaises(gate.GateError):
                gate.classify(corrupted, 1, 101)
        for returncode in (0, 124, 137, -9):
            with self.subTest(returncode=returncode), self.assertRaises(gate.GateError):
                gate.classify(log(True), returncode, 101)

    def test_mutation_targets_common_validation_and_leaves_fallback_intact(self):
        repository = Path(__file__).parent.parent
        original = (repository / gate.MUTANT_FILE).read_bytes()
        fallback = (repository / "src/accounting.rs").read_bytes()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path = root / gate.MUTANT_FILE
            path.parent.mkdir(parents=True)
            path.write_bytes(original)
            (root / "src/accounting.rs").write_bytes(fallback)
            line = gate.mutate_all_loop(root)
            mutated = path.read_text()
            self.assertEqual(mutated.splitlines()[line - 1],
                             "        for requested_kind in LimitKind::ALL {")
            self.assertEqual(mutated.count("for requested_kind in LimitKind::ALL"), 1)
            self.assertIn("observe(charge.first.filter(|(kind, _)| *kind == requested_kind));", mutated)
            self.assertIn("observe(charge.second.filter(|(kind, _)| *kind == requested_kind));", mutated)
            self.assertNotIn("        observe(charge.first);", mutated)
            self.assertEqual((root / "src/accounting.rs").read_bytes(), fallback)
            with self.assertRaises(gate.GateError):
                gate.mutate_all_loop(root)  # Already mutated is not a fresh source.
            path.write_text("changed production validation\n")
            with self.assertRaises(gate.GateError):
                gate.mutate_all_loop(root)
        self.assertEqual((repository / gate.MUTANT_FILE).read_bytes(), original)
        self.assertEqual((repository / "src/accounting.rs").read_bytes(), fallback)


class ProcessLifecycleTests(unittest.TestCase):
    """Use actual processes and owned scopes; no replacement kill functions."""

    def family(self, scope=False, leader_exits=False):
        import subprocess
        import sys
        import uuid
        child = (
            "import os,signal;signal.signal(signal.SIGTERM,signal.SIG_IGN);"
            "print(os.getpid(),flush=True);signal.pause()"
        )
        leader = "import subprocess,sys,signal;subprocess.Popen([sys.executable,'-c'," + repr(child) + "]);"
        leader += "sys.exit(0)" if leader_exits else "signal.pause()"
        command = [sys.executable, "-c", leader]
        observation = {"cgroup": None, "last_cgroup_sample": None}
        unit = "meter-kani-test-" + uuid.uuid4().hex
        if scope:
            command = ["systemd-run", "--user", "--scope", "--quiet", "--unit=" + unit,
                       "-p", "MemoryMax=16G", "-p", "MemorySwapMax=0", *command]
        process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   text=True, start_new_session=True)
        self.addCleanup(self.remove_family, process, scope, unit, observation)
        line = process.stdout.readline()
        self.assertTrue(line.strip().isdigit(), "controlled descendant must signal readiness")
        child_pid = int(line.strip())
        if scope:
            gate.observe_scope(unit, observation)
            self.assertEqual(observation["last_cgroup_sample"]["memory_max"], str(gate.MEMORY_BYTES))
            self.assertEqual(observation["last_cgroup_sample"]["swap_max"], "0")
        return process, child_pid, unit, observation

    @staticmethod
    def remove_family(process, scope, unit, observation):
        import os
        import signal
        import subprocess
        if scope:
            subprocess.run(["systemctl", "--user", "kill", "--signal=KILL", unit + ".scope"],
                           capture_output=True, check=False)
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait(timeout=5)
        process.stdout.close()
        process.stderr.close()

    def assert_not_live(self, pid):
        path = Path(f"/proc/{pid}/stat")
        self.assertTrue(not path.exists() or path.read_text().rsplit(")", 1)[1].split()[0] in {"Z", "X"},
                        f"descendant {pid} must not remain live")

    def test_group_cleanup_after_leader_exit(self):
        process, child, _, _ = self.family(leader_exits=True)
        process.wait(timeout=5)
        with self.assertRaisesRegex(gate.GateError, "live descendants"):
            gate.wait_group(process, 5, grace=0.1)
        self.assert_not_live(child)
        self.assertEqual(gate.live_group_members(process.pid), [])

    def test_group_timeout_kills_term_resistant_descendant(self):
        import subprocess
        process, child, _, _ = self.family()
        with self.assertRaises(subprocess.TimeoutExpired):
            gate.wait_group(process, 0.1, grace=0.1)
        self.assert_not_live(child)
        self.assertEqual(gate.live_group_members(process.pid), [])

    def interrupt(self, operation):
        import signal
        previous = signal.signal(signal.SIGALRM, gate.interrupted)
        try:
            signal.setitimer(signal.ITIMER_REAL, 0.1)
            with self.assertRaises(KeyboardInterrupt):
                operation()
        finally:
            signal.setitimer(signal.ITIMER_REAL, 0)
            signal.signal(signal.SIGALRM, previous)

    def test_group_interruption_kills_term_resistant_descendant(self):
        process, child, _, _ = self.family()
        self.interrupt(lambda: gate.wait_group(process, 5, grace=0.1))
        self.assert_not_live(child)
        self.assertEqual(gate.live_group_members(process.pid), [])

    def test_scope_cleanup_after_leader_exit(self):
        process, child, unit, observation = self.family(scope=True, leader_exits=True)
        process.wait(timeout=5)
        with self.assertRaisesRegex(gate.GateError, "live descendants"):
            gate.wait_scope(process, unit, observation, grace=0.1)
        self.assert_not_live(child)
        self.assertTrue(gate.scope_empty(unit, observation))

    def test_scope_timeout_kills_term_resistant_descendant(self):
        import subprocess
        process, child, unit, observation = self.family(scope=True)
        with self.assertRaises(subprocess.TimeoutExpired):
            gate.wait_scope(process, unit, observation, timeout=0.1, grace=0.1)
        self.assert_not_live(child)
        self.assertTrue(gate.scope_empty(unit, observation))

    def test_scope_interruption_kills_term_resistant_descendant(self):
        process, child, unit, observation = self.family(scope=True)
        self.interrupt(lambda: gate.wait_scope(process, unit, observation, grace=0.1))
        self.assert_not_live(child)
        self.assertTrue(gate.scope_empty(unit, observation))


class HostGuardTests(unittest.TestCase):
    def helper(self, name, argv="harmless-helper"):
        import subprocess
        import sys
        code = ("from pathlib import Path;import signal;"
                f"Path('/proc/self/comm').write_text({name!r});"
                "print('ready',flush=True);signal.pause()")
        process = subprocess.Popen([argv, "-c", code], executable=sys.executable,
                                   stdout=subprocess.PIPE, text=True, start_new_session=True)
        self.addCleanup(self.remove_helper, process)
        self.assertEqual(process.stdout.readline().strip(), "ready")
        return process

    @staticmethod
    def remove_helper(process):
        gate.stop_group(process, grace=0.1)
        process.stdout.close()

    def test_required_os_names_cannot_hide_behind_argv(self):
        for name in ("cargo-kani", "kani-driver", "kani-compiler", "kani", "cbmc",
                     "goto-instrument", "goto-cc", "goto-analyzer"):
            with self.subTest(os_name=name):
                process = self.helper(name)
                self.assertIn(process.pid, [pid for pid, _ in gate.unrelated_proof_processes()])
                self.remove_helper(process)

    def test_ordinary_build_and_test_names_are_permitted(self):
        for name in ("cargo", "rustc", "clippy-driver", "rustfmt", "native-test"):
            with self.subTest(os_name=name):
                process = self.helper(name)
                self.assertNotIn(process.pid, [pid for pid, _ in gate.unrelated_proof_processes()])
                self.remove_helper(process)

    def test_native_target_deps_executable_is_permitted(self):
        import shutil
        import subprocess
        with tempfile.TemporaryDirectory() as temporary:
            executable = Path(temporary) / "target" / "deps" / "native-test"
            executable.parent.mkdir(parents=True)
            shutil.copy2(shutil.which("sleep"), executable)
            process = subprocess.Popen([str(executable), "60"], start_new_session=True)
            try:
                self.assertEqual(Path(f"/proc/{process.pid}/exe").resolve(), executable)
                self.assertNotIn(process.pid, [pid for pid, _ in gate.unrelated_proof_processes()])
            finally:
                gate.stop_group(process, grace=0.1)

    def test_fake_argv_does_not_replace_os_identity(self):
        process = self.helper("innocent-helper", argv="kani")
        self.assertNotIn(process.pid, [pid for pid, _ in gate.unrelated_proof_processes()])
        self.remove_helper(process)

    def test_actual_named_ancestor_is_exempt(self):
        import subprocess
        import sys
        original = Path('/proc/self/comm').read_text().strip()
        try:
            Path('/proc/self/comm').write_text('cargo-kani')
            code = ("import os,meter_kani_gate as gate;"
                    "assert os.getppid() not in [pid for pid,_ in gate.unrelated_proof_processes()]")
            result = subprocess.run([sys.executable, "-c", code], cwd=Path(__file__).parent,
                                    capture_output=True, text=True, check=False)
            self.assertEqual(result.returncode, 0, result.stderr)
        finally:
            Path('/proc/self/comm').write_text(original)


if __name__ == "__main__":
    unittest.main()
