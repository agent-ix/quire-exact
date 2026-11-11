// SPDX-License-Identifier: AGPL-3.0-or-later
//! Executable binding for the units Kani proofs of the meter's common charge path.
//!
//! The test runs real Kani; it is ignored by default and run by `make kani-meter-gate`.
//! Each harness must report `VERIFICATION:- SUCCESSFUL` with every cover satisfied within 30 minutes.

use ix_trace_rs::trace;
use std::process::{Command, Output, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const PROOFS: [&str; 3] = [
    "units_charge_matches_independent_transition",
    "units_adapter_maps_common_charge_to_units_charge",
    "units_injection_cancellation_preserves_occurrence_and_saturation",
];

/// Kani prints ` ** N of M cover properties satisfied`; accept only N == M.
fn covers_all_satisfied(log: &str) -> bool {
    log.lines().any(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        matches!(words.as_slice(), ["**", n, "of", m, "cover", "properties", "satisfied", ..] if n == m)
    })
}

/// Per-harness wall-clock limit; a harness that does not finish in time fails the gate.
const TIMEOUT: Duration = Duration::from_secs(30 * 60);

fn run_proof(proof: &str) -> Output {
    let child = Command::new("cargo")
        .args(["kani", "--harness"])
        .arg(format!("accounting::units_proofs::{proof}"))
        .args(["-Z", "function-contracts", "--exact", "--unwind", "3"])
        .args([
            "--jobs",
            "1",
            "--solver",
            "cadical",
            "--output-format",
            "regular",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start cargo kani");
    let pid = child.id();
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let _ = sender.send(child.wait_with_output());
    });
    match receiver.recv_timeout(TIMEOUT) {
        Ok(output) => output.expect("wait for cargo kani"),
        Err(_) => {
            let _ = Command::new("pkill")
                .args(["-TERM", "-P", &pid.to_string()])
                .status();
            let _ = Command::new("kill").arg(pid.to_string()).status();
            panic!("{proof} did not finish within {TIMEOUT:?}");
        }
    }
}

#[trace("TC-931", "FR-359-AC-9")]
#[test]
#[ignore = "runs Kani/CBMC (about one minute, about 1.3 GB); run with `make kani-meter-gate`"]
fn units_kani_proofs_verify() {
    for proof in PROOFS {
        let output = run_proof(proof);
        let log = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success() && log.contains("VERIFICATION:- SUCCESSFUL"),
            "{proof} did not verify:\n{log}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            covers_all_satisfied(&log),
            "{proof} left a cover unsatisfied:\n{log}"
        );
    }
}
