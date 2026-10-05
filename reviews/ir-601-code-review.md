---
id: SR-004
title: "Code review of quire-exact PR #2: single-member integer division (IR-601)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@45c000f1e28f5b2443e636721b98ba2b8a5937f9; PR #2 diff against origin/main: src/division.rs, src/outcome.rs, src/lib.rs; compared against agent-ix/quire-specification PR #188 (IR-601, FR-147) head f0002b92"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-357
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-096
    type: reviews
---
# Code review of quire-exact PR #2

## Summary

Ticket: IR-601. PR: quire-exact#2. Rust lane (rust-review) folded in.

Checked and clean:
- `divide(profile, member, ..)` computes `(q, r)` exactly under the selected law and checks
  only the exposed member. The pair `divide`, `QuotientRemainder` and
  `Refusal::DivisionPairOutOfDomain` are gone with no shim, and nothing in the crate still
  names them.
- Charge order matches QSpec #188: operands, zero-divisor check, arithmetic, domain
  (`value_occurrences=1`), membership, result (`results(1)`). A refusal is raised after the
  domain charge and before the result charge, as #188's DIV-15 requires.
- The `(q, r)` laws are unchanged. `i64::MIN / -1` goes through `Integer` (bignum), so it
  cannot overflow.
- `refused_interval` on a mathematical domain returns `CheckedInvariant`. That cannot
  happen, because a mathematical domain holds every value.
- `modulo` (owner question 3) stays Euclidean-only with `ModuloOutOfDomain` /
  `modulo_out_of_domain` / `outside-domain`. QSpec #188's FR-147 table says the same:
  `mod` checks the Euclidean `r` and refuses with `modulo_out_of_domain, outside-domain`
  under every profile, and "cannot inherit a truncating or floor profile". Sending `mod`
  through `divide` would make it depend on the profile and give it the division code, which
  contradicts #188. It is consistent as written. No finding.
- Test oracles (owner question 4) are literal values worked out by hand, not echoes of the
  code. -7/2: trunc (-3,-1), floor (-4,1), Euclid (-4,1). -7/-2: trunc (3,-1),
  floor (3,-1), Euclid (4,1). I checked each one against `a = b*q + r` and its law. The
  FR-357-AC-2 vectors fail under the old pair check (remainder 0 is outside 1..=10, and
  quotient 10 is outside -10..=5), so the test tells member-only checking apart from pair
  checking. The AC-3 vectors put only the exposed member outside the domain.
- `make ci` exit=0 at the reviewed head, per the dispatching brief's log. No local build
  was needed for this review.

## Verdict

Request changes: two high findings and one medium. The member-only logic is correct. The
two strings a refusal record or an incomplete record carries do not match QSpec #188, which
is the language authority for both. The MIN/-1 test does not check the case that shows the
member-only rule.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The refusal code is `division_member_out_of_domain` (variant `DivisionMemberOutOfDomain`). QSpec #188 names it `division_out_of_domain` in FR-147, FR-271, FR-272, native-diagnostics.md and checked_package_v2.rs. The cause spellings match. Fix: align to #188. Rename the variant to `DivisionOutOfDomain` (so FR-096's `<name in snake case>` convention still holds) and the code to `division_out_of_domain`, and update the outcome.rs test table and the division.rs assert. | src/outcome.rs:135; src/outcome.rs:227; src/outcome.rs:377-404; src/division.rs:290 |
| FND-002 | high | The charge points stay `integer-division.domain-pair` and `integer-division.result-pair`, and each is now charged 1. QSpec owns the charge catalog (`quire.value.accounting/v1`, proposals/quire-v1/definitions/value-accounting.md). quire-exact only implements it in `ChargePoint::name()`. #188 renames them to `integer-division.domain` (`value_occurrences=1`) and `integer-division.result-retain` (`result_units += 1`). These names appear in `incomplete { charge_point: .. }` records, so the old names now break the spec and also say "pair" for a single member. Fix: align to #188. Rename the `ChargePoint` variants and their `name()` strings, and the doc comment in division.rs:132. The amounts already match. | src/accounting.rs:194-197; src/accounting.rs:379-380; src/division.rs:132; src/division.rs:153-162 |
| FND-003 | medium | `i64_min_divided_by_minus_one_is_exact` checks the narrow case as `matches!(narrow, Outcome::Refused(_))`, with no code or cause. It never runs the remainder of MIN/-1 in a signed-64 domain, which is the case that shows member-only checking (QSpec #188 DIV-05: the `rem` completes with 0 because the quotient is not checked). The narrow domain 0..=i64::MAX is not signed-64 either. Fix: use `range(i64::MIN, i64::MAX)`, assert the quotient refuses with cause `quotient-outside-domain`, and assert the remainder completes with 0 under each profile. | src/division.rs:325-346 |

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-exact@8137134a5aad2ac467c5905304fda644e9a13ea5 (fix range 45c000f1..8137134a).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The doc comment on `only_the_exposed_member_must_be_in_domain` still writes `x % -1` on the same line as the corrected `10 div y`. The rest of the change moved to `div`/`rem` (SR-006 FND-003). Fix: write `x rem -1`. | src/division.rs:262 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8137134a5aad2ac467c5905304fda644e9a13ea5 |
| FND-002 | fixed | 8137134a5aad2ac467c5905304fda644e9a13ea5 |
| FND-003 | fixed | 8137134a5aad2ac467c5905304fda644e9a13ea5 |
