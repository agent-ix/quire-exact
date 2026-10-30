// SPDX-License-Identifier: AGPL-3.0-or-later
//! Typed failures and checked byte counts for structural identity copies.

use alloc::string::String;
use core::alloc::Layout;

/// A fallible structural identity copy failed at a caller policy or native reservation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityCopyError<PolicyRefusal> {
    /// The caller's original refusal, without conversion to an allocation error.
    Policy(PolicyRefusal),
    /// A representable native reservation was denied by the allocator.
    Allocation {
        /// The minimum additional payload bytes requested at that site.
        requested: usize,
    },
    /// The requested count, byte product or layout cannot be represented.
    Capacity {
        /// The minimum additional payload bytes, or `usize::MAX` on overflow.
        requested: usize,
    },
}

pub(crate) fn payload_bytes<T>(
    count: usize,
) -> Result<usize, IdentityCopyError<core::convert::Infallible>> {
    count
        .checked_mul(core::mem::size_of::<T>())
        .ok_or(IdentityCopyError::Capacity {
            requested: usize::MAX,
        })
}

pub(crate) fn array_bytes<T>(
    count: usize,
) -> Result<usize, IdentityCopyError<core::convert::Infallible>> {
    let requested = payload_bytes::<T>(count)?;
    Layout::array::<T>(count).map_err(|_| IdentityCopyError::Capacity { requested })?;
    Ok(requested)
}

pub(crate) fn text_bytes(
    len: usize,
) -> Result<usize, IdentityCopyError<core::convert::Infallible>> {
    array_bytes::<u8>(len)
}

pub(crate) fn retype<E>(
    error: IdentityCopyError<core::convert::Infallible>,
) -> IdentityCopyError<E> {
    match error {
        IdentityCopyError::Policy(never) => match never {},
        IdentityCopyError::Allocation { requested } => IdentityCopyError::Allocation { requested },
        IdentityCopyError::Capacity { requested } => IdentityCopyError::Capacity { requested },
    }
}

pub(crate) fn copy_text<E>(
    text: &str,
    before_reserve: &mut impl FnMut(usize) -> Result<(), E>,
    native_reserve: &mut impl FnMut(usize) -> bool,
) -> Result<String, IdentityCopyError<E>> {
    let mut copy = String::new();
    if !text.is_empty() {
        let requested = text_bytes(text.len()).map_err(retype)?;
        before_reserve(requested).map_err(IdentityCopyError::Policy)?;
        if !native_reserve(requested) {
            return Err(IdentityCopyError::Allocation { requested });
        }
        copy.try_reserve_exact(text.len())
            .map_err(|_| IdentityCopyError::Allocation { requested })?;
        copy.push_str(text);
    }
    Ok(copy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::OriginalDeclarationKey;
    use ix_trace_rs::trace;

    /// Trace: FR-391-AC-3, TC-940.
    #[trace("FR-391-AC-3", "TC-940")]
    #[test]
    fn production_count_checks_refuse_overflow_and_impossible_layout() {
        let element_size = core::mem::size_of::<OriginalDeclarationKey>();
        let max_count = usize::MAX / element_size;
        assert_eq!(
            payload_bytes::<OriginalDeclarationKey>(max_count),
            Ok(max_count * element_size)
        );
        assert_eq!(
            payload_bytes::<OriginalDeclarationKey>(max_count + 1),
            Err(IdentityCopyError::Capacity {
                requested: usize::MAX
            })
        );
        assert_eq!(
            array_bytes::<OriginalDeclarationKey>(max_count),
            Err(IdentityCopyError::Capacity {
                requested: max_count * element_size
            })
        );
    }
}
