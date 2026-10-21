// SPDX-License-Identifier: AGPL-3.0-or-later
//! Executable binding for the standalone Meter proof and its trip-count control.

use ix_trace_rs::trace;
use std::{path::PathBuf, process::Command, time::SystemTime};

#[trace("FR-359-AC-9")]
#[test]
#[ignore = "requires Kani, cgroup v2 and systemd user scopes; run make kani-meter-gate"]
fn meter_proof_and_production_all_loop_control() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let stamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("clock is after Unix epoch")
        .as_nanos();
    let report = root
        .join("target/meter-kani-reports")
        .join(format!("{stamp}-{}", std::process::id()));
    let status = Command::new("python3")
        .arg(root.join("scripts/meter_kani_gate.py"))
        .arg("--report")
        .arg(&report)
        .current_dir(&root)
        .status()
        .expect("start the Meter Kani gate with Python 3");
    assert!(
        status.success(),
        "healthy Meter proof and production ALL-loop control must pass; report: {}",
        report.display()
    );
}
