use std::sync::Arc;
use naga::FastHashMap;
use async_trait::async_trait;
use std::marker::PhantomData;
use parking_lot::RwLock;
use crate::algebra::MultiVectorClass;
use crate::ast2::datatype::{ExpressionType, ClassesFromRegistry, MultiVector, AnyClasses};
use crate::ast2::{RawVariableDeclaration, Variable};
use crate::ast2::expressions::{TraitResultType, TraitResult, Expression, AnyExpression};

enum TraitTypeConsensus {
    NoVotes,
    AllAgree(ExpressionType),
    Disagreement
}

enum TraitParam {
    Generic(TraitConstraint),
    Fixed(ExpressionType)
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
    type Owner: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    // TODO I feel that this bit is still awkward
    fn result_type(result: &Self::Output) -> TraitResult;
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<'impls, HasReturned>>;
    // TODO invokes
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_1Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    // TODO I feel that this bit is still awkward
    fn result_type(result: &Self::Output) -> TraitResult;
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<'impls, HasReturned>>;

    // TODO give bool option to inline or not, then create variants of invoke that just say it outright
    async fn invoke<Expr: Expression<MultiVector>>(
        b: &mut TraitImplBuilder<HasNotReturned>, owner: Expr
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        todo!()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_1Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Other: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    // TODO I feel that this bit is still awkward
    fn result_type(result: &Self::Output) -> TraitResult;
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<'impls, HasReturned>>;
    async fn invoke<Expr: Expression<MultiVector>>(
        b: &mut TraitImplBuilder<HasNotReturned>, owner: Expr, other: MultiVector
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        todo!()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_2Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Other: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    // TODO I feel that this bit is still awkward
    fn result_type(result: &Self::Output) -> TraitResult;
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<'impls, HasReturned>>;

    async fn invoke<Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        b: &mut TraitImplBuilder<HasNotReturned>, owner: Expr1, other: Expr2
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        todo!()
    }
}


pub struct RawTraitImplementation {
    definition: Arc<RawTraitDefinition>,
    // TODO transitive dependencies to avoid cycles (maybe just a feature of the builder)
    dependencies: Vec<Arc<RawTraitImplementation>>,
    owner: TraitParam,
    owner_is_param: bool,
    other_params: Vec<TraitParam>,
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

pub enum CommentOrVariableDeclaration {
    Comment(String),
    VarDec(Arc<RawVariableDeclaration>)
}

pub struct TraitImplBuilder<'impls, ReturnStatus> {
    registry: &'impls TraitImplRegistry,
    trait_def: Arc<RawTraitDefinition>,
    inline_dependencies: bool,
    variables: FastHashMap<String, Arc<RawVariableDeclaration>>,
    lines: Vec<CommentOrVariableDeclaration>,
    return_comment: Option<String>,
    return_expr: Option<AnyExpression>,
    return_status: PhantomData<ReturnStatus>,
}

impl<'impls> TraitImplBuilder<'impls, HasNotReturned> {

    fn make_var_name_unique(&mut self, var_name: String) -> String {
        // TODO
        var_name
    }

    pub fn comment<C: Into<String>>(&mut self, comment: C) {
        self.lines.push(CommentOrVariableDeclaration::Comment(comment.into()))
    }

    pub fn variable<
        V: Into<String>,
        ExprType,
        Expr: Expression<ExprType>
    >(
        &mut self,
        var_name: V,
        expr: Expr
    ) -> Variable<ExprType> {
        let var_name = var_name.into();
        let unique_name = self.make_var_name_unique(var_name);
        let expr_type = expr.strong_expression_type();
        let decl = Arc::new(RawVariableDeclaration {
            comment: None,
            name: unique_name.clone(),
            expr: Some(expr.into_any_expression()),
        });
        let existing = self.variables.insert(unique_name.clone(), decl.clone());
        assert!(existing.is_none(), "Variable {unique_name} is already taken");
        self.lines.push(CommentOrVariableDeclaration::VarDec(decl.clone()));
        Variable { expr_type, decl, }
    }

    // pub fn comment_variable<
    //     C: Into<String>,
    //     V: Into<String>,
    //     Expr
    // >(
    //     &mut self,
    //     comment: C,
    //     var_name: V,
    //     expr: Expr
    // ) -> Variable<Expr> {
    // }


    pub fn return_expr<ExprType, Expr: Expression<ExprType>>(
        self, expr: Expr
    ) -> Option<TraitImplBuilder<'impls, HasReturned>> {
        return Some(TraitImplBuilder {
            registry: self.registry,
            trait_def: self.trait_def,
            inline_dependencies: false,
            variables: self.variables,
            lines: self.lines,
            return_comment: None,
            return_expr: Some(expr.into_any_expression()),
            return_status: PhantomData,
        })
    }

    pub fn comment_return<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: C, expr: Expr
    ) -> Option<TraitImplBuilder<'impls, HasReturned>> {
        return Some(TraitImplBuilder {
            registry: self.registry,
            trait_def: self.trait_def,
            inline_dependencies: false,
            variables: self.variables,
            lines: self.lines,
            return_comment: Some(comment.into()),
            return_expr: Some(expr.into_any_expression()),
            return_status: PhantomData,
        })
    }
}


impl<'impls> TraitImplBuilder<'impls, HasReturned> {
    fn register(self, impls: &mut TraitImplRegistry) {
        // let thing = &mut impls.all;
        // let trait_name = self.trait_def.name.clone();
        // let class_names = vec![];
        // let raw = Arc::new(self.into());
        // thing.insert((trait_name, class_names), raw);
    }
}






