// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public checked-invariant carrier controls for FR-369.

use ix_trace_rs::trace;
use quire_exact::{CheckedInvariantCause, IllTypedCause, Refusal};

// Independently authored exhaustive expectations, rather than a production list.
fn expected_name(cause: CheckedInvariantCause) -> &'static str {
    match cause {
        CheckedInvariantCause::NativeIntegerCapacityBreach => "native-integer-capacity",
        CheckedInvariantCause::CollectionElementNotAdmitted => "collection-element",
        CheckedInvariantCause::DeferredResultNotAdmitted => "deferred-result",
        CheckedInvariantCause::CanonicalKeyUnavailable => "canonical-key",
        CheckedInvariantCause::BoundedDivisionExpected => "bounded-division",
        CheckedInvariantCause::CollectionKindMismatch => "collection-kind",
        CheckedInvariantCause::PopulationPair => "population-pair",
        CheckedInvariantCause::ValueKindMismatch => "value-kind",
        CheckedInvariantCause::UnknownCheckedFunction => "unknown-function",
        CheckedInvariantCause::ForeignCheckedExpression => "foreign-expression",
        CheckedInvariantCause::MeterBorrowConflict => "meter-borrow",
        CheckedInvariantCause::PackageReentry => "package-reentry",
        CheckedInvariantCause::EqualityScheduleMismatch => "equality-schedule",
        CheckedInvariantCause::ScheduledComparisonRefused { .. } => "scheduled-comparison",
        CheckedInvariantCause::EqualityOperandSourceNotAdmitted => "equality-source-admission",
        CheckedInvariantCause::EqualityOperandNonIntegralDecimal => "equality-nonintegral-decimal",
        CheckedInvariantCause::EqualityQuantityConversionRejected { .. } => "equality-conversion",
        CheckedInvariantCause::EqualityQuantityNonExactPlacement => "equality-placement",
        CheckedInvariantCause::EqualityConversionShapeMismatch => "equality-conversion-shape",
        CheckedInvariantCause::EqualityOperandTargetNotAdmitted => "equality-target-admission",
        CheckedInvariantCause::EqualityUnitUnresolved => "equality-unit",
        CheckedInvariantCause::EqualityEnumVariantUnresolved => "equality-enum-variant",
        CheckedInvariantCause::ExpectedIntegerPlacement => "integer-placement",
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

#[trace("TC-917", "FR-369-AC-1", "FR-369-AC-5", "FR-096-AC-8")]
#[test]
fn all_typed_causes_are_distinct_internal_faults_with_required_traits() {
    fn cause_traits<T: Copy + Clone + std::fmt::Debug + Eq + std::hash::Hash + PartialEq>() {}
    fn refusal_traits<T: Clone + std::fmt::Debug + Eq + PartialEq>() {}
    cause_traits::<CheckedInvariantCause>();
    refusal_traits::<Refusal>();

    let cases = [
        (
            CheckedInvariantCause::NativeIntegerCapacityBreach,
            "native-integer-capacity",
        ),
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
        (
            CheckedInvariantCause::UnknownCheckedFunction,
            "unknown-function",
        ),
        (
            CheckedInvariantCause::ForeignCheckedExpression,
            "foreign-expression",
        ),
        (CheckedInvariantCause::MeterBorrowConflict, "meter-borrow"),
        (CheckedInvariantCause::PackageReentry, "package-reentry"),
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
            CheckedInvariantCause::EqualityOperandSourceNotAdmitted,
            "equality-source-admission",
        ),
        (
            CheckedInvariantCause::EqualityOperandNonIntegralDecimal,
            "equality-nonintegral-decimal",
        ),
        (
            CheckedInvariantCause::EqualityQuantityConversionRejected {
                cause: IllTypedCause::IncompatibleDimensions,
            },
            "equality-conversion",
        ),
        (
            CheckedInvariantCause::EqualityQuantityNonExactPlacement,
            "equality-placement",
        ),
        (
            CheckedInvariantCause::EqualityConversionShapeMismatch,
            "equality-conversion-shape",
        ),
        (
            CheckedInvariantCause::EqualityOperandTargetNotAdmitted,
            "equality-target-admission",
        ),
        (
            CheckedInvariantCause::EqualityUnitUnresolved,
            "equality-unit",
        ),
        (
            CheckedInvariantCause::EqualityEnumVariantUnresolved,
            "equality-enum-variant",
        ),
        (
            CheckedInvariantCause::ExpectedIntegerPlacement,
            "integer-placement",
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
    assert_eq!(cases.len(), 31);
    let mut names = std::collections::BTreeSet::new();
    for (index, (cause, name)) in cases.iter().enumerate() {
        assert!(
            names.insert(*name),
            "duplicate checked-invariant cause name: {name}"
        );
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

#[trace("TC-917", "FR-369-AC-1", "FR-369-AC-5")]
#[test]
fn comparison_payloads_retain_the_original_typed_cause() {
    let constructors: [fn(IllTypedCause) -> CheckedInvariantCause; 4] = [
        |cause| CheckedInvariantCause::ScheduledComparisonRefused { cause },
        |cause| CheckedInvariantCause::EqualityQuantityConversionRejected { cause },
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
                        | CheckedInvariantCause::EqualityQuantityConversionRejected { cause }
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

#[trace("TC-917", "FR-369-AC-1", "FR-369-AC-5")]
#[test]
fn rust_compiler_rejects_unit_constructor_and_removed_causes() {
    use std::{fs, process::Command};

    let executable = std::env::current_exe().expect("integration test executable");
    let dependencies = executable.parent().expect("Cargo dependency directory");
    let library = fs::read_dir(dependencies)
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
        // Feature variants can coexist; Cargo just built this lane's dependency.
        .max_by_key(|path| fs::metadata(path).unwrap().modified().unwrap())
        .expect("Cargo built the kernel rlib");
    let directory = dependencies.join(format!("checked-invariant-controls-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let compile = |name: &str, source: &str| {
        let input = directory.join(format!("{name}.rs"));
        fs::write(&input, source).unwrap();
        Command::new("rustc")
            .args([
                "--edition=2021",
                "--crate-type",
                "lib",
                "--emit",
                "metadata",
            ])
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
    let removed = compile(
        "removed_depth_cause",
        "use quire_exact::CheckedInvariantCause;\n\
         pub fn cause() -> CheckedInvariantCause { CheckedInvariantCause::CallDepthExceeded }",
    );
    let error = String::from_utf8_lossy(&removed.stderr);
    assert!(!removed.status.success());
    assert!(
        error.contains("E0599") && error.contains("CallDepthExceeded"),
        "{error}"
    );

    let removed = compile(
        "removed_placeholder_cause",
        "use quire_exact::CheckedInvariantCause;\n\
         pub fn cause() -> CheckedInvariantCause { CheckedInvariantCause::GeneratedBodyPlaceholderInvoked }",
    );
    let error = String::from_utf8_lossy(&removed.stderr);
    assert!(!removed.status.success());
    assert!(
        error.contains("E0599") && error.contains("GeneratedBodyPlaceholderInvoked"),
        "{error}"
    );

    fs::remove_dir_all(directory).unwrap();
}
