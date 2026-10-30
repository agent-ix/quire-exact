// SPDX-License-Identifier: AGPL-3.0-or-later
//! Public structural identity constructors and borrowed read paths (TC-938).
//! Selected components follow QSpec TC-233 SI-02..SI-06; no vector copy lives here.

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
    let original = OriginalDeclarationKey::new("acme/orders", "Order");
    let effective = EffectiveId::new(None, original);
    let universe = UniverseId::new(declaration.clone());
    let variant = VariantId::new(declaration.clone(), "Circle").unwrap();
    let field = MemberId::new(MemberOwner::Effective(effective.clone()), "total").unwrap();
    let unit_node = NodeKey::new("acme/shapes", vec![PathSegment::Text("m".into())], 0).unwrap();
    let unit = UnitId::compound(vec![(unit_node.clone(), Integer::from(2_i64))]);
    let population_node =
        NodeKey::new("acme/shapes", vec![PathSegment::Text("counters".into())], 0).unwrap();
    let population = PopulationId::new(population_node.clone());

    assert_eq!(declaration.package(), "acme/shapes");
    assert_eq!(universe.root_type(), &declaration);
    assert_eq!(variant.owner(), &declaration);
    assert_eq!(variant.case(), "Circle");
    assert_eq!(field.member(), "total");
    assert_eq!(&unit.compound_terms().unwrap()[0].0, &unit_node);
    assert_eq!(population.node(), &population_node);
    assert_eq!(effective.clone(), effective);
}
