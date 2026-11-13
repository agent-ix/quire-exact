// SPDX-License-Identifier: AGPL-3.0-or-later
//! Host-only decision for the four normalizing profiles: whether `retained`
//! is exactly the owner's normalization of `payload`. The proof path does
//! not use it; those profiles are unsupported there (FR-404 Behavior 10).

use crate::text::NormalizationForm;

/// Scalar-by-scalar comparison of the owner's normalization of `payload`
/// with `retained`. A payload that is not UTF-8 has no normal form.
pub(super) fn is_retained_of(form: NormalizationForm, payload: &[u8], retained: &[u8]) -> bool {
    let (Ok(payload), Ok(retained)) = (
        core::str::from_utf8(payload),
        core::str::from_utf8(retained),
    ) else {
        return false;
    };
    form.apply(payload).eq(retained.chars())
}
