// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native Text carrier against the shipped `compare_text` and an
//! independent reference (FR-404, TC-952).
use ix_trace_rs::trace;
use quire_exact::{
    admit_text, compare_native_text, compare_text, ComparisonOperator, IllTyped, IllTypedCause,
    LimitKind, Meter, NativeText, NativeTextRefusal, Outcome, ScalarLimits, Text,
    TextNotAdmittedCause, TextPayload, TextProfile, TextType, NATIVE_TEXT_CAPACITY as C,
};
use std::cmp::Ordering;

fn limits(value: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: value,
        decimal_digits: value,
        scale_expansion: value,
        text_input_bytes: value,
        text_scalars: value,
        normalized_scalars: value,
        unit_edges: value,
        value_occurrences: value,
        work_units: value,
        result_units: value,
    }
}

fn set(mut base: ScalarLimits, kind: LimitKind, value: u64) -> ScalarLimits {
    match kind {
        LimitKind::IntegerBits => base.integer_bits = value,
        LimitKind::DecimalDigits => base.decimal_digits = value,
        LimitKind::ScaleExpansion => base.scale_expansion = value,
        LimitKind::TextInputBytes => base.text_input_bytes = value,
        LimitKind::TextScalars => base.text_scalars = value,
        LimitKind::NormalizedScalars => base.normalized_scalars = value,
        LimitKind::UnitEdges => base.unit_edges = value,
        LimitKind::ValueOccurrences => base.value_occurrences = value,
        LimitKind::WorkUnits => base.work_units = value,
        LimitKind::ResultUnits => base.result_units = value,
    }
    base
}

fn ty(profile: TextProfile) -> TextType {
    TextType::new(0, 8 * C as u64, profile).unwrap()
}

fn owner_in(text_type: TextType, text: &str) -> Text {
    let payload = TextPayload::from_utf8(text.as_bytes()).unwrap();
    admit_text(&payload, &text_type, &mut Meter::new(limits(u64::MAX)))
        .completed()
        .expect("owner admits the sample")
}

fn native_in(text_type: TextType, text: &str) -> NativeText {
    let retained = owner_in(text_type, text).retained().to_owned();
    NativeText::from_admitted(text_type, text.as_bytes(), retained.as_bytes())
        .expect("native admits the sample")
}

fn owner(profile: TextProfile, text: &str) -> Text {
    owner_in(ty(profile), text)
}

fn native(profile: TextProfile, text: &str) -> NativeText {
    native_in(ty(profile), text)
}

fn filler(length: usize, tail: &str) -> String {
    "a".repeat(length - tail.len()) + tail
}

// ---- AC-1 ----

type ConstructorFn = fn(TextType, &[u8], &[u8]) -> Result<NativeText, NativeTextRefusal>;
type CompareFn =
    fn(ComparisonOperator, &NativeText, &NativeText, &mut Meter) -> Result<Outcome<bool>, IllTyped>;

fn assert_copy<T: Copy>() {}

#[test]
#[trace("TC-952", "FR-404-AC-1")]
fn carrier_is_copy_without_drop_glue_and_signatures_match() {
    assert_copy::<NativeText>();
    const { assert!(!std::mem::needs_drop::<NativeText>()) };
    let _: ConstructorFn = NativeText::from_admitted;
    let _: CompareFn = compare_native_text;
}

#[test]
#[trace("TC-952", "FR-404-AC-1")]
fn constructing_a_full_payload_allocates_nothing() {
    let text = "b".repeat(C);
    let text_type = TextType::new(0, C as u64, TextProfile::BinaryUtf8).unwrap();
    let mut built = None;
    let info = allocation_counter::measure(|| {
        built = Some(NativeText::from_admitted(
            text_type,
            text.as_bytes(),
            text.as_bytes(),
        ));
    });
    assert!(built.unwrap().is_ok());
    assert_eq!(info.count_total, 0);
}

// ---- AC-2, AC-3 ----

fn tail_is_zero(text: &NativeText) {
    assert!(text.payload_storage()[text.payload().len()..]
        .iter()
        .all(|byte| *byte == 0));
    assert!(text.retained_storage()[text.retained().len()..]
        .iter()
        .all(|byte| *byte == 0));
}

#[test]
#[trace("TC-952", "FR-404-AC-2")]
fn bytes_after_each_length_are_zero_and_equal_texts_are_equal() {
    for profile in [TextProfile::BinaryUtf8, TextProfile::UnicodeScalars] {
        for length in [0, 1, C - 1, C] {
            for tail in ["b", "é", "€", "🦀"] {
                if length < tail.len() {
                    continue;
                }
                let text = filler(length, tail);
                let first = native(profile, &text);
                tail_is_zero(&first);
                assert_eq!(first, native(profile, &text));
            }
        }
    }
}

#[test]
#[trace("TC-952", "FR-404-AC-3")]
fn capacity_bytes_are_held_and_one_more_refuses() {
    for profile in [TextProfile::BinaryUtf8, TextProfile::UnicodeScalars] {
        let text_type = ty(profile);
        for tail in ["b", "€"] {
            for length in [C - 1, C] {
                let text = filler(length, tail);
                let held = NativeText::from_admitted(text_type, text.as_bytes(), text.as_bytes())
                    .expect("within capacity");
                assert_eq!(held.payload(), text.as_bytes());
                assert_eq!(held.retained(), text.as_bytes());
            }
            for length in [C + 1, 2 * C] {
                let text = filler(length, tail);
                assert_eq!(
                    NativeText::from_admitted(text_type, text.as_bytes(), text.as_bytes()),
                    Err(NativeTextRefusal::PayloadExceedsCapacity),
                    "{length} bytes ending {tail:?}"
                );
            }
        }
        let long = "a".repeat(C + 1);
        assert_eq!(
            NativeText::from_admitted(text_type, b"a", long.as_bytes()),
            Err(NativeTextRefusal::PayloadExceedsCapacity)
        );
        let full = "a".repeat(C);
        assert!(NativeText::from_admitted(text_type, full.as_bytes(), full.as_bytes()).is_ok());
    }
}

#[test]
#[trace("TC-952", "FR-404-AC-3")]
fn constructor_decides_like_admission_within_capacity() {
    let scalars = TextType::new(1, 1, TextProfile::UnicodeScalars).unwrap();
    let refused = |cause| Err(NativeTextRefusal::NotAdmitted(cause));
    assert_eq!(
        NativeText::from_admitted(scalars, b"ab", b"a"),
        refused(TextNotAdmittedCause::RetainedNotNormalForm)
    );
    assert_eq!(
        NativeText::from_admitted(scalars, b"ab", b"ab"),
        refused(TextNotAdmittedCause::LengthOutOfDomain)
    );
    assert_eq!(
        NativeText::from_admitted(scalars, b"", b""),
        refused(TextNotAdmittedCause::LengthOutOfDomain)
    );
    let bytes = TextType::new(0, 1, TextProfile::BinaryUtf8).unwrap();
    assert_eq!(
        NativeText::from_admitted(bytes, "é".as_bytes(), "é".as_bytes()),
        refused(TextNotAdmittedCause::LengthOutOfDomain)
    );
    assert!(NativeText::from_admitted(scalars, "é".as_bytes(), "é".as_bytes()).is_ok());
    let nfc = TextType::new(1, 1, TextProfile::Nfc).unwrap();
    assert!(NativeText::from_admitted(nfc, "e\u{301}".as_bytes(), "\u{E9}".as_bytes()).is_ok());
    assert_eq!(
        NativeText::from_admitted(nfc, "e\u{301}".as_bytes(), "e\u{301}".as_bytes()),
        refused(TextNotAdmittedCause::RetainedNotNormalForm)
    );
}

// ---- AC-4 ----

/// Reference written apart from the replacement: operator over byte slices
/// by the language's own `==` and `cmp`.
fn reference_holds(operator: ComparisonOperator, left: &[u8], right: &[u8]) -> bool {
    let ordering = left.cmp(right);
    match operator {
        ComparisonOperator::Equal => left == right,
        ComparisonOperator::NotEqual => left != right,
        ComparisonOperator::Less => ordering == Ordering::Less,
        ComparisonOperator::LessOrEqual => ordering != Ordering::Greater,
        ComparisonOperator::Greater => ordering == Ordering::Greater,
        ComparisonOperator::GreaterOrEqual => ordering != Ordering::Less,
    }
}

const KINDS: [LimitKind; 10] = LimitKind::ALL;

fn budgets(consumed: &[u64; 10]) -> Vec<ScalarLimits> {
    let mut all = vec![limits(0)];
    let mut exact = limits(u64::MAX);
    for (kind, value) in KINDS.iter().zip(consumed) {
        exact = set(exact, *kind, *value);
    }
    all.push(exact);
    for (kind, value) in KINDS.iter().zip(consumed) {
        if *value > 0 {
            all.push(set(limits(u64::MAX), *kind, value - 1));
        }
    }
    all
}

fn consumed_of(meter: &Meter) -> [u64; 10] {
    KINDS.map(|kind| meter.consumed(kind))
}

/// One operand pair, every operator, every budget: native against shipped.
fn differential(profile: TextProfile, left: &str, right: &str) {
    differential_in(ty(profile), left, right);
}

fn differential_in(text_type: TextType, left: &str, right: &str) {
    let profile = text_type.profile();
    let (owner_left, owner_right) = (owner_in(text_type, left), owner_in(text_type, right));
    let (native_left, native_right) = (native_in(text_type, left), native_in(text_type, right));
    for operator in ComparisonOperator::ALL {
        let mut generous = Meter::new(limits(u64::MAX));
        let shipped = compare_text(operator, &owner_left, &owner_right, &mut generous).unwrap();
        assert_eq!(
            shipped,
            Outcome::Completed(reference_holds(
                operator,
                owner_left.retained().as_bytes(),
                owner_right.retained().as_bytes()
            )),
            "reference {operator:?} {left:?} {right:?}"
        );
        for budget in budgets(&consumed_of(&generous)) {
            let mut expected_meter = Meter::new(budget);
            let mut actual_meter = Meter::new(budget);
            let expected =
                compare_text(operator, &owner_left, &owner_right, &mut expected_meter).unwrap();
            let actual =
                compare_native_text(operator, &native_left, &native_right, &mut actual_meter)
                    .unwrap();
            let context = format!("{profile:?} {operator:?} {left:?} {right:?} {budget:?}");
            assert_eq!(actual, expected, "{context}");
            assert_eq!(
                consumed_of(&actual_meter),
                consumed_of(&expected_meter),
                "{context}"
            );
            #[cfg(feature = "test-support")]
            assert_eq!(
                actual_meter.admitted_charges(),
                expected_meter.admitted_charges(),
                "{context}"
            );
        }
    }
}

/// Lengths of the Behavior 8 family and, per profile, why a length has no
/// admitted payload. The two non-normalizing profiles admit every length
/// (declared maximum 8C bytes, so A = min(A, C) = C and L - 1 exists).
const SKIPPED: [(&str, &str); 4] = [
    (
        "nfc",
        "unsupported on the proof path (Behavior 10); no VERIFIED normalization record",
    ),
    (
        "nfd",
        "unsupported on the proof path (Behavior 10); no VERIFIED normalization record",
    ),
    (
        "nfkc",
        "unsupported on the proof path (Behavior 10); no VERIFIED normalization record",
    ),
    (
        "nfkd",
        "unsupported on the proof path (Behavior 10); no VERIFIED normalization record",
    ),
];

fn family() -> Vec<String> {
    let mut texts = vec![
        String::new(),
        "x".into(),
        "x\0".into(),
        "xy".into(),
        "y".into(),
    ];
    for length in [2, C - 1, C] {
        for tail in ["b", "c", "é", "€", "🦀"] {
            if length >= tail.len() {
                texts.push(filler(length, tail));
            }
        }
    }
    texts
}

#[test]
#[trace("TC-952", "FR-404-AC-4")]
fn replacement_matches_shipped_compare_over_the_family() {
    for (profile, reason) in SKIPPED {
        assert!(!reason.is_empty(), "{profile} is listed with its reason");
    }
    let texts = family();
    for profile in [TextProfile::BinaryUtf8, TextProfile::UnicodeScalars] {
        assert!(NativeText::proof_path_supports(profile));
        for left in &texts {
            for right in &texts {
                differential(profile, left, right);
            }
        }
    }
}

#[test]
#[trace("TC-952", "FR-404-AC-4")]
fn normalizing_profiles_match_shipped_charges_on_samples() {
    for profile in [
        TextProfile::Nfc,
        TextProfile::Nfd,
        TextProfile::Nfkc,
        TextProfile::Nfkd,
    ] {
        assert!(!NativeText::proof_path_supports(profile));
        let texts = [
            "",
            "a",
            "e\u{301}",
            "\u{E9}",
            "\u{FB03}",
            "u\u{308}\u{301}x",
        ];
        for left in texts {
            for right in texts {
                differential(profile, left, right);
            }
        }
    }
}

/// The family of CG FR-052-AC-14 at the operand's declared `max`: the
/// length-L payload (A = C), the length-L_b payload (the final unit replaced:
/// the final byte dropped for `binary-utf8`, a three-byte scalar for the
/// four-byte final scalar of `unicode-scalars`), and per length a final-byte
/// pair, a length pair and a first-byte pair. Declared types are
/// `Text[0, max]`. The normalizing profiles are unsupported and have no
/// family here.
fn ac14_family(profile: TextProfile) -> (u64, Vec<String>) {
    const { assert!(C.is_multiple_of(4)) };
    match profile {
        TextProfile::BinaryUtf8 => {
            let body = "a".repeat(C - 2);
            let family = vec![
                format!("{body}ab"),
                format!("{body}ac"),
                format!("z{}ab", &body[1..]),
                format!("{body}a"),
                format!("{body}b"),
                format!("z{}a", &body[1..]),
            ];
            (C as u64, family)
        }
        TextProfile::UnicodeScalars => {
            let unit = "\u{1F980}";
            let other_last = "\u{1F981}";
            let other_lead = "\u{5F980}";
            let head = unit.repeat(C / 4 - 1);
            let lead_tail = unit.repeat(C / 4 - 2);
            let family = vec![
                format!("{head}{unit}"),
                format!("{head}{other_last}"),
                format!("{other_lead}{lead_tail}{unit}"),
                format!("{head}\u{20AC}"),
                format!("{head}\u{20AD}"),
                format!("{other_lead}{lead_tail}\u{20AC}"),
            ];
            ((C / 4) as u64, family)
        }
        _ => panic!("unsupported profile has no family"),
    }
}

#[test]
#[trace("TC-952", "FR-404-AC-4")]
fn replacement_matches_shipped_compare_over_the_ac14_family() {
    for profile in [TextProfile::BinaryUtf8, TextProfile::UnicodeScalars] {
        let (max, family) = ac14_family(profile);
        let text_type = TextType::new(0, max, profile).unwrap();
        assert_eq!(family[0].len(), C, "length L");
        assert_eq!(family[3].len(), C - 1, "length L_b");
        for left in &family {
            for right in &family {
                differential_in(text_type, left, right);
            }
        }
    }
}

#[test]
#[trace("TC-952", "FR-404-AC-3")]
fn parts_that_are_not_utf8_refuse_under_every_profile() {
    for profile in TextProfile::ALL {
        let text_type = TextType::new(0, 8, profile).unwrap();
        for (payload, retained) in [
            (&b"\xff"[..], &b"\xff"[..]),
            (&b"a"[..], &b"\xff"[..]),
            (&b"\xc3"[..], &b"\xc3"[..]),
        ] {
            assert_eq!(
                NativeText::from_admitted(text_type, payload, retained),
                Err(NativeTextRefusal::NotAdmitted(
                    TextNotAdmittedCause::RetainedNotNormalForm
                )),
                "{profile:?} {payload:?} {retained:?}"
            );
        }
    }
}

#[test]
#[trace("TC-952", "FR-404-AC-4")]
fn distinct_profiles_are_ill_typed_and_consume_nothing() {
    let (a, b) = (
        native(TextProfile::BinaryUtf8, "x"),
        native(TextProfile::UnicodeScalars, "x"),
    );
    let (oa, ob) = (
        owner(TextProfile::BinaryUtf8, "x"),
        owner(TextProfile::UnicodeScalars, "x"),
    );
    for operator in ComparisonOperator::ALL {
        let mut meter = Meter::new(limits(u64::MAX));
        let actual = compare_native_text(operator, &a, &b, &mut meter);
        assert_eq!(
            actual,
            Err(IllTyped {
                cause: IllTypedCause::DistinctTextProfiles
            })
        );
        assert_eq!(consumed_of(&meter), [0; 10]);
        let mut shipped_meter = Meter::new(limits(u64::MAX));
        assert_eq!(actual, compare_text(operator, &oa, &ob, &mut shipped_meter));
    }
}

// ---- AC-5 ----

#[derive(Debug, Eq, PartialEq)]
enum Violation {
    CfgKani,
    BannedToken(String),
    LoopHeader(String),
    Callee(String),
    Cycle,
    SliceComparison,
}

fn strip_comments(source: &str) -> String {
    let bytes: Vec<char> = source.chars().collect();
    let mut out = String::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == '/' && bytes.get(index + 1) == Some(&'/') {
            while index < bytes.len() && bytes[index] != '\n' {
                index += 1;
            }
        } else if bytes[index] == '/' && bytes.get(index + 1) == Some(&'*') {
            index += 2;
            while index + 1 < bytes.len() && !(bytes[index] == '*' && bytes[index + 1] == '/') {
                index += 1;
            }
            index += 2;
        } else {
            out.push(bytes[index]);
            index += 1;
        }
    }
    out
}

fn words(text: &str) -> Vec<&str> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|word| !word.is_empty())
        .collect()
}

/// Remove `#[...]` attributes (their `error("..")` and `derive(..)` are not
/// calls).
fn without_attributes(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("#[") {
        out.push_str(&rest[..start]);
        let mut depth = 0;
        let mut end = rest.len();
        for (offset, ch) in rest[start..].char_indices() {
            match ch {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        end = start + offset + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// Strings are blanked so their content is not scanned.
fn without_strings(text: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for ch in text.chars() {
        if ch == '"' {
            inside = !inside;
            out.push('"');
        } else if !inside {
            out.push(ch);
        }
    }
    out
}

const ALLOWED_CALLEES: [&str; 20] = [
    "Ok",
    "Err",
    "Some",
    "len",
    "new",
    "size",
    "results",
    "charge",
    "length_amount",
    "from_stop",
    "holds",
    "normalization",
    "is_retained_of",
    "min",
    "max",
    "profile",
    "text_type",
    "is_some",
    "NotAdmitted",
    "are_utf8",
];
const KEYWORDS: [&str; 8] = ["if", "match", "for", "in", "return", "let", "else", "fn"];
const BANNED_WORDS: [&str; 14] = [
    "while",
    "loop",
    "from_le_bytes",
    "from_be_bytes",
    "from_ne_bytes",
    "read_unaligned",
    "wrapping_mul",
    "compare_text",
    "admit_text",
    "reference_holds",
    "order",
    "chars",
    "from_utf8",
    "apply",
];
const BANNED_CALLS: [&str; 14] = [
    "iter",
    "bytes",
    "chars",
    "map",
    "zip",
    "enumerate",
    "fold",
    "all",
    "any",
    "position",
    "windows",
    "chunks",
    "chunks_exact",
    "cmp",
];

fn defined_functions(code: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut search = 0;
    while let Some(at) = code[search..].find("fn ") {
        let start = search + at;
        search = start + 3;
        if start > 0 && code.as_bytes()[start - 1].is_ascii_alphanumeric() {
            continue;
        }
        let name: String = code[start + 3..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let Some(open) = code[start..].find('{') else {
            continue;
        };
        let mut depth = 0;
        let mut end = code.len();
        for (offset, ch) in code[start + open..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = start + open + offset + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        found.push((name, code[start + open..end].to_owned()));
    }
    found
}

fn calls_in(body: &str) -> Vec<String> {
    let chars: Vec<char> = body.chars().collect();
    let mut calls = Vec::new();
    for (index, ch) in chars.iter().enumerate() {
        if *ch == '(' && index > 0 {
            let mut start = index;
            while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
                start -= 1;
            }
            if start < index
                && chars[start..index]
                    .iter()
                    .next()
                    .is_some_and(|c| !c.is_numeric())
            {
                let name: String = chars[start..index].iter().collect();
                if !KEYWORDS.contains(&name.as_str()) {
                    calls.push(name);
                }
            }
        }
    }
    calls
}

/// Names declared with a slice or array type, so a bare comparison of them is
/// a memcmp or lexicographic comparison and not an indexed loop.
fn slice_comparisons(code: &str) -> Vec<Violation> {
    let chars: Vec<char> = code.chars().collect();
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut slices: Vec<String> = Vec::new();
    for (at, window) in chars.windows(2).enumerate() {
        if window[0] == ':' && window[1] == ' ' && at > 0 {
            let mut end = at;
            while end > 0 && ident(chars[end - 1]) {
                end -= 1;
            }
            let ty: String = chars[at + 2..].iter().take(8).collect();
            if end < at && (ty.starts_with("&[u8]") || ty.starts_with("[u8;")) {
                slices.push(chars[end..at].iter().collect());
            }
        }
    }
    let mut found = Vec::new();
    for (at, ch) in chars.iter().enumerate() {
        if !matches!(ch, '<' | '>' | '=' | '!') {
            continue;
        }
        let two: String = chars[at..(at + 2).min(chars.len())].iter().collect();
        let width = if matches!(two.as_str(), "==" | "!=" | "<=" | ">=") {
            2
        } else if matches!(ch, '<' | '>') && chars.get(at.wrapping_sub(1)) != Some(&'-') {
            1
        } else {
            continue;
        };
        if width == 1 && chars.get(at + 1) == Some(&'=') {
            continue;
        }
        let mut left = at;
        while left > 0 && chars[left - 1] == ' ' {
            left -= 1;
        }
        let mut start = left;
        while start > 0 && ident(chars[start - 1]) {
            start -= 1;
        }
        let mut right = at + width;
        while chars.get(right) == Some(&' ') {
            right += 1;
        }
        let mut stop = right;
        while chars.get(stop).is_some_and(|c| ident(*c)) {
            stop += 1;
        }
        let left_name: String = chars[start..left].iter().collect();
        let right_name: String = chars[right..stop].iter().collect();
        let bare = |name: &String, next: Option<&char>| {
            slices.contains(name) && !matches!(next, Some('[' | '.' | '('))
        };
        if bare(&left_name, chars.get(left)) && start < left || bare(&right_name, chars.get(stop)) {
            found.push(Violation::SliceComparison);
        }
    }
    found
}

fn scan(source: &str) -> Vec<Violation> {
    let stripped = strip_comments(source);
    let mut violations = Vec::new();
    if stripped.replace(' ', "").contains("cfg(kani)") {
        violations.push(Violation::CfgKani);
    }
    let code = without_strings(&without_attributes(&stripped));
    let word_list = words(&code);
    for banned in BANNED_WORDS {
        if word_list.contains(&banned) {
            violations.push(Violation::BannedToken(banned.into()));
        }
    }
    for banned in BANNED_CALLS {
        if code.contains(&format!(".{banned}(")) || code.contains(&format!("::{banned}(")) {
            violations.push(Violation::BannedToken(banned.into()));
        }
    }
    if code.contains("as u64") || code.contains("as u128") {
        violations.push(Violation::BannedToken("word cast".into()));
    }
    if word_list
        .iter()
        .any(|word| word.starts_with("0x") && word.len() >= 10)
    {
        violations.push(Violation::BannedToken("word-sized mask".into()));
    }
    violations.extend(slice_comparisons(&code));
    // Every `for` header is `for <index> in 0..<NAME> {` with NAME a const
    // defined in the source.
    let consts: Vec<&str> = word_list
        .windows(2)
        .filter(|pair| pair[0] == "const")
        .map(|pair| pair[1])
        .collect();
    let mut rest = code.as_str();
    while let Some(at) = rest.find("for ") {
        let before = rest[..at].chars().last();
        let header: String = rest[at..].chars().take_while(|c| *c != '{').collect();
        rest = &rest[at + 4..];
        if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let parts: Vec<&str> = header.split_whitespace().collect();
        let ok = parts.len() == 4
            && parts[2] == "in"
            && parts[3]
                .strip_prefix("0..")
                .is_some_and(|name| consts.contains(&name) && name == name.to_uppercase());
        if !ok {
            violations.push(Violation::LoopHeader(header.trim().to_owned()));
        }
    }
    let functions = defined_functions(&code);
    let names: Vec<&str> = functions.iter().map(|(name, _)| name.as_str()).collect();
    for (_, body) in &functions {
        for call in calls_in(body) {
            if !ALLOWED_CALLEES.contains(&call.as_str()) && !names.contains(&call.as_str()) {
                violations.push(Violation::Callee(call));
            }
        }
    }
    let edges: Vec<(&str, Vec<String>)> = functions
        .iter()
        .map(|(name, body)| {
            (
                name.as_str(),
                calls_in(body)
                    .into_iter()
                    .filter(|call| names.contains(&call.as_str()))
                    .collect(),
            )
        })
        .collect();
    for (start, _) in &edges {
        let mut stack: Vec<String> = edges
            .iter()
            .filter(|(name, _)| name == start)
            .flat_map(|(_, calls)| calls.clone())
            .collect();
        let mut seen: Vec<String> = Vec::new();
        while let Some(next) = stack.pop() {
            if next == *start {
                violations.push(Violation::Cycle);
                break;
            }
            if seen.contains(&next) {
                continue;
            }
            seen.push(next.clone());
            for (name, calls) in &edges {
                if *name == next {
                    stack.extend(calls.clone());
                }
            }
        }
    }
    violations.dedup();
    violations
}

fn carrier_source() -> String {
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/native_text.rs")).unwrap()
}

#[test]
#[trace("TC-952", "FR-404-AC-5")]
fn carrier_source_is_clean() {
    assert_eq!(scan(&carrier_source()), vec![]);
}

/// The one declared exemption: the host-only decisions of the constructor
/// (std UTF-8 validation and the owner's normalization, for the profiles the
/// proof path does not support) live in `normal_form.rs`, whose violations
/// are exactly these and nothing else.
#[test]
#[trace("TC-952", "FR-404-AC-5")]
fn host_only_gate_has_exactly_the_declared_violations() {
    let source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/native_text/normal_form.rs"
    ))
    .unwrap();
    let mut found: Vec<String> = scan(&source).iter().map(|v| format!("{v:?}")).collect();
    found.sort();
    found.dedup();
    let mut declared: Vec<String> = [
        "BannedToken(\"apply\")",
        "BannedToken(\"chars\")",
        "BannedToken(\"from_utf8\")",
        "Callee(\"eq\")",
        "Callee(\"from_utf8\")",
        "Callee(\"apply\")",
        "Callee(\"chars\")",
        "Callee(\"is_ok\")",
    ]
    .map(String::from)
    .to_vec();
    declared.sort();
    assert_eq!(found, declared);
}

fn seeded(base: &str, seed: &str, expected: &Violation) {
    let violations = scan(&format!("{base}\n{seed}"));
    assert!(
        violations.contains(expected),
        "seed {seed:?} was not rejected as {expected:?}: {violations:?}"
    );
}

#[test]
#[trace("TC-952", "FR-404-AC-5")]
fn scanner_rejects_each_seeded_violation() {
    let base = carrier_source();
    let token = |name: &str| Violation::BannedToken(name.into());
    seeded(&base, "#[cfg(kani)]\nfn seeded() {}", &Violation::CfgKani);
    seeded(&base, "fn seeded() { seeded(); }", &Violation::Cycle);
    seeded(
        &base,
        "fn seeded_a() { seeded_b(); }\nfn seeded_b() { seeded_a(); }",
        &Violation::Cycle,
    );
    seeded(&base, "fn seeded() { while true {} }", &token("while"));
    seeded(&base, "fn seeded() { loop {} }", &token("loop"));
    seeded(
        &base,
        "fn seeded(a: &[u8]) { for index in 0..a.len() { let _ = index; } }",
        &Violation::LoopHeader("for index in 0..a.len()".into()),
    );
    // Every listed iterator adapter, `chars` and `cmp`, each as a method call.
    for call in [
        "iter",
        "bytes",
        "chars",
        "map",
        "zip",
        "enumerate",
        "fold",
        "all",
        "any",
        "position",
        "windows",
        "chunks",
        "chunks_exact",
        "cmp",
    ] {
        seeded(
            &base,
            &format!("fn seeded(a: &[u8]) {{ let _ = a.{call}(); }}"),
            &token(call),
        );
    }
    // SWAR: word loads, the word cast and the multiply-and-mask step.
    for load in [
        "from_le_bytes",
        "from_be_bytes",
        "from_ne_bytes",
        "read_unaligned",
    ] {
        seeded(
            &base,
            &format!("fn seeded(a: [u8; 8]) {{ let _ = u64::{load}(a); }}"),
            &token(load),
        );
    }
    seeded(
        &base,
        "fn seeded(a: usize) -> u64 { a as u64 }",
        &token("word cast"),
    );
    seeded(
        &base,
        "fn seeded(w: usize) -> usize { (w * 0x0101010101010101) & 0x8080808080808080 }",
        &token("word-sized mask"),
    );
    // Slice comparisons by operator.
    for operator in ["==", "!=", "<", ">", "<=", ">="] {
        seeded(
            &base,
            &format!("fn seeded(a: &[u8], b: &[u8]) -> bool {{ a {operator} b }}"),
            &Violation::SliceComparison,
        );
    }
    // The shipped function and the expected-side helpers of the differential.
    for helper in ["compare_text", "admit_text", "reference_holds"] {
        seeded(
            &base,
            &format!("fn seeded(a: u32) {{ let _ = {helper}(a); }}"),
            &token(helper),
        );
    }
    seeded(
        &base,
        "fn seeded(a: u32) { let _ = a.order(a); }",
        &token("order"),
    );
    // An unlisted callee is rejected on its own, before a macro and negated.
    seeded(
        &base,
        "fn seeded(a: u32) { let _ = unlisted_helper(a); }",
        &Violation::Callee("unlisted_helper".into()),
    );
    seeded(
        &base,
        "fn seeded(a: u32) { let _ = unlisted_helper(a); assert!(a > 0); }",
        &Violation::Callee("unlisted_helper".into()),
    );
    seeded(
        &base,
        "fn seeded(a: u32) -> bool { !unlisted_helper(a) }",
        &Violation::Callee("unlisted_helper".into()),
    );
}

// ---- AC-6 .. AC-9: the evidence record ----

struct Row {
    operation: String,
    cells: Vec<String>,
}

fn evidence_rows() -> Vec<Row> {
    let record = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/spec/test-cases/TC-952-kani-tractable-native-text-carrier.md"
    ))
    .unwrap();
    let section = record.split("## Evidence record").nth(1).unwrap();
    let section = section.split("## Expected Results").next().unwrap();
    section
        .lines()
        .filter(|line| {
            line.starts_with('|') && !line.contains("---") && !line.contains("Operation")
        })
        .map(|line| {
            let cells: Vec<String> = line
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_owned())
                .collect();
            Row {
                operation: cells[0].clone(),
                cells,
            }
        })
        .collect()
}

fn probe(rows: &[Row], capacity: usize) -> &Row {
    let label = format!("Probe harness of FR-404 Behavior 9, C = {capacity}");
    rows.iter()
        .find(|row| row.operation == label)
        .unwrap_or_else(|| panic!("record row {label}"))
}

#[test]
#[trace("TC-952", "FR-404-AC-6", "FR-404-AC-8")]
fn probe_records_are_executed_and_choose_the_capacity() {
    let rows = evidence_rows();
    assert!(rows
        .iter()
        .all(|row| !row.cells.iter().any(|c| c == "PENDING")));
    let mut chosen = None;
    for capacity in [64, 32] {
        let row = probe(&rows, capacity);
        let outcome = row.cells[1].as_str();
        assert!(
            [
                "VERIFIED",
                "TIMEOUT",
                "OUT-OF-MEMORY",
                "FAILED",
                "INCONCLUSIVE"
            ]
            .contains(&outcome),
            "terminal outcome {outcome:?}"
        );
        assert!(row.cells[2].ends_with(" s"), "wall time {:?}", row.cells[2]);
        assert!(
            row.cells[3].ends_with(" bytes"),
            "measured peak {:?}",
            row.cells[3]
        );
        let peak: u64 = row.cells[3].trim_end_matches(" bytes").parse().unwrap();
        assert!(peak > 0);
        assert_eq!(row.cells[4], "0.68.0");
        if outcome == "VERIFIED" {
            assert!(peak < 16 * 1024 * 1024 * 1024, "peak fits 16 GiB");
            chosen.get_or_insert(capacity);
        }
    }
    assert_eq!(
        C,
        chosen.unwrap_or(32),
        "C is the largest VERIFIED candidate, else 32"
    );
}

/// Behavior 10's consistency only. It passes on a `NOT RUN` record by design
/// and is not an AC-7 or AC-9 binder: those criteria need an executed
/// FR-401 record that does not exist, so no test claims them.
#[test]
fn normalizing_profiles_are_supported_only_on_a_verified_record() {
    let rows = evidence_rows();
    let row = rows
        .iter()
        .find(|row| row.operation == "FR-401 normalization at the chosen C")
        .expect("normalization row");
    assert_ne!(row.cells[1], "PENDING");
    assert!(row.cells[5].contains("owner pending planner ruling") || row.cells[1] == "VERIFIED");
    let supported = row.cells[1] == "VERIFIED";
    for profile in [
        TextProfile::Nfc,
        TextProfile::Nfd,
        TextProfile::Nfkc,
        TextProfile::Nfkd,
    ] {
        assert_eq!(NativeText::proof_path_supports(profile), supported);
    }
}
