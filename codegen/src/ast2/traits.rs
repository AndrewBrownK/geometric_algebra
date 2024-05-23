use std::sync::Arc;
use naga::FastHashMap;
use async_trait::async_trait;
use std::marker::PhantomData;
use parking_lot::RwLock;
use crate::algebra::MultiVectorClass;
use crate::ast2::datatype::{ClassesFromRegistry, DataType};
use crate::ast2::{MultiVectorParam, RawVariableDeclaration, VariableDeclaration, VariableInvocation};
use crate::ast2::expressions::{AnyExpression, ExpressionOf};

enum TraitTypeConsensus {
    NoVotes,
    AllAgree(DataType),
    Disagreement
}

enum TraitParam {
    Generic(TraitConstraint),
    Fixed(DataType)
}

pub struct TraitConstraint {
    requirements: Vec<String>,
}

pub struct RawTraitDefinition {
    name: String,
    aliases: Vec<String>,
    documentation: String,
    inherits: Vec<Arc<RawTraitDefinition>>,

    owner: Arc<RwLock<TraitTypeConsensus>>,
    owner_is_param: bool,
    params: Vec<TraitParam>,
    output: Arc<RwLock<TraitTypeConsensus>>,
}

// TODO Inputs needs to stand in for many MVC per parameter, not just one per parameter
pub struct TraitDefinition<Inherits, Owner, Inputs, /*Output*/> {
    name: String,
    aliases: Vec<String>,
    documentation: String,
    // TODO I'm not sure I like how inherits looks actually
    inherits: Inherits,

    owner: Owner,
    inputs: Inputs,
    // outputs: Output
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_0Param {
    type Owner: ClassesFromRegistry;
    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, (), HasNotReturned>,
        owner: Arc<MultiVectorClass>,
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_1Param {
    type Owner: ClassesFromRegistry;
    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, (), HasNotReturned>,
        slf: MultiVectorParam,
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_1Param {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, (), HasNotReturned>,
        slf: MultiVectorParam,
        other: Arc<MultiVectorParam>,
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_2Param {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    async fn general_impl<'impls>(
        b: TraitImplBuilder<'impls, (), HasNotReturned>,
        slf: MultiVectorParam,
        other: MultiVectorParam,
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>>;
}


pub struct TraitDefBuilder {
    // TODO
}


pub struct RawTraitImplementation {
    definition: Arc<RawTraitDefinition>,
    output: DataType,
    // TODO transitive dependencies to avoid cycles (maybe just a feature of the builder)
    dependencies: Vec<Arc<RawTraitImplementation>>,
    owner: TraitParam,
    owner_is_param: bool,
    other_params: Vec<DataType>,
    variables: FastHashMap<String, RawVariableDeclaration>,
    return_expression: AnyExpression,
    specialized_override: bool,
}

pub struct TraitDefRegistry(
    FastHashMap<String, Arc<RawTraitDefinition>>
);

pub struct TraitImplRegistry {
    // TODO split this up by number of parameters and stuff so we can efficiently
    //  look up by whichever approach we prefer, by owning type or by trait name
    all: FastHashMap<(String, Vec<String>), Arc<RawTraitImplementation>>,
}


pub struct HasNotReturned;

pub struct HasReturned;

pub struct TraitImplBuilder<'impls, Vars, ReturnStatus> {
    registry: &'impls TraitImplRegistry,
    trait_def: Arc<RawTraitDefinition>,
    variables: Vars,
    raw_variables: Vec<(String, RawVariableDeclaration)>,
    return_expr: Option<AnyExpression>,
    return_status: PhantomData<ReturnStatus>,
}

impl<'impls, Vars> TraitImplBuilder<'impls, Vars, HasNotReturned> {
    pub fn assign_var<'vars, Ty, Expr>(
        mut self, var_name: String, var_expr: Expr
    ) -> (
        VariableInvocation<'vars, Ty>,
        TraitImplBuilder<'impls, (Arc<VariableDeclaration<'vars, Ty>>, Vars), HasNotReturned>
    ) where
        Expr: ExpressionOf<'vars, Ty> + 'vars,
        Ty: Clone + Into<DataType> {

        // TODO obviously check for unique variable name, and mangle if necessary
        let fixed_name = var_name;
        let ty: Ty = var_expr.ty().clone();
        let expr = Some(var_expr.into());
        let variable_declaration = VariableDeclaration {
            name: fixed_name.clone(),
            ty,
            phantom: PhantomData,
            expr,
        };
        self.raw_variables.push((fixed_name.clone(), variable_declaration.clone().into()));
        let variable_declaration = Arc::new(variable_declaration);
        let mut new_self = TraitImplBuilder {
            registry: self.registry,
            trait_def: self.trait_def,
            variables: (variable_declaration.clone(), self.variables),
            raw_variables: self.raw_variables,
            return_expr: self.return_expr,
            return_status: self.return_status,
        };
        return (VariableInvocation { var: variable_declaration, }, new_self);
    }

    pub fn return_expr<'var, DT, Expr: ExpressionOf<'var, DT>>(
        self, expr: Expr
    ) -> Option<TraitImplBuilder<'impls, (), HasReturned>> {
        return Some(TraitImplBuilder {
            registry: self.registry,
            trait_def: self.trait_def,
            variables: (),
            raw_variables: self.raw_variables,
            return_expr: Some(expr.into()),
            return_status: PhantomData,
        })
    }
}


impl<'impls> TraitImplBuilder<'impls, (), HasReturned> {
    fn register(self, impls: &mut TraitImplRegistry) {
        let thing = &mut impls.all;
        let trait_name = self.trait_def.name.clone();
        let class_names = vec![];
        let raw = Arc::new(self.into());
        thing.insert((trait_name, class_names), raw);
    }
}


impl<'impls> From<TraitImplBuilder<'impls, (), HasReturned>> for RawTraitImplementation {
    fn from(value: TraitImplBuilder<'impls, (), HasReturned>) -> Self {
        todo!()
    }
}