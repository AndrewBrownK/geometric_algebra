use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use either::Either;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use regex::Regex;
use tokio::io::AsyncReadExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use crate::ast2::{RawVariableDeclaration, Variable};
use crate::ast2::datatype::{AnyClasses, ClassesFromRegistry, ExpressionType, MultiVector};
use crate::ast2::expressions::{AnyExpression, Expression, TraitResultType};
use crate::utility::{AsyncMap, AsyncMapResult, AwaitOrClone};

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
    documentation: String,
    names: TraitNames,
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
        let trait_key = self.trait_names().trait_key;
        if b.cycle_detector.contains(&(trait_key, owner.clone(), None)) {
            let all_in_cycle = b.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            b.cycle_detector.insert((trait_key.clone(), owner.clone(), None));
        }
        if b.inline_dependencies {
            return self.inline(b, owner).await;
        }


        let f = async {
            todo!()
        };
        let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), f).await;


        let impl_key = (trait_key.clone(), owner.clone());
        let f = async {
            // Create and register the implementation
            // TODO don't forget to add initial variables in traits that do have params
            let mut fresh_variable_scope = HashMap::new();
            let builder = TraitImplBuilder::new(
                the_def,
                b.registry,
                false,
                &mut fresh_variable_scope,
                b.cycle_detector.clone()
            );
            // Do not early return because we have to share None results too
            // So await.map() instead of await?;
            let trait_impl = Self::general_implementation(builder, owner.clone()).await?;
            Some(trait_impl.register10(trait_key.clone(), owner));
        };
        let the_impl = b.registry.traits10.get_or_create_or_panic(impl_key.clone(), f).await?;

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = b.traits10_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
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

    traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,

    owner: TraitParam,
    owner_is_param: bool,
    other_params: Vec<TraitParam>,
    lines: Vec<CommentOrVariableDeclaration>,
    return_comment: Option<String>,
    return_expr: AnyExpression,
    // TODO handle specialized overriding, but maybe don't need this property here. Not sure yet.
    // specialized_override: bool,
}

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


// TODO let the impl registry hold a reference (or Arc) to the definition registry too
#[derive(Clone)]
pub struct TraitImplRegistry {
    defs: TraitDefRegistry,
    // The use of options is to help short circuit the forgoing of implementations
    // that we've already determined cannot exist. Basically, the map might get fat with
    // None entries, but should save us a bit of compute cycles from repeatedly attempting
    // and failing to generate some trait implementations.
    traits10: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits11: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits21: AsyncMap<(TraitKey, MultiVector, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits22: AsyncMap<(TraitKey, MultiVector, MultiVector), Option<Arc<RawTraitImplementation>>>,
}
#[derive(Clone)]
pub struct TraitDefRegistry {
    traits10: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits11: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits21: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits22: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
}


pub struct HasNotReturned;

pub enum CommentOrVariableDeclaration {
    Comment(String),
    VarDec(Arc<RawVariableDeclaration>)
}

pub struct TraitImplBuilder<'build_ctx, ReturnStatus> {
    registry: &'build_ctx TraitImplRegistry,
    trait_def: Arc<RawTraitDefinition>,
    inline_dependencies: bool,

    // TODO cycle detection
    cycle_detector: HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
    traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,

    variables: &'build_ctx mut HashMap<String, Arc<RawVariableDeclaration>>,
    lines: Vec<CommentOrVariableDeclaration>,
    return_comment: Option<String>,
    return_expr: Option<AnyExpression>,
    return_status: PhantomData<ReturnStatus>,
}

impl<'build_ctx> TraitImplBuilder<'build_ctx, HasNotReturned> {
    fn new(
        trait_def: Arc<RawTraitDefinition>,
        registry: &'build_ctx TraitImplRegistry,
        inline_dependencies: bool,
        variables: &'build_ctx mut HashMap<String, Arc<RawVariableDeclaration>>,
        cycle_detector: HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
    ) -> Self {
        TraitImplBuilder {
            registry,
            trait_def,
            inline_dependencies,
            cycle_detector,
            traits10_dependencies: Default::default(),
            traits11_dependencies: Default::default(),
            traits21_dependencies: Default::default(),
            traits22_dependencies: Default::default(),
            variables,
            lines: vec![],
            return_comment: None,
            return_expr: None,
            return_status: PhantomData,
        }
    }

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
    ) -> Option<TraitImplBuilder<'build_ctx, ExprType>> {
        self.comment_return_impl(None::<String>, expr)
    }

    pub fn comment_return<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: C, expr: Expr
    ) -> Option<TraitImplBuilder<'build_ctx, ExprType>> {
        self.comment_return_impl(Some(comment), expr)
    }

    fn comment_return_impl<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: Option<C>, expr: Expr
    ) -> Option<TraitImplBuilder<'build_ctx, ExprType>> {
        return Some(TraitImplBuilder {
            registry: self.registry,
            // trait_def: self.trait_def,
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


impl<'impls, T> TraitImplBuilder<'impls, T> {
    fn register10(
        self, tk: TraitKey, owner: MultiVector
    ) -> Arc<RawTraitImplementation> {
        let result = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: TraitParam::Fixed(ExpressionType::Class(owner.clone())),
            owner_is_param: false,
            other_params: vec![],
            lines: self.lines,
            return_comment: self.return_comment,
            // This shouldn't be a problem because of type level state and function visibilities
            return_expr: self.return_expr.expect("Must have return expression in order to register"),
        });
        let existing = self.registry.traits10.insert((tk, owner), result.clone());
        if existing.is_some() {
            panic!("It is inefficient to generate redundant trait implementations ({tk:?}). Use better concurrency control.")
        }
        return result;
    }
}





