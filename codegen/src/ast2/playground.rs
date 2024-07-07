use async_trait::async_trait;
use crate::algebra2::basis::BasisElement;
use crate::ast2::datatype::MultiVector;
use crate::ast2::impls::Elaborated;
use crate::ast2::traits::{HasNotReturned, NameTrait, TraitAlias, TraitDef_1Class_1Param, TraitDef_2Class_2Param, TraitImpl_11, TraitImpl_22, TraitImplBuilder, TraitKey};
use crate::ast2::Variable;

#[derive(Clone, Copy)]
struct Wedge;
#[derive(Clone, Copy)]
struct AntiDual;
#[derive(Clone, Copy)]
struct Expansion;

pub static WEDGE: Elaborated<Wedge> = Wedge
    .name("Wedge")
    .alias("Join")
    .alias_docs_only("ExteriorProduct")
    .blurb("The famous Wedge product, also known as ExteriorProduct. \
    When a Wedge product exists between two multivectors, the result has a grade that is the \
    sum of the two factors' grades. In point-based interpretations, the Wedge is a Join \
    because increasing grade corresponds to increasing the dimensionality of objects. In \
    hyperplane-based interpretations, the Wedge is a Meet because increasing grade corresponds \
    to the intersection of objects.");

static ANTI_DUAL: Elaborated<AntiDual> = AntiDual.name("AntiDual");
static EXPANSION: Elaborated<Expansion> = Expansion.name("Expansion");



#[async_trait]
impl TraitImpl_22 for Wedge {
    type Output = MultiVector;

    async fn general_implementation<'impls, const AntiScalar: BasisElement>(
        b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
        return None;
    }
}

#[async_trait]
impl TraitImpl_11 for AntiDual {
    type Output = MultiVector;

    async fn general_implementation<'impls, const AntiScalar: BasisElement>(
        b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
        return None;
    }
}



#[async_trait]
impl TraitImpl_22 for Expansion {
    type Output = MultiVector;

    async fn general_implementation<'impls, const AntiScalar: BasisElement>(
        mut b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
        let anti_dual = ANTI_DUAL.invoke(&mut b, other).await?;
        let anti_dual = b.variable("anti_dual", anti_dual);
        let wedge = WEDGE.inline(&mut b, slf, anti_dual).await?;
        b.comment_return("Hello comment", wedge)
    }
}



#[test]
fn thingy() {
    use crate::algebra2::basis::elements::*;
    let s = scalar;
    let a = e12.negate();
    let b = e23;
    let c = e3215;
    println!("Here are some things: {s} {a} {b} {c}");
}

