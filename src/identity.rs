// SPDX-License-Identifier: AGPL-3.0-or-later
//! Structural value-component identities supplied by an admitted caller.
//!
//! QSpec FR-201 fixes each component list and its comparison order. This
//! no_std leaf carries those components without hashing, decoding packages or
//! deriving model identities. The caller validates joins to declarations,
//! model selections and population bindings before constructing values.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::{cmp::Ordering, fmt};

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
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct EffectiveId {
    // Root effective type first, then each member in owner order. This is
    // the recursive FR-150 value in flat form, so Clone and Drop use no
    // call stack proportional to owner depth.
    chain: Vec<OriginalDeclarationKey>,
}

impl fmt::Debug for EffectiveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EffectiveId({:?})", self.chain)
    }
}

impl EffectiveId {
    /// Carry an admitted effective identity. A type has no owner; a member
    /// carries the identity of its owning effective type.
    pub fn new(
        owner_effective_type: Option<EffectiveId>,
        original: OriginalDeclarationKey,
    ) -> Self {
        let mut chain = owner_effective_type.map_or_else(Vec::new, |owner| owner.chain);
        chain.push(original);
        Self { chain }
    }

    /// The owning effective type, when this is a member. Reconstructing the
    /// owner clones its flat prefix; use [`Self::owner_chain`] to borrow it.
    pub fn owner_effective_type(&self) -> Option<EffectiveId> {
        (self.chain.len() > 1).then(|| EffectiveId {
            chain: self.chain[..self.chain.len() - 1].to_vec(),
        })
    }

    /// The original declaration key.
    pub fn original(&self) -> &OriginalDeclarationKey {
        self.chain
            .last()
            .expect("EffectiveId::new always records an original")
    }

    /// The owning declarations from the root type through this declaration.
    pub fn owner_chain(&self) -> &[OriginalDeclarationKey] {
        &self.chain
    }

    #[cfg(test)]
    pub(crate) fn fixture(byte: u8) -> Self {
        Self::new(
            None,
            OriginalDeclarationKey::new("test", alloc::format!("Node{byte}")),
        )
    }
}

impl Ord for EffectiveId {
    fn cmp(&self, other: &Self) -> Ordering {
        // Recursive FR-150 compares null versus owner before any original.
        // A shorter chain reaches null first; equal depths compare the
        // original keys from the root owner through the leaf.
        self.chain
            .len()
            .cmp(&other.chain.len())
            .then_with(|| self.chain.cmp(&other.chain))
    }
}

impl PartialOrd for EffectiveId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The least root type node of an object universe.
///
/// The owned node is boxed so a refusal carrying two universe ids remains a
/// small error value; the public identity and its component order are unchanged.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UniverseId(Box<NodeKey>);

impl UniverseId {
    /// Carry the admitted node id of the least root type declaration.
    pub fn new(root_type: NodeKey) -> Self {
        Self(Box::new(root_type))
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
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnitId {
    /// One declared unit node.
    Declared(NodeKey),
    /// The evaluator-owned, ordered list of root-unit nodes and nonzero exponents.
    Compound(Box<[(NodeKey, Integer)]>),
}

impl fmt::Debug for UnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Keep a unit on one line inside an alternate Debug of a quantity
        // type, as FR-387's bounded type layout requires.
        match self {
            Self::Declared(node) => write!(f, "UnitId(Declared({node:?}))"),
            Self::Compound(terms) => write!(f, "UnitId(Compound({terms:?}))"),
        }
    }
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
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VariantId {
    owner: NodeKey,
    case: String,
}

impl fmt::Debug for VariantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VariantId({:?}, {:?})", self.owner, self.case)
    }
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
    use alloc::string::ToString;
    use ix_trace_rs::trace;
    use std::collections::{hash_map::DefaultHasher, HashSet};
    use std::hash::{Hash, Hasher};

    #[trace("TC-938", "FR-389-AC-3", "FR-389-AC-7")]
    #[test]
    fn deep_effective_owner_chain_clones_compares_hashes_and_drops_on_small_stack() {
        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let mut id = EffectiveId::new(None, OriginalDeclarationKey::new("Pkg", "root"));
                for _ in 0..10_000 {
                    id = EffectiveId::new(Some(id), OriginalDeclarationKey::new("Pkg", "member"));
                }
                let copy = id.clone();
                assert_eq!(id, copy);
                assert_eq!(id.cmp(&copy), Ordering::Equal);
                let mut changed = copy.clone();
                *changed
                    .chain
                    .last_mut()
                    .expect("effective id has an original") =
                    OriginalDeclarationKey::new("Pkg", "member2");
                assert!(id < changed);
                let hash = |value: &EffectiveId| {
                    let mut hasher = DefaultHasher::new();
                    value.hash(&mut hasher);
                    hasher.finish()
                };
                assert_eq!(hash(&id), hash(&copy));
                assert!(alloc::format!("{id:?}").starts_with("EffectiveId"));
                assert_eq!(id.owner_chain().len(), 10_001);
                let owner = id.owner_effective_type().expect("deep member has an owner");
                assert_eq!(owner.owner_chain().len(), 10_000);
                drop(owner);
                drop(copy);
                drop(changed);
                drop(id);
            })
            .expect("spawn small stack")
            .join()
            .expect("deep effective owner operations complete");
    }

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
        assert_eq!(member.owner_effective_type(), Some(type_id.clone()));
        let root_z = EffectiveId::new(None, OriginalDeclarationKey::new("Pkg", "Z"));
        let member_of_a = EffectiveId::new(
            Some(EffectiveId::new(
                None,
                OriginalDeclarationKey::new("Pkg", "A"),
            )),
            OriginalDeclarationKey::new("Pkg", "member"),
        );
        assert!(
            root_z < member_of_a,
            "null owner sorts before any present owner"
        );
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
            UnitId::compound(vec![(NodeKey::fixture(1), Integer::from(2_i64))])
        );
        assert_ne!(
            compound,
            UnitId::compound(vec![(NodeKey::fixture(2), Integer::one())])
        );
        assert!(
            UnitId::compound(vec![(NodeKey::fixture(1), Integer::from(2_i64))])
                < UnitId::compound(vec![(NodeKey::fixture(1), Integer::from(10_i64))])
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
