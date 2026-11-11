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

Verify FR-398-AC-1 through FR-398-AC-10 through the public kernel API, the host differential against the shipped owner route, the Kani proofs and the recorded fit measurement. Every control below is a real buildable variant of the constructor in a scratch copy, and each names the input in the boundary set that makes it fail. A control that no input distinguishes from the unmutated constructor is not admitted. This case is planned; no part of it has run.

## Test Procedure

The boundary set, used by steps 1 to 5 and 7, is fixed before the constructor is written and is derived from independent literals, not from the constructor:

- `Int[-5, 100]`: `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1`, `i64::MAX` against carriers `Int[i64::MIN, i64::MAX]` (all admitted) and `Int[-5, 100]` for `-6`, `-5`, `100`, `101` (the first and last refused, the middle two admitted).
- `Decimal[-1000, 1000; 0, 2; Exact]`: `(10, 1)`, `(-5, 1)`, `(0, 7)`, `(1000, 0)`, `(100000, 2)`, `(100001, 2)`, `(i64::MIN, 0)`, `(i64::MAX, 0)`. `Decimal[0, 100; 0, 2; Exact]`: `(5000, 2)`, `(10001, 2)`, `(9999, 2)`, `(1, 3)`. `Decimal[0, 549755813888; 0, 4294967295; Exact]`: `(1, 37)`, `(1, 38)`, `(1, 39)`, `(1, 4294967295)`, `(549755813888, 39)` (that is 2^39 over 10^39, canonical `1/5^39`). `Decimal[0, 1; 0, 2; Exact]`: `(1, 39)` (outside the domain and not representable).
- `Rational[-9223372036854775808, 9223372036854775807; 1, 9223372036854775807]`: `(0, 1)`, `(1, 1)`, `(-1, 1)`, `(1, 2)`, `(2, 1)`, `(i64::MIN, 1)`, `(1, i64::MAX)`, `(i64::MIN, i64::MAX)`, and the non-canonical or zero pairs `(2, 4)`, `(1, -2)`, `(-1, -2)`, `(0, 2)`, `(1, 0)`, `(0, 0)`. `Rational[1, 5; 2, 4]`: `(1, 2)`, `(1, 3)`, `(5, 4)`, `(0, 1)`, `(6, 5)`, `(1, 5)`.
- Every carrier above is also presented with a binding of another carrier.

1. For AC-1, AC-2, AC-3 and AC-4, construct every boundary binding with the public constructor. Check the magnitude against independently written decimal literals as a numerator and denominator pair, the unit against the type's `UnitId`, and each refusal against its named cause. Include the order controls of AC-5: a pair that is `NonCanonical` and outside the domain, `Decimal[0,1;0,2]` `(1, 39)`, and the five `CarrierMismatch` presentations.
2. For AC-7, build every admitted boundary binding a second time by the shipped owner route (`Integer::from`, `Decimal::new` then `to_rational`, or `Rational::new`, then `Quantity::new`) and check admission with `IntegerInterval::contains`, `DecimalType::contains` and `RationalDomain::contains`. Assert equal magnitude, unit and admission, equal `Quantity` under `Eq`, `ZeroDenominator` for both on `(1, 0)` and `(0, 0)`, and `NonCanonical` from the constructor where `Rational::new` returns the reduced value. Assert that these are the only disagreements by counting the whole set.
3. For AC-6, build the crate for the host with default features and compile `Copy` and a no-`Meter`-parameter use as a static bound. Run the construction of the whole boundary set inside a counting global allocator window and assert zero allocations and unchanged counters of a `Meter` created beforehand. Inspect that no body is selected by `cfg(kani)`.
4. For AC-8, compile `compile_fail` programs that pass a `BoundedQuantityType`, a bare `UnitId` and a `Rational` to the constructor.
5. For AC-9, run the three Kani proofs with no unwind attribute and the cover, under the repository's authorized runner. Run the reference comparison on the host over the boundary set to bind the reference to the owner route. The proofs are credited only on `VERIFIED` with the cover satisfied.
6. Run each mutant of the next section, host differential first, then, for the loop and allocation mutants, the Kani proof. Each must fail at its named input; a mutant that passes is a defect in this case, not a result.
7. For AC-10, run CG's FR-050 composed harness against the delivered constructor on a Decimal, an Int and a Rational leaf under the authorized runner, and record the peak memory and its provenance, time, tool version and terminal outcome on IR-1443. If the harness cannot be built yet, record that the measurement is open and do not claim a fit.

### Mutants

Each is a buildable variant of the constructor and fails the stated input.

| Mutant | Failing input |
| --- | --- |
| `Decimal` ignores the scale (`c/1`) | `(10, 1)`, expected `1/1` |
| `Decimal` does not reduce (`c/10^s`) | `(10, 1)`, expected `1/1`, not `10/10` |
| `Decimal` fits by `scale <= 38` instead of the canonical fraction | `(549755813888, 39)`, expected admitted |
| `Decimal` fits by `scale <= 37` or `<= 39` | `(1, 38)` admitted, `(1, 39)` refused |
| `Decimal` checks the raw coefficient against `[lo, hi]` | `(5000, 2)` in `Decimal[0, 100; 0, 2]`, expected admitted |
| `Decimal` compares the value, not the normalized coefficient, with `[lo, hi]` | `(9999, 2)` in `Decimal[0, 100; 0, 2]`, expected `OutsideCarrierDomain` |
| `Decimal` drops the sign | `(-5, 1)`, expected `-1/2` |
| `Rational` swaps numerator and denominator | `(1, 2)`, expected `1/2` |
| `Rational` reduces instead of refusing | `(2, 4)`, expected `NonCanonical` |
| `Rational` accepts a negative denominator | `(1, -2)`, expected `NonCanonical` |
| `Rational` reports `NonCanonical` for a zero denominator | `(1, 0)`, expected `ZeroDenominator` |
| `Rational` checks only the numerator range | `(1, 3)` in `Rational[1, 5; 2, 4]` is admitted, `(1, 5)` and `(6, 5)` are refused |
| `Int` bound off by one at either end | `-6`, `-5`, `100`, `101` in `Int[-5, 100]` |
| `Int` negates `i64::MIN` through `abs` | `i64::MIN`, expected `-9223372036854775808/1` |
| Order reversed so the domain is checked before canonical form | the `NonCanonical`-and-outside pair of AC-5 |
| A loop over the scale digits to compute `10^s` | the Kani proof with no unwind bound fails to complete |
| The magnitude is held in a heap type | the `Copy` bound does not compile and the allocation counter is nonzero |
| A body selected by `cfg(kani)` that differs from the host body | the host differential on `(10, 1)` fails |

## Expected Results

Every boundary binding yields the independently written magnitude or the named refusal, and the constructor and the shipped owner route agree on the whole set except the `NonCanonical` pairs. Construction allocates nothing, charges nothing and is `Copy`. The three proofs verify with no unwind bound and their cover. Every mutant fails at its named input. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass. The measurement of step 7 is a recorded number with its provenance, or an explicit open status; it is never a prediction.

## Status

Planned. No implementation, test, proof or measurement has run.
