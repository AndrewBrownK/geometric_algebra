use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use async_trait::async_trait;
use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use regex::Regex;
use tokio::task::JoinSet;

use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::GeometricAlgebra;
use crate::algebra2::multivector::{DynamicMultiVector, MultiVecRepository};
use crate::ast2::{RawVariableDeclaration, Variable};
use crate::ast2::datatype::{ClassesFromRegistry, ExpressionType, MultiVector};
use crate::ast2::expressions::{AnyExpression, Expression, extract_multivector_expr, MultiVectorExpr, TraitResultType};
use crate::ast2::impls::Elaborated;
use crate::ast2::operations_tracker::{TrackOperations, TraitOperationsLookup, VectoredOperationsTracker};
use crate::utility::AsyncMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraitTypeConsensus {
    NoVotes,
    AllAgree(ExpressionType, bool),
    AlwaysSelf,
    Disagreement
}
impl TraitTypeConsensus {
    pub fn add_vote(slf: &Arc<RwLock<TraitTypeConsensus>>, expr_type: ExpressionType, is_self: bool) {
        let output = slf.read();
        match output.deref() {
            TraitTypeConsensus::Disagreement => return,
            TraitTypeConsensus::AlwaysSelf if is_self => return,
            TraitTypeConsensus::AllAgree(agreed, was_self)
                if *agreed == expr_type && is_self == *was_self => return,
            _ => {}
        }
        drop(output);
        let mut owners = slf.write();
        *owners = match owners.deref() {
            TraitTypeConsensus::Disagreement => return,
            TraitTypeConsensus::AlwaysSelf if is_self => return,
            TraitTypeConsensus::AlwaysSelf => TraitTypeConsensus::Disagreement,
            TraitTypeConsensus::NoVotes => TraitTypeConsensus::AllAgree(expr_type, is_self),
            TraitTypeConsensus::AllAgree(agreed_type, was_self) => {
                match (*agreed_type == expr_type, *was_self && is_self) {
                    (true, true) => return,
                    (true, false) => TraitTypeConsensus::AllAgree(expr_type, false),
                    (false, true) => TraitTypeConsensus::AlwaysSelf,
                    (false, false) => TraitTypeConsensus::Disagreement,
                }
            },
        }
    }
}

pub type TraitParam = ExpressionType;

#[derive(Clone, Copy)]
pub enum TraitArity {
    Zero, 
    One, 
    Two
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

pub(crate) struct RawTraitDefinition {
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
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}


#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_0Param: TraitImpl_10 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String { String::new() }
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
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
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

        let slf = self.clone();
        let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner.clone());
        let owner_clone = owner.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let ga = b.ga.clone();
        let mvs = b.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let fresh_variable_scope = Arc::new(Mutex::new(HashMap::new()));
            let builder = TraitImplBuilder::new(
                ga, mvs, the_def_clone, registry, false, fresh_variable_scope, cycle_detector_clone
            );
            let trait_impl = t_self.general_implementation(builder, owner_clone.clone()).await?;
            Some(trait_impl.into_trait10(owner_clone))
        };
        let the_impl = b.registry.traits10.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        the_def.dependencies.lock().insert(trait_key);

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

    async fn inline<const AntiScalar: BasisElement>(
        &self,
        b: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.clone());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = b.registry.traits10.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return b.inline_by_copy_existing_10::<Self>(&trait_key, raw_impl)
            }
        }

        let slf = self.clone();
        let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let builder = TraitImplBuilder::new(
            b.ga.clone(), b.mvs.clone(), the_def, b.registry.clone(), b.inline_dependencies, b.variables.clone(), b.cycle_detector.clone()
        );
        let trait_impl = self.general_implementation(builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(b, var_name)
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_11: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1Class_1Param: TraitImpl_11 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String { String::new() }
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
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>, owner: Expr
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

        let slf = self.clone();
        let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let ga = b.ga.clone();
        let mvs = b.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(declare_self.clone());
            let builder = TraitImplBuilder::new(
                ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone
            );
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = t_self.general_implementation(builder, var_self).await?;
            Some(trait_impl.into_trait11(owner_class_clone))
        };
        let the_impl = b.registry.traits11.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        the_def.dependencies.lock().insert(trait_key);

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
    async fn inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        b: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.strong_expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = b.registry.traits11.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return b.inline_by_copy_existing_11::<Self, _>(&trait_key, raw_impl, owner)
            }
        }

        let slf = self.clone();
        let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let mut builder = TraitImplBuilder::new(
            b.ga.clone(), b.mvs.clone(), the_def, b.registry.clone(), b.inline_dependencies, b.variables.clone(), b.cycle_detector.clone()
        );
        let owner = builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(b, var_name)
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_21: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_1Param: TraitImpl_21 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    fn general_documentation(&self) -> String { String::new() }
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
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
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

        let slf = self.clone();
        let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let ga = b.ga.clone();
        let mvs = b.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(declare_self.clone());
            let builder = TraitImplBuilder::new(
                ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone
            );
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, other_class_clone.clone()).await?;
            Some(trait_impl.into_trait21(owner_class_clone, other_class_clone.clone()))
        };
        let the_impl = b.registry.traits21.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        the_def.dependencies.lock().insert(trait_key);

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

    async fn inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        b: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.strong_expression_type(), other.clone());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = b.registry.traits21.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return b.inline_by_copy_existing_21::<Self, _>(&trait_key, raw_impl, owner)
            }
        }

        let slf = self.clone();
        let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let mut builder = TraitImplBuilder::new(
            b.ga.clone(), b.mvs.clone(), the_def, b.registry.clone(), b.inline_dependencies, b.variables.clone(), b.cycle_detector.clone()
        );
        let owner = builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(b, var_name)
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_22: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        b: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2Class_2Param: TraitImpl_22 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    fn general_documentation(&self) -> String { String::new() }
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
        b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
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

        let slf = self.clone();
        let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let registry = b.registry.clone();
        let cycle_detector_clone = b.cycle_detector.clone();
        let ga = b.ga.clone();
        let mvs = b.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(declare_self.clone());
            let declare_other = param_other();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(declare_other.clone());
            let builder = TraitImplBuilder::new(
                ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone
            );
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
        let the_impl = b.registry.traits22.get_or_create_or_panic(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        the_def.dependencies.lock().insert(trait_key);

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

    async fn inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        b: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.strong_expression_type(), other.strong_expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(raw_impl)) = b.registry.traits22.get(&impl_key).await {
            // It is faster to inline here, rather than copying existing impls,
            // because variable substitution involves copying and mutating an entire AST.
            // So the only time we want to copy an AST is if it is specialized.
            if raw_impl.specialized {
                return b.inline_by_copy_existing_22::<Self, _, _>(&trait_key, raw_impl, owner, other)
            }
        }

        let slf = self.clone();
        let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let mut builder = TraitImplBuilder::new(
            b.ga.clone(), b.mvs.clone(), the_def, b.registry.clone(), b.inline_dependencies, b.variables.clone(), b.cycle_detector.clone()
        );
        let owner = builder.coerce_variable("self", owner);
        let other = builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(b, var_name)
    }

    // TODO "strong_inline" function that expands all expressions, even when it would result
    //  in redundant calculations. It will inline and expand every variable invocation to its
    //  declaration, unless/until it is a feat variable with no declaration expression.
    //  This is useful for playground activity where defining a mathematical set of varaibles
    //  and fully expanding the mathematical interactions.
}


#[marker]
pub trait CanNameTrait {}
impl<I> CanNameTrait for I where I: TraitImpl_10 {}
impl<I> CanNameTrait for I where I: TraitImpl_11 {}
impl<I> CanNameTrait for I where I: TraitImpl_21 {}
impl<I> CanNameTrait for I where I: TraitImpl_22 {}


#[const_trait]
pub trait NameTrait: Sized + Copy {
    fn new_trait_named(self, name: &'static str) -> Elaborated<Self>;
}
impl<I> const NameTrait for I where I: CanNameTrait + Copy {
    fn new_trait_named(self, name: &'static str) -> Elaborated<Self> {
        Elaborated::new_with_name(self, name)
    }
}


pub(crate) struct RawTraitImplementation {
    pub(crate) definition: Arc<RawTraitDefinition>,
    pub(crate) multivector_dependencies: HashSet<MultiVector>,
    pub(crate) traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    pub(crate) traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
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
        Self {
            final_name: name,
        }
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
    pub output_in_shaders: bool
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
            output_in_shaders: shaders
        }
    }
}


#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum UnaryOps {
    Neg,
    Not,
}
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum BinaryOps {
    // TODO should not allow overriding Add and Sub... ...which also means Neg...
    //  ah whatever... maybe to just allow full freedom, but create an easy method to
    //  activate sensible defaults.
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
            BinaryOps::BitAnd => "bit_and",
            BinaryOps::BitOr => "bit_or",
            BinaryOps::BitXor => "bit_xor",
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
    pub const fn into_u32(self) -> u32 {
        match self {
            Ops::Unary(o) => {
                o as u32
            }
            Ops::Binary(o) => {
                (o as u32) + 2
            }
        }
    }
}

#[derive(Clone)]
pub struct TraitDefRegistry {
    traits10: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits11: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits21: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits22: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
}
impl TraitDefRegistry {
    fn new() -> Self {
        TraitDefRegistry {
            traits10: AsyncMap::new(),
            traits11: AsyncMap::new(),
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
            traits21: AsyncMap::new(),
            traits22: AsyncMap::new(),
            has_set_operators: Arc::new(Mutex::new(BTreeSet::new())),
            infix_trick: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_binary_operator<TD: TraitDef_2Class_2Param>(&self, op: BinaryOps, td: TD) {
        let mut set_ops = self.has_set_operators.lock();
        let op = Ops::Binary(op);
        if set_ops.contains(&op) {
            panic!("There was an attempt to set an operator's trait more than once: {op:?}")
        }
        set_ops.insert(op);
        drop(set_ops);

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let tdr = self.defs.clone();
        rt.block_on(async move {
            let def = td.def();
            let key = def.names.trait_key;
            let def = tdr.traits22.get_or_create_or_panic(key, async move { def }).await;
            let mut the_op = def.op.lock();
            if the_op.is_some() {
                // Shouldn't really happen because of previous panic/check in this function
                panic!("Somehow an operator was set twice, we should try to prevent this \
                    earlier than when we enter async machinery. {op:?}");
            }
            *the_op = Some(op);
        });
    }

    pub fn set_unary_operator<TD: TraitDef_1Class_1Param>(&self, op: UnaryOps, td: TD) {
        let mut set_ops = self.has_set_operators.lock();
        let op = Ops::Unary(op);
        if set_ops.contains(&op) {
            panic!("There was an attempt to set an operator's trait more than once: {op:?}")
        }
        set_ops.insert(op);
        drop(set_ops);

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let tdr = self.defs.clone();
        rt.block_on(async move {
            let def = td.def();
            let key = def.names.trait_key;
            let def = tdr.traits11.get_or_create_or_panic(key, async move { def }).await;
            let mut the_op = def.op.lock();
            if the_op.is_some() {
                // Shouldn't really happen because of previous panic/check in this function
                panic!("Somehow an operator was set twice, we should try to prevent this \
                    earlier than when we enter async machinery. {op:?}");
            }
            *the_op = Some(op);
        });
    }

    pub fn generate_infix_trick(&self, op: BinaryOps) {
        let mut trick = self.infix_trick.lock();
        if trick.is_some() {
            panic!("Do not set the infix trick more than once. Just decide what you want it to be \
                once and leave it.")
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
}


#[async_trait]
pub trait Register10: TraitDef_1Class_0Param {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tr: TraitImplRegistry,
        mvs: Arc<MultiVecRepository<AntiScalar>>,
    );
}
#[async_trait]
impl<T: TraitDef_1Class_0Param> Register10 for T {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
    ) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            let tir_2 = tir.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let def_2 = def.clone();
            js.spawn(async move {
                let mv_a = MultiVector::from(mv_a);
                let tir_3 = tir_2.clone();
                let def_3 = def_2.clone();
                let the_impl = tir_2.traits10.get_or_create_or_panic((trait_key, mv_a), async move {
                    let mut variables = Arc::new(Mutex::new(HashMap::new()));
                    let b = TraitImplBuilder::new(
                        ga_2, mv_repo_2, def_3, tir_3, false, variables, im::HashSet::new()
                    );
                    let result = self.general_implementation(b, mv_a.clone()).await;
                    match result {
                        None => None,
                        Some(result) => Some(result.into_trait10(mv_a)),
                    }
                }).await;
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
    }
}
#[async_trait]
pub trait Register11: TraitDef_1Class_1Param {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
    );
}
#[async_trait]
impl<T: TraitDef_1Class_1Param> Register11 for T {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
    ) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            let tir_2 = tir.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let def_2 = def.clone();
            js.spawn(async move {
                let mv_a = MultiVector::from(mv_a);
                let tir_3 = tir_2.clone();
                let def_3 = def_2.clone();
                let the_impl = tir_2.traits11.get_or_create_or_panic((trait_key, mv_a), async move {
                    let mut variables = HashMap::new();
                    let declare_self = param_self();
                    variables.entry(declare_self.name.clone()).or_insert(declare_self.clone());
                    let b = TraitImplBuilder::new(
                        ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new()
                    );
                    let var_self: Variable<MultiVector> = Variable {
                        expr_type: mv_a.clone(),
                        decl: declare_self,
                    };
                    let result = self.general_implementation(b, var_self).await;
                    match result {
                        None => None,
                        Some(result) => Some(result.into_trait11(mv_a)),
                    }
                }).await;
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
    }
}
#[async_trait]
pub trait Register21: TraitDef_2Class_1Param {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
    );
}
#[async_trait]
impl<T: TraitDef_2Class_1Param> Register21 for T {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
    ) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            for mv_b in mv_repo.all_classes() {
                let tir_2 = tir.clone();
                let ga_2 = ga.clone();
                let mv_repo_2 = mv_repo.clone();
                let def_2 = def.clone();
                js.spawn(async move {
                    let mv_a = MultiVector::from(mv_a);
                    let mv_b = MultiVector::from(mv_b);
                    let tir_3 = tir_2.clone();
                    let def_3 = def_2.clone();
                    let the_impl = tir_2.traits21.get_or_create_or_panic((trait_key, mv_a, mv_b), async move {
                        let mut variables = HashMap::new();
                        let declare_self = param_self();
                        variables.entry(declare_self.name.clone()).or_insert(declare_self.clone());
                        let b = TraitImplBuilder::new(
                            ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new()
                        );
                        let var_self: Variable<MultiVector> = Variable {
                            expr_type: mv_a.clone(),
                            decl: declare_self,
                        };
                        let result = self.general_implementation(b, var_self, mv_b.clone()).await;
                        match result {
                            None => None,
                            Some(result) => Some(result.into_trait21(mv_a, mv_b)),
                        }
                    }).await;
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
    }
}
#[async_trait]
pub trait Register22: TraitDef_2Class_2Param {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
    );
}
#[async_trait]
impl<T: TraitDef_2Class_2Param> Register22 for T {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
    ) {
        let ga = mv_repo.ga();
        let trait_key = self.trait_names().trait_key;
        let def = tir.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { self.def() }).await;

        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            for mv_b in mv_repo.all_classes() {
                let tir_2 = tir.clone();
                let ga_2 = ga.clone();
                let mv_repo_2 = mv_repo.clone();
                let def_2 = def.clone();
                js.spawn(async move {
                    let mv_a = MultiVector::from(mv_a);
                    let mv_b = MultiVector::from(mv_b);
                    let tir_3 = tir_2.clone();
                    let def_3 = def_2.clone();
                    let the_impl = tir_2.traits22.get_or_create_or_panic((trait_key, mv_a, mv_b), async move {
                        let mut variables = HashMap::new();
                        let declare_self = param_self();
                        variables.entry(declare_self.name.clone()).or_insert(declare_self.clone());
                        let declare_other = param_other();
                        variables.entry(declare_self.name.clone()).or_insert(declare_other.clone());
                        let b = TraitImplBuilder::new(
                            ga_2, mv_repo_2, def_3, tir_3, false, Arc::new(Mutex::new(variables)), im::HashSet::new()
                        );
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
                    }).await;
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
    }
}

#[macro_export]
macro_rules! register_all {
    ($mv_repo:expr; $($t:ident)+ $(| $($t2:ident)+)*) => {
        {
            use $crate::build_scripts2::common_traits::*;
            let tir = $crate::ast2::traits::TraitImplRegistry::new();
            use $crate::ast2::traits::{Register10, Register11, Register21, Register22};
            let rt = tokio::runtime::Runtime::new().expect("Tokio should work");
            let _: () = rt.block_on(async {
                let mut js = tokio::task::JoinSet::new();
                $(
                let tir_c = tir.clone();
                let mv_repo_c = $mv_repo.clone();
                js.spawn(async move {
                    $t.register(tir_c, mv_repo_c).await;
                });
                )+
                while let Some(_) = js.join_next().await {}

                $(
                let mut js = tokio::task::JoinSet::new();
                $(
                let tir_c = tir.clone();
                let mv_repo_c = $mv_repo.clone();
                js.spawn(async move {
                    $t2.register(tir_c, mv_repo_c).await;
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
    ($tir:ident $(; infix => $itr:ident)? $(; binary $($bop:ident => $btr:ident),+)? $(; unary $($uop:ident => $utr:ident),+ )? $(;)? ) => {
        {
            use $crate::build_scripts2::common_traits::*;
            use $crate::ast2::traits::BinaryOps::*;
            use $crate::ast2::traits::UnaryOps::*;
            $($tir.generate_infix_trick($itr);)?
            $($(
                $tir.set_binary_operator($bop, $btr);
            )+)?
            $($(
                $tir.set_unary_operator($uop, $utr);
            )+)?
        }
    };
}


pub struct HasNotReturned;

fn param_self() -> Arc<RawVariableDeclaration> {
    Arc::new(RawVariableDeclaration {
        comment: None,
        name: ("self".to_string(), 0),
        expr: None,
    })
}
fn param_other() -> Arc<RawVariableDeclaration> {
    Arc::new(RawVariableDeclaration {
        comment: None,
        name: ("other".to_string(), 0),
        expr: None,
    })
}

#[derive(Clone)]
pub enum CommentOrVariableDeclaration {
    Comment(Cow<'static, String>),
    VarDec(Arc<RawVariableDeclaration>)
}

pub struct TraitImplBuilder<const AntiScalar: BasisElement, ReturnType> {
    pub ga: Arc<GeometricAlgebra<AntiScalar>>,
    pub mvs: Arc<MultiVecRepository<AntiScalar>>,
    registry: TraitImplRegistry,
    pub(crate) trait_def: Arc<RawTraitDefinition>,
    inline_dependencies: bool,
    specialized: bool,

    cycle_detector: im::HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
    pub(crate) multivector_dependencies: Mutex<HashSet<MultiVector>>,
    traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    wanted_multi_vecs: Mutex<HashSet<BTreeSet<BasisSignature>>>,

    variables: Arc<Mutex<HashMap<(String, usize), Arc<RawVariableDeclaration>>>>,
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
        variables: Arc<Mutex<HashMap<(String, usize), Arc<RawVariableDeclaration>>>>,
        cycle_detector: im::HashSet<(TraitKey, MultiVector, Option<MultiVector>)>,
    ) -> Self {
        TraitImplBuilder {
            ga,
            mvs,
            registry,
            trait_def,
            inline_dependencies,
            specialized: false,
            cycle_detector,
            multivector_dependencies: Default::default(),
            traits10_dependencies: Default::default(),
            traits11_dependencies: Default::default(),
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
    pub(crate) fn new_sandbox(
        ga: Arc<GeometricAlgebra<AntiScalar>>,
        mvs: Arc<MultiVecRepository<AntiScalar>>,
    ) -> Self {
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
            specialized: false,
            cycle_detector,
            multivector_dependencies: Default::default(),
            traits10_dependencies: Default::default(),
            traits11_dependencies: Default::default(),
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

    pub fn variable<
        V: Into<String>,
        ExprType,
        Expr: Expression<ExprType>
    >(
        &self,
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
        &self,
        comment: Option<C>,
        var_name: V,
        expr_type: ExprType,
        expr: AnyExpression
    ) -> Variable<ExprType> {
        let var_name = var_name.into();
        let unique_name = self.make_var_name_unique(var_name);
        let decl = Arc::new(RawVariableDeclaration {
            comment: comment.map(|it| Cow::Owned(it.into())),
            name: unique_name.clone(),
            expr: Some(expr),
        });
        let mut vars = self.variables.lock();
        let existing = vars.insert(unique_name.clone(), decl.clone());
        assert!(existing.is_none(), "Variable {unique_name:?} is already taken");
        self.lines.lock().push(CommentOrVariableDeclaration::VarDec(decl.clone()));
        Variable { expr_type, decl, }
    }

    fn coerce_variable<V: Into<String>, ExprType, Expr: Expression<ExprType>>(&self, name_if_new_var: V, expr: Expr) -> Variable<ExprType> {
        return match expr.try_into_variable() {
            Some(already_done) => already_done,
            None => self.variable(name_if_new_var, expr),
        }
    }

    pub fn return_expr<ExprType, Expr: Expression<ExprType>>(
        self, expr: Expr
    ) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        self.comment_return_impl(None::<String>, expr)
    }

    pub fn comment_return<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: C, expr: Expr
    ) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
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

    fn comment_return_impl<C: Into<String>, ExprType, Expr: Expression<ExprType>>(
        self, comment: Option<C>, expr: Expr
    ) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        let return_type = expr.strong_expression_type();
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
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            wanted_multi_vecs: self.wanted_multi_vecs,
            variables: self.variables,
            lines: self.lines,
            return_comment: comment.map(|it| it.into()),
            return_expr: Some(expr.into_any_expression()),
            return_type,
            specialized: self.specialized,
        })
    }

    fn inline_by_copy_existing_10<
        T: TraitDef_1Class_0Param + ?Sized,
    >(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
    ) -> Option<Variable<T::Output>> {
        let mut var_replacements: Vec<(Arc<RawVariableDeclaration>, Arc<RawVariableDeclaration>)> = vec![];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(
            raw_impl.return_comment.clone(),
            trait_key.as_lower_snake(),
            return_expr_type,
            return_expr
        );
        Some(var)
    }

    fn inline_by_copy_existing_11<
        T: TraitDef_1Class_1Param + ?Sized,
        Expr: Expression<MultiVector>
    >(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let old_self =  param_self();
        let mut var_replacements = vec![(old_self, new_self)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(
            raw_impl.return_comment.clone(),
            trait_key.as_lower_snake(),
            return_expr_type,
            return_expr
        );
        Some(var)
    }

    fn inline_by_copy_existing_21<
        T: TraitDef_2Class_1Param + ?Sized,
        Expr: Expression<MultiVector>
    >(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let old_self =  param_self();
        let mut var_replacements = vec![(old_self, new_self)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(
            raw_impl.return_comment.clone(),
            trait_key.as_lower_snake(),
            return_expr_type,
            return_expr
        );
        Some(var)
    }

    fn inline_by_copy_existing_22<
        T: TraitDef_2Class_2Param + ?Sized,
        Expr1: Expression<MultiVector>,
        Expr2: Expression<MultiVector>,
    >(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let new_other = self.coerce_variable("other", other).decl.clone();
        let old_self =  param_self();
        let old_other =  param_other();
        let mut var_replacements = vec![(old_self, new_self), (old_other, new_other)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(
            raw_impl.return_comment.clone(),
            trait_key.as_lower_snake(),
            return_expr_type,
            return_expr
        );
        Some(var)
    }

    fn inline_the_lines(
        &self,
        var_replacements: &mut Vec<(Arc<RawVariableDeclaration>, Arc<RawVariableDeclaration>)>,
        lines: &Vec<CommentOrVariableDeclaration>,
    ) {
        let mut our_lines = self.lines.lock();
        let their_lines = lines;
        for line in their_lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(c) => {
                    our_lines.push(CommentOrVariableDeclaration::Comment(c.clone()))
                }
                CommentOrVariableDeclaration::VarDec(old_decl) => {
                    let new_var_comment = old_decl.comment.clone();
                    let new_var_name = self.make_var_name_unique(old_decl.name.0.clone());
                    let mut new_var_expr = old_decl.expr.clone()
                        .expect("Non-Parameter Variables are always initialized");
                    for (old, new) in var_replacements.iter() {
                        // Update all variables used in this expression
                        new_var_expr.substitute_variable(old.clone(), new.clone());
                    }
                    // And create the new replacement declaration for this variable
                    let new_decl = Arc::new(RawVariableDeclaration {
                        comment: new_var_comment,
                        name: new_var_name,
                        expr: Some(new_var_expr),
                    });
                    // Then add it to the list
                    var_replacements.push((old_decl.clone(), new_decl.clone()));
                    // Then add it to lines
                    our_lines.push(CommentOrVariableDeclaration::VarDec(new_decl))
                }
            }
        }
    }
}


impl<const AntiScalar: BasisElement, ExprType> TraitImplBuilder<AntiScalar, ExprType> {
    fn into_trait10(
        self, owner: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();
        let lines = self.lines.into_inner();
        for line in lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(_) => {}
                CommentOrVariableDeclaration::VarDec(vd) => {
                    if let Some(v) = &vd.expr {
                        statistics += v.count_operations(&lookup);
                    }
                }
            }
        }
        // This shouldn't be a problem because of type level state and function visibilities
        let return_expr = self.return_expr.expect("Must have return expression in order to register");
        statistics += return_expr.count_operations(&lookup);
        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
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

    fn into_trait11(
        self, owner: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();
        let lines = self.lines.into_inner();
        for line in lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(_) => {}
                CommentOrVariableDeclaration::VarDec(vd) => {
                    if let Some(v) = &vd.expr {
                        statistics += v.count_operations(&lookup);
                    }
                }
            }
        }
        // This shouldn't be a problem because of type level state and function visibilities
        let return_expr = self.return_expr.expect("Must have return expression in order to register");
        statistics += return_expr.count_operations(&lookup);
        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
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
            statistics
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }

    fn into_trait21(
        self, owner: MultiVector, other: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();
        let lines = self.lines.into_inner();
        for line in lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(_) => {}
                CommentOrVariableDeclaration::VarDec(vd) => {
                    if let Some(v) = &vd.expr {
                        statistics += v.count_operations(&lookup);
                    }
                }
            }
        }
        // This shouldn't be a problem because of type level state and function visibilities
        let return_expr = self.return_expr.expect("Must have return expression in order to register");
        statistics += return_expr.count_operations(&lookup);
        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
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
            statistics
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }

    fn into_trait22(
        self, owner: MultiVector, other: MultiVector,
    ) -> Arc<RawTraitImplementation> {
        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };
        let mut statistics = VectoredOperationsTracker::zero();
        let lines = self.lines.into_inner();
        for line in lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(_) => {}
                CommentOrVariableDeclaration::VarDec(vd) => {
                    if let Some(v) = &vd.expr {
                        statistics += v.count_operations(&lookup);
                    }
                }
            }
        }
        // This shouldn't be a problem because of type level state and function visibilities
        let return_expr = self.return_expr.expect("Must have return expression in order to register");
        statistics += return_expr.count_operations(&lookup);
        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            multivector_dependencies: self.multivector_dependencies.into_inner(),
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
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
            statistics
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        return ti;
    }
}

impl<const AntiScalar: BasisElement, ExprType> TraitImplBuilder<AntiScalar, ExprType> {
    fn finish_inline<V: Into<String>>(self, b: &TraitImplBuilder<AntiScalar, HasNotReturned>, var_name: V) -> Option<Variable<ExprType>> {
        let mut outer_lines = b.lines.lock();
        let inner_lines = self.lines.into_inner();
        for line in inner_lines.into_iter() {
            outer_lines.push(line);
        }

        // If there was no return expression provided, assume the implementation "failed"
        // under normal circumstances like some combination of classes that doesn't produce a result
        let var = b.comment_variable_impl(self.return_comment, var_name, self.return_type, self.return_expr?);
        Some(var)
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