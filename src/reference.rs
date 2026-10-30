// SPDX-License-Identifier: AGPL-3.0-or-later
//! Terminal `Reference<T>` values.
//!
//! A reference is terminal: its identity is the triple of `EffectiveId` (the
//! object type), `UniverseId` and `ObjectId`, and equality never inspects
//! referenced state. `EffectiveId` and `UniverseId` carry structural
//! identities from the caller; `ObjectId` carries its authored UTF-8 bytes.
//! Checking a reference against a declaration
//! registry is not a kernel concern; a caller does it.

use crate::identity::{EffectiveId, ObjectId, UniverseId};

/// A `Reference<T>` value: its identity triple. Cloning preserves its
/// structural universe and type ids and its authored object identity.
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
    pub fn universe(&self) -> &UniverseId {
        &self.universe
    }

    /// The object-type effective identity.
    pub fn object_type(&self) -> &EffectiveId {
        &self.object_type
    }

    /// The object identity.
    pub fn object(&self) -> &ObjectId {
        &self.object
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// two references built from the same triple are equal; changing
    /// any one component of the triple makes them distinct (equality never
    /// inspects referenced state -- only the triple).
    #[test]
    fn reference_equality_follows_the_identity_triple() {
        let universe = UniverseId::fixture(1);
        let object_type = EffectiveId::fixture(2);
        let object = ObjectId::new("o1").unwrap();
        let a = ObjectReference::new(universe.clone(), object_type.clone(), object.clone());
        let b = ObjectReference::new(universe.clone(), object_type.clone(), object);
        let c = ObjectReference::new(universe, object_type, ObjectId::new("o2").unwrap());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
