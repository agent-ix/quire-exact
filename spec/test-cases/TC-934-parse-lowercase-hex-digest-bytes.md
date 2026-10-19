---
id: TC-934
title: "Plain lowercase hex digest byte parser"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-384
    type: verifies
---
# TC-934: Plain lowercase hex digest byte parser

## Description

Verify FR-384 through the crate-root public function. No identity or value
constructor participates in decoding.

## Test Procedure

1. Decode a mixed lowercase hex digest and compare every output byte with an
   independent expected array. Render a byte array with the existing
   lowercase `NodeKey` display encoder, then parse it back to bytes.
2. Offer 63 and 65 digits, uppercase, prefix, whitespace, nonhex ASCII,
   empty text, sign, and multibyte characters. Vary the position of a bad
   character across the whole 64-digit input.
3. Change the production length check to admit 65 digits and the charset
   check to admit `g`. Each mutation must make step 2 fail.

## Expected Results

- Step 1: exact bytes, with no constructed identity or value from parsing.
- Steps 2 and 3: every malformed input is refused; both mutations are caught.
