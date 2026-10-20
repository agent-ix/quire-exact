// SPDX-License-Identifier: AGPL-3.0-or-later
//! Fresh dependency resolution must preserve QSpec-FR-141's Unicode 17 profile.

use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use ix_trace_rs::trace;

struct ScratchDirectory(PathBuf);

impl ScratchDirectory {
    fn new(parent: &Path) -> io::Result<Self> {
        fs::create_dir_all(parent)?;
        // Atomic directory creation claims ownership; PID reuse and concurrent
        // invocations skip existing directories rather than reading their inputs.
        for attempt in 0..128 {
            let path = parent.join(format!("consumer-{}-{attempt}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "all 128 consumer scratch directory candidates are occupied",
        ))
    }
}

impl Drop for ScratchDirectory {
    fn drop(&mut self) {
        // Also runs if a Cargo invocation or assertion panics. Keep diagnostics
        // without risking a second panic while an assertion is unwinding.
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!(
                "could not remove consumer scratch {}: {error}",
                self.0.display()
            );
        }
    }
}

/// Provenance: IR-935, QSpec-TC-186
#[trace("QSpec-FR-141-AC-1")]
#[test]
fn fresh_consumer_resolves_and_runs_unicode17_normalization() {
    let owner = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch_parent = owner.join("target/fresh-unicode-consumers");
    // Occupy the first candidate to exercise collision handling, without granting
    // the consumer ownership of another invocation's files.
    let occupied = ScratchDirectory::new(&scratch_parent).unwrap();
    fs::write(occupied.0.join("sentinel"), "preserve other owner").unwrap();
    let scratch = ScratchDirectory::new(&scratch_parent).unwrap();
    let directory = &scratch.0;
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

    // Reuse a separate build cache without retaining consumer inputs or locks.
    // Cargo serializes this cache independently of the outer cargo test target.
    // Offline resolution uses the ordinary registry cache without consumer pins.
    let output = Command::new(env!("CARGO"))
        .current_dir(&directory)
        .args(["run", "--offline", "--jobs", "1"])
        .env(
            "CARGO_TARGET_DIR",
            owner.join("target/fresh-unicode-consumer-build"),
        )
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
    let removed = directory.clone();
    drop(scratch);
    assert!(
        !removed.exists(),
        "consumer inputs and lock must be removed"
    );
    assert_eq!(
        fs::read_to_string(occupied.0.join("sentinel")).unwrap(),
        "preserve other owner"
    );
    let removed = occupied.0.clone();
    drop(occupied);
    assert!(!removed.exists(), "collision control must be removed");

    let failure = ScratchDirectory::new(&scratch_parent).unwrap();
    let removed = failure.0.clone();
    fs::write(removed.join("Cargo.lock"), "failure cleanup control").unwrap();
    let failed = std::panic::catch_unwind(move || {
        let _owned_until_unwind = failure;
        panic!("simulate a consumer assertion failure");
    });
    assert!(failed.is_err());
    assert!(!removed.exists(), "failed consumer inputs must be removed");
}
