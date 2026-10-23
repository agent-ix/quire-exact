---
id: SR-16291
title: "spec-review of FR-385 distinct native Bytes and JsonObject kernel values (quire-exact PR 82)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@a31caf1e; spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md; spec/test-cases/TC-934-distinct-native-bytes-and-jsonobject-values.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-385
    type: references
  - target: ix://agent-ix/quire-exact/TC-934
    type: references
  - target: ix://agent-ix/quire-exact/FR-370
    type: references
---

# Spec review of FR-385 distinct native Bytes and JsonObject kernel values (quire-exact PR 82)

## Summary

Ticket: STD-418. One spec-review pass over `git diff origin/main...a31caf1e` (main is an ancestor). Gate: `quire validate --scope . 'spec/**/*.md'` exit 0 with no failure line; FR-385 and TC-934 validate alone. FR-385 and TC-934 are on no other open PR head (#76) and not on main; #82 merges cleanly with #76. Consistency with QSL PR 1369 (refs/rev/1369): the lowercase-hex Bytes payload, the RFC 8785 JsonObject payload, the rejected vectors (`00FF`, `0`, `0x00`, `00 ff`, ` 00`, `gg`; `{"b":1,"a":1}`, `{"a": 1}`, `{"a":1.0}`, `{"a":1e2}`, `{"a":-0}`, `{"a":9007199254740993}`, `{"a":1e999}`, duplicate member, empty, `{`) and 'nesting depth is not a refusal of its own' match FR-056-AC-46. QSpec names cited (FR-149 and FR-322 on main; the Bytes and JsonObject rows of FR-149 in #408) exist with the same spelling; the other QSpec PRs (#403 #405 #409 #419 #413 #418 #407 #410 #417) are not cited by this PR. The AC-3 vectors, the `9007199254740992` boundary and the accounting figures (four octets, seven bytes) were checked by hand and hold.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | AC-6 and TC-934 step 2 order Bytes `10` against `2`, but `2` is an odd-length payload and AC-2 refuses it as `BytesNoncanonical` (as it does `0`). No Bytes value `2` exists, so the vector cannot run. The `10` before `2` example is copied from FR-370's Timestamp decimal text. Fix: use admitted payloads, for example `10` against `0200`, or `0a` against `0a00` for the proper-prefix case. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md | wrong-requirement |
| FND-002 | medium | FR-385 names the causes `BytesNoncanonical` and `JsonObjectNoncanonical` (lead default). QSL PR 1369 FR-056 says 'IR-704 owns the Rust variant spellings' and names only the classes 'noncanonical Bytes payload' and 'noncanonical JsonObject payload'. Two owners for one spelling. Fix: align with IR-704 or cite that ticket as owner here. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md | wrong-requirement |
| FND-003 | medium | No AC-3 vector covers JCS string canonicalisation: an escaped character that JCS writes literally (`"\u00e9"`, `"\/"`), a control character written as lowercase `\u001f`, or UTF-16 member order for a non-BMP name. These are where a hand parser and RFC 8785 differ, and the Behavior text requires them. Fix: add accepted and refused string spellings to AC-3 and TC-934 step 1. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md | missing-requirement |

## Verdict

Not mergeable: FND-001 (high) block the PR; FND-002, FND-003 (medium) in the same round.

Merge: yes: origin/main is an ancestor; merge-tree with #76 clean.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-004 | medium | The accepted member vector is written `{"𐀀":1,"\uffff":2}` with the escape `\uffff`, while its parenthetical says the name is 'written as a literal character'. The same AC refuses `"\u00e9"` because JCS writes the literal, so a test author who copies the vector as printed builds a noncanonical payload and the accepted case fails. Fix: print the U+FFFF name as the literal character in the accepted vector, or state that the AC's `\uXXXX` in member names is notation. | spec/functional/FR-385-distinct-native-bytes-and-jsonobject-values.md | missing-requirement |

## Dispositions

Round 1 at fe6d84e4. Round 1 against fe6d84e4. Gate: `quire validate --scope . 'spec/**/*.md'` exit 0 with no failure line. The fix delta changes only FR-385 and TC-934. Bytes ordering and JCS / UTF-16 vectors were derived by hand (see Dispositions). Names: QSpec #408 FR-149 Bytes and JsonObject rows still exist with the cited spelling.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Hand-derived: `02` < `0a` (ASCII '2' 0x32 < 'a' 0x61), `0a` < `0a00` (proper prefix), `0200` < `10` ('0' < '1'); all are even-length lowercase payloads that AC-2 admits. |
| FND-002 | fixed | FR-385 now says the two spellings are this kernel's names for the classes FR-056 leaves to IR-704, and that IR-704 owns the final variant spellings. |
| FND-003 | fixed | Hand-derived against RFC 8785: literal `é`, `\n`, `\u001f` (lowercase), `\\`, `\"` and a literal `/` are canonical; `\u00e9`, `\/`, `\u001F`, `\u000a`, an escaped surrogate pair and a lone surrogate are not; U+10000 (D800 DC00) sorts before U+FFFF in UTF-16 order, so the code-point-ordered object is refused. |

Verdict after round 1: Mergeable after FND-004 (medium) is fixed.
