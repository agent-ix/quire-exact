// SPDX-License-Identifier: AGPL-3.0-or-later
//! The proof-path text normalization against the shipped normalization
//! (FR-401, TC-949). The loop-bound measurement (AC-9) is a unit test of the
//! module, `src/native_text/normalize/trip_tests.rs`, because the counters
//! exist under `cfg(test)` only.
mod generator;
mod scan;

use ix_trace_rs::trace;
use quire_exact::{
    admit_text, compare_text, normalize_text, ChargePoint, ComparisonOperator, Integer, LimitKind,
    Meter, NormalizationForm, NormalizeRefusal, ScalarLimits, TextPayload, TextProfile, TextType,
};
use unicode_normalization::char::{
    canonical_combining_class, compose, decompose_canonical, decompose_compatible,
};

const FORMS: [NormalizationForm; 4] = [
    NormalizationForm::Nfc,
    NormalizationForm::Nfd,
    NormalizationForm::Nfkc,
    NormalizationForm::Nfkd,
];

fn profile_of(form: NormalizationForm) -> TextProfile {
    match form {
        NormalizationForm::Nfc => TextProfile::Nfc,
        NormalizationForm::Nfd => TextProfile::Nfd,
        NormalizationForm::Nfkc => TextProfile::Nfkc,
        NormalizationForm::Nfkd => TextProfile::Nfkd,
    }
}

/// The function under test over `text`, with zeroed buffers.
fn run<const N: usize, const D: usize, const R: usize>(
    form: NormalizationForm,
    text: &[u8],
) -> Result<(String, usize), NormalizeRefusal> {
    let mut payload = [0u8; N];
    let len = text.len();
    payload[..len.min(N)].copy_from_slice(&text[..len.min(N)]);
    let mut output = [0u8; R];
    let size = normalize_text::<N, D, R>(form, &payload, len, &mut output)?;
    let text = String::from_utf8(output[..size.bytes].to_vec()).expect("UTF-8 output");
    assert!(output[size.bytes..].iter().all(|byte| *byte == 0));
    Ok((text, size.scalars))
}

fn shipped(form: NormalizationForm, text: &str) -> (String, usize) {
    let out: String = form.apply(text).collect();
    let count = out.chars().count();
    (out, count)
}

/// Whether the buffers of (64, 32, 64) hold `text` under `form`: the
/// intermediate decomposition within 32 scalars and the output within 64
/// bytes.
fn fits(form: NormalizationForm, text: &str) -> bool {
    let decomposing = match form {
        NormalizationForm::Nfc | NormalizationForm::Nfd => NormalizationForm::Nfd,
        NormalizationForm::Nfkc | NormalizationForm::Nfkd => NormalizationForm::Nfkd,
    };
    shipped(decomposing, text).1 <= 32 && shipped(form, text).0.len() <= 64
}

fn check(form: NormalizationForm, text: &str) {
    let actual = run::<64, 32, 64>(form, text.as_bytes());
    if fits(form, text) {
        assert_eq!(
            actual.expect("fits"),
            shipped(form, text),
            "{form:?} {text:?}"
        );
    } else {
        assert_eq!(
            actual,
            Err(NormalizeRefusal::ExceedsCapacity),
            "{form:?} {text:?}"
        );
    }
}

const PAIR_SCALARS: [char; 21] = [
    'a',
    'u',
    'A',
    '\u{300}',
    '\u{301}',
    '\u{307}',
    '\u{308}',
    '\u{310}',
    '\u{323}',
    '\u{915}',
    '\u{93C}',
    '\u{1100}',
    '\u{1161}',
    '\u{11A8}',
    '\u{AC00}',
    '\u{D7A3}',
    '\u{1F82}',
    '\u{1F600}',
    '\u{212B}',
    '\u{FB03}',
    '\u{FDFA}',
];

const SEQUENCES: [&str; 6] = [
    "u\u{308}\u{301}",
    "a\u{307}\u{323}",
    "a\u{310}\u{300}",
    "\u{915}\u{93C}",
    "\u{1100}\u{1161}\u{11A8}",
    "",
];

fn inputs() -> Vec<String> {
    let mut all = Vec::new();
    for first in PAIR_SCALARS {
        for second in PAIR_SCALARS {
            all.push(format!("{first}{second}"));
        }
    }
    all.extend(SEQUENCES.map(String::from));
    all
}

#[test]
#[trace("TC-949", "FR-401-AC-1")]
fn every_scalar_pairs_and_sequences_equal_the_shipped_normalization() {
    for form in FORMS {
        for text in inputs() {
            check(form, &text);
        }
    }
    let mut buffer = [0u8; 4];
    for c in generator::scalars() {
        let text = c.encode_utf8(&mut buffer);
        for form in FORMS {
            check(form, text);
        }
    }
}

#[test]
#[trace("TC-949", "FR-401-AC-1")]
fn the_named_sequences_have_the_named_results() {
    let nfc = NormalizationForm::Nfc;
    assert_eq!(
        run::<64, 32, 64>(nfc, "u\u{308}\u{301}".as_bytes())
            .unwrap()
            .0,
        "\u{1D8}"
    );
    assert_eq!(
        run::<64, 32, 64>(nfc, "a\u{307}\u{323}".as_bytes())
            .unwrap()
            .0,
        "\u{1EA1}\u{307}"
    );
    assert_eq!(
        run::<64, 32, 64>(nfc, "a\u{310}\u{300}".as_bytes())
            .unwrap()
            .0,
        "a\u{310}\u{300}"
    );
    assert_eq!(
        run::<64, 32, 64>(nfc, "\u{915}\u{93C}".as_bytes())
            .unwrap()
            .0,
        "\u{915}\u{93C}"
    );
    assert_eq!(
        run::<64, 32, 64>(nfc, "\u{1100}\u{1161}\u{11A8}".as_bytes())
            .unwrap()
            .0,
        "\u{AC01}"
    );
    assert_eq!(run::<64, 32, 64>(nfc, b"").unwrap(), (String::new(), 0));
}

fn tolerates_len(text: &[u8], len: usize) -> Result<(), NormalizeRefusal> {
    let mut payload = [0u8; 64];
    payload[..text.len()].copy_from_slice(text);
    let mut output = [0u8; 64];
    normalize_text::<64, 32, 64>(NormalizationForm::Nfd, &payload, len, &mut output).map(|_| ())
}

#[test]
#[trace("TC-949", "FR-401-AC-2")]
fn refusals_are_exact() {
    let nfd = NormalizationForm::Nfd;
    let nfkd = NormalizationForm::Nfkd;
    let ypogegrammeni = "\u{1F82}".as_bytes();
    assert_eq!(ypogegrammeni.len(), 3);
    assert_eq!(shipped(nfd, "\u{1F82}").0.len(), 8);
    assert!(run::<64, 32, 9>(nfd, ypogegrammeni).is_ok());
    assert!(run::<64, 32, 8>(nfd, ypogegrammeni).is_ok());
    assert_eq!(
        run::<64, 32, 7>(nfd, ypogegrammeni),
        Err(NormalizeRefusal::ExceedsCapacity)
    );
    let fdfa = "\u{FDFA}".as_bytes();
    assert_eq!(shipped(nfkd, "\u{FDFA}").1, 18);
    assert!(run::<64, 18, 64>(nfkd, fdfa).is_ok());
    assert_eq!(
        run::<64, 17, 64>(nfkd, fdfa),
        Err(NormalizeRefusal::ExceedsCapacity)
    );
    assert_eq!(run::<64, 0, 0>(nfd, b"").unwrap(), (String::new(), 0));
    assert_eq!(
        tolerates_len(b"", 65),
        Err(NormalizeRefusal::LengthAboveCapacity)
    );
    assert!(tolerates_len(&[b'a'; 32], 32).is_ok());
    for bad in [
        &[0x80u8][..],
        &[0xC3, 0x28],
        &[0xC0, 0x80],
        &[0xED, 0xA0, 0x80],
        &[0xF4, 0x90, 0x80, 0x80],
        &[0xE2, 0x82],
    ] {
        assert_eq!(
            run::<64, 32, 64>(nfd, bad),
            Err(NormalizeRefusal::InvalidUtf8),
            "{bad:?}"
        );
    }
}

/// The ratio of FR-400 for each form: the largest number of payload scalars
/// one retained scalar can come from (K_D and K_KD are both 4).
fn ratio(form: NormalizationForm) -> usize {
    match form {
        NormalizationForm::Nfc | NormalizationForm::Nfkc => 4,
        NormalizationForm::Nfd | NormalizationForm::Nfkd => 1,
    }
}

macro_rules! at_max {
    ($max:literal, $ratio:literal, $form:expr, $text:expr) => {
        run::<64, { $ratio * $max }, { 4 * $max }>($form, $text)
    };
}

fn at(
    max: usize,
    form: NormalizationForm,
    text: &[u8],
) -> Result<(String, usize), NormalizeRefusal> {
    match (max, ratio(form)) {
        (1, 1) => at_max!(1, 1, form, text),
        (2, 1) => at_max!(2, 1, form, text),
        (3, 1) => at_max!(3, 1, form, text),
        (4, 1) => at_max!(4, 1, form, text),
        (1, 4) => at_max!(1, 4, form, text),
        (2, 4) => at_max!(2, 4, form, text),
        (3, 4) => at_max!(3, 4, form, text),
        (4, 4) => at_max!(4, 4, form, text),
        _ => unreachable!(),
    }
}

#[test]
#[trace("TC-949", "FR-401-AC-2")]
fn an_admitted_scalar_fits_the_ratio_and_four_bytes_per_scalar() {
    let mut buffer = [0u8; 4];
    for c in generator::scalars() {
        let text = c.encode_utf8(&mut buffer);
        for form in FORMS {
            let (expected, count) = shipped(form, text);
            for max in 1..=4 {
                if count <= max {
                    let (actual, scalars) = at(max, form, text.as_bytes())
                        .unwrap_or_else(|refusal| panic!("{form:?} {c:?} {max}: {refusal}"));
                    assert_eq!((actual, scalars), (expected.clone(), count));
                }
            }
        }
    }
    // The admission of the owner agrees on the attaining scalar.
    let payload = TextPayload::from_utf8("\u{1F82}".as_bytes()).unwrap();
    let text_type = TextType::new(0, 1, TextProfile::Nfc).unwrap();
    let mut meter = Meter::new(limits());
    assert!(admit_text(&payload, &text_type, &mut meter)
        .completed()
        .is_some());
}

fn limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    }
}

#[test]
#[trace("TC-949", "FR-401-AC-3")]
fn normalization_allocates_nothing() {
    let mut prepared: Vec<([u8; 64], usize)> = Vec::new();
    let mut buffer = [0u8; 4];
    let texts: Vec<String> = inputs()
        .into_iter()
        .chain(
            generator::scalars()
                .step_by(97)
                .map(|c| c.encode_utf8(&mut buffer).to_owned()),
        )
        .collect();
    for text in &texts {
        let mut payload = [0u8; 64];
        payload[..text.len()].copy_from_slice(text.as_bytes());
        prepared.push((payload, text.len()));
    }
    let mut output = [0u8; 64];
    let info = allocation_counter::measure(|| {
        for (payload, len) in &prepared {
            for form in FORMS {
                let _ = normalize_text::<64, 32, 64>(form, payload, *len, &mut output);
            }
        }
    });
    assert_eq!(info.count_total, 0);
}

fn admitted(profile: TextProfile, text: &str) -> quire_exact::Text {
    let text_type = TextType::new(0, 64, profile).unwrap();
    let payload = TextPayload::from_utf8(text.as_bytes()).unwrap();
    admit_text(&payload, &text_type, &mut Meter::new(limits()))
        .completed()
        .expect("admitted under Text[0,64]")
}

#[test]
#[trace("TC-949", "FR-401-AC-5", "FR-401-AC-6")]
fn order_and_counts_match_the_shipped_compare() {
    let texts = inputs();
    for form in FORMS {
        let profile = profile_of(form);
        for left in &texts {
            for right in texts.iter().step_by(5) {
                if !fits(form, left) || !fits(form, right) {
                    continue;
                }
                let (left_out, left_count) = run::<64, 32, 64>(form, left.as_bytes()).unwrap();
                let (right_out, right_count) = run::<64, 32, 64>(form, right.as_bytes()).unwrap();
                let order = left_out.as_bytes().cmp(right_out.as_bytes());
                let (owner_left, owner_right) = (admitted(profile, left), admitted(profile, right));
                let mut capture_meter = Meter::new(limits());
                capture_meter.enable_charge_capture(4096, 4096);
                for operator in ComparisonOperator::ALL {
                    let mut meter = Meter::new(limits());
                    meter.enable_charge_capture(4096, 4096);
                    let outcome = compare_text(operator, &owner_left, &owner_right, &mut meter)
                        .unwrap()
                        .completed()
                        .unwrap();
                    assert_eq!(
                        outcome,
                        operator.holds(order),
                        "{form:?} {left:?} {right:?}"
                    );
                    let capture = meter.disable_charge_capture().unwrap().unwrap();
                    let amounts: Vec<Integer> = capture
                        .events()
                        .iter()
                        .filter(|event| event.point == ChargePoint::TextNormalizeOutput)
                        .map(|event| {
                            let (kind, amount) = &event.sizes[0];
                            assert_eq!(*kind, LimitKind::NormalizedScalars);
                            amount.clone()
                        })
                        .collect();
                    let expected: Vec<Integer> = (1..=(left_count + right_count) as u64)
                        .map(Integer::from)
                        .collect();
                    assert_eq!(amounts, expected, "{form:?} {left:?} {right:?}");
                }
            }
        }
    }
}

fn run_of_marks() -> String {
    let marks = ['\u{301}', '\u{323}', '\u{334}'];
    let mut text = String::from("a");
    for index in 0..31 {
        text.push(marks[index % 3]);
    }
    text
}

#[test]
#[trace("TC-949", "FR-401-AC-7")]
fn long_runs_of_non_starters_reorder_and_respect_the_buffer() {
    let text = run_of_marks();
    for form in FORMS {
        assert_eq!(
            run::<64, 32, 64>(form, text.as_bytes()).unwrap(),
            shipped(form, &text)
        );
    }
    for form in [NormalizationForm::Nfd, NormalizationForm::Nfkd] {
        assert_eq!(
            run::<64, 31, 64>(form, text.as_bytes()),
            Err(NormalizeRefusal::ExceedsCapacity)
        );
    }
    let short = "a\u{301}\u{323}\u{334}\u{301}\u{323}";
    for form in FORMS {
        assert_eq!(
            run::<64, 32, 64>(form, short.as_bytes()).unwrap(),
            shipped(form, short)
        );
    }
}

#[test]
#[trace("TC-949", "FR-401-AC-8")]
fn the_checked_in_tables_equal_a_regeneration() {
    let regenerated = generator::render();
    if std::env::var_os("QUIRE_EXACT_GENERATE_TABLES").is_some() {
        std::fs::write(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/native_text/normalize/tables.rs"
            ),
            &regenerated,
        )
        .unwrap();
        return;
    }
    let checked_in = include_str!("../../src/native_text/normalize/tables.rs");
    assert!(
        checked_in == regenerated,
        "tables.rs differs from the generator output"
    );
}

fn decomposition(c: char, f: fn(char, &mut dyn FnMut(char))) -> Vec<char> {
    let mut out = Vec::new();
    f(c, &mut |d| out.push(d));
    out
}

#[test]
#[trace("TC-949", "FR-401-AC-8")]
fn every_table_entry_equals_the_public_api() {
    let tables = generator::build();
    let canon: std::collections::BTreeMap<u32, &Vec<u32>> =
        tables.canon.iter().map(|(k, v)| (*k, v)).collect();
    let compat: std::collections::BTreeMap<u32, &Vec<u32>> =
        tables.compat.iter().map(|(k, v)| (*k, v)).collect();
    for c in generator::scalars() {
        if generator::is_hangul_syllable(c) {
            continue;
        }
        let canonical = decomposition(c, |c, f| decompose_canonical(c, f));
        let table: Vec<char> = canon.get(&(c as u32)).map_or(vec![c], |v| {
            v.iter().map(|d| char::from_u32(*d).unwrap()).collect()
        });
        assert_eq!(table, canonical, "canonical {c:?}");
        let compatible = decomposition(c, |c, f| decompose_compatible(c, f));
        let table: Vec<char> = compat
            .get(&(c as u32))
            .or(canon.get(&(c as u32)))
            .map_or(vec![c], |v| {
                v.iter().map(|d| char::from_u32(*d).unwrap()).collect()
            });
        assert_eq!(table, compatible, "compatibility {c:?}");
        let class = tables
            .ccc
            .iter()
            .find(|(start, end, _)| (*start..=*end).contains(&(c as u32)))
            .map_or(0, |row| row.2);
        assert_eq!(class, canonical_combining_class(c), "class {c:?}");
    }
    for (l, r, c) in &tables.compose {
        let composed = compose(char::from_u32(*l).unwrap(), char::from_u32(*r).unwrap());
        assert_eq!(composed.map(|c| c as u32), Some(*c));
    }
}

/// The pair sweep: for every scalar a and every right element b of a table
/// triple, `compose(a, b)` is `Some` exactly when the table holds (a, b, c)
/// or the pair is a Hangul composition. Run by `make test-exhaustive`.
#[test]
#[ignore = "exhaustive pair sweep; run with --release -- --ignored"]
#[trace("TC-949", "FR-401-AC-8")]
fn every_composable_pair_is_in_the_table() {
    let tables = generator::build();
    let rights: std::collections::BTreeSet<u32> = tables.compose.iter().map(|t| t.1).collect();
    let pairs: std::collections::BTreeSet<(u32, u32)> =
        tables.compose.iter().map(|t| (t.0, t.1)).collect();
    for a in generator::scalars() {
        for b in &rights {
            let b = char::from_u32(*b).unwrap();
            let public = compose(a, b).is_some();
            let hangul = ('\u{1100}'..'\u{1113}').contains(&a)
                && ('\u{1161}'..'\u{1176}').contains(&b)
                || ('\u{AC00}'..='\u{D7A3}').contains(&a) && ('\u{11A8}'..'\u{11C3}').contains(&b);
            assert_eq!(
                public,
                pairs.contains(&(a as u32, b as u32)) || hangul,
                "{a:?} {b:?}"
            );
        }
    }
}

#[test]
#[trace("TC-949", "FR-401-AC-4")]
fn the_module_passes_the_scan() {
    assert_eq!(scan::scan(&scan::module_source()), Vec::<String>::new());
}

#[test]
#[trace("TC-949", "FR-401-AC-4")]
fn the_scanner_rejects_each_seeded_violation() {
    scan::each_seed_is_rejected();
}
