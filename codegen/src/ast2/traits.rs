use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use either::Either;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use regex::Regex;

use crate::ast2::{RawVariableDeclaration, RawVariableInvocation, Variable};
use crate::ast2::datatype::{AnyClasses, ClassesFromRegistry, ExpressionType, MultiVector};
use crate::ast2::expressions::{AnyExpression, Expression, extract_multivector_expr, TraitResultType};
use crate::utility::AsyncMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraitTypeConsensus {
    NoVotes,
    AllAgree(ExpressionType),
    Disagreement
}
impl TraitTypeConsensus {
    pub fn add_vote(slf: &Arc<RwLock<TraitTypeConsensus>>, expr_type: ExpressionType) {
        let output = slf.read();
        match output.deref() {
            TraitTypeConsensus::Disagreement => return,
            TraitTypeConsensus::AllAgree(agreed) if *agreed == expr_type => return,
            TraitTypeConsensus::NoVotes | TraitTypeConsensus::AllAgree(_) => {
                drop(output);
                let mut owners = slf.write();
                *owners = match owners.deref() {
                    TraitTypeConsensus::NoVotes => TraitTypeConsensus::AllAgree(expr_type),
                    TraitTypeConsensus::AllAgree(agreed) if *agreed == expr_type => return,
                    TraitTypeConsensus::AllAgree(_actually_not) => TraitTypeConsensus::Disagreement,
                    _ => return,
                }
            }
        }
    }
}


pub enum TraitParam {
    Generic,
    Fixed(ExpressionType)
}

pub struct RawTraitDefinition {
    documentation: String,
    names: TraitNames,
    // TODO inherits?
    // inherits: Vec<Arc<RawTraitDefinition>>,

    owner: Arc<RwLock<TraitTypeConsensus>>,
    owner_is_param: bool,
    // TODO generic params?
    // params: Vec<TraitParam>,
    output: Arc<RwLock<TraitTypeConsensus>>,
}


#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_0Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    fn trait_names(&self) -> TraitNames;
    fn general_documentation() -> String { String::new() }

    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    fn def(trait_names: TraitNames) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: Self::general_documentation(),
            names: trait_names,
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            owner_is_param: false,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
        })
    }

    async fn invoke<'impl_ctx>(
        &self,
        b: &'impl_ctx mut TraitImplBuilder<'impl_ctx, HasNotReturned>,
        owner: MultiVector
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let cycle_detector_key = (trait_key.clone(), owner.clone(), None);
        if b.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = b.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            b.cycle_detector.insert(cycle_detector_key);
        }
        if b.inline_dependencies {
            let return_as_var = self.inline(b, owner).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_10(return_as_var));
        }

        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner.clone());
        let owner_clone = owner.clone();
        let trait_key_clone = trait_key.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let builder = TraitImplBuilder::new(
                the_def_clone, registry, false, &mut fresh_variable_scope, cycle_detector_clone
            );
            let trait_impl = Self::general_implementation(builder, owner_clone.clone()).await?;
            Some(trait_impl.into_trait10(trait_key_clone, owner_clone))
        };
        let the_impl = b.registry.traits10.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type);
        TraitTypeConsensus::add_vote(&the_def.output, return_type);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = b.traits10_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => { Some(mv.strong_expression_type()) }
            _ => None,
        };
        let invocation_expression = <Self::Output as TraitResultType>::expr_10(trait_key.clone(), owner, mv_result);

        Some(invocation_expression)
    }

    async fn inline<'impl_ctx>(
        &self,
        b: &'impl_ctx mut TraitImplBuilder<'impl_ctx, HasNotReturned>,
        owner: MultiVector
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let builder = TraitImplBuilder::new(
            the_def, b.registry.clone(), b.inline_dependencies, &mut b.variables, b.cycle_detector.clone()
        );
        // TODO manual overrides/specialization looks like it will be tricky.
        let trait_impl = Self::general_implementation(builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.into_append().append_to(b, var_name)
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_1Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    fn trait_names(&self) -> TraitNames;
    fn general_documentation() -> String { String::new() }
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    fn def(trait_names: TraitNames) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: Self::general_documentation(),
            names: trait_names,
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            owner_is_param: true,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
        })
    }

    async fn invoke<Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>, owner: Expr
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.strong_expression_type();
        let owner_param = owner;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), None);
        if b.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = b.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            b.cycle_detector.insert(cycle_detector_key);
        }
        if b.inline_dependencies {
            let return_as_var = self.inline(b, owner_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_11(return_as_var));
        }

        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        let trait_key_clone = trait_key.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = Arc::new(RawVariableDeclaration {
                comment: None,
                name: "self".to_string(),
                expr: None,
            });
            fresh_variable_scope.insert("self".to_string(), declare_self.clone());
            let builder = TraitImplBuilder::new(
                the_def_clone, registry, false, &mut fresh_variable_scope, cycle_detector_clone
            );
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = Self::general_implementation(builder, var_self).await?;
            Some(trait_impl.into_trait11(trait_key_clone, owner_class_clone))
        };
        let the_impl = b.registry.traits11.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type);
        TraitTypeConsensus::add_vote(&the_def.output, return_type);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = b.traits11_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => { Some(mv.strong_expression_type()) }
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_11(trait_key.clone(), owner_param, mv_result);

        Some(invocation_expression)
    }
    async fn inline<Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>, owner: Expr
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let mut builder = TraitImplBuilder::new(
            the_def, b.registry.clone(), b.inline_dependencies, &mut b.variables, b.cycle_detector.clone()
        );
        // TODO manual overrides/specialization looks like it will be tricky.
        let owner = builder.coerce_variable("self", owner);
        let trait_impl = Self::general_implementation(builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.into_append().append_to(b, var_name)
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_1Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Other: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    fn trait_names(&self) -> TraitNames;
    fn general_documentation() -> String { String::new() }
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    fn def(trait_names: TraitNames) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: Self::general_documentation(),
            names: trait_names,
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            owner_is_param: true,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
        })
    }

    async fn invoke<Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: Expr,
        other: MultiVector
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.strong_expression_type();
        let owner_param = owner;
        let other_class = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), Some(other_class.clone()));
        if b.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = b.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            b.cycle_detector.insert(cycle_detector_key);
        }
        if b.inline_dependencies {
            let return_as_var = self.inline(b, owner_param, other_class.clone()).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_21(return_as_var));
        }

        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let trait_key_clone = trait_key.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = Arc::new(RawVariableDeclaration {
                comment: None,
                name: "self".to_string(),
                expr: None,
            });
            fresh_variable_scope.insert("self".to_string(), declare_self.clone());
            let builder = TraitImplBuilder::new(
                the_def_clone, registry, false, &mut fresh_variable_scope, cycle_detector_clone
            );
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = Self::general_implementation(builder, var_self, other_class_clone.clone()).await?;
            Some(trait_impl.into_trait21(trait_key_clone, owner_class_clone, other_class_clone.clone()))
        };
        let the_impl = b.registry.traits21.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type);
        TraitTypeConsensus::add_vote(&the_def.output, return_type);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = b.traits21_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => { Some(mv.strong_expression_type()) }
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_21(trait_key.clone(), owner_param, other_class, mv_result);

        Some(invocation_expression)
    }

    async fn inline<Expr: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>, owner: Expr, other: MultiVector
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let mut builder = TraitImplBuilder::new(
            the_def, b.registry.clone(), b.inline_dependencies, &mut b.variables, b.cycle_detector.clone()
        );
        // TODO manual overrides/specialization looks like it will be tricky.
        let owner = builder.coerce_variable("self", owner);
        let trait_impl = Self::general_implementation(builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.into_append().append_to(b, var_name)
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_2Param {
    type Owner: ClassesFromRegistry = AnyClasses;
    type Other: ClassesFromRegistry = AnyClasses;
    type Output: TraitResultType;
    fn trait_names(&self) -> TraitNames;
    fn general_documentation() -> String { String::new() }
    async fn general_implementation<'impls>(
        b: TraitImplBuilder<'impls, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<'impls, Self::Output>>;

    fn def(trait_names: TraitNames) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: Self::general_documentation(),
            names: trait_names,
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            owner_is_param: true,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
        })
    }

    async fn invoke<Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: Expr1,
        other: Expr2
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.strong_expression_type();
        let owner_param = owner;
        let other_class = other.strong_expression_type();
        let other_param = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), Some(other_class.clone()));
        if b.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = b.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            b.cycle_detector.insert(cycle_detector_key);
        }
        if b.inline_dependencies {
            let return_as_var = self.inline(b, owner_param, other_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_22(return_as_var));
        }

        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let trait_key_clone = trait_key.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = Arc::new(RawVariableDeclaration {
                comment: None,
                name: "self".to_string(),
                expr: None,
            });
            let declare_other = Arc::new(RawVariableDeclaration {
                comment: None,
                name: "other".to_string(),
                expr: None,
            });
            fresh_variable_scope.insert("self".to_string(), declare_self.clone());
            fresh_variable_scope.insert("other".to_string(), declare_other.clone());
            let builder = TraitImplBuilder::new(
                the_def_clone, registry, false, &mut fresh_variable_scope, cycle_detector_clone
            );
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let var_other: Variable<MultiVector> = Variable {
                expr_type: other_class.clone(),
                decl: declare_other,
            };
            let trait_impl = Self::general_implementation(builder, var_self, var_other).await?;
            Some(trait_impl.into_trait22(trait_key_clone.clone(), owner_class_clone.clone(), other_class_clone.clone()))
        };
        let the_impl = b.registry.traits22.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type);
        TraitTypeConsensus::add_vote(&the_def.output, return_type);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = b.traits22_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => { Some(mv.strong_expression_type()) }
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let other_param = extract_multivector_expr(other_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_22(trait_key.clone(), owner_param, other_param, mv_result);

        Some(invocation_expression)
    }

    async fn inline<Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        b: &mut TraitImplBuilder<HasNotReturned>,
        owner: Expr1,
        other: Expr2
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let trait_names = self.trait_names();
        let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { Self::def(trait_names) }).await;
        let mut builder = TraitImplBuilder::new(
            the_def, b.registry.clone(), b.inline_dependencies, &mut b.variables, b.cycle_detector.clone()
        );
        // TODO manual overrides/specialization looks like it will be tricky.
        let owner = builder.coerce_variable("self", owner);
        let other = builder.coerce_variable("other", other);
        let trait_impl = Self::general_implementation(builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.into_append().append_to(b, var_name)
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
    other_type_params: Vec<TraitParam>,
    other_var_params: Vec<TraitParam>,
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

    pub fn as_upper_camel(&self) -> String {
        self.final_name.to_string()
    }

    pub fn as_lower_camel(&self) -> String {
        let n = self.final_name;
        let f = n[0..1].to_lowercase();
        let inal_name = n[1..n.len()].to_string();
        format!("{f}{inal_name}")
    }

    pub fn as_lower_snake(&self) -> String {
        let mut snake = String::new();
        let mut chars = self.final_name.chars().peekable();

        while let Some(c) = chars.next() {
            if c.is_uppercase() {
                // If not the first character and the next character is not uppercase,
                // or the previous character is not uppercase, add an underscore.
                if !snake.is_empty() &&
                    (chars.peek().map_or(false, |next| !next.is_uppercase()) ||
                        snake.chars().last().map_or(false, |prev| !prev.is_uppercase())) {
                    snake.push('_');
                }
                for lowercase in c.to_lowercase() {
                    snake.push(lowercase);
                }
            } else {
                snake.push(c);
            }
        }

        snake
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

pub struct TraitImplBuilder<'impl_ctx, ReturnType> {
    registry: TraitImplRegistry,
    trait_def: Arc<RawTraitDefinition>,
    inline_dependencies: bool,

    cycle_detector: im::HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
    traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,

    variables: &'impl_ctx mut HashMap<String, Arc<RawVariableDeclaration>>,
    lines: Vec<CommentOrVariableDeclaration>,
    return_comment: Option<String>,
    return_expr: Option<AnyExpression>,
    return_type: ReturnType,
}

impl<'impl_ctx> TraitImplBuilder<'impl_ctx, HasNotReturned> {
    fn new(
        trait_def: Arc<RawTraitDefinition>,
        registry: TraitImplRegistry,
        inline_dependencies: bool,
        variables: &'impl_ctx mut HashMap<String, Arc<RawVariableDeclaration>>,
        cycle_detector: im::HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
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
            return_type: HasNotReturned,
        }
    }

    fn make_var_name_unique(&mut self, var_name: String) -> String {
        let mut unique_name = var_name.to_string();
        let mut counter = 1;
        while self.variables.contains_key(&unique_name) {
            unique_name = format!("{}_{}", var_name, format!("{:02}", counter));
            counter += 1;
        }
        unique_name
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
        self.comment_variable_impl(None::<String>, var_name, expr.strong_expression_type(), expr.into_any_expression())
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
        self.comment_variable_impl(Some(comment), var_name, expr.strong_expression_type(), expr.into_any_expression())
    }

    fn comment_variable_impl<
        C: Into<String>,
        V: Into<String>,
        ExprType,
    >(
        &mut self,
        comment: Option<C>,
        var_name: V,
        expr_type: ExprType,
        expr: AnyExpression
    ) -> Variable<ExprType> {
        let var_name = var_name.into();
        let unique_name = self.make_var_name_unique(var_name);
        let decl = Arc::new(RawVariableDeclaration {
            comment: comment.map(|it| it.into()),
            name: unique_name.clone(),
            expr: Some(expr),
        });
        let existing = self.variables.insert(unique_name.clone(), decl.clone());
        assert!(existing.is_none(), "Variable {unique_name} is already taken");
        self.lines.push(CommentOrVariableDeclaration::VarDec(decl.clone()));
        Variable { expr_type, decl, }
    }

    fn coerce_variable<V: Into<String>, ExprType, Expr: Expression<ExprType>>(&mut self, name_if_new_var: V, expr: Expr) -> Variable<ExprType> {
        return match expr.try_into_variable() {
            Either::Right(already_done) => already_done,
            Either::Left(not_yet) => self.variable(name_if_new_var, not_yet),
        }
    }

    pub fn return_expr<ExprType, Expr: Expression<ExprType>>(
        self, expr: Expr
    ) -> Option<TraitImplBuilder<'impl_ctx, ExprType>> {
        self.comment_return_impl(None::<String>, expr)
    }

    pub fn comment_return<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: C, expr: Expr
    ) -> Option<TraitImplBuilder<'impl_ctx, ExprType>> {
        self.comment_return_impl(Some(comment), expr)
    }

    fn comment_return_impl<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: Option<C>, expr: Expr
    ) -> Option<TraitImplBuilder<'impl_ctx, ExprType>> {
        let return_type = expr.strong_expression_type();
        return Some(TraitImplBuilder {
            registry: self.registry,
            // trait_def: self.trait_def,
            trait_def: self.trait_def,
            inline_dependencies: false,
            cycle_detector: self.cycle_detector,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            variables: self.variables,
            lines: self.lines,
            return_comment: comment.map(|it| it.into()),
            return_expr: Some(expr.into_any_expression()),
            return_type,
        })
    }
}


impl<'impl_ctx, ExprType> TraitImplBuilder<'impl_ctx, ExprType> {
    fn into_trait10(
        self, tk: TraitKey, owner: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        return Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: TraitParam::Fixed(ExpressionType::Class(owner.clone())),
            owner_is_param: false,
            other_type_params: vec![],
            other_var_params: vec![],
            lines: self.lines,
            return_comment: self.return_comment,
            // This shouldn't be a problem because of type level state and function visibilities
            return_expr: self.return_expr.expect("Must have return expression in order to register"),
        });
    }

    fn into_trait11(
        self, tk: TraitKey, owner: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        return Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: TraitParam::Fixed(ExpressionType::Class(owner.clone())),
            owner_is_param: true,
            other_type_params: vec![],
            other_var_params: vec![],
            lines: self.lines,
            return_comment: self.return_comment,
            // This shouldn't be a problem because of type level state and function visibilities
            return_expr: self.return_expr.expect("Must have return expression in order to register"),
        });
    }

    fn into_trait21(
        self, tk: TraitKey, owner: MultiVector, other: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        return Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: TraitParam::Fixed(ExpressionType::Class(owner.clone())),
            owner_is_param: true,
            other_type_params: vec![TraitParam::Fixed(ExpressionType::Class(other))],
            other_var_params: vec![],
            lines: self.lines,
            return_comment: self.return_comment,
            // This shouldn't be a problem because of type level state and function visibilities
            return_expr: self.return_expr.expect("Must have return expression in order to register"),
        });
    }

    fn into_trait22(
        self, tk: TraitKey, owner: MultiVector, other: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        return Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: TraitParam::Fixed(ExpressionType::Class(owner.clone())),
            owner_is_param: true,
            other_type_params: vec![TraitParam::Fixed(ExpressionType::Class(other.clone()))],
            other_var_params: vec![TraitParam::Fixed(ExpressionType::Class(other))],
            lines: self.lines,
            return_comment: self.return_comment,
            // This shouldn't be a problem because of type level state and function visibilities
            return_expr: self.return_expr.expect("Must have return expression in order to register"),
        });
    }
}

pub struct BuilderAppend<ReturnType> {
    lines: Vec<CommentOrVariableDeclaration>,
    return_comment: Option<String>,
    return_expr: Option<AnyExpression>,
    return_type: ReturnType,
}

impl<'impl_ctx, ExprType> TraitImplBuilder<'impl_ctx, ExprType> {
    fn into_append(self) -> BuilderAppend<ExprType> {
        BuilderAppend {
            lines: self.lines,
            return_comment: self.return_comment,
            return_expr: self.return_expr,
            return_type: self.return_type,
        }
    }
}

impl<ExprType> BuilderAppend<ExprType> {
    fn append_to<V: Into<String>>(self, b: &mut TraitImplBuilder<HasNotReturned>, var_name: V) -> Option<Variable<ExprType>> {
        for line in self.lines {
            b.lines.push(line);
        }

        // If there was no return expression provided, assume the implementation "failed"
        // under normal circumstances like some combination of classes that doesn't produce a result
        let var = b.comment_variable_impl(self.return_comment, var_name, self.return_type, self.return_expr?);
        Some(var)
    }
}
