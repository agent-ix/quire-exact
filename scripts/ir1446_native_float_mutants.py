#!/usr/bin/env python3
"""Buildable mutants of the native float bodies (FR-405-AC-10, TC-953 Mutants).

Each mutant is an exact-text edit of a scratch copy of the crate (the working
tree is never touched), built and run against tests/ir1446_native_float.rs.
A mutant is admitted only if it compiles, its old text occurs exactly once, and
the named test fails; a survivor or a build failure exits nonzero.

Usage: python3 scripts/ir1446_native_float_mutants.py [mutant-id ...]
"""
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NF = "src/native_float.rs"
KP = "src/native_float/kani_proofs.rs"

# Step 3: bits, flags, provenance against the owner and the literals.
S3 = ["differential_on_the_boundary_grid", "differential_on_biased_random", "tie_overflow_zero_sign",
      "flags_and_nan_results", "differential_on_cancellation", "comparisons_equal"]
S4 = ["charges_denials_and_incomplete_records", "a_cumulative_meter_charges", "comparisons_equal_the_owner_with_charges"]
S2 = ["every_operation_and_comparison_runs_and_cross_width"]

M = []


def m(mid, site, old, new, tests, file=NF):
    M.append((mid, site, file, old, new, tests))


m("tie-even-is-away", "the tie rule of nearest-even is nearest-away",
  "above_half || (tie && kept & 1 == 1)", "above_half || tie", S3)
m("away-ties-to-even", "nearest-away rounds ties to even",
  "RoundingMode::NearestAway => !below_half,", "RoundingMode::NearestAway => above_half || (tie && kept & 1 == 1),", S3)
m("toward-zero-increments", "toward-zero increments the magnitude",
  "RoundingMode::TowardZero => false,\n        RoundingMode::TowardPositive => inexact", "RoundingMode::TowardZero => inexact,\n        RoundingMode::TowardPositive => inexact", S3)
m("directed-swapped", "toward-positive and toward-negative are swapped",
  "RoundingMode::TowardPositive => inexact && !negative,\n        RoundingMode::TowardNegative => inexact && negative,",
  "RoundingMode::TowardPositive => inexact && negative,\n        RoundingMode::TowardNegative => inexact && !negative,", S3)
m("sticky-dropped-in-alignment", "the sticky bit is dropped in alignment",
  "low.mag & ((1_u128 << distance) - 1) != 0,\n        )\n    };", "false,\n        )\n    };", S3)
m("subnormal-boundary", "the subnormal boundary is off by one", "if above > 0 {", "if above > 1 {", S3)
m("overflow-ignores-mode", "the overflow result ignores the mode",
  "RoundingMode::TowardZero => false,\n            RoundingMode::TowardPositive => !negative,\n            RoundingMode::TowardNegative => negative,",
  "RoundingMode::TowardZero => true,\n            RoundingMode::TowardPositive => true,\n            RoundingMode::TowardNegative => true,", S3)
m("overflow-flag-missing", "the overflow flag is not raised",
  "flags.with(IeeeFlag::Overflow).with(IeeeFlag::Inexact)", "flags.with(IeeeFlag::Inexact)", S3)
m("overflow-on-largest-finite", "the overflow flag is raised on the largest finite",
  "index + 1 >= fmt.exponent_ones()", "index + 2 >= fmt.exponent_ones()", S3)
m("zero-sum-sign", "an exact zero sum takes the wrong sign",
  "negative: rounding == RoundingMode::TowardNegative,", "negative: false,", S3)
m("nan-payload-cleared", "the NaN payload is cleared",
  "Err(Step::Decided(nan_bits | fmt.quiet_bit(), flags))", "Err(Step::Decided(fmt.canonical_nan(), flags))", S3)
m("nan-not-quieted", "the NaN is not quieted",
  "Err(Step::Decided(nan_bits | fmt.quiet_bit(), flags))", "Err(Step::Decided(nan_bits, flags))", S3)
m("nan-signalling-swapped", "signalling and quiet NaNs are swapped",
  "signaling: fraction & fmt.quiet_bit() == 0,", "signaling: fraction & fmt.quiet_bit() != 0,", S3)
m("inf-minus-inf-not-invalid", "invalid is not raised for infinity minus infinity",
  "decided(fmt.infinity(x))\n            } else {\n                invalid(fmt)", "decided(fmt.infinity(x))\n            } else {\n                decided(fmt.canonical_nan())", S3)
m("divide-by-zero-missing", "divide_by_zero is not raised for one over zero",
  "IeeeFlags::EMPTY.with(IeeeFlag::DivideByZero),", "IeeeFlags::EMPTY,", S3)
m("underflow-on-exact", "underflow is raised on an exact subnormal result", "if tiny && inexact {", "if tiny {", S3)
m("inexact-ignores-sticky", "inexact is not raised when only the sticky bit is set",
  "let inexact = remainder != 0 || sticky;", "let inexact = remainder != 0;", S3)
m("exact-returns-rounded", "Exact returns the rounded value",
  "if rounding == RoundingMode::Exact && !flags.is_empty() {", "if rounding == RoundingMode::Exact && !flags.is_empty() && false {", S3)
m("exact-refusal-drops-flags", "the Exact refusal drops the would-be flags",
  "would_be: flags,", "would_be: IeeeFlags::EMPTY,", S3)
m("sqrt-negative-not-invalid", "square root of a negative nonzero does not raise invalid",
  "        | Operand::Finite { negative: true, .. }\n        | Operand::Nan { .. } => invalid(fmt),", "        | Operand::Nan { .. } => invalid(fmt),", S3)
m("sqrt-negative-zero-sign", "square root of negative zero loses its sign",
  "Operand::Zero { negative } => Step::Finite(Exact::Zero { negative }),", "Operand::Zero { negative: _ } => Step::Finite(Exact::Zero { negative: false }),", S3)
m("sqrt-inexact-lost", "square root drops its sticky bit", "sticky: root * root != radicand,", "sticky: false,", S3)
m("sqrt-odd-exponent", "square root ignores an odd exponent", "if exponent & 1 != 0 {", "if false {", S3)
m("divide-sticky-lost", "divide drops its remainder", "sticky: !numerator.is_multiple_of(divisor),", "sticky: false,", S3)
m("fma-rounds-product", "fused multiply-add rounds the product before the addition",
  "(false, _) => Step::Finite(sum(rounding, product_term(a, b), term(c))),",
  "(false, _) => {\n            let p = product_term(a, b);\n            Step::Finite(sum(rounding, Term { mag: (p.mag >> 60) << 60, ..p }, term(c)))\n        }", S3)
m("subtract-jam-bias", "a jammed subtraction forgets the borrow",
  "high.mag - shifted - 1, true)", "high.mag - shifted, true)", S3)
m("shift-sticky-lost", "the rounding shift drops its lost bits",
  "mag & ((1_u128 << shift as u32) - 1) != 0,", "false,", S3)
m("numeric-equal-zeros", "NumericEqual treats zeros of opposite sign as unequal",
  "!nan && (both_zero || left.bits == right.bits)", "!nan && left.bits == right.bits", S3)
m("numeric-equal-nan", "NumericEqual treats a NaN as equal to itself",
  "!nan && (both_zero || left.bits == right.bits)", "both_zero || left.bits == right.bits", S3)
m("total-order-sign", "TotalOrder ignores the sign",
  "total_order_key(fmt, left.bits) <= total_order_key(fmt, right.bits)",
  "(left.bits & !fmt.sign_mask()) <= (right.bits & !fmt.sign_mask())", S3)
m("bit-identical-numeric", "BitIdentical compares numeric value",
  "IeeeComparison::BitIdentical => left.bits == right.bits,",
  "IeeeComparison::BitIdentical => left.bits == right.bits || ((left.bits | right.bits) & !fmt.sign_mask() == 0),", S3)
m("negate-quiets", "Negate quiets a NaN",
  "bits: value.bits ^ fmt.sign_mask(),", "bits: (value.bits ^ fmt.sign_mask()) | fmt.quiet_bit(),", S3)
m("provenance-rounding", "the provenance of a result is not the owner's",
  "flags: IeeeFlags::EMPTY,\n            rounding,\n        });", "flags: IeeeFlags::EMPTY,\n            rounding: RoundingMode::NearestEven,\n        });", S3)
m("no-exact-intermediate", "ieee.exact-intermediate is not charged on a finite plan",
  "            charge_width(meter, ChargePoint::IeeeExactIntermediate, fmt)?;\n", "", S4)
m("exact-intermediate-on-decided", "ieee.exact-intermediate is charged on a classification-decided input",
  "Step::Decided(bits, flags) => {\n            charge_result(meter)?;",
  "Step::Decided(bits, flags) => {\n            charge_width(meter, ChargePoint::IeeeExactIntermediate, fmt)?;\n            charge_result(meter)?;", S4)
m("no-round-charge", "ieee.round is not charged on a finite plan",
  "            charge_width(meter, ChargePoint::IeeeRound, fmt)?;\n", "", S4)
m("round-on-decided", "ieee.round is charged on a classification-decided input",
  "Step::Decided(bits, flags) => {\n            charge_result(meter)?;",
  "Step::Decided(bits, flags) => {\n            charge_width(meter, ChargePoint::IeeeRound, fmt)?;\n            charge_result(meter)?;", S4)
m("sqrt-operand-amount", "the operand amount for SquareRoot is not 1",
  "NativeFloatOperation::Negate(..) | NativeFloatOperation::SquareRoot(..) => 1,",
  "NativeFloatOperation::Negate(..) => 1,\n        NativeFloatOperation::SquareRoot(..) => 2,", S4)
m("fma-operand-amount", "the operand amount for FusedMultiplyAdd is not 3",
  "NativeFloatOperation::FusedMultiplyAdd(..) => 3,", "NativeFloatOperation::FusedMultiplyAdd(..) => 2,", S4)
m("operand-charge-dropped", "the operand charge is dropped",
  "    charge_operands(meter, fmt, arity)?;\n", "", S4)
m("operand-charge-twice", "the operand charge is charged twice",
  "    charge_operands(meter, fmt, arity)?;\n", "    charge_operands(meter, fmt, arity)?;\n    charge_operands(meter, fmt, arity)?;\n", S4)
m("result-charge-dropped", "the result charge is dropped on a finite plan",
  "            }\n            charge_result(meter)?;\n", "            }\n", S4)
m("result-charge-amount", "the result charge is at the wrong amount",
  ".results(1))?;", ".results(2))?;", S4)
m("ill-typed-charges", "an ill-typed call charges the Meter",
  "    same_width(left.width, right.width)?;\n    Ok(Outcome::from_stop(compare",
  "    let _ = meter.charge(Charge::new(ChargePoint::IeeeOperands));\n    same_width(left.width, right.width)?;\n    Ok(Outcome::from_stop(compare", S2)
m("charges-reordered", "the charges of ieee.exact-intermediate and ieee.round are reordered",
  "            charge_width(meter, ChargePoint::IeeeExactIntermediate, fmt)?;\n            charge_width(meter, ChargePoint::IeeeRound, fmt)?;\n",
  "            charge_width(meter, ChargePoint::IeeeRound, fmt)?;\n            charge_width(meter, ChargePoint::IeeeExactIntermediate, fmt)?;\n", S4)
m("denied-charge-then-later", "the denied charge is followed by a later charge",
  "            charge_width(meter, ChargePoint::IeeeRound, fmt)?;\n",
  "            let _ = charge_width(meter, ChargePoint::IeeeRound, fmt);\n", S4)
m("negate-raises-flag", "Negate raises a flag",
  "flags: IeeeFlags::EMPTY,\n            rounding,\n        });", "flags: IeeeFlags::EMPTY.with(IeeeFlag::Inexact),\n            rounding,\n        });", S3)
m("total-order-nan-payload", "TotalOrder ignores the payload of a NaN",
  "total_order_key(fmt, left.bits) <= total_order_key(fmt, right.bits)",
  "{ let clean = |b: u64| if matches!(decode(fmt, b), Operand::Nan { .. }) { (b & !fmt.fraction_mask()) | fmt.quiet_bit() } else { b }; total_order_key(fmt, clean(left.bits)) <= total_order_key(fmt, clean(right.bits)) }", S3)
m("zero-over-zero-divide-by-zero", "divide_by_zero is raised for zero over zero",
  "        (Operand::Infinite { .. }, Operand::Infinite { .. })\n        | (Operand::Zero { .. }, Operand::Zero { .. }) => invalid(fmt),",
  "        (Operand::Infinite { .. }, Operand::Infinite { .. }) => invalid(fmt),\n        (Operand::Zero { .. }, Operand::Zero { .. }) => Step::Decided(\n            fmt.canonical_nan(),\n            IeeeFlags::EMPTY.with(IeeeFlag::DivideByZero),\n        ),", S3)


def run(mutant, only):
    mid, site, file, old, new, tests = mutant
    work = tempfile.mkdtemp(prefix=f"ir1446-{mid}-")
    try:
        for entry in ["Cargo.toml", "Cargo.lock", "src", "tests", "clippy.toml", "rustfmt.toml", "rust-toolchain.toml"]:
            src = os.path.join(ROOT, entry)
            if os.path.isdir(src):
                shutil.copytree(src, os.path.join(work, entry))
            elif os.path.exists(src):
                shutil.copy(src, work)
        path = os.path.join(work, file)
        text = open(path).read()
        if text.count(old) != 1:
            return f"BAD-EDIT ({text.count(old)} occurrences)"
        open(path, "w").write(text.replace(old, new))
        env = dict(os.environ)
        env.setdefault("CARGO_TARGET_DIR", os.path.join(ROOT, "target", "mutants"))
        proc = subprocess.run(
            ["cargo", "test", "--features", "test-support", "--test", "ir1446_native_float", "--no-fail-fast"],
            cwd=work, env=env, capture_output=True, text=True)
        out = proc.stdout + proc.stderr
        if "error[" in out or "could not compile" in out:
            return "DOES-NOT-BUILD"
        failed = [line.split()[1] for line in out.splitlines() if line.startswith("test ") and line.endswith("FAILED")]
        hit = [f for f in failed if any(f.startswith(t) for t in tests)]
        return ("KILLED at " + hit[0]) if hit else f"SURVIVED (failed: {failed})"
    finally:
        shutil.rmtree(work, ignore_errors=True)


def main():
    only = set(sys.argv[1:])
    bad = 0
    for mutant in M:
        if only and mutant[0] not in only:
            continue
        verdict = run(mutant, only)
        print(f"{mutant[0]:34} {verdict}   [{mutant[1]}]", flush=True)
        bad += not verdict.startswith("KILLED")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
