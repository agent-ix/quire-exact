// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani-tractable native Text carrier (FR-404).
//!
//! A [`NativeText`] holds the owner's [`TextProfile`], a payload of at most
//! [`NATIVE_TEXT_CAPACITY`] bytes and the retained sequence of at most that
//! many bytes, in fixed arrays with every byte at or after a length zero. It
//! is `Copy`, owns no heap type and has no drop glue. [`compare_native_text`]
//! replaces [`crate::compare_text`] over two carriers with indexed byte loops
//! bounded by the capacity: no `chars()`, iterator adapter, word-sized load,
//! recursion or data-dependent loop bound, and one body for host and Kani
//! builds.

use crate::accounting::{length_amount, Charge, ChargePoint, LimitKind, Meter};
use crate::comparison::{ComparisonOperator, IllTyped, IllTypedCause};
use crate::outcome::{Outcome, Stop};
use crate::text::{TextNotAdmittedCause, TextProfile, TextType};
use core::cmp::Ordering;

mod normal_form;

/// The capacity C of a [`NativeText`] in bytes, for the payload and for the
/// retained sequence each. Fixed by the Kani probe recorded in TC-952.
pub const NATIVE_TEXT_CAPACITY: usize = 64;

/// The most scalars the retained sequences of two operands can emit.
const NORMALIZED_OUTPUT_BOUND: usize = 2 * NATIVE_TEXT_CAPACITY;

/// The refusals of [`NativeText::from_admitted`] (FR-404 Behavior 3).
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeTextRefusal {
    /// The payload or the retained sequence is longer than
    /// [`NATIVE_TEXT_CAPACITY`] bytes.
    #[error("text payload exceeds the native carrier capacity")]
    PayloadExceedsCapacity,
    /// The parts are not what admission would have produced.
    #[error("text parts are not admitted: {0:?}")]
    NotAdmitted(TextNotAdmittedCause),
}

/// A fixed-capacity text value: profile, payload and retained sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeText {
    profile: TextProfile,
    payload: [u8; NATIVE_TEXT_CAPACITY],
    payload_len: usize,
    retained: [u8; NATIVE_TEXT_CAPACITY],
    retained_len: usize,
}

impl NativeText {
    /// Build a carrier from parts that were already admitted.
    ///
    /// Precondition: the parts were already admitted by the owner's
    /// admission, so they are valid UTF-8. The two proof-path profiles
    /// (`unicode-scalars`, `binary-utf8`) do not re-validate UTF-8, which is
    /// not Kani-tractable; the four normalizing profiles are host-only and
    /// refuse parts that are not UTF-8 with `RetainedNotNormalForm`.
    ///
    /// A payload or retained sequence longer than the capacity refuses with
    /// [`NativeTextRefusal::PayloadExceedsCapacity`] before any other
    /// decision. Otherwise the retained sequence must be the retained
    /// sequence of the payload under the profile, then its profile length
    /// must lie within the type's bounds.
    pub fn from_admitted(
        text_type: TextType,
        payload: &[u8],
        retained: &[u8],
    ) -> Result<NativeText, NativeTextRefusal> {
        if payload.len() > NATIVE_TEXT_CAPACITY || retained.len() > NATIVE_TEXT_CAPACITY {
            return Err(NativeTextRefusal::PayloadExceedsCapacity);
        }
        let profile = text_type.profile();
        let normal = match profile.normalization() {
            Some(form) => normal_form::is_retained_of(form, payload, retained),
            None => same_bytes(payload, retained),
        };
        if !normal {
            return Err(NativeTextRefusal::NotAdmitted(
                TextNotAdmittedCause::RetainedNotNormalForm,
            ));
        }
        let length = match profile {
            TextProfile::BinaryUtf8 => retained.len(),
            TextProfile::UnicodeScalars
            | TextProfile::Nfc
            | TextProfile::Nfd
            | TextProfile::Nfkc
            | TextProfile::Nfkd => scalar_starts(retained),
        };
        let length = length_amount(length);
        if length < text_type.min() || length > text_type.max() {
            return Err(NativeTextRefusal::NotAdmitted(
                TextNotAdmittedCause::LengthOutOfDomain,
            ));
        }
        Ok(Self::zero_padded(profile, payload, retained))
    }

    /// Whether the proof path supports `profile`: the four normalizing
    /// profiles stay unsupported until a recorded Kani run of the FR-401
    /// normalization at the capacity is VERIFIED (FR-404 Behavior 10).
    pub const fn proof_path_supports(profile: TextProfile) -> bool {
        matches!(
            profile,
            TextProfile::UnicodeScalars | TextProfile::BinaryUtf8
        )
    }

    /// The profile of the carried text.
    pub const fn profile(&self) -> TextProfile {
        self.profile
    }

    /// The payload bytes, up to the payload length.
    pub fn payload(&self) -> &[u8] {
        &self.payload[..self.payload_len]
    }

    /// The retained-sequence bytes, up to the retained length.
    pub fn retained(&self) -> &[u8] {
        &self.retained[..self.retained_len]
    }

    /// The whole payload array, zero at and after the payload length.
    pub const fn payload_storage(&self) -> &[u8; NATIVE_TEXT_CAPACITY] {
        &self.payload
    }

    /// The whole retained array, zero at and after the retained length.
    pub const fn retained_storage(&self) -> &[u8; NATIVE_TEXT_CAPACITY] {
        &self.retained
    }

    /// The zero-padding step: copy each input below its length and nothing
    /// else. Takes no UTF-8 or admission decision.
    pub(crate) fn zero_padded(profile: TextProfile, payload: &[u8], retained: &[u8]) -> Self {
        let mut padded = Self {
            profile,
            payload: [0; NATIVE_TEXT_CAPACITY],
            payload_len: payload.len(),
            retained: [0; NATIVE_TEXT_CAPACITY],
            retained_len: retained.len(),
        };
        for index in 0..NATIVE_TEXT_CAPACITY {
            if index < payload.len() {
                padded.payload[index] = payload[index];
            }
            if index < retained.len() {
                padded.retained[index] = retained[index];
            }
        }
        padded
    }
}

/// Byte equality of two slices.
fn same_bytes(left: &[u8], right: &[u8]) -> bool {
    let mut same = left.len() == right.len();
    for index in 0..NATIVE_TEXT_CAPACITY {
        if index < left.len() && index < right.len() && left[index] != right[index] {
            same = false;
        }
    }
    same
}

/// The number of bytes that are not UTF-8 continuation bytes.
fn scalar_starts(bytes: &[u8]) -> usize {
    let mut count = 0;
    for index in 0..NATIVE_TEXT_CAPACITY {
        if index < bytes.len() && bytes[index] & 0xc0 != 0x80 {
            count += 1;
        }
    }
    count
}

/// Lexicographic byte order of two slices; a proper prefix orders first.
fn order_bytes(left: &[u8], right: &[u8]) -> Ordering {
    let mut ordering = Ordering::Equal;
    for index in 0..NATIVE_TEXT_CAPACITY {
        if ordering == Ordering::Equal && index < left.len() && index < right.len() {
            if left[index] < right[index] {
                ordering = Ordering::Less;
            } else if left[index] > right[index] {
                ordering = Ordering::Greater;
            }
        }
    }
    if ordering != Ordering::Equal {
        return ordering;
    }
    if left.len() < right.len() {
        Ordering::Less
    } else if left.len() > right.len() {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Compare two native text values. Operands of different profiles are
/// ill-typed and consume nothing. Charges are those of
/// [`crate::compare_text`].
pub fn compare_native_text(
    operator: ComparisonOperator,
    left: &NativeText,
    right: &NativeText,
    meter: &mut Meter,
) -> Result<Outcome<bool>, IllTyped> {
    if left.profile != right.profile {
        return Err(IllTyped {
            cause: IllTypedCause::DistinctTextProfiles,
        });
    }
    Ok(Outcome::from_stop(compare(operator, left, right, meter)))
}

fn compare(
    operator: ComparisonOperator,
    left: &NativeText,
    right: &NativeText,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    let inputs = length_amount(left.payload_len) + length_amount(right.payload_len);
    meter.charge(
        Charge::new(ChargePoint::TextInputBytes)
            .size(LimitKind::TextInputBytes, inputs)
            .size(LimitKind::ValueOccurrences, 2),
    )?;
    let decoded = length_amount(scalar_starts(left.payload()))
        + length_amount(scalar_starts(right.payload()));
    meter.charge(
        Charge::new(ChargePoint::TextDecodeScalars).size(LimitKind::TextScalars, decoded),
    )?;
    if left.profile.normalization().is_some() {
        meter.charge(Charge::new(ChargePoint::TextNormalizeInput))?;
        let first = scalar_starts(left.retained());
        let total = first + scalar_starts(right.retained());
        for index in 0..NORMALIZED_OUTPUT_BOUND {
            let emitted = index + 1;
            if emitted <= total {
                meter.charge(
                    Charge::new(ChargePoint::TextNormalizeOutput)
                        .size(LimitKind::NormalizedScalars, length_amount(emitted)),
                )?;
            }
        }
    }
    let ordering = order_bytes(left.retained(), right.retained());
    meter.charge(Charge::new(ChargePoint::TextResultRetain).results(1))?;
    // `holds` maps an ordering that `order_bytes` already decided to the
    // operator's truth value; it decides neither equality nor order.
    Ok(operator.holds(ordering))
}
