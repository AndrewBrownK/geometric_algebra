use async_trait::async_trait;

use crate::algebra::basis::BasisElement;
use crate::algebra::basis::filter::{allow_all_signatures, signatures_containing, signatures_not_containing};
use crate::algebra::multivector::DynamicMultiVector;
use crate::ast::datatype::{Float, MultiVector};
use crate::ast::expressions::FloatExpr;
use crate::ast::impls::Elaborated;
use crate::ast::traits::{HasNotReturned, NameTrait, TraitDef_1_Type_1_Arg, TraitDef_2_Types_2_Args, TraitImpl_11, TraitImplBuilder};
use crate::ast::Variable;
use crate::build_scripts::common_traits::{Addition, AntiDotProduct, AntiSquareRoot, RightAntiDual, DotProduct, SquareRoot, SubType, Wedge};
use crate::trait_impl_1_type_1_arg;

#[allow(non_snake_case)]
pub const fn RoundBulk(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundBulkImpl> {
    RoundBulkImpl { origin, infinity }
        .new_trait_named("RoundBulk")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct RoundBulkImpl {
    origin: BasisElement,
    infinity: BasisElement,
}
#[async_trait]
impl TraitImpl_11 for RoundBulkImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let all = allow_all_signatures();
        let not_origin = signatures_not_containing(self.origin);
        let not_infinity = signatures_not_containing(self.infinity);
        let result = SubType(all, not_origin & not_infinity, all)
            .inline(&builder, slf).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn RoundWeight(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundWeightImpl> {
    RoundWeightImpl { origin, infinity }
        .new_trait_named("RoundWeight")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct RoundWeightImpl {
    origin: BasisElement,
    infinity: BasisElement,
}
#[async_trait]
impl TraitImpl_11 for RoundWeightImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let all = allow_all_signatures();
        let not_origin = signatures_containing(self.origin);
        let not_infinity = signatures_not_containing(self.infinity);
        let result = SubType(all, not_origin & not_infinity, all)
            .inline(&builder, slf).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn FlatBulk(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatBulkImpl> {
    FlatBulkImpl { origin, infinity }
        .new_trait_named("FlatBulk")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatBulkImpl {
    origin: BasisElement,
    infinity: BasisElement,
}
#[async_trait]
impl TraitImpl_11 for FlatBulkImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let all = allow_all_signatures();
        let not_origin = signatures_not_containing(self.origin);
        let not_infinity = signatures_containing(self.infinity);
        let result = SubType(all, not_origin & not_infinity, all)
            .inline(&builder, slf).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn FlatWeight(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatWeightImpl> {
    FlatWeightImpl { origin, infinity }
        .new_trait_named("FlatWeight")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatWeightImpl {
    origin: BasisElement,
    infinity: BasisElement,
}
#[async_trait]
impl TraitImpl_11 for FlatWeightImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let all = allow_all_signatures();
        let not_origin = signatures_containing(self.origin);
        let not_infinity = signatures_containing(self.infinity);
        let result = SubType(all, not_origin & not_infinity, all)
            .inline(&builder, slf).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn CenterNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<CenterNormImpl> {
    CenterNormImpl { origin, infinity }
        .new_trait_named("CenterNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct CenterNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for CenterNormImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let cns = CenterNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = SquareRoot.invoke(&mut builder, cns).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn CenterNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<CenterNormSquaredImpl> {
    CenterNormSquaredImpl { origin, infinity }
        .new_trait_named("CenterNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct CenterNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for CenterNormSquaredImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let rbns = RoundBulkNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let fwns = FlatWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let ad = RightAntiDual.invoke(&mut builder, fwns).await?;
        let result = Addition.inline(&builder, rbns, ad).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn FlatBulkNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatBulkNormImpl> {
    FlatBulkNormImpl { origin, infinity }
        .new_trait_named("FlatBulkNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatBulkNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for FlatBulkNormImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let fbns = FlatBulkNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = SquareRoot.invoke(&mut builder, fbns).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn FlatBulkNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatBulkNormSquaredImpl> {
    FlatBulkNormSquaredImpl { origin, infinity }
        .new_trait_named("FlatBulkNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatBulkNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for FlatBulkNormSquaredImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let mut dyn_origin = DynamicMultiVector::zero();
        dyn_origin += (FloatExpr::Literal(1.0), self.origin);
        let origin = dyn_origin.construct(&builder)?;

        let flat_bulk = FlatBulk(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let wedge = Wedge.invoke(&mut builder, flat_bulk, origin).await?;
        let fbt = builder.variable("flat_bulk_thing", wedge);
        let dot = DotProduct.invoke(&mut builder, fbt.clone(), fbt).await?;
        builder.return_expr(dot)
    }
}

#[allow(non_snake_case)]
pub const fn FlatNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatNormImpl> {
    FlatNormImpl { origin, infinity }
        .new_trait_named("FlatNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for FlatNormImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let fbn = FlatBulkNorm(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let fwn = FlatWeightNorm(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = Addition.inline(&builder, fbn, fwn).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn FlatNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatNormSquaredImpl> {
    FlatNormSquaredImpl { origin, infinity }
        .new_trait_named("FlatNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for FlatNormSquaredImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let fbn = FlatBulkNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let fwn = FlatWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = Addition.inline(&builder, fbn, fwn).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn FlatWeightNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatWeightNormImpl> {
    FlatWeightNormImpl { origin, infinity }
        .new_trait_named("FlatWeightNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatWeightNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for FlatWeightNormImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let fwns = FlatWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = AntiSquareRoot.invoke(&mut builder, fwns).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn FlatWeightNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<FlatWeightNormSquaredImpl> {
    FlatWeightNormSquaredImpl { origin, infinity }
        .new_trait_named("FlatWeightNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct FlatWeightNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for FlatWeightNormSquaredImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let fw = FlatWeight(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let fw = builder.variable("flat_weight", fw);
        let anti_dot = AntiDotProduct.invoke(&mut builder, fw.clone(), fw).await?;
        builder.return_expr(anti_dot)
    }
}
pub static RadiusNorm: Elaborated<RadiusNormImpl> = RadiusNormImpl
    .new_trait_named("RadiusNorm")
    .blurb("TODO");
trait_impl_1_type_1_arg!(RadiusNormImpl(builder, slf) -> MultiVector {
    let rns = RadiusNormSquared.invoke(&mut builder, slf.clone()).await?;
    let result = SquareRoot.invoke(&mut builder, rns).await?;
    builder.return_expr(result)
});
pub static RadiusNormSquared: Elaborated<RadiusNormSquaredImpl> = RadiusNormSquaredImpl
    .new_trait_named("RadiusNormSquared")
    .blurb("TODO");
trait_impl_1_type_1_arg!(RadiusNormSquaredImpl(builder, slf) -> MultiVector {
    let anti_dot = AntiDotProduct.invoke(&mut builder, slf.clone(), slf.clone()).await?;
    let result = RightAntiDual.invoke(&mut builder, anti_dot).await?;
    builder.return_expr(result)
});

#[allow(non_snake_case)]
pub const fn RoundBulkNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundBulkNormImpl> {
    RoundBulkNormImpl { origin, infinity }
        .new_trait_named("RoundBulkNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct RoundBulkNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for RoundBulkNormImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let rbns = RoundBulkNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = SquareRoot.invoke(&mut builder, rbns).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn RoundBulkNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundBulkNormSquaredImpl> {
    RoundBulkNormSquaredImpl { origin, infinity }
        .new_trait_named("RoundBulkNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct RoundBulkNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for RoundBulkNormSquaredImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let rb = RoundBulk(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let rb = builder.variable("round_bulk", rb);
        let dot = DotProduct.invoke(&mut builder, rb.clone(), rb).await?;
        builder.return_expr(dot)
    }
}

#[allow(non_snake_case)]
pub const fn RoundNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundNormImpl> {
    RoundNormImpl { origin, infinity }
        .new_trait_named("RoundNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct RoundNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for RoundNormImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let rbn = RoundBulkNorm(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let rwn = RoundWeightNorm(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = Addition.inline(&builder, rbn, rwn).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn RoundNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundNormSquaredImpl> {
    RoundNormSquaredImpl { origin, infinity }
        .new_trait_named("RoundNormSquared")
        .blurb("TODO")
}#[derive(Clone, Copy)]
pub struct RoundNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for RoundNormSquaredImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let rbn = RoundBulkNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let rwn = RoundWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = Addition.inline(&builder, rbn, rwn).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn RoundWeightNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundWeightNormImpl> {
    RoundWeightNormImpl { origin, infinity }
        .new_trait_named("RoundWeightNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct RoundWeightNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for RoundWeightNormImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let rwns = RoundWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let result = AntiSquareRoot.invoke(&mut builder, rwns).await?;
        builder.return_expr(result)
    }
}

#[allow(non_snake_case)]
pub const fn RoundWeightNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundWeightNormSquaredImpl> {
    RoundWeightNormSquaredImpl { origin, infinity }
        .new_trait_named("RoundWeightNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct RoundWeightNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for RoundWeightNormSquaredImpl {
    type Output = MultiVector;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let mut dyn_infinity = DynamicMultiVector::zero();
        dyn_infinity += (FloatExpr::Literal(1.0), self.infinity);
        let infinity = dyn_infinity.construct(&builder)?;

        let rw = RoundWeight(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let wedge = Wedge.invoke(&mut builder, rw, infinity).await?;
        let v = builder.variable("round_weight_carrier", wedge);
        let anti_dot = AntiDotProduct.invoke(&mut builder, v.clone(), v).await?;
        builder.return_expr(anti_dot)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedCenterNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedCenterNormImpl> {
    UnitizedCenterNormImpl { origin, infinity }
        .new_trait_named("UnitizedCenterNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedCenterNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedCenterNormImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let ucns = UnitizedCenterNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let sqrt = FloatExpr::Product(vec![(ucns, 0.5)], 1.0);
        builder.return_expr(sqrt)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedCenterNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedCenterNormSquaredImpl> {
    UnitizedCenterNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedCenterNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedCenterNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedCenterNormSquaredImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let numerator = CenterNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let denominator = RoundWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let numerator = FloatExpr::AccessMultiVecFlat(numerator, 0);
        let denominator = FloatExpr::AccessMultiVecFlat(denominator, 0);
        let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
        builder.return_expr(divide)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedFlatNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedFlatNormImpl> {
    UnitizedFlatNormImpl { origin, infinity }
        .new_trait_named("UnitizedFlatNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedFlatNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedFlatNormImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let ufns = UnitizedFlatNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let sqrt = FloatExpr::Product(vec![(ufns, 0.5)], 1.0);
        builder.return_expr(sqrt)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedFlatNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedFlatNormSquaredImpl> {
    UnitizedFlatNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedFlatNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedFlatNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedFlatNormSquaredImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let numerator = FlatBulkNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let denominator = FlatWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let numerator = FloatExpr::AccessMultiVecFlat(numerator, 0);
        let denominator = FloatExpr::AccessMultiVecFlat(denominator, 0);
        let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
        builder.return_expr(divide)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedRadiusNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRadiusNormImpl> {
    UnitizedRadiusNormImpl { origin, infinity }
        .new_trait_named("UnitizedRadiusNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedRadiusNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedRadiusNormImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let urns = UnitizedRadiusNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let sqrt = FloatExpr::Product(vec![(urns, 0.5)], 1.0);
        builder.return_expr(sqrt)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedRadiusNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRadiusNormSquaredImpl> {
    UnitizedRadiusNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedRadiusNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedRadiusNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedRadiusNormSquaredImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let numerator = RadiusNormSquared.invoke(&mut builder, slf.clone()).await?;
        let denominator = RoundWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let numerator = FloatExpr::AccessMultiVecFlat(numerator, 0);
        let denominator = FloatExpr::AccessMultiVecFlat(denominator, 0);
        let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
        builder.return_expr(divide)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedRoundNorm(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRoundNormImpl> {
    UnitizedRoundNormImpl { origin, infinity }
        .new_trait_named("UnitizedRoundNorm")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedRoundNormImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedRoundNormImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let urns = UnitizedRoundNormSquared(self.origin, self.infinity).invoke(&mut builder, slf).await?;
        let sqrt = FloatExpr::Product(vec![(urns, 0.5)], 1.0);
        builder.return_expr(sqrt)
    }
}

#[allow(non_snake_case)]
pub const fn UnitizedRoundNormSquared(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRoundNormSquaredImpl> {
    UnitizedRoundNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedRoundNormSquared")
        .blurb("TODO")
}
#[derive(Clone, Copy)]
pub struct UnitizedRoundNormSquaredImpl { origin: BasisElement, infinity: BasisElement }
#[async_trait]
impl TraitImpl_11 for UnitizedRoundNormSquaredImpl {
    type Output = Float;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let numerator = RoundBulkNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let denominator = RoundWeightNormSquared(self.origin, self.infinity).invoke(&mut builder, slf.clone()).await?;
        let numerator = FloatExpr::AccessMultiVecFlat(numerator, 0);
        let denominator = FloatExpr::AccessMultiVecFlat(denominator, 0);
        let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
        builder.return_expr(divide)
    }
}
