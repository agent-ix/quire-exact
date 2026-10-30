// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public structural identity constructors and borrowed read paths (TC-938).

use ix_trace_rs::trace;
use quire_exact::{
    EffectiveId, Integer, MemberId, MemberOwner, NodeKey, OriginalDeclarationKey, PathSegment,
    PopulationId, UnitId, UniverseId, VariantId,
};

#[trace("TC-938", "FR-389-AC-7")]
#[test]
fn public_identities_keep_admitted_components_without_digest_bytes() {
    let declaration =
        NodeKey::new("acme/shapes", vec![PathSegment::Text("Shape".into())], 0).unwrap();
    let original = OriginalDeclarationKey::new("acme/shapes", "ix://acme/shapes/Shape");
    let effective = EffectiveId::new(None, original);
    let universe = UniverseId::new(declaration.clone());
    let variant = VariantId::new(declaration.clone(), "Gold Plus").unwrap();
    let field = MemberId::new(MemberOwner::Effective(effective.clone()), "value").unwrap();
    let unit = UnitId::compound(vec![(declaration.clone(), Integer::from(2))]);
    let population = PopulationId::new(declaration.clone());

    assert_eq!(declaration.package(), "acme/shapes");
    assert_eq!(universe.root_type(), &declaration);
    assert_eq!(variant.owner(), &declaration);
    assert_eq!(variant.case(), "Gold Plus");
    assert_eq!(field.member(), "value");
    assert_eq!(&unit.compound_terms().unwrap()[0].0, &declaration);
    assert_eq!(population.node(), &declaration);
    assert_eq!(effective.clone(), effective);
}
