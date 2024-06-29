use async_trait::async_trait;

use crate::ast2::datatype::MultiVector;
use crate::ast2::impls::Elaborated;
use crate::ast2::traits::{HasNotReturned, NameTrait, TraitAlias, TraitDef_1Class_1Param, TraitDef_2Class_2Param, TraitImpl_11, TraitImpl_22, TraitImplBuilder};
use crate::ast2::Variable;

#[derive(Clone, Copy)]
struct Wedge;
#[derive(Clone, Copy)]
struct AntiDual;
#[derive(Clone, Copy)]
struct Expansion;

// static WEDGE: CustomizedTraitDef<Wedge> = CustomizedTraitDef::new(Wedge, "Wedge");
// static ANTI_DUAL: CustomizedTraitDef<AntiDual> = CustomizedTraitDef::new(AntiDual, "AntiDual");
// static EXPANSION: CustomizedTraitDef<Expansion> = CustomizedTraitDef::new(Expansion, "Expansion");
static WEDGE: Elaborated<Wedge> = Wedge
    .named("Wedge")
    .with_alias("Join")
    .with_alias("ExteriorProduct")
    .with_blurb("The famous Wedge product, also known as ExteriorProduct. \
    When a Wedge product exists between two multivectors, the result has a grade that is the \
    sum of the two factors' grades. In point-based interpretations, the Wedge is a Join \
    because increasing grade corresponds to increasing the dimensionality of objects. In \
    hyperplane-based interpretations, the Wedge is a Meet because increasing grade corresponds \
    to the intersection of objects.");

static ANTI_DUAL: Elaborated<AntiDual> = AntiDual.named("AntiDual");
static EXPANSION: Elaborated<Expansion> = Expansion.named("Expansion");



#[async_trait]
impl TraitImpl_22 for Wedge {
    type Output = MultiVector;

    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, Self::Output>> {
        return None;
    }
}

#[async_trait]
impl TraitImpl_11 for AntiDual {
    type Output = MultiVector;

    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, Self::Output>> {
        return None;
    }
}



#[async_trait]
impl TraitImpl_22 for Expansion {
    type Output = MultiVector;

    async fn general_implementation<'impls>(
        mut b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, Self::Output>> {
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


    // let mut sq = GeneratorSquares::empty();
    // for i in 0..17 {
    //     let b = sq.next_available_basis().unwrap();
    //     sq = sq.append([(b, 0)]).unwrap();
    //     sq.append([(PrimaryBasis::e3, 0)]).unwrap();
    // }
}

