// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exercise the actual owner's public Unicode profile from an ordinary consumer.

use quire_exact::{NormalizationForm, TextProfile, UNICODE_TEXT_DEFINITION, UNICODE_VERSION};

fn main() {
    assert_eq!(UNICODE_VERSION, (17, 0, 0));
    assert_eq!(
        UNICODE_TEXT_DEFINITION,
        "quire.value.text.unicode-17.0.0/v1"
    );
    for (profile, form, input, expected) in [
        (TextProfile::Nfc, NormalizationForm::Nfc, "e\u{301}", "é"),
        (TextProfile::Nfd, NormalizationForm::Nfd, "é", "e\u{301}"),
        (
            TextProfile::Nfkc,
            NormalizationForm::Nfkc,
            "\u{FB03}",
            "ffi",
        ),
        (
            TextProfile::Nfkd,
            NormalizationForm::Nfkd,
            "\u{212B}",
            "A\u{30A}",
        ),
    ] {
        assert_eq!(profile.normalization(), Some(form));
        assert_eq!(profile.table_definition(), Some(UNICODE_TEXT_DEFINITION));
        let normalized: String = form.apply(input).collect();
        assert_eq!(normalized, expected, "{profile:?}");
        assert_ne!(normalized, form.apply("distinct").collect::<String>());
    }
    assert_eq!(TextProfile::UnicodeScalars.table_definition(), None);
    assert_eq!(TextProfile::BinaryUtf8.table_definition(), None);
}
