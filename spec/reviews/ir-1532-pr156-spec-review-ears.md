---
id: SR-17940486709564847047767
title: "IR-1532 PR #156 spec review (EARS conformance)"
type: SpecReview
analysis: ears-conformance
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #156 @ ad0c0c85; IR-1532; spec/functional/FR-406-kernel-version-constant.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-406
    type: reviews
---

## Summary

Ticket: IR-1532. EARS requirement-grammar check of the six FR-406 Behavior statements in PR #156 (head ad0c0c85). Behaviors 1, 2 and 5 are well-formed ubiquitous statements with the kernel as subject. Behaviors 3 and 6 are not system requirements in EARS form.

## Verdict

PASS with low findings: 2 low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Behavior 3's subject is "a change" and the obligation falls on the change's author, not the kernel; restate as an event-driven rule ("When a change alters ..., the change SHALL increase KERNEL_VERSION") or name the author as the responsible party explicitly | spec/functional/FR-406-kernel-version-constant.md:26 |
| FND-002 | low | Behavior 6 defers its content to the test case ("SHALL cover the cases named in the test case"), inverting the trace; the requirement should name the cases and the TC should verify them | spec/functional/FR-406-kernel-version-constant.md:29 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-406-B1 | examined | spec/functional/FR-406-kernel-version-constant.md:24 | The kernel SHALL export KERNEL_VERSION: u32 from the crate root, as a const available in a no_std build, with a doc comment that states the bump rule of Behavior 3. |
| FR-406-B2 | examined | spec/functional/FR-406-kernel-version-constant.md:25 | KERNEL_VERSION SHALL be a function of the source only, so two builds of the same source with any feature set export the same value |
| FR-406-B3 | examined | spec/functional/FR-406-kernel-version-constant.md:26 | A change that can change the result of any public kernel operation for some input ... SHALL bump KERNEL_VERSION by at least one. |
| FR-406-B4 | examined | spec/functional/FR-406-kernel-version-constant.md:27 | KERNEL_VERSION SHALL never decrease. |
| FR-406-B5 | examined | spec/functional/FR-406-kernel-version-constant.md:28 | The kernel SHALL carry a golden test that evaluates a fixed grid of public operations |
| FR-406-B6 | examined | spec/functional/FR-406-kernel-version-constant.md:29 | The golden grid SHALL cover the cases named in the test case, not every input. |
