---
id: TC-918
title: "Distinct native UUID and Timestamp values"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-370
    type: verifies
---
# TC-918: Distinct native UUID and Timestamp values

## Description

Verify [FR-370](../functional/FR-370-distinct-native-uuid-and-timestamp-values.md)
through the kernel's public type-admission, equality-plan and canonical-key
interfaces. Scope: FR-370-AC-1 through FR-370-AC-8.

## Test Procedure

1. Construct two equal and two different UUID values and Timestamp values
   using UUID octets `00000000-0000-0000-0000-000000000000`,
   `ffffffff-ffff-ffff-ffff-ffffffffffff` and a non-RFC-version pattern,
   and Timestamp nanoseconds `-1`, `0`, `1`,
   `-170141183460469231731687303715884105728` and
   `170141183460469231731687303715884105727`. Also offer each endpoint's
   adjacent exterior count as canonical decimal text.
   Admit each against both native types, then offer an Integer, Text and
   Reference value to both native types.
   Mutate UUID case, group widths, separator positions, length and hex
   characters, braces, URN form and whitespace; mutate Timestamp spelling
   with overflow, `-0`, `+`, a leading zero, empty text, a lone `-`, fraction,
   exponent, Unicode digit, calendar/offset form and whitespace. Also offer
   `0170141183460469231731687303715884105728`, which is both noncanonical
   and one above `i128::MAX`. Call the public canonical-text
   constructors directly and record the `ConstructionRefusal` component and
   cause for each rejected payload.
2. For each native kind, compare the equal and unequal pair with
   `plan_equality` and `planned_equality` under the same allowance used for a
   Boolean-leaf control pair. Observe the Boolean, pair count and charged
   schedule.
3. Use one constructed representative of each existing `Value` variant in
   this bounded kind table, not an enumeration of every payload or declared
   type. The direct call does not assert that Population is kernel-admitted or
   that Float has generic equality:

   | Native kind | Other value kinds |
   | --- | --- |
   | UUID | Timestamp; Boolean, Integer (`Integer` and `Int` share this value variant), Rational, Decimal, Float, Quantity, Text, Enum, Population, Option, Composite, Collection, Reference |
   | Timestamp | UUID; Boolean, Integer (`Integer` and `Int` share this value variant), Rational, Decimal, Float, Quantity, Text, Enum, Population, Option, Composite, Collection, Reference |

   For every tabled pair, call `plan_equality` and `compare_keys` with the
   native on the left and then with the operand order reversed. These are
   direct kernel precondition violations; QSL's checked mixed-type refusal
   remains at its caller boundary.
4. Compare the equal and unequal same-kind native pairs with `compare_keys`,
   including their reverse order. Compare Timestamp `10` against `2` and
   `-1` against `-2` to distinguish canonical-content key order from
   chronological order. Form a
   set in opposite insertion orders and inspect its visiting order and
   equality.
5. Repeat the existing Boolean, Integer, Text, Reference, Option and
   collection controls through their public admission, equality and key
   interfaces.

## Expected Results

- Step 1: each native value is admitted only by its own type; every crossed
  native kind and existing kind is rejected. Both timestamp bounds and all
  UUID bit patterns remain intact; every malformed UUID spelling returns
  `ConstructionRefusal { component: Value, cause: UuidNoncanonical }`, every
  malformed Timestamp spelling returns `TimestampNoncanonical`, and a
  canonical decimal beyond either `i128` bound returns
  `TimestampOutOfDomain`. Empty text, a lone minus and the leading-zero
  exterior value return `TimestampNoncanonical`; the latter establishes
  noncanonical-before-range precedence. All refusals produce no repaired or
  partially constructed value.
- Step 2: equal pairs yield `true` and unequal pairs `false`; each is one
  terminal pair with the Boolean-leaf schedule's pair count and charges.
- Step 3: in both operand orders, every tabled mixed equality pair returns
  the typed `ValueKindMismatch` checked-invariant fault, and every mixed key
  pair yields `None`; neither returns a Boolean or an equal key. The table
  covers value kinds rather than every admitted value of those kinds.
- Step 4: each same-kind key relation is total, reverses consistently, and
  returns `Equal` exactly for equal native values. The Timestamp key places
  `"10"` before `"2"` and `"-1"` before `"-2"` by unsigned ASCII byte order;
  set visiting order and equality do not depend on insertion order.
- Step 5: every existing control retains its prior result and accounting.

## Status

Planned. QSL's tagged-wire cases are outside this kernel test and are specified
by the published owning QSL/QSpec contracts.
