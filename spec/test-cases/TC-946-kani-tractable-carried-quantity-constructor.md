---
id: TC-946
title: "The carried-quantity constructor matches the owner route and verifies without allocation"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-398
    type: verifies
---
# TC-946: The carried-quantity constructor matches the owner route and verifies without allocation

## Description

Verify FR-398-AC-1 through FR-398-AC-11 through the public kernel API, the host differential, the Kani proofs and the recorded fit measurement. Every mutant below is a real buildable variant of the constructor in a scratch copy of the shared body. Each names the input, or the inspection, that fails it; a mutant that no input and no inspection distinguishes from the unmutated constructor is not admitted. This case is planned; no part of it has run.

## Test Procedure

The boundary set, used by steps 1, 2, 3 and 5, is fixed before the constructor is written and is derived from independent literals, not from the constructor. A binding is `(coefficient, scale)` for Decimal and `(numerator, denominator)` for Rational; a Decimal scale is an `i64`.

- `Int[i64::MIN, i64::MAX]`: `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1`, `i64::MAX`, all admitted. `Int[-5, 100]`: `-6` and `101` refused, `-5` and `100` admitted.
- `Decimal[-1000, 1000; 0, 2; Exact]`: `(10, 1)`, `(-5, 1)`, `(0, 7)`, `(1000, 0)`, `(-1000, 0)`, `(100000, 2)`, `(1001, 0)`, `(-1001, 0)`, `(i64::MIN, 0)`, `(i64::MAX, 0)`, and `(1, 4294967296)`. In the wide carrier below, `(1, -1)`. `Decimal[0, 100; 0, 2; Exact]`: `(5000, 2)`, `(10, 3)`, `(10001, 2)`, `(9999, 2)`, `(1, 3)`. `Decimal[0, 100; 2, 3; Exact]`: `(1, 0)`, `(2, 0)`. `Decimal[0, 100; 60, 70; Exact]`: `(1, 0)`, `(0, 0)`. `Decimal[0, 100; 128, 130; Exact]`: `(1, 0)`. `Decimal[-100, 0; 60, 70; Exact]`: `(-1, 0)`. `Decimal[0, 549755813888; 0, 4294967295; Exact]`: `(1, 37)`, `(1, 38)`, `(1, 39)`, `(1, 4294967295)`, `(549755813888, 39)` (2^39 over 10^39, canonical `1/5^39`). `Decimal[0, 1; 0, 2; Exact]`: `(1, 39)`.
- `Rational[-9223372036854775808, 9223372036854775807; 1, 9223372036854775807]`: `(0, 1)`, `(1, 1)`, `(-1, 1)`, `(1, 2)`, `(2, 1)`, `(i64::MIN, 1)`, `(1, i64::MAX)`, `(i64::MIN, i64::MAX)`, and the pairs `(2, 4)`, `(1, -2)`, `(-1, -2)`, `(0, 2)`, `(1, 0)`, `(0, 0)`. `Rational[2, 5; 3, 7]`: the four corners `(2, 3)`, `(2, 7)`, `(5, 3)`, `(5, 7)`, and the one-at-a-time violations `(1, 4)`, `(6, 7)`, `(3, 2)`, `(2, 9)`. `Rational[1, 5; 2, 4]`: `(6, 10)`.
- Types with a bound outside the native width: `Int[0, 2^127]` and `Int[0, 2^128 + 1]`, `Decimal[0, 2^127; 0, 2]` and `Decimal[0, 2^128 + 1; 0, 2]`, `Rational[1, 5; 2, 2^127]` and `Rational[1, 5; 2, 2^128 + 1]`, each against binding `1`, `(1, 0)` or `(1, 2)` as the carrier requires, and `Rational[1, 5; 2, 2^127]` also against `(1, 0)` and `(2, 4)`.
- The Decimal binding `(1, 0)` against `Int[0, 2^127]`, for the mismatch-before-type precedence.
- The six ordered mismatches: each carrier's binding presented with each of the other two carrier types.

1. For AC-1 to AC-6, construct every boundary binding with the public constructor. Check the magnitude against independently written numerator and denominator literals, and each refusal against its named cause, including the precedence pairs `Decimal` `(1, 0)` against `Int[0, 2^127]`, `Rational[1,5; 2,4]` `(6, 10)` and `Decimal[0,1; 0,2]` `(1, 39)`.
2. For AC-8, build every boundary binding a second time by the oracle of FR-398 item 8. Where the owner route runs (every binding except Decimal scale `4294967295`, and the scales `-1` and `4294967296` of the bindings `(1, -1)` and `(1, 4294967296)`, which `Decimal::new`'s `u32` cannot take and whose expected `OutsideCarrierDomain` is a literal), use `Integer::from`, `Decimal::new` then `to_rational`, or `Rational::new`, then `Quantity::new`, with `IntegerInterval::contains`, `DecimalType::contains` and `RationalDomain::contains`. For Decimal scale `4294967295`, which the owner route would materialize as a power of ten of about 1.7 GB, run `DecimalType::contains` and take the canonical fraction and its fit from the exponent reference, which counts the factors of 2 and 5 of the coefficient and never builds `10^s`; check the reference itself against the owner route for every Decimal binding of scale at most 64. Exclude the carrier-mismatched bindings (the six ordered mismatches and `Decimal` `(1, 0)` against `Int[0, 2^127]`), which have no owner-route result and are checked in step 1 against their literal `CarrierMismatch` only. Assert, over the remaining bindings, equal magnitude, unit and admission, equal `Quantity` under `Eq`, and that the complete list of disagreements is exactly three sets: the `NonCanonical` pairs `(2, 4)`, `(1, -2)`, `(-1, -2)`, `(0, 2)`, `(6, 10)`; the `NotRepresentable` bindings `(1, 39)` and `(1, 4294967295)` in `Decimal[0, 549755813888; 0, 4294967295]`; and every binding of the type's own carrier against the six types with a bound outside the native width (`TypeNotRepresentable`), including those the owner refuses, such as `(1, 0)` against `Rational[1, 5; 2, 2^127]`, where the owner returns `ZeroDenominator`. `Decimal[0,1; 0,2]` `(1, 39)` is not in the second set, because the owner refuses it.
3. For AC-7, inspect the constructor body's source for a `cfg(kani)` token, for `for`, `while`, `loop` and iterator adapters, and for a `Meter` parameter; build the crate for the host with default features; coerce the constructor to a function pointer of the `Meter`-free signature; check `Copy` and the absence of a `UnitId` field; and run the whole boundary set inside a counting global allocator window, asserting zero allocations. The read-out is called after the window.
4. For AC-9, compile three `compile_fail,E0308` programs that pass a `BoundedQuantityType`, a bare `UnitId` and a `Rational` to the constructor; the pinned error code makes an unrelated compile error fail the control.
5. For AC-10, run the three Kani proofs with no unwind attribute and the cover, under the repository's authorized runner, and run the host test that binds each reference to the owner route over the boundary set. A proof is credited only on `VERIFIED` with the cover satisfied.
6. Run each mutant of the next section: the input mutants through the checks of steps 1 and 2 (step 1 catches the refusal-cause and precedence mutants, including the carrier-mismatched binding, which step 2 excludes; step 2 catches the magnitude and admission mutants), the inspection mutants through step 3. Each must fail as named; a mutant that passes is a defect in this case, not a result.
7. For AC-11, once CG's FR-050 composed harness (IR-263) consumes the delivered constructor, run it on a Decimal, an Int and a Rational leaf under the authorized runner and record the peak memory and its provenance, time, tool version and terminal outcome under IR-1443. Until IR-263 delivers the harness this step is not executable and AC-11 stays unexecuted; no fit is claimed in the meantime.

### Mutants

| Mutant | Fails at |
| --- | --- |
| `Decimal` ignores the scale (`c/1`) | `(10, 1)`, expected `1/1` |
| `Decimal` does not reduce (`c/10^s`) | `(10, 1)`, expected `1/1`, not `10/10` |
| `Decimal` fits by `scale <= 38` instead of the canonical fraction | `(549755813888, 39)`, expected admitted |
| `Decimal` fits by `scale <= 37` | `(1, 38)`, expected admitted |
| `Decimal` fits by `scale <= 39` | `(1, 39)`, expected `NotRepresentable` |
| `Decimal` checks the raw coefficient against `[lo, hi]` | `(5000, 2)` in `Decimal[0, 100; 0, 2]`, expected admitted |
| `Decimal` compares the value, not the normalized coefficient, with `[lo, hi]` | `(9999, 2)` in `Decimal[0, 100; 0, 2]`, expected `OutsideCarrierDomain` |
| `Decimal` compares the unlifted coefficient (skips the `smin` lift) | `(2, 0)` in `Decimal[0, 100; 2, 3]`, expected `OutsideCarrierDomain` |
| `Decimal` lifts with a wrapping multiply to zero | `(1, 0)` in `Decimal[0, 100; 128, 130]` (10^128 is 0 modulo 2^128), expected `OutsideCarrierDomain` |
| `Decimal` treats an oversize lift as zero | `(1, 0)` in `Decimal[0, 100; 60, 70]` and `(-1, 0)` in `Decimal[-100, 0; 60, 70]`, expected `OutsideCarrierDomain` |
| `Decimal` tests `scale < smax` | `(10, 3)` in `Decimal[0, 100; 0, 2]` (normalized scale 2), expected admitted |
| `Decimal` tests the raw scale against `smax` | `(0, 7)` in `Decimal[-1000, 1000; 0, 2]`, expected admitted |
| `Decimal` drops the sign | `(-5, 1)`, expected `-1/2` |
| `Decimal` low bound off by one | `(-1000, 0)` admitted and `(-1001, 0)` refused in `Decimal[-1000, 1000; 0, 2]` |
| `Decimal` truncates the scale to `u32` | `(1, 4294967296)` in `Decimal[-1000, 1000; 0, 2]` and `(1, -1)` in `Decimal[0, 549755813888; 0, 4294967295]`, expected `OutsideCarrierDomain` |
| `Rational` swaps numerator and denominator | `(1, 2)`, expected `1/2` |
| `Rational` reduces instead of refusing | `(2, 4)`, expected `NonCanonical` |
| `Rational` accepts a negative denominator | `(1, -2)`, expected `NonCanonical` |
| `Rational` reports `NonCanonical` for a zero denominator | `(1, 0)`, expected `ZeroDenominator` |
| `Rational` omits one range bound | the single-violation pair for that bound in `Rational[2, 5; 3, 7]`: `(1, 4)`, `(6, 7)`, `(3, 2)` or `(2, 9)` |
| `Rational` refuses a corner | the corner pair of `Rational[2, 5; 3, 7]` that it refuses |
| `Int` bound off by one at either end | `-6`, `-5`, `100`, `101` in `Int[-5, 100]` |
| `Int` negates `i64::MIN` through `abs` | `i64::MIN`, expected `-9223372036854775808/1` |
| Canonical form judged after membership | `Rational[1, 5; 2, 4]` `(6, 10)`, expected `NonCanonical` |
| Representability judged before membership | `Decimal[0, 1; 0, 2]` `(1, 39)`, expected `OutsideCarrierDomain` |
| An oversize type bound is compared after a saturating cast | binding `1` against `Int[0, 2^127]`, expected `TypeNotRepresentable` (the mutant admits it) |
| An oversize type bound is compared after a wrapping `as` cast | binding `1` against `Int[0, 2^128 + 1]` (the bound wraps to 1), expected `TypeNotRepresentable` |
| Type representability judged before the carrier match | `(1, 0)` as a Decimal binding against `Int[0, 2^127]`, expected `CarrierMismatch` (the mutant reports `TypeNotRepresentable`) |
| Type representability judged after the binding checks | `(1, 0)` against `Rational[1, 5; 2, 2^127]`, expected `TypeNotRepresentable` (the mutant reports `ZeroDenominator`) |
| The magnitude is held in a heap type | the `Copy` check does not compile and the allocation counter is nonzero |
| The result carries a `UnitId` | the `UnitId`-free `Copy` check does not compile |
| A loop, fixed or binding-bounded, computes `10^s` | the source inspection of step 3 (a fixed-bound loop verifies under Kani, so only inspection catches it; a binding-bounded one also fails to complete in Kani, which is inconclusive, not the kill) |
| A `cfg(kani)`-selected body | the source inspection of step 3 (a host build never compiles that body, so only inspection catches it) |
| A `Meter` parameter is added | the function-pointer coercion of step 3 does not compile |

## Expected Results

Every boundary binding yields the independently written magnitude or the named refusal. The constructor and the oracle agree on the whole set except the three listed sets. Construction allocates nothing and is `Copy`, and the source inspection finds no `cfg(kani)`, loop or iterator adapter. The three proofs verify with no unwind bound and their cover. Every mutant fails as named. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass. The measurement of step 7 is a recorded number with its provenance once IR-263 delivers its harness, and is never a prediction.

## Status

Planned. No implementation, test, proof or measurement has run.
