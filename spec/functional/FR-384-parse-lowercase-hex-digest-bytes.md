---
id: FR-384
title: "Parse lowercase hex digest text into bytes without minting an identity"
type: FR
---
# FR-384: Parse lowercase hex digest text into bytes without minting an identity

## Description

The `quire-exact` public `parse_hex32(&str) -> Option<[u8; 32]>` SHALL decode
exactly 64 lowercase ASCII hexadecimal digits into 32 bytes. It SHALL return
`None` for every other length or character, without trimming, case folding,
or accepting a prefix. Parsing SHALL construct neither a `NodeKey` nor a
`Value`; admission and identity checks remain with the caller.

## Use case

A caller reads a digest spelling from an admitted package and needs its bytes
before applying the separate admitted-key decoding rule.

## Behavior

Each adjacent digit pair is one byte, high nibble first. The function reads
the UTF-8 input as bytes, accepts only `0` through `9` and `a` through `f`,
and returns exactly the decoded byte array. It works in the crate's
`no_std` plus `alloc` environment without allocating.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-384-AC-1 | The public function decodes 64 lowercase hex digits to the expected 32 bytes and recovers byte arrays rendered as lowercase hex; the result is only `[u8; 32]`. | Test |
| FR-384-AC-2 | Inputs of 63 or 65 digits, uppercase, `0x` prefix, leading or trailing whitespace, nonhex ASCII, empty text, sign, and multibyte characters all return `None`; mutations that weaken exact length or lowercase charset checks fail these tests. | Test |
