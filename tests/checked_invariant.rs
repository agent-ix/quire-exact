// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public checked-invariant carrier controls for FR-369.

use quire_exact::{CheckedInvariantCause, IllTypedCause, Refusal};

// Independently authored exhaustive expectations, rather than a production list.
fn expected_name(cause: CheckedInvariantCause) -> &'static str {
    match cause {
        CheckedInvariantCause::CollectionElementNotAdmitted => "collection-element",
        CheckedInvariantCause::DeferredResultNotAdmitted => "deferred-result",
        CheckedInvariantCause::CanonicalKeyUnavailable => "canonical-key",
        CheckedInvariantCause::BoundedDivisionExpected => "bounded-division",
        CheckedInvariantCause::CollectionKindMismatch => "collection-kind",
        CheckedInvariantCause::PopulationPair => "population-pair",
        CheckedInvariantCause::ValueKindMismatch => "value-kind",
        CheckedInvariantCause::CallDepthExceeded => "call-depth",
        CheckedInvariantCause::UnknownCheckedFunction => "unknown-function",
        CheckedInvariantCause::ForeignCheckedExpression => "foreign-expression",
        CheckedInvariantCause::MeterBorrowConflict => "meter-borrow",
        CheckedInvariantCause::EqualityScheduleMismatch => "equality-schedule",
        CheckedInvariantCause::ScheduledComparisonRefused { .. } => "scheduled-comparison",
        CheckedInvariantCause::ExpectedIntegerPlacement => "integer-placement",
        CheckedInvariantCause::GeneratedBodyPlaceholderInvoked => "generated-placeholder",
        CheckedInvariantCause::GeneratedArgumentShapeMismatch => "generated-arguments",
        CheckedInvariantCause::GeneratedOperandKindUnsupported => "generated-operand",
        CheckedInvariantCause::GeneratedUnexpectedOutcome => "generated-outcome",
        CheckedInvariantCause::GeneratedIntervalInvalid => "generated-interval",
        CheckedInvariantCause::GeneratedEnvironmentRejected => "generated-environment",
        CheckedInvariantCause::GeneratedTypeCheckRejected { .. } => "generated-type-check",
        CheckedInvariantCause::GeneratedEqualityCheckRejected { .. } => "generated-equality-check",
        CheckedInvariantCause::GeneratedDescriptorReconstructionFailed => "generated-descriptor",
    }
}

/// Trace: FR-369-AC-1, FR-369-AC-5, FR-096-AC-8
#[test]
fn all_typed_causes_are_distinct_internal_faults_with_required_traits() {
    fn cause_traits<T: Copy + Clone + std::fmt::Debug + Eq + std::hash::Hash + PartialEq>() {}
    fn refusal_traits<T: Clone + std::fmt::Debug + Eq + PartialEq>() {}
    cause_traits::<CheckedInvariantCause>();
    refusal_traits::<Refusal>();

    let cases = [
        (
            CheckedInvariantCause::CollectionElementNotAdmitted,
            "collection-element",
        ),
        (
            CheckedInvariantCause::DeferredResultNotAdmitted,
            "deferred-result",
        ),
        (
            CheckedInvariantCause::CanonicalKeyUnavailable,
            "canonical-key",
        ),
        (
            CheckedInvariantCause::BoundedDivisionExpected,
            "bounded-division",
        ),
        (
            CheckedInvariantCause::CollectionKindMismatch,
            "collection-kind",
        ),
        (CheckedInvariantCause::PopulationPair, "population-pair"),
        (CheckedInvariantCause::ValueKindMismatch, "value-kind"),
        (CheckedInvariantCause::CallDepthExceeded, "call-depth"),
        (
            CheckedInvariantCause::UnknownCheckedFunction,
            "unknown-function",
        ),
        (
            CheckedInvariantCause::ForeignCheckedExpression,
            "foreign-expression",
        ),
        (CheckedInvariantCause::MeterBorrowConflict, "meter-borrow"),
        (
            CheckedInvariantCause::EqualityScheduleMismatch,
            "equality-schedule",
        ),
        (
            CheckedInvariantCause::ScheduledComparisonRefused {
                cause: IllTypedCause::DistinctTextProfiles,
            },
            "scheduled-comparison",
        ),
        (
            CheckedInvariantCause::ExpectedIntegerPlacement,
            "integer-placement",
        ),
        (
            CheckedInvariantCause::GeneratedBodyPlaceholderInvoked,
            "generated-placeholder",
        ),
        (
            CheckedInvariantCause::GeneratedArgumentShapeMismatch,
            "generated-arguments",
        ),
        (
            CheckedInvariantCause::GeneratedOperandKindUnsupported,
            "generated-operand",
        ),
        (
            CheckedInvariantCause::GeneratedUnexpectedOutcome,
            "generated-outcome",
        ),
        (
            CheckedInvariantCause::GeneratedIntervalInvalid,
            "generated-interval",
        ),
        (
            CheckedInvariantCause::GeneratedEnvironmentRejected,
            "generated-environment",
        ),
        (
            CheckedInvariantCause::GeneratedTypeCheckRejected {
                cause: IllTypedCause::DistinctUnits,
            },
            "generated-type-check",
        ),
        (
            CheckedInvariantCause::GeneratedEqualityCheckRejected {
                cause: IllTypedCause::OperatorIneligible,
            },
            "generated-equality-check",
        ),
        (
            CheckedInvariantCause::GeneratedDescriptorReconstructionFailed,
            "generated-descriptor",
        ),
    ];
    for (index, (cause, name)) in cases.iter().enumerate() {
        assert_eq!(expected_name(*cause), *name);
        let refusal = Refusal::CheckedInvariant { cause: *cause };
        assert_eq!(refusal.code(), None, "{name}");
        assert_eq!(refusal.cause(), None, "{name}");
        assert_eq!(refusal, refusal.clone());
        for (other_index, (other, _)) in cases.iter().enumerate() {
            assert_eq!(
                refusal == Refusal::CheckedInvariant { cause: *other },
                index == other_index,
                "distinct causes must compare unequal: {cause:?}, {other:?}"
            );
        }
    }
}

/// Trace: FR-369-AC-1, FR-369-AC-5
#[test]
fn comparison_payloads_retain_the_original_typed_cause() {
    let constructors: [fn(IllTypedCause) -> CheckedInvariantCause; 3] = [
        |cause| CheckedInvariantCause::ScheduledComparisonRefused { cause },
        |cause| CheckedInvariantCause::GeneratedTypeCheckRejected { cause },
        |cause| CheckedInvariantCause::GeneratedEqualityCheckRejected { cause },
    ];
    for construct in constructors {
        let first = construct(IllTypedCause::DistinctUnits);
        let second = construct(IllTypedCause::IncompatibleDimensions);
        for (value, expected) in [
            (first, IllTypedCause::DistinctUnits),
            (second, IllTypedCause::IncompatibleDimensions),
        ] {
            let refusal = Refusal::CheckedInvariant { cause: value };
            let retained = match refusal {
                Refusal::CheckedInvariant {
                    cause:
                        CheckedInvariantCause::ScheduledComparisonRefused { cause }
                        | CheckedInvariantCause::GeneratedTypeCheckRejected { cause }
                        | CheckedInvariantCause::GeneratedEqualityCheckRejected { cause },
                } => cause,
                other => panic!("typed comparison payload lost: {other:?}"),
            };
            assert_eq!(retained, expected);
            assert_eq!(refusal.code(), None);
            assert_eq!(refusal.cause(), None);
        }
        assert_ne!(first, second);
        assert_ne!(
            Refusal::CheckedInvariant { cause: first },
            Refusal::CheckedInvariant { cause: second }
        );
    }
}

/// Trace: FR-369-AC-1
#[test]
fn rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause() {
    use std::{fs, process::Command};

    let executable = std::env::current_exe().expect("integration test executable");
    let dependencies = executable.parent().expect("Cargo dependency directory");
    let mut libraries: Vec<_> = fs::read_dir(dependencies)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("libquire_exact-")
                && path
                    .extension()
                    .is_some_and(|extension| extension == "rlib")
        })
        .collect();
    libraries.sort();
    let library = libraries.first().expect("Cargo built the kernel rlib");
    let directory = dependencies.join(format!("checked-invariant-controls-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let compile = |name: &str, source: &str| {
        let input = directory.join(format!("{name}.rs"));
        fs::write(&input, source).unwrap();
        Command::new("rustc")
            .args(["--edition=2021", "--crate-type=lib", "--emit=metadata"])
            .arg("--extern")
            .arg(format!("quire_exact={}", library.display()))
            .arg("-L")
            .arg(format!("dependency={}", dependencies.display()))
            .arg("--out-dir")
            .arg(&directory)
            .arg(input)
            .output()
            .expect("run the real Rust compiler")
    };
    let valid = compile(
        "required_payload",
        "use quire_exact::{Refusal, CheckedInvariantCause};\n\
         pub fn refusal() -> Refusal { Refusal::CheckedInvariant { cause: CheckedInvariantCause::PopulationPair } }",
    );
    assert!(
        valid.status.success(),
        "{}",
        String::from_utf8_lossy(&valid.stderr)
    );
    let invalid = compile(
        "former_unit",
        "pub fn refusal() -> quire_exact::Refusal { quire_exact::Refusal::CheckedInvariant }",
    );
    let error = String::from_utf8_lossy(&invalid.stderr);
    assert!(!invalid.status.success());
    assert!(
        error.contains("E0533") && error.contains("CheckedInvariant"),
        "{error}"
    );

    // Compile the actual owning enum with the independently authored match,
    // then add a new unit cause to prove that this match cannot absorb it.
    let owning_source = include_str!("../src/outcome.rs");
    let start = owning_source
        .find("pub enum CheckedInvariantCause {")
        .unwrap();
    let end = start + owning_source[start..].find("\n}\n").unwrap() + 2;
    let declaration = &owning_source[start..end];
    let test_source = include_str!("checked_invariant.rs");
    let start = test_source.find("fn expected_name(").unwrap();
    let end = start + test_source[start..].find("\n}\n").unwrap() + 2;
    let matcher = &test_source[start..end];
    let source = format!("use quire_exact::IllTypedCause;\n{declaration}\n{matcher}");
    let valid = compile("exhaustive_current", &source);
    assert!(
        valid.status.success(),
        "{}",
        String::from_utf8_lossy(&valid.stderr)
    );
    let extended = source.replacen(
        "pub enum CheckedInvariantCause {",
        "pub enum CheckedInvariantCause {\n    UnhandledControlCause,",
        1,
    );
    let invalid = compile("exhaustive_extended", &extended);
    let error = String::from_utf8_lossy(&invalid.stderr);
    assert!(!invalid.status.success());
    assert!(
        error.contains("E0004") && error.contains("UnhandledControlCause"),
        "{error}"
    );
    fs::remove_dir_all(directory).unwrap();
}
