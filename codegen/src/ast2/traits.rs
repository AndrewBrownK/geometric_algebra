use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use lazy_static::lazy_static;
use naga::FastHashMap;
use parking_lot::RwLock;
use regex::{Regex, RegexBuilder};

use crate::ast2::datatype::{AnyClasses, ClassesFromRegistry, ExpressionType, Integer, MultiVector};
use crate::ast2::expressions::{AnyExpression, Expression, TraitResult, TraitResultType};
use crate::ast2::{RawVariableDeclaration, Variable};

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


#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_0Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    // TODO I feel that this bit is still awkward
    fn result_type(result: &Self::Output) -> TraitResult;
    fn trait_names(&self) -> TraitNames;

    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    async fn invoke(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: MultiVector
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        if b.inline_dependencies {
            return self.inline(b, owner).await;
        }
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.clone());
        let mut raw_impl = b.registry.traits10.get(&impl_key);
        let Some(raw_impl) = b.registry.traits10.get(&impl_key) else {
            // Create and register the implementation
            // TODO actually create impl
            todo!()
        };

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = b.traits10_dependencies.insert(impl_key, raw_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &raw_impl.return_expression {
            AnyExpression::Class(mv) => { Some(mv.strong_expression_type()) }
            _ => None,
        };
        let invocation_expression = <Self::Output as TraitResultType>::into_expr_10(trait_key, owner, mv_result);
        Some(invocation_expression)
    }

    async fn inline(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: MultiVector
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        todo!()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_1Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    // TODO I feel that this bit is still awkward
    fn result_type(result: &Self::Output) -> TraitResult;
    fn trait_names(&self) -> TraitNames;
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    async fn invoke<Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>, owner: Expr
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        if b.inline_dependencies {
            return self.inline(b, owner).await;
        }

        todo!()
    }
    async fn inline<Expr: Expression<MultiVector>>(
        &self,
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
    fn trait_names(&self) -> TraitNames;
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    async fn invoke<Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: Expr,
        other: MultiVector
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        if b.inline_dependencies {
            return self.inline(b, owner, other).await;
        }
        todo!()
    }

    async fn inline<Expr: Expression<MultiVector>>(
        &self,
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
    fn trait_names(&self) -> TraitNames;
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    async fn invoke<Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: Expr1,
        other: Expr2
    ) -> Option<<Self::Output as TraitResultType>::ExprType> {
        if b.inline_dependencies {
            return self.inline(b, owner, other).await;
        }
        todo!()
    }

    async fn inline<Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: Expr1,
        other: Expr2
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

/// Each TraitKey should be the final name of a trait, and correspond
/// to exactly one trait item in rust. It should be in UpperCamelCase
/// with no funny business or special characters. It will be converted
/// to lowerCamelCase for output in shaders and lower_snake_case for
/// function names inside the trait item.
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, Hash)]
pub struct TraitKey {
    final_name: &'static str,
}

lazy_static! {
    static ref TRAIT_KEY_REGEX: Regex = Regex::new("^[A-Z][a-zA-Z0-9]+$").expect("TraitKey regex is valid");
}
impl TraitKey {
    pub fn new(name: &'static str) -> Self {
        assert!(
            TRAIT_KEY_REGEX.is_match(name),
            "TraitKeys must be UpperCamelCase without any funny business or special characters."
        );
        Self {
            final_name: name,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, Hash)]
pub struct TraitNames {
    pub trait_key: TraitKey,
    pub aliases: Vec<TraitAlias>,
}
impl TraitNames {
    pub fn just(name: &'static str) -> Self {
        TraitNames {
            trait_key: TraitKey::new(name),
            aliases: vec![],
        }
    }
}


#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, Hash)]
pub struct TraitAlias {
    pub alias_key: TraitKey,
    pub mention_in_documentation: bool,
    pub rust_trait_sharing: TraitSharing,
    pub output_in_shaders: bool
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum TraitSharing {
    DontOutput,
    Consolidate,
    DistinctTrait,
}

impl TraitAlias {
    fn usual(alias_key: TraitKey) -> Self {
        Self::new(alias_key, true, TraitSharing::Consolidate, true)
    }
    fn usual_except_shaders(alias_key: TraitKey) -> Self {
        Self::new(alias_key, true, TraitSharing::Consolidate, false)
    }
    fn docs_only(alias_key: TraitKey) -> Self {
        Self::new(alias_key, true, TraitSharing::DontOutput, false)
    }
    fn new(alias_key: TraitKey, docs: bool, share: TraitSharing, shaders: bool) -> Self {
        TraitAlias {
            alias_key,
            mention_in_documentation: docs,
            rust_trait_sharing: share,
            output_in_shaders: shaders
        }
    }
}



pub struct TraitImplRegistry {
    traits10: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
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

    traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,

    variables: HashMap<String, Arc<RawVariableDeclaration>>,
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
        self.comment_variable_impl(None::<String>, var_name, expr)
    }

    pub fn comment_variable<
        C: Into<String>,
        V: Into<String>,
        ExprType,
        Expr: Expression<ExprType>
    >(
        &mut self,
        comment: C,
        var_name: V,
        expr: Expr
    ) -> Variable<ExprType> {
        self.comment_variable_impl(Some(comment), var_name, expr)
    }

    fn comment_variable_impl<
        C: Into<String>,
        V: Into<String>,
        ExprType,
        Expr: Expression<ExprType>
    >(
        &mut self,
        comment: Option<C>,
        var_name: V,
        expr: Expr
    ) -> Variable<ExprType> {
        let var_name = var_name.into();
        let unique_name = self.make_var_name_unique(var_name);
        let expr_type = expr.strong_expression_type();
        let decl = Arc::new(RawVariableDeclaration {
            comment: comment.map(|it| it.into()),
            name: unique_name.clone(),
            expr: Some(expr.into_any_expression()),
        });
        let existing = self.variables.insert(unique_name.clone(), decl.clone());
        assert!(existing.is_none(), "Variable {unique_name} is already taken");
        self.lines.push(CommentOrVariableDeclaration::VarDec(decl.clone()));
        Variable { expr_type, decl, }
    }


    // TODO return type should really match the trait definition, it can't just be any type.
    pub fn return_expr<ExprType, Expr: Expression<ExprType>>(
        self, expr: Expr
    ) -> Option<TraitImplBuilder<'impls, ExprType>> {
        self.comment_return_impl(None::<String>, expr)
    }

    pub fn comment_return<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: C, expr: Expr
    ) -> Option<TraitImplBuilder<'impls, ExprType>> {
        self.comment_return_impl(Some(comment), expr)
    }

    fn comment_return_impl<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: Option<C>, expr: Expr
    ) -> Option<TraitImplBuilder<'impls, ExprType>> {
        return Some(TraitImplBuilder {
            registry: self.registry,
            trait_def: self.trait_def,
            inline_dependencies: false,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            variables: self.variables,
            lines: self.lines,
            return_comment: comment.map(|it| it.into()),
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





