// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native float carrier against the shipped `evaluate_ieee` and
//! `compare_ieee` and independent literals (FR-405, TC-953).
use ix_trace_rs::trace;
use quire_exact::{
    compare_ieee, compare_native_float, evaluate_ieee, evaluate_native_float, IeeeComparison,
    IeeeFlag, IeeeFlags, IeeeOperation, IeeeValue, IeeeWidth, IllTyped, IllTypedCause, LimitKind,
    Meter, NativeFloat, NativeFloatOperation, NativeFloatRefusal, NativeFloatResult, Outcome,
    Refusal, RoundingMode, ScalarLimits,
};

// ---- helpers -------------------------------------------------------------------

fn limits(value: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: value,
        decimal_digits: value,
        scale_expansion: value,
        text_input_bytes: value,
        text_scalars: value,
        normalized_scalars: value,
        unit_edges: value,
        value_occurrences: value,
        work_units: value,
        result_units: value,
    }
}

fn set(mut base: ScalarLimits, kind: LimitKind, value: u64) -> ScalarLimits {
    match kind {
        LimitKind::IntegerBits => base.integer_bits = value,
        LimitKind::DecimalDigits => base.decimal_digits = value,
        LimitKind::ScaleExpansion => base.scale_expansion = value,
        LimitKind::TextInputBytes => base.text_input_bytes = value,
        LimitKind::TextScalars => base.text_scalars = value,
        LimitKind::NormalizedScalars => base.normalized_scalars = value,
        LimitKind::UnitEdges => base.unit_edges = value,
        LimitKind::ValueOccurrences => base.value_occurrences = value,
        LimitKind::WorkUnits => base.work_units = value,
        LimitKind::ResultUnits => base.result_units = value,
    }
    base
}

fn generous() -> Meter {
    Meter::new(limits(u64::MAX))
}

const WIDTHS: [IeeeWidth; 2] = [IeeeWidth::Binary32, IeeeWidth::Binary64];

fn nf(width: IeeeWidth, bits: u64) -> NativeFloat {
    NativeFloat::new(width, bits).expect("pattern fits the width")
}

fn sign_bit(width: IeeeWidth) -> u64 {
    1 << (width.bits() - 1)
}

/// The boundary patterns of TC-953, as independent literals, both signs.
fn boundary(width: IeeeWidth) -> Vec<u64> {
    let positive: Vec<u64> = match width {
        IeeeWidth::Binary32 => vec![
            0x0000_0000, // zero
            0x0000_0001, // smallest subnormal
            0x007F_FFFF, // largest subnormal
            0x0080_0000, // smallest normal
            0x7F7F_FFFF, // largest finite
            0x3F80_0000, // one
            0x3F80_0001, // next above one
            0x3F7F_FFFF, // next below one
            0x7F80_0000, // infinity
            0x7FC0_0000, // quiet NaN
            0x7F80_0001, // signalling NaN
            0x7FC0_0001, // NaN with a payload
            0x4000_0000, // two
            0x4040_0000, // three
            0x3F00_0000, // one half
            0x3400_0000, // 2^-23
            0x3380_0000, // 2^-24
            0x3440_0000, // 1.5 * 2^-23
            0x4B00_0000, // 2^23
            0x4B80_0000, // 2^24
            0x7F00_0000, // 2^127
            0x0100_0000, // 2^-126 * 2
        ],
        IeeeWidth::Binary64 => vec![
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0001,
            0x000F_FFFF_FFFF_FFFF,
            0x0010_0000_0000_0000,
            0x7FEF_FFFF_FFFF_FFFF,
            0x3FF0_0000_0000_0000,
            0x3FF0_0000_0000_0001,
            0x3FEF_FFFF_FFFF_FFFF,
            0x7FF0_0000_0000_0000,
            0x7FF8_0000_0000_0000,
            0x7FF0_0000_0000_0001,
            0x7FF8_0000_0000_0001,
            0x4000_0000_0000_0000,
            0x4008_0000_0000_0000,
            0x3FE0_0000_0000_0000,
            0x3CB0_0000_0000_0000, // 2^-52
            0x3CA0_0000_0000_0000, // 2^-53
            0x3CB8_0000_0000_0000, // 1.5 * 2^-52
            0x4330_0000_0000_0000, // 2^52
            0x4340_0000_0000_0000, // 2^53
            0x7FE0_0000_0000_0000, // 2^1023
            0x0020_0000_0000_0000,
        ],
    };
    let sign = sign_bit(width);
    let mut all = positive.clone();
    all.extend(positive.iter().map(|bits| bits | sign));
    all
}

/// A deterministic xorshift generator.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A pattern biased toward the interesting regions: any bits, small and
    /// large exponents, subnormals, and neighbours of a power of two.
    fn pattern(&mut self, width: IeeeWidth) -> u64 {
        let bits = u64::from(width.bits());
        let fraction_bits = if bits == 32 { 23 } else { 52 };
        let exponent_bits = bits - 1 - fraction_bits;
        let full = if bits == 32 {
            u64::from(u32::MAX)
        } else {
            u64::MAX
        };
        let raw = self.next() & full;
        let fraction = raw & ((1 << fraction_bits) - 1);
        let sign = (self.next() & 1) << (bits - 1);
        let ones = (1_u64 << exponent_bits) - 1;
        let bias = ones >> 1;
        let exponent = match self.next() % 8 {
            0 => 0,
            1 => ones,
            2 => bias + (self.next() % 9) - 4,
            3 => 1 + self.next() % 4,
            4 => ones - 1 - self.next() % 3,
            5 => bias + self.next() % (fraction_bits + 4),
            6 => bias - self.next() % (fraction_bits + 4),
            _ => self.next() % (ones + 1),
        };
        let fraction = match self.next() % 5 {
            0 => 0,
            1 => (1 << fraction_bits) - 1,
            2 => 1,
            3 => 1 << (fraction_bits - 1),
            _ => fraction,
        };
        sign | (exponent << fraction_bits) | fraction
    }
}

fn operations(width: IeeeWidth, operands: [u64; 3]) -> Vec<NativeFloatOperation> {
    let [a, b, c] = operands.map(|bits| nf(width, bits));
    vec![
        NativeFloatOperation::Negate(a),
        NativeFloatOperation::Add(a, b),
        NativeFloatOperation::Subtract(a, b),
        NativeFloatOperation::Multiply(a, b),
        NativeFloatOperation::Divide(a, b),
        NativeFloatOperation::SquareRoot(a),
        NativeFloatOperation::FusedMultiplyAdd(a, b, c),
    ]
}

fn owner_operation(operation: NativeFloatOperation) -> IeeeOperation {
    let v = NativeFloat::to_ieee_value;
    match operation {
        NativeFloatOperation::Negate(a) => IeeeOperation::Negate(v(a)),
        NativeFloatOperation::Add(a, b) => IeeeOperation::Add(v(a), v(b)),
        NativeFloatOperation::Subtract(a, b) => IeeeOperation::Subtract(v(a), v(b)),
        NativeFloatOperation::Multiply(a, b) => IeeeOperation::Multiply(v(a), v(b)),
        NativeFloatOperation::Divide(a, b) => IeeeOperation::Divide(v(a), v(b)),
        NativeFloatOperation::SquareRoot(a) => IeeeOperation::SquareRoot(v(a)),
        NativeFloatOperation::FusedMultiplyAdd(a, b, c) => {
            IeeeOperation::FusedMultiplyAdd(v(a), v(b), v(c))
        }
    }
}

type NativeOutcome = Result<Outcome<NativeFloatResult>, IllTyped>;

fn owner_outcome(
    operation: NativeFloatOperation,
    rounding: RoundingMode,
    meter: &mut Meter,
) -> NativeOutcome {
    evaluate_ieee(owner_operation(operation), rounding, meter).map(|outcome| match outcome {
        Outcome::Completed(result) => {
            assert_eq!(result.provenance().rounding(), rounding);
            assert_eq!(result.provenance().width(), result.value().width());
            Outcome::Completed(NativeFloatResult {
                value: result.value().into(),
                flags: result.flags(),
                rounding: result.provenance().rounding(),
            })
        }
        Outcome::Undefined(reason) => Outcome::Undefined(reason),
        Outcome::Refused(reason) => Outcome::Refused(reason),
        Outcome::Incomplete(record) => Outcome::Incomplete(record),
    })
}

/// Run both on fresh meters of `limits` and require identical outcomes and
/// identical meters (every consumed counter and, under `test-support`, the
/// ordered charge log).
fn agree(operation: NativeFloatOperation, rounding: RoundingMode, limits: ScalarLimits) {
    let (mut native_meter, mut owner_meter) = (Meter::new(limits), Meter::new(limits));
    let native = evaluate_native_float(operation, rounding, &mut native_meter);
    let owner = owner_outcome(operation, rounding, &mut owner_meter);
    assert_eq!(native, owner, "{operation:?} {rounding:?}");
    assert_eq!(native_meter, owner_meter, "{operation:?} {rounding:?}");
}

const KINDS: [LimitKind; 4] = [
    LimitKind::IntegerBits,
    LimitKind::ValueOccurrences,
    LimitKind::WorkUnits,
    LimitKind::ResultUnits,
];

/// A sufficient, an exact and a one-less limit for every kind the owner
/// consumes on this case.
fn agree_at_limits(operation: NativeFloatOperation, rounding: RoundingMode) {
    let mut probe = generous();
    let _ = owner_outcome(operation, rounding, &mut probe);
    agree(operation, rounding, limits(u64::MAX));
    for kind in KINDS {
        let used = probe.consumed(kind);
        agree(operation, rounding, set(limits(u64::MAX), kind, used));
        if used > 0 {
            agree(operation, rounding, set(limits(u64::MAX), kind, used - 1));
        }
    }
}

fn grid_operations(width: IeeeWidth) -> Vec<NativeFloatOperation> {
    let grid = boundary(width);
    let mut all = Vec::new();
    for &a in &grid {
        all.extend(operations(width, [a, a, a])[..1].iter().copied());
        all.push(NativeFloatOperation::SquareRoot(nf(width, a)));
        for &b in &grid {
            all.extend(operations(width, [a, b, a])[1..5].iter().copied());
        }
    }
    // Fused multiply-add over the full boundary grid in all three operands.
    for &a in &grid {
        for &b in &grid {
            for &c in &grid {
                all.push(NativeFloatOperation::FusedMultiplyAdd(
                    nf(width, a),
                    nf(width, b),
                    nf(width, c),
                ));
            }
        }
    }
    all
}

// ---- AC-1, AC-2 -----------------------------------------------------------------

fn assert_copy<T: Copy>() {}

#[test]
#[trace("TC-953", "FR-405-AC-1")]
fn carrier_is_copy_without_drop_glue_and_round_trips_the_boundary() {
    assert_copy::<NativeFloat>();
    const { assert!(!std::mem::needs_drop::<NativeFloat>()) };
    for width in WIDTHS {
        for bits in boundary(width) {
            let carried = nf(width, bits);
            assert_eq!((carried.width(), carried.bits()), (width, bits));
            let owner: IeeeValue = carried.into();
            assert_eq!((owner.width(), owner.bits()), (width, bits));
            assert_eq!(NativeFloat::from(owner), carried);
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-2")]
fn constructor_refuses_out_of_width_and_equality_is_structural() {
    // A constructor with no `Meter` in its signature.
    let _: fn(IeeeWidth, u64) -> Result<NativeFloat, NativeFloatRefusal> = NativeFloat::new;
    for bits in [1_u64 << 32, u64::from(u32::MAX) + 1, u64::MAX] {
        assert_eq!(
            NativeFloat::new(IeeeWidth::Binary32, bits),
            Err(NativeFloatRefusal::OutsideCarrierDomain)
        );
    }
    assert!(NativeFloat::new(IeeeWidth::Binary32, u64::from(u32::MAX)).is_ok());
    assert!(NativeFloat::new(IeeeWidth::Binary64, u64::MAX).is_ok());
    for width in WIDTHS {
        assert_ne!(nf(width, 0), nf(width, sign_bit(width)));
        let nan = if width == IeeeWidth::Binary32 {
            0x7F80_0000
        } else {
            0x7FF0_0000_0000_0000_u64
        };
        assert_ne!(nf(width, nan | 1), nf(width, nan | 2));
    }
    assert_ne!(nf(IeeeWidth::Binary32, 0), nf(IeeeWidth::Binary64, 0));
}

// ---- AC-3 -----------------------------------------------------------------------

#[test]
#[trace("TC-953", "FR-405-AC-3")]
fn every_operation_and_comparison_runs_and_cross_width_is_ill_typed_without_a_charge() {
    let (a, b) = (
        nf(IeeeWidth::Binary32, 0x3F80_0000),
        nf(IeeeWidth::Binary64, 0),
    );
    for operation in [
        NativeFloatOperation::Add(a, b),
        NativeFloatOperation::Subtract(a, b),
        NativeFloatOperation::Multiply(a, b),
        NativeFloatOperation::Divide(a, b),
        NativeFloatOperation::FusedMultiplyAdd(a, a, b),
        NativeFloatOperation::FusedMultiplyAdd(b, a, a),
    ] {
        let mut meter = generous();
        let before = meter.clone();
        assert_eq!(
            evaluate_native_float(operation, RoundingMode::NearestEven, &mut meter),
            Err(IllTyped {
                cause: IllTypedCause::DistinctIeeeWidths
            })
        );
        assert_eq!(meter, before);
    }
    for comparison in IeeeComparison::ALL {
        let mut meter = generous();
        let before = meter.clone();
        assert!(compare_native_float(comparison, a, b, &mut meter).is_err());
        assert_eq!(meter, before);
    }
    for width in WIDTHS {
        for operation in operations(width, [0x3F80_0000, 0x4000_0000, 0x4040_0000]) {
            let mut meter = generous();
            let outcome = evaluate_native_float(operation, RoundingMode::NearestEven, &mut meter);
            assert!(
                matches!(outcome, Ok(Outcome::Completed(_))),
                "{operation:?}"
            );
        }
        for comparison in IeeeComparison::ALL {
            let mut meter = generous();
            assert!(matches!(
                compare_native_float(comparison, nf(width, 1), nf(width, 2), &mut meter),
                Ok(Outcome::Completed(_))
            ));
        }
    }
}

// ---- AC-4, AC-5: independent literals -----------------------------------------------

fn run(operation: NativeFloatOperation, rounding: RoundingMode) -> Outcome<NativeFloatResult> {
    evaluate_native_float(operation, rounding, &mut generous()).expect("typed")
}

fn done(operation: NativeFloatOperation, rounding: RoundingMode) -> (u64, Vec<IeeeFlag>) {
    match run(operation, rounding) {
        Outcome::Completed(result) => (result.value.bits(), result.flags.iter().collect()),
        other => panic!("{operation:?} {rounding:?}: {other:?}"),
    }
}

fn b32(bits: u32) -> NativeFloat {
    NativeFloat::binary32(bits)
}

fn b64(bits: u64) -> NativeFloat {
    NativeFloat::binary64(bits)
}

use RoundingMode::{NearestAway, NearestEven, TowardNegative, TowardPositive, TowardZero};

#[test]
#[trace("TC-953", "FR-405-AC-4")]
fn tie_overflow_zero_sign_and_subnormal_results_equal_independent_literals() {
    use NativeFloatOperation::{Add, Divide, Multiply};
    // 1 + 2^-24 is the tie between 0x3F800000 (even) and 0x3F800001 (odd).
    let (one, tie) = (b32(0x3F80_0000), b32(0x3380_0000));
    assert_eq!(done(Add(one, tie), NearestEven).0, 0x3F80_0000);
    assert_eq!(done(Add(one, tie), NearestAway).0, 0x3F80_0001);
    assert_eq!(done(Add(one, tie), TowardZero).0, 0x3F80_0000);
    assert_eq!(done(Add(one, tie), TowardPositive).0, 0x3F80_0001);
    assert_eq!(done(Add(one, tie), TowardNegative).0, 0x3F80_0000);
    // 1 + 1.5 * 2^-23 ties between 0x3F800001 (odd) and 0x3F800002 (even).
    let upper = b32(0x3440_0000);
    assert_eq!(done(Add(one, upper), NearestEven).0, 0x3F80_0002);
    assert_eq!(done(Add(one, upper), NearestAway).0, 0x3F80_0002);
    // One unit off the tie in each direction.
    let below = b32(0x3380_0001);
    assert_eq!(done(Add(one, below), NearestEven).0, 0x3F80_0001);
    // Overflow: the largest finite plus itself, in each mode and sign.
    let max = b32(0x7F7F_FFFF);
    let negative_max = b32(0xFF7F_FFFF);
    for (mode, positive, negative) in [
        (NearestEven, 0x7F80_0000, 0xFF80_0000),
        (NearestAway, 0x7F80_0000, 0xFF80_0000),
        (TowardZero, 0x7F7F_FFFF, 0xFF7F_FFFF),
        (TowardPositive, 0x7F80_0000, 0xFF7F_FFFF),
        (TowardNegative, 0x7F7F_FFFF, 0xFF80_0000),
    ] {
        let (bits, flags) = done(Add(max, max), mode);
        assert_eq!(bits, positive, "{mode:?}");
        assert_eq!(flags, [IeeeFlag::Overflow, IeeeFlag::Inexact]);
        assert_eq!(done(Add(negative_max, negative_max), mode).0, negative);
    }
    assert!(done(Add(max, b32(0)), NearestEven).1.is_empty());
    // The exact-zero sum is +0 except toward negative.
    let minus_one = b32(0xBF80_0000);
    for mode in [NearestEven, NearestAway, TowardZero, TowardPositive] {
        assert_eq!(done(Add(one, minus_one), mode).0, 0);
    }
    assert_eq!(done(Add(one, minus_one), TowardNegative).0, 0x8000_0000);
    // The largest subnormal plus the smallest subnormal is the smallest
    // normal, exactly, without underflow.
    let (bits, flags) = done(Add(b32(0x007F_FFFF), b32(1)), NearestEven);
    assert_eq!((bits, flags), (0x0080_0000, vec![]));
    // A product below the subnormal range underflows to the sign of zero.
    let (bits, flags) = done(Multiply(b32(1), b32(0x3F00_0000)), NearestEven);
    assert_eq!(
        (bits, flags),
        (0, vec![IeeeFlag::Underflow, IeeeFlag::Inexact])
    );
    // Binary64: the same tie.
    let (one, tie) = (b64(0x3FF0_0000_0000_0000), b64(0x3CA0_0000_0000_0000));
    assert_eq!(done(Add(one, tie), NearestEven).0, 0x3FF0_0000_0000_0000);
    assert_eq!(done(Add(one, tie), NearestAway).0, 0x3FF0_0000_0000_0001);
    // Exact refuses with only the would-be flags: 1 / 3.
    match run(
        Divide(b32(0x3F80_0000), b32(0x4040_0000)),
        RoundingMode::Exact,
    ) {
        Outcome::Refused(Refusal::IeeeNotExact { target, would_be }) => {
            assert_eq!(target, IeeeWidth::Binary32);
            assert_eq!(
                would_be,
                [IeeeFlag::Inexact].into_iter().collect::<IeeeFlags>()
            );
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        done(Divide(b32(0x3F80_0000), b32(0x4040_0000)), NearestEven).0,
        0x3EAA_AAAB
    );
    assert_eq!(
        done(Divide(b32(0x3F80_0000), b32(0x4040_0000)), TowardZero).0,
        0x3EAA_AAAA
    );
}

#[test]
#[trace("TC-953", "FR-405-AC-5")]
fn flags_and_nan_results_equal_independent_literals() {
    use NativeFloatOperation::{
        Add, Divide, FusedMultiplyAdd, Multiply, Negate, SquareRoot, Subtract,
    };
    let (inf, minus_inf) = (b32(0x7F80_0000), b32(0xFF80_0000));
    let (zero, one) = (b32(0), b32(0x3F80_0000));
    assert_eq!(
        done(Subtract(inf, inf), NearestEven),
        (0x7FC0_0000, vec![IeeeFlag::Invalid])
    );
    assert_eq!(
        done(Add(inf, minus_inf), NearestEven),
        (0x7FC0_0000, vec![IeeeFlag::Invalid])
    );
    assert_eq!(
        done(Multiply(inf, zero), NearestEven),
        (0x7FC0_0000, vec![IeeeFlag::Invalid])
    );
    assert_eq!(
        done(Divide(zero, zero), NearestEven),
        (0x7FC0_0000, vec![IeeeFlag::Invalid])
    );
    assert_eq!(
        done(Divide(one, zero), NearestEven),
        (0x7F80_0000, vec![IeeeFlag::DivideByZero])
    );
    assert_eq!(
        done(Divide(b32(0xBF80_0000), zero), NearestEven).0,
        0xFF80_0000
    );
    assert_eq!(
        done(SquareRoot(b32(0xBF80_0000)), NearestEven),
        (0x7FC0_0000, vec![IeeeFlag::Invalid])
    );
    assert_eq!(
        done(SquareRoot(b32(0x8000_0000)), NearestEven),
        (0x8000_0000, vec![])
    );
    assert_eq!(
        done(SquareRoot(b32(0x4080_0000)), NearestEven),
        (0x4000_0000, vec![])
    );
    assert_eq!(
        done(SquareRoot(b32(0x4000_0000)), NearestEven),
        (0x3FB5_04F3, vec![IeeeFlag::Inexact])
    );
    // A signalling NaN is quieted with its payload and raises invalid; a
    // quiet NaN raises nothing; the leftmost NaN decides.
    assert_eq!(
        done(Add(b32(0x7F80_0001), one), NearestEven),
        (0x7FC0_0001, vec![IeeeFlag::Invalid])
    );
    assert_eq!(
        done(Add(b32(0xFFC0_0005), one), NearestEven),
        (0xFFC0_0005, vec![])
    );
    assert_eq!(
        done(Add(b32(0x7FC0_0002), b32(0x7F80_0003)), NearestEven),
        (0x7FC0_0002, vec![IeeeFlag::Invalid])
    );
    // Negation is bitwise, raises no flag and keeps a signalling NaN as is.
    assert_eq!(
        done(Negate(b32(0x7F80_0001)), NearestEven),
        (0xFF80_0001, vec![])
    );
    assert_eq!(done(Negate(zero), NearestEven), (0x8000_0000, vec![]));
    // An exact subnormal sum raises nothing.
    assert!(done(Add(b32(1), b32(1)), NearestEven).1.is_empty());
    // Fused multiply-add rounds once: (1 + 2^-23)^2 - (1 + 2^-22).
    let a = b32(0x3F80_0001);
    let c = b32(0xBF80_0002);
    assert_eq!(done(FusedMultiplyAdd(a, a, c), NearestEven).0, 0x2880_0000);
    assert_eq!(
        done(
            Add(b32(done(Multiply(a, a), NearestEven).0 as u32), c),
            NearestEven
        )
        .0,
        0
    );
}

// ---- AC-4, AC-5, AC-6, AC-10: the host differential -----------------------------------

#[test]
#[trace("TC-953", "FR-405-AC-4")]
#[trace("TC-953", "FR-405-AC-5")]
#[trace("TC-953", "FR-405-AC-10")]
fn differential_on_the_boundary_grid_in_every_rounding_spelling() {
    for width in WIDTHS {
        for operation in grid_operations(width) {
            for rounding in RoundingMode::ALL {
                agree(operation, rounding, limits(u64::MAX));
            }
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-4")]
#[trace("TC-953", "FR-405-AC-5")]
#[trace("TC-953", "FR-405-AC-10")]
fn differential_on_biased_random_patterns() {
    for width in WIDTHS {
        let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ u64::from(width.bits()));
        for _ in 0..40_000 {
            let operands = [rng.pattern(width), rng.pattern(width), rng.pattern(width)];
            for operation in operations(width, operands) {
                let rounding = RoundingMode::ALL[(rng.next() % 6) as usize];
                agree(operation, rounding, limits(u64::MAX));
            }
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-4")]
#[trace("TC-953", "FR-405-AC-10")]
fn differential_on_cancellation_and_ties_near_every_exponent() {
    for width in WIDTHS {
        let mut rng = Rng(0xD1B5_4A32_D192_ED03 ^ u64::from(width.bits()));
        let fraction_bits = if width == IeeeWidth::Binary32 { 23 } else { 52 };
        for _ in 0..20_000 {
            let a = rng.pattern(width);
            // Neighbours of `a`, its negation, and values that cancel against it.
            let near = |delta: i64| {
                (a as i64).wrapping_add(delta) as u64
                    & if width == IeeeWidth::Binary32 {
                        u64::from(u32::MAX)
                    } else {
                        u64::MAX
                    }
            };
            let others = [
                a ^ sign_bit(width),
                near(1),
                near(-1),
                near(1) ^ sign_bit(width),
                a & !((1 << fraction_bits) - 1),
            ];
            for b in others {
                for operation in [
                    NativeFloatOperation::Add(nf(width, a), nf(width, b)),
                    NativeFloatOperation::Subtract(nf(width, a), nf(width, b)),
                    NativeFloatOperation::Multiply(nf(width, a), nf(width, b)),
                    NativeFloatOperation::Divide(nf(width, a), nf(width, b)),
                    NativeFloatOperation::FusedMultiplyAdd(
                        nf(width, a),
                        nf(width, a),
                        nf(width, b),
                    ),
                    NativeFloatOperation::FusedMultiplyAdd(
                        nf(width, a),
                        nf(width, b),
                        nf(width, b ^ sign_bit(width)),
                    ),
                ] {
                    agree(
                        operation,
                        RoundingMode::ALL[(rng.next() % 6) as usize],
                        limits(u64::MAX),
                    );
                }
            }
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-6")]
fn charges_denials_and_incomplete_records_equal_the_owner_at_every_limit() {
    for width in WIDTHS {
        let grid = boundary(width);
        // Every boundary case: the full grid in the first two operands, the
        // third rotating through the grid, in every rounding spelling.
        for (i, &a) in grid.iter().enumerate() {
            for (j, &b) in grid.iter().enumerate() {
                let c = grid[(i + 2 * j + 1) % grid.len()];
                for operation in operations(width, [a, b, c]) {
                    for rounding in RoundingMode::ALL {
                        agree_at_limits(operation, rounding);
                    }
                }
            }
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-6")]
fn a_cumulative_meter_charges_like_the_owner_across_calls() {
    for width in WIDTHS {
        let cases = operations(width, [0x3F80_0000, 0x4040_0000, 0x3F80_0001]);
        let mut probe = generous();
        for operation in &cases {
            let _ = owner_outcome(*operation, NearestEven, &mut probe);
        }
        let total = probe.consumed(LimitKind::WorkUnits);
        for work in [u64::MAX, total, total - 1, total / 2, 3, 0] {
            let (mut native, mut owner) = (
                Meter::new(set(limits(u64::MAX), LimitKind::WorkUnits, work)),
                Meter::new(set(limits(u64::MAX), LimitKind::WorkUnits, work)),
            );
            for operation in &cases {
                assert_eq!(
                    evaluate_native_float(*operation, NearestEven, &mut native),
                    owner_outcome(*operation, NearestEven, &mut owner)
                );
            }
            assert_eq!(native, owner);
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-6")]
fn comparisons_equal_the_owner_with_charges_on_the_grid_and_at_limits() {
    for width in WIDTHS {
        let grid = boundary(width);
        for &a in &grid {
            for &b in &grid {
                for comparison in IeeeComparison::ALL {
                    for (kind, value) in [
                        (LimitKind::WorkUnits, u64::MAX),
                        (LimitKind::WorkUnits, 2),
                        (LimitKind::WorkUnits, 1),
                        (LimitKind::WorkUnits, 0),
                        (LimitKind::ResultUnits, 0),
                        (LimitKind::IntegerBits, u64::from(width.bits()) - 1),
                        (LimitKind::ValueOccurrences, 1),
                    ] {
                        let l = set(limits(u64::MAX), kind, value);
                        let (mut native, mut owner) = (Meter::new(l), Meter::new(l));
                        let n = compare_native_float(
                            comparison,
                            nf(width, a),
                            nf(width, b),
                            &mut native,
                        );
                        let o = compare_ieee(
                            comparison,
                            nf(width, a).to_ieee_value(),
                            nf(width, b).to_ieee_value(),
                            &mut owner,
                        );
                        assert_eq!(n, o, "{comparison:?} {a:#x} {b:#x} {kind:?}");
                        assert_eq!(native, owner);
                    }
                }
            }
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-3")]
#[trace("TC-953", "FR-405-AC-4")]
fn comparisons_equal_independent_literals() {
    let value =
        |comparison, x, y| match compare_native_float(comparison, b32(x), b32(y), &mut generous())
            .unwrap()
        {
            Outcome::Completed(value) => value,
            other => panic!("{other:?}"),
        };
    use IeeeComparison::{BitIdentical, NumericEqual, TotalOrder};
    assert!(value(NumericEqual, 0x8000_0000, 0));
    assert!(!value(NumericEqual, 0x7FC0_0000, 0x7FC0_0000));
    assert!(!value(BitIdentical, 0x8000_0000, 0));
    assert!(!value(BitIdentical, 0x7FC0_0000, 0x7FC0_0001));
    assert!(value(BitIdentical, 0x7FC0_0001, 0x7FC0_0001));
    assert!(value(TotalOrder, 0xFFC0_0000, 0x7FC0_0000));
    assert!(!value(TotalOrder, 0x7FC0_0000, 0xFFC0_0000));
    assert!(value(TotalOrder, 0x7FC0_0001, 0x7FC0_0002));
    assert!(!value(TotalOrder, 0x7FC0_0002, 0x7FC0_0001));
    // Negative NaNs order in reverse payload order.
    assert!(value(TotalOrder, 0xFFC0_0002, 0xFFC0_0001));
    assert!(!value(TotalOrder, 0xFFC0_0001, 0xFFC0_0002));
    assert!(value(TotalOrder, 0x8000_0000, 0));
    assert!(!value(TotalOrder, 0, 0x8000_0000));
    assert!(value(TotalOrder, 0x7F80_0000, 0x7F80_0001));
}

// ---- AC-8 ----------------------------------------------------------------------------------

// The `test-support` charge log allocates by design, so the window is measured
// on the production meter only.
#[cfg(not(feature = "test-support"))]
#[test]
#[trace("TC-953", "FR-405-AC-8")]
fn the_native_operations_allocate_nothing_over_the_boundary_grid() {
    let mut cases = Vec::new();
    for width in WIDTHS {
        cases.extend(grid_operations(width));
    }
    let mut meter = generous();
    let info = allocation_counter::measure(|| {
        for operation in &cases {
            for rounding in RoundingMode::ALL {
                let _ = evaluate_native_float(*operation, rounding, &mut meter);
            }
        }
        for width in WIDTHS {
            for comparison in IeeeComparison::ALL {
                for a in [0_u64, 1, 2] {
                    let _ =
                        compare_native_float(comparison, nf(width, a), nf(width, 2), &mut meter);
                }
            }
        }
    });
    assert_eq!(info.count_total, 0);
}

// ---- AC-10 (mutants), AC-11, AC-12, AC-13, AC-15: the evidence record ----------------------------

fn read_spec(path: &str) -> String {
    std::fs::read_to_string(format!("{}/{path}", env!("CARGO_MANIFEST_DIR")))
        .expect("spec readable")
}

const TC: &str = "spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md";
const GIB: u64 = 1 << 30;

/// (name in the spec, snake case in the harness id, rounds)
const OPERATIONS: [(&str, &str, bool); 10] = [
    ("Negate", "negate", false),
    ("Add", "add", true),
    ("Subtract", "subtract", true),
    ("Multiply", "multiply", true),
    ("Divide", "divide", true),
    ("SquareRoot", "square_root", true),
    ("FusedMultiplyAdd", "fused_multiply_add", true),
    ("NumericEqual", "numeric_equal", false),
    ("TotalOrder", "total_order", false),
    ("BitIdentical", "bit_identical", false),
];
const SPELLINGS: [&str; 3] = ["nearest_even", "exact", "directed"];

struct Row {
    operation: String,
    width: String,
    outcome: String,
    time: String,
    peak_cell: String,
    peak: u64,
    version: String,
    id: String,
}

/// The rows of a markdown table under `heading` in `text`: cells, trimmed of
/// backticks, with the header and separator rows dropped.
fn table_rows(text: &str, heading: &str) -> Vec<Vec<String>> {
    let section = text.split(heading).nth(1).expect("heading present");
    let section = section.split("\n#").next().unwrap();
    section
        .lines()
        .filter(|line| line.starts_with("| ") && !line.starts_with("| ---"))
        .skip(1)
        .map(|line| {
            line.trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().trim_matches('`').to_owned())
                .collect()
        })
        .collect()
}

/// One row per executed harness: operation, width, spelling, outcome, time,
/// peak, Kani version, harness id.
fn evidence_rows() -> Vec<Row> {
    table_rows(&read_spec(TC), "## Evidence record")
        .into_iter()
        .map(|cells| {
            assert_eq!(cells.len(), 8, "{cells:?}");
            Row {
                operation: cells[0].clone(),
                width: cells[1].clone(),
                outcome: cells[3].clone(),
                time: cells[4].clone(),
                peak_cell: cells[5].clone(),
                peak: cells[5]
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .expect("a byte count first"),
                version: cells[6].clone(),
                id: cells[7].clone(),
            }
        })
        .collect()
}

/// The harness ids that FR-405 Behavior 19 requires for one combination: one
/// for an operation that does not round, one per spelling group otherwise.
fn expected_ids(snake: &str, rounds: bool, width: &str) -> Vec<String> {
    let prefix = if width == "Binary32" {
        "float32"
    } else {
        "float64"
    };
    if rounds {
        SPELLINGS
            .iter()
            .map(|spelling| format!("native_float_kani_proofs::{prefix}_{snake}_{spelling}"))
            .collect()
    } else {
        vec![format!("native_float_kani_proofs::{prefix}_{snake}")]
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-11")]
#[trace("TC-953", "FR-405-AC-13")]
fn every_harness_of_every_operation_and_width_has_a_recorded_outcome_and_peak() {
    let rows = evidence_rows();
    let mut seen = 0;
    for width in ["Binary32", "Binary64"] {
        for (name, snake, rounds) in OPERATIONS {
            let mut ids: Vec<String> = rows
                .iter()
                .filter(|r| r.operation == name && r.width == width)
                .map(|r| r.id.clone())
                .collect();
            ids.sort();
            let mut expected = expected_ids(snake, rounds, width);
            expected.sort();
            assert_eq!(ids, expected, "{name} {width}");
            seen += ids.len();
        }
    }
    assert_eq!(seen, rows.len());
    for row in &rows {
        assert!(
            ["VERIFIED", "timeout", "memory exhaustion", "unreliable"]
                .contains(&row.outcome.as_str()),
            "{}",
            row.id
        );
        assert_eq!(row.version, "0.68.0", "{}", row.id);
        let above = row.peak > 16 * GIB;
        assert_eq!(
            row.peak_cell.contains("above the 16 GiB ceiling"),
            above,
            "{}: the row claims the ceiling status its number has",
            row.id
        );
        if row.outcome == "VERIFIED" {
            assert!(!above, "a VERIFIED peak is at most 16 GiB: {}", row.id);
            let seconds: f64 = row.time.trim_end_matches(" s").parse().expect("seconds");
            assert!(seconds < 600.0, "{}", row.id);
        } else if row.outcome == "timeout" {
            assert!(row.time.starts_with("600 s"), "{}", row.id);
        }
    }
}

#[test]
#[trace("TC-953", "FR-405-AC-15")]
fn the_status_records_a_verdict_or_table_row_for_every_operation_and_width() {
    let fr = read_spec("spec/functional/FR-405-kani-tractable-native-float-carrier.md");
    let status = fr
        .split("## Status")
        .nth(1)
        .and_then(|s| s.split("## Dependencies").next())
        .unwrap();
    let rows = evidence_rows();
    for (name, _, _) in OPERATIONS {
        let line = status
            .lines()
            .find(|line| line.starts_with(&format!("| {name} |")))
            .unwrap_or_else(|| panic!("{name} missing from the Status"));
        let cells: Vec<_> = line.trim_matches('|').split('|').map(str::trim).collect();
        for (cell, width) in cells[1..].iter().zip(["Binary32", "Binary64"]) {
            // A combination is VERIFIED only when all of its harnesses are.
            let mut outcomes: Vec<&str> = rows
                .iter()
                .filter(|r| r.operation == name && r.width == width && r.outcome != "VERIFIED")
                .map(|r| r.outcome.as_str())
                .collect();
            outcomes.sort_unstable();
            outcomes.dedup();
            let expected = if outcomes.is_empty() {
                "VERIFIED".to_owned()
            } else {
                format!("table row: {}", outcomes.join(" + "))
            };
            assert_eq!(*cell, expected, "{name} {width}");
        }
    }
    assert!(status.contains("No proof is claimed for a table-row combination"));
}

#[test]
#[trace("TC-953", "FR-405-AC-12")]
fn every_verified_harness_has_reached_covers_and_a_failing_harness_mutant() {
    let tc = read_spec(TC);
    let table = table_rows(&tc, "### Vacuity and harness mutants");
    let script = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/scripts/ir1446_native_float_mutants.py"
    ))
    .expect("mutant script readable");
    let verified: Vec<String> = evidence_rows()
        .into_iter()
        .filter(|r| r.outcome == "VERIFIED")
        .map(|r| r.id)
        .collect();
    let mut recorded: Vec<String> = table.iter().map(|cells| cells[0].clone()).collect();
    recorded.sort();
    let mut expected = verified.clone();
    expected.sort();
    assert_eq!(recorded, expected, "one vacuity row per VERIFIED harness");
    for cells in &table {
        assert_eq!(cells.len(), 4, "{cells:?}");
        // "k of k": every cover property of the harness was reached.
        let counts: Vec<&str> = cells[1].split(" of ").collect();
        assert_eq!(counts.len(), 2, "{cells:?}");
        assert!(
            counts[0] == counts[1] && counts[0].parse::<u32>().unwrap() > 0,
            "{cells:?}"
        );
        // The mutant is a named, re-runnable edit of the script, and it failed.
        assert!(script.contains(&format!("m(\"{}\"", cells[2])), "{cells:?}");
        assert_eq!(cells[3], "FAILED", "{cells:?}");
    }
}

#[test]
#[ignore = "builds 54 scratch copies of the crate; run explicitly"]
#[trace("TC-953", "FR-405-AC-10")]
fn every_mutant_of_the_native_bodies_fails_the_step_it_names() {
    let status = std::process::Command::new("python3")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/scripts/ir1446_native_float_mutants.py"
        ))
        .status()
        .expect("python3 runs");
    assert!(status.success());
}

#[test]
#[ignore = "runs Kani on a scratch copy per VERIFIED harness; run under the authorized runner"]
#[trace("TC-953", "FR-405-AC-12")]
fn every_harness_mutant_fails_its_harness_under_kani() {
    let status = std::process::Command::new("python3")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/scripts/ir1446_native_float_kani_mutants.py"
        ))
        .status()
        .expect("python3 runs");
    assert!(status.success());
}
