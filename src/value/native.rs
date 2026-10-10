// SPDX-License-Identifier: AGPL-3.0-or-later
//! Canonical native UUID and POSIX-epoch nanosecond values.

use alloc::string::ToString;
use core::fmt::{self, Write};

use super::{Component, ConstructionCause, ConstructionRefusal};

/// An opaque UUID represented by its 16 network-order octets.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Uuid([u8; 16]);

impl Uuid {
    /// Admit only the lowercase ASCII `8-4-4-4-12` UUID spelling.
    pub fn from_canonical_text(text: &str) -> Result<Self, ConstructionRefusal> {
        let bytes = text.as_bytes();
        let invalid = || ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::UuidNoncanonical,
        };
        if bytes.len() != 36 || [8, 13, 18, 23].iter().any(|&index| bytes[index] != b'-') {
            return Err(invalid());
        }
        let mut octets = [0_u8; 16];
        let mut digits = bytes.iter().copied().filter(|byte| *byte != b'-');
        for octet in &mut octets {
            let high = hex_digit(digits.next().ok_or_else(invalid)?).ok_or_else(invalid)?;
            let low = hex_digit(digits.next().ok_or_else(invalid)?).ok_or_else(invalid)?;
            *octet = (high << 4) | low;
        }
        if digits.next().is_some() {
            return Err(invalid());
        }
        Ok(Self(octets))
    }

    /// The uninterpreted 16 network-order octets.
    pub fn octets(self) -> [u8; 16] {
        self.0
    }

    pub(crate) fn canonical_bytes(self) -> [u8; 36] {
        let mut bytes = [b'-'; 36];
        let mut position = 0;
        for octet in self.0 {
            if matches!(position, 8 | 13 | 18 | 23) {
                position += 1;
            }
            bytes[position] = hex_char(octet >> 4);
            bytes[position + 1] = hex_char(octet & 0x0f);
            position += 2;
        }
        bytes
    }
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn hex_char(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=15 => b'a' + digit - 10,
        _ => unreachable!("a nibble is at most 15"),
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.canonical_bytes() {
            formatter.write_char(char::from(byte))?;
        }
        Ok(())
    }
}

/// Signed nanoseconds since the POSIX epoch, with no leap-second coordinate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp(i128);

impl Timestamp {
    /// Admit only canonical ASCII decimal text within the full signed `i128` range.
    pub fn from_canonical_text(text: &str) -> Result<Self, ConstructionRefusal> {
        let invalid = || ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::TimestampNoncanonical,
        };
        let bytes = text.as_bytes();
        let digits = if bytes.first() == Some(&b'-') {
            &bytes[1..]
        } else {
            bytes
        };
        if digits.is_empty()
            || !digits.iter().all(u8::is_ascii_digit)
            || (digits.len() > 1 && digits[0] == b'0')
            || (bytes.first() == Some(&b'-') && digits == b"0")
        {
            return Err(invalid());
        }
        text.parse::<i128>()
            .map(Self)
            .map_err(|_| ConstructionRefusal {
                component: Component::Value,
                cause: ConstructionCause::TimestampOutOfDomain,
            })
    }

    /// Exact signed POSIX-epoch nanoseconds.
    pub fn nanoseconds(self) -> i128 {
        self.0
    }

    pub(crate) fn canonical_text(self) -> alloc::string::String {
        self.0.to_string()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    /// Native UUID octets retain all bits and reject every noncanonical spelling.
    #[trace("TC-918", "FR-370-AC-6", "FR-370-AC-8")]
    #[test]
    fn uuid_canonical_text_preserves_network_octets() {
        for (text, expected) in [
            ("00000000-0000-0000-0000-000000000000", [0; 16]),
            ("ffffffff-ffff-ffff-ffff-ffffffffffff", [0xff; 16]),
            (
                "00112233-4455-6677-8899-aabbccddeeff",
                [
                    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
                    0xdd, 0xee, 0xff,
                ],
            ),
        ] {
            let value = Uuid::from_canonical_text(text).expect("canonical UUID");
            assert_eq!(value.octets(), expected);
            assert_eq!(value.to_string(), text);
        }
        for text in [
            "00112233-4455-6677-8899-AABBCCDDEEFF",
            "{00112233-4455-6677-8899-aabbccddeeff}",
            "urn:uuid:00112233-4455-6677-8899-aabbccddeeff",
            "001122334455-6677-8899-aabbccddeeff",
            "00112233_4455-6677-8899-aabbccddeeff",
            "00112233-4455-6677-8899-aabbccddeef",
            "00112233-4455-6677-8899-aabbccddeefg",
            " 00112233-4455-6677-8899-aabbccddeeff",
        ] {
            assert_eq!(
                Uuid::from_canonical_text(text),
                Err(ConstructionRefusal {
                    component: Component::Value,
                    cause: ConstructionCause::UuidNoncanonical,
                }),
                "{text}"
            );
        }
    }

    /// Canonical Timestamp syntax precedes signed-range checking.
    #[trace("TC-918", "FR-370-AC-7", "FR-370-AC-8")]
    #[test]
    fn timestamp_canonical_text_preserves_full_signed_nanoseconds() {
        for (text, expected) in [
            ("-170141183460469231731687303715884105728", i128::MIN),
            ("-1", -1),
            ("0", 0),
            ("1", 1),
            ("170141183460469231731687303715884105727", i128::MAX),
        ] {
            let value = Timestamp::from_canonical_text(text).expect("canonical timestamp");
            assert_eq!(value.nanoseconds(), expected);
            assert_eq!(value.to_string(), text);
        }
        for text in [
            "",
            "-",
            "-0",
            "+1",
            "01",
            "-01",
            "1.0",
            "1e0",
            "１",
            "1970-01-01T00:00:00Z",
            " 1",
            "1 ",
            "0170141183460469231731687303715884105728",
        ] {
            assert_eq!(
                Timestamp::from_canonical_text(text),
                Err(ConstructionRefusal {
                    component: Component::Value,
                    cause: ConstructionCause::TimestampNoncanonical,
                }),
                "{text}"
            );
        }
        for text in [
            "-170141183460469231731687303715884105729",
            "170141183460469231731687303715884105728",
        ] {
            assert_eq!(
                Timestamp::from_canonical_text(text),
                Err(ConstructionRefusal {
                    component: Component::Value,
                    cause: ConstructionCause::TimestampOutOfDomain,
                }),
                "{text}"
            );
        }
    }
}
