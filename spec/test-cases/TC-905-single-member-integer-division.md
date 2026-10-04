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

Verify FR-357-AC-1 to AC-5. It catches a `divide` that checks the unexposed member against the domain, that confuses a zero divisor with a refusal, that applies the wrong law to a negative operand, or that overflows at `i64::MIN / -1`.

Scope: FR-357-AC-1, FR-357-AC-2, FR-357-AC-3, FR-357-AC-4, FR-357-AC-5.

## Test Procedure

1. Divide by zero under each profile and member.
2. Evaluate `10 / 5` over `1..=10` (quotient) and `-10 % -1` over `-10..=5` (remainder).
3. Evaluate a quotient and a remainder outside a bounded domain, and read `Refusal::cause()`.
4. Evaluate -7 by 2 and -7 by -2 under each profile, both members.
5. Evaluate `i64::MIN / -1` over a wide and a narrow domain.

## Expected Results

Each step returns the value or refusal named in its acceptance criterion.

## Status

Implemented. The tests are in `src/division.rs`: `division_by_zero_is_undefined`, `only_the_exposed_member_must_be_in_domain`, `exposed_member_outside_domain_refuses_with_its_cause`, `profiles_differ_on_negative_operands`, `i64_min_divided_by_minus_one_is_exact`.
