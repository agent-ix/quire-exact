---
id: TC-950
title: "The native Decimal carrier matches the owner route and verifies without allocation"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: verifies
---
# TC-950: The native Decimal carrier matches the owner route and verifies without allocation

## Description

Verify FR-402-AC-1 through FR-402-AC-12 through the public kernel API, the host differential, the Kani proofs and the recorded measurement. Every mutant below is a real buildable variant of the shared body in a scratch copy. Each names the input, or the inspection, that fails it; a mutant that no input and no inspection distinguishes from the unmutated body is not admitted. This case is planned; no part of it has run.

## Test Procedure

The boundary set, used by steps 1 to 4, is fixed before the constructor is written and is derived from independent literals, not from the constructor. A binding is `(coefficient, scale)`, both `i64`. Types are written `Decimal[lo, hi; smin, smax]`; the rounding spelling is `Exact` throughout and is not exercised here.

- `Decimal[0, 100; 0, 2]`: `(5000, 2)`, `(10, 3)`, `(1, 3)`, `(10001, 2)`, `(9999, 2)`, `(100, 2)`, `(1, 0)`, `(1, 4294967295)`, `(10, 4294967295)`.
- `Decimal[-1000, 1000; 0, 2]`: `(0, 7)`, `(1000, 0)`, `(-1000, 0)`, `(1001, 0)`, `(-1001, 0)`, `(1, 4294967296)`, `(i64::MIN, 0)`, `(i64::MAX, 0)`.
- `Decimal[-9223372036854775808, 9223372036854775807; 0, 0]`: `(i64::MIN, 0)`, `(i64::MAX, 0)`.
- `Decimal[0, 100; 2, 3]`: `(1, 0)`, `(2, 0)`.
- `Decimal[0, 170141183460469231731687303715884105727; 38, 38]`: `(1, 0)`, `(2, 0)`. The same upper bound with `39, 39`: `(1, 0)`, `(0, 0)`. `Decimal[-170141183460469231731687303715884105728, 0; 38, 38]`: `(-1, 0)`, `(-2, 0)`.
- `Decimal[0, 100; 60, 70]`: `(1, 0)`, `(-1, 0)`, `(0, 0)`. `Decimal[-100, 0; 60, 70]`: `(-1, 0)`. `Decimal[0, 100; 128, 130]`: `(1, 0)`.
- `Decimal[0, 1; 0, 4294967295]`: `(1, 4294967295)`, `(0, 4294967295)`, `(2, 4294967295)`, `(1, -1)`.
- Normalized-view inputs with their expected literals (any carried Decimal, built by the total constructor or by a binding): `(0, 7)` to `(0, 0)`, `(-1000, 3)` to `(-1, 0)`, `(120, 5)` to `(12, 4)`, `(7, 3)` to `(7, 3)`, `(1000000000000000000, 18)` to `(1, 0)`, `(1000000000000000000, 19)` to `(1, 1)`, `(1000000000000000000, 5)` to `(10000000000000, 0)`, `(9000000000000000000, 20)` to `(9, 2)`, `(100, 2)` to `(1, 0)`, and with the total constructor `(10^37, 40)` to `(1, 3)`, `(-(10^38), 38)` to `(-1, 0)`, `(i128::MIN, 0)` to itself, `(i128::MIN, 4294967295)` to itself and `(10^38, 4294967295)` to `(1, 4294967257)`.
- Type-construction and ingress inputs: `new(1, 0, 0, 2)`, `new(0, 1, 3, 2)`, `new(0, 1, 0, 4294967296)`, `new(i128::MIN, i128::MAX, 0, 4294967295)`, `new(0, 0, 0, 0)`; owner types `Decimal[i128::MIN, i128::MAX; 0, 2]`, `Decimal[0, 2^127; 0, 2]`, `Decimal[-2^127 - 1, 0; 0, 2]`, `Decimal[0, 2^128 + 1; 0, 2]`.

1. For AC-1 to AC-5 and AC-12, build every type and construct every boundary binding with the public API. Read the retained coefficient through an `i128` accessor and the retained scale through a `u32` accessor for the AC-12 bindings. Check each retained and normalized pair against independently written literals and each refusal against its named cause. Check that `(100, 2)` and `(1, 0)` in `Decimal[0, 100; 0, 2]` are unequal under structural equality and equal in their normalized views.
2. For AC-6 and AC-8, build every boundary binding with scale in `0..=4294967295` a second time by the owner route: `Decimal::new(Integer::from(coefficient), scale)` and `DecimalType::contains` on an owner `DecimalType` built from the same bounds, and the owner's `normalized()`. Compare admission, the read-out `representation()` and the normalized pair. Exclude the bindings of scale `4294967296` and `-1` (their scale does not fit `Decimal::new`'s `u32`), which are checked in step 1 against `OutsideCarrierDomain` only. For the four owner types of the ingress inputs, assert at the level of the type that `DecimalType::new` succeeds for each and that the ingress succeeds for `Decimal[i128::MIN, i128::MAX; 0, 2]` and returns `TypeNotRepresentable` for the other three; those three are the complete list of disagreements, and no binding is built against them. Compare the native normalized view of each normalized-view input with the owner's `normalized()` of `Decimal::new(Integer::from(coefficient), scale)` as well as with its literal.
3. For AC-7, inspect the source of the constructor and the normalized view for a `cfg(kani)` token and for `for`, `while`, `loop` and iterator adapters, and the source of the constructor and of the normalized view for the owner types `Integer`, `Decimal` and `Rational` (matched as the owner paths `crate::integer::Integer`, `crate::decimal::Decimal` and `crate::rational::Rational`, or as a whole-word identifier, never as a substring of the carrier's own `NativeDecimal` names); build the crate for the host with default features; coerce the constructor and the normalized view to function pointers of `Meter`-free signatures; check `Copy` of the carried Decimal and the native type by a generic bound, and that neither has a field of an owner type; and run the whole boundary set and the normalized view of every normalized-view input inside a counting global allocator window, asserting zero allocations. The read-out and the ingress are called outside the window.
4. For AC-9, compile `compile_fail,E0308` programs that pass an `Integer`, a `Decimal` and a `DecimalType` to the constructor in place of the `i64` bindings and the native type, and a `compile_fail,E0451` program that builds the carried Decimal by struct literal; each has a compiling positive twin that uses the public constructor. The pinned error code makes an unrelated compile error fail the control.
5. For AC-10, run the two Kani proofs `native_decimal_from_bindings_refines_owner_membership` and `native_decimal_normalized_is_least_scale_pair` with no unwind attribute and their covers, under the repository's authorized runner, and run the host test that binds each independent reference to the owner route over the boundary set. The normalization reference checks the postcondition of FR-402 Behavior 9 by indexed lookup in a constant table of powers of ten, not by a loop. A proof is credited only on `VERIFIED` with the covers satisfied. The repository's standing instruction forbids running the Kani runners until the feature work is done: the step stays unexecuted until the implementation ticket's lead authorizes the runner.
6. Run each mutant of the next section: the input mutants through the checks of steps 1 and 2 by this rule: a mutant whose failing input has a literal expectation in the boundary set (retention, membership, lift, scale-cast, normalized-view, structural-equality, type-constructor and ingress mutants) is run through step 1; the `read-out normalizes` mutant is caught only by step 2 (the `representation()` comparison of AC-6); the inspection, `Copy`, function-pointer and allocation mutants are run through step 3.
7. For AC-11, once CG's decimal row (IR-259) consumes the delivered carrier, run it under the authorized runner and record the peak memory and its provenance, time, tool version and terminal outcome under IR-1444. Until then this step is not executable and AC-11 stays unexecuted; no fit is claimed in the meantime.

### Mutants

| Mutant | Fails at |
| --- | --- |
| The constructor retains the normalized pair | `(5000, 2)` in `Decimal[0, 100; 0, 2]`, expected retained `(5000, 2)` |
| The raw coefficient is compared with `[lo, hi]` | `(5000, 2)` in `Decimal[0, 100; 0, 2]`, expected admitted |
| The decimal value rather than the lifted normalized coefficient is compared with `[lo, hi]` | `(9999, 2)` in `Decimal[0, 100; 0, 2]`, expected `OutsideCarrierDomain` |
| The negative lift is a saturating multiply | `(-2, 0)` in `Decimal[i128::MIN, 0; 38, 38]` (saturates to `i128::MIN`, which is not below the lower bound), expected `OutsideCarrierDomain` |
| The raw scale is compared with `smax` | `(0, 7)` in `Decimal[-1000, 1000; 0, 2]`, expected admitted |
| The normalized scale is tested `< smax` | `(10, 3)` in `Decimal[0, 100; 0, 2]`, expected admitted |
| The `smin` lift is skipped | `(2, 0)` in `Decimal[0, 100; 2, 3]`, expected `OutsideCarrierDomain` |
| The lift is a wrapping multiply | `(1, 0)` in `Decimal[0, 100; 128, 130]` (`10^128` is 0 modulo `2^128`), expected `OutsideCarrierDomain` |
| The lift is a saturating multiply | `(2, 0)` in `Decimal[0, i128::MAX; 38, 38]` (saturates to `i128::MAX`, which is not above the upper bound), expected `OutsideCarrierDomain` |
| The lift treats a power of ten above `10^37` as oversize | `(1, 0)` in `Decimal[0, i128::MAX; 38, 38]`, expected admitted |
| An oversize positive lift is treated as inside | `(1, 0)` in `Decimal[0, 100; 60, 70]`, expected `OutsideCarrierDomain` |
| An oversize negative lift is treated as inside | `(-1, 0)` in `Decimal[-100, 0; 60, 70]`, expected `OutsideCarrierDomain` |
| The lower bound is off by one | `(-1000, 0)` admitted and `(-1001, 0)` refused in `Decimal[-1000, 1000; 0, 2]` |
| The upper bound is off by one | `(1000, 0)` admitted and `(1001, 0)` refused in the same type |
| The scale is cast to `u32` with `as` before the range check | `(1, 4294967296)` in `Decimal[-1000, 1000; 0, 2]` (truncates to 0, admitted) and `(1, -1)` in `Decimal[0, 1; 0, 4294967295]` (becomes 4294967295, admitted), expected `OutsideCarrierDomain` |
| The coefficient is widened through `as u64 as i128` | `(-1000, 0)` in `Decimal[-1000, 1000; 0, 2]`, expected admitted and retained `-1000` |
| Structural equality compares the normalized pair | `(100, 2)` against `(1, 0)`, expected unequal |
| The read-out normalizes | `(100, 2)`, expected read-out representation `(100, 2)` |
| The normalized view ignores the scale limit | `(1000000000000000000, 5)`, expected `(10000000000000, 0)` |
| The normalized view removes at most one zero | `(-1000, 3)`, expected `(-1, 0)` |
| The normalized view keeps the scale of zero | `(0, 7)`, expected `(0, 0)` |
| The normalized view strips zeros from the absolute value and returns it unsigned | `(-1000, 3)`, expected `(-1, 0)` (fails by value, independent of overflow checks) |
| The type ingress compares after a saturating cast | the owner type `Decimal[0, 2^127; 0, 2]`, expected `TypeNotRepresentable` |
| The type ingress compares after a wrapping `as` cast | the owner type `Decimal[0, 2^128 + 1; 0, 2]` (the bound wraps to 1), expected `TypeNotRepresentable` |
| The type ingress compares only the upper bound | the owner type `Decimal[-2^127 - 1, 0; 0, 2]`, expected `TypeNotRepresentable` |
| The type ingress refuses `i128::MIN` or `i128::MAX` | `Decimal[i128::MIN, i128::MAX; 0, 2]`, expected a native type |
| The type constructor accepts `lower > upper`, `min_scale > max_scale` or `max_scale > u32::MAX` | `new(1, 0, 0, 2)`, `new(0, 1, 3, 2)`, `new(0, 1, 0, 4294967296)`, each expected `MalformedDecimalType` |
| The normalized view is built on `Integer` or allocates | the owner-type scan and the allocation window of step 3, over `(1000000000000000000, 18)` and `(10^38, 4294967295)` (a Kani proof of it would only be inconclusive) |
| The carried Decimal holds an `Integer` or a heap type | the `Copy` check does not compile and the allocation counter is nonzero |
| A loop, fixed or data-bounded, strips the zeros of the normalized view | the source inspection of step 3 (a fixed-bound loop verifies under Kani, so only inspection catches it) |
| A `cfg(kani)`-selected body | the source inspection of step 3 (a host build never compiles that body) |
| A `Meter` parameter is added to the constructor | the function-pointer coercion of step 3 does not compile |

## Expected Results

Every boundary binding yields the independently written retained pair, normalized pair or named refusal. The constructor and the owner route agree on the whole set except the listed ingress refusals. Construction allocates nothing, the types are `Copy`, and the source inspection finds no `cfg(kani)`, loop, iterator adapter or heap type. The two proofs verify with no unwind bound and their covers. Every mutant fails as named. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass. The measurement of step 7 is a recorded number with its provenance once CG's decimal row consumes the carrier, and is never a prediction.

## Status

Planned. No implementation, test, proof or measurement has run.
