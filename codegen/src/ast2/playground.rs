use async_trait::async_trait;
use crate::algebra::MultiVectorClassRegistry;
use crate::ast2::MultiVectorParam;
use crate::ast2::datatype::AnyClasses;
use crate::ast2::traits::{HasNotReturned, HasReturned, TraitDef_1Class_1Param, TraitDef_2Class_2Param, TraitDefinition, TraitImplBuilder, TraitImplRegistry};

struct Wedge;
struct AntiDual;
struct Expansion;


#[async_trait]
impl TraitDef_2Class_2Param for Wedge {
    type Owner = AnyClasses;
    type Other = AnyClasses;

    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, (), HasNotReturned>,
        slf: MultiVectorParam,
        other: MultiVectorParam
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>> {
        return None;
    }
}

#[async_trait]
impl TraitDef_1Class_1Param for AntiDual {
    type Owner = AnyClasses;

    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, (), HasNotReturned>,
        slf: MultiVectorParam
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>> {
        return None;
    }
}

#[async_trait]
impl TraitDef_2Class_2Param for Expansion {
    type Owner = AnyClasses;
    type Other = AnyClasses;

    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, (), HasNotReturned>,
        slf: MultiVectorParam,
        other: MultiVectorParam
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>> {
        // let anti_dual = traits.invoke(AntiDual, (other,)).await?;
        // let anti_dual = builder.assign_var("anti_dual", anti_dual).await?;
        // let wedge = traits.invoke(Wedge, (self_, anti_dual)).await?;
        // b.return_expr(wedge)
        return None;
    }
}

#[test]
fn thingy() {
    // TODO
}


fn get_class_registry() -> MultiVectorClassRegistry {
    todo!()
}
fn get_impl_registry() -> TraitImplRegistry {
    todo!()
}

async fn pretend() {
    let mut class_registry = get_class_registry();
    let mut impl_registry = get_impl_registry();

    // let expansion = TraitDefinition {
    //     name: "Expansion".to_string(),
    //     aliases: vec![],
    //     documentation: "Blah blah blah".to_string(),
    //     inherits: (Wedge, AntiDual),
    //     owner: AnyClasses,
    //     inputs: AnyClasses,
    //     // outputs: (),
    // };


    // let mut builder = general_trait_impl(
    //     &class_registry,
    //     expansion.clone(),
    //     expansion.into(),
    // ).await;
    // let (param_self, builder) = builder.take_param();

}