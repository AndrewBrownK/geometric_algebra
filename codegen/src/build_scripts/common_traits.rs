#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]

use crate::algebra::basis::filter::{SigFilter, SigSetFilter};
use crate::algebra::basis::grades::{AntiGrades, Grades};
use crate::ast::impls::{Elaborated, InlineOnly};
use crate::ast::traits::{NameTrait};
use crate::build_scripts::common_traits::impls::*;

pub mod conformal;

pub static Zero: Elaborated<ZeroImpl> = ZeroImpl
    .new_trait_named("Zero")
    .blurb("All elements set to zero.");
pub static One: Elaborated<OneImpl> = OneImpl
    .new_trait_named("One")
    .blurb("The scalar element set to one, and all other elements set to zero.");
pub static AntiOne: Elaborated<AntiOneImpl> = AntiOneImpl
    .new_trait_named("AntiOne")
    .blurb("The anti-scalar element set to one, and all other elements set to zero.");
pub static Unit: Elaborated<UnitImpl> = UnitImpl
    .new_trait_named("Unit")
    .blurb("All elements set to one.");
pub static Grade: Elaborated<GradeImpl> = GradeImpl
    .new_trait_named("Grade")
    .blurb("A multivector class may have uniform grade, or mixed grade, depending on \
    the grades of its elements. This trait only characterizes uniform grade multivectors.");
pub static AntiGrade: Elaborated<AntiGradeImpl> = AntiGradeImpl
    .new_trait_named("AntiGrade")
    .blurb("The AntiGrade can be described as the missing Grade with respect to an \
    AntiScalar. This trait only characterizes uniform anti-grade multivectors.");

pub static RightDual: Elaborated<RightDualImpl> = RightDualImpl
    .new_trait_named("RightDual")
    .blurb("TODO");

pub static RightAntiDual: Elaborated<RightAntiDualImpl> = RightAntiDualImpl
    .new_trait_named("RightAntiDual")
    .blurb("TODO");

pub static LeftDual: Elaborated<LeftDualImpl> = LeftDualImpl
    .new_trait_named("LeftDual")
    .blurb("TODO");

pub static LeftAntiDual: Elaborated<LeftAntiDualImpl> = LeftAntiDualImpl
    .new_trait_named("LeftAntiDual")
    .blurb("TODO");

pub static Reverse: Elaborated<ReverseImpl> = ReverseImpl
    .new_trait_named("Reverse")
    .blurb("TODO");

pub static AntiReverse: Elaborated<AntiReverseImpl> = AntiReverseImpl
    .new_trait_named("AntiReverse")
    .blurb("TODO");

pub static AutoMorphism: Elaborated<AutoMorphismImpl> = AutoMorphismImpl
    .new_trait_named("AutoMorphism")
    .blurb("TODO");

pub static AntiAutoMorphism: Elaborated<AntiAutoMorphismImpl> = AntiAutoMorphismImpl
    .new_trait_named("AntiAutoMorphism")
    .blurb("TODO");

pub static Conjugation: Elaborated<ConjugationImpl> = ConjugationImpl
    .new_trait_named("Conjugation")
    .blurb("TODO");

pub static AntiConjugation: Elaborated<AntiConjugationImpl> = AntiConjugationImpl
    .new_trait_named("AntiConjugation")
    .blurb("TODO");

pub static Complement: Elaborated<RightComplementImpl> = RightComplementImpl
    .new_trait_named("Complement")
    .blurb("TODO");

pub static RightComplement: Elaborated<RightComplementImpl> = RightComplementImpl
    .new_trait_named("RightComplement")
    .blurb("TODO");

pub static LeftComplement: Elaborated<LeftComplementImpl> = LeftComplementImpl
    .new_trait_named("LeftComplement")
    .blurb("TODO");

pub static DoubleComplement: Elaborated<DoubleComplementImpl> = DoubleComplementImpl
    .new_trait_named("DoubleComplement")
    .blurb("TODO");

pub static Negation: Elaborated<NegationImpl> = NegationImpl
    .new_trait_named("Negation")
    .blurb("TODO");

pub static Addition: Elaborated<AdditionImpl> = AdditionImpl
    .new_trait_named("Addition")
    .blurb("TODO");

pub static Subtraction: Elaborated<SubtractionImpl> = SubtractionImpl
    .new_trait_named("Subtraction")
    .blurb("TODO");

pub static Wedge: Elaborated<WedgeImpl> = WedgeImpl
    .new_trait_named("Wedge")
    .blurb("TODO");
pub static AntiWedge: Elaborated<AntiWedgeImpl> = AntiWedgeImpl
    .new_trait_named("AntiWedge")
    .blurb("TODO");

pub static GeometricProduct: Elaborated<GeometricProductImpl> = GeometricProductImpl
    .new_trait_named("GeometricProduct")
    .blurb("TODO");

pub static GeometricAntiProduct: Elaborated<GeometricAntiProductImpl> = GeometricAntiProductImpl
    .new_trait_named("GeometricAntiProduct")
    .blurb("TODO");

pub static Sandwich: Elaborated<SandwichImpl> = SandwichImpl
    .new_trait_named("Sandwich")
    .blurb("TODO");

pub static AntiSandwich: Elaborated<AntiSandwichImpl> = AntiSandwichImpl
    .new_trait_named("AntiSandwich")
    .blurb("TODO");

pub static DotProduct: Elaborated<DotProductImpl> = DotProductImpl
    .new_trait_named("DotProduct")
    .blurb("TODO");

pub static AntiDotProduct: Elaborated<AntiDotProductImpl> = AntiDotProductImpl
    .new_trait_named("AntiDotProduct")
    .blurb("TODO");

pub static ScalarNormSquared: Elaborated<ScalarNormSquaredImpl> = ScalarNormSquaredImpl
    .new_trait_named("ScalarNormSquared")
    .blurb("TODO");

pub static AntiScalarNormSquared: Elaborated<AntiScalarNormImpl> = AntiScalarNormImpl
    .new_trait_named("AntiScalarNormSquared")
    .blurb("TODO");

pub static ScalarNorm: Elaborated<ScalarNormSquaredImpl> = ScalarNormSquaredImpl
    .new_trait_named("ScalarNorm")
    .blurb("TODO");

pub static AntiScalarNorm: Elaborated<AntiScalarNormImpl> = AntiScalarNormImpl
    .new_trait_named("AntiScalarNorm")
    .blurb("TODO");





#[cfg(feature = "incorrect-wip-traits")]
pub static Square: Elaborated<SquareImpl> = SquareImpl
    .new_trait_named("Square")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static AntiSquare: Elaborated<AntiSquareImpl> = AntiSquareImpl
    .new_trait_named("AntiSquare")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static Powf: Elaborated<PowfImpl> = PowfImpl
    .new_trait_named("Powf")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static Powi: Elaborated<PowiImpl> = PowiImpl
    .new_trait_named("Powi")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static AntiPowf: Elaborated<AntiPowfImpl> = AntiPowfImpl
    .new_trait_named("AntiPowf")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static AntiPowi: Elaborated<AntiPowiImpl> = AntiPowiImpl
    .new_trait_named("AntiPowi")
    .blurb("TODO");






pub static ConstraintViolation: Elaborated<ConstraintViolationImpl> = ConstraintViolationImpl
    .new_trait_named("ConstraintViolation")
    .blurb("TODO");

pub static AntiConstraintViolation: Elaborated<AntiConstraintViolationImpl> = AntiConstraintViolationImpl
    .new_trait_named("AntiConstraintViolation")
    .blurb("TODO");

pub static ConstraintValid: Elaborated<ConstraintValidImpl> = ConstraintValidImpl
    .new_trait_named("ConstraintValid")
    .blurb("TODO");

pub static AntiConstraintValid: Elaborated<AntiConstraintValidImpl> = AntiConstraintValidImpl
    .new_trait_named("AntiConstraintValid")
    .blurb("TODO");

pub static Fix: Elaborated<FixImpl> = FixImpl
    .new_trait_named("Fix")
    .blurb("TODO");

pub static AntiFix: Elaborated<AntiFixImpl> = AntiFixImpl
    .new_trait_named("AntiFix")
    .blurb("TODO");

pub static Inverse: Elaborated<InverseImpl> = InverseImpl
    .new_trait_named("Inverse")
    .blurb("TODO");

pub static AntiInverse: Elaborated<AntiInverseImpl> = AntiInverseImpl
    .new_trait_named("AntiInverse")
    .blurb("TODO");

pub static GeometricQuotient: Elaborated<GeometricQuotientImpl> = GeometricQuotientImpl
    .new_trait_named("GeometricQuotient")
    .blurb("TODO");

pub static GeometricAntiQuotient: Elaborated<GeometricAntiQuotientImpl> = GeometricAntiQuotientImpl
    .new_trait_named("GeometricAntiQuotient")
    .blurb("TODO");

pub static SquareRoot: Elaborated<SquareRootImpl> = SquareRootImpl
    .new_trait_named("SquareRoot")
    .blurb("TODO");

pub static AntiSquareRoot: Elaborated<AntiSquareRootImpl> = AntiSquareRootImpl
    .new_trait_named("AntiSquareRoot")
    .blurb("TODO");

pub static BulkContraction: Elaborated<BulkContractionImpl> = BulkContractionImpl
    .new_trait_named("BulkContraction")
    .blurb("TODO");

pub static WeightContraction: Elaborated<WeightContractionImpl> = WeightContractionImpl
    .new_trait_named("WeightContraction")
    .blurb("TODO");

pub static BulkExpansion: Elaborated<BulkExpansionImpl> = BulkExpansionImpl
    .new_trait_named("BulkExpansion")
    .blurb("TODO");

pub static WeightExpansion: Elaborated<WeightExpansionImpl> = WeightExpansionImpl
    .new_trait_named("WeightExpansion")
    .blurb("TODO");

pub static Into: Elaborated<IntoImpl> = IntoImpl
    .new_trait_named("Into")
    .blurb("TODO");

pub static TryInto: Elaborated<TryIntoImpl> = TryIntoImpl
    .new_trait_named("TryInto")
    .blurb("TODO");

#[allow(non_snake_case)]
pub fn SubType<F1, F2, F3>(filter_multivecs_in: F1, filter_elements: F2, filter_multivecs_out: F3) -> InlineOnly<SubTypeImpl<F1, F2, F3>> where
    F1: SigSetFilter + Copy + Send + Sync + 'static,
    F2: SigFilter + Copy + Send + Sync + 'static,
    F3: SigSetFilter + Copy + Send + Sync + 'static {
    InlineOnly::new("SubType", SubTypeImpl { filter_multivecs_in, filter_elements, filter_multivecs_out })
}


// NOTE: If you find yourself wanting to generate grade selection traits, you are
// probably generating extremely wasteful implementations that perform a lot more
// floating point calculations than necessary. That is why these trait definitions
// are InlineOnly. They will not generate trait definitions, but you can still invoke
// them as if they were traits, and they will always be inlined instead. (With inlining,
// terms that are unused by the final result of each function will be cut out of the AST
// automatically. With grade selection declarations (not inlining strictly always), then
// you'd have the chance to use grade selection in application code which is very undesirable.)

macro_rules! select_grades {
    ($($gr:literal),+ $(,)?) => {
        pub static SelectGrade: [InlineOnly<SelectGradesImpl>; 17] = [$(
            InlineOnly::new(concat!("SelectGrade", $gr), SelectGradesImpl(Grades::new($gr))),
        )+];
        pub static SelectAntiGrade: [InlineOnly<SelectAntiGradesImpl>; 17] = [$(
            InlineOnly::new(concat!("SelectAntiGrade", $gr), SelectAntiGradesImpl(AntiGrades::new($gr))),
        )+];
    };
}
select_grades!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

pub const fn select_grades(grades: Grades) -> InlineOnly<SelectGradesImpl> {
    InlineOnly::new("SelectMultipleGrades", SelectGradesImpl(grades))
}
pub const fn select_anti_grades(anti_grades: AntiGrades) -> InlineOnly<SelectAntiGradesImpl> {
    InlineOnly::new("SelectMultipleAntiGrades", SelectAntiGradesImpl(anti_grades))
}

mod impls {
    use std::collections::{BTreeMap, BTreeSet};

    use async_trait::async_trait;

    use crate::algebra::basis::{BasisElement, BasisSignature};
    use crate::algebra::basis::filter::{SigFilter, SigSetFilter};
    use crate::algebra::basis::grades::{AntiGrades, Grades};
    use crate::algebra::multivector::DynamicMultiVector;
    use crate::ast::datatype::{Integer, MultiVector};
    use crate::ast::expressions::{Expression, FloatExpr, IntExpr};
    use crate::ast::traits::{HasNotReturned, TraitDef_1_Type_1_Arg, TraitDef_2_Types_2_Args, TraitImpl_11, TraitImplBuilder};
    use crate::ast::Variable;
    use crate::build_scripts::common_traits::{AntiInverse, AntiReverse, AntiDotProduct, AntiSquareRoot, AntiWedge, GeometricAntiProduct, GeometricProduct, Inverse, Reverse, RightAntiDual, RightDual, DotProduct, SquareRoot, Subtraction, Wedge};
    use crate::elements::scalar;

    #[macro_export]
    macro_rules! trait_impl_1_type_0_args {
        ($trait_impl:ident($builder:ident, $owner:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_10 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $owner: $crate::ast::datatype::MultiVector,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_1_type_1_arg {
        ($trait_impl:ident($builder:ident, $slf:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_11 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_2_types_1_arg {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_21 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::datatype::MultiVector,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_2_types_2_args {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_22 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_1_type_2_arg_i32 {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_12i for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::Variable<$crate::ast::datatype::Integer>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_1_type_2_arg_f32 {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_12f for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::Variable<$crate::ast::datatype::Float>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }

    trait_impl_1_type_0_args!(ZeroImpl(builder, owner) -> MultiVector {
        builder.return_expr(owner.construct(|_| FloatExpr::Literal(0.0)))
    });

    trait_impl_1_type_0_args!(OneImpl(builder, owner) -> MultiVector {
        if !owner.elements().into_iter().any(|el| el.signature() == BasisSignature::scalar) {
            return None;
        }
        builder.return_expr(owner.construct(|element| {
            if element.signature() == BasisSignature::scalar {
                FloatExpr::Literal(1.0)
            } else {
                FloatExpr::Literal(0.0)
            }
        }))
    });

    trait_impl_1_type_0_args!(AntiOneImpl(builder, owner) -> MultiVector {
        if !owner.elements().into_iter().any(|el| el.signature() == AntiScalar.signature()) {
            return None;
        }
        builder.return_expr(owner.construct(|element| {
            if element.signature() == AntiScalar.signature() {
                FloatExpr::Literal(1.0)
            } else {
                FloatExpr::Literal(0.0)
            }
        }))
    });

    trait_impl_1_type_0_args!(UnitImpl(builder, owner) -> MultiVector {
        builder.return_expr(owner.construct(|_| FloatExpr::Literal(1.0)))
    });

    trait_impl_1_type_0_args!(GradeImpl(builder, owner) -> Integer {
        let gr = owner.grade()?;
        builder.return_expr(IntExpr::Literal(gr))
    });

    trait_impl_1_type_0_args!(AntiGradeImpl(builder, owner) -> Integer {
        let ag = owner.anti_grade()?;
        builder.return_expr(IntExpr::Literal(ag))
    });

    // TODO left duals, because https://terathon.com/blog/poor-foundations-ga.html
    //  Also rename scalar product to inner product, also for technical reasons in that blog post

    trait_impl_1_type_1_arg!(RightDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.right_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(RightAntiDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.right_anti_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(LeftDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.left_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(LeftAntiDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.left_anti_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ReverseImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.reverse(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiReverseImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.anti_reverse(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AutoMorphismImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements_by_groups() {
            if el.grade() % 2 == 1 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiAutoMorphismImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements_by_groups() {
            if el.anti_grade(AntiScalar) % 2 == 1 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ConjugationImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements_by_groups() {
            if (el.grade() + 3) % 4 < 2 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiConjugationImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements_by_groups() {
            if (el.anti_grade(AntiScalar) + 3) % 4 < 2 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(RightComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.fix_name_and_sign(el.right_complement(AntiScalar));
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(LeftComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.fix_name_and_sign(el.left_complement(AntiScalar));
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(DoubleComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let el = el.right_complement(AntiScalar).right_complement(AntiScalar);
            let (f, el) = builder.ga.fix_name_and_sign(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(NegationImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            result += (fe * -1.0, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(AdditionImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            dyn_mv += (a, a_el);
        }
        for (b, b_el) in other.elements_by_groups() {
            dyn_mv += (b, b_el);
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(SubtractionImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            dyn_mv += (a, a_el);
        }
        for (b, b_el) in other.elements_by_groups() {
            dyn_mv += (b * -1.0, b_el);
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(WedgeImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            for (b, b_el) in other.elements_by_groups() {
                let a = a.clone();
                let (f, c) = builder.ga.wedge(a_el, b_el);
                dyn_mv += (a * b * f, c);
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(AntiWedgeImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            for (b, b_el) in other.elements_by_groups() {
                let a = a.clone();
                let (f, c) = builder.ga.anti_wedge(a_el, b_el);
                dyn_mv += (a * b * f, c);
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(GeometricProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            for (b, b_el) in other.elements_by_groups() {
                let sop = builder.ga.product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(GeometricAntiProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            for (b, b_el) in other.elements_by_groups() {
                let sop = builder.ga.anti_product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(SandwichImpl(builder, slf, other) -> MultiVector {
        // TODO incorrect cycle detection if use all invoke
        let c = GeometricProduct.inline(&builder, slf.clone(), other).await?;
        let r = Reverse.invoke(&mut builder, slf).await?;
        let result = GeometricProduct.invoke(&mut builder, c, r).await?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(AntiSandwichImpl(builder, slf, other) -> MultiVector {
        // TODO incorrect cycle detection if use all invoke
        let c = GeometricAntiProduct.inline(&builder, slf.clone(), other).await?;
        let r = AntiReverse.invoke(&mut builder, slf).await?;
        let result = GeometricAntiProduct.invoke(&mut builder, c, r).await?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(DotProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            for (b, b_el) in other.elements_by_groups() {
                let sop = builder.ga.scalar_product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = p.coefficient;
                    dyn_mv += (a * b * c, p.element);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(AntiDotProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_by_groups() {
            for (b, b_el) in other.elements_by_groups() {
                let sop = builder.ga.anti_scalar_product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = p.coefficient;
                    dyn_mv += (a * b * c, p.element);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_1_type_1_arg!(ScalarNormSquaredImpl(builder, slf) -> MultiVector {
        let result = DotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiScalarNormSquaredImpl(builder, slf) -> MultiVector {
        let result = AntiDotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ScalarNormImpl(builder, slf) -> MultiVector {
        let dot = DotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        let result = SquareRoot.invoke(&mut builder, dot).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiScalarNormImpl(builder, slf) -> MultiVector {
        let dot = AntiDotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        let result = AntiSquareRoot.invoke(&mut builder, dot).await?;
        builder.return_expr(result)
    });





    // TODO troublesome traits that won't make it to 1.0
    #[cfg(feature = "incorrect-wip-traits")]
    include!("wip_traits.rs");






    // TODO these violation traits should only be implemented where the constraint is possible to violate
    //  then also make a trait where it is always/only implemented where the constraint is impossible to violate
    trait_impl_1_type_1_arg!(ConstraintViolationImpl(builder, slf) -> MultiVector {
        let r = Reverse.inline(&builder, slf.clone()).await?;
        let p = GeometricProduct.inline(&builder, slf.clone(), r).await?;
        let d = DotProduct.inline(&builder, slf.clone(), slf.clone()).await?;
        let result = Subtraction.inline(&builder, p, d).await?;
        builder.return_expr(result)
    });
    trait_impl_1_type_1_arg!(AntiConstraintViolationImpl(builder, slf) -> MultiVector {
        let r = AntiReverse.inline(&builder, slf.clone()).await?;
        let p = GeometricAntiProduct.inline(&builder, slf.clone(), r).await?;
        let d = AntiDotProduct.inline(&builder, slf.clone(), slf.clone()).await?;
        let result = Subtraction.inline(&builder, p, d).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ConstraintValidImpl(builder, slf) -> MultiVector {
        let r = Reverse.inline(&builder, slf.clone()).await?;
        let p = GeometricProduct.inline(&builder, slf.clone(), r).await;
        let d = DotProduct.inline(&builder, slf.clone(), slf.clone()).await;
        let (p, d) = match (p, d) {
            (None, None) => return builder.return_expr(slf),
            (p, d) => (p?, d?),
        };
        match Subtraction.inline(&builder, p, d).await {
            None => builder.return_expr(slf),
            Some(_) => None,
        }
    });
    trait_impl_1_type_1_arg!(AntiConstraintValidImpl(builder, slf) -> MultiVector {
        let r = AntiReverse.inline(&builder, slf.clone()).await?;
        let p = GeometricAntiProduct.inline(&builder, slf.clone(), r).await;
        let d = AntiDotProduct.inline(&builder, slf.clone(), slf.clone()).await;
        let (p, d) = match (p, d) {
            (None, None) => return builder.return_expr(slf),
            (p, d) => (p?, d?),
        };
        match Subtraction.inline(&builder, p, d).await {
            None => builder.return_expr(slf),
            Some(_) => None,
        }
    });

    // See section 3.4.3 and 3.6.2 of the book
    // TODO we absolutely need advanced inlining and factorization for Fix and AntiFix
    trait_impl_1_type_1_arg!(FixImpl(builder, slf) -> MultiVector {
        let r = Reverse.inline(&builder, slf.clone()).await?;
        let p = GeometricProduct.inline(&builder, slf.clone(), r).await?;
        let sqrt = SquareRoot.inline(&builder, p).await?;
        let i = Inverse.inline(&builder, sqrt).await?;
        let result = GeometricProduct.inline(&builder, slf, i).await?;
        builder.return_expr(result)
    });
    trait_impl_1_type_1_arg!(AntiFixImpl(builder, slf) -> MultiVector {
        let r = AntiReverse.inline(&builder, slf.clone()).await?;
        let p = GeometricAntiProduct.inline(&builder, slf.clone(), r).await?;
        let sqrt = AntiSquareRoot.inline(&builder, p).await?;
        let i = AntiInverse.inline(&builder, sqrt).await?;
        let result = GeometricAntiProduct.inline(&builder, slf, i).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(InverseImpl(builder, slf) -> MultiVector {
        let scalar_mv = MultiVector::from(builder.mvs.scalar());
        let dot = DotProduct.inline(&builder, slf.clone(), slf).await?;
        let raw_dot = FloatExpr::AccessMultiVecFlat(dot.into(), 0);
        let result = scalar_mv.construct_direct([(scalar, FloatExpr::Product(vec![(raw_dot, -1.0)], 1.0))]);
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiInverseImpl(builder, slf) -> MultiVector {
        let anti_scalar_mv = MultiVector::from(builder.mvs.anti_scalar());
        let anti_scalar = builder.ga.anti_scalar();
        let dot = AntiDotProduct.inline(&builder, slf.clone(), slf).await?;
        let raw_dot = FloatExpr::AccessMultiVecFlat(dot.into(), 0);
        let result = anti_scalar_mv.construct_direct([(anti_scalar, FloatExpr::Product(vec![(raw_dot, -1.0)], 1.0))]);
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(GeometricQuotientImpl(builder, slf, other) -> MultiVector {
        let i = Inverse.inline(&builder, other).await?;
        let result = GeometricProduct.inline(&builder, slf, i).await?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(GeometricAntiQuotientImpl(builder, slf, other) -> MultiVector {
        let i = AntiInverse.inline(&builder, other).await?;
        let result = GeometricAntiProduct.inline(&builder, slf, i).await?;
        builder.return_expr(result)
    });







    trait_impl_1_type_1_arg!(SquareRootImpl(builder, slf) -> MultiVector {
        let scalar_mv = MultiVector::from(builder.mvs.scalar());
        if slf.expr_type == scalar_mv {
            let raw = FloatExpr::AccessMultiVecFlat(slf.into(), 0);
            let sqrt = FloatExpr::Product(vec![(raw, 0.5)], 1.0);
            let result = scalar_mv.construct_direct([(scalar, sqrt)]);
            return builder.return_expr(result)
        }
        let mv_elements = scalar_mv.elements();
        if !mv_elements.contains(&scalar) {
            return None
        }

        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements_flat() {
            for (b, b_el) in slf.elements_flat() {
                let sop = builder.ga.product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        if mv.mv_class != slf.expr_type {
            return None
        }

        // TODO support SquareRoot with more types of MultiVector

        // TODO I think these ones stand out to me as telling:
        //  - Scalar squares to Scalar
        //  - AntiScalar squares to Scalar
        //  - VersorOdd squares to VersorOdd
        //  - VersorEven squares to VersorOdd
        //  - AntiDualNum squares to AntiDualNum
        //  - DualNum squares to AntiDualNum

        // TODO other things of note...
        //  - CircleRotor squares to VersorOdd
        //  - DipoleInversion squares to VersorOdd

        None
    });

    trait_impl_1_type_1_arg!(AntiSquareRootImpl(builder, slf) -> MultiVector {
        let anti_scalar_mv = MultiVector::from(builder.mvs.anti_scalar());
        let anti_scalar = builder.ga.anti_scalar();
        if slf.expr_type == anti_scalar_mv {
            let raw = FloatExpr::AccessMultiVecFlat(slf.into(), 0);
            let sqrt = FloatExpr::Product(vec![(raw, 0.5)], 1.0);
            let result = anti_scalar_mv.construct_direct([(anti_scalar, sqrt)]);
            return builder.return_expr(result)
        }
        let mv_elements = anti_scalar_mv.elements();
        if !mv_elements.contains(&anti_scalar) {
            return None
        }
        // TODO support AntiSquareRoot with more types of MultiVector

        // TODO I think these ones stand out to me as telling:
        //  - Scalar anti-squares to AntiScalar
        //  - AntiScalar anti-squares to AntiScalar
        //  - VersorOdd anti-squares to VersorEven
        //  - VersorEven anti-squares to VersorEven
        //  - AntiDualNum anti-squares to DualNum
        //  - DualNum anti-squares to DualNum

        // TODO so knowing that, we can try to predict....
        //  - CircleRotor anti-squares to CircleRotor? No.... VersorEven.
        //  - DipoleInversion anti-squares to VersorEven

        None
    });

    trait_impl_2_types_2_args!(BulkExpansionImpl(builder, slf, other) -> MultiVector {
        let dual = RightDual.inline(&builder, other).await?;
        let wedge = Wedge.inline(&builder, slf, dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(WeightExpansionImpl(builder, slf, other) -> MultiVector {
        let anti_dual = RightAntiDual.inline(&builder, other).await?;
        let wedge = Wedge.inline(&builder, slf, anti_dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(BulkContractionImpl(builder, slf, other) -> MultiVector {
        let dual = RightDual.inline(&builder, other).await?;
        let anti_wedge = AntiWedge.inline(&builder, slf, dual).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_types_2_args!(WeightContractionImpl(builder, slf, other) -> MultiVector {
        let anti_dual = RightAntiDual.inline(&builder, other).await?;
        let anti_wedge = AntiWedge.inline(&builder, slf, anti_dual).await?;
        builder.return_expr(anti_wedge)
    });

    // Into is treated kind of special, because we actually want to implement From,
    // but the TraitImpl_21 pattern assumes the first argument is the owner.
    // So in the code generation we have a special exception to treat Into as From instead.
    trait_impl_2_types_1_arg!(IntoImpl(builder, slf, other) -> MultiVector {
        if slf.expression_type() == other {
            return None;
        }
        let other_elements: BTreeSet<_> = other.elements().into_iter().collect();
        let mut these_elements: BTreeMap<_, _> = BTreeMap::new();
        for (f, el) in slf.elements_flat() {
            if !other_elements.contains(&el) {
                return None;
            }
            these_elements.insert(el, f);
        }
        let result = other.construct(|el| these_elements.remove(&el).unwrap_or(FloatExpr::Literal(0.0)));
        builder.return_expr(result)
    });

    // TryInto is treated kind of special, because we actually want to implement TryFrom,
    // but the TraitImpl_21 pattern assumes the first argument is the owner.
    // So in the code generation we have a special exception to treat TrInto as TryFrom instead.
    trait_impl_2_types_1_arg!(TryIntoImpl(builder, slf, other) -> MultiVector {
        let mut missing_some = false;
        let mut overlapping_some = false;
        let other_elements: BTreeSet<_> = other.elements().into_iter().collect();
        let mut these_elements: BTreeMap<_, _> = BTreeMap::new();
        for (f, el) in slf.elements_flat() {
            if other_elements.contains(&el) {
                overlapping_some = true;
            } else {
                missing_some = true;
            }
            these_elements.insert(el, f);
        }
        if !missing_some || !overlapping_some {
            return None;
        }
        let result = other.construct(|el| these_elements.remove(&el).unwrap_or(FloatExpr::Literal(0.0)));
        builder.return_expr(result)
    });




    #[derive(Clone, Copy)]
    pub struct SubTypeImpl<F1, F2, F3> {
        pub filter_multivecs_in: F1,
        pub filter_elements: F2,
        pub filter_multivecs_out: F3,
    }
    #[async_trait]
    impl<F1, F2, F3> TraitImpl_11 for SubTypeImpl<F1, F2, F3> where
        F1: SigSetFilter + Copy + Send + Sync + 'static,
        F2: SigFilter + Copy + Send + Sync + 'static,
        F3: SigSetFilter + Copy + Send + Sync + 'static {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let sig_in = slf.expr_type.signatures();
            if !self.filter_multivecs_in.filter_sig_set(&sig_in) {
                return None;
            }
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                if self.filter_elements.filter_sig(a_el.signature()) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            let sig_out = mv.mv_class.signatures();
            if !self.filter_multivecs_out.filter_sig_set(&sig_out) {
                return None;
            }
            builder.return_expr(mv)
        }
    }






    #[derive(Clone, Copy)]
    pub struct SelectGradesImpl(pub Grades);
    #[async_trait]
    impl TraitImpl_11 for SelectGradesImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                if self.0.contains(a_el.grades()) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            builder.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct SelectAntiGradesImpl(pub AntiGrades);
    #[async_trait]
    impl TraitImpl_11 for SelectAntiGradesImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                if self.0.contains(a_el.anti_grades(AntiScalar)) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            builder.return_expr(mv)
        }
    }
}
