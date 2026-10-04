---
id: TC-905
title: "divide returns one member with typed outcomes"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-357
    type: verifies
---
# TC-905: divide returns one member with typed outcomes

## Description

Verify FR-357-AC-1 to AC-6. It catches a `divide` that checks the unexposed member against the domain, that confuses a zero divisor with a refusal, that applies the wrong law to a negative operand, that overflows at `i64::MIN div -1`, or that charges the wrong points or amounts.

Scope: FR-357-AC-1, FR-357-AC-2, FR-357-AC-3, FR-357-AC-4, FR-357-AC-5, FR-357-AC-6.

## Test Procedure

1. Divide by zero under each profile and member.
2. Evaluate `10 div 5` over `1..=10` and `-10 rem -1` over `-10..=5`.
3. Under each profile, evaluate a quotient and a remainder outside a bounded domain, and read `Refusal::code()` and `Refusal::cause()`.
4. Evaluate -7 by 2 and -7 by -2 under each profile, both members.
5. Evaluate `i64::MIN div -1` over a domain holding 2^63, then quotient and remainder in a signed 64-bit domain.
6. Read the admitted charge points and result units for a success, a refusal and a zero divisor, and divide with a result limit of 0.

## Expected Results

Each step returns the value or refusal named in its acceptance criterion.

## Status

Implemented. The tests are in `src/division.rs`: `division_by_zero_is_undefined`, `only_the_exposed_member_must_be_in_domain`, `exposed_member_outside_domain_refuses_with_its_cause`, `profiles_differ_on_negative_operands`, `i64_min_divided_by_minus_one_is_exact`, and (under `test-support`) `division_charges_one_domain_occurrence_and_one_result_unit`.
