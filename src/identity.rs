// SPDX-License-Identifier: AGPL-3.0-or-later
//! Structural value-component identities supplied by an admitted caller.
//!
//! QSpec FR-201 fixes each component list and its comparison order. This
//! no_std leaf carries those components without hashing, decoding packages or
//! deriving model identities. The caller validates joins to declarations,
//! model selections and population bindings before constructing values.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt;

use crate::integer::Integer;
use crate::node::NodeKey;

/// The original declaration key retained by model intake.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OriginalDeclarationKey {
    package: String,
    node: String,
}

impl OriginalDeclarationKey {
    /// Carry the admitted domain package identity and IR node identity.
    pub fn new(package: impl Into<String>, node: impl Into<String>) -> Self {
        Self {
            package: package.into(),
            node: node.into(),
        }
    }

    /// The owning domain package identity.
    pub fn package(&self) -> &str {
        &self.package
    }

    /// The IR node identity.
    pub fn node(&self) -> &str {
        &self.node
    }
}

/// A structural effective declaration identity from QSpec FR-150.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectiveId {
    owner_effective_type: Option<Box<EffectiveId>>,
    original: OriginalDeclarationKey,
}

impl EffectiveId {
    /// Carry an admitted effective identity. A type has no owner; a member
    /// carries the identity of its owning effective type.
    pub fn new(
        owner_effective_type: Option<EffectiveId>,
        original: OriginalDeclarationKey,
    ) -> Self {
        Self {
            owner_effective_type: owner_effective_type.map(Box::new),
            original,
        }
    }

    /// The owning effective type, when this is a member.
    pub fn owner_effective_type(&self) -> Option<&EffectiveId> {
        self.owner_effective_type.as_deref()
    }

    /// The original declaration key.
    pub fn original(&self) -> &OriginalDeclarationKey {
        &self.original
    }

    #[cfg(test)]
    pub(crate) fn fixture(byte: u8) -> Self {
        Self::new(
            None,
            OriginalDeclarationKey::new("test", alloc::format!("Node{byte}")),
        )
    }
}

/// The least root type node of an object universe.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UniverseId(NodeKey);

impl UniverseId {
    /// Carry the admitted node id of the least root type declaration.
    pub fn new(root_type: NodeKey) -> Self {
        Self(root_type)
    }

    /// The least root type declaration node.
    pub fn root_type(&self) -> &NodeKey {
        &self.0
    }

    #[cfg(test)]
    pub(crate) fn fixture(byte: u8) -> Self {
        Self::new(NodeKey::fixture(byte))
    }
}

/// Which of QSpec FR-142's two unit identity kinds a [`UnitId`] carries.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnitDomain {
    /// A declared unit's admitted node identity.
    Declared,
    /// A compound unit's canonical list of root-unit terms.
    Compound,
}

/// A quantity's unit identity: a declared unit node or canonical compound terms.
/// The two domains remain distinct even when their displayed parts coincide.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnitId {
    /// One declared unit node.
    Declared(NodeKey),
    /// The evaluator-owned, ordered list of root-unit nodes and nonzero exponents.
    Compound(Box<[(NodeKey, Integer)]>),
}

impl UnitId {
    /// Carry the admitted node identity of one declared unit.
    pub fn declared(key: NodeKey) -> Self {
        Self::Declared(key)
    }

    /// Carry caller-admitted canonical compound-unit terms without
    /// normalizing or validating unit algebra. An empty list is dimensionless.
    pub fn compound(terms: Vec<(NodeKey, Integer)>) -> Self {
        Self::Compound(terms.into_boxed_slice())
    }

    /// The identity domain.
    pub fn domain(&self) -> UnitDomain {
        match self {
            Self::Declared(_) => UnitDomain::Declared,
            Self::Compound(_) => UnitDomain::Compound,
        }
    }

    /// The admitted node for a declared unit, if this is one.
    pub fn declared_key(&self) -> Option<&NodeKey> {
        match self {
            Self::Declared(key) => Some(key),
            Self::Compound(_) => None,
        }
    }

    /// The canonical terms of a compound unit, if this is one.
    pub fn compound_terms(&self) -> Option<&[(NodeKey, Integer)]> {
        match self {
            Self::Declared(_) => None,
            Self::Compound(terms) => Some(terms),
        }
    }
}

/// An empty union member name.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("a union member identity requires a nonempty name")]
pub struct EmptyVariantName;

/// The declaration node and exact case name of an enum or union member.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VariantId {
    owner: NodeKey,
    case: String,
}

impl VariantId {
    /// Carry the admitted declaration and case. The caller checks membership.
    pub fn new(owner: NodeKey, case: impl Into<String>) -> Result<Self, EmptyVariantName> {
        let case = case.into();
        if case.is_empty() {
            return Err(EmptyVariantName);
        }
        Ok(Self { owner, case })
    }

    /// The owning enum or union declaration node.
    pub fn owner(&self) -> &NodeKey {
        &self.owner
    }

    /// The exact UTF-8 case name.
    pub fn case(&self) -> &str {
        &self.case
    }

    #[cfg(test)]
    pub(crate) fn fixture(byte: u8) -> Self {
        Self::new(NodeKey::fixture(0), alloc::format!("Case{byte}")).unwrap()
    }
}

/// The declaring node or effective type that owns a member.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemberOwner {
    /// A checked record, event, or object declaration node.
    Node(NodeKey),
    /// An effective model type declaration.
    Effective(EffectiveId),
}

/// A member name is empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("a member identity requires a nonempty name")]
pub struct EmptyMemberName;

/// The owning declaration and exact field identifier of a member.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemberId {
    owner: MemberOwner,
    member: String,
}

impl MemberId {
    /// Carry an admitted owner and nonempty declared field identifier.
    pub fn new(owner: MemberOwner, member: impl Into<String>) -> Result<Self, EmptyMemberName> {
        let member = member.into();
        if member.is_empty() {
            return Err(EmptyMemberName);
        }
        Ok(Self { owner, member })
    }

    /// The owning declaration identity.
    pub fn owner(&self) -> &MemberOwner {
        &self.owner
    }

    /// The exact field identifier.
    pub fn member(&self) -> &str {
        &self.member
    }

    #[cfg(test)]
    pub(crate) fn fixture(byte: u8) -> Self {
        Self::new(
            MemberOwner::Node(NodeKey::fixture(0)),
            alloc::format!("field{byte}"),
        )
        .unwrap()
    }
}

/// The admitted `relation`/`population` declaration node identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PopulationId(NodeKey);

impl PopulationId {
    /// Carry the admitted population declaration node.
    pub fn new(node: NodeKey) -> Self {
        Self(node)
    }

    /// The population declaration node.
    pub fn node(&self) -> &NodeKey {
        &self.0
    }

    #[cfg(test)]
    pub(crate) fn fixture(byte: u8) -> Self {
        Self::new(NodeKey::fixture(byte))
    }
}

/// An empty [`ObjectId`]: QSpec FR-035 requires a non-empty authored object
/// identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[error("an object identity has at least one authored UTF-8 byte")]
pub struct EmptyObjectIdentity;

/// The authored object half of a `Reference` payload's
/// `(UniverseId, ObjectId)` pair, carried as exact UTF-8 bytes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectId(Box<str>);

impl ObjectId {
    /// The one public constructor: `identity`'s own authored UTF-8 bytes,
    /// exactly as supplied. Refuses only when `identity` is empty (QSpec
    /// QSpec-FR-035); this crate performs no other validation of the authored
    /// identity.
    pub fn new(identity: impl Into<String>) -> Result<Self, EmptyObjectIdentity> {
        let identity = identity.into();
        if identity.is_empty() {
            return Err(EmptyObjectIdentity);
        }
        Ok(Self(identity.into_boxed_str()))
    }

    /// The identity's own exact UTF-8 bytes.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use std::collections::HashSet;

    /// Trace: FR-389-AC-3, TC-938.
    #[trace("FR-389-AC-3", "TC-938")]
    #[test]
    fn effective_and_universe_components_keep_their_owners() {
        let original = OriginalDeclarationKey::new("Pkg", "Type");
        let type_id = EffectiveId::new(None, original.clone());
        let same = EffectiveId::new(None, original.clone());
        let member = EffectiveId::new(
            Some(type_id.clone()),
            OriginalDeclarationKey::new("Pkg", "field"),
        );
        assert_eq!(type_id, same);
        assert_eq!(HashSet::from([type_id.clone(), same]).len(), 1);
        assert_ne!(type_id, member);
        assert_eq!(member.owner_effective_type(), Some(&type_id));
        assert_ne!(
            type_id,
            EffectiveId::new(None, OriginalDeclarationKey::new("Other", "Type"))
        );

        let universe = UniverseId::new(NodeKey::fixture(1));
        let same_universe = UniverseId::new(NodeKey::fixture(1));
        assert_eq!(universe, same_universe);
        assert_eq!(HashSet::from([universe.clone(), same_universe]).len(), 1);
        assert_ne!(universe, UniverseId::new(NodeKey::fixture(2)));
        assert!(universe < UniverseId::new(NodeKey::fixture(2)));
    }

    /// Trace: FR-389-AC-4, FR-389-AC-5, TC-938.
    #[trace("FR-389-AC-4", "FR-389-AC-5", "TC-938")]
    #[test]
    fn member_variant_and_population_identities_keep_exact_components() {
        let owner = NodeKey::fixture(1);
        let other = NodeKey::fixture(2);
        let field = MemberId::new(MemberOwner::Node(owner.clone()), "field").unwrap();
        assert_eq!(
            field,
            MemberId::new(MemberOwner::Node(owner.clone()), "field").unwrap()
        );
        assert_eq!(
            HashSet::from([
                field.clone(),
                MemberId::new(MemberOwner::Node(owner.clone()), "field").unwrap(),
            ])
            .len(),
            1
        );
        assert_ne!(
            field,
            MemberId::new(MemberOwner::Node(other.clone()), "field").unwrap()
        );
        assert_ne!(
            field,
            MemberId::new(MemberOwner::Node(owner.clone()), "other").unwrap()
        );
        assert!(MemberOwner::Node(owner.clone()) < MemberOwner::Effective(EffectiveId::fixture(1)));
        let effective_field = MemberId::new(
            MemberOwner::Effective(EffectiveId::new(
                None,
                OriginalDeclarationKey::new("acme/orders", "Order"),
            )),
            "total",
        )
        .unwrap();
        let other_effective_field = MemberId::new(
            MemberOwner::Effective(EffectiveId::new(
                None,
                OriginalDeclarationKey::new("acme/orders", "Invoice"),
            )),
            "total",
        )
        .unwrap();
        assert_ne!(effective_field, other_effective_field);
        assert!(field < effective_field);
        assert_eq!(
            MemberId::new(MemberOwner::Node(owner.clone()), ""),
            Err(EmptyMemberName)
        );

        let case = VariantId::new(owner.clone(), "Gold Plus").unwrap();
        assert_eq!(case.case(), "Gold Plus");
        assert_eq!(
            HashSet::from([
                case.clone(),
                VariantId::new(owner.clone(), "Gold Plus").unwrap()
            ])
            .len(),
            1
        );
        assert_ne!(case, VariantId::new(other, "Gold Plus").unwrap());
        assert_ne!(case, VariantId::new(owner, "Café").unwrap());
        assert_eq!(
            VariantId::new(NodeKey::fixture(1), ""),
            Err(EmptyVariantName)
        );

        assert_eq!(
            PopulationId::new(NodeKey::fixture(3)),
            PopulationId::new(NodeKey::fixture(3))
        );
        assert_eq!(
            HashSet::from([
                PopulationId::new(NodeKey::fixture(3)),
                PopulationId::new(NodeKey::fixture(3)),
            ])
            .len(),
            1
        );
        assert_ne!(
            PopulationId::new(NodeKey::fixture(3)),
            PopulationId::new(NodeKey::fixture(4))
        );
        assert!(PopulationId::new(NodeKey::fixture(3)) < PopulationId::new(NodeKey::fixture(4)));
    }

    /// Trace: FR-389-AC-6, FR-088-AC-12, TC-938, TC-411.
    #[trace("FR-389-AC-6", "FR-389-AC-8", "FR-088-AC-12", "TC-938", "TC-411")]
    #[test]
    fn unit_domain_and_compound_terms_decide_identity_and_order() {
        let declared = UnitId::declared(NodeKey::fixture(1));
        let compound = UnitId::compound(vec![(NodeKey::fixture(1), Integer::one())]);
        assert_ne!(declared, compound);
        assert_eq!(declared.domain(), UnitDomain::Declared);
        assert_eq!(compound.domain(), UnitDomain::Compound);
        assert!(declared < compound);
        assert_eq!(
            compound,
            UnitId::compound(vec![(NodeKey::fixture(1), Integer::one())])
        );
        assert_ne!(
            compound,
            UnitId::compound(vec![(NodeKey::fixture(1), Integer::from(2))])
        );
        assert_ne!(
            compound,
            UnitId::compound(vec![(NodeKey::fixture(2), Integer::one())])
        );
        assert!(
            UnitId::compound(vec![(NodeKey::fixture(1), Integer::from(2))])
                < UnitId::compound(vec![(NodeKey::fixture(1), Integer::from(10))])
        );
        assert!(UnitId::compound(vec![])
            .compound_terms()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn object_id_keeps_authored_utf8_and_refuses_empty() {
        assert_eq!(ObjectId::new(""), Err(EmptyObjectIdentity));
        let a = ObjectId::new("o-1").unwrap();
        assert_eq!(a.as_str(), "o-1");
        assert_eq!(a.to_string(), "o-1");
        assert_eq!(a, ObjectId::new("o-1").unwrap());
        assert_ne!(a, ObjectId::new("o-2").unwrap());
    }
}
