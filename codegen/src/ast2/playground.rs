use std::sync::Arc;
use async_trait::async_trait;
use crate::algebra::{MultiVectorClass, MultiVectorClassRegistry};
use crate::ast2::{Variable};
use crate::ast2::datatype::{AnyClasses, MultiVector};
use crate::ast2::expressions::{MultiVectorExpr, TraitResult};
use crate::ast2::traits::{HasNotReturned, HasReturned, TraitDef_1Class_1Param, TraitDef_2Class_2Param, TraitDefinition, TraitImplBuilder, TraitImplRegistry};


struct TraitAlias {
    alias_name: String,
    mention_in_documentation: bool,
    share_main_trait_in_rust_and_delegate_by_default: bool,
    output_distinct_trait_in_rust: bool,
    output_in_shaders: bool
}
impl TraitAlias {
    fn usual(alias_name: String) -> Self {
        Self::new(alias_name, true, true, false, true)
    }
    fn usual_except_shaders(alias_name: String) -> Self {
        Self::new(alias_name, true, true, false, false)
    }
    fn docs_only(alias_name: String) -> Self {
        Self::new(alias_name, true, false, false, false)
    }
    fn new(alias_name: String, docs: bool, share: bool, distinct: bool, shaders: bool) -> Self {
        TraitAlias {
            alias_name,
            mention_in_documentation: docs,
            share_main_trait_in_rust_and_delegate_by_default: share,
            output_distinct_trait_in_rust: distinct,
            output_in_shaders: shaders
        }
    }
}
struct TraitNamesAndStuff {
    trait_name: String,
    aliases: Vec<TraitAlias>,
}



struct Wedge;
struct AntiDual;
struct Expansion;


#[async_trait]
impl TraitDef_2Class_2Param for Wedge {
    type Output = MultiVector;

    fn result_type(result: &Self::Output) -> TraitResult {
        TraitResult::AnyClass(result)
    }

    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, HasReturned>> {
        return None;
    }
}

#[async_trait]
impl TraitDef_1Class_1Param for AntiDual {
    type Output = MultiVector;

    fn result_type(result: &Self::Output) -> TraitResult {
        TraitResult::AnyClass(result)
    }

    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, HasReturned>> {
        return None;
    }
}

#[async_trait]
impl TraitDef_2Class_2Param for Expansion {
    type Output = MultiVector;

    fn result_type(result: &Self::Output) -> TraitResult {
        TraitResult::AnyClass(result)
    }

    async fn general_impl<'impls>(
        mut b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>
    ) -> Option<TraitImplBuilder<'impls, HasReturned>> {
        let anti_dual = AntiDual::invoke(&mut b, other).await?;
        let anti_dual = b.variable("anti_dual", anti_dual);
        let wedge = Wedge::invoke(&mut b, slf, anti_dual).await?;
        b.comment_return("Hello comment", wedge)
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