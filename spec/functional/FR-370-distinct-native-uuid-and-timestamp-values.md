---
id: FR-370
title: "Keep UUID and Timestamp as distinct native kernel values"
type: FR
relationships:
  - target: ix://agent-ix/quire-spec-language/FR-056
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
---
# FR-370: Keep UUID and Timestamp as distinct native kernel values

## Description

When a caller supplies a native UUID or Timestamp value, the `quire-exact`
kernel SHALL retain its native type and value kind distinctly through type
admission, equality planning and canonical-key comparison. The caller owns the
mapping from its declared native type and input representation to this kernel
value. The kernel SHALL NOT treat either kind as an Integer, Text, Reference,
object identity, or the other native kind.

## Use case

A caller admits a UUID correlation value and a Timestamp occurrence value.
Equal UUIDs compare equal to UUIDs; equal Timestamps compare equal to
Timestamps. An `Int` cannot stand in for the occurrence Timestamp, even when
the integer happens to describe the same instant in a caller's domain.

## Behavior

1. **Distinct types.** `ValueType::Uuid` and `ValueType::Timestamp` are distinct
   public type variants. Their corresponding `Value` variants are distinct.
   `ValueType::admits` accepts each value only against its matching native
   type and rejects a crossed UUID/Timestamp pair or an Integer, Text or
   Reference offered as either native type.
2. **Native payloads.** A UUID is one opaque 128-bit value whose 16 octets are
   decoded most-significant/network octet first. Its sole admitted caller
   payload is lowercase ASCII hexadecimal grouped `8-4-4-4-12` with exactly
   four hyphens at those positions; every 128-bit pattern, including Nil and
   Max, is valid. Version and variant bits are retained uninterpreted. A
   Timestamp is one signed `i128` count of exact nanoseconds from
   `1970-01-01T00:00:00Z` (the POSIX epoch): one unit is `10^-9` POSIX second,
   with 86,400 seconds per day and no leap-second coordinate. The full signed
   `i128` range, from `-170141183460469231731687303715884105728` through
   `170141183460469231731687303715884105727` inclusive, is admitted. Its
   sole admitted caller payload is the canonical
   ASCII decimal spelling of that count: `0` for zero; otherwise an optional
   `-` followed by a nonzero digit and then zero or more digits. UUID and
   Timestamp payload admission SHALL reject a malformed or noncanonical
   payload without normalization, rounding, wrapping, floating-point
   conversion or a partially constructed value. Neither payload is converted
   through Integer or Text for kernel admission.
3. **Tagged caller wire and constructor refusal.** The selected wire form of a
   UUID is the closed
   object `{"kind":"uuid","text":"00112233-4455-6677-8899-aabbccddeeff"}`;
   the selected wire form of a Timestamp is the closed object
   `{"kind":"timestamp","nanoseconds":"0"}`. The enclosing caller
   serializer owns JSON member order and escaping. The caller diagnoses wire
   shape, tag, member and non-string errors before invoking the kernel's
   canonical-text constructor. The kernel's existing public
   `ConstructionRefusal { component: Component::Value, cause }` distinguishes
   the closed native causes below. Noncanonical decimal syntax is diagnosed
   before a canonical decimal outside the signed `i128` range. A native payload
   refusal SHALL NOT use the internal `CheckedInvariant` fault for an
   unreachable post-admission program state.

   | Native payload condition | `ConstructionCause` |
   | --- | --- |
   | UUID text violates lowercase ASCII `8-4-4-4-12` canonical syntax | `UuidNoncanonical` |
   | Timestamp text violates canonical signed ASCII decimal syntax | `TimestampNoncanonical` |
   | Canonical Timestamp decimal is outside signed `i128` | `TimestampOutOfDomain` |

   No `UuidOutOfDomain` case exists: every 128-bit pattern is admitted.
4. **Typed equality.** Two admitted UUID values and two admitted Timestamp
   values follow the kernel's existing equality-plan schedule: the pair is a
   terminal leaf and contributes one `equality.pair` event. Equal native
   values yield `true`; different native values yield `false`. No implicit
   conversion creates equality across the two kinds or against an existing
   value kind. Calling the kernel plan directly on unlike kinds returns the
   existing `CheckedInvariantCause::ValueKindMismatch` internal fault, never
   Boolean `false`. The caller's checked mixed-type equality refuses as
   `ill_typed`/`type-mismatch` before reaching this kernel plan; a direct
   mixed-kind plan is a checked-program invariant failure, not an ordinary
   caller-visible type error.
5. **Canonical key.** Each native kind has a total `compare_keys` order for
   values admitted by that same type, comparing the canonical encoded payload
   as unsigned ASCII bytes with a proper prefix first. For Timestamp this
   is representation order, so `"10"` sorts before `"2"`; it is not
   chronological order. Within each kind, two values have equal keys exactly
   when the typed equality plan says they are equal. A mixed-kind key
   comparison returns `None`. The key fixes collection rank only; it does
   not authorize a native ordering operator or timestamp arithmetic.
6. **Unchanged kinds.** Admission, equality, key comparison, and accounting
   for all existing `ValueType` and `Value` variants retain their current
   behavior.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-370-AC-1 | A UUID value is admitted only by `ValueType::Uuid` and a Timestamp value only by `ValueType::Timestamp`; crossed native kinds and Integer, Text and Reference values are rejected by both native types, without a conversion or alias. | Test |
| FR-370-AC-2 | Each same-kind native pair follows the existing one-terminal-pair equality schedule: an equal pair yields `true`, an unequal pair yields `false`, and the planned pair count and charges equal the corresponding Boolean-leaf schedule. | Test |
| FR-370-AC-3 | A direct equality plan pairing UUID or Timestamp with another value kind returns `Refusal::CheckedInvariant { cause: ValueKindMismatch }`, while a direct mixed-kind `compare_keys` call returns `None`; neither returns a Boolean or a false equal key. | Test |
| FR-370-AC-4 | `compare_keys` gives each native type a total key consistent with its equality relation and ordered by canonical payload ASCII bytes; Timestamp `"10"` precedes `"2"` and `"-1"` precedes `"-2"` in the key, independent of value construction or insertion order and without exposing a native ordering operator. | Test |
| FR-370-AC-5 | Existing scalar and composite admission, equality, key comparison and accounting vectors retain their prior results. | Test |
| FR-370-AC-6 | UUID values preserve exactly 16 network-order octets, including Nil, Max and non-RFC-version bit patterns; only the lowercase ASCII `8-4-4-4-12` caller payload admits, and uppercase, braces, URN form, altered grouping/separators, whitespace, length or nonhex characters refuse without repair or value. | Test |
| FR-370-AC-7 | Timestamp values preserve signed `i128` POSIX-epoch nanoseconds, including inclusive bounds `-170141183460469231731687303715884105728` and `170141183460469231731687303715884105727` and `-1`, `0`, `1` as one nanosecond before, at and after the epoch; only canonical signed base-10 caller payload admits, while either adjacent exterior count, `-0`, plus signs, leading zero, fraction, exponent, non-ASCII digits, calendar/offset text and whitespace refuse without repair or value. | Test |
| FR-370-AC-8 | Kernel constructors accept canonical UUID or Timestamp text supplied after the caller has validated its tagged wire envelope. Each malformed text returns `ConstructionRefusal` at `Component::Value` with its tabled cause, no value and no internal checked-invariant fault; canonical out-of-range decimal returns `TimestampOutOfDomain` after syntax checking. The kernel has no second parser-error type or UUID out-of-domain case. | Test |

## Status

Planned. The owning QSL and QSpec contracts are published; the kernel source
implementation and acceptance tests remain to be delivered.

## Dependencies

- QSL `ix://agent-ix/quire-spec-language/FR-056` defines the canonical UUID/Timestamp payload and closed tagged caller-wire forms; QSpec `ix://agent-ix/quire-specification/FR-149` owns their typed equality and key relation. QSpec `ix://agent-ix/quire-specification/FR-208` declares that an event occurrence field resolves to native `Timestamp`; QSpec `ix://agent-ix/quire-specification/TC-235` keeps `UUID`, `Timestamp`, and `Int` distinct in K2.
- QSL owns tagged-wire validation and maps this kernel's three typed constructor causes to `invalid_runtime_input`/`invalid-value` at the actual input path under QSpec `ix://agent-ix/quire-specification/FR-272`. QSL also owns K2's `Instant` to `Timestamp` binding and `Int` substitution refusal; this kernel owns the canonical payload constructors and native value operations. No shared catalogue code is added.
