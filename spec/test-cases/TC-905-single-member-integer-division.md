---
id: TC-905
title: "Division and modulo return exact typed outcomes"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-357
    type: verifies
---
# TC-905: Division and modulo return exact typed outcomes

## Description

Verify FR-357-AC-1 to AC-7. It catches a `divide` that checks the unexposed member against the domain, that confuses a zero divisor with a refusal, that applies the wrong law to a negative operand, that overflows at `i64::MIN div -1`, or that charges the wrong points or amounts.

Scope: FR-357-AC-1, FR-357-AC-2, FR-357-AC-3, FR-357-AC-4, FR-357-AC-5, FR-357-AC-6, FR-357-AC-7.

## Test Procedure

1. Divide by zero under each profile and member.
2. Evaluate `10 div 5` over `1..=10` and `-10 rem -1` over `-10..=5`.
3. Under each profile, evaluate a quotient and a remainder outside a bounded domain, and read `Refusal::code()` and `Refusal::cause()`.
4. Evaluate -7 by 2 and -7 by -2 under each profile, both members.
5. Evaluate `i64::MIN div -1` over a domain holding 2^63, then quotient and remainder in a signed 64-bit domain.
6. Read the admitted charge points and result units for a success, a refusal and a zero divisor, and divide with a result limit of 0.

7. Through the public `modulo` API, compute `(7, 3)`, `(7, -3)`, `(-7, 3)` and `(-7, -3)` on fresh sufficiently funded meters in the mathematical domain. For each `DivisionProfile`, first call `divide`, then call `modulo` using the same generously limited meter and repeat the four sign pairs. Compare exact values, not only signs.

## Expected Results

Steps 1 through 6 return the value or refusal named in their criteria. Step 7 returns 1, 1, 2 and 2 for the four sign pairs under every preceding division profile.

## Status

AC-1 through AC-6 are implemented; AC-7 is planned and has no executable binder. The existing tests are in `src/division.rs`: `division_by_zero_is_undefined`, `only_the_exposed_member_must_be_in_domain`, `exposed_member_outside_domain_refuses_with_its_cause`, `profiles_differ_on_negative_operands`, `i64_min_divided_by_minus_one_is_exact`, and (under `test-support`) `division_charges_one_domain_occurrence_and_one_result_unit`.
