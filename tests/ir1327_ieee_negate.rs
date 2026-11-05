// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public IEEE sign negation and native meter evidence.

use quire_exact::{
    evaluate_ieee, ChargePoint, IeeeOperation, IeeeOperationKind, IeeeValue, IeeeWidth, Integer,
    LimitKind, Meter, Outcome, RoundingMode, ScalarLimits,
};

fn limits(width: u64, work: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: width,
        decimal_digits: 0,
        scale_expansion: 0,
        text_input_bytes: 0,
        text_scalars: 0,
        normalized_scalars: 0,
        unit_edges: 0,
        value_occurrences: 1,
        work_units: work,
        result_units: 1,
    }
}

fn value(width: IeeeWidth, bits: u64) -> IeeeValue {
    match width {
        IeeeWidth::Binary32 => IeeeValue::binary32(u32::try_from(bits).unwrap()),
        IeeeWidth::Binary64 => IeeeValue::binary64(bits),
    }
}

/// Trace: FR-394-AC-1, FR-394-AC-2
#[test]
fn negate_flips_only_sign_for_qspec_patterns_under_every_mode() {
    // QSpec FR-148-AC-11 names these representative bit patterns and modes.
    for (width, mask, patterns) in [
        (
            IeeeWidth::Binary32,
            0x8000_0000,
            &[
                0x3f80_0000,
                0,
                0x8000_0000,
                0x7f80_0000,
                0x7f80_0001,
                0xffc0_0005,
            ][..],
        ),
        (
            IeeeWidth::Binary64,
            0x8000_0000_0000_0000,
            &[
                0x3ff0_0000_0000_0000,
                0,
                0x8000_0000_0000_0000,
                0x7ff0_0000_0000_0000,
                0x7ff0_0000_0000_0001,
                0xfff8_0000_0000_0005,
            ][..],
        ),
    ] {
        for &bits in patterns {
            for mode in RoundingMode::ALL {
                let source = value(width, bits);
                assert_eq!(
                    IeeeOperation::Negate(source).kind(),
                    IeeeOperationKind::Negate
                );
                let result = evaluate_ieee(
                    IeeeOperation::Negate(source),
                    mode,
                    &mut Meter::new(limits(width.bits().into(), 2)),
                )
                .unwrap()
                .completed()
                .expect("negation completes");
                assert_eq!(result.value().width(), width);
                assert_eq!(
                    result.value().bits(),
                    bits ^ mask,
                    "{width:?} {bits:#x} {mode:?}"
                );
                assert!(result.flags().is_empty());
                assert_eq!(result.provenance().width(), width);
                assert_eq!(result.provenance().rounding(), mode);
                assert_eq!(result.value().is_signaling_nan(), source.is_signaling_nan());
                let restored = evaluate_ieee(
                    IeeeOperation::Negate(result.value()),
                    mode,
                    &mut Meter::new(limits(width.bits().into(), 2)),
                )
                .unwrap()
                .completed()
                .expect("second negation completes");
                assert_eq!(restored.value().bits(), bits);
            }
        }
    }
}

/// Trace: FR-394-AC-3
#[test]
fn negate_charges_only_operand_and_result_retention() {
    for (source, expected) in [
        (IeeeValue::binary32(0x4000_0000), 0xc000_0000),
        (
            IeeeValue::binary64(0x4000_0000_0000_0000),
            0xc000_0000_0000_0000,
        ),
    ] {
        let mut meter = Meter::new(limits(u64::from(source.width().bits()), 2));
        let result = evaluate_ieee(
            IeeeOperation::Negate(source),
            RoundingMode::Exact,
            &mut meter,
        )
        .unwrap()
        .completed()
        .expect("negation completes");
        assert_eq!(result.value().bits(), expected);
        assert_eq!(
            meter
                .admitted_charges()
                .iter()
                .map(|c| c.point)
                .collect::<Vec<_>>(),
            [ChargePoint::IeeeOperands, ChargePoint::IeeeResultRetain]
        );
        assert_eq!(
            meter.admitted_charges()[0].integer_bits,
            Some(u64::from(source.width().bits()))
        );
        assert_eq!(meter.admitted_charges()[0].value_occurrences, Some(1));
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 2);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    }
}

/// Trace: FR-394-AC-4
#[test]
fn negate_denial_exposes_only_admitted_prefix() {
    for (width_limit, work_limit, point, kind, limit, consumed, next, admitted) in [
        (
            32,
            1,
            ChargePoint::IeeeResultRetain,
            LimitKind::WorkUnits,
            1,
            1,
            1,
            vec![ChargePoint::IeeeOperands],
        ),
        (
            32,
            0,
            ChargePoint::IeeeOperands,
            LimitKind::WorkUnits,
            0,
            0,
            1,
            vec![],
        ),
        (
            31,
            2,
            ChargePoint::IeeeOperands,
            LimitKind::IntegerBits,
            31,
            0,
            32,
            vec![],
        ),
    ] {
        let mut meter = Meter::new(limits(width_limit, work_limit));
        let outcome = evaluate_ieee(
            IeeeOperation::Negate(IeeeValue::binary32(0x4000_0000)),
            RoundingMode::Exact,
            &mut meter,
        )
        .unwrap();
        let Outcome::Incomplete(denial) = outcome else {
            panic!("expected incomplete");
        };
        assert_eq!(denial.charge_point, point);
        assert_eq!(denial.limit_kind, kind);
        assert_eq!(denial.limit, limit);
        assert_eq!(denial.consumed, consumed);
        assert_eq!(denial.next_charge, Integer::from(next));
        assert_eq!(
            meter
                .admitted_charges()
                .iter()
                .map(|c| c.point)
                .collect::<Vec<_>>(),
            admitted
        );
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    }
}
