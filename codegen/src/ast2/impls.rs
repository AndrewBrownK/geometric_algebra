use async_trait::async_trait;
use crate::algebra2::basis::BasisElement;
use crate::ast2::datatype::MultiVector;
use crate::ast2::traits::{HasNotReturned, TraitAlias, TraitDef_1Class_0Param, TraitDef_1Class_1Param, TraitDef_2Class_1Param, TraitDef_2Class_2Param, TraitImpl_10, TraitImpl_11, TraitImpl_21, TraitImpl_22, TraitImplBuilder, TraitKey, TraitNames};
use crate::ast2::Variable;

#[derive(Clone, Copy)]
pub struct Elaborated<Impl> {
    pub trait_names: TraitNames,
    blurb: &'static str,
    the_impl: Impl
}
pub fn standard_documentation(n: TraitNames, blurb: &'static str) -> String {
    let name = n.trait_key.as_upper_camel();
    let mut aliases = String::new();
    let mut has_output_aliases_yet = false;
    for alias in n.aliases {
        let Some(alias) = alias else { continue };
        if !alias.mention_in_documentation {
            continue
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
    pub const fn new<const D: usize>(
        the_impl: Impl,
        name: &'static str,
        blurb: &'static str,
        aliases: [TraitAlias; D],
    ) -> Self where [(); D]: Sized {
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

    pub const fn alias(mut self, alias: &'static str) -> Self {
        let alias = TraitAlias::usual(alias);
        self.alias_custom(alias);
        self
    }

    pub const fn alias_docs_only(mut self, alias: &'static str) -> Self {
        let alias = TraitAlias::docs_only(alias);
        self.alias_custom(alias);
        self
    }

    pub const fn alias_except_shaders(mut self, alias: &'static str) -> Self {
        let alias = TraitAlias::usual_except_shaders(alias);
        self.alias_custom(alias);
        self
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


#[async_trait]
impl<Impl: TraitImpl_10> TraitImpl_10 for Elaborated<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<'impls, const AntiScalar: BasisElement>(b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>, owner: MultiVector) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
        Impl::general_implementation(b, owner).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Impl: TraitImpl_10> TraitDef_1Class_0Param<AntiScalar> for Elaborated<Impl> {
    fn trait_names(&self) -> TraitNames {
        self.trait_names.clone()
    }
    fn general_documentation(&self) -> String {
        standard_documentation(
            <Elaborated<Impl> as TraitDef_1Class_0Param<AntiScalar>>::trait_names(self),
            self.blurb
        )
    }
}
#[async_trait]
impl<Impl: TraitImpl_11> TraitImpl_11 for Elaborated<Impl> {
    type Output = Impl::Output;

    async fn general_implementation<'impls, const AntiScalar: BasisElement>(b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>, slf: Variable<MultiVector>) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
        Impl::general_implementation(b, slf).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Impl: TraitImpl_11> TraitDef_1Class_1Param<AntiScalar> for Elaborated<Impl> {
    fn trait_names(&self) -> TraitNames {
        self.trait_names.clone()
    }
    fn general_documentation(&self) -> String {
        standard_documentation(
            <Elaborated<Impl> as TraitDef_1Class_1Param<AntiScalar>>::trait_names(self),
            self.blurb
        )
    }
}
#[async_trait]
impl<Impl: TraitImpl_21> TraitImpl_21 for Elaborated<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<'impls, const AntiScalar: BasisElement>(b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>, slf: Variable<MultiVector>, other: MultiVector) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
        Impl::general_implementation(b, slf, other).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Impl: TraitImpl_21> TraitDef_2Class_1Param<AntiScalar> for Elaborated<Impl> {
    fn trait_names(&self) -> TraitNames {
        self.trait_names.clone()
    }
    fn general_documentation(&self) -> String {
        standard_documentation(
            <Elaborated<Impl> as TraitDef_2Class_1Param<AntiScalar>>::trait_names(self),
            self.blurb
        )
    }
}
#[async_trait]
impl<Impl: TraitImpl_22> TraitImpl_22 for Elaborated<Impl> {
    type Output = Impl::Output;
    async fn general_implementation<'impls, const AntiScalar: BasisElement>(b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>, slf: Variable<MultiVector>, other: Variable<MultiVector>) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
        Impl::general_implementation(b, slf, other).await
    }
}
#[async_trait]
impl<const AntiScalar: BasisElement, Impl: TraitImpl_22> TraitDef_2Class_2Param<AntiScalar> for Elaborated<Impl> {
    fn trait_names(&self) -> TraitNames {
        self.trait_names.clone()
    }
    fn general_documentation(&self) -> String {
        standard_documentation(
            <Elaborated<Impl> as TraitDef_2Class_2Param<AntiScalar>>::trait_names(self),
            self.blurb
        )
    }
}