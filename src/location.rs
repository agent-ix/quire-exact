// SPDX-License-Identifier: AGPL-3.0-or-later
//! Source occurrence identity: `Origin` and `Location`.
//!
//! The kernel provenance shape names a node id and an occurrence key and carries
//! no bytes. The occurrence key is `(NodeKey, role, ordinal)`. `Origin` and
//! `Location` say which source occurrence of which checked node a value or
//! outcome traces back to.
//!
//! `Role` is a lexical-string newtype rather than a closed enum: the set of
//! occurrence roles ("declaration", "reference", "default", ...) is the caller's
//! preimage schema's to define and extend, and the kernel takes no position on
//! which roles exist -- it only carries the role the caller attached.
//!
//! **`Role`'s `String` has no length bound.** This is deliberate: a role's
//! spelling is a small, fixed vocabulary word that the caller's preimage schema
//! names at check time, not a value
//! materialized from caller-supplied, runtime-metered evaluation input the way a
//! `Decimal`'s scale or a `Text`'s bytes are. Nothing on any evaluation path
//! constructs a `Role` from adversarial input, so it takes no charge and needs no
//! bound the way [`crate::Meter`]-charged materializations do.

use alloc::string::String;
use core::fmt;

use crate::node::NodeKey;

/// A source occurrence's role, as the caller's preimage schema names it (e.g.
/// `"declaration"`, `"reference"`). The kernel does not enumerate roles; it
/// carries whichever lexical role string the caller attached to the occurrence.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Role(String);

impl Role {
    /// Wrap an already-known role string.
    pub fn new(role: impl Into<String>) -> Self {
        Self(role.into())
    }

    /// The role's lexical spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        Self::new(role)
    }
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        Self::new(role)
    }
}

/// A source occurrence key within one checked node: a role
/// and an ordinal disambiguating repeated occurrences of that role on the
/// same node (e.g. the third `"reference"` occurrence).
///
/// quire:canonical
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Origin {
    role: Role,
    ordinal: u64,
}

impl Origin {
    /// Name a source occurrence by role and ordinal.
    pub fn new(role: Role, ordinal: u64) -> Self {
        Self { role, ordinal }
    }

    /// The occurrence's role.
    pub fn role(&self) -> &Role {
        &self.role
    }

    /// The occurrence's ordinal, disambiguating repeated occurrences of the
    /// same role on the same node.
    pub fn ordinal(&self) -> u64 {
        self.ordinal
    }
}

/// A full source location: which checked node, and which
/// occurrence within it. Names a node id and an occurrence key; carries no
/// bytes of its own.
///
/// quire:canonical
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Location {
    node: NodeKey,
    occurrence: Origin,
}

impl Location {
    /// Name a location by node and occurrence.
    pub fn new(node: NodeKey, occurrence: Origin) -> Self {
        Self { node, occurrence }
    }

    /// The located node.
    pub fn node(&self) -> NodeKey {
        self.node
    }

    /// The occurrence within that node.
    pub fn occurrence(&self) -> &Origin {
        &self.occurrence
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

    /// two locations naming the same node and the same
    /// `(role, ordinal)` occurrence are equal; a different ordinal makes
    /// them distinct.
    #[test]
    fn location_equality_follows_node_and_occurrence() {
        let node = NodeKey::from_digest(digest(1));
        let a = Location::new(node, Origin::new(Role::from("declaration"), 0));
        let b = Location::new(node, Origin::new(Role::from("declaration"), 0));
        let c = Location::new(node, Origin::new(Role::from("declaration"), 1));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    /// `Origin`'s derived `Ord` orders
    /// first by role, then by ordinal within the same role -- the field
    /// order the derive relies on, exercised rather than merely round-
    /// tripped through the accessors.
    #[test]
    fn origin_orders_by_role_then_ordinal() {
        let declaration_0 = Origin::new(Role::from("declaration"), 0);
        let declaration_1 = Origin::new(Role::from("declaration"), 1);
        let reference_0 = Origin::new(Role::from("reference"), 0);
        assert!(declaration_0 < declaration_1);
        assert!(declaration_1 < reference_0);
        assert_eq!(reference_0.role().as_str(), "reference");
        assert_eq!(reference_0.ordinal(), 0);
    }
}
