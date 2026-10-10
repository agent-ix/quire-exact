---
id: SR-5121
title: "spec-review/base review of IR-704 PR 21"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@7c073ba952953aaf80c536274f506802c800e05a; spec/functional/FR-370-distinct-native-uuid-and-timestamp-values.md, spec/test-cases/TC-918-distinct-native-uuid-and-timestamp-values.md"
review_set: subset
---

## Summary

Ticket: IR-704. Reviewed PR #21 at 7c073ba952953aaf80c536274f506802c800e05a. The merged QSL #690 and QSpec #201 contracts were the cross-repo baseline.

## Verdict

**FAIL** — One concrete test-procedure gap requires a fix.

## Examined scope

- `FR-370-AC-1` (examined): A UUID value is admitted only by `ValueType::Uuid` and a Timestamp value only by `ValueType::Timestamp`; crossed native kinds and Integer, Text and Reference values are rejected by both native types, without a conversion or alias.
- `FR-370-AC-2` (examined): Each same-kind native pair follows the existing one-terminal-pair equality schedule: an equal pair yields `true`, an unequal pair yields `false`, and the planned pair count and charges equal the corresponding Boolean-leaf schedule.
- `FR-370-AC-3` (examined): A direct equality plan pairing UUID or Timestamp with another value kind returns `Refusal::CheckedInvariant { cause: ValueKindMismatch }`, while a direct mixed-kind `compare_keys` call returns `None`; neither returns a Boolean or a false equal key.
- `FR-370-AC-4` (examined): `compare_keys` gives each native type a total key consistent with its equality relation and ordered by canonical payload ASCII bytes; Timestamp `"10"` precedes `"2"` and `"-1"` precedes `"-2"` in the key, independent of value construction or insertion order and without exposing a native ordering operator.
- `FR-370-AC-5` (examined): Existing scalar and composite admission, equality, key comparison and accounting vectors retain their prior results.
- `FR-370-AC-6` (examined): UUID values preserve exactly 16 network-order octets, including Nil, Max and non-RFC-version bit patterns; only the lowercase ASCII `8-4-4-4-12` caller payload admits, and uppercase, braces, URN form, altered grouping/separators, whitespace, length or nonhex characters refuse without repair or value.
- `FR-370-AC-7` (examined): Timestamp values preserve signed `i128` POSIX-epoch nanoseconds, including inclusive bounds `-170141183460469231731687303715884105728` and `170141183460469231731687303715884105727` and `-1`, `0`, `1` as one nanosecond before, at and after the epoch; only canonical signed base-10 caller payload admits, while either adjacent exterior count, `-0`, plus signs, leading zero, fraction, exponent, non-ASCII digits, calendar/offset text and whitespace refuse without repair or value.
- `FR-370-AC-8` (examined): Kernel constructors accept canonical UUID or Timestamp text supplied after the caller has validated its tagged wire envelope. Each malformed text returns `ConstructionRefusal` at `Component::Value` with its tabled cause, no value and no internal checked-invariant fault; canonical out-of-range decimal returns `TimestampOutOfDomain` after syntax checking. The kernel has no second parser-error type or UUID out-of-domain case.
- `TC-918` (examined): Verify FR-370-AC-1 through FR-370-AC-8 through public type-admission, equality-plan and canonical-key interfaces.
- `QSL FR-056-AC-17` (context_only): The canonical native controls admit UUID and Timestamp payloads, retain distinct identities and refuse the stated alternatives without normalization.
- `QSpec FR-149-AC-14..16` (context_only): Native equality uses one terminal pair, mixed kinds refuse, and keys compare canonical content unsigned ASCII bytewise.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-918 samples only four mixed-kind pairs although FR-370-AC-3 claims refusal for each native against every other value kind. | spec/test-cases/TC-918-distinct-native-uuid-and-timestamp-values.md:32 |

**FND-001 detail:** The criterion quantifies over UUID or Timestamp paired with another value kind; this procedure never pairs UUID with Integer/Text/Reference or Timestamp with Boolean/Reference, and does not check both operand orders. A defective mixed-kind arm can satisfy all four named examples while violating the stated criterion. Add a bounded table of native versus the existing kinds and both orientations, or narrow the criterion to the tested domain.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 706b616f5d473a9217ba23a9122641b5ed30d13c |

**FND-001 after excerpt:** For every tabled pair, call `plan_equality` and `compare_keys` with the native on the left and then with the operand order reversed. These are direct kernel precondition violations; QSL's checked mixed-type refusal remains at its caller boundary.
