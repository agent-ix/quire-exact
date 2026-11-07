---
id: FR-396
title: "Charge every limb block of integer parse, sum, product, exact quotient and gcd to a caller reservation"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-390
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
---
# FR-396: Charge every limb block of integer parse, sum, product, exact quotient and gcd to a caller reservation

## Description

When a consumer parses a signed decimal integer or adds, multiplies, exactly divides or takes the greatest common divisor of admitted `Integer` values under a memory budget, the exact kernel SHALL expose a variant of each operation that asks a caller `Reservation` for the full size of every limb block before requesting it from the allocator, and that returns the denial with no partial value. The consumer owns the budget, its locus and the refusal it reports; the kernel owns the amounts asked and their order. IR FR-038 ("Owner prerequisites of the budget-charged read") is the consuming requirement.

## Inputs

- A canonical decimal spelling, or borrowed `Integer` operands (a `NonZeroInteger` divisor for division), and a `&mut` value implementing `Reservation`: `fn reserve(&mut self, bytes: usize) -> Result<(), ReservationDenied>`. The trait and the unit `ReservationDenied` have the shape of `quire-canonical`'s, so one caller meter implements both; this leaf does not depend on that crate.

## Outputs

- `Integer::parse_reserved`, `add_reserved`, `mul_reserved`, `exact_div_reserved` and `gcd_reserved`, each returning the value or `IntegerReserveError::{Denied, Allocation { requested }}`; the parse returns `IntegerParseError::{NonCanonical, Reserve}`.

## Behavior

1. **Charge before request, in full.** Each limb block an operation requests is a `Vec<u32>` requested once with its exact capacity. The operation calls `reserve` once per block with that capacity in bytes (limb count times four, `usize::MAX` reported as `Allocation` if the product overflows), and requests the block only after `Ok`. Nothing is credited back when a block is released. The sum of the amounts asked therefore equals the bytes the operation requests from the allocator; a block that holds a temporary result later narrowed to an inline value is charged in full.
2. **Blocks per operation.** The hot loops read two limbs as one 64-bit digit, so a block that holds digits has an even limb count (a zero pad limb on top of an odd-length value). Parse of a spelling outside `i128`: one block of `even(ceil(ceil(10 * digits / 3) / 32))` limbs. Sum: one block of `max(left, right) + 1` limbs (`max(left, right)` for operands of opposite sign). Product: one block of `left + right` limbs, then, when the shorter factor has at least 64 limbs (Karatsuba), one scratch block of exactly the limbs the recursive split needs, asked after the product block. Exact division: a divisor work copy and a dividend work copy of one more digit and a quotient of `dividend - divisor + 1` digits, in that order (one-digit divisors: the quotient block only). Greatest common divisor: one even-length work copy of each operand (a single copy of the nonzero operand when the other is zero). A parse, sum, product, quotient or gcd whose operands and result fit `i128` requests nothing, as does a zero product, a quotient with magnitude below the divisor's and equal operands of opposite sign in a sum.
3. **Denial.** When `reserve` returns `ReservationDenied`, the operation requests no further block, releases those it holds and returns `IntegerReserveError::Denied` (or `IntegerParseError::Reserve(Denied)`) with no value. A failed allocator request after an admitted charge returns `Allocation { requested }` with the amount charged for that block. A non-canonical spelling is refused before any request.
4. **Allocation-free reads.** Comparison (`Ord`, `Eq`), `is_zero`, `is_negative` and `is_even` allocate nothing for every storage form, including a value held as limbs; they do not build the lazy `BigInt` image.
5. **One implementation.** The unmetered `Integer::add`, `mul`, `exact_div`, `gcd` and `FromStr` are these operations run with a reservation that admits every request, so the result and its limb storage are the reserved operation's. Every value outside `i128` is held as limbs, none as a bare `BigInt`; `as_big` builds the image on first use as it does for a copy, and `from_big` keeps the image it was given. The unmetered entries stay within 1.5 times `num-bigint`'s time at 300 and 3,000 digits for multiplication, exact division and parse, and within about 1.5 times for gcd, measured by `tests/ir1404_perf.rs` (ignored by default).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-396-AC-1 | For zero, both signs, the `i128` edges and the first values outside them, and multi-limb spellings, `parse_reserved` returns a value equal to an independent `BigInt` parse. The bytes recorded by the reservation equal the bytes the allocator observes around the call, with one request per block; a spelling inside `i128` requests nothing; a non-canonical spelling returns `NonCanonical` with no request. | Test (TC-943) |
| FR-396-AC-2 | For every pair of the AC-1 values, in both large storage forms, `add_reserved` equals the independent sum, charges exactly the bytes allocated, is admitted by a provision equal to the bytes charged, and for every smaller provision returns `Denied` having allocated exactly the bytes it admitted. | Test (TC-943) |
| FR-396-AC-3 | The same holds for `mul_reserved` against the independent product. | Test (TC-943) |
| FR-396-AC-4 | The same holds for `exact_div_reserved` of each product by each nonzero factor, against the other factor, across the one-digit and multi-digit divisor routes. | Test (TC-943) |
| FR-396-AC-5 | The same holds for `gcd_reserved` over multi-limb common factors, mixed signs, a zero operand, both zero and `gcd(i128::MIN, i128::MIN)` (outside `i128`), and the unmetered `gcd` returns the same values. | Test (TC-943) |
| FR-396-AC-6 | `Ord`, `Eq`, `is_zero`, `is_negative` and `is_even` over all AC-1 values in both large storage forms perform zero allocations, and `cmp` agrees with the independent order on adjacent values. | Test (TC-943) |
| FR-396-AC-7 | For every pair of AC-1 values the unmetered `add`, `mul` and `exact_div` equal the independent results and the reserved operation run with an unlimited provision. | Test (TC-943) |
| FR-396-AC-8 | For random decimal operands of 1 to 3,000 digits on both sides of every limb-pair, 64-limb Karatsuba and unbalanced-split boundary, and for runs of equal limbs (`2^k` and `2^k - 1`), sum, product, parse, exact quotient and gcd equal the independent `num-bigint` results; the charge equals the allocation, the exact provision admits, and a provision one byte short of each cumulative request refuses having allocated exactly the requests before it (the product's scratch block is one of those requests). | Test (TC-943) |

## Status

Implemented.

## Dependencies

- [FR-390](./FR-390-fallible-integer-copy-and-decimal-output.md) owns the fallible copy and decimal output that share the limb representation.
- IR FR-038 owns the budget, its read-storage locus and the consuming call sites.
