// SPDX-License-Identifier: AGPL-3.0-or-later
//! Value-component identities.
//!
//! Seven identities appear as bare payload inside kernel `Value`s:
//! `EffectiveId` (declaration identity), `UniverseId` and `ObjectId` (a
//! `Reference` payload), `UnitId` (a `Quantity` payload), `VariantId` (an `Enum`
//! payload), `MemberId` and `PopulationId` (a `Population` payload). Five of the
//! seven -- `EffectiveId`, `UniverseId`, `VariantId`, `MemberId`,
//! `PopulationId` -- are single-domain opaque 32-byte digests with exactly one
//! public constructor, `from_digest` (each type's own, e.g.
//! [`EffectiveId::from_digest`]), mirroring [`crate::NodeKey::from_digest`]: the
//! kernel wraps an already-computed digest and never hashes. The caller computes
//! each digest over its own preimage schema and mints the id; this crate holds
//! no preimage knowledge for any of them. They share one shape (`Eq`/`Ord`/`Hash`,
//! hex `Display`/`Debug`), so the `digest_identity!` macro below generates all
//! five from one macro body ("one fact, one place"). `UnitId` and `ObjectId` are
//! the other two, and each differs: [`UnitId`] is a domain-labelled digest record
//! over the two unit identities QSpec FR-142 defines, with one constructor per
//! domain, so it is written out by hand below. [`ObjectId`] is different in kind,
//! not degree: QSpec FR-204 states a declared object identity is "authored, not
//! digested", so `ObjectId` wraps the object's own exact UTF-8 bytes directly,
//! with one public constructor, `new`, that takes the string itself and refuses
//! only when it is empty (QSpec FR-035) -- never a digest, never a hash -- and is
//! likewise written out by hand, immediately after `PopulationId` below.
//!
//! **Domain strings.** `EffectiveId`'s domain is
//! `quire.model.effective-declaration/v1` and `UniverseId`'s is
//! `quire.model.object-universe/v1`. `UnitId`'s two domains are QSpec FR-142's:
//! [`crate::NODE_KEY_DOMAIN`] for a declared unit's node key and
//! [`COMPOUND_UNIT_DOMAIN`] for a compound unit. `ObjectId` carries no digest
//! domain at all: it is the authored UTF-8 object identity (QSpec FR-204,
//! "authored, not digested"; non-empty per QSpec FR-035), never hashed. The
//! `VariantId`, `MemberId` and `PopulationId` domain strings are placeholders,
//! not settled product semantics, and are not to be treated as ratified until the
//! model digest domains are specified. Each of the three types is used elsewhere
//! in this crate (`Value::Enum` and `ValueType`'s enum sum shape for
//! `VariantId`, [`crate::value::FieldDeclaration`] for `MemberId`,
//! `Value::Population` and `ValueType::Population` for `PopulationId`).

use alloc::{boxed::Box, string::String};
use core::cmp::Ordering;
use core::fmt;

use crate::node::{NodeKey, NODE_KEY_DOMAIN};

macro_rules! digest_identity {
    (
        $(#[$doc:meta])*
        $name:ident, $domain_const:ident, $domain:literal
    ) => {
        /// Digest domain for
        #[doc = concat!("[`", stringify!($name), "`].")]
        pub const $domain_const: &str = $domain;

        $(#[$doc])*
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; 32]);

        impl $name {
            /// The one public constructor: wrap an already-computed
            #[doc = concat!("`", $domain, "` digest.")]
            /// The caller computes the digest over its own canonical
            /// preimage; this type performs no hashing and holds no preimage
            /// knowledge.
            pub fn from_digest(digest: [u8; 32]) -> Self {
                Self(digest)
            }

            /// The raw digest bytes.
            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({self})", stringify!($name))
            }
        }
    };
}

digest_identity!(
    /// An opaque `quire.model.effective-declaration/v1` identity: the effective, fully-resolved
    /// identity of a declaration after archetype composition. Only the
    /// caller's model stage calls [`EffectiveId::from_digest`] in production.
    EffectiveId,
    EFFECTIVE_ID_DOMAIN,
    "quire.model.effective-declaration/v1"
);

digest_identity!(
    /// An opaque `quire.model.object-universe/v1` identity: the universe
    /// half of a `Reference` payload's `(UniverseId, ObjectId)` pair. One per connected component of the
    /// object-type supertype graph. Only the caller's model stage computes this
    /// digest, over the `ObjectUniverse` preimage
    /// (`quire.model.object-universe/v1`).
    UniverseId,
    UNIVERSE_ID_DOMAIN,
    "quire.model.object-universe/v1"
);

/// Digest domain and preimage version of a compound unit's [`UnitId`]: QSpec
/// QSpec-FR-142's evaluator-owned `quire.value.compound-unit/v1` value identity.
pub const COMPOUND_UNIT_DOMAIN: &str = "quire.value.compound-unit/v1";

/// Which of QSpec FR-142's two unit identities a [`UnitId`] carries. No other domain is admitted.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnitDomain {
    /// A declared unit: its node key, in [`NODE_KEY_DOMAIN`]
    /// (`quire.checked-semantic-node/v1` over the `quire.unit-node/v1`
    /// preimage).
    Declared,
    /// A compound unit: its [`COMPOUND_UNIT_DOMAIN`] digest.
    Compound,
}

impl UnitDomain {
    /// The digest-domain label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Declared => NODE_KEY_DOMAIN,
            Self::Compound => COMPOUND_UNIT_DOMAIN,
        }
    }
}

/// A `Quantity` payload's unit: a domain-labelled
/// digest record over the two unit identities QSpec FR-142
/// defines. A declared unit carries its node key as opaque bytes under the
/// [`NODE_KEY_DOMAIN`] label, and a compound unit carries its
/// [`COMPOUND_UNIT_DOMAIN`] digest. The kernel names no declaration type and
/// computes neither digest: the caller mints every `UnitId`.
///
/// Equality and order are lexical on the domain label, then on the bytes
/// (R-04), so a declared unit and a compound unit are never equal whatever
/// their bytes.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct UnitId {
    domain: UnitDomain,
    digest: [u8; 32],
}

impl UnitId {
    /// The declared-unit arm: `key` is the unit's node key. The caller
    /// admits only a key minted over a `quire.unit-node/v1` preimage; this type cannot tell one node key from another.
    pub fn declared(key: NodeKey) -> Self {
        Self {
            domain: UnitDomain::Declared,
            digest: *key.as_bytes(),
        }
    }

    /// The compound-unit arm: wrap an already-computed
    /// `quire.value.compound-unit/v1` digest.
    pub fn compound(digest: [u8; 32]) -> Self {
        Self {
            domain: UnitDomain::Compound,
            digest,
        }
    }

    /// The domain this identity is labelled with.
    pub fn domain(&self) -> UnitDomain {
        self.domain
    }

    /// The raw digest bytes, meaningful only under [`Self::domain`].
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.digest
    }
}

impl Ord for UnitId {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.domain.label(), &self.digest).cmp(&(other.domain.label(), &other.digest))
    }
}

impl PartialOrd for UnitId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The digest bytes as 64 lowercase hex digits; the domain is not rendered
/// (see [`UnitId::domain`]).
impl fmt::Display for UnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.digest
            .iter()
            .try_for_each(|byte| write!(f, "{byte:02x}"))
    }
}

impl fmt::Debug for UnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnitId({}, {self})", self.domain.label())
    }
}

digest_identity!(
    /// An opaque `quire.enum-variant/v1` identity: a bare `Enum` payload
    /// "with no `NodeKey`".
    VariantId,
    VARIANT_ID_DOMAIN,
    "quire.enum-variant/v1"
);

digest_identity!(
    /// An opaque `quire.member/v1` identity: a collection or
    /// composite member's identity.
    MemberId,
    MEMBER_ID_DOMAIN,
    "quire.member/v1"
);

digest_identity!(
    /// An opaque `quire.population/v1` identity: a `Population` payload's admission identity.
    /// Only the caller's model stage calls [`PopulationId::from_digest`] in production,
    /// minting it from the admitted binding's own preimage (domain package,
    /// `population_key`, and the closed three-state admission-role
    /// discriminator `Direct`/`Pre`/`Post`) at admission time -- this crate
    /// holds none of that preimage knowledge, only the resulting digest.
    /// `PopulationBinding` itself -- admission, membership, and the
    /// `allInstances`/`lookup` closure state -- stays a caller's model type,
    /// never a kernel one.
    PopulationId,
    POPULATION_ID_DOMAIN,
    "quire.population/v1"
);

/// An empty [`ObjectId`]: QSpec FR-035 requires a non-empty authored object
/// identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[error("an object identity has at least one authored UTF-8 byte")]
pub struct EmptyObjectIdentity;

/// The object half of a `Reference` payload's `(UniverseId, ObjectId)` pair
/// The object's own authored identity,
/// carried as its exact UTF-8 bytes. Unlike every type the `digest_identity!`
/// macro above generates, `ObjectId` is never a digest and has no digest
/// domain (QSpec FR-204: "authored, not digested") -- `model` never hashes
/// it, so there is no digest for this type to wrap.
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
    use alloc::string::ToString;
    use ix_trace_rs::trace;

    use super::*;

    fn digest(byte: u8) -> [u8; 32] {
        let mut bytes = [0_u8; 32];
        bytes[31] = byte;
        bytes
    }

    /// Each of the five single-domain digest
    /// identities (`EffectiveId`, `UniverseId`, `VariantId`, `MemberId`,
    /// `PopulationId`) treats equal digest bytes as interchangeable ids --
    /// not just `==`, but hashing equal (either stands in for the other as a
    /// set/map key) -- and distinct digests as distinct, differently
    /// ordered ids. `UnitId`
    /// and `ObjectId` are each hand-written, not
    /// macro-generated, and have their own equivalent coverage below
    /// (`tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes`,
    /// `equal_object_identity_bytes_mint_interchangeable_identities`).
    #[test]
    fn equal_digest_bytes_mint_interchangeable_identities() {
        macro_rules! check {
            ($ty:ident) => {
                let a = $ty::from_digest(digest(1));
                let b = $ty::from_digest(digest(1));
                let c = $ty::from_digest(digest(2));
                assert_eq!(a, b);
                assert_eq!(a.as_bytes(), b.as_bytes());
                assert_eq!(std::collections::HashSet::from([a, b, c]).len(), 2);
                assert!(a < c);
            };
        }
        check!(EffectiveId);
        check!(UniverseId);
        check!(VariantId);
        check!(MemberId);
        check!(PopulationId);
    }

    /// The same check for `ObjectId`: equal authored UTF-8
    /// bytes mint interchangeable ids -- `==`, equal hashing, and distinct
    /// strings compare unequal and order lexically over their bytes (`ObjectId` carries its bytes directly, never a
    /// digest, so there is no digest to equate here).
    #[test]
    fn equal_object_identity_bytes_mint_interchangeable_identities() {
        let a = ObjectId::new("o1").unwrap();
        let b = ObjectId::new("o1").unwrap();
        let c = ObjectId::new("o2").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.as_str(), b.as_str());
        assert_eq!(
            std::collections::HashSet::from([a.clone(), b, c.clone()]).len(),
            2
        );
        assert!(a < c);
    }

    /// An empty authored object identity refuses (QSpec FR-035): this is the
    /// one validation `ObjectId::new` performs, so it is worth its own
    /// mutation-provable assertion (removing the `is_empty` check would let
    /// this construct `Ok`).
    #[test]
    fn empty_object_identity_refuses() {
        assert_eq!(ObjectId::new("").unwrap_err(), EmptyObjectIdentity);
        assert!(ObjectId::new("o1").is_ok());
    }

    /// Each identity's domain constant is
    /// distinct, so no two of the identities (`EffectiveId`, `UniverseId`,
    /// `VariantId`, `MemberId`, `PopulationId`, and the compound-unit arm
    /// of `UnitId`) can be confused by domain string.
    /// Also pins `EFFECTIVE_ID_DOMAIN` and `UNIVERSE_ID_DOMAIN` to the literal
    /// strings the module doc names. `ObjectId` has no digest domain to include.
    #[test]
    fn domain_constants_are_pairwise_distinct() {
        assert_eq!(EFFECTIVE_ID_DOMAIN, "quire.model.effective-declaration/v1");
        assert_eq!(UNIVERSE_ID_DOMAIN, "quire.model.object-universe/v1");
        let domains = [
            EFFECTIVE_ID_DOMAIN,
            UNIVERSE_ID_DOMAIN,
            COMPOUND_UNIT_DOMAIN,
            VARIANT_ID_DOMAIN,
            MEMBER_ID_DOMAIN,
            POPULATION_ID_DOMAIN,
        ];
        for (i, a) in domains.iter().enumerate() {
            for (j, b) in domains.iter().enumerate() {
                assert_eq!(i == j, a == b, "domains at {i} and {j} collided");
            }
        }
    }

    /// TC-411 (kernel half): a `UnitId` is labelled with one of its two
    /// domains; equality and order are lexical on the label, then the bytes,
    /// so a declared and a compound unit over identical bytes are unequal,
    /// and every declared unit orders before every compound unit
    /// (`quire.checked-semantic-node/v1` < `quire.value.compound-unit/v1`).
    #[trace("FR-088-AC-12", "TC-411")]
    #[test]
    fn tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes() {
        let declared = UnitId::declared(NodeKey::from_digest(digest(2)));
        let compound = UnitId::compound(digest(2));
        assert_eq!(declared.as_bytes(), compound.as_bytes());
        assert_ne!(declared, compound);
        assert_eq!(
            std::collections::HashSet::from([declared, compound]).len(),
            2
        );
        assert_eq!(declared.domain(), UnitDomain::Declared);
        assert_eq!(declared.domain().label(), NODE_KEY_DOMAIN);
        assert_eq!(compound.domain(), UnitDomain::Compound);
        assert_eq!(compound.domain().label(), COMPOUND_UNIT_DOMAIN);
        assert!(declared < compound);
        assert!(UnitId::declared(NodeKey::from_digest(digest(9))) < UnitId::compound(digest(1)));
        assert!(UnitId::compound(digest(1)) < UnitId::compound(digest(2)));
        assert_eq!(declared, UnitId::declared(NodeKey::from_digest(digest(2))));
    }

    /// `Display` renders exactly 64 lowercase hex digits.
    #[test]
    fn display_is_64_lowercase_hex_digits() {
        let id = UniverseId::from_digest(digest(0xcd));
        let rendered = id.to_string();
        assert_eq!(rendered.len(), 64);
        assert!(rendered
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
        assert!(rendered.ends_with("cd"));
    }

    /// The same check for `ObjectId`: `Display` renders the authored
    /// bytes exactly, never hex -- `ObjectId` is "not a digest" and that holds
    /// all the way through formatting.
    #[test]
    fn object_id_display_is_the_authored_bytes_exactly() {
        let id = ObjectId::new("zz9-plural-z-alpha").unwrap();
        assert_eq!(id.to_string(), "zz9-plural-z-alpha");
        assert_eq!(id.as_str(), "zz9-plural-z-alpha");
    }
}
