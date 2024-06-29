use async_trait::async_trait;
use crate::ast2::datatype::MultiVector;
use crate::ast2::traits::{HasNotReturned, TraitImpl_10, TraitImpl_11, TraitImplBuilder};
use crate::ast2::Variable;

#[derive(Clone, Copy)]
pub struct Zero;

#[async_trait]
impl TraitImpl_10 for Zero {
    type Output = MultiVector;

    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        owner: MultiVector
    ) -> Option<TraitImplBuilder<'impls, Self::Output>> {
        todo!()
    }
}