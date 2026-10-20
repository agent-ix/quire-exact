// SPDX-License-Identifier: AGPL-3.0-or-later
//! Fresh dependency resolution must preserve QSpec-FR-141's Unicode 17 profile.

use std::{fs, path::Path, process::Command};

/// Trace: ix://agent-ix/quire-specification/FR-141-AC-1
/// Provenance: IR-935, QSpec TC-186
#[test]
fn fresh_consumer_resolves_and_runs_unicode17_normalization() {
    let owner = Path::new(env!("CARGO_MANIFEST_DIR"));
    let directory = owner
        .join("target")
        .join(format!("fresh-unicode-consumer-{}", std::process::id()));
    // A new directory, rather than the owner's lockfile, supplies the resolution.
    fs::create_dir_all(directory.parent().unwrap()).unwrap();
    fs::create_dir(&directory).expect("fresh consumer directory must not already exist");
    fs::create_dir(directory.join("src")).unwrap();
    fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"fresh-unicode-consumer\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\
             [workspace]\n\
             [dependencies]\nquire-exact = {{ path = {owner:?} }}\n"
        ),
    )
    .unwrap();
    fs::write(
        directory.join("src/main.rs"),
        include_str!("fixtures/unicode17_consumer.rs"),
    )
    .unwrap();
    assert!(!directory.join("Cargo.lock").exists());

    // A separate target avoids recursively locking the outer cargo test target.
    // Offline resolution uses the ordinary registry cache without consumer pins.
    let output = Command::new(env!("CARGO"))
        .current_dir(&directory)
        .args(["run", "--offline", "--jobs", "1"])
        .env("CARGO_TARGET_DIR", directory.join("target"))
        .output()
        .expect("run the real consumer through Cargo");
    assert!(
        output.status.success(),
        "fresh Unicode 17 consumer failed: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(directory.join("Cargo.lock").is_file());
}
