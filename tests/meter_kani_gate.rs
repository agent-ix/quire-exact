// SPDX-License-Identifier: AGPL-3.0-or-later
//! Executable binding for the units Kani proofs of the meter's common charge path.
//!
//! The test runs real Kani; it is ignored by default and run by `make kani-meter-gate`.
//! Each harness must report `VERIFICATION:- SUCCESSFUL` with every cover satisfied.

use ix_trace_rs::trace;
use std::process::Command;

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

#[trace("TC-931", "FR-359-AC-9")]
#[test]
#[ignore = "runs Kani/CBMC (about one minute, about 1.3 GB); run with `make kani-meter-gate`"]
fn units_kani_proofs_verify() {
    for proof in PROOFS {
        let output = Command::new("cargo")
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
            .output()
            .expect("start cargo kani");
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
