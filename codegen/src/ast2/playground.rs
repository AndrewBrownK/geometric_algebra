use async_trait::async_trait;

use crate::algebra::MultiVectorClassRegistry;
use crate::ast2::datatype::MultiVector;
use crate::ast2::traits::{HasNotReturned, TraitDef_1Class_1Param, TraitDef_2Class_2Param, TraitImplBuilder, TraitImplRegistry, TraitNames};
use crate::ast2::Variable;

struct Wedge {
    names: TraitNames
}
impl Default for Wedge {
    fn default() -> Self {
        Wedge {
            names: TraitNames::just("Wedge")
        }
    }
}
struct AntiDual;
struct Expansion;


#[async_trait]
impl TraitDef_2Class_2Param for Wedge {
    type Output = MultiVector;

    fn trait_names(&self) -> TraitNames {
        self.names.clone()
    }

    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, Self::Output>> {
        return None;
    }
}

#[async_trait]
impl TraitDef_1Class_1Param for AntiDual {
    type Output = MultiVector;

    fn trait_names(&self) -> TraitNames {
        TraitNames::just("AntiDual")
    }

    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, Self::Output>> {
        return None;
    }
}



#[async_trait]
impl TraitDef_2Class_2Param for Expansion {
    type Output = MultiVector;

    fn trait_names(&self) -> TraitNames {
        TraitNames::just("Expansion")
    }

    async fn general_implementation<'impls>(
        mut b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, Self::Output>> {
        let anti_dual = AntiDual.invoke(&mut b, other).await?;
        let anti_dual = b.variable("anti_dual", anti_dual);
        let wedge = Wedge::default().inline(&mut b, slf, anti_dual).await?;
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

