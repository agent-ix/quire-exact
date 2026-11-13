// SPDX-License-Identifier: AGPL-3.0-or-later
//! The public `KERNEL_VERSION` constant and the golden outcome and charge grid
//! that guards the operations it versions (FR-406, TC-954).

use ix_trace_rs::trace;
use quire_exact::KERNEL_VERSION;

/// Usable in a const context from an integration test.
const VERSION_IN_CONST: u32 = KERNEL_VERSION;

#[trace("TC-954", "FR-406-AC-1")]
#[test]
fn kernel_version_is_a_public_u32_const() {
    let as_u32: u32 = VERSION_IN_CONST;
    assert!(as_u32 >= 1);
}

#[cfg(feature = "test-support")]
mod golden {
    use std::fmt::Debug;

    use ix_trace_rs::trace;
    use quire_exact::{
        admit_text, build_typed_prestate, compare_quantity, compare_text, divide, evaluate_decimal,
        evaluate_ieee, evaluate_integer_arithmetic, evaluate_native_decimal,
        evaluate_native_integer_arithmetic, evaluate_native_integer_division,
        evaluate_native_integer_ordering, evaluate_rational_arithmetic, form_collection, modulo,
        AdmittedCharge, CardinalityBound, CollectionKind, CollectionType, ComparisonOperator,
        ConstructionCause, Decimal, DecimalOperation, DecimalType, DivisionMember, DivisionProfile,
        EffectiveId, IeeeOperation, IeeeValue, Integer, IntegerArithmetic, IntegerDomain,
        LimitKind, Meter, NativeDecimal, NativeDecimalOperation, NativeDecimalType, NativeInteger,
        NativeIntegerOperands, NativeIntegerOperation, NativeIntegerPlan, NodeKey, ObjectId,
        OrderingOperator, OriginalDeclarationKey, PathSegment, Quantity, Rational,
        RationalArithmetic, RoundingMode, ScalarLimits, TextPayload, TextProfile, TextType, UnitId,
        Value, ValueType,
    };

    use super::KERNEL_VERSION;

    fn limits(integer_bits: u64) -> ScalarLimits {
        ScalarLimits {
            integer_bits,
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
        Meter::new(limits(u64::MAX))
    }

    fn n(value: i64) -> Integer {
        Integer::from(value)
    }

    fn ratio(numerator: i64, denominator: i64) -> Rational {
        Rational::new(n(numerator), n(denominator)).unwrap()
    }

    fn unit(name: &str) -> UnitId {
        UnitId::declared(NodeKey::new("test", vec![PathSegment::Text(name.into())], 0).unwrap())
    }

    /// One recorded or observed case: the outcome and the ordered charges.
    #[derive(Clone, Debug, PartialEq)]
    struct Case {
        name: String,
        outcome: String,
        charges: String,
    }

    fn recorded(name: &str, outcome: &str, charges: &str) -> Case {
        Case {
            name: name.into(),
            outcome: outcome.into(),
            charges: charges.into(),
        }
    }

    /// The ordered admitted charges, then the consumed total of every limit
    /// kind, so work and result units (which `AdmittedCharge` does not record)
    /// are part of the record.
    fn charges(meter: &Meter) -> String {
        let admitted = meter
            .admitted_charges()
            .iter()
            .map(render_charge)
            .collect::<Vec<_>>()
            .join(" ; ");
        let consumed = LimitKind::ALL
            .iter()
            .map(|kind| format!("{kind:?}={}", meter.consumed(*kind)))
            .collect::<Vec<_>>()
            .join(" ");
        format!("{admitted} | consumed {consumed}")
    }

    fn render_charge(charge: &AdmittedCharge) -> String {
        let AdmittedCharge {
            point,
            integer_bits,
            decimal_digits,
            scale_expansion,
            text_input_bytes,
            text_scalars,
            normalized_scalars,
            unit_edges,
            value_occurrences,
        } = *charge;
        let sizes = [
            ("integer_bits", integer_bits),
            ("decimal_digits", decimal_digits),
            ("scale_expansion", scale_expansion),
            ("text_input_bytes", text_input_bytes),
            ("text_scalars", text_scalars),
            ("normalized_scalars", normalized_scalars),
            ("unit_edges", unit_edges),
            ("value_occurrences", value_occurrences),
        ];
        let mut rendered = format!("{point:?}");
        for (kind, amount) in sizes {
            if let Some(amount) = amount {
                rendered.push_str(&format!(" {kind}={amount}"));
            }
        }
        rendered
    }

    fn case<T: Debug>(name: &str, limits: ScalarLimits, run: impl FnOnce(&mut Meter) -> T) -> Case {
        let mut meter = Meter::new(limits);
        let outcome = run(&mut meter);
        Case {
            name: name.into(),
            outcome: format!("{outcome:?}"),
            charges: charges(&meter),
        }
    }

    fn native_decimal_case(name: &str, operation: NativeDecimalOperation, max_scale: u64) -> Case {
        let ty =
            NativeDecimalType::new(-1000, 1000, 0, max_scale, RoundingMode::NearestEven).unwrap();
        case(name, limits(u64::MAX), |m| {
            evaluate_native_decimal(ty.admit(operation).unwrap(), m)
        })
    }

    fn native_integer_case(
        name: &str,
        operation: NativeIntegerOperation,
        a: i128,
        b: i128,
    ) -> Case {
        let operands = NativeIntegerOperands::Binary {
            left: NativeInteger::new(a),
            right: NativeInteger::new(b),
        };
        let admitted = NativeIntegerPlan::new(operation, None)
            .unwrap()
            .admit(operands)
            .unwrap();
        case(name, limits(u64::MAX), |m| {
            evaluate_native_integer_arithmetic(admitted, m)
        })
    }

    fn singleton(value: i128) -> quire_exact::NativeIntegerInterval {
        quire_exact::NativeIntegerInterval::new(value, value).unwrap()
    }

    /// Evaluate the grid of FR-406 Behavior 5 on the current source.
    fn observe() -> Vec<Case> {
        let (a, b, zero) = (n(7), n(2), n(0));
        let mathematical = IntegerDomain::Mathematical;
        let mut grid = vec![
            case("integer.add", limits(u64::MAX), |m| {
                evaluate_integer_arithmetic(IntegerArithmetic::Add(&a, &b), None, m)
            }),
            case("integer.subtract", limits(u64::MAX), |m| {
                evaluate_integer_arithmetic(IntegerArithmetic::Subtract(&a, &b), None, m)
            }),
            case("integer.multiply", limits(u64::MAX), |m| {
                evaluate_integer_arithmetic(IntegerArithmetic::Multiply(&a, &b), None, m)
            }),
            case("integer.negate", limits(u64::MAX), |m| {
                evaluate_integer_arithmetic(IntegerArithmetic::Negate(&a), None, m)
            }),
            case("integer.divide.truncating", limits(u64::MAX), |m| {
                let minus = n(-7);
                divide(
                    DivisionProfile::Truncating,
                    DivisionMember::Quotient,
                    &minus,
                    &b,
                    &mathematical,
                    m,
                )
            }),
            case("integer.divide.floor.remainder", limits(u64::MAX), |m| {
                let minus = n(-7);
                divide(
                    DivisionProfile::Floor,
                    DivisionMember::Remainder,
                    &minus,
                    &b,
                    &mathematical,
                    m,
                )
            }),
            case("integer.modulo", limits(u64::MAX), |m| {
                let minus = n(-7);
                modulo(&minus, &b, &mathematical, m)
            }),
            case("integer.divide.by-zero", limits(u64::MAX), |m| {
                divide(
                    DivisionProfile::Euclidean,
                    DivisionMember::Quotient,
                    &a,
                    &zero,
                    &mathematical,
                    m,
                )
            }),
            case("integer.modulo.by-zero", limits(u64::MAX), |m| {
                modulo(&a, &zero, &mathematical, m)
            }),
        ];

        let (half, third) = (
            Quantity::new(ratio(1, 2), unit("m")),
            Quantity::new(ratio(1, 3), unit("m")),
        );
        for operator in ComparisonOperator::ALL {
            grid.push(case(
                &format!("compare.{operator:?}"),
                limits(u64::MAX),
                |m| compare_quantity(operator, &half, &third, m),
            ));
        }
        grid.push(case("quantity.unit-mismatch", limits(u64::MAX), |m| {
            compare_quantity(
                ComparisonOperator::Less,
                &half,
                &Quantity::new(ratio(1, 3), unit("s")),
                m,
            )
        }));
        grid.push(case("ordering.integers", limits(u64::MAX), |m| {
            quire_exact::order_numbers(
                OrderingOperator::Less,
                quire_exact::OrderedOperands::Integers(&b, &a),
                m,
            )
        }));

        let decimal_type =
            DecimalType::new(n(-1000), n(1000), 0, 2, RoundingMode::NearestEven).unwrap();
        let (x, y, tie) = (
            Decimal::new(n(15), 1),
            Decimal::new(n(25), 1),
            Decimal::new(n(25), 1),
        );
        grid.push(case("decimal.multiply", limits(u64::MAX), |m| {
            evaluate_decimal(DecimalOperation::Multiply(&x, &y), &decimal_type, m)
        }));
        let to_integer =
            DecimalType::new(n(-1000), n(1000), 0, 0, RoundingMode::NearestEven).unwrap();
        grid.push(case("decimal.round-at-tie", limits(u64::MAX), |m| {
            evaluate_decimal(DecimalOperation::Round(&tie), &to_integer, m)
        }));

        grid.push(native_integer_case(
            "native-integer.add",
            NativeIntegerOperation::Add {
                left: singleton(9),
                right: singleton(4),
            },
            9,
            4,
        ));
        grid.push(case("native-integer.divide", limits(u64::MAX), |m| {
            evaluate_native_integer_division(
                DivisionProfile::Floor,
                DivisionMember::Quotient,
                NativeInteger::new(-9),
                NativeInteger::new(4),
                None,
                m,
            )
        }));
        grid.push(case("native-integer.order", limits(u64::MAX), |m| {
            evaluate_native_integer_ordering(
                OrderingOperator::Less,
                NativeInteger::new(-9),
                NativeInteger::new(4),
                m,
            )
        }));
        grid.push(native_decimal_case(
            "native-decimal.add",
            NativeDecimalOperation::Add(
                NativeDecimal::from_retained(15, 1),
                NativeDecimal::from_retained(25, 1),
            ),
            2,
        ));
        grid.push(native_decimal_case(
            "native-decimal.round-at-tie",
            NativeDecimalOperation::Round(NativeDecimal::from_retained(25, 1)),
            0,
        ));

        grid.push(case("rational.reduction", limits(u64::MAX), |m| {
            let (p, q) = (ratio(1, 6), ratio(1, 3));
            evaluate_rational_arithmetic(RationalArithmetic::Add(&p, &q), None, m)
        }));

        grid.push(case("ieee.add.inexact-overflow", limits(u64::MAX), |m| {
            let max = IeeeValue::binary32(0x7f7f_ffff);
            evaluate_ieee(IeeeOperation::Add(max, max), RoundingMode::NearestEven, m)
        }));
        grid.push(case("ieee.add.tie-to-even", limits(u64::MAX), |m| {
            // 1.0 + 2^-24 lies exactly between two binary32 values.
            let one = IeeeValue::binary32(0x3f80_0000);
            let half_ulp = IeeeValue::binary32(0x3380_0000);
            evaluate_ieee(
                IeeeOperation::Add(one, half_ulp),
                RoundingMode::NearestEven,
                m,
            )
        }));

        let text_type = TextType::new(0, 8, TextProfile::Nfc).unwrap();
        let admit = |bytes: &[u8]| {
            admit_text(
                &TextPayload::from_utf8(bytes).unwrap(),
                &text_type,
                &mut meter(),
            )
            .completed()
            .unwrap()
        };
        let (prefix, longer) = (admit(b"ab"), admit(b"abc"));
        grid.push(case("text.compare.prefix", limits(u64::MAX), |m| {
            compare_text(ComparisonOperator::Less, &prefix, &longer, m)
        }));

        let sequence = CollectionType::new(
            CollectionKind::Sequence,
            ValueType::Integer,
            Some(CardinalityBound::new(1, 2).unwrap()),
        );
        let values = |count: i64| {
            (1..=count)
                .map(|v| Value::Integer(n(v)))
                .collect::<Vec<_>>()
        };
        grid.push(case(
            "collection.insert-past-bound",
            limits(u64::MAX),
            |m| form_collection(&sequence, values(3), m),
        ));
        grid.push(case("collection.typed-prestate", limits(u64::MAX), |m| {
            let mut admitted =
                |_: &ValueType, _: &Value| -> Result<(), ConstructionCause> { Ok(()) };
            build_typed_prestate(&sequence, values(2), m, Some(&mut admitted))
        }));

        grid.push(Case {
            name: "identity.construction".into(),
            outcome: format!(
                "{:?} {:?}",
                EffectiveId::new(None, OriginalDeclarationKey::new("acme/orders", "Order")),
                ObjectId::new("o1").unwrap()
            ),
            charges: charges(&meter()),
        });

        // Operand bits equal the limit, so the operands charge is admitted at
        // exactly the limit and the next charge is the one denied.
        grid.push(case("meter.denied-at-exact-limit", limits(4), |m| {
            evaluate_integer_arithmetic(IntegerArithmetic::Add(&n(15), &n(1)), None, m)
        }));
        grid
    }

    /// Every way `observed` differs from `recorded`, naming the part that differs.
    fn differences(recorded: &[Case], observed: &[Case]) -> Vec<String> {
        let mut found = Vec::new();
        for (index, want) in recorded.iter().enumerate() {
            match observed.get(index) {
                Some(got) if got.name == want.name => {
                    if got.outcome != want.outcome {
                        found.push(format!(
                            "{} outcome\n  recorded: {}\n  observed: {}",
                            want.name, want.outcome, got.outcome
                        ));
                    }
                    if got.charges != want.charges {
                        found.push(format!(
                            "{} charges\n  recorded: {}\n  observed: {}",
                            want.name, want.charges, got.charges
                        ));
                    }
                }
                _ => found.push(format!("{} missing from the observed grid", want.name)),
            }
        }
        if observed.len() > recorded.len() {
            found.push("observed grid has cases that are not recorded".into());
        }
        found
    }

    fn golden() -> Vec<Case> {
        vec![
recorded("integer.add", "Completed(Integer(9))", "IntegerArithmeticOperands integer_bits=3 value_occurrences=2 ; IntegerArithmeticArithmetic integer_bits=4 ; IntegerArithmeticResultRetain | consumed IntegerBits=4 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("integer.subtract", "Completed(Integer(5))", "IntegerArithmeticOperands integer_bits=3 value_occurrences=2 ; IntegerArithmeticArithmetic integer_bits=4 ; IntegerArithmeticResultRetain | consumed IntegerBits=4 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("integer.multiply", "Completed(Integer(14))", "IntegerArithmeticOperands integer_bits=3 value_occurrences=2 ; IntegerArithmeticArithmetic integer_bits=5 ; IntegerArithmeticResultRetain | consumed IntegerBits=5 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("integer.negate", "Completed(Integer(-7))", "IntegerArithmeticOperands integer_bits=3 value_occurrences=1 ; IntegerArithmeticArithmetic integer_bits=3 ; IntegerArithmeticResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=1 WorkUnits=3 ResultUnits=1"),
recorded("integer.divide.truncating", "Completed(Integer(-3))", "IntegerDivisionOperands integer_bits=3 value_occurrences=2 ; IntegerDivisionArithmetic integer_bits=3 ; IntegerDivisionDomain value_occurrences=1 ; IntegerDivisionResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("integer.divide.floor.remainder", "Completed(Integer(1))", "IntegerDivisionOperands integer_bits=3 value_occurrences=2 ; IntegerDivisionArithmetic integer_bits=3 ; IntegerDivisionDomain value_occurrences=1 ; IntegerDivisionResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("integer.modulo", "Completed(Integer(1))", "IntegerModulusOperands integer_bits=3 value_occurrences=2 ; IntegerModulusArithmetic integer_bits=3 ; IntegerModulusDomain value_occurrences=1 ; IntegerModulusResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("integer.divide.by-zero", "Undefined(DivisionByZero)", "IntegerDivisionOperands integer_bits=3 value_occurrences=2 | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=1 ResultUnits=0"),
recorded("integer.modulo.by-zero", "Undefined(DivisionByZero)", "IntegerModulusOperands integer_bits=3 value_occurrences=2 | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=1 ResultUnits=0"),
recorded("compare.Equal", "Ok(Completed(false))", "OrderingOperands integer_bits=2 value_occurrences=2 ; OrderingArithmetic integer_bits=3 ; OrderingResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("compare.NotEqual", "Ok(Completed(true))", "OrderingOperands integer_bits=2 value_occurrences=2 ; OrderingArithmetic integer_bits=3 ; OrderingResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("compare.Less", "Ok(Completed(false))", "OrderingOperands integer_bits=2 value_occurrences=2 ; OrderingArithmetic integer_bits=3 ; OrderingResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("compare.LessOrEqual", "Ok(Completed(false))", "OrderingOperands integer_bits=2 value_occurrences=2 ; OrderingArithmetic integer_bits=3 ; OrderingResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("compare.Greater", "Ok(Completed(true))", "OrderingOperands integer_bits=2 value_occurrences=2 ; OrderingArithmetic integer_bits=3 ; OrderingResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("compare.GreaterOrEqual", "Ok(Completed(true))", "OrderingOperands integer_bits=2 value_occurrences=2 ; OrderingArithmetic integer_bits=3 ; OrderingResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("quantity.unit-mismatch", "Err(IllTyped { cause: DistinctUnits })", " | consumed IntegerBits=0 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=0 WorkUnits=0 ResultUnits=0"),
recorded("ordering.integers", "Completed(true)", "OrderingOperands integer_bits=3 value_occurrences=2 ; OrderingArithmetic integer_bits=3 ; OrderingResultRetain | consumed IntegerBits=3 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("decimal.multiply", "Completed(DecimalResult { value: Decimal { representation: DecimalRepresentation { coefficient: Integer(375), scale: 2 }, normalized: DecimalRepresentation { coefficient: Integer(375), scale: 2 } }, loss: None })", "DecimalOperands integer_bits=5 decimal_digits=2 value_occurrences=2 ; DecimalScaleExpansion scale_expansion=0 ; DecimalArithmetic integer_bits=9 decimal_digits=4 ; DecimalResultRetain integer_bits=9 decimal_digits=3 scale_expansion=0 value_occurrences=1 | consumed IntegerBits=9 DecimalDigits=4 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("decimal.round-at-tie", "Completed(DecimalResult { value: Decimal { representation: DecimalRepresentation { coefficient: Integer(2), scale: 0 }, normalized: DecimalRepresentation { coefficient: Integer(2), scale: 0 } }, loss: Some(DecimalLoss { exact: ExactLossValue { numerator: Integer(5), cofactor: Integer(1), twos: 1, fives: 0 }, rounded: DecimalRepresentation { coefficient: Integer(2), scale: 0 }, mode: NearestEven }) })", "DecimalOperands integer_bits=5 decimal_digits=2 value_occurrences=1 ; DecimalScaleExpansion scale_expansion=0 ; DecimalArithmetic integer_bits=5 decimal_digits=2 ; DecimalRounding integer_bits=5 decimal_digits=2 ; DecimalResultRetain integer_bits=2 decimal_digits=1 scale_expansion=0 value_occurrences=1 | consumed IntegerBits=5 DecimalDigits=2 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=1 WorkUnits=5 ResultUnits=1"),
recorded("native-integer.add", "Completed(NativeInteger(13))", "IntegerArithmeticOperands integer_bits=4 value_occurrences=2 ; IntegerArithmeticArithmetic integer_bits=5 ; IntegerArithmeticResultRetain | consumed IntegerBits=5 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("native-integer.divide", "Completed(NativeInteger(-3))", "IntegerDivisionOperands integer_bits=4 value_occurrences=2 ; IntegerDivisionArithmetic integer_bits=4 ; IntegerDivisionDomain value_occurrences=1 ; IntegerDivisionResultRetain | consumed IntegerBits=4 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("native-integer.order", "Completed(true)", "OrderingOperands integer_bits=4 value_occurrences=2 ; OrderingArithmetic integer_bits=4 ; OrderingResultRetain | consumed IntegerBits=4 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=3 ResultUnits=1"),
recorded("native-decimal.add", "Completed(NativeDecimalResult { value: NativeDecimal { coefficient: 400, scale: 2 }, loss: None })", "DecimalOperands integer_bits=5 decimal_digits=2 value_occurrences=2 ; DecimalScaleExpansion scale_expansion=0 ; DecimalArithmetic integer_bits=6 decimal_digits=3 ; DecimalResultRetain integer_bits=10 decimal_digits=3 scale_expansion=1 value_occurrences=1 | consumed IntegerBits=10 DecimalDigits=3 ScaleExpansion=1 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("native-decimal.round-at-tie", "Completed(NativeDecimalResult { value: NativeDecimal { coefficient: 2, scale: 0 }, loss: Some(NativeDecimalLoss { numerator: 5, cofactor: 1, twos: 1, fives: 0, rounded: NativeDecimal { coefficient: 2, scale: 0 }, mode: NearestEven }) })", "DecimalOperands integer_bits=5 decimal_digits=2 value_occurrences=1 ; DecimalScaleExpansion scale_expansion=0 ; DecimalArithmetic integer_bits=5 decimal_digits=2 ; DecimalRounding integer_bits=5 decimal_digits=2 ; DecimalResultRetain integer_bits=2 decimal_digits=1 scale_expansion=0 value_occurrences=1 | consumed IntegerBits=5 DecimalDigits=2 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=1 WorkUnits=5 ResultUnits=1"),
recorded("rational.reduction", "Completed(Rational { numerator: Integer(1), denominator: Integer(2) })", "RationalArithmeticOperands integer_bits=3 value_occurrences=2 ; RationalArithmeticArithmetic integer_bits=5 ; RationalArithmeticNormalize integer_bits=5 ; RationalArithmeticResultRetain | consumed IntegerBits=5 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("ieee.add.inexact-overflow", "Ok(Completed(IeeeResult { value: IeeeValue { width: Binary32, bits: 2139095040 }, flags: IeeeFlags(20), provenance: IeeeProvenance { width: Binary32, rounding: NearestEven } }))", "IeeeOperands integer_bits=32 value_occurrences=2 ; IeeeExactIntermediate integer_bits=32 ; IeeeRound integer_bits=32 ; IeeeResultRetain | consumed IntegerBits=32 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("ieee.add.tie-to-even", "Ok(Completed(IeeeResult { value: IeeeValue { width: Binary32, bits: 1065353216 }, flags: IeeeFlags(16), provenance: IeeeProvenance { width: Binary32, rounding: NearestEven } }))", "IeeeOperands integer_bits=32 value_occurrences=2 ; IeeeExactIntermediate integer_bits=32 ; IeeeRound integer_bits=32 ; IeeeResultRetain | consumed IntegerBits=32 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=4 ResultUnits=1"),
recorded("text.compare.prefix", "Ok(Completed(true))", "TextInputBytes text_input_bytes=5 value_occurrences=2 ; TextDecodeScalars text_scalars=5 ; TextNormalizeInput ; TextNormalizeOutput normalized_scalars=1 ; TextNormalizeOutput normalized_scalars=2 ; TextNormalizeOutput normalized_scalars=3 ; TextNormalizeOutput normalized_scalars=4 ; TextNormalizeOutput normalized_scalars=5 ; TextResultRetain | consumed IntegerBits=0 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=5 TextScalars=5 NormalizedScalars=5 UnitEdges=0 ValueOccurrences=2 WorkUnits=9 ResultUnits=1"),
recorded("collection.insert-past-bound", "Ok(Refused(CardinalityOutOfBound { violation: AboveMaximum, kind: Sequence, bound: CardinalityBound { minimum: 1, maximum: 2 }, count: 3 }))", "CollectionBound value_occurrences=3 | consumed IntegerBits=0 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=3 WorkUnits=1 ResultUnits=0"),
recorded("collection.typed-prestate", "Ok(Completed(Collection(CollectionValue { collection_type: CollectionType { kind: Sequence, element: Integer, bound: Some(CardinalityBound { minimum: 1, maximum: 2 }) }, elements: [Integer(Integer(1)), Integer(Integer(2))], occ: Integer(3) })))", "CollectionBound value_occurrences=2 ; CollectionResultRetain value_occurrences=3 | consumed IntegerBits=0 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=3 WorkUnits=2 ResultUnits=3"),
recorded("identity.construction", "EffectiveId([OriginalDeclarationKey { package: \"acme/orders\", node: \"Order\" }]) ObjectId(\"o1\")", " | consumed IntegerBits=0 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=0 WorkUnits=0 ResultUnits=0"),
recorded("meter.denied-at-exact-limit", "Incomplete(Incomplete { limit_kind: IntegerBits, limit: 4, consumed: 4, next_charge: Integer(5), charge_point: IntegerArithmeticArithmetic })", "IntegerArithmeticOperands integer_bits=4 value_occurrences=2 | consumed IntegerBits=4 DecimalDigits=0 ScaleExpansion=0 TextInputBytes=0 TextScalars=0 NormalizedScalars=0 UnitEdges=0 ValueOccurrences=2 WorkUnits=1 ResultUnits=0"),
        ]
    }

    #[test]
    #[ignore = "prints the observed grid as Rust source for re-recording"]
    fn print_observed_grid() {
        for c in observe() {
            println!("recorded({:?}, {:?}, {:?}),", c.name, c.outcome, c.charges);
        }
    }

    #[trace("TC-954", "FR-406-AC-2")]
    #[test]
    fn golden_grid_matches_the_record() {
        let found = differences(&golden(), &observe());
        assert!(
            found.is_empty(),
            "KERNEL_VERSION is {KERNEL_VERSION}; bump it if the change is intended:\n{}",
            found.join("\n")
        );
    }

    #[trace("TC-954", "FR-406-AC-3")]
    #[test]
    fn altered_outcome_or_charge_is_reported() {
        let observed = observe();
        assert!(differences(&observed, &observed).is_empty());

        let mut altered = observed.clone();
        altered[0].outcome.push('!');
        let found = differences(&altered, &observed);
        assert_eq!(found.len(), 1);
        assert!(found[0].starts_with("integer.add outcome"));

        let mut altered = observed.clone();
        altered[0].charges.push('!');
        let found = differences(&altered, &observed);
        assert_eq!(found.len(), 1);
        assert!(found[0].starts_with("integer.add charges"));
    }
}
