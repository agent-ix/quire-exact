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
        text += check(1, "accounting::Meter::charge.unwind.1", "FAILURE",
                      "unwinding assertion loop 1",
                      "src/accounting.rs:862:9 in function accounting::Meter::charge")
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
        self.assertEqual(gate.classify(log(True), 1, 862)["verdict"],
                         "ExpectedProductionUnwindFailure")
        for corrupted in (
            log(True).replace("Meter::charge.unwind.1", "kani_proofs::step.assertion.1"),
            log(True).replace("Meter::charge.unwind.1", "Meter::charge.unwind.0"),
            log(True).replace("src/accounting.rs:862:", "src/accounting/kani_proofs.rs:862:"),
            log(True).replace("src/accounting.rs:862:", "src/accounting.rs:863:"),
            log(True).replace("unwinding assertion loop 1", "assertion failed"),
            log(True).replace("Status: FAILURE", "Status: UNDETERMINED"),
            log(True).replace("Status: FAILURE", "Status: ERROR"),
            log(True) + check(2, "model.assertion.1", "FAILURE"),
            log(True) + log(True),
            "error: could not compile quire-exact\n",
        ):
            with self.subTest(log=corrupted), self.assertRaises(gate.GateError):
                gate.classify(corrupted, 1, 862)
        for returncode in (0, 124, 137, -9):
            with self.subTest(returncode=returncode), self.assertRaises(gate.GateError):
                gate.classify(log(True), returncode, 862)

    def test_mutation_keeps_original_input_and_targets_real_request_loop(self):
        original = (Path(__file__).parent.parent / "src/accounting.rs").read_bytes()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "src").mkdir()
            path = root / "src/accounting.rs"
            path.write_bytes(original)
            line = gate.mutate_all_loop(root)
            mutated = path.read_text()
            self.assertEqual(mutated.splitlines()[line - 1],
                             "        for requested_kind in LimitKind::ALL {")
            self.assertIn("if *kind != requested_kind {", mutated)
            self.assertEqual(mutated.count("for (kind, amount) in charge.sizes.iter()"), 1)
            path.write_text("changed production loop\n")
            with self.assertRaises(gate.GateError):
                gate.mutate_all_loop(root)
        self.assertEqual((Path(__file__).parent.parent / "src/accounting.rs").read_bytes(), original)


if __name__ == "__main__":
    unittest.main()
