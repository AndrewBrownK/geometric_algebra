#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use async_trait::async_trait;

use crate::algebra::basis::BasisElement;
use crate::algebra::multivector::MultiVec;
use crate::ast::datatype::{AnyClasses, Float, Integer, MultiVector, Specifically};
use crate::ast::expressions::{Expression, TraitResultType};
use crate::ast::traits::{HasNotReturned, ProvideTraitNames, TraitAlias, TraitDef_1_Type_0_Args, TraitDef_1_Type_1_Arg, TraitDef_2_Types_1_Arg, TraitDef_2_Types_2_Args, TraitImplBuilder, TraitImpl_10, TraitImpl_11, TraitImpl_21, TraitImpl_22, TraitKey, TraitNames, TraitImpl_12f, TraitDef_1_Type_2_Args_f32, TraitImpl_12i, TraitDef_1_Type_2_Args_i32};
use crate::ast::Variable;

#[derive(Clone, Copy)]
pub struct Elaborated<Impl> {
    pub trait_names: TraitNames,
    blurb: &'static str,
    the_impl: Impl,
}
pub fn standard_documentation(n: TraitNames, blurb: &'static str) -> String {
    let name = n.trait_key.as_upper_camel();
    let mut aliases = String::new();
    let mut has_output_aliases_yet = false;
    for alias in n.aliases {
        let Some(alias) = alias else { continue };
        if !alias.mention_in_documentation {
            continue;
        }
        if !has_output_aliases_yet {
            aliases.push_str("\nAliases: ")
        } else {
            aliases.push_str(", ")
        }
        has_output_aliases_yet = true;
        aliases.push_str(alias.alias_key.as_upper_camel().as_str());
    }
    let mut has_blurb = String::new();
    if !blurb.is_empty() {
        has_blurb.push_str("\n")
    }
    format!("{name}{has_blurb}{blurb}{aliases}")
}

impl<Impl: Copy> Elaborated<Impl> {
    pub const fn new<const D: usize>(the_impl: Impl, name: &'static str, blurb: &'static str, aliases: [TraitAlias; D]) -> Self
    where
        [(); D]: Sized,
    {
        let provided_aliases = aliases;
        let mut aliases = [None; 16];
        let mut idx = 0;
        while idx < 16 && idx < D {
            aliases[idx] = Some(provided_aliases[idx]);
            idx += 1;
        }
        Elaborated {
            trait_names: TraitNames {
                trait_key: TraitKey::new(name),
                aliases,
            },
            blurb,
            the_impl,
        }
    }

    pub const fn new_with_name(i: Impl, n: &'static str) -> Self {
        Self::new(i, n, "", [])
    }

    pub const fn rename(mut self, n: &'static str) -> Self {
        self.trait_names.trait_key = TraitKey::new(n);
        self
    }

    pub const fn blurb(mut self, explanation: &'static str) -> Self {
        self.blurb = explanation;
        self
    }

    pub const fn alias(self, alias: &'static str) -> Self {
        self.alias_custom(TraitAlias::usual(alias))
    }

    pub const fn alias_docs_only(self, alias: &'static str) -> Self {
        self.alias_custom(TraitAlias::docs_only(alias))
    }

    pub const fn alias_except_shaders(self, alias: &'static str) -> Self {
        self.alias_custom(TraitAlias::usual_except_shaders(alias))
    }

    pub const fn alias_custom(mut self, alias: TraitAlias) -> Self {
        let arr = &mut self.trait_names.aliases;
        let mut idx = 0;
        while idx < arr.len() {
            if arr[idx].is_none() {
                arr[idx] = Some(alias);
                break;
            }
            idx += 1;
        }
        self
    }
}

impl<Impl> const ProvideTraitNames for Elaborated<Impl> {
    fn trait_names(&self) -> TraitNames {
        self.trait_names
    }
}
#[async_trait]
impl<Impl: TraitImpl_10> TraitImpl_10 for Elaborated<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, owner).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_10> TraitDef_1_Type_0_Args for Elaborated<Impl> {
    type Owner = AnyClasses;
    fn general_documentation(&self) -> String {
        standard_documentation(<Elaborated<Impl> as ProvideTraitNames>::trait_names(self), self.blurb)
    }
    fn domain(&self) -> Self::Owner {
        AnyClasses
    }
}
#[async_trait]
impl<Impl: TraitImpl_11> TraitImpl_11 for Elaborated<Impl> {
    type Output = Impl::Output;

    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, slf).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_11> TraitDef_1_Type_1_Arg for Elaborated<Impl> {
    type Owner = AnyClasses;
    fn general_documentation(&self) -> String {
        standard_documentation(<Elaborated<Impl> as ProvideTraitNames>::trait_names(self), self.blurb)
    }
    fn domain(&self) -> Self::Owner {
        AnyClasses
    }
}
#[async_trait]
impl<Impl: TraitImpl_21> TraitImpl_21 for Elaborated<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, slf, other).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_21> TraitDef_2_Types_1_Arg for Elaborated<Impl> {
    type Owner = AnyClasses;
    type Other = AnyClasses;
    fn general_documentation(&self) -> String {
        standard_documentation(<Elaborated<Impl> as ProvideTraitNames>::trait_names(self), self.blurb)
    }
    fn domain(&self) -> (Self::Owner, Self::Other) {
        (AnyClasses, AnyClasses)
    }
}
#[async_trait]
impl<Impl: TraitImpl_22> TraitImpl_22 for Elaborated<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, slf, other).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_22> TraitDef_2_Types_2_Args for Elaborated<Impl> {
    type Owner = AnyClasses;
    type Other = AnyClasses;
    fn general_documentation(&self) -> String {
        standard_documentation(<Elaborated<Impl> as ProvideTraitNames>::trait_names(self), self.blurb)
    }
    fn domain(&self) -> (Self::Owner, Self::Other) {
        (AnyClasses, AnyClasses)
    }
}
#[async_trait]
impl<Impl: TraitImpl_12f> TraitImpl_12f for Elaborated<Impl> {
    type Output = Impl::Output;

    async fn general_implementation<const AntiScalar: BasisElement>(
        self, builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<Float>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(builder, slf, other).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_12f> TraitDef_1_Type_2_Args_f32 for Elaborated<Impl> {
    type Owner = AnyClasses;
    fn general_documentation(&self) -> String {
        standard_documentation(<Elaborated<Impl> as ProvideTraitNames>::trait_names(self), self.blurb)
    }
    fn domain(&self) -> Self::Owner {
        AnyClasses
    }
}
#[async_trait]
impl<Impl: TraitImpl_12i> TraitImpl_12i for Elaborated<Impl> {
    type Output = Impl::Output;

    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<Integer>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(builder, slf, other).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_12i> TraitDef_1_Type_2_Args_i32 for Elaborated<Impl> {
    type Owner = AnyClasses;
    fn general_documentation(&self) -> String {
        standard_documentation(<Elaborated<Impl> as ProvideTraitNames>::trait_names(self), self.blurb)
    }
    fn domain(&self) -> Self::Owner {
        AnyClasses
    }
}

#[derive(Clone, Copy)]
pub struct InlineOnly<Impl> {
    name: &'static str,
    the_impl: Impl,
}
impl<Impl: Copy> InlineOnly<Impl> {
    /// Even though the name is never used for a trait declaration, the name
    /// will probably still be used in a local variable declaration.
    pub const fn new(name: &'static str, the_impl: Impl) -> Self {
        InlineOnly { name, the_impl }
    }
}

impl<Impl> const ProvideTraitNames for InlineOnly<Impl> {
    fn trait_names(&self) -> TraitNames {
        TraitNames::just(self.name)
    }
}
#[async_trait]
impl<Impl: TraitImpl_10> TraitImpl_10 for InlineOnly<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, owner).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_10> TraitDef_1_Type_0_Args for InlineOnly<Impl> {
    type Owner = AnyClasses;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner {
        AnyClasses
    }

    async fn invoke<const AntiScalar: BasisElement>(
        &self,
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let return_as_var = self.inline(b, owner).await?;
        return Some(<Self::Output as TraitResultType>::inlined_expr_10(return_as_var));
    }
}
#[async_trait]
impl<Impl: TraitImpl_11> TraitImpl_11 for InlineOnly<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, slf).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_11> TraitDef_1_Type_1_Arg for InlineOnly<Impl> {
    type Owner = AnyClasses;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner {
        AnyClasses
    }

    async fn invoke<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let return_as_var = self.inline(b, owner).await?;
        return Some(<Self::Output as TraitResultType>::inlined_expr_11(return_as_var));
    }
}
#[async_trait]
impl<Impl: TraitImpl_21> TraitImpl_21 for InlineOnly<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, slf, other).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_21> TraitDef_2_Types_1_Arg for InlineOnly<Impl> {
    type Owner = AnyClasses;
    type Other = AnyClasses;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> (Self::Owner, Self::Other) {
        (AnyClasses, AnyClasses)
    }

    async fn invoke<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let return_as_var = self.inline(b, owner, other.clone()).await?;
        return Some(<Self::Output as TraitResultType>::inlined_expr_21(return_as_var));
    }
}
#[async_trait]
impl<Impl: TraitImpl_22> TraitImpl_22 for InlineOnly<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        self.the_impl.general_implementation(b, slf, other).await
    }
}
#[async_trait]
impl<Impl: TraitImpl_22> TraitDef_2_Types_2_Args for InlineOnly<Impl> {
    type Owner = AnyClasses;
    type Other = AnyClasses;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> (Self::Owner, Self::Other) {
        (AnyClasses, AnyClasses)
    }

    async fn invoke<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let return_as_var = self.inline(b, owner, other).await?;
        return Some(<Self::Output as TraitResultType>::inlined_expr_22(return_as_var));
    }
}

// TODO InlineOnly 12f and 12i



pub type SpecializedImpl_10<const AntiScalar: BasisElement, Output> = &'static (dyn Fn(TraitImplBuilder<AntiScalar, HasNotReturned>, MultiVector) -> Pin<Box<dyn Future<Output = Option<TraitImplBuilder<AntiScalar, Output>>> + Send>>
              + Send
              + Sync);
#[derive(Clone, Copy)]
pub struct Specialized_10<const AntiScalar: BasisElement, Output: TraitResultType>(
    TraitNames,
    &'static MultiVec<AntiScalar>,
    PhantomData<Output>,
    SpecializedImpl_10<AntiScalar, Output>,
);
impl<const AntiScalar: BasisElement, Output: TraitResultType> const ProvideTraitNames for Specialized_10<AntiScalar, Output> {
    fn trait_names(&self) -> TraitNames {
        self.0
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Output: TraitResultType> TraitImpl_10 for Specialized_10<AntiScalar, Output>
where
    Self: Copy,
{
    type Output = Output;
    async fn general_implementation<const AntiScalar2: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar2, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar2, Self::Output>> {
        if AntiScalar != AntiScalar2 {
            panic!(
                "When specializing trait implementations, ensure you are using an AntiScalar \
            that matches your GeometricAlgebra and MultiVecs. Expected {AntiScalar2}, found \
            {AntiScalar}"
            )
        }
        b.mark_as_specialized_implementation();
        // SAFETY: This is actually safe because of the above AntiScalar != AntiScalar2 check
        let f = unsafe {
            let p0 = &self.3;
            let p1: *const SpecializedImpl_10<AntiScalar, Output> = p0 as *const SpecializedImpl_10<AntiScalar, Output>;
            let p2: *const SpecializedImpl_10<AntiScalar2, Output> = p1.cast::<SpecializedImpl_10<AntiScalar2, Output>>();
            *(p2) as SpecializedImpl_10<AntiScalar2, Output>
        };
        (*f)(b, owner).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Output: TraitResultType> TraitDef_1_Type_0_Args for Specialized_10<AntiScalar, Output>
where
    Self: Copy,
{
    type Owner = Specifically;
    fn domain(&self) -> Self::Owner {
        Specifically(MultiVector::from(self.1))
    }
}
#[const_trait]
pub trait Specialize_10: ProvideTraitNames + TraitImpl_10 {
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_10<AntiScalar, Output>,
    ) -> Specialized_10<AntiScalar, Output>;
}
impl<TD> const Specialize_10 for TD
where
    TD: TraitImpl_10 + ProvideTraitNames + ~const ProvideTraitNames,
{
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_10<AntiScalar, Output>,
    ) -> Specialized_10<AntiScalar, Output> {
        let trait_names: TraitNames = ProvideTraitNames::trait_names(self);
        Specialized_10(trait_names, owner, PhantomData, the_impl)
    }
}
pub type SpecializedImpl_11<const AntiScalar: BasisElement, Output> = &'static (dyn Fn(TraitImplBuilder<AntiScalar, HasNotReturned>, Variable<MultiVector>) -> Pin<Box<dyn Future<Output = Option<TraitImplBuilder<AntiScalar, Output>>> + Send>>
              + Send
              + Sync);
#[derive(Clone, Copy)]
pub struct Specialized_11<const AntiScalar: BasisElement, Output: TraitResultType>(
    TraitNames,
    &'static MultiVec<AntiScalar>,
    PhantomData<Output>,
    SpecializedImpl_11<AntiScalar, Output>,
);
impl<const AntiScalar: BasisElement, Output: TraitResultType> const ProvideTraitNames for Specialized_11<AntiScalar, Output> {
    fn trait_names(&self) -> TraitNames {
        self.0
    }
}
#[async_trait]
impl<Output: TraitResultType, const AntiScalar: BasisElement> TraitImpl_11 for Specialized_11<AntiScalar, Output>
where
    Self: Copy,
{
    type Output = Output;
    async fn general_implementation<const AntiScalar2: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar2, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar2, Self::Output>> {
        if AntiScalar != AntiScalar2 {
            panic!(
                "When specializing trait implementations, ensure you are using an AntiScalar \
            that matches your GeometricAlgebra and MultiVecs. Expected {AntiScalar2}, found \
            {AntiScalar}"
            )
        }
        b.mark_as_specialized_implementation();
        // SAFETY: This is actually safe because of the above AntiScalar != AntiScalar2 check
        let f = unsafe {
            let p0 = &self.3;
            let p1: *const SpecializedImpl_11<AntiScalar, Output> = p0 as *const SpecializedImpl_11<AntiScalar, Output>;
            let p2: *const SpecializedImpl_11<AntiScalar2, Output> = p1.cast::<SpecializedImpl_11<AntiScalar2, Output>>();
            *(p2) as SpecializedImpl_11<AntiScalar2, Output>
        };
        (*f)(b, slf).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Output: TraitResultType> TraitDef_1_Type_1_Arg for Specialized_11<AntiScalar, Output>
where
    Self: Copy,
{
    type Owner = Specifically;
    fn domain(&self) -> Self::Owner {
        Specifically(MultiVector::from(self.1))
    }
}
#[const_trait]
pub trait Specialize_11: ProvideTraitNames + TraitImpl_11 {
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_11<AntiScalar, Output>,
    ) -> Specialized_11<AntiScalar, Output>;
}
impl<TD> const Specialize_11 for TD
where
    TD: TraitImpl_11 + ProvideTraitNames + ~const ProvideTraitNames,
{
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_11<AntiScalar, Output>,
    ) -> Specialized_11<AntiScalar, Output> {
        let trait_names: TraitNames = ProvideTraitNames::trait_names(self);
        Specialized_11(trait_names, owner, PhantomData, the_impl)
    }
}
pub type SpecializedImpl_21<const AntiScalar: BasisElement, Output> = &'static (dyn Fn(
    TraitImplBuilder<AntiScalar, HasNotReturned>,
    Variable<MultiVector>,
    MultiVector,
) -> Pin<Box<dyn Future<Output = Option<TraitImplBuilder<AntiScalar, Output>>> + Send>>
              + Send
              + Sync);
#[derive(Clone, Copy)]
pub struct Specialized_21<const AntiScalar: BasisElement, Output: TraitResultType>(
    TraitNames,
    &'static MultiVec<AntiScalar>,
    PhantomData<Output>,
    SpecializedImpl_21<AntiScalar, Output>,
);
impl<const AntiScalar: BasisElement, Output: TraitResultType> const ProvideTraitNames for Specialized_21<AntiScalar, Output> {
    fn trait_names(&self) -> TraitNames {
        self.0
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Output: TraitResultType> TraitImpl_21 for Specialized_21<AntiScalar, Output>
where
    Self: Copy,
{
    type Output = Output;
    async fn general_implementation<const AntiScalar2: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar2, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar2, Self::Output>> {
        if AntiScalar != AntiScalar2 {
            panic!(
                "When specializing trait implementations, ensure you are using an AntiScalar \
            that matches your GeometricAlgebra and MultiVecs. Expected {AntiScalar2}, found \
            {AntiScalar}"
            )
        }
        b.mark_as_specialized_implementation();
        // SAFETY: This is actually safe because of the above AntiScalar != AntiScalar2 check
        let f = unsafe {
            let p0 = &self.3;
            let p1: *const SpecializedImpl_21<AntiScalar, Output> = p0 as *const SpecializedImpl_21<AntiScalar, Output>;
            let p2: *const SpecializedImpl_21<AntiScalar2, Output> = p1.cast::<SpecializedImpl_21<AntiScalar2, Output>>();
            *(p2) as SpecializedImpl_21<AntiScalar2, Output>
        };
        (*f)(b, slf, other).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Output: TraitResultType> TraitDef_2_Types_1_Arg for Specialized_21<AntiScalar, Output>
where
    Self: Copy,
{
    type Owner = Specifically;
    type Other = Specifically;
    fn domain(&self) -> (Self::Owner, Self::Other) {
        (Specifically(MultiVector::from(self.1)), Specifically(MultiVector::from(self.1)))
    }
}
#[const_trait]
pub trait Specialize_21: ProvideTraitNames + TraitImpl_21 {
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_21<AntiScalar, Output>,
    ) -> Specialized_21<AntiScalar, Output>;
}
impl<TD> const Specialize_21 for TD
where
    TD: TraitImpl_21 + ProvideTraitNames + ~const ProvideTraitNames,
{
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_21<AntiScalar, Output>,
    ) -> Specialized_21<AntiScalar, Output> {
        let trait_names: TraitNames = ProvideTraitNames::trait_names(self);
        Specialized_21(trait_names, owner, PhantomData, the_impl)
    }
}
pub type SpecializedImpl_22<const AntiScalar: BasisElement, Output> = &'static (dyn Fn(
    TraitImplBuilder<AntiScalar, HasNotReturned>,
    Variable<MultiVector>,
    Variable<MultiVector>,
) -> Pin<Box<dyn Future<Output = Option<TraitImplBuilder<AntiScalar, Output>>> + Send>>
              + Send
              + Sync);
#[derive(Clone, Copy)]
pub struct Specialized_22<const AntiScalar: BasisElement, Output: TraitResultType>(
    TraitNames,
    &'static MultiVec<AntiScalar>,
    PhantomData<Output>,
    SpecializedImpl_22<AntiScalar, Output>,
);
impl<const AntiScalar: BasisElement, Output: TraitResultType> const ProvideTraitNames for Specialized_22<AntiScalar, Output> {
    fn trait_names(&self) -> TraitNames {
        self.0
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Output: TraitResultType> TraitImpl_22 for Specialized_22<AntiScalar, Output>
where
    Self: Copy,
{
    type Output = Output;
    async fn general_implementation<const AntiScalar2: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar2, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar2, Self::Output>> {
        if AntiScalar != AntiScalar2 {
            panic!(
                "When specializing trait implementations, ensure you are using an AntiScalar \
            that matches your GeometricAlgebra and MultiVecs. Expected {AntiScalar2}, found \
            {AntiScalar}"
            )
        }
        b.mark_as_specialized_implementation();
        // SAFETY: This is actually safe because of the above AntiScalar != AntiScalar2 check
        let f = unsafe {
            let p0 = &self.3;
            let p1: *const SpecializedImpl_22<AntiScalar, Output> = p0 as *const SpecializedImpl_22<AntiScalar, Output>;
            let p2: *const SpecializedImpl_22<AntiScalar2, Output> = p1.cast::<SpecializedImpl_22<AntiScalar2, Output>>();
            *(p2) as SpecializedImpl_22<AntiScalar2, Output>
        };
        (*f)(b, slf, other).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Output: TraitResultType> TraitDef_2_Types_2_Args for Specialized_22<AntiScalar, Output>
where
    Self: Copy,
{
    type Owner = Specifically;
    type Other = Specifically;
    fn domain(&self) -> (Self::Owner, Self::Other) {
        (Specifically(MultiVector::from(self.1)), Specifically(MultiVector::from(self.1)))
    }
}
#[const_trait]
pub trait Specialize_22: ProvideTraitNames + TraitImpl_22 {
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_22<AntiScalar, Output>,
    ) -> Specialized_22<AntiScalar, Output>;
}
impl<TD> const Specialize_22 for TD
where
    TD: TraitImpl_22 + ProvideTraitNames + ~const ProvideTraitNames,
{
    fn specialize<const AntiScalar: BasisElement, Output: TraitResultType>(
        &self,
        owner: &'static MultiVec<AntiScalar>,
        the_impl: SpecializedImpl_22<AntiScalar, Output>,
    ) -> Specialized_22<AntiScalar, Output> {
        let trait_names: TraitNames = ProvideTraitNames::trait_names(self);
        Specialized_22(trait_names, owner, PhantomData, the_impl)
    }
}


// TODO Specialize 12f and 12i



#[derive(Clone, Copy)]
pub struct OvertDelegate<Impl> {
    name: &'static str,
    the_impl: Impl,
}
impl<Impl: Copy> OvertDelegate<Impl> {
    pub const fn new(name: &'static str, the_impl: Impl) -> Self {
        OvertDelegate { name, the_impl }
    }
}

impl<Impl> const ProvideTraitNames for OvertDelegate<Impl> {
    fn trait_names(&self) -> TraitNames {
        TraitNames::just(self.name)
    }
}
#[async_trait]
impl<Impl: TraitDef_1_Type_0_Args> TraitImpl_10 for OvertDelegate<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let result = self.the_impl.invoke(&mut b, owner).await?;
        b.return_expr(result)
    }
}
#[async_trait]
impl<Impl: TraitDef_1_Type_0_Args> TraitDef_1_Type_0_Args for OvertDelegate<Impl> {
    type Owner = Impl::Owner;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner {
        self.the_impl.domain()
    }
}
#[async_trait]
impl<Impl: TraitDef_1_Type_1_Arg> TraitImpl_11 for OvertDelegate<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let result = self.the_impl.invoke(&mut b, slf).await?;
        b.return_expr(result)
    }
}
#[async_trait]
impl<Impl: TraitDef_1_Type_1_Arg> TraitDef_1_Type_1_Arg for OvertDelegate<Impl> {
    type Owner = Impl::Owner;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner {
        self.the_impl.domain()
    }
}
#[async_trait]
impl<Impl: TraitDef_2_Types_1_Arg> TraitImpl_21 for OvertDelegate<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let result = self.the_impl.invoke(&mut b, slf, other).await?;
        b.return_expr(result)
    }
}
#[async_trait]
impl<Impl: TraitDef_2_Types_1_Arg> TraitDef_2_Types_1_Arg for OvertDelegate<Impl> {
    type Owner = Impl::Owner;
    type Other = Impl::Other;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> (Self::Owner, Self::Other) {
        self.the_impl.domain()
    }
}
#[async_trait]
impl<Impl: TraitDef_2_Types_2_Args> TraitImpl_22 for OvertDelegate<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
        let result = self.the_impl.invoke(&mut b, slf, other).await?;
        b.return_expr(result)
    }
}
#[async_trait]
impl<Impl: TraitDef_2_Types_2_Args> TraitDef_2_Types_2_Args for OvertDelegate<Impl> {
    type Owner = Impl::Owner;
    type Other = Impl::Other;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> (Self::Owner, Self::Other) {
        self.the_impl.domain()
    }
}

// TODO OvertDelgate 12f and 12i