// SPDX-License-Identifier: AGPL-3.0-or-later
//! The proof-path text normalization (FR-401): the retained scalars of a
//! payload under `nfc`, `nfd`, `nfkc` or `nfkd`, over fixed-capacity buffers
//! and constant tables of this crate. It allocates nothing, charges no
//! meter, reads no `unicode-normalization` item and uses only indexed loops
//! `for <index> in 0..<NAME>` with `NAME` a constant of this module or one of
//! the const generics `N`, `D` and `R`.

use crate::text::NormalizationForm;

mod tables;
#[cfg(test)]
mod trips;

#[cfg(test)]
mod trip_tests;

/// One trip of the loop at `$site`, counted under `cfg(test)` only.
macro_rules! trip {
    ($site:ident) => {
        #[cfg(test)]
        trips::hit(trips::$site);
    };
}

/// A loop at `$site` is entered, counted under `cfg(test)` only.
macro_rules! entry {
    ($site:ident) => {
        #[cfg(test)]
        trips::enter(trips::$site);
    };
}

const CANON_STEPS: usize = tables::CANON_STEPS;
const COMPAT_STEPS: usize = tables::COMPAT_STEPS;
const CCC_STEPS: usize = tables::CCC_STEPS;
const COMPOSE_STEPS: usize = tables::COMPOSE_STEPS;

const HANGUL_SYLLABLE_BASE: u32 = 0xAC00;
const HANGUL_SYLLABLE_COUNT: u32 = 11172;
const HANGUL_LEAD_BASE: u32 = 0x1100;
const HANGUL_LEAD_COUNT: u32 = 19;
const HANGUL_VOWEL_BASE: u32 = 0x1161;
const HANGUL_VOWEL_COUNT: u32 = 21;
const HANGUL_TRAIL_BASE: u32 = 0x11A7;
const HANGUL_TRAIL_COUNT: u32 = 28;
const HANGUL_VOWEL_TRAIL: u32 = 588;
const SCALAR_MAX: u32 = 0x10_FFFF;

/// The refusals of [`normalize_text`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NormalizeRefusal {
    /// The intermediate form needs more than D scalars or the output more
    /// than R bytes.
    #[error("normalization exceeds the buffer capacity")]
    ExceedsCapacity,
    /// The payload bytes are not UTF-8.
    #[error("payload is not UTF-8")]
    InvalidUtf8,
    /// The length is above the payload capacity N.
    #[error("payload length exceeds the payload capacity")]
    LengthAboveCapacity,
}

/// The size of a normalized text written by [`normalize_text`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NormalizedSize {
    /// The number of retained scalars.
    pub scalars: usize,
    /// The number of UTF-8 bytes written to the output buffer.
    pub bytes: usize,
}

/// The unwind bound a Kani harness needs at the instantiation `N`, `D`, `R`:
/// the largest declared loop bound plus one.
pub const fn unwind_bound<const N: usize, const D: usize, const R: usize>() -> usize {
    let mut largest = N;
    if D > largest {
        largest = D;
    }
    if R > largest {
        largest = R;
    }
    if CANON_STEPS > largest {
        largest = CANON_STEPS;
    }
    if COMPAT_STEPS > largest {
        largest = COMPAT_STEPS;
    }
    if CCC_STEPS > largest {
        largest = CCC_STEPS;
    }
    if COMPOSE_STEPS > largest {
        largest = COMPOSE_STEPS;
    }
    largest + 1
}

/// Normalize `payload[..len]` under `form` into `output` as UTF-8, with `D`
/// scalars of intermediate form. Bytes of `output` past the returned size
/// are left as they were.
pub fn normalize_text<const N: usize, const D: usize, const R: usize>(
    form: NormalizationForm,
    payload: &[u8; N],
    len: usize,
    output: &mut [u8; R],
) -> Result<NormalizedSize, NormalizeRefusal> {
    if len > N {
        return Err(NormalizeRefusal::LengthAboveCapacity);
    }
    let compatibility = matches!(form, NormalizationForm::Nfkc | NormalizationForm::Nfkd);
    let mut scalars: [u32; D] = [0; D];
    let mut classes: [u8; D] = [0; D];
    let mut count = decompose(compatibility, payload, len, &mut scalars, &mut classes)?;
    reorder(count, &mut scalars, &mut classes);
    if matches!(form, NormalizationForm::Nfc | NormalizationForm::Nfkc) {
        count = recompose(count, &mut scalars, &mut classes);
    }
    let bytes = encode(count, &scalars, output)?;
    Ok(NormalizedSize {
        scalars: count,
        bytes,
    })
}

/// Decode `payload` and write the full decomposition of every scalar to
/// `scalars`, with the combining classes in `classes`. Returns the count.
fn decompose<const N: usize, const D: usize>(
    compatibility: bool,
    payload: &[u8; N],
    len: usize,
    scalars: &mut [u32; D],
    classes: &mut [u8; D],
) -> Result<usize, NormalizeRefusal> {
    let mut count = 0;
    let mut position = 0;
    entry!(DECODE);
    for _slot in 0..N {
        trip!(DECODE);
        if position < len {
            let Some((scalar, width)) = decode(payload, position, len) else {
                return Err(NormalizeRefusal::InvalidUtf8);
            };
            position += width;
            count = push_decomposed(compatibility, scalar, count, scalars, classes)?;
        }
    }
    Ok(count)
}

/// The byte of `payload` at `index`, zero at or after `len`.
fn byte_at<const N: usize>(payload: &[u8; N], index: usize, len: usize) -> u8 {
    if index < len && index < N {
        payload[index]
    } else {
        0
    }
}

/// The six payload bits of the continuation byte at `index`.
fn continuation<const N: usize>(payload: &[u8; N], index: usize, len: usize) -> Option<u32> {
    let byte = u32::from(byte_at(payload, index, len));
    if byte >= 0x80 && byte < 0xC0 {
        Some(byte & 0x3F)
    } else {
        None
    }
}

/// Decode the scalar at `position`: its value and its width in bytes, or
/// `None` for bytes that are not UTF-8.
fn decode<const N: usize>(payload: &[u8; N], position: usize, len: usize) -> Option<(u32, usize)> {
    let lead = u32::from(byte_at(payload, position, len));
    if lead < 0x80 {
        return Some((lead, 1));
    }
    let (width, minimum, initial) = if lead >= 0xC2 && lead < 0xE0 {
        (2, 0x80, lead & 0x1F)
    } else if lead >= 0xE0 && lead < 0xF0 {
        (3, 0x800, lead & 0x0F)
    } else if lead >= 0xF0 && lead < 0xF5 {
        (4, 0x1_0000, lead & 0x07)
    } else {
        return None;
    };
    if position + width > len {
        return None;
    }
    let mut value = (initial << 6) | continuation(payload, position + 1, len)?;
    if width >= 3 {
        value = (value << 6) | continuation(payload, position + 2, len)?;
    }
    if width >= 4 {
        value = (value << 6) | continuation(payload, position + 3, len)?;
    }
    let surrogate = value >= 0xD800 && value < 0xE000;
    if value < minimum || value > SCALAR_MAX || surrogate {
        return None;
    }
    Some((value, width))
}

/// Append `scalar` and its combining class at `count`.
fn push_one<const D: usize>(
    scalar: u32,
    count: usize,
    scalars: &mut [u32; D],
    classes: &mut [u8; D],
) -> Result<usize, NormalizeRefusal> {
    if count >= D {
        return Err(NormalizeRefusal::ExceedsCapacity);
    }
    scalars[count] = scalar;
    classes[count] = class_of(scalar);
    Ok(count + 1)
}

/// Append the full decomposition of `scalar`.
fn push_decomposed<const D: usize>(
    compatibility: bool,
    scalar: u32,
    count: usize,
    scalars: &mut [u32; D],
    classes: &mut [u8; D],
) -> Result<usize, NormalizeRefusal> {
    let offset = scalar.wrapping_sub(HANGUL_SYLLABLE_BASE);
    if offset < HANGUL_SYLLABLE_COUNT {
        let lead = HANGUL_LEAD_BASE + offset / HANGUL_VOWEL_TRAIL;
        let vowel = HANGUL_VOWEL_BASE + (offset % HANGUL_VOWEL_TRAIL) / HANGUL_TRAIL_COUNT;
        let trail = HANGUL_TRAIL_BASE + offset % HANGUL_TRAIL_COUNT;
        let mut next = push_one(lead, count, scalars, classes)?;
        next = push_one(vowel, next, scalars, classes)?;
        if trail != HANGUL_TRAIL_BASE {
            next = push_one(trail, next, scalars, classes)?;
        }
        return Ok(next);
    }
    let (from_compatibility, start, width, found) = lookup(compatibility, scalar);
    if !found {
        return push_one(scalar, count, scalars, classes);
    }
    if width > D {
        return Err(NormalizeRefusal::ExceedsCapacity);
    }
    let mut next = count;
    entry!(COPY);
    for k in 0..D {
        trip!(COPY);
        if k < width {
            let member = if from_compatibility {
                tables::COMPAT_POOL[start + k]
            } else {
                tables::CANON_POOL[start + k]
            };
            next = push_one(member, next, scalars, classes)?;
        }
    }
    Ok(next)
}

/// The table entry of `scalar`: whether it is in the compatibility table,
/// its start in that pool, its width, and whether it was found.
fn lookup(compatibility: bool, scalar: u32) -> (bool, usize, usize, bool) {
    if compatibility {
        let (start, width, found) = search_compat(scalar);
        if found {
            return (true, start, width, true);
        }
    }
    let (start, width, found) = search_canon(scalar);
    (false, start, width, found)
}

fn search_canon(scalar: u32) -> (usize, usize, bool) {
    let mut low = 0;
    let mut high = tables::CANON_COUNT;
    entry!(CANON_SEARCH);
    for _step in 0..CANON_STEPS {
        trip!(CANON_SEARCH);
        if low < high {
            let middle = (low + high) / 2;
            if tables::CANON_KEYS[middle] < scalar {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
    }
    if low < tables::CANON_COUNT && tables::CANON_KEYS[low] == scalar {
        (
            usize::from(tables::CANON_START[low]),
            usize::from(tables::CANON_LEN[low]),
            true,
        )
    } else {
        (0, 0, false)
    }
}

fn search_compat(scalar: u32) -> (usize, usize, bool) {
    let mut low = 0;
    let mut high = tables::COMPAT_COUNT;
    entry!(COMPAT_SEARCH);
    for _step in 0..COMPAT_STEPS {
        trip!(COMPAT_SEARCH);
        if low < high {
            let middle = (low + high) / 2;
            if tables::COMPAT_KEYS[middle] < scalar {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
    }
    if low < tables::COMPAT_COUNT && tables::COMPAT_KEYS[low] == scalar {
        (
            usize::from(tables::COMPAT_START[low]),
            usize::from(tables::COMPAT_LEN[low]),
            true,
        )
    } else {
        (0, 0, false)
    }
}

/// The canonical combining class of `scalar`.
fn class_of(scalar: u32) -> u8 {
    let mut low = 0;
    let mut high = tables::CCC_COUNT;
    entry!(CCC_SEARCH);
    for _step in 0..CCC_STEPS {
        trip!(CCC_SEARCH);
        if low < high {
            let middle = (low + high) / 2;
            if tables::CCC_START[middle] <= scalar {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
    }
    if low > 0 && scalar <= tables::CCC_END[low - 1] {
        tables::CCC_CLASS[low - 1]
    } else {
        0
    }
}

/// Canonical ordering: a stable sort of every run of non-starters by class.
fn reorder<const D: usize>(count: usize, scalars: &mut [u32; D], classes: &mut [u8; D]) {
    entry!(SORT_OUTER);
    for _pass in 0..D {
        trip!(SORT_OUTER);
        entry!(SORT_INNER);
        for index in 0..D {
            trip!(SORT_INNER);
            if index >= 1 && index < count {
                let before = classes[index - 1];
                let after = classes[index];
                if after != 0 && before > after {
                    classes[index - 1] = after;
                    classes[index] = before;
                    let scalar = scalars[index];
                    scalars[index] = scalars[index - 1];
                    scalars[index - 1] = scalar;
                }
            }
        }
    }
}

/// Canonical composition in place. Returns the new count.
fn recompose<const D: usize>(count: usize, scalars: &mut [u32; D], classes: &mut [u8; D]) -> usize {
    let mut written = 0;
    let mut starter = 0;
    let mut has_starter = false;
    let mut last_class = 0;
    entry!(COMPOSE);
    for index in 0..D {
        trip!(COMPOSE);
        if index < count {
            let scalar = scalars[index];
            let class = classes[index];
            let mut merged = false;
            if has_starter && (written == starter + 1 || last_class < class) {
                if let Some(composite) = compose_pair(scalars[starter], scalar) {
                    scalars[starter] = composite;
                    merged = true;
                }
            }
            if !merged {
                scalars[written] = scalar;
                classes[written] = class;
                if class == 0 {
                    starter = written;
                    has_starter = true;
                    last_class = 0;
                } else {
                    last_class = class;
                }
                written += 1;
            }
        }
    }
    written
}

/// The primary composite of `first` and `second`, if any.
fn compose_pair(first: u32, second: u32) -> Option<u32> {
    let lead = first.wrapping_sub(HANGUL_LEAD_BASE);
    let vowel = second.wrapping_sub(HANGUL_VOWEL_BASE);
    if lead < HANGUL_LEAD_COUNT && vowel < HANGUL_VOWEL_COUNT {
        let index = lead * HANGUL_VOWEL_COUNT + vowel;
        return Some(HANGUL_SYLLABLE_BASE + index * HANGUL_TRAIL_COUNT);
    }
    let syllable = first.wrapping_sub(HANGUL_SYLLABLE_BASE);
    let trail = second.wrapping_sub(HANGUL_TRAIL_BASE);
    if syllable < HANGUL_SYLLABLE_COUNT
        && syllable % HANGUL_TRAIL_COUNT == 0
        && trail >= 1
        && trail < HANGUL_TRAIL_COUNT
    {
        return Some(first + trail);
    }
    let mut low = 0;
    let mut high = tables::COMPOSE_COUNT;
    entry!(COMPOSE_SEARCH);
    for _step in 0..COMPOSE_STEPS {
        trip!(COMPOSE_SEARCH);
        if low < high {
            let middle = (low + high) / 2;
            let left = tables::COMPOSE_LEFT[middle];
            if left < first || (left == first && tables::COMPOSE_RIGHT[middle] < second) {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
    }
    if low < tables::COMPOSE_COUNT
        && tables::COMPOSE_LEFT[low] == first
        && tables::COMPOSE_RIGHT[low] == second
    {
        Some(tables::COMPOSE_RESULT[low])
    } else {
        None
    }
}

/// Write the scalars as UTF-8. Returns the byte length.
fn encode<const D: usize, const R: usize>(
    count: usize,
    scalars: &[u32; D],
    output: &mut [u8; R],
) -> Result<usize, NormalizeRefusal> {
    let mut bytes = 0;
    entry!(ENCODE);
    for index in 0..D {
        trip!(ENCODE);
        if index < count {
            bytes = write_scalar(scalars[index], bytes, output)?;
        }
    }
    Ok(bytes)
}

/// Write one scalar at `at`. Returns the next offset.
fn write_scalar<const R: usize>(
    scalar: u32,
    at: usize,
    output: &mut [u8; R],
) -> Result<usize, NormalizeRefusal> {
    let width = if scalar < 0x80 {
        1
    } else if scalar < 0x800 {
        2
    } else if scalar < 0x1_0000 {
        3
    } else {
        4
    };
    if at + width > R {
        return Err(NormalizeRefusal::ExceedsCapacity);
    }
    if width == 1 {
        output[at] = scalar as u8;
    } else if width == 2 {
        output[at] = (0xC0 | (scalar >> 6)) as u8;
        output[at + 1] = (0x80 | (scalar & 0x3F)) as u8;
    } else if width == 3 {
        output[at] = (0xE0 | (scalar >> 12)) as u8;
        output[at + 1] = (0x80 | ((scalar >> 6) & 0x3F)) as u8;
        output[at + 2] = (0x80 | (scalar & 0x3F)) as u8;
    } else {
        output[at] = (0xF0 | (scalar >> 18)) as u8;
        output[at + 1] = (0x80 | ((scalar >> 12) & 0x3F)) as u8;
        output[at + 2] = (0x80 | ((scalar >> 6) & 0x3F)) as u8;
        output[at + 3] = (0x80 | (scalar & 0x3F)) as u8;
    }
    Ok(at + width)
}
