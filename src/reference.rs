// SPDX-License-Identifier: AGPL-3.0-or-later
//! Terminal `Reference<T>` values.
//!
//! A reference is terminal: its identity is the triple of `EffectiveId` (the
//! object type), `UniverseId` and `ObjectId`, and equality never inspects
//! referenced state. `EffectiveId` and `UniverseId` are opaque digests minted by
//! the caller from its own preimage; `ObjectId` is the object's own authored
//! UTF-8 bytes, never a digest. Checking a reference against a declaration
//! registry is not a kernel concern; a caller does it.

use crate::identity::{EffectiveId, ObjectId, UniverseId};

/// A `Reference<T>` value: its identity triple. Not `Copy`:
/// `ObjectId` carries its own authored bytes rather
/// than a fixed-size digest, so cloning a reference clones those bytes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectReference {
    universe: UniverseId,
    object_type: EffectiveId,
    object: ObjectId,
}

impl ObjectReference {
    /// The reference `(universe, object_type, object)` supplied by a bound
    /// model snapshot.
    pub fn new(universe: UniverseId, object_type: EffectiveId, object: ObjectId) -> Self {
        Self {
            universe,
            object_type,
            object,
        }
    }

    /// The universe identity.
    pub fn universe(&self) -> UniverseId {
        self.universe
    }

    /// The object-type effective identity.
    pub fn object_type(&self) -> EffectiveId {
        self.object_type
    }

    /// The object identity.
    pub fn object(&self) -> &ObjectId {
        &self.object
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn digest(byte: u8) -> [u8; 32] {
        let mut bytes = [0_u8; 32];
        bytes[31] = byte;
        bytes
    }

    /// two references built from the same triple are equal; changing
    /// any one component of the triple makes them distinct (equality never
    /// inspects referenced state -- only the triple).
    #[test]
    fn reference_equality_follows_the_identity_triple() {
        let universe = UniverseId::from_digest(digest(1));
        let object_type = EffectiveId::from_digest(digest(2));
        let object = ObjectId::new("o1").unwrap();
        let a = ObjectReference::new(universe, object_type, object.clone());
        let b = ObjectReference::new(universe, object_type, object);
        let c = ObjectReference::new(universe, object_type, ObjectId::new("o2").unwrap());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
