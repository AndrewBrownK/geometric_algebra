#![allow(non_upper_case_globals)]

use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::Deref;
use std::sync::{Arc, Weak};

use async_trait::async_trait;
use indicatif::MultiProgress;
use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use regex::Regex;
use tokio::task::JoinSet;

use crate::algebra::basis::{BasisElement, BasisSignature};
use crate::algebra::multivector::{DynamicMultiVector, MultiVecRepository};
use crate::algebra::GeometricAlgebra;
use crate::ast::datatype::{ClassesFromRegistry, ExpressionType, Float, Integer, MultiVector};
use crate::ast::expressions::{extract_multivector_expr, AnyExpression, Expression, MultiVectorExpr, TraitResultType, IntExpr, FloatExpr, Vec2Expr, Vec3Expr, Vec4Expr, MultiVectorVia, extract_float_expr, extract_integer_expr};
use crate::ast::impls::{Elaborated, InlineOnly, OvertDelegate};
use crate::ast::operations_tracker::{TrackOperations, TraitOperationsLookup, VectoredOperationsTracker};
use crate::ast::{RawVariableDeclaration, Variable};
use crate::utility::AsyncMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraitTypeConsensus {
    NoVotes,
    AllAgree(ExpressionType, bool),
    AlwaysSelf,
    Disagreement,
}
impl TraitTypeConsensus {
    pub fn add_vote(slf: &Arc<RwLock<TraitTypeConsensus>>, expr_type: ExpressionType, is_self: bool) {
        let output = slf.read();
        match output.deref() {
            TraitTypeConsensus::Disagreement => return,
            TraitTypeConsensus::AlwaysSelf if is_self => return,
            TraitTypeConsensus::AllAgree(agreed, was_self) if *agreed == expr_type && is_self == *was_self => return,
            _ => {}
        }
        drop(output);
        let mut owners = slf.write();
        *owners = match owners.deref() {
            TraitTypeConsensus::Disagreement => return,
            TraitTypeConsensus::AlwaysSelf if is_self => return,
            TraitTypeConsensus::AlwaysSelf => TraitTypeConsensus::Disagreement,
            TraitTypeConsensus::NoVotes => TraitTypeConsensus::AllAgree(expr_type, is_self),
            TraitTypeConsensus::AllAgree(agreed_type, was_self) => match (*agreed_type == expr_type, *was_self && is_self) {
                (true, true) => return,
                (true, false) => TraitTypeConsensus::AllAgree(expr_type, false),
                (false, true) => TraitTypeConsensus::AlwaysSelf,
                (false, false) => TraitTypeConsensus::Disagreement,
            },
        }
    }
}

pub type TraitParam = ExpressionType;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TraitArity {
    Zero,
    One,
    Two,
}
impl TraitArity {
    pub fn as_str(&self) -> &'static str {
        match self {
            TraitArity::Zero => "arity_0",
            TraitArity::One => "arity_1",
            TraitArity::Two => "arity_2",
        }
    }
}

pub struct RawTraitDefinition {
    pub(crate) documentation: String,
    pub(crate) names: TraitNames,
    pub(crate) owner: Arc<RwLock<TraitTypeConsensus>>,
    pub(crate) arity: TraitArity,
    pub(crate) output: Arc<RwLock<TraitTypeConsensus>>,
    pub(crate) op: Arc<Mutex<Option<Ops>>>,
    pub(crate) dependencies: Arc<Mutex<HashSet<TraitKey>>>,
}

#[const_trait]
pub trait ProvideTraitNames {
    fn trait_names(&self) -> TraitNames;
}

// The "Copy" ancestor for TraitImpls is experimental.
//  It might seem absurd and far too constraining at first, but actually it might be pretty
//  viable. Even when the implementing type isn't trivial, the ability to define static values
//  combined with the ability to impl traits for &'static Item makes this actually start
//  to look quite viable. I mean, scoped references are all we need for Copy, but the reason
//  we might want to use references to static items is so we can reference them inside
//  general_implementation without access to an enclosing scope (aside from static values).
//  In this way it is almost like the entire rs file becomes a script, defining variables
//  as statics and interdependent script snippets as trait implementations.

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_10: Copy + Send + Sync + 'static {
    type Output: TraitResultType;

    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_0_Args: TraitImpl_10 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner;

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Zero,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let cycle_detector_key = (trait_key.clone(), owner.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = builder.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.insert(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_10(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner.clone());
        let owner_clone = owner.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let fresh_variable_scope = Arc::new(Mutex::new(HashMap::new()));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, fresh_variable_scope, cycle_detector_clone);
            let trait_impl = t_self.general_implementation(builder, owner_clone.clone()).await?;
            Some(trait_impl.into_trait10(owner_clone))
        };
        let the_impl = builder.registry.traits10.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits10_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let invocation_expression = <Self::Output as TraitResultType>::expr_10(trait_key.clone(), owner, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement>(&self, builder: &TraitImplBuilder<AntiScalar, HasNotReturned>, owner: MultiVector) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.clone());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = builder.registry.traits10.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return builder.inline_by_copy_existing_10::<Self>(&trait_key, raw_impl);
            }
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_11: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_1_Arg: TraitImpl_11 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner;

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::One,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = builder.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.insert(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_11(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = t_self.general_implementation(builder, var_self).await?;
            Some(trait_impl.into_trait11(owner_class_clone))
        };
        let the_impl = builder.registry.traits11.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits11_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_11(trait_key.clone(), owner_param, mv_result);

        Some(invocation_expression)
    }
    async fn inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = builder.registry.traits11.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return builder.inline_by_copy_existing_11::<Self, _>(&trait_key, raw_impl, owner);
            }
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_21: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2_Types_1_Arg: TraitImpl_21 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> (Self::Owner, Self::Other);

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::One,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_class = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), Some(other_class.clone()));
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = builder.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.insert(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_class.clone()).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_21(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, other_class_clone.clone()).await?;
            Some(trait_impl.into_trait21(owner_class_clone, other_class_clone.clone()))
        };
        let the_impl = builder.registry.traits21.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits21_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_21(trait_key.clone(), owner_param, other_class, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type(), other.clone());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = builder.registry.traits21.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return builder.inline_by_copy_existing_21::<Self, _>(&trait_key, raw_impl, owner);
            }
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_22: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2_Types_2_Args: TraitImpl_22 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> (Self::Owner, Self::Other);

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Two,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_class = other.expression_type();
        let other_param = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), Some(other_class.clone()));
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = builder.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.insert(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_22(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let declare_other = param_other();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let var_other: Variable<MultiVector> = Variable {
                expr_type: other_class.clone(),
                decl: declare_other,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, var_other).await?;
            Some(trait_impl.into_trait22(owner_class_clone.clone(), other_class_clone.clone()))
        };
        let the_impl = builder.registry.traits22.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits22_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let other_param = extract_multivector_expr(other_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_22(trait_key.clone(), owner_param, other_param, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type(), other.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = builder.registry.traits22.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return builder.inline_by_copy_existing_22::<Self, _, _>(&trait_key, raw_impl, owner, other);
            }
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // The correct/best way to do this is to use regular inline, and then substitute
        //  the variable declarations independently. This is because we support trait definitions
        //  using either invoke or inline, and so we don't know which we're going to get, but
        //  in any case a trait definition might define variables on its own, without necessarily
        //  doing so from an inlined invocation. So if it does that, then you have to replace
        //  variable declarations anyway. Regular inlining works by appending inner builders to
        //  an outer builder with finish_inline, so I think we can use a middle builder. The
        //  outer builder is whatever called deep_inline. The middle builder will do the variable
        //  inlining on the return value, and accumulate inner inlining. The inner builders do
        //  the regular inlinings which inline all the way down and use variables.

        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_12f: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<Float>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_2_Args_f32: TraitImpl_12f + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner;

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Two,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_class = other.expression_type();
        let other_param = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = builder.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.insert(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_12f(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits12f.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        // let other_class_clone = other_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let declare_other = param_other();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let var_other: Variable<Float> = Variable {
                expr_type: Float,
                decl: declare_other,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, var_other).await?;
            Some(trait_impl.into_trait12f(owner_class_clone.clone()))
        };
        let the_impl = builder.registry.traits12f.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits12f_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let other_param = extract_float_expr(other_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_12f(trait_key.clone(), owner_param, other_param, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = builder.registry.traits12f.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return builder.inline_by_copy_existing_12f::<Self, _, _>(&trait_key, raw_impl, owner, other);
            }
        }

        let slf = self.clone();
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let slf = self.clone();
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_12i: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<Integer>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_2_Args_i32: TraitImpl_12i + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn domain(&self) -> Self::Owner;

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Two,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_class = other.expression_type();
        let other_param = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = builder.cycle_detector.iter().collect::<Vec<_>>();
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.insert(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_12i(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits12f.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        // let other_class_clone = other_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let declare_other = param_other();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let var_other: Variable<Integer> = Variable {
                expr_type: Integer,
                decl: declare_other,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, var_other).await?;
            Some(trait_impl.into_trait12i(owner_class_clone.clone()))
        };
        let the_impl = builder.registry.traits12i.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits12i_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let other_param = extract_integer_expr(other_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_12i(trait_key.clone(), owner_param, other_param, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = builder.registry.traits12i.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return builder.inline_by_copy_existing_12i::<Self, _, _>(&trait_key, raw_impl, owner, other);
            }
        }

        let slf = self.clone();
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // The correct/best way to do this is to use regular inline, and then substitute
        //  the variable declarations independently. This is because we support trait definitions
        //  using either invoke or inline, and so we don't know which we're going to get, but
        //  in any case a trait definition might define variables on its own, without necessarily
        //  doing so from an inlined invocation. So if it does that, then you have to replace
        //  variable declarations anyway. Regular inlining works by appending inner builders to
        //  an outer builder with finish_inline, so I think we can use a middle builder. The
        //  outer builder is whatever called deep_inline. The middle builder will do the variable
        //  inlining on the return value, and accumulate inner inlining. The inner builders do
        //  the regular inlinings which inline all the way down and use variables.

        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}

#[marker]
pub trait CanNameTrait {}
impl<I> CanNameTrait for I where I: TraitImpl_10 {}
impl<I> CanNameTrait for I where I: TraitImpl_11 {}
impl<I> CanNameTrait for I where I: TraitImpl_12f {}
impl<I> CanNameTrait for I where I: TraitImpl_12i {}
impl<I> CanNameTrait for I where I: TraitImpl_21 {}
impl<I> CanNameTrait for I where I: TraitImpl_22 {}

#[const_trait]
pub trait NameTrait: Sized + Copy {
    fn new_trait_named(self, name: &'static str) -> Elaborated<Self>;
}
impl<I> const NameTrait for I
where
    I: CanNameTrait + Copy,
{
    fn new_trait_named(self, name: &'static str) -> Elaborated<Self> {
        Elaborated::new_with_name(self, name)
    }
}

pub(crate) struct RawTraitImplementation {
    pub(crate) definition: Arc<RawTraitDefinition>,
    pub(crate) multivector_dependencies: HashSet<MultiVector>,
    pub(crate) traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits12f_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits12i_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,

    pub(crate) owner: TraitParam,
    pub(crate) owner_is_param: bool,
    pub(crate) other_type_params: Vec<TraitParam>,
    pub(crate) other_var_params: Vec<TraitParam>,
    pub(crate) lines: Vec<CommentOrVariableDeclaration>,
    pub(crate) return_comment: Option<String>,
    pub(crate) return_expr: AnyExpression,
    pub(crate) specialized: bool,
    pub(crate) statistics: VectoredOperationsTracker,
}

/// Each TraitKey should be the final name of a trait, and correspond
/// to exactly one trait item in rust. It should be in UpperCamelCase
/// with no funny business or special characters. It will be converted
/// to lowerCamelCase for output in shaders and lower_snake_case for
/// function names inside the trait item.
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub struct TraitKey {
    final_name: &'static str,
}

lazy_static! {
    static ref TRAIT_KEY_REGEX: Regex = Regex::new("^[A-Z][a-zA-Z0-9]+$").expect("TraitKey regex is valid");
}
impl TraitKey {
    pub const fn new(name: &'static str) -> Self {
        // Cannot validate with regex in const eval.
        // So we'll do validation at use sites instead.

        // assert!(
        //     TRAIT_KEY_REGEX.is_match(name),
        //     "TraitKeys must be UpperCamelCase without any funny business or special characters."
        // );
        Self { final_name: name }
    }

    pub fn as_upper_camel(&self) -> String {
        assert!(
            TRAIT_KEY_REGEX.is_match(self.final_name),
            "TraitKeys must be UpperCamelCase without any funny business or special characters."
        );
        self.final_name.to_string()
    }

    pub fn as_lower_camel(&self) -> String {
        assert!(
            TRAIT_KEY_REGEX.is_match(self.final_name),
            "TraitKeys must be UpperCamelCase without any funny business or special characters."
        );
        let n = self.final_name;
        let f = n[0..1].to_lowercase();
        let inal_name = n[1..n.len()].to_string();
        format!("{f}{inal_name}")
    }

    pub fn as_lower_snake(&self) -> String {
        assert!(
            TRAIT_KEY_REGEX.is_match(self.final_name),
            "TraitKeys must be UpperCamelCase without any funny business or special characters."
        );
        let mut snake = String::new();
        let mut chars = self.final_name.chars().peekable();

        while let Some(c) = chars.next() {
            if c.is_uppercase() {
                // If not the first character and the next character is not uppercase,
                // or the previous character is not uppercase, add an underscore.
                if !snake.is_empty() && (chars.peek().map_or(false, |next| !next.is_uppercase()) || snake.chars().last().map_or(false, |prev| !prev.is_uppercase())) {
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

    pub fn as_screaming_snake(&self) -> String {
        self.as_lower_snake().to_uppercase()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub struct TraitNames {
    pub trait_key: TraitKey,
    // ArrayVec does not allow quite enough const operations
    // pub aliases: ArrayVec<[TraitAlias; 16]>,
    pub aliases: [Option<TraitAlias>; 16],
}
impl TraitNames {
    pub const fn just(name: &'static str) -> Self {
        TraitNames {
            trait_key: TraitKey::new(name),
            aliases: [None; 16],
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub struct TraitAlias {
    pub alias_key: TraitKey,
    pub mention_in_documentation: bool,
    pub rust_trait_sharing: TraitSharing,
    pub output_in_shaders: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum TraitSharing {
    DontOutput,
    Consolidate,
    DistinctTrait,
}

impl TraitAlias {
    pub const fn usual(alias: &'static str) -> Self {
        Self::new(alias, true, TraitSharing::Consolidate, true)
    }
    pub const fn usual_except_shaders(alias: &'static str) -> Self {
        Self::new(alias, true, TraitSharing::Consolidate, false)
    }
    pub const fn docs_only(alias: &'static str) -> Self {
        Self::new(alias, true, TraitSharing::DontOutput, false)
    }
    pub const fn new(alias: &'static str, docs: bool, share: TraitSharing, shaders: bool) -> Self {
        TraitAlias {
            alias_key: TraitKey::new(alias),
            mention_in_documentation: docs,
            rust_trait_sharing: share,
            output_in_shaders: shaders,
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum UnaryOps {
    Neg,
    Not,
}
impl UnaryOps {
    pub fn rust_mod(self) -> &'static str {
        "std::ops::"
        // match self {
        //     UnaryOps::Neg => "core::ops::arith::",
        //     UnaryOps::Not => "core::ops::bit::",
        // }
    }
    pub fn rust_trait_name(self) -> &'static str {
        match self {
            UnaryOps::Neg => "Neg",
            UnaryOps::Not => "Not",
        }
    }
    pub fn rust_trait_method(self) -> &'static str {
        match self {
            UnaryOps::Neg => "neg",
            UnaryOps::Not => "not",
        }
    }
    pub fn rust_operator(self) -> &'static str {
        match self {
            UnaryOps::Neg => "-",
            UnaryOps::Not => "!",
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum BinaryOps {
    Add,
    Sub,
    Mul,
    Div,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
}
impl BinaryOps {
    pub fn rust_mod(self) -> &'static str {
        "std::ops::"
        // match self {
        //     BinaryOps::Add | BinaryOps::Sub | BinaryOps::Mul | BinaryOps::Div => "core::ops::arith::",
        //     BinaryOps::Shl | BinaryOps::Shr | BinaryOps::BitAnd | BinaryOps::BitOr | BinaryOps::BitXor => "core::ops::bit::",
        // }
    }

    pub fn rust_trait_name(self) -> &'static str {
        match self {
            BinaryOps::Add => "Add",
            BinaryOps::Sub => "Sub",
            BinaryOps::Mul => "Mul",
            BinaryOps::Div => "Div",
            BinaryOps::Shl => "Shl",
            BinaryOps::Shr => "Shr",
            BinaryOps::BitAnd => "BitAnd",
            BinaryOps::BitOr => "BitOr",
            BinaryOps::BitXor => "BitXor",
        }
    }

    pub fn rust_trait_method(self) -> &'static str {
        match self {
            BinaryOps::Add => "add",
            BinaryOps::Sub => "sub",
            BinaryOps::Mul => "mul",
            BinaryOps::Div => "div",
            BinaryOps::Shl => "shl",
            BinaryOps::Shr => "shr",
            BinaryOps::BitAnd => "bitand",
            BinaryOps::BitOr => "bitor",
            BinaryOps::BitXor => "bitxor",
        }
    }

    pub fn rust_operator(self) -> &'static str {
        match self {
            BinaryOps::Add => "+",
            BinaryOps::Sub => "-",
            BinaryOps::Mul => "*",
            BinaryOps::Div => "/",
            BinaryOps::Shl => "<<",
            BinaryOps::Shr => ">>",
            BinaryOps::BitAnd => "&",
            BinaryOps::BitOr => "|",
            BinaryOps::BitXor => "^",
        }
    }
}
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum ExperimentalOps {
    Deref,
    Fn,
    Index,
    RangeBounds,
    Rem,
    FromResidual,
    OneSidedRange,
    Residual,
    Try,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum Ops {
    Unary(UnaryOps),
    Binary(BinaryOps),
    // Exp(ExperimentalOps),
}
impl Ops {
    pub fn rust_mod(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_mod(),
            Ops::Binary(op) => op.rust_mod(),
        }
    }
    pub const fn into_u32(self) -> u32 {
        match self {
            Ops::Unary(o) => o as u32,
            Ops::Binary(o) => (o as u32) + 2,
        }
    }
    pub fn rust_trait_name(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_trait_name(),
            Ops::Binary(op) => op.rust_trait_name(),
        }
    }
    pub fn rust_trait_method(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_trait_method(),
            Ops::Binary(op) => op.rust_trait_method(),
        }
    }
    pub fn rust_operator(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_operator(),
            Ops::Binary(op) => op.rust_operator(),
        }
    }
}

#[derive(Clone)]
pub struct TraitDefRegistry {
    traits10: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits11: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits12f: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits12i: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits21: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits22: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
}
impl TraitDefRegistry {
    fn new() -> Self {
        TraitDefRegistry {
            traits10: AsyncMap::new(),
            traits11: AsyncMap::new(),
            traits12f: AsyncMap::new(),
            traits12i: AsyncMap::new(),
            traits21: AsyncMap::new(),
            traits22: AsyncMap::new(),
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
    traits12i: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits12f: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits21: AsyncMap<(TraitKey, MultiVector, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits22: AsyncMap<(TraitKey, MultiVector, MultiVector), Option<Arc<RawTraitImplementation>>>,

    has_set_operators: Arc<Mutex<BTreeSet<Ops>>>,
    pub infix_trick: Arc<Mutex<Option<BinaryOps>>>,
}

impl TraitImplRegistry {
    pub fn new() -> Self {
        TraitImplRegistry {
            defs: TraitDefRegistry::new(),
            traits10: AsyncMap::new(),
            traits11: AsyncMap::new(),
            traits12i: AsyncMap::new(),
            traits12f: AsyncMap::new(),
            traits21: AsyncMap::new(),
            traits22: AsyncMap::new(),
            has_set_operators: Arc::new(Mutex::new(BTreeSet::new())),
            infix_trick: Arc::new(Mutex::new(None)),
        }
    }

    // TODO allow 12f and 12i as well
    pub fn set_binary_operator<TD: TraitDef_2_Types_2_Args, const AntiScalar: BasisElement>(&self, repo: Arc<MultiVecRepository<AntiScalar>>, op: BinaryOps, td: TD) {
        let mut set_ops = self.has_set_operators.lock();
        let op_key = op.rust_trait_name();
        let op = Ops::Binary(op);
        if set_ops.contains(&op) {
            panic!("There was an attempt to set an operator's trait more than once: {op:?}")
        }
        set_ops.insert(op);
        drop(set_ops);

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let tdr = self.defs.clone();
        let slf = self.clone();
        let multi_progress = Arc::new(MultiProgress::new());
        rt.block_on(async move {
            // Check if the trait has already been registered.
            // If it has, then the operator will overtly delegate to it.
            // If it hasn't, then we will inline the trait, so that it doesn't have to be declared separately.
            let orig_key = td.def().names.trait_key;
            let orig_td = tdr.traits22.get(&orig_key).await;
            let key = if orig_td.is_none() {
                let td = OvertDelegate::new(op_key, InlineOnly::new(orig_key.final_name, td));
                td.register(slf, repo, multi_progress).await;
                td.trait_names().trait_key
            } else {
                let td = OvertDelegate::new(op_key, td);
                td.register(slf, repo, multi_progress).await;
                td.trait_names().trait_key
            };
            let def = tdr.traits22.get(&key).await.expect("Created during registration");
            let mut the_op = def.op.lock();
            if the_op.is_some() {
                // Shouldn't really happen because of previous panic/check in this function
                panic!(
                    "Somehow an operator was set twice, we should try to prevent this \
                    earlier than when we enter async machinery. {op:?}"
                );
            }
            *the_op = Some(op);
        });
    }

    pub fn set_unary_operator<TD: TraitDef_1_Type_1_Arg, const AntiScalar: BasisElement>(&self, repo: Arc<MultiVecRepository<AntiScalar>>, op: UnaryOps, td: TD) {
        let mut set_ops = self.has_set_operators.lock();
        let op_key = op.rust_trait_name();
        let op = Ops::Unary(op);
        if set_ops.contains(&op) {
            panic!("There was an attempt to set an operator's trait more than once: {op:?}")
        }
        set_ops.insert(op);
        drop(set_ops);

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let tdr = self.defs.clone();
        let slf = self.clone();
        let multi_progress = Arc::new(MultiProgress::new());
        rt.block_on(async move {
            // Check if the trait has already been registered.
            // If it has, then the operator will overtly delegate to it.
            // If it hasn't, then we will inline the trait, so that it doesn't have to be declared separately.
            let orig_key = td.def().names.trait_key;
            let orig_td = tdr.traits11.get(&orig_key).await;
            let key = if orig_td.is_none() {
                let td = OvertDelegate::new(op_key, InlineOnly::new(orig_key.final_name, td));
                td.register(slf, repo, multi_progress).await;
                td.trait_names().trait_key
            } else {
                let td = OvertDelegate::new(op_key, td);
                td.register(slf, repo, multi_progress).await;
                td.trait_names().trait_key
            };
            let def = tdr.traits11.get(&key).await.expect("Created during registration");
            let mut the_op = def.op.lock();
            if the_op.is_some() {
                // Shouldn't really happen because of previous panic/check in this function
                panic!(
                    "Somehow an operator was set twice, we should try to prevent this \
                    earlier than when we enter async machinery. {op:?}"
                );
            }
            *the_op = Some(op);
        });
    }

    pub fn generate_infix_trick(&self, op: BinaryOps) {
        let mut trick = self.infix_trick.lock();
        if trick.is_some() {
            panic!(
                "Do not set the infix trick more than once. Just decide what you want it to be \
                once and leave it."
            )
        }
        *trick = Some(op);
    }

    pub(crate) async fn get_defs(&self) -> Vec<Arc<RawTraitDefinition>> {
        let mut v = vec![];
        for dep in self.defs.traits10.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits11.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits12f.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits12i.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits21.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits22.to_vec().await {
            v.push(dep);
        }
        v
    }

    pub(crate) async fn get_impls(&self) -> Vec<Arc<RawTraitImplementation>> {
        let mut v = vec![];
        for dep in self.traits10.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits11.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits12f.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits12i.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits21.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits22.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        v
    }

    pub fn finish(self) -> Arc<Self> {
        Arc::new(self)
    }
}

pub(crate) fn progress_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .expect("Template should be good or you gotta fix it")
        .progress_chars("#>-")
}

#[async_trait]
pub trait Register10: TraitDef_1_Type_0_Args {
    async fn register<const AntiScalar: BasisElement>(self, tr: TraitImplRegistry, mvs: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>);
}
#[async_trait]
impl<T: TraitDef_1_Type_0_Args> Register10 for T {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let pb = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
        pb.set_style(progress_style());
        let n = trait_key.as_upper_camel();
        pb.set_message(format!("AST - {n}"));

        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            let tir_2 = tir.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let def_2 = def.clone();
            let pb = pb.clone();
            js.spawn(async move {
                let mv_a = MultiVector::from(mv_a);
                let tir_3 = tir_2.clone();
                let def_3 = def_2.clone();
                let the_impl = tir_2
                    .traits10
                    .get_or_create_or_panic((trait_key, mv_a), async move {
                        let variables = Arc::new(Mutex::new(HashMap::new()));
                        let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_3, tir_3, false, variables, im::HashSet::new());
                        let result = self.general_implementation(b, mv_a.clone()).await;
                        match result {
                            None => None,
                            Some(result) => Some(result.into_trait10(mv_a)),
                        }
                    })
                    .await;
                pb.inc(1);
                let Some(the_impl) = the_impl else { return };
                let owner_type = ExpressionType::Class(mv_a.clone());
                let return_type = the_impl.return_expr.expression_type();
                TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
            });
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        pb.finish();
    }
}
#[async_trait]
pub trait Register11: TraitDef_1_Type_1_Arg {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>);
}
#[async_trait]
impl<T: TraitDef_1_Type_1_Arg> Register11 for T {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let pb = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
        pb.set_style(progress_style());
        let n = trait_key.as_upper_camel();
        pb.set_message(format!("AST - {n}"));

        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            let tir_2 = tir.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let def_2 = def.clone();
            let pb = pb.clone();
            js.spawn(async move {
                let mv_a = MultiVector::from(mv_a);
                let tir_3 = tir_2.clone();
                let def_3 = def_2.clone();
                let the_impl = tir_2
                    .traits11
                    .get_or_create_or_panic((trait_key, mv_a), async move {
                        let mut variables = HashMap::new();
                        let declare_self = param_self();
                        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                        let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new());
                        let var_self: Variable<MultiVector> = Variable {
                            expr_type: mv_a.clone(),
                            decl: declare_self,
                        };
                        let result = self.general_implementation(b, var_self).await;
                        match result {
                            None => None,
                            Some(result) => Some(result.into_trait11(mv_a)),
                        }
                    })
                    .await;
                pb.inc(1);
                let Some(the_impl) = the_impl else { return };
                let owner_type = ExpressionType::Class(mv_a.clone());
                let return_type = the_impl.return_expr.expression_type();
                TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
            });
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        pb.finish();
    }
}
#[async_trait]
pub trait Register21: TraitDef_2_Types_1_Arg {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>);
}
#[async_trait]
impl<T: TraitDef_2_Types_1_Arg> Register21 for T {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let qty = qty * qty;
        let pb = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
        pb.set_style(progress_style());
        let n = trait_key.as_upper_camel();
        pb.set_message(format!("AST - {n}"));
        pb.tick();

        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            for mv_b in mv_repo.all_classes() {
                let tir_2 = tir.clone();
                let ga_2 = ga.clone();
                let mv_repo_2 = mv_repo.clone();
                let def_2 = def.clone();
                let pb = pb.clone();
                js.spawn(async move {
                    let mv_a = MultiVector::from(mv_a);
                    let mv_b = MultiVector::from(mv_b);
                    let tir_3 = tir_2.clone();
                    let def_3 = def_2.clone();
                    let the_impl = tir_2
                        .traits21
                        .get_or_create_or_panic((trait_key, mv_a, mv_b), async move {
                            let mut variables = HashMap::new();
                            let declare_self = param_self();
                            variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                            let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new());
                            let var_self: Variable<MultiVector> = Variable {
                                expr_type: mv_a.clone(),
                                decl: declare_self,
                            };
                            let result = self.general_implementation(b, var_self, mv_b.clone()).await;
                            match result {
                                None => None,
                                Some(result) => Some(result.into_trait21(mv_a, mv_b)),
                            }
                        })
                        .await;
                    pb.inc(1);
                    let Some(the_impl) = the_impl else { return };
                    let owner_type = ExpressionType::Class(mv_a.clone());
                    let return_type = the_impl.return_expr.expression_type();
                    TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                    TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
                });
            }
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        pb.finish();
    }
}
#[async_trait]
pub trait Register22: TraitDef_2_Types_2_Args {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>);
}
#[async_trait]
impl<T: TraitDef_2_Types_2_Args> Register22 for T {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let qty = qty * qty;
        let pb = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
        pb.set_style(progress_style());
        let n = trait_key.as_upper_camel();
        pb.set_message(format!("AST - {n}"));

        let mut js = JoinSet::new();
        // TODO actually restrict by the domain on the TraitDefs
        for mv_a in mv_repo.all_classes() {
            for mv_b in mv_repo.all_classes() {
                let tir_2 = tir.clone();
                let ga_2 = ga.clone();
                let mv_repo_2 = mv_repo.clone();
                let def_2 = def.clone();
                let pb = pb.clone();
                js.spawn(async move {
                    let mv_a = MultiVector::from(mv_a);
                    let mv_b = MultiVector::from(mv_b);
                    let tir_3 = tir_2.clone();
                    let def_3 = def_2.clone();
                    let the_impl = tir_2
                        .traits22
                        .get_or_create_or_panic((trait_key, mv_a, mv_b), async move {
                            let mut variables = HashMap::new();
                            let declare_self = param_self();
                            variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                            let declare_other = param_other();
                            variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
                            let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new());
                            let var_self: Variable<MultiVector> = Variable {
                                expr_type: mv_a.clone(),
                                decl: declare_self,
                            };
                            let var_other: Variable<MultiVector> = Variable {
                                expr_type: mv_b.clone(),
                                decl: declare_other,
                            };
                            let result = self.general_implementation(b, var_self, var_other).await;
                            match result {
                                None => None,
                                Some(result) => Some(result.into_trait22(mv_a, mv_b)),
                            }
                        })
                        .await;
                    pb.inc(1);
                    let Some(the_impl) = the_impl else { return };
                    let owner_type = ExpressionType::Class(mv_a.clone());
                    let return_type = the_impl.return_expr.expression_type();
                    TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                    TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
                });
            }
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        pb.finish();
    }
}
#[async_trait]
pub trait Register12f: TraitDef_1_Type_2_Args_f32 {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>);
}
#[async_trait]
impl<T: TraitDef_1_Type_2_Args_f32> Register12f for T {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits12f.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let qty = qty * qty;
        let pb = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
        pb.set_style(progress_style());
        let n = trait_key.as_upper_camel();
        pb.set_message(format!("AST - {n}"));

        let mut js = JoinSet::new();
        // TODO actually restrict by the domain on the TraitDefs
        for mv_a in mv_repo.all_classes() {
            let tir_2 = tir.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let def_2 = def.clone();
            let pb = pb.clone();
            js.spawn(async move {
                let mv_a = MultiVector::from(mv_a);
                let tir_3 = tir_2.clone();
                let def_3 = def_2.clone();
                let the_impl = tir_2
                    .traits12f
                    .get_or_create_or_panic((trait_key, mv_a), async move {
                        let mut variables = HashMap::new();
                        let declare_self = param_self();
                        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                        let declare_other = param_other();
                        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
                        let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new());
                        let var_self: Variable<MultiVector> = Variable {
                            expr_type: mv_a.clone(),
                            decl: declare_self,
                        };
                        let var_other: Variable<Float> = Variable {
                            expr_type: Float,
                            decl: declare_other,
                        };
                        let result = self.general_implementation(b, var_self, var_other).await;
                        match result {
                            None => None,
                            Some(result) => Some(result.into_trait12f(mv_a)),
                        }
                    })
                    .await;
                pb.inc(1);
                let Some(the_impl) = the_impl else { return };
                let owner_type = ExpressionType::Class(mv_a.clone());
                let return_type = the_impl.return_expr.expression_type();
                TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
            });
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        pb.finish();
    }
}
#[async_trait]
pub trait Register12i: TraitDef_1_Type_2_Args_i32 {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>);
}
#[async_trait]
impl<T: TraitDef_1_Type_2_Args_i32> Register12i for T {
    async fn register<const AntiScalar: BasisElement>(self, tir: TraitImplRegistry, mv_repo: Arc<MultiVecRepository<AntiScalar>>, progress: Arc<indicatif::MultiProgress>) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits12i.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let qty = qty * qty;
        let pb = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
        pb.set_style(progress_style());
        let n = trait_key.as_upper_camel();
        pb.set_message(format!("AST - {n}"));

        let mut js = JoinSet::new();
        // TODO actually restrict by the domain on the TraitDefs
        for mv_a in mv_repo.all_classes() {
            let tir_2 = tir.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let def_2 = def.clone();
            let pb = pb.clone();
            js.spawn(async move {
                let mv_a = MultiVector::from(mv_a);
                let tir_3 = tir_2.clone();
                let def_3 = def_2.clone();
                let the_impl = tir_2
                    .traits12i
                    .get_or_create_or_panic((trait_key, mv_a), async move {
                        let mut variables = HashMap::new();
                        let declare_self = param_self();
                        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                        let declare_other = param_other();
                        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
                        let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new());
                        let var_self: Variable<MultiVector> = Variable {
                            expr_type: mv_a.clone(),
                            decl: declare_self,
                        };
                        let var_other: Variable<Integer> = Variable {
                            expr_type: Integer,
                            decl: declare_other,
                        };
                        let result = self.general_implementation(b, var_self, var_other).await;
                        match result {
                            None => None,
                            Some(result) => Some(result.into_trait12i(mv_a)),
                        }
                    })
                    .await;
                pb.inc(1);
                let Some(the_impl) = the_impl else { return };
                let owner_type = ExpressionType::Class(mv_a.clone());
                let return_type = the_impl.return_expr.expression_type();
                TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
            });
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        pb.finish();
    }
}

pub fn tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("Tokio should work")
}
pub fn tokio_joinset<T>() -> JoinSet<T> {
    JoinSet::new()
}
pub fn indicatif_multi_progress() -> Arc<indicatif::MultiProgress> {
    Arc::new(indicatif::MultiProgress::new())
}

#[macro_export]
macro_rules! register_all {
    ($mv_repo:expr; $($t:ident)+ $(| $($t2:ident)+)*) => {
        {
            use $crate::build_scripts::common_traits::*;
            let tir = $crate::ast::traits::TraitImplRegistry::new();
            use $crate::ast::traits::{Register10, Register11, Register21, Register22, Register12f, Register12i};
            let rt = $crate::ast::traits::tokio_rt();

            let multi_progress = $crate::ast::traits::indicatif_multi_progress();
            let _: () = rt.block_on(async {

                let mut js = $crate::ast::traits::tokio_joinset();
                $(
                let tir_c = tir.clone();
                let mv_repo_c = $mv_repo.clone();
                let mp = multi_progress.clone();
                js.spawn(async move {
                    $t.register(tir_c, mv_repo_c, mp).await;
                });
                )+
                while let Some(_) = js.join_next().await {}

                $(
                let mut js = $crate::ast::traits::tokio_joinset();
                $(
                let tir_c = tir.clone();
                let mv_repo_c = $mv_repo.clone();
                let mp = multi_progress.clone();
                js.spawn(async move {
                    $t2.register(tir_c, mv_repo_c, mp).await;
                });
                )+
                while let Some(_) = js.join_next().await {}
                )*
            });
            tir
        }
    };
}

#[macro_export]
macro_rules! operators {
    ($mv_repo:expr, $tir:ident $(; fancy_infix => $itr:ident)? $(; binary $($bop:ident => $btr:ident),+)? $(; unary $($uop:ident => $utr:ident),+ )? $(;)? ) => {
        {
            use $crate::build_scripts::common_traits::*;
            use $crate::ast::traits::BinaryOps::*;
            use $crate::ast::traits::UnaryOps::*;
            $($tir.generate_infix_trick($itr);)?
            $($(
                $tir.set_binary_operator($mv_repo.clone(), $bop, $btr);
            )+)?
            $($(
                $tir.set_unary_operator($mv_repo.clone(), $uop, $utr);
            )+)?
        }
    };
}

pub struct HasNotReturned;

pub(crate) fn param_self() -> Arc<RawVariableDeclaration> {
    Arc::new(RawVariableDeclaration {
        comment: None,
        name: ("self".to_string(), 0),
        expr: None,
    })
}
pub(crate) fn param_other() -> Arc<RawVariableDeclaration> {
    Arc::new(RawVariableDeclaration {
        comment: None,
        name: ("other".to_string(), 0),
        expr: None,
    })
}

#[derive(Clone)]
pub enum CommentOrVariableDeclaration {
    Comment(Cow<'static, String>),
    VarDec(Weak<RawVariableDeclaration>),
}

pub struct TraitImplBuilder<const AntiScalar: BasisElement, ReturnType> {
    pub ga: Arc<GeometricAlgebra<AntiScalar>>,
    pub mvs: Arc<MultiVecRepository<AntiScalar>>,
    registry: TraitImplRegistry,
    pub(crate) trait_def: Arc<RawTraitDefinition>,
    inline_dependencies: bool,
    pub(crate) is_deep_inlining: bool,
    specialized: bool,

    cycle_detector: im::HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
    pub(crate) multivector_dependencies: Mutex<HashSet<MultiVector>>,
    traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits12f_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits12i_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    wanted_multi_vecs: Mutex<HashSet<BTreeSet<BasisSignature>>>,

    variables: Arc<Mutex<HashMap<(String, usize), Weak<RawVariableDeclaration>>>>,
    lines: Mutex<Vec<CommentOrVariableDeclaration>>,
    return_comment: Option<String>,
    return_expr: Option<AnyExpression>,
    return_type: ReturnType,
}

impl<const AntiScalar: BasisElement> TraitImplBuilder<AntiScalar, HasNotReturned> {
    fn new(
        ga: Arc<GeometricAlgebra<AntiScalar>>,
        mvs: Arc<MultiVecRepository<AntiScalar>>,
        trait_def: Arc<RawTraitDefinition>,
        registry: TraitImplRegistry,
        inline_dependencies: bool,
        variables: Arc<Mutex<HashMap<(String, usize), Weak<RawVariableDeclaration>>>>,
        cycle_detector: im::HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
    ) -> Self {
        TraitImplBuilder {
            ga,
            mvs,
            registry,
            trait_def,
            inline_dependencies,
            is_deep_inlining: false,
            specialized: false,
            cycle_detector,
            multivector_dependencies: Default::default(),
            traits10_dependencies: Default::default(),
            traits11_dependencies: Default::default(),
            traits12f_dependencies: Default::default(),
            traits12i_dependencies: Default::default(),
            traits21_dependencies: Default::default(),
            traits22_dependencies: Default::default(),
            wanted_multi_vecs: Default::default(),
            variables,
            lines: Mutex::new(vec![]),
            return_comment: None,
            return_expr: None,
            return_type: HasNotReturned,
        }
    }

    // TODO put a wrapper type on this maybe... or rename it to be independent of trait building.
    pub fn new_sandbox(ga: Arc<GeometricAlgebra<AntiScalar>>, mvs: Arc<MultiVecRepository<AntiScalar>>) -> Self {
        let trait_def = Arc::new(RawTraitDefinition {
            documentation: "Sandbox".to_string(),
            names: TraitNames::just("Sandbox"),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Zero,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        });
        let inline_dependencies = true;
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let cycle_detector = im::HashSet::new();
        let registry = TraitImplRegistry::new();
        TraitImplBuilder {
            ga,
            mvs,
            registry,
            trait_def,
            inline_dependencies,
            is_deep_inlining: true,
            specialized: false,
            cycle_detector,
            multivector_dependencies: Default::default(),
            traits10_dependencies: Default::default(),
            traits11_dependencies: Default::default(),
            traits12i_dependencies: Default::default(),
            traits12f_dependencies: Default::default(),
            traits21_dependencies: Default::default(),
            traits22_dependencies: Default::default(),
            wanted_multi_vecs: Default::default(),
            variables,
            lines: Mutex::new(vec![]),
            return_comment: None,
            return_expr: None,
            return_type: HasNotReturned,
        }
    }

    pub fn mark_as_specialized_implementation(&mut self) {
        self.specialized = true;
    }

    fn make_var_name_unique(&self, var_name: String) -> (String, usize) {
        let mut key = (var_name.to_string(), 0);
        let vars = self.variables.lock();
        while vars.contains_key(&mut key) {
            key.1 += 1;
        }
        key
    }

    pub fn comment<C: Into<String>>(&mut self, comment: C) {
        self.lines.lock().push(CommentOrVariableDeclaration::Comment(Cow::Owned(comment.into())))
    }

    pub fn variable<V: Into<String>, ExprType, Expr: Expression<ExprType>>(&self, var_name: V, expr: Expr) -> Variable<ExprType> {
        self.comment_variable_impl(None::<String>, var_name, expr.expression_type(), expr.into_any_expression())
    }

    pub fn comment_variable<C: Into<String>, V: Into<String>, ExprType, Expr: Expression<ExprType>>(&mut self, comment: C, var_name: V, expr: Expr) -> Variable<ExprType> {
        self.comment_variable_impl(Some(comment), var_name, expr.expression_type(), expr.into_any_expression())
    }

    fn comment_variable_impl<C: Into<String>, V: Into<String>, ExprType>(&self, comment: Option<C>, var_name: V, expr_type: ExprType, expr: AnyExpression) -> Variable<ExprType> {
        match &expr {
            AnyExpression::Int(IntExpr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Float(FloatExpr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => return Variable { expr_type, decl: v.decl.clone() },
            _ => {}
        }

        let var_name = var_name.into();
        let unique_name = self.make_var_name_unique(var_name);
        let decl = Arc::new(RawVariableDeclaration {
            comment: comment.map(|it| Cow::Owned(it.into())),
            name: unique_name.clone(),
            expr: Some(Arc::new(RwLock::new(expr))),
        });
        let mut vars = self.variables.lock();
        let existing = vars.insert(unique_name.clone(), Arc::downgrade(&decl));
        assert!(existing.is_none(), "Variable {unique_name:?} is already taken");
        self.lines.lock().push(CommentOrVariableDeclaration::VarDec(Arc::downgrade(&decl)));
        Variable { expr_type, decl }
    }

    fn coerce_variable<V: Into<String>, ExprType, Expr: Expression<ExprType>>(&self, name_if_new_var: V, expr: Expr) -> Variable<ExprType> {
        return match expr.try_into_variable() {
            Some(already_done) => already_done,
            None => self.variable(name_if_new_var, expr),
        };
    }

    pub fn return_expr<ExprType, Expr: Expression<ExprType>>(self, expr: Expr) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        self.comment_return_impl(None::<String>, expr)
    }

    pub fn comment_return<C: Into<String>, ExprType, Expr: Expression<ExprType>>(self, comment: C, expr: Expr) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        self.comment_return_impl(Some(comment), expr)
    }

    pub fn construct(&self, dynamic_multi_vector: DynamicMultiVector) -> Option<MultiVectorExpr> {
        dynamic_multi_vector.construct(&self)
    }

    pub fn construct_exact(&self, dynamic_multi_vector: DynamicMultiVector) -> Option<MultiVectorExpr> {
        dynamic_multi_vector.construct_exact(&self)
    }

    pub(crate) fn note_wanted(&self, sig: BTreeSet<BasisSignature>) {
        self.wanted_multi_vecs.lock().insert(sig);
    }

    fn comment_return_impl<C: Into<String>, ExprType, Expr: Expression<ExprType>>(self, comment: Option<C>, expr: Expr) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        let return_type = expr.expression_type();
        return Some(TraitImplBuilder {
            ga: self.ga.clone(),
            mvs: self.mvs.clone(),
            registry: self.registry,
            // trait_def: self.trait_def,
            trait_def: self.trait_def,
            inline_dependencies: false,
            cycle_detector: self.cycle_detector,
            multivector_dependencies: self.multivector_dependencies,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            wanted_multi_vecs: self.wanted_multi_vecs,
            variables: self.variables,
            lines: self.lines,
            return_comment: comment.map(|it| it.into()),
            return_expr: Some(expr.into_any_expression()),
            return_type,
            specialized: self.specialized,
            is_deep_inlining: self.is_deep_inlining,
        });
    }

    fn inline_by_copy_existing_10<T: TraitDef_1_Type_0_Args + ?Sized>(&self, trait_key: &TraitKey, raw_impl: Arc<RawTraitImplementation>) -> Option<Variable<T::Output>> {
        let mut var_replacements: Vec<(Arc<RawVariableDeclaration>, Arc<RawVariableDeclaration>)> = vec![];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_11<T: TraitDef_1_Type_1_Arg + ?Sized, Expr: Expression<MultiVector>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let old_self = param_self();
        let mut var_replacements = vec![(old_self, new_self)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_21<T: TraitDef_2_Types_1_Arg + ?Sized, Expr: Expression<MultiVector>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let old_self = param_self();
        let mut var_replacements = vec![(old_self, new_self)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_22<T: TraitDef_2_Types_2_Args + ?Sized, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let new_other = self.coerce_variable("other", other).decl.clone();
        let old_self = param_self();
        let old_other = param_other();
        let mut var_replacements = vec![(old_self, new_self), (old_other, new_other)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_12i<T: TraitDef_1_Type_2_Args_i32 + ?Sized, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let new_other = self.coerce_variable("other", other).decl.clone();
        let old_self = param_self();
        let old_other = param_other();
        let mut var_replacements = vec![(old_self, new_self), (old_other, new_other)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_12f<T: TraitDef_1_Type_2_Args_f32 + ?Sized, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let new_other = self.coerce_variable("other", other).decl.clone();
        let old_self = param_self();
        let old_other = param_other();
        let mut var_replacements = vec![(old_self, new_self), (old_other, new_other)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_the_lines(&self, var_replacements: &mut Vec<(Arc<RawVariableDeclaration>, Arc<RawVariableDeclaration>)>, lines: &Vec<CommentOrVariableDeclaration>) {
        let mut our_lines = self.lines.lock();
        let their_lines = lines;
        for line in their_lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(c) => our_lines.push(CommentOrVariableDeclaration::Comment(c.clone())),
                CommentOrVariableDeclaration::VarDec(old_decl) => {
                    let Some(old_decl) = old_decl.upgrade() else { continue };
                    let new_var_comment = old_decl.comment.clone();
                    let new_var_name = self.make_var_name_unique(old_decl.name.0.clone());
                    let mut new_var_expr = old_decl.expr.as_ref().expect("Non-Parameter Variables are always initialized").read().clone();
                    for (old, new) in var_replacements.iter() {
                        // Update all variables used in this expression
                        new_var_expr.substitute_variable(old.clone(), new.clone());
                    }
                    // And create the new replacement declaration for this variable
                    let new_decl = Arc::new(RawVariableDeclaration {
                        comment: new_var_comment,
                        name: new_var_name,
                        expr: Some(Arc::new(RwLock::new(new_var_expr))),
                    });
                    // Then add it to the list
                    var_replacements.push((old_decl.clone(), new_decl.clone()));
                    // Then add it to lines
                    our_lines.push(CommentOrVariableDeclaration::VarDec(Arc::downgrade(&new_decl)))
                }
            }
        }
    }
}

// TODO DRY these into_traitXX methods

impl<const AntiScalar: BasisElement, ExprType> TraitImplBuilder<AntiScalar, ExprType> {
    fn into_trait10(self, owner: MultiVector) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits12i: &self.traits12i_dependencies,
            traits12f: &self.traits12f_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();

        // This shouldn't be a problem because of type level state and function visibilities
        let mut return_expr = self.return_expr.expect("Must have return expression in order to register");
        return_expr.final_simplify();
        statistics += return_expr.count_operations(&lookup);
        let maybe_variable = match &return_expr {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(v),
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(v),
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(v),
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(v),
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(v),
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => Some(v),
            _ => None,
        };
        if let Some(variable) = maybe_variable {
            let decl = &variable.decl;
            if 1 == Arc::strong_count(decl) && decl.expr.is_some() {
                if let Some(lock) = decl.expr.as_ref() {
                    let guard = lock.read();
                    let inlined_return_expr = guard.deref().clone();
                    drop(guard);
                    return_expr = inlined_return_expr;
                    return_expr.final_simplify();
                    statistics += return_expr.count_operations(&lookup);
                    // It is important that the Arc<RawVariableDeclaration> is dropped here,
                    // so it is not included in the 'lines' Vec a few lines of code down from here.
                }
            }
        }

        // Scan through the lines in reverse, drop unused variables, and count operations
        let mut lines = self.lines.into_inner();
        let mut i = lines.len();
        while i > 0 {
            i -= 1;
            let CommentOrVariableDeclaration::VarDec(vd) = &lines[i] else { continue };
            match vd.upgrade() {
                None => drop(lines.remove(i)),
                Some(vd) => {
                    if let Some(v) = &vd.expr {
                        let mut expr = v.write();
                        expr.final_simplify();
                        statistics += expr.count_operations(&lookup);
                    }
                }
            }
        }

        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: ExpressionType::Class(owner.clone()),
            owner_is_param: false,
            other_type_params: vec![],
            other_var_params: vec![],
            lines,
            return_comment: self.return_comment,
            return_expr,
            specialized: self.specialized,
            statistics,
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }

    fn into_trait11(self, owner: MultiVector) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits12i: &self.traits12i_dependencies,
            traits12f: &self.traits12f_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();

        // This shouldn't be a problem because of type level state and function visibilities
        let mut return_expr = self.return_expr.expect("Must have return expression in order to register");
        return_expr.final_simplify();
        statistics += return_expr.count_operations(&lookup);
        let maybe_variable = match &return_expr {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(v),
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(v),
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(v),
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(v),
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(v),
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => Some(v),
            _ => None,
        };
        if let Some(variable) = maybe_variable {
            // TODO it looks like single-use variables are not getting inlined perfectly,
            //  according to impl AntiSupport for Flector
            let decl = &variable.decl;
            if 1 == Arc::strong_count(decl) && decl.expr.is_some() {
                if let Some(lock) = decl.expr.as_ref() {
                    let guard = lock.read();
                    let inlined_return_expr = guard.deref().clone();
                    drop(guard);
                    return_expr = inlined_return_expr;
                    return_expr.final_simplify();
                    statistics += return_expr.count_operations(&lookup);
                    // It is important that the Arc<RawVariableDeclaration> is dropped here,
                    // so it is not included in the 'lines' Vec a few lines of code down from here.
                }
            }
        }

        // Scan through the lines in reverse, drop unused variables, and count operations
        let mut lines = self.lines.into_inner();
        let mut i = lines.len();
        while i > 0 {
            i -= 1;
            let CommentOrVariableDeclaration::VarDec(vd) = &lines[i] else { continue };
            match vd.upgrade() {
                None => drop(lines.remove(i)),
                Some(vd) => {
                    if let Some(v) = &vd.expr {
                        let mut expr = v.write();
                        expr.final_simplify();
                        statistics += expr.count_operations(&lookup);
                    }
                }
            }
        }

        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: ExpressionType::Class(owner.clone()),
            owner_is_param: true,
            other_type_params: vec![],
            other_var_params: vec![],
            lines,
            return_comment: self.return_comment,
            return_expr,
            specialized: self.specialized,
            statistics,
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }

    fn into_trait21(self, owner: MultiVector, other: MultiVector) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits12i: &self.traits12i_dependencies,
            traits12f: &self.traits12f_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();

        // This shouldn't be a problem because of type level state and function visibilities
        let mut return_expr = self.return_expr.expect("Must have return expression in order to register");
        return_expr.final_simplify();
        statistics += return_expr.count_operations(&lookup);
        let maybe_variable = match &return_expr {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(v),
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(v),
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(v),
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(v),
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(v),
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => Some(v),
            _ => None,
        };
        if let Some(variable) = maybe_variable {
            let decl = &variable.decl;
            if 1 == Arc::strong_count(decl) && decl.expr.is_some() {
                if let Some(lock) = decl.expr.as_ref() {
                    let guard = lock.read();
                    let inlined_return_expr = guard.deref().clone();
                    drop(guard);
                    return_expr = inlined_return_expr;
                    return_expr.final_simplify();
                    statistics += return_expr.count_operations(&lookup);
                    // It is important that the Arc<RawVariableDeclaration> is dropped here,
                    // so it is not included in the 'lines' Vec a few lines of code down from here.
                }
            }
        }

        // Scan through the lines in reverse, drop unused variables, and count operations
        let mut lines = self.lines.into_inner();
        let mut i = lines.len();
        while i > 0 {
            i -= 1;
            let CommentOrVariableDeclaration::VarDec(vd) = &lines[i] else { continue };
            match vd.upgrade() {
                None => drop(lines.remove(i)),
                Some(vd) => {
                    if let Some(v) = &vd.expr {
                        let mut expr = v.write();
                        expr.final_simplify();
                        statistics += expr.count_operations(&lookup);
                    }
                }
            }
        }

        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: ExpressionType::Class(owner.clone()),
            owner_is_param: true,
            other_type_params: vec![ExpressionType::Class(other)],
            other_var_params: vec![],
            lines,
            return_comment: self.return_comment,
            return_expr,
            specialized: self.specialized,
            statistics,
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }

    fn into_trait22(self, owner: MultiVector, other: MultiVector) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits12i: &self.traits12i_dependencies,
            traits12f: &self.traits12f_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();

        // This shouldn't be a problem because of type level state and function visibilities
        let mut return_expr = self.return_expr.expect("Must have return expression in order to register");
        return_expr.final_simplify();
        statistics += return_expr.count_operations(&lookup);
        let maybe_variable = match &return_expr {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(v),
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(v),
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(v),
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(v),
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(v),
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => Some(v),
            _ => None,
        };
        if let Some(variable) = maybe_variable {
            let decl = &variable.decl;
            if 1 == Arc::strong_count(decl) && decl.expr.is_some() {
                if let Some(lock) = decl.expr.as_ref() {
                    let guard = lock.read();
                    let inlined_return_expr = guard.deref().clone();
                    drop(guard);
                    return_expr = inlined_return_expr;
                    return_expr.final_simplify();
                    statistics += return_expr.count_operations(&lookup);
                    // It is important that the Arc<RawVariableDeclaration> is dropped here,
                    // so it is not included in the 'lines' Vec a few lines of code down from here.
                }
            }
        }

        // Scan through the lines in reverse, drop unused variables, and count operations
        let mut lines = self.lines.into_inner();
        let mut i = lines.len();
        while i > 0 {
            i -= 1;
            let CommentOrVariableDeclaration::VarDec(vd) = &lines[i] else { continue };
            match vd.upgrade() {
                None => drop(lines.remove(i)),
                Some(vd) => {
                    if let Some(v) = &vd.expr {
                        let mut expr = v.write();
                        expr.final_simplify();
                        statistics += expr.count_operations(&lookup);
                    }
                }
            }
        }

        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: ExpressionType::Class(owner.clone()),
            owner_is_param: true,
            other_type_params: vec![ExpressionType::Class(other.clone())],
            other_var_params: vec![ExpressionType::Class(other)],
            lines,
            return_comment: self.return_comment,
            return_expr,
            specialized: self.specialized,
            statistics,
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }

    fn into_trait12i(self, owner: MultiVector) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits12i: &self.traits12i_dependencies,
            traits12f: &self.traits12f_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();

        // This shouldn't be a problem because of type level state and function visibilities
        let mut return_expr = self.return_expr.expect("Must have return expression in order to register");
        return_expr.final_simplify();
        statistics += return_expr.count_operations(&lookup);
        let maybe_variable = match &return_expr {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(v),
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(v),
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(v),
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(v),
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(v),
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => Some(v),
            _ => None,
        };
        if let Some(variable) = maybe_variable {
            let decl = &variable.decl;
            if 1 == Arc::strong_count(decl) && decl.expr.is_some() {
                if let Some(lock) = decl.expr.as_ref() {
                    let guard = lock.read();
                    let inlined_return_expr = guard.deref().clone();
                    drop(guard);
                    return_expr = inlined_return_expr;
                    return_expr.final_simplify();
                    statistics += return_expr.count_operations(&lookup);
                    // It is important that the Arc<RawVariableDeclaration> is dropped here,
                    // so it is not included in the 'lines' Vec a few lines of code down from here.
                }
            }
        }

        // Scan through the lines in reverse, drop unused variables, and count operations
        let mut lines = self.lines.into_inner();
        let mut i = lines.len();
        while i > 0 {
            i -= 1;
            let CommentOrVariableDeclaration::VarDec(vd) = &lines[i] else { continue };
            match vd.upgrade() {
                None => drop(lines.remove(i)),
                Some(vd) => {
                    if let Some(v) = &vd.expr {
                        let mut expr = v.write();
                        expr.final_simplify();
                        statistics += expr.count_operations(&lookup);
                    }
                }
            }
        }

        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: ExpressionType::Class(owner.clone()),
            owner_is_param: true,
            other_type_params: vec![],
            other_var_params: vec![ExpressionType::Int(Integer)],
            lines,
            return_comment: self.return_comment,
            return_expr,
            specialized: self.specialized,
            statistics,
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }

    fn into_trait12f(self, owner: MultiVector) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits12i: &self.traits12i_dependencies,
            traits12f: &self.traits12f_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();

        // This shouldn't be a problem because of type level state and function visibilities
        let mut return_expr = self.return_expr.expect("Must have return expression in order to register");
        return_expr.final_simplify();
        statistics += return_expr.count_operations(&lookup);
        let maybe_variable = match &return_expr {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(v),
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(v),
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(v),
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(v),
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(v),
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => Some(v),
            _ => None,
        };
        if let Some(variable) = maybe_variable {
            let decl = &variable.decl;
            if 1 == Arc::strong_count(decl) && decl.expr.is_some() {
                if let Some(lock) = decl.expr.as_ref() {
                    let guard = lock.read();
                    let inlined_return_expr = guard.deref().clone();
                    drop(guard);
                    return_expr = inlined_return_expr;
                    return_expr.final_simplify();
                    statistics += return_expr.count_operations(&lookup);
                    // It is important that the Arc<RawVariableDeclaration> is dropped here,
                    // so it is not included in the 'lines' Vec a few lines of code down from here.
                }
            }
        }

        // Scan through the lines in reverse, drop unused variables, and count operations
        let mut lines = self.lines.into_inner();
        let mut i = lines.len();
        while i > 0 {
            i -= 1;
            let CommentOrVariableDeclaration::VarDec(vd) = &lines[i] else { continue };
            match vd.upgrade() {
                None => drop(lines.remove(i)),
                Some(vd) => {
                    if let Some(v) = &vd.expr {
                        let mut expr = v.write();
                        expr.final_simplify();
                        statistics += expr.count_operations(&lookup);
                    }
                }
            }
        }

        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            owner: ExpressionType::Class(owner.clone()),
            owner_is_param: true,
            other_type_params: vec![],
            other_var_params: vec![ExpressionType::Float(Float)],
            lines,
            return_comment: self.return_comment,
            return_expr,
            specialized: self.specialized,
            statistics,
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }
}

impl<const AntiScalar: BasisElement, ExprType: TraitResultType> TraitImplBuilder<AntiScalar, ExprType> {
    fn finish_inline<V: Into<String>>(self, b: &TraitImplBuilder<AntiScalar, HasNotReturned>, var_name: V) -> Option<Variable<ExprType>> {
        let mut outer_lines = b.lines.lock();
        let inner_lines = self.lines.into_inner();
        for line in inner_lines.into_iter() {
            outer_lines.push(line);
        }

        // Don't deadlock in next method
        drop(outer_lines);

        // If there was no return expression provided, assume the implementation "failed"
        // under normal circumstances like some combination of classes that doesn't produce a result
        let var = b.comment_variable_impl(self.return_comment, var_name, self.return_type, self.return_expr?);
        Some(var)
    }

    fn finish_deep_inline(self) -> Option<<ExprType as TraitResultType>::Expr> {
        let mut r = self.return_expr?;
        r.deep_inline_variables();
        ExprType::select_expr(r)
    }
}

#[macro_export]
macro_rules! variants {
    (
        $declarations:ident;
        $(
        $(#docs($docs:expr))?
        $($prefix:ident)? {type} $($suffix:ident)? => ($el_filter:expr) where $mv_in:expr => $mv_out:expr
        );+
        $(;)?
    ) => {
        {
            $(
            let mut doc: std::option::Option<&'static str> = None;
            $(doc = Some($docs);)?
            $declarations.variants(
                stringify!($($prefix)?),
                stringify!($($suffix)?),
                $mv_in, $el_filter, $mv_out,
                doc,
            );
            )+
        }
    };
    (
        $declarations:ident;
        $(
        $(#docs($docs:expr))?
        $($prefix:ident)? {Vector} $($suffix:ident)? => ($el_filter:expr) where $mv_in:expr => $mv_out:expr
        );+
        $(;)?
    ) => {
        {
            $(
            let mut doc: std::option::Option<&'static str> = None;
            $(doc = Some($docs);)?
            $declarations.variants(
                stringify!($($prefix)?),
                stringify!($($suffix)?),
                $mv_in, $el_filter, $mv_out,
                doc,
            );
            )+
        }
    };
}
