// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Kani-tractable native enum carrier (FR-407).
//!
//! A [`NativeEnum`] holds a member's rank, its declaration's member count and
//! ordered flag, and an opaque declaration tag. It is `Copy`, owns no heap
//! type and has no drop glue. [`compare_native_enum`] replaces the owner's
//! `compare_enum` over two carriers with a loop-free, allocation-free body and
//! one source for host and Kani builds.

use crate::accounting::{Charge, ChargePoint, LimitKind, Meter};
use crate::comparison::{ComparisonOperator, IllTyped, IllTypedCause};
use crate::outcome::{Outcome, Stop};
use crate::value::{EnumMember, EnumShape};

/// The refusals of the [`NativeEnum`] constructors (FR-407 Behavior 3, 4).
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NativeEnumRefusal {
    /// The index is not below the member count.
    #[error("enum member index is not below the member count")]
    IndexOutOfRange,
    /// The member is not the shape's member at its claimed rank.
    #[error("enum member is not in the shape at its claimed rank")]
    MemberNotInShape,
}

/// A fixed-size enum member: rank, member count, ordered flag and an opaque
/// declaration tag. A value exists only if `rank < count`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeEnum {
    rank: u32,
    count: u32,
    ordered: bool,
    declaration: u64,
}

impl NativeEnum {
    /// Build a carrier from a rank and its declaration's member count.
    ///
    /// `declaration` is an opaque tag the caller assigns, one per distinct
    /// (declaration key, ordered flag, member list) triple; the kernel does
    /// not derive it. Refuses with [`NativeEnumRefusal::IndexOutOfRange`]
    /// when `index >= member_count`, a zero count included. Charges nothing.
    pub const fn from_index(
        declaration: u64,
        ordered: bool,
        index: u32,
        member_count: u32,
    ) -> Result<NativeEnum, NativeEnumRefusal> {
        if index >= member_count {
            return Err(NativeEnumRefusal::IndexOutOfRange);
        }
        Ok(Self {
            rank: index,
            count: member_count,
            ordered,
            declaration,
        })
    }

    /// Build a carrier from the kernel's admitted shape and member, taking
    /// the count and the ordered flag from the shape.
    ///
    /// Refuses with [`NativeEnumRefusal::MemberNotInShape`] when the shape's
    /// rank of the member's variant is not the member's rank, and with
    /// [`NativeEnumRefusal::IndexOutOfRange`] when the shape's variant count
    /// does not fit a `u32`.
    pub fn from_member(
        declaration: u64,
        shape: &EnumShape,
        member: &EnumMember,
    ) -> Result<NativeEnum, NativeEnumRefusal> {
        if shape.rank(member.variant()) != Some(member.rank()) {
            return Err(NativeEnumRefusal::MemberNotInShape);
        }
        let count = u32::try_from(shape.variants().count())
            .map_err(|_| NativeEnumRefusal::IndexOutOfRange)?;
        Self::from_index(declaration, shape.is_ordered(), member.rank(), count)
    }

    /// The member's zero-based rank.
    pub const fn index(&self) -> u32 {
        self.rank
    }

    /// The declaration's member count.
    pub const fn member_count(&self) -> u32 {
        self.count
    }

    /// Whether the declaration is an `ordered enum`.
    pub const fn is_ordered(&self) -> bool {
        self.ordered
    }

    /// The caller's opaque declaration tag.
    pub const fn declaration(&self) -> u64 {
        self.declaration
    }

    /// The lossless inverse of [`Self::from_index`]: `(index, member_count,
    /// ordered)`.
    pub const fn parts(&self) -> (u32, u32, bool) {
        (self.rank, self.count, self.ordered)
    }
}

/// Compare two native enum members. Operands of different declarations, and
/// an ordering over an unordered declaration, are ill-typed and consume
/// nothing. Charges are those of the owner's `compare_enum`.
pub fn compare_native_enum(
    operator: ComparisonOperator,
    left: &NativeEnum,
    right: &NativeEnum,
    meter: &mut Meter,
) -> Result<Outcome<bool>, IllTyped> {
    if left.declaration != right.declaration
        || left.count != right.count
        || left.ordered != right.ordered
    {
        return Err(IllTyped {
            cause: IllTypedCause::DistinctEnumDeclarations,
        });
    }
    if operator.is_ordering() && !left.ordered {
        return Err(IllTyped {
            cause: IllTypedCause::UnorderedEnumOrdering,
        });
    }
    Ok(Outcome::from_stop(compare(operator, left, right, meter)))
}

fn compare(
    operator: ComparisonOperator,
    left: &NativeEnum,
    right: &NativeEnum,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    meter
        .charge(Charge::new(ChargePoint::EnumIdentityRead).size(LimitKind::ValueOccurrences, 1))?;
    meter
        .charge(Charge::new(ChargePoint::EnumIdentityRead).size(LimitKind::ValueOccurrences, 2))?;
    let holds = operator.holds(left.rank.cmp(&right.rank));
    meter.charge(Charge::new(ChargePoint::EnumResultRetain).results(1))?;
    Ok(holds)
}
