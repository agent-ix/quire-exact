---
id: SR-14160
title: IR-935 code-review of quire-exact PR 64
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; Cargo.toml, tests/fresh_unicode_consumer.rs, tests/fixtures/unicode17_consumer.rs;
  owner text boundary and external QSpec context
review_set: subset
---

## Summary

The owning dependency constraint preserves the existing Unicode 17 contract and no_std feature boundary. The actual fresh Cargo regression is meaningful, but its directory lifecycle and external trace qualification need correction.

Ticket: IR-935. Frozen head: c77706ba1e3cbb68085ab583c977ee1a4a4f2db3; base: aa63308958b7fa3a08142a24ce3694e94c28d62b. Runtime model: gpt-6.1-sol; run: ir935-655382c9-d124-4240-bff0-0a331bc7603c. Repo visibility is public; review comments are private Linear only. No source, branch or specification changes by reviewer.

## Verdict

**CONDITIONAL** — Same coder must fix the two actionable findings before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Fresh consumer builds are never removed; each successful run retains a separate target (108 MiB in the author green run), and eventual PID reuse makes create_dir fail. Use a uniquely claimed scratch directory with cleanup on success and failure while retaining diagnostic output. | tests/fresh_unicode_consumer.rs:10-17 |
| FND-002 | low | The new external FR-141 trace omits the documented QSpec- namespace. Use QSpec-FR-141-AC-1 so the tag preserves ownership consistently with src/lib.rs and existing external-requirement tests; do not add a local FR-141 copy. | tests/fresh_unicode_consumer.rs:6 |

## Coverage

Code-review ran with the Rust lane in this same artifact. All three diff files, unchanged table assertion/public normalization implementation, external FR-141-AC-1 and TC-186 context, and the existing FR-367 profile mismatch boundary were examined. No code or spec vendoring, extra dependency, consumer pin, local patch, initial lockfile, recursive owner dev-test invocation, public API change, unsafe addition, or gate weakening was introduced. The fixture is owned regression input exercising the public API, not copied upstream implementation. The nested target is separate and --jobs 1; ambient process environment is preserved except the child target override. Offline resolution relies on the normal local Cargo registry cache. The harness currently retains every private target and uses PID-only naming; FND-001 addresses both consequences of that lifecycle. No applicable AssuranceProfile is declared under this owner spec tree. No spec/plan diff: spec-review and sub-analyses not applicable.

## Verification and limitations

Reviewer performed static Rust inspection and Quire only. All Cargo/rustc/rustfmt/Kani execution is UNRUN by this reviewer under the no-heavy grant. The author raw logs show old-range fresh resolution selecting 0.1.26 and failing E0080, then new-range success with the retained fresh lock selecting 0.1.25. Author fmt/lint covers default, test-support and bare-metal no_std; author text/native units and profile-mismatch boundary pass in both feature modes. Aggregate make ci, deny, docs and full test suites await the lead after latest artifact custody. No CG native, negative-feature, Kani, mutation, proof or full IR-935 acceptance credit is claimed.

## Review data

```yaml
clean: false
scope:
- id: normalization-dependency
  path: Cargo.toml
  role: examined
  excerpt: '# The Unicode 17 text definition requires 0.1.25''s tables; 0.1.26 uses Unicode 18.

    unicode-normalization = { version = "=0.1.25", default-features = false }'
  lines: 31-32
- id: fresh-consumer-directory
  path: tests/fresh_unicode_consumer.rs
  role: examined
  excerpt: "    let owner = Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    let directory = owner\n        .join(\"\
    target\")\n        .join(format!(\"fresh-unicode-consumer-{}\", std::process::id()));\n    // A new directory,\
    \ rather than the owner's lockfile, supplies the resolution.\n    fs::create_dir_all(directory.parent().unwrap()).unwrap();\n\
    \    fs::create_dir(&directory).expect(\"fresh consumer directory must not already exist\");\n    fs::create_dir(directory.join(\"\
    src\")).unwrap();"
  lines: 10-17
- id: fresh-consumer-manifest
  path: tests/fresh_unicode_consumer.rs
  role: examined
  excerpt: "    fs::write(\n        directory.join(\"Cargo.toml\"),\n        format!(\n            \"[package]\\\
    nname = \\\"fresh-unicode-consumer\\\"\\nversion = \\\"0.0.0\\\"\\nedition = \\\"2021\\\"\\n\\\n           \
    \  [workspace]\\n\\\n             [dependencies]\\nquire-exact = {{ path = {owner:?} }}\\n\"\n        ),\n \
    \   )\n    .unwrap();\n    fs::write(\n        directory.join(\"src/main.rs\"),\n        include_str!(\"fixtures/unicode17_consumer.rs\"\
    ),\n    )\n    .unwrap();\n    assert!(!directory.join(\"Cargo.lock\").exists());\n\n    // A separate target\
    \ avoids recursively locking the outer cargo test target."
  lines: 18-34
- id: fresh-consumer-cargo-command
  path: tests/fresh_unicode_consumer.rs
  role: examined
  excerpt: "    let output = Command::new(env!(\"CARGO\"))\n        .current_dir(&directory)\n        .args([\"\
    run\", \"--offline\", \"--jobs\", \"1\"])\n        .env(\"CARGO_TARGET_DIR\", directory.join(\"target\"))\n\
    \        .output()\n        .expect(\"run the real consumer through Cargo\");\n    assert!(\n        output.status.success(),\n\
    \        \"fresh Unicode 17 consumer failed: {}\\nstdout:\\n{}\\nstderr:\\n{}\",\n        output.status,\n \
    \       String::from_utf8_lossy(&output.stdout),\n        String::from_utf8_lossy(&output.stderr),\n    );\n\
    \    assert!(directory.join(\"Cargo.lock\").is_file());\n}"
  lines: 36-50
- id: fresh-consumer-trace
  path: tests/fresh_unicode_consumer.rs
  role: examined
  excerpt: '/// Trace: FR-141-AC-1'
  lines: 6-6
- id: unicode17-consumer-identity
  path: tests/fixtures/unicode17_consumer.rs
  role: examined
  excerpt: "fn main() {\n    assert_eq!(UNICODE_VERSION, (17, 0, 0));\n    assert_eq!(\n        UNICODE_TEXT_DEFINITION,\n\
    \        \"quire.value.text.unicode-17.0.0/v1\"\n    );"
  lines: 6-11
- id: unicode17-consumer-normalization-vectors
  path: tests/fixtures/unicode17_consumer.rs
  role: examined
  excerpt: "    for (profile, form, input, expected) in [\n        (TextProfile::Nfc, NormalizationForm::Nfc, \"\
    e\\u{301}\", \"é\"),\n        (TextProfile::Nfd, NormalizationForm::Nfd, \"é\", \"e\\u{301}\"),\n        (\n\
    \            TextProfile::Nfkc,\n            NormalizationForm::Nfkc,\n            \"\\u{FB03}\",\n        \
    \    \"ffi\",\n        ),\n        (\n            TextProfile::Nfkd,\n            NormalizationForm::Nfkd,\n\
    \            \"\\u{212B}\",\n            \"A\\u{30A}\",\n        ),\n    ] {"
  lines: 12-27
- id: unicode17-consumer-assertions
  path: tests/fixtures/unicode17_consumer.rs
  role: examined
  excerpt: "        assert_eq!(profile.normalization(), Some(form));\n        assert_eq!(profile.table_definition(),\
    \ Some(UNICODE_TEXT_DEFINITION));\n        let normalized: String = form.apply(input).collect();\n        assert_eq!(normalized,\
    \ expected, \"{profile:?}\");\n        assert_ne!(normalized, form.apply(\"distinct\").collect::<String>());\n\
    \    }\n    assert_eq!(TextProfile::UnicodeScalars.table_definition(), None);\n    assert_eq!(TextProfile::BinaryUtf8.table_definition(),\
    \ None);\n}"
  lines: 28-36
- id: unicode-table-definition
  path: src/text.rs
  role: examined
  excerpt: "/// Definition identity of the normalization tables.\npub const UNICODE_TEXT_DEFINITION: &str = \"quire.value.text.unicode-17.0.0/v1\"\
    ;\n\n/// Unicode version the normalization tables implement.\npub const UNICODE_VERSION: (u8, u8, u8) = (17,\
    \ 0, 0);\n\nconst _: () = assert!(\n    matches!(unicode_normalization::UNICODE_VERSION, (17, 0, 0)),\n    \"\
    quire.value.text.unicode-17.0.0/v1 requires Unicode 17.0.0 normalization tables\"\n);"
  lines: 39-48
- id: text-profile-normalization
  path: src/text.rs
  role: examined
  excerpt: "            Self::Nfkc => Some(NormalizationForm::Nfkc),\n            Self::Nfkd => Some(NormalizationForm::Nfkd),\n\
    \            Self::UnicodeScalars | Self::BinaryUtf8 => None,\n        }\n    }\n\n    /// The Unicode table\
    \ definition a normalizing profile evaluates under.\n    pub fn table_definition(self) -> Option<&'static str>\
    \ {\n        self.normalization().map(|_| UNICODE_TEXT_DEFINITION)\n    }\n\n    /// Profile length of a retained\
    \ sequence: bytes for `binary-utf8`, scalars\n    /// otherwise, counted on the retained (normalized) sequence.\n\
    \    ///\n    /// `pub`, not `fn`-private: a call"
  lines: 111-126
- id: normalization-form-apply
  path: src/text.rs
  role: examined
  excerpt: "    pub fn apply(self, text: &str) -> impl Iterator<Item = char> + '_ {\n        match self {\n    \
    \        Self::Nfc => Normalized::Composed(text.nfc()),\n            Self::Nfd => Normalized::Decomposed(text.nfd()),\n\
    \            Self::Nfkc => Normalized::Composed(text.nfkc()),\n            Self::Nfkd => Normalized::Decomposed(text.nfkd()),\n\
    \        }\n    }\n}"
  lines: 173-181
- id: external-requirement-namespace
  path: src/lib.rs
  role: context_only
  excerpt: '//!

    //! Value-semantics requirements (decimals, text and enumerations, quantities

    //! and units, integer division, IEEE profiles) are owned by

    //! `agent-ix/quire-specification`. A test carries a `QSpec-` tagged AC only

    //! where its own assertions, not merely its module doc''s citation, tell the

    //! AC''s claim from a wrong implementation. The requirements this repository

    //! owns are in `spec/`.'
  lines: 44-50
- id: FR-141-AC-1
  path: /home/peter/dev/quire-specification/spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md
  role: examined
  excerpt: Canonically equivalent text compares according to the selected normalization profile and distinct text
    remains distinct.
  lines: '119'
  artifact_type: FR criterion
- id: FR-141-profile-definition
  path: /home/peter/dev/quire-specification/spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md
  role: context_only
  excerpt: '

    ## Text and enumeration profiles


    `unicode-scalars` compares the decoded scalar sequence without normalization;

    `nfc`, `nfd`, `nfkc` and `nfkd` apply exactly the named Unicode normalization

    form using the `quire.value.text.unicode-17.0.0/v1` tables; `binary-utf8`

    compares the admitted UTF-8 payload bytes. A source string''s payload is the

    canonical UTF-8 encoding of its decoded scalar sequence, so raw `é` and'
  lines: 38-45
- id: TC-186-rust-boundary
  path: /home/peter/dev/quire-specification/spec/test-cases/TC-186-text-and-enum-identity-semantics.md
  role: context_only
  excerpt: '

    Select language `ix:native`, edition `1-draft`, and the

    `quire.value.complete/v1` definition and the

    `quire.value.text.unicode-17.0.0/v1` definition through

    [`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).

    Execute these vectors through the real

    local Rust boundary:'
  lines: 17-23
- id: FR-367-AC-1
  path: spec/functional/FR-367-text-profile-mismatch-before-charging.md
  role: examined
  excerpt: 'Admit the same UTF-8 text "a" once with Nfc and once with BinaryUtf8 using separate sufficient meters.
    On a fresh comparison meter whose ten limits are zero, public `compare_text(Equal, left, right)` returns precisely
    `Err(IllTyped { cause: DistinctTextProfiles })`. Every consumed counter and admission count remains zero; under
    `test-support` the charge log is empty and truncation is false.'
  lines: '21'
  artifact_type: FR criterion
- id: CLAUDE.md
  path: CLAUDE.md
  role: context_only
  excerpt: '# quire-exact


    The exact-value kernel: checked identity, outcome and refusal, provenance and bound value types, with no dependency
    on any other crate in the quire ecosystem.


    ## Commands


    ```bash

    make fmt            # format with rustfmt

    make fmt-check      # verify formatting (CI gate)

    make lint           # clippy with -D warnings

    make test           # cargo test (default features, then test-support)

    make build          # release build

    make clean          # cargo clean

    make deny           # cargo deny check

    make ci             # fmt-check + lint + test + no_std build + deny + docs'
  lines: 1-15
- id: Makefile
  path: Makefile
  role: context_only
  excerpt: "# =============================================================================\n# quire-exact Makefile\n\
    # =============================================================================\n\nCARGO ?= cargo\n# A target\
    \ with no `std` at all: building for it proves the crate pulls in\n# nothing from `std`.\nNO_STD_TARGET ?= thumbv7em-none-eabi\n\
    \n.PHONY: help\nhelp:\n\t@echo \"Available targets:\"\n\t@echo \"  make fmt              - Format with rustfmt\"\
    \n\t@echo \"  make fmt-check        - Verify formatting (CI gate)\"\n\t@echo \"  make lint             - Clippy\
    \ with -D warnings\""
  lines: 1-15
- id: clippy.toml
  path: clippy.toml
  role: context_only
  excerpt: 'msrv = "1.98"

    cognitive-complexity-threshold = 30

    too-many-arguments-threshold = 8'
  lines: 1-3
- id: deny.toml
  path: deny.toml
  role: context_only
  excerpt: "[licenses]\nversion = 2\nallow = [\n    # This crate's own license (new-project mandates AGPL).\n  \
    \  \"AGPL-3.0-or-later\",\n    \"MIT\",\n    \"Apache-2.0\",\n    \"BSD-2-Clause\",\n    \"BSD-3-Clause\",\n\
    \    \"CDLA-Permissive-2.0\",\n    \"ISC\",\n    \"Unicode-3.0\",\n    \"Zlib\",\n]\nconfidence-threshold =\
    \ 0.9"
  lines: 1-15
- id: rustfmt.toml
  path: rustfmt.toml
  role: context_only
  excerpt: 'edition = "2021"

    max_width = 100

    use_small_heuristics = "Default"

    imports_granularity = "Crate"

    group_imports = "StdExternalCrate"'
  lines: 1-5
- id: red-fresh-consumer.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/red-fresh-consumer.log
  role: context_only
  excerpt: "     Locking 15 packages to latest compatible versions\n   Compiling proc-macro2 v1.0.107\n   Compiling\
    \ unicode-ident v1.0.27\n   Compiling quote v1.0.47\n   Compiling autocfg v1.5.1\n   Compiling num-traits v0.2.19\n\
    \   Compiling syn v3.0.9\n   Compiling thiserror v2.0.21\n   Compiling thiserror-impl v2.0.21\n   Compiling\
    \ num-integer v0.1.47\n   Compiling tinyvec v1.13.3\n   Compiling unicode-normalization v0.1.26\n   Compiling\
    \ num-bigint v0.4.8\n   Compiling once_cell v1.21.4\n   Compiling allocator-api2 v0.2.21\n   Compiling quire-exact\
    \ v0.1.0 (/home/peter/dev/worktrees/ir935-unicode)\nerror[E0080"
  lines: 38-57
- id: green-fresh-consumer.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/green-fresh-consumer.log
  role: context_only
  excerpt: '

    running 1 test

    test fresh_consumer_resolves_and_runs_unicode17_normalization ... ok


    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.09s'
  lines: 5-10
- id: focused-fmt-lint.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/focused-fmt-lint.log
  role: context_only
  excerpt: "    Checking num-traits v0.2.19\n    Checking allocator-api2 v0.2.21\n    Checking num-integer v0.1.47\n\
    \    Checking tinyvec v1.13.3\n    Checking unicode-normalization v0.1.25\n    Checking thiserror v2.0.21\n\
    \    Checking num-bigint v0.4.8\n    Checking libc v0.2.190\n    Checking forge-alloc-core v0.2.3\n    Checking\
    \ subtle v2.6.1\n    Checking once_cell v1.21.4\n    Checking forge-alloc v0.3.7\n    Checking allocation-counter\
    \ v0.8.1"
  lines: 40-52
- id: text-default.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/text-default.log
  role: context_only
  excerpt: '

    running 8 tests

    test equality::tests::native_string_equality_does_not_normalize_like_text ... ok

    test text::tests::admit_refuses_past_the_declared_maximum ... ok

    test text::tests::admitted_lengths_and_decode_charges_match_scalar_counts ... ok

    test text::tests::utf8_payload_within_bound_is_admitted ... ok

    test text::tests::valid_utf8_scalar_count_matches_char_iteration ... ok

    test value::native::tests::timestamp_canonical_text_preserves_full_signed_nanoseconds ... ok

    test value::native::tests::uuid_canonical_text_preserves_network_octets ... ok

    test value::tests::native_string_admiss'
  lines: 5-17
- id: text-test-support.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/text-test-support.log
  role: context_only
  excerpt: '

    running 8 tests

    test equality::tests::native_string_equality_does_not_normalize_like_text ... ok

    test text::tests::admit_refuses_past_the_declared_maximum ... ok

    test text::tests::admitted_lengths_and_decode_charges_match_scalar_counts ... ok

    test text::tests::utf8_payload_within_bound_is_admitted ... ok

    test text::tests::valid_utf8_scalar_count_matches_char_iteration ... ok

    test value::native::tests::timestamp_canonical_text_preserves_full_signed_nanoseconds ... ok

    test value::native::tests::uuid_canonical_text_preserves_network_octets ... ok

    test value::tests::native_string_admiss'
  lines: 5-17
- id: profile-boundary-default.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/profile-boundary-default.log
  role: context_only
  excerpt: '

    running 1 test

    test text_profile_mismatch_refuses_before_any_comparison_charge ... ok


    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s'
  lines: 5-10
- id: profile-boundary-test-support.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/profile-boundary-test-support.log
  role: context_only
  excerpt: '

    running 1 test

    test text_profile_mismatch_refuses_before_any_comparison_charge ... ok


    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s'
  lines: 5-10
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: fresh-consumer-directory
  related_ids: []
  path: tests/fresh_unicode_consumer.rs
  lines: 10-17
  excerpt: "    let owner = Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    let directory = owner\n        .join(\"\
    target\")\n        .join(format!(\"fresh-unicode-consumer-{}\", std::process::id()));\n    // A new directory,\
    \ rather than the owner's lockfile, supplies the resolution.\n    fs::create_dir_all(directory.parent().unwrap()).unwrap();\n\
    \    fs::create_dir(&directory).expect(\"fresh consumer directory must not already exist\");\n    fs::create_dir(directory.join(\"\
    src\")).unwrap();"
  finding: Fresh consumer builds are never removed; each successful run retains a separate target (108 MiB in the
    author green run), and eventual PID reuse makes create_dir fail. Use a uniquely claimed scratch directory with
    cleanup on success and failure while retaining diagnostic output.
- fnd: FND-002
  severity: low
  confidence: high
  method: code-review
  check: trace
  artifact_id: fresh-consumer-trace
  related_ids:
  - FR-141-AC-1
  path: tests/fresh_unicode_consumer.rs
  lines: '6'
  excerpt: '/// Trace: FR-141-AC-1'
  finding: The new external FR-141 trace omits the documented QSpec- namespace. Use QSpec-FR-141-AC-1 so the tag
    preserves ownership consistently with src/lib.rs and existing external-requirement tests; do not add a local
    FR-141 copy.
  test_id: fresh_consumer_resolves_and_runs_unicode17_normalization
  claimed_ac: FR-141-AC-1
  correct_ac: QSpec-FR-141-AC-1
bindings:
- test_id: fresh_consumer_resolves_and_runs_unicode17_normalization
  ac_id: FR-141-AC-1
  trace: wrong
  path: tests/fresh_unicode_consumer.rs
  claimed_ac: FR-141-AC-1
  correct_ac: QSpec-FR-141-AC-1
  basis: The asserted kernel normalization behavior belongs to external QSpec FR-141-AC-1; the tag omits the documented
    QSpec namespace.
- test_id: text_profile_mismatch_refuses_before_any_comparison_charge
  ac_id: FR-367-AC-1
  trace: correct
  path: tests/ir673_ieee_text.rs
  line: 264
  basis: Existing test asserts distinct-profile refusal and zero accounting; author logs cover both feature modes.
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Atomic create_dir claims skip occupied names with a bounded 128-candidate search; an owned Drop guard removes fresh source/manifest/lock inputs on normal completion and unwind. Explicit controls verify sentinel preservation and removal. One reusable build cache replaces per-run build targets; cleanup errors report path/error and success paths assert removal. |
| FND-002 | fixed | The actual test now uses the existing ix_trace_rs canonical attribute with the QSpec- namespace. Independent reviewer Quire trace reports exactly one canonical claim; no new dependency or locally copied requirement was introduced. |

## Disposition round 1

PASS for the current PR delta; both actionable code-review findings fixed. Original CONDITIONAL verdict is retained as the historical review result.

Reviewed actual ec37d86/d878209 code and source-owned classifications at d878209d5c11b4085963dee17cbb0619dc961110. Source, dependency, public Unicode identity/assertion, fixture normalization vectors and feature boundaries remain unchanged except the reviewed harness fix. Fresh consumers still have independently created source/manifest inputs and no initial Cargo.lock; the shared build cache contains build products only and cannot supply a consumer lockfile. The separate cache has its own Cargo lock, so it does not recurse into the outer owner test target. Parent test process environment is preserved apart from the child target override and explicit serial jobs. RAII cleanup is best-effort with path/error diagnostics on filesystem deletion failure; successful-run and caught-unwind removal are asserted. The current scratch parent is empty. Earlier red/green evidence directories are retained separately and never used as inputs. No new findings were identified in this fix review.

Reviewer actually ran Quire trace and strict matrix only. Trace resolves the canonical external attribute; strict exits 1 for exactly the same four inherited gaps, with zero ignored-only. Author fix1-consumer-default.log and fix1-consumer-test-support.log each report one passing real Cargo consumer test including lifecycle controls. Author fix1-fmt-lint-final.log shows fmt-check and all three clippy lanes passing; existing stable-rustfmt configuration warnings remain visible. Reviewer Cargo/rustc/fmt/Kani checks are UNRUN under the no-heavy grant. Aggregate make ci, downstream CG native qualification and Kani/mutation obligations await their owners; none is credited from the owner normalization test.

Plan completion: not assessed

Runtime model: gpt-6.1-sol; run: ir935-disposition1-e929a750-9cf8-4d45-81ae-71afdbb0a68c. No source edits, commits or PR review comments by this reviewer.

```yaml
round: 1
head: d878209d5c11b4085963dee17cbb0619dc961110
base: aa63308958b7fa3a08142a24ce3694e94c28d62b
current_verdict: PASS for the current PR delta; both actionable code-review findings fixed. Original CONDITIONAL
  verdict is retained as the historical review result.
scope:
- id: ScratchDirectory::new
  path: tests/fresh_unicode_consumer.rs
  lines: 15-30
  role: examined
  excerpt: "    fn new(parent: &Path) -> io::Result<Self> {\n        fs::create_dir_all(parent)?;\n        // Atomic\
    \ directory creation claims ownership; PID reuse and concurrent\n        // invocations skip existing directories\
    \ rather than reading their inputs.\n        for attempt in 0..128 {\n            let path = parent.join(format!(\"\
    consumer-{}-{attempt}\", std::process::id()));\n            match fs::create_dir(&path) {\n                Ok(())\
    \ => return Ok(Self(path)),\n                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,\n\
    \                Err(error) => return Err("
- id: ScratchDirectory::drop
  path: tests/fresh_unicode_consumer.rs
  lines: 34-45
  role: examined
  excerpt: "impl Drop for ScratchDirectory {\n    fn drop(&mut self) {\n        // Also runs if a Cargo invocation\
    \ or assertion panics. Keep diagnostics\n        // without risking a second panic while an assertion is unwinding.\n\
    \        if let Err(error) = fs::remove_dir_all(&self.0) {\n            eprintln!(\n                \"could\
    \ not remove consumer scratch {}: {error}\",\n                self.0.display()\n            );\n        }\n\
    \    }\n}"
- id: canonical-external-test-trace
  path: tests/fresh_unicode_consumer.rs
  lines: 47-50
  role: examined
  excerpt: '/// Provenance: IR-935, QSpec-TC-186

    #[trace("QSpec-FR-141-AC-1")]

    #[test]

    fn fresh_consumer_resolves_and_runs_unicode17_normalization() {'
- id: collision-control
  path: tests/fresh_unicode_consumer.rs
  lines: 51-59
  role: examined
  excerpt: "    let owner = Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    let scratch_parent = owner.join(\"target/fresh-unicode-consumers\"\
    );\n    // Occupy the first candidate to exercise collision handling, without granting\n    // the consumer\
    \ ownership of another invocation's files.\n    let occupied = ScratchDirectory::new(&scratch_parent).unwrap();\n\
    \    fs::write(occupied.0.join(\"sentinel\"), \"preserve other owner\").unwrap();\n    let scratch = ScratchDirectory::new(&scratch_parent).unwrap();\n\
    \    let directory = &scratch.0;\n    fs::create_dir(directory.join(\"src\")).unwrap();"
- id: fresh-consumer-manifest-and-lock
  path: tests/fresh_unicode_consumer.rs
  lines: 60-74
  role: examined
  excerpt: "    fs::write(\n        directory.join(\"Cargo.toml\"),\n        format!(\n            \"[package]\\\
    nname = \\\"fresh-unicode-consumer\\\"\\nversion = \\\"0.0.0\\\"\\nedition = \\\"2021\\\"\\n\\\n           \
    \  [workspace]\\n\\\n             [dependencies]\\nquire-exact = {{ path = {owner:?} }}\\n\"\n        ),\n \
    \   )\n    .unwrap();\n    fs::write(\n        directory.join(\"src/main.rs\"),\n        include_str!(\"fixtures/unicode17_consumer.rs\"\
    ),\n    )\n    .unwrap();\n    assert!(!directory.join(\"Cargo.lock\").exists());"
- id: independent-build-cache
  path: tests/fresh_unicode_consumer.rs
  lines: 76-87
  role: examined
  excerpt: "    // Reuse a separate build cache without retaining consumer inputs or locks.\n    // Cargo serializes\
    \ this cache independently of the outer cargo test target.\n    // Offline resolution uses the ordinary registry\
    \ cache without consumer pins.\n    let output = Command::new(env!(\"CARGO\"))\n        .current_dir(directory)\n\
    \        .args([\"run\", \"--offline\", \"--jobs\", \"1\"])\n        .env(\n            \"CARGO_TARGET_DIR\"\
    ,\n            owner.join(\"target/fresh-unicode-consumer-build\"),\n        )\n        .output()\n        .expect(\"\
    run the real consumer through Cargo\");"
- id: success-cleanup-and-owner-isolation
  path: tests/fresh_unicode_consumer.rs
  lines: 95-108
  role: examined
  excerpt: "    assert!(directory.join(\"Cargo.lock\").is_file());\n    let removed = directory.clone();\n    drop(scratch);\n\
    \    assert!(\n        !removed.exists(),\n        \"consumer inputs and lock must be removed\"\n    );\n  \
    \  assert_eq!(\n        fs::read_to_string(occupied.0.join(\"sentinel\")).unwrap(),\n        \"preserve other\
    \ owner\"\n    );\n    let removed = occupied.0.clone();\n    drop(occupied);\n    assert!(!removed.exists(),\
    \ \"collision control must be removed\");"
- id: unwind-cleanup-control
  path: tests/fresh_unicode_consumer.rs
  lines: 110-118
  role: examined
  excerpt: "    let failure = ScratchDirectory::new(&scratch_parent).unwrap();\n    let removed = failure.0.clone();\n\
    \    fs::write(removed.join(\"Cargo.lock\"), \"failure cleanup control\").unwrap();\n    let failed = std::panic::catch_unwind(move\
    \ || {\n        let _owned_until_unwind = failure;\n        panic!(\"simulate a consumer assertion failure\"\
    );\n    });\n    assert!(failed.is_err());\n    assert!(!removed.exists(), \"failed consumer inputs must be\
    \ removed\");"
- id: owning-normalization-constraint
  path: Cargo.toml
  lines: 31-32
  role: examined
  excerpt: '# The Unicode 17 text definition requires 0.1.25''s tables; 0.1.26 uses Unicode 18.

    unicode-normalization = { version = "=0.1.25", default-features = false }'
- id: existing-normalization-fixture
  path: tests/fixtures/unicode17_consumer.rs
  lines: 28-36
  role: examined
  excerpt: "        assert_eq!(profile.normalization(), Some(form));\n        assert_eq!(profile.table_definition(),\
    \ Some(UNICODE_TEXT_DEFINITION));\n        let normalized: String = form.apply(input).collect();\n        assert_eq!(normalized,\
    \ expected, \"{profile:?}\");\n        assert_ne!(normalized, form.apply(\"distinct\").collect::<String>());\n\
    \    }\n    assert_eq!(TextProfile::UnicodeScalars.table_definition(), None);\n    assert_eq!(TextProfile::BinaryUtf8.table_definition(),\
    \ None);\n}"
- id: documented-external-namespace
  path: src/lib.rs
  lines: 44-50
  role: context_only
  excerpt: '//!

    //! Value-semantics requirements (decimals, text and enumerations, quantities

    //! and units, integer division, IEEE profiles) are owned by

    //! `agent-ix/quire-specification`. A test carries a `QSpec-` tagged AC only

    //! where its own assertions, not merely its module doc''s citation, tell the

    //! AC''s claim from a wrong implementation. The requirements this repository

    //! owns are in `spec/`.'
- id: fix1-consumer-default.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/fix1-consumer-default.log
  lines: 5-10
  role: context_only
  excerpt: '

    running 1 test

    test fresh_consumer_resolves_and_runs_unicode17_normalization ... ok


    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.08s

    '
- id: fix1-consumer-test-support.log
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/fix1-consumer-test-support.log
  lines: 5-10
  role: context_only
  excerpt: '

    running 1 test

    test fresh_consumer_resolves_and_runs_unicode17_normalization ... ok


    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

    '
- id: author-final-lint
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/fix1-fmt-lint-final.log
  lines: 38-55
  role: context_only
  excerpt: "Warning: can't set `group_imports = StdExternalCrate`, unstable features are only available in nightly\
    \ channel.\ncargo clippy --all-targets -- -D warnings\n    Checking quire-exact v0.1.0 (/home/peter/dev/worktrees/ir935-unicode)\n\
    \    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s\ncargo clippy --all-targets --features\
    \ test-support -- -D warnings\n    Checking quire-exact v0.1.0 (/home/peter/dev/worktrees/ir935-unicode)\n \
    \   Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s\ncargo clippy --lib --target thumbv7em-none-eabi\
    \ -- -D warnings\n    Finished `d"
- id: reviewer-trace-result
  path: /home/peter/dev/worktrees/kani-lead-codex/ir935/review/round1-trace.json
  lines: 1-1
  role: context_only
  excerpt: '{"query":{"kind":"file","path":"tests/fresh_unicode_consumer.rs"},"resolved":true,"claims":{"verifies":[{"trace_id":"QSpec-FR-141-AC-1","symbol":"fresh_consumer_resolves_and_runs_unicode17_normalization","path":"tests/fresh_unicode_consumer.rs","line":50,"form":"rust-trace-attribute","provenance":"canonical","language":"rust","confidence":"structural"}],"implements":[]},"citations":[{"trace_id":"AGPL-3","path":"tests/fresh_unicode_consumer.rs","symbol":null,"kind":null,"bucket":"mention","line":1,"excerpt":"//
    SPDX-License-Identifier: AGPL-3.0-or-later","language":"rust","confidence"'
- id: FR-141-AC-1
  path: /home/peter/dev/quire-specification/spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md
  lines: '119'
  role: examined
  excerpt: Canonically equivalent text compares according to the selected normalization profile and distinct text
    remains distinct.
- id: QSpec-unicode-table-context
  path: /home/peter/dev/quire-specification/spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md
  lines: 38-45
  role: context_only
  excerpt: '

    ## Text and enumeration profiles


    `unicode-scalars` compares the decoded scalar sequence without normalization;

    `nfc`, `nfd`, `nfkc` and `nfkd` apply exactly the named Unicode normalization

    form using the `quire.value.text.unicode-17.0.0/v1` tables; `binary-utf8`

    compares the admitted UTF-8 payload bytes. A source string''s payload is the

    canonical UTF-8 encoding of its decoded scalar sequence, so raw `é` and'
bindings:
- test_id: fresh_consumer_resolves_and_runs_unicode17_normalization
  ac_id: QSpec-FR-141-AC-1
  trace: correct
  path: tests/fresh_unicode_consumer.rs
  line: 50
  basis: Actual reviewer quire trace finds one canonical rust-trace-attribute verifies claim; assertion fixture
    exercises kernel normalization and distinctness, not all TC-186 evaluator cases.
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "        for attempt in 0..128 {\n            let path = parent.join(format!(\"consumer-{}-{attempt}\"\
    , std::process::id()));\n            match fs::create_dir(&path) {\n                Ok(()) => return Ok(Self(path)),\n\
    \                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,\n                Err(error)\
    \ => return Err(error),\n            }"
  reason: Atomic create_dir claims skip occupied names with a bounded 128-candidate search; an owned Drop guard
    removes fresh source/manifest/lock inputs on normal completion and unwind. Explicit controls verify sentinel
    preservation and removal. One reusable build cache replaces per-run build targets; cleanup errors report path/error
    and success paths assert removal.
- fnd: FND-002
  outcome: fixed
  after_excerpt: '#[trace("QSpec-FR-141-AC-1")]'
  reason: The actual test now uses the existing ix_trace_rs canonical attribute with the QSpec- namespace. Independent
    reviewer Quire trace reports exactly one canonical claim; no new dependency or locally copied requirement was
    introduced.
new_findings: []
```
