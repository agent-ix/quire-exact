---
id: FR-408
title: "A Kani-tractable native Rational carrier and add, subtract, multiply, compare and equality refining the owner path"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-398
    type: references
  - target: ix://agent-ix/quire-exact/FR-403
    type: references
  - target: ix://agent-ix/quire-exact/FR-362
    type: references
  - target: ix://agent-ix/quire-exact/FR-375
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-046
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---
# FR-408: A Kani-tractable native Rational carrier and add, subtract, multiply, compare and equality refining the owner path

## Description

The kernel SHALL provide a native Rational carrier, a small `Copy` canonical fraction of two signed 128-bit integers with no heap type, and native `add`, `subtract`, `multiply`, ordering comparison and equality over two such carriers that refine the owner's arithmetic, ordering and equality path (Behavior 1 to 20).

The owner route, `Rational::new` then `Rational::add` through `Integer` multiply, `gcd` and `exact_div` and the `Meter`, did not finish under CBMC in more than two hours for numerators in `[-9, 9]`, denominators in `[1, 9]` and unwind 12 (measured, IR-258). It is the heap-shaped route that made the general `Integer` path intractable, so the owner route cannot be the Kani subject for the rational row. This requirement supplies the native expression, in the manner of [FR-402](./FR-402-kani-tractable-native-decimal-carrier.md) and [FR-403](./FR-403-native-decimal-operations-refine-owner-path.md) for Decimal, [FR-398](./FR-398-kani-tractable-carried-quantity-constructor.md) for the quantity magnitude and [FR-407](./FR-407-kani-tractable-native-enum-carrier.md) for enums. The owner's arithmetic, ordering and equality charges are [FR-362](./FR-362-exact-scalar-arithmetic-and-atom-charges.md) and [FR-375](./FR-375-composable-equality-effect-seam.md); this requirement adds no arithmetic, charge point or refusal for an operand the native path admits.

Two items own code that this requirement reuses and does not duplicate:

- **FR-398 owns the canonical-pair rule and its 128-step coprimality test** (Behavior 5 of FR-398: lowest terms, positive denominator, zero as `0/1`; the unrolled binary-gcd `coprime` over two `u64`). The native constructor from `i64` parts uses it. These exist only on open PR #162 (branch `code/ir-1443-quantity-carrier`), where `coprime` is private and the canonical checks are inlined in a private function mixed with the carrier range check. FR-408 code therefore depends on PR #162 landing.
- **FR-403 owns the fixed-step greatest-common-divisor sequence** (Behavior 16 of FR-403: exactly 256 binary-gcd steps over `u128` magnitudes with no data-dependent loop, `gcd256`). Native `add`, `subtract` and `multiply` reduce their results with it. FR-398's `u64` test is too narrow to reduce a sum, whose numerator reaches 127 bits, so the 256-step sequence is the one used. `gcd256` is `pub(super)` in the decimal operations module. The coder SHALL move the canonical-pair check (as its own function, separate from any range check), `coprime` and `gcd256` into ONE `pub(crate)` shared module that FR-398, FR-403 and FR-408 all call. They are moved, not copied.

The consumer is the rational function-predicate row of Contract CodeGen (CG FR-046, IR-258). CG's harness, its measurements and its credit are CG's and are not this requirement's acceptance (as for FR-402, FR-404 and FR-407). Kani execution here is limited to the proofs of Behavior 18, whose verdict the code lead records.

## Use case

CG's `rational_commutative(a: Rational[-9,9;1,9], b: Rational[-9,9;1,9]): Boolean` with body `a + b = b + a` draws each argument as a symbolic `i64` pair, builds a carrier with `NativeRational::from_i64_parts`, calls `native_rational_add` twice and `native_rational_equal` under a `Meter`, and requires the completed Boolean true. The wrong-addend mutant `a + b = a + a` fails with a counterexample.

## Inputs

- Two canonical carriers (or `i64` parts to build them), an operator and a `Meter`.
- No result domain: the first row passes none (the owner's optional `RationalDomain` is outside this requirement).

## Outputs

A carrier or a typed refusal; for an operation, the owner's `Outcome` over a carrier or a Boolean, or a typed capacity refusal that precedes any charge.

## Behavior

### Carrier

1. A `NativeRational` SHALL hold a numerator (`i128`) and a denominator (`i128`), SHALL be `Copy`, SHALL have no drop glue, SHALL hold no `Integer`, `Rational`, `BigInt`, `Arc`, `String`, `Box` or `Vec`, and SHALL keep its fields private. A value SHALL exist only if the denominator is positive and the pair is in lowest terms, with zero as `0/1`. Every public constructor and every operation result SHALL satisfy this, so no duplicate representation exists.
2. Structural equality of two carriers SHALL be equality of both fields and SHALL equal equality of the mathematical values.

### Construction

3. `NativeRational::from_i64_parts(numerator: i64, denominator: i64) -> Result<NativeRational, NativeRationalRefusal>` SHALL be total, take no `Meter` and charge nothing. `NativeRationalRefusal` is a `Copy` enum with the variants `ZeroDenominator`, `NonCanonical` and `Capacity(NativeRationalCapacity)` (Behavior 9). The checks run in the order `ZeroDenominator`, `NonCanonical`. A zero denominator SHALL be `ZeroDenominator`. A pair that is not canonical (a negative denominator, a common factor above one, or zero over a denominator other than one) SHALL be `NonCanonical`, judged on the raw pair by the canonical rule of FR-398. The constructor SHALL NOT normalize an input to make it admissible, so it never builds `1/2` from `2/4`. `i64::MIN` as a numerator is admitted when canonical; as a denominator it is `NonCanonical` (it is negative).
4. `NativeRational::from_rational(&Rational) -> Result<NativeRational, NativeRationalRefusal>` SHALL build the carrier from the owner's canonical `Rational` and SHALL return `Capacity(Input)` when a part does not fit signed 128 bits. It SHALL take no `Meter`. `to_rational(&self) -> Rational` SHALL yield the owner `Rational` equal under `Eq` to `Rational::new` of the same parts. The read-out may allocate; the construction of Behavior 3 SHALL NOT.
5. Accessors `numerator()` and `denominator()` SHALL return the parts as `i128`. The constructors, the accessors and the operations SHALL perform zero allocations.

### Operations

Notation. For operands `a/p` and `c/q`, `B(x)` is the bit length of `|x|` with `B(0) = 1`, as in FR-362.

6. `native_rational_add`, `native_rational_subtract` and `native_rational_multiply` SHALL each take `(&NativeRational, &NativeRational, &mut Meter)` and return `Result<Outcome<NativeRational>, NativeRationalRefusal>`. The unreduced result is, for add, `(a q + c p) / (p q)`, for subtract `(a q - c p) / (p q)` and for multiply `(a c) / (p q)`. The completed value SHALL be that fraction reduced by the 256-step gcd of FR-403 Behavior 16, the sign already positive in the denominator, zero as `0/1`. The value SHALL equal the owner's exact result.
7. Ordering `native_rational_order(operator, &left, &right, &mut Meter)` SHALL take the owner's `OrderingOperator` (`Less`, `LessOrEqual`, `Greater`, `GreaterOrEqual`) and return `Result<Outcome<bool>, NativeRationalRefusal>`. The Boolean is the comparison of `a q` with `c p`, exact for every pair that Behavior 9 admits.
8. `native_rational_equal(&left, &right, &mut Meter)` SHALL return `Outcome<bool>`. The Boolean is equality of the two carriers (Behavior 2). Equality SHALL NOT refuse for capacity, since it forms no product.
9. **Capacity.** The owner continues in arbitrary precision; the native path does not. Where a formed integer does not fit `i128`, the native operation SHALL return `Capacity(Quantity)` naming the first failing quantity, before any charge, with the `Meter` unchanged, and SHALL NOT wrap, saturate, clip or approximate. `NativeRationalCapacity` is a `Copy` enum with the variants `Input`, `LeftProduct` (`a q` or, for multiply, `a c`), `RightProduct` (`c p`), `Combination` (the sum or difference of the two products) and `Denominator` (`p q`). They are checked in that order. Subtract forms `a q - c p` as `checked_sub` of the two products, never by negating `c`, so a representable difference is never refused. A capacity refusal is not a completed, undefined, refused or incomplete outcome and not a proof result. For every operand pair with all four parts inside signed 64 bits, no operation of Behavior 6 or 7 is a capacity refusal, because each product is at most `2^126` in magnitude (`2^126 - 2^63` for a numerator-times-denominator product) and a sum or difference of two such products is at most `2^127 - 2^64`. Forming a sum never refuses in this region, but the sum's own parts may reach 127 bits, so a further operation on a sum can refuse. This region covers every value CG's first row draws and the sum of two of them.
10. The native path SHALL be undefined nowhere: add, subtract, multiply and the comparisons have no zero-divisor case, and divide is out of scope (Behavior 20).

### Charges

The charge points, order, limit kinds and amounts are the owner's (FR-362, FR-375) and are restated here so a native path cannot drift from them. A denied charge SHALL stop at that charge with the owner's incomplete record, a later charge SHALL NOT be made, and no result unit is consumed by a denied charge.

11. Add, subtract, multiply SHALL charge, in order: `rational-arithmetic.operands` with `integer_bits = max(B(a), B(p), B(c), B(q))` and `value_occurrences = 2`; `rational-arithmetic.arithmetic` with `integer_bits = max(N, D)` where for add and subtract `N = max(B(a) + B(q), B(c) + B(p)) + 1` and `D = B(p) + B(q)`, and for multiply `N = B(a) + B(c)` and `D = B(p) + B(q)`; `rational-arithmetic.normalize` with `integer_bits` the maximum actual bit length of the unreduced numerator and denominator (not the reduced parts); and `rational-arithmetic.result-retain` consuming one result unit. Every admitted point consumes one work unit. The amounts SHALL be computed from the operand parts by the formulas above, never from a different encoding of the same value.
12. Ordering SHALL charge `ordering.operands` with `integer_bits = max(B(a), B(p), B(c), B(q))` and `value_occurrences = 2`, then `ordering.arithmetic` with `integer_bits = max(B(a) + B(q), B(c) + B(p))`, then `ordering.result-retain`, in that order.
13. Equality SHALL charge the owner's equality schedule for two Rational scalar leaves (FR-375: `equality.plan-form`, `equality.plan`, one `equality.pair`, `equality.result-retain`, with the amounts FR-375 states for one planned pair) by calling the existing `charge_native_integer_schedule(NativeIntegerSchedule::Equality, ..)`, with no third copy of that schedule. The differential of Behavior 19 is the check of those literals.
14. Every operation SHALL charge and stop identically on one `Meter` shared across calls, so a sequence `a + b`, `b + a`, `=` accumulates the sum of the three schedules and a denial in the second call stops that call's prefix and leaves the first call's charges.

### Shape

15. The bodies of the constructor from `i64` parts, the three arithmetic operations, the ordering, the equality and the charge amounts SHALL contain no loop, iterator adapter, recursion or allocation, SHALL name no `Integer`, `Rational`, `BigInt` or `Vec`, SHALL contain no `cfg(kani)` token, and SHALL be one source for the host and the Kani build. A fixed-bound loop is not permitted (IR-1436 D4); the gcd is the unrolled sequence of FR-403.
16. The result, the refusal and the capacity cause SHALL be `Copy`.

### Proof evidence

17a. Before any Kani acceptance run of Behavior 18 is attempted, the code lead SHALL run one 600 s probe of the existing `gcd256` plus `u128` division over symbolic inputs bounded to 8 bits, and record its time, peak memory and terminal outcome. The step count is not parameterized: the shipped code runs 256 steps, so a smaller instance would prove code that does not ship. If the probe does not finish, the carrier SHALL use a width-tiered reduction: `u64` gcd and division when both unreduced magnitudes fit `u64`, otherwise `gcd256`. It is one source with an ordinary data branch, no `cfg(kani)`, and the same result for every input. Both branches are covered by the differential.

17. The refusal order of Behavior 3 and the capacity order of Behavior 9 SHALL be observable and distinct, so a harness can cover each.
18. A Kani harness SHALL prove, with a cover that the success path is reached and that each refusal path is reached, the following against an independent expected side written separately (arithmetic on `i128` with `checked_*`, a reduction by Euclid's algorithm in the harness, and the literal charge sequence): (a) `from_i64_parts` over all symbolic `i64` pairs with no unwind bound; (b) `native_rational_add`, `native_rational_subtract`, `native_rational_multiply`, `native_rational_order` for the four operators and `native_rational_equal`, over symbolic `i64` parts constrained to the CG row's domain (numerator in `[-9, 9]`, denominator in `[1, 9]`, canonical) and over symbolic parts drawn from the full `i64` domain where the measurement finishes, with a `Meter` of symbolic sufficient and insufficient limits; (c) the composed predicate `add(a, b) == add(b, a)` over the CG domain. Each harness SHALL fail on a mutant of the operation, SHALL run under the 10-minute wall clock and 16 GiB ceiling of CG FR-028 and SHALL return a terminal verdict. A timeout, memory exhaustion or unreliable outcome is recorded as the measured row and claims no proof. No Kani verdict is recorded by this requirement until the code lead records one. The host differential of Behavior 19 is not a proof over the full width, and the full-width rows (b) beyond the CG domain may not finish, as FR-403-AC-15 recorded for Decimal.

### Differential

19. A host differential SHALL run the native operations beside the owner route (`Rational::new` from `Integer`, `evaluate_rational_arithmetic` or `Rational::add`/`sub`/`mul`, `order_numbers` and `planned_equality`) for every ordered pair of canonical values with numerator in `[-9, 9]` and denominator in `[1, 9]` (111 values, 12,321 pairs) for every operation of Behavior 6 to 8, and over a boundary set that includes `i64::MIN`, `i64::MAX` parts, zero, one, minus one, pairs whose sum needs reduction, and FR-403's 252-step gcd pair `(2^126 - 1)/2^126` (built with `from_rational`, since its parts exceed `i64`) multiplied by `1/1`. It SHALL compare the typed outcome, the completed value (through `to_rational`), the Boolean, every charge point with its limit kinds and amounts, the denied charge, the counters and the incomplete record, with a `Meter` at a sufficient, an exact and a one-less limit for each limit kind the charges use. The one intended difference is capacity refusal (Behavior 9): the differential SHALL list the capacity inputs exhaustively, check each against its named quantity and the unchanged `Meter`, and assert that no native completion differs from the owner. The differential lives in quire-exact's `tests/`, because the owner route and the native path are both in quire-exact.
20. Out of scope: division, negation, `pow`, an optional `RationalDomain`, conversion of a carrier to an `IeeeFloat` (FR-373) and any operand form other than two carriers. CG FR-046 uses addition and equality only; subtract, multiply and the four orderings are included because they share the schedule of FR-362 and cost no further design, and CG consumes them when a later rational row needs them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-408-AC-1 | `NativeRational` is `Copy`, has no drop glue, holds no `Integer`, `Rational`, `BigInt`, `Arc`, `String`, `Box` or `Vec`, and has no public field (a struct-literal program fails with `E0451`); construction, accessors and all operations perform zero allocations under a counting allocator. | Test (TC-956) |
| FR-408-AC-2 | `from_i64_parts` admits `(0, 1)`, `(1, 1)`, `(-1, 1)`, `(1, 2)`, `(-1, 2)`, `(i64::MIN, 1)`, `(1, i64::MAX)` and `(i64::MIN, i64::MAX)`; refuses `(1, 0)` and `(0, 0)` with `ZeroDenominator`; refuses `(2, 4)`, `(1, -2)`, `(-1, -2)`, `(0, 2)`, `(0, -1)`, `(i64::MIN, -1)` and `(1, i64::MIN)` with `NonCanonical`; never builds a normalized value; takes no `Meter`; and `ZeroDenominator` takes precedence over `NonCanonical`. | Test (TC-956) |
| FR-408-AC-3 | `from_rational` and `to_rational` round-trip every canonical value of the CG domain and of the boundary set, equal under `Eq` to `Rational::new` of the same parts; a `Rational` with a part outside signed 128 bits (for example `2^127` over `1`) is `Capacity(Input)`; the canonical pair is unique, so `0` is only `0/1`. | Test (TC-956) |
| FR-408-AC-4 | Add, subtract and multiply complete to the exact reduced value: `1/2 + 1/3 = 5/6`, `1/2 + 1/2 = 1/1`, `1/2 - 1/2 = 0/1`, `-1/2 - 1/3 = -5/6`, `2/3 * 3/2 = 1/1`, `0/1 * 7/9 = 0/1`, `-9/1 + -9/1 = -18/1`, `9/1 + 9/1 = 18/1` (a sum outside the input domain), `i64::MAX/1 + i64::MAX/1 = (2^64 - 2)/1` and `(i64::MIN/1) * (i64::MIN/1) = 2^126/1`. The result of every operation is canonical (Behavior 1). | Test (TC-956) |
| FR-408-AC-5 | Ordering by the four operators equals the numeric order of the values: `1/3 < 1/2`, `1/2 <= 1/2`, `-1/2 > -2/3`, `7/3 >= 5/8`, with the equal pair true for `<=` and `>=` and false for `<` and `>`. Equality is true only for the identical canonical pair and false for `1/2` against `2/3`, `1/2` against `-1/2` and `0/1` against `1/1`. | Test (TC-956) |
| FR-408-AC-6 | Capacity. Inside signed 64 bits no operation is a capacity refusal. With parts that overflow `i128`, each refusal names its first failing quantity in the order of Behavior 9 (`LeftProduct`, `RightProduct`, `Combination`, `Denominator`) and leaves the `Meter` unchanged, including a `Meter` with a zero budget: for example `2^100/1 * 2^100/1` is `LeftProduct`, `1/2^100 + 1/3^80` is `Denominator`, the maximum numerator over `1` added to itself is `Combination`, and `-1/1 - i128::MIN/1` completes to `i128::MAX/1` (no refusal). Equality of any two carriers never refuses. | Test (TC-956) |
| FR-408-AC-7 | Charges. Against a literal expected sequence on a recording `Meter` (not the owner), `1/2 + 1/3` charges `rational-arithmetic.operands` (`integer_bits` 2, `value_occurrences` 2), `rational-arithmetic.arithmetic` (`integer_bits` 4: `N = max(1 + 2, 1 + 2) + 1 = 4`, `D = 2 + 2 = 4`), `rational-arithmetic.normalize` (`integer_bits` 3, the unreduced `5/6`) and `rational-arithmetic.result-retain`, consuming four work units and one result unit; the normalize amount uses the unreduced parts (`1/2 + 1/2` normalizes `4/4` at `integer_bits` 3, not the reduced `1/1` at 1). `9/1 + 9/1` charges `rational-arithmetic.arithmetic` with `integer_bits` 6 (`N = max(4 + 1, 4 + 1) + 1 = 6`, `D = 2`), which a missing `+ 1` would make 5. Ordering charges the three points of Behavior 12. At a sufficient, an exact and a one-less limit for each limit kind used, the denied charge, the admitted prefix, the counters and the incomplete record are as stated, and a denied charge consumes no result unit. | Test (TC-956) |
| FR-408-AC-8 | Equality charges the equality schedule of Behavior 13 and stops at the first denied point; a sequence `a + b`, `b + a`, `=` on one `Meter` accumulates the three schedules, and a denial inside the second addition leaves the first addition's charges (Behavior 14). | Test (TC-956) |
| FR-408-AC-9 | Host differential (Behavior 19): over the 12,321 pairs of the CG domain and the boundary set, the native path and the owner route agree on typed outcome, value, Boolean, every charge point, limit kind and amount, the denied charge, the counters and the incomplete record, at sufficient, exact and one-less limits, for add, subtract, multiply, the four orderings and equality. The only difference is the listed capacity set, each checked against its named quantity. Each mutant of TC-956 fails the differential at the step it names. | Test (TC-956) |
| FR-408-AC-10 | The constructor, arithmetic, ordering, equality and charge-amount bodies contain no loop, iterator adapter, recursion, allocation, `cfg(kani)` token, `Integer`, `Rational`, `BigInt` or `Vec`; the reduction calls the one `gcd256`, the canonical test calls the one of FR-398, and equality calls the one `charge_native_integer_schedule`, all in the shared `pub(crate)` module of the Description, with no second copy of any. | Inspection (code review) |
| FR-408-AC-11 | PLANNED: the Kani harnesses of Behavior 18 (a, b over the CG domain, c) return an executed terminal VERIFIED verdict, their covers are reached and each mutant fails, with time, peak memory, tool version and harness id recorded in TC-956's evidence record. A FAILED verdict fails the criterion; a timeout, memory exhaustion or unreliable outcome is the measured row and claims no proof. The full-`i64` rows of Behavior 18 (b) are recorded as measured and are not required to verify. | Test (TC-956) |

## Relationship to CG FR-046

CG FR-046 builds each Rational argument by `Rational::new(Integer, Integer)` with an accessor roundtrip, evaluates the body through the owner's exact operations and Meter, and draws two physical `i64` bindings per parameter. For the Kani row CG consumes this carrier instead: `NativeRational::from_i64_parts` as the representation constraint (it admits exactly the canonical pairs, so the roundtrip is `from_i64_parts` succeeding), `native_rational_add` and `native_rational_equal` as the generated body, and the values and charges of this requirement as the oracle in place of the owner's `Rational::add`. FR-046-AC-3 (body evaluation preserves the owner's operations, laziness and Meter order) is then a claim of equivalence that this requirement's differential (Behavior 19, FR-408-AC-9) backs on the host, and the Kani proof (FR-408-AC-11) backs inside the CG domain. CG owns the mapping from an admitted `Rational[min,max; dmin,dmax]` parameter to the two `i64` bindings, the refusal dispositions for a missing or non-`i64` domain, the replay path through `qsl_replay::WitnessValue::Rational`, the Meter entry state, and the FR-025 ABI. The kernel does not know the declared domain; a domain bound is a harness `assume`, not a kernel argument. A parameter whose domain exceeds signed 64 bits stays CG's `unsupported`. This requirement edits no CG specification. The row needs an amendment of CG FR-046 (its Behavior, FR-046-AC-2 and FR-046-AC-3, which today require the owner `Rational::new` roundtrip and the owner's exact operations) so that they call this native carrier instead. That amendment is a dependency of the row, delivered in the CG PR for IR-258 (no separate ticket); until it lands FR-046 and FR-408 disagree on the oracle.

**Limitations CG must carry.** The carrier does not model values outside signed 128 bits that the owner continues to compute. Inside signed 64 bits for both operands no capacity refusal exists (Behavior 9), which covers the first row; for the composed predicate over wider domains a capacity refusal is a typed cause, not a proof or a counterexample. The Kani proof over the CG domain is a bounded proof; it is not a universal 128-bit proof, which waits on the same Verus track as FR-403-AC-15.

## Status

Specified (IR-258). Not implemented; no Rust accompanies this requirement, no Kani harness has run and no measurement exists.

## Dependencies

- FR-398 owns the canonical-pair rule and the 128-step coprimality test; FR-403 owns the 256-step gcd sequence. FR-362 and FR-375 own the arithmetic, ordering and equality charge schedules that this requirement restates and the differential checks. FR-373 owns the Rational-to-IEEE outcome, which is out of scope here.
- CG FR-046 is the consumer and CG FR-028 owns the 16 GiB ceiling.
