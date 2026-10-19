// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public hexadecimal byte decoding, with no identity construction by the parser.

use quire_exact::{parse_hex32, NodeKey};

/// Trace: FR-384-AC-1, TC-934.
#[test]
fn parses_exact_lowercase_hex_bytes_and_round_trips_existing_encoder() {
    let text = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    let expected = core::array::from_fn(|index| index as u8);
    assert_eq!(parse_hex32(text), Some(expected));

    let encoded = NodeKey::from_digest(expected).to_string();
    assert_eq!(encoded, text);
    assert_eq!(parse_hex32(&encoded), Some(expected));
    assert_eq!(parse_hex32(&"f".repeat(64)), Some([0xff; 32]));
}

/// Trace: FR-384-AC-2, TC-934.
#[test]
fn refuses_noncanonical_length_and_characters_at_every_position() {
    let valid = "a".repeat(64);
    for invalid in [
        "",
        &"a".repeat(63),
        &"a".repeat(65),
        &format!("0x{}", "a".repeat(62)),
        &format!(" {}", "a".repeat(63)),
        &format!("{} ", "a".repeat(63)),
        &format!("+{}", "a".repeat(63)),
        &format!("-{}", "a".repeat(63)),
        &format!("é{}", "a".repeat(62)),
        &format!("{}é", "a".repeat(62)),
    ] {
        assert_eq!(parse_hex32(invalid), None, "accepted {invalid:?}");
    }

    for bad in ['A', 'G', 'g', 'x', '/', ':', '\n', 'é', '💡'] {
        for index in 0..64 {
            let mut invalid = valid.clone();
            invalid.replace_range(index..index + 1, &bad.to_string());
            assert_eq!(parse_hex32(&invalid), None, "accepted {bad:?} at {index}");
        }
    }
}
