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
        CheckedInvariantCause::EqualityOperandSourceNotAdmitted => "equality-source-admission",
        CheckedInvariantCause::EqualityOperandNonIntegralDecimal => "equality-nonintegral-decimal",
        CheckedInvariantCause::EqualityQuantityConversionRejected { .. } => "equality-conversion",
        CheckedInvariantCause::EqualityQuantityNonExactPlacement => "equality-placement",
        CheckedInvariantCause::EqualityConversionShapeMismatch => "equality-conversion-shape",
        CheckedInvariantCause::EqualityOperandTargetNotAdmitted => "equality-target-admission",
        CheckedInvariantCause::EqualityUnitUnresolved => "equality-unit",
        CheckedInvariantCause::EqualityEnumVariantUnresolved => "equality-enum-variant",
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

/// Trace: FR-369-AC-1, FR-369-AC-5
#[test]
fn rust_compiler_rejects_unit_constructor_and_an_unhandled_new_cause() {
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
    let compile = |name: &str, source: &str, executable: bool| {
        let input = directory.join(format!("{name}.rs"));
        fs::write(&input, source).unwrap();
        Command::new("rustc")
            .args([
                "--edition=2021",
                "--crate-type",
                if executable { "bin" } else { "lib" },
                "--emit",
                if executable { "link" } else { "metadata" },
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
        false,
    );
    assert!(
        valid.status.success(),
        "{}",
        String::from_utf8_lossy(&valid.stderr)
    );
    let invalid = compile(
        "former_unit",
        "pub fn refusal() -> quire_exact::Refusal { quire_exact::Refusal::CheckedInvariant }",
        false,
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
    let valid = compile("exhaustive_current", &source, false);
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
    let invalid = compile("exhaustive_extended", &extended, false);
    let error = String::from_utf8_lossy(&invalid.stderr);
    assert!(!invalid.status.success());
    assert!(
        error.contains("E0004") && error.contains("UnhandledControlCause"),
        "{error}"
    );

    // Mutate the actual public carrier mapping, keeping the same successful
    // compiler setup and payload. The intended None assertion must then fail.
    let start = owning_source.find("pub enum Refusal {").unwrap();
    let end = start + owning_source[start..].find("\n}\n").unwrap() + 2;
    let refusal = &owning_source[start..end];
    let start = owning_source.find("impl Refusal {").unwrap();
    let end = start + owning_source[start..].find("\n}\n").unwrap() + 2;
    let mappings = &owning_source[start..end];
    let source = format!(
        "use quire_exact::{{BoundViolation, CardinalityBound, CheckedInvariantCause, \
         CollectionKind, DecimalType, DivisionMember, IeeeFlags, IeeeWidth, IllTypedCause, \
         InexactTarget, IntegerInterval, RationalDomain, TextType, UniverseId}};\n\
         #[derive(Clone, Debug, Eq, PartialEq)]\n{refusal}\n{mappings}\n\
         fn main() {{\n\
             let refusal = Refusal::CheckedInvariant {{\n\
                 cause: CheckedInvariantCause::EqualityQuantityConversionRejected {{\n\
                     cause: IllTypedCause::DistinctUnits\n\
                 }}\n\
             }};\n\
             assert_eq!(refusal.code(), None, \"typed carrier has no catalog code\");\n\
             assert_eq!(refusal.cause(), None, \"typed carrier has no catalog cause\");\n\
         }}"
    );
    for (name, source, should_pass) in [
        ("carrier_mapping", source.clone(), true),
        (
            "carrier_mapping_mutated",
            source.replacen(
                "Self::CheckedInvariant { .. } => None",
                "Self::CheckedInvariant { .. } => Some(\"mutation-catalog-code\")",
                1,
            ),
            false,
        ),
    ] {
        let built = compile(name, &source, true);
        assert!(
            built.status.success(),
            "{name} must compile: {}",
            String::from_utf8_lossy(&built.stderr)
        );
        let ran = Command::new(directory.join(name)).output().unwrap();
        assert_eq!(ran.status.success(), should_pass);
        if !should_pass {
            let error = String::from_utf8_lossy(&ran.stderr);
            assert!(
                error.contains("typed carrier has no catalog code")
                    && error.contains("mutation-catalog-code"),
                "mutation must fail the intended mapping assertion: {error}"
            );
        }
    }
    fs::remove_dir_all(directory).unwrap();
}
