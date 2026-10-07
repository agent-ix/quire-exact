// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public IEEE and text owner evidence for IR-673.

use quire_exact::{
    admit_text, compare_ieee, compare_text, convert_ieee_width, evaluate_ieee, ieee_to_exact,
    ComparisonOperator, IeeeComparison, IeeeExactLoss, IeeeExactTarget, IeeeFlag, IeeeFlags,
    IeeeOperation, IeeeValue, IeeeWidth, IllTyped, IllTypedCause, Integer, IntegerInterval,
    LimitKind, Meter, Outcome, Rational, RationalDomain, Refusal, RoundingMode, ScalarLimits,
    TextPayload, TextProfile, TextType,
};

fn limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    }
}

fn meter() -> Meter {
    Meter::new(limits())
}

fn value(width: IeeeWidth, bits: u64) -> IeeeValue {
    match width {
        IeeeWidth::Binary32 => IeeeValue::binary32(u32::try_from(bits).unwrap()),
        IeeeWidth::Binary64 => IeeeValue::binary64(bits),
    }
}

/// Trace: TC-912, FR-364-AC-1
#[test]
fn ieee_flags_iterate_all_subsets_in_vocabulary_order() {
    let vocabulary = [
        IeeeFlag::Invalid,
        IeeeFlag::DivideByZero,
        IeeeFlag::Overflow,
        IeeeFlag::Underflow,
        IeeeFlag::Inexact,
    ];
    assert_eq!(IeeeFlag::ALL, vocabulary);
    for mask in 0..32_u32 {
        let selected: Vec<_> = vocabulary
            .into_iter()
            .enumerate()
            .filter_map(|(i, f)| ((mask & (1 << i)) != 0).then_some(f))
            .collect();
        let forward: IeeeFlags = selected.iter().copied().collect();
        let reverse: IeeeFlags = selected.iter().rev().copied().collect();
        assert_eq!(forward.iter().collect::<Vec<_>>(), selected);
        assert_eq!(reverse.iter().collect::<Vec<_>>(), selected);
        assert_eq!(forward.is_empty(), mask == 0);
    }
}

/// Trace: TC-914, FR-366-AC-1
#[test]
fn ieee_nan_selection_quiets_first_and_reports_later_signaling() {
    for (width, exp, quiet, sign, one) in [
        (
            IeeeWidth::Binary32,
            0x7f80_0000,
            0x0040_0000,
            0x8000_0000,
            0x3f80_0000,
        ),
        (
            IeeeWidth::Binary64,
            0x7ff0_0000_0000_0000,
            0x0008_0000_0000_0000,
            0x8000_0000_0000_0000,
            0x3ff0_0000_0000_0000,
        ),
    ] {
        let positive_quiet = value(width, exp | quiet | 5);
        let negative_signaling = value(width, sign | exp | 9);
        for (left, right, expected) in [
            (positive_quiet, negative_signaling, positive_quiet),
            (
                negative_signaling,
                positive_quiet,
                value(width, sign | exp | quiet | 9),
            ),
        ] {
            let result = evaluate_ieee(
                IeeeOperation::Add(left, right),
                RoundingMode::NearestEven,
                &mut meter(),
            )
            .unwrap()
            .completed()
            .unwrap();
            assert_eq!(result.value().bits(), expected.bits());
            assert!(result.flags().contains(IeeeFlag::Invalid));
        }
        let other_quiet = value(width, sign | exp | quiet | 9);
        let result = evaluate_ieee(
            IeeeOperation::Add(positive_quiet, other_quiet),
            RoundingMode::NearestEven,
            &mut meter(),
        )
        .unwrap()
        .completed()
        .unwrap();
        assert_eq!(result.value().bits(), positive_quiet.bits());
        assert!(!result.flags().contains(IeeeFlag::Invalid));
        let result = evaluate_ieee(
            IeeeOperation::FusedMultiplyAdd(positive_quiet, value(width, one), negative_signaling),
            RoundingMode::NearestEven,
            &mut meter(),
        )
        .unwrap()
        .completed()
        .unwrap();
        assert_eq!(result.value().bits(), positive_quiet.bits());
        assert!(result.flags().contains(IeeeFlag::Invalid));
    }
}

/// Trace: TC-914, FR-366-AC-2
#[test]
fn ieee_nan_width_conversion_refuses_unrepresentable_payload() {
    let exp64 = 0x7ff0_0000_0000_0000_u64;
    let quiet64 = 1_u64 << 51;
    for payload in [1_u64 << 22, 1_u64 << 30] {
        let mut m = meter();
        assert!(matches!(
            convert_ieee_width(
                IeeeValue::binary64(exp64 | quiet64 | payload),
                IeeeWidth::Binary32,
                RoundingMode::NearestEven,
                &mut m
            ),
            Outcome::Refused(Refusal::IeeeNanPayloadNotRepresentable {
                source: IeeeWidth::Binary64,
                target: IeeeWidth::Binary32
            })
        ));
        assert_eq!(m.consumed(LimitKind::ResultUnits), 0);
    }
    for (sign64, sign32) in [(0_u64, 0_u64), (1_u64 << 63, 1_u64 << 31)] {
        let mut m = meter();
        let result = convert_ieee_width(
            IeeeValue::binary64(sign64 | exp64 | quiet64 | ((1 << 22) - 1)),
            IeeeWidth::Binary32,
            RoundingMode::NearestEven,
            &mut m,
        )
        .completed()
        .unwrap();
        assert_eq!(
            result.value().bits(),
            sign32 | 0x7f80_0000 | 0x0040_0000 | ((1 << 22) - 1)
        );
        assert_eq!(m.consumed(LimitKind::ResultUnits), 1);
    }
}

/// Trace: TC-914, FR-366-AC-3
#[test]
fn ieee_signed_zero_converts_to_canonical_rational_with_loss() {
    let domain = RationalDomain::new(
        IntegerInterval::new(Integer::from(-1_i64), Integer::from(1_i64)).unwrap(),
        IntegerInterval::new(Integer::from(1_i64), Integer::from(1_i64)).unwrap(),
    )
    .unwrap();
    for (bits, loss) in [
        (0_u32, None),
        (0x8000_0000, Some(IeeeExactLoss::NegativeZeroSign)),
    ] {
        let result = ieee_to_exact(
            IeeeValue::binary32(bits),
            IeeeExactTarget::Rational(&domain),
            &mut meter(),
        )
        .unwrap()
        .completed()
        .unwrap();
        assert_eq!(
            result.value(),
            &Rational::new(Integer::zero(), Integer::one()).unwrap()
        );
        assert_eq!(result.loss(), loss);
    }
}

/// Trace: TC-914, FR-366-AC-4
#[test]
fn ieee_total_order_separates_ten_classes_and_signed_zero() {
    for (width, sign, exp, quiet, one) in [
        (
            IeeeWidth::Binary32,
            0x8000_0000,
            0x7f80_0000,
            0x0040_0000,
            0x3f80_0000,
        ),
        (
            IeeeWidth::Binary64,
            0x8000_0000_0000_0000,
            0x7ff0_0000_0000_0000,
            0x0008_0000_0000_0000,
            0x3ff0_0000_0000_0000,
        ),
    ] {
        let ordered = [
            sign | exp | quiet | 1,
            sign | exp | 1,
            sign | exp,
            sign | one,
            sign,
            0,
            one,
            exp,
            exp | 1,
            exp | quiet | 1,
        ];
        for pair in ordered.windows(2) {
            let (left, right) = (value(width, pair[0]), value(width, pair[1]));
            assert!(left.total_order_key() < right.total_order_key());
            assert_eq!(
                compare_ieee(IeeeComparison::TotalOrder, left, right, &mut meter())
                    .unwrap()
                    .completed(),
                Some(true)
            );
            assert_eq!(
                compare_ieee(IeeeComparison::TotalOrder, right, left, &mut meter())
                    .unwrap()
                    .completed(),
                Some(false)
            );
        }
        for (lower, higher) in [
            (exp | 1, exp | 2),
            (exp | quiet | 1, exp | quiet | 2),
            (sign | exp | 2, sign | exp | 1),
            (sign | exp | quiet | 2, sign | exp | quiet | 1),
        ] {
            assert!(value(width, lower).total_order_key() < value(width, higher).total_order_key());
        }
        assert_ne!(
            value(width, sign).total_order_key(),
            value(width, 0).total_order_key()
        );
        assert_eq!(
            compare_ieee(
                IeeeComparison::NumericEqual,
                value(width, sign),
                value(width, 0),
                &mut meter()
            )
            .unwrap()
            .completed(),
            Some(true)
        );
    }
}

/// Trace: TC-915, FR-367-AC-1
#[test]
fn text_profile_mismatch_refuses_before_any_comparison_charge() {
    let payload = TextPayload::from_utf8(b"a").unwrap();
    let nfc = TextType::new(0, 4, TextProfile::Nfc).unwrap();
    let binary = TextType::new(0, 4, TextProfile::BinaryUtf8).unwrap();
    let left = admit_text(&payload, &nfc, &mut meter())
        .completed()
        .unwrap();
    let right = admit_text(&payload, &binary, &mut meter())
        .completed()
        .unwrap();
    let mut comparison = Meter::new(ScalarLimits {
        integer_bits: 0,
        decimal_digits: 0,
        scale_expansion: 0,
        text_input_bytes: 0,
        text_scalars: 0,
        normalized_scalars: 0,
        unit_edges: 0,
        value_occurrences: 0,
        work_units: 0,
        result_units: 0,
    });
    assert_eq!(
        compare_text(ComparisonOperator::Equal, &left, &right, &mut comparison),
        Err(IllTyped {
            cause: IllTypedCause::DistinctTextProfiles
        })
    );
    for kind in LimitKind::ALL {
        assert_eq!(comparison.consumed(kind), 0);
    }
    assert_eq!(comparison.admission_count(), 0);
    #[cfg(feature = "test-support")]
    assert!(comparison.admitted_charges().is_empty());
    #[cfg(feature = "test-support")]
    assert!(!comparison.charge_log_truncated());
}
