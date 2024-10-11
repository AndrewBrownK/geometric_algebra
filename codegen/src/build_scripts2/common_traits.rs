#![allow(non_upper_case_globals)]

use crate::algebra2::basis::grades::{AntiGrades, Grades};
use crate::ast2::impls::{Elaborated, InlineOnly};
use crate::ast2::traits::{NameTrait, TraitDef_1_Type_1_Arg, TraitImpl_10, TraitImpl_11, TraitImpl_21};
use crate::build_scripts2::common_traits::impls::*;

pub static Zero: Elaborated<ZeroImpl> = ZeroImpl.new_trait_named("Zero").blurb("All elements set to zero.");
pub static One: Elaborated<OneImpl> = OneImpl.new_trait_named("One").blurb("The scalar element set to one, and all other elements set to zero.");
pub static AntiOne: Elaborated<AntiOneImpl> = AntiOneImpl.new_trait_named("AntiOne").blurb("The anti-scalar element set to one, and all other elements set to zero.");
pub static Unit: Elaborated<UnitImpl> = UnitImpl.new_trait_named("Unit").blurb("All elements set to one.");
pub static Grade: Elaborated<GradeImpl> = GradeImpl.new_trait_named("Grade").blurb(
    "A multivector class may have uniform grade, or mixed grade, depending on \
    the grades of its elements. This trait only characterizes uniform grade multivectors.",
);
pub static AntiGrade: Elaborated<AntiGradeImpl> = AntiGradeImpl.new_trait_named("AntiGrade").blurb(
    "The AntiGrade can be described as the missing Grade with respect to an \
    AntiScalar. This trait only characterizes uniform anti-grade multivectors.",
);

pub static RightDual: Elaborated<RightDualImpl> = RightDualImpl.new_trait_named("RightDual").blurb("TODO");

pub static RightAntiDual: Elaborated<RightAntiDualImpl> = RightAntiDualImpl.new_trait_named("RightAntiDual").blurb("TODO");

pub static LeftDual: Elaborated<LeftDualImpl> = LeftDualImpl.new_trait_named("LeftDual").blurb("TODO");

pub static LeftAntiDual: Elaborated<LeftAntiDualImpl> = LeftAntiDualImpl.new_trait_named("LeftAntiDual").blurb("TODO");

pub static Reverse: Elaborated<ReverseImpl> = ReverseImpl.new_trait_named("Reverse").blurb("TODO");

pub static AntiReverse: Elaborated<AntiReverseImpl> = AntiReverseImpl.new_trait_named("AntiReverse").blurb("TODO");

pub static AutoMorphism: Elaborated<AutoMorphismImpl> = AutoMorphismImpl.new_trait_named("AutoMorphism").blurb("TODO");

pub static AntiAutoMorphism: Elaborated<AntiAutoMorphismImpl> = AntiAutoMorphismImpl.new_trait_named("AntiAutoMorphism").blurb("TODO");

pub static Conjugation: Elaborated<ConjugationImpl> = ConjugationImpl.new_trait_named("Conjugation").blurb("TODO");

pub static AntiConjugation: Elaborated<AntiConjugationImpl> = AntiConjugationImpl.new_trait_named("AntiConjugation").blurb("TODO");

pub static RightComplement: Elaborated<RightComplementImpl> = RightComplementImpl.new_trait_named("RightComplement").blurb("TODO");

pub static LeftComplement: Elaborated<LeftComplementImpl> = LeftComplementImpl.new_trait_named("LeftComplement").blurb("TODO");

pub static DoubleComplement: Elaborated<DoubleComplementImpl> = DoubleComplementImpl.new_trait_named("DoubleComplement").blurb("TODO");

pub static Negation: Elaborated<NegationImpl> = NegationImpl.new_trait_named("Negation").blurb("TODO");

pub static Addition: Elaborated<AdditionImpl> = AdditionImpl.new_trait_named("Addition").blurb("TODO");

pub static Subtraction: Elaborated<SubtractionImpl> = SubtractionImpl.new_trait_named("Subtraction").blurb("TODO");

pub static Wedge: Elaborated<WedgeImpl> = WedgeImpl.new_trait_named("Wedge").blurb("TODO");
pub static AntiWedge: Elaborated<AntiWedgeImpl> = AntiWedgeImpl.new_trait_named("AntiWedge").blurb("TODO");

pub static GeometricProduct: Elaborated<GeometricProductImpl> = GeometricProductImpl.new_trait_named("GeometricProduct").blurb("TODO");

pub static GeometricAntiProduct: Elaborated<GeometricAntiProductImpl> = GeometricAntiProductImpl.new_trait_named("GeometricAntiProduct").blurb("TODO");

pub static Sandwich: Elaborated<SandwichImpl> = SandwichImpl.new_trait_named("Sandwich").blurb("TODO");

pub static AntiSandwich: Elaborated<AntiSandwichImpl> = AntiSandwichImpl.new_trait_named("AntiSandwich").blurb("TODO");

pub static ScalarProduct: Elaborated<ScalarProductImpl> = ScalarProductImpl.new_trait_named("ScalarProduct").blurb("TODO");

pub static AntiScalarProduct: Elaborated<AntiScalarProductImpl> = AntiScalarProductImpl.new_trait_named("AntiScalarProduct").blurb("TODO");

pub static ScalarNormSquared: Elaborated<ScalarNormSquaredImpl> = ScalarNormSquaredImpl.new_trait_named("ScalarNormSquared").blurb("TODO");

pub static AntiScalarNormSquared: Elaborated<AntiScalarNormImpl> = AntiScalarNormImpl.new_trait_named("AntiScalarNormSquared").blurb("TODO");

pub static ScalarNorm: Elaborated<ScalarNormSquaredImpl> = ScalarNormSquaredImpl.new_trait_named("ScalarNorm").blurb("TODO");

pub static AntiScalarNorm: Elaborated<AntiScalarNormImpl> = AntiScalarNormImpl.new_trait_named("AntiScalarNorm").blurb("TODO");

pub static Powf: Elaborated<PowfImpl> = PowfImpl.new_trait_named("Powf").blurb("TODO");
pub static Powi: Elaborated<PowiImpl> = PowiImpl.new_trait_named("Powi").blurb("TODO");
pub static AntiPowf: Elaborated<AntiPowfImpl> = AntiPowfImpl.new_trait_named("AntiPowf").blurb("TODO");
pub static AntiPowi: Elaborated<AntiPowiImpl> = AntiPowiImpl.new_trait_named("AntiPowi").blurb("TODO");

pub static ConstraintViolation: Elaborated<ConstraintViolationImpl> = ConstraintViolationImpl.new_trait_named("ConstraintViolation").blurb("TODO");

pub static AntiConstraintViolation: Elaborated<AntiConstraintViolationImpl> = AntiConstraintViolationImpl.new_trait_named("AntiConstraintViolation").blurb("TODO");

pub static ConstraintValid: Elaborated<ConstraintValidImpl> = ConstraintValidImpl.new_trait_named("ConstraintValid").blurb("TODO");

pub static AntiConstraintValid: Elaborated<AntiConstraintValidImpl> = AntiConstraintValidImpl.new_trait_named("AntiConstraintValid").blurb("TODO");

pub static Fix: Elaborated<FixImpl> = FixImpl.new_trait_named("Fix").blurb("TODO");

pub static AntiFix: Elaborated<AntiFixImpl> = AntiFixImpl.new_trait_named("AntiFixImpl").blurb("TODO");

pub static Inverse: Elaborated<InverseImpl> = InverseImpl.new_trait_named("Inverse").blurb("TODO");

pub static AntiInverse: Elaborated<AntiInverseImpl> = AntiInverseImpl.new_trait_named("AntiInverse").blurb("TODO");

pub static GeometricQuotient: Elaborated<GeometricQuotientImpl> = GeometricQuotientImpl.new_trait_named("GeometricQuotient").blurb("TODO");

pub static GeometricAntiQuotient: Elaborated<GeometricAntiQuotientImpl> = GeometricAntiQuotientImpl.new_trait_named("GeometricAntiQuotient").blurb("TODO");

pub static SquareRoot: Elaborated<SquareRootImpl> = SquareRootImpl.new_trait_named("SquareRoot").blurb("TODO");

pub static AntiSquareRoot: Elaborated<AntiSquareRootImpl> = AntiSquareRootImpl.new_trait_named("AntiSquareRoot").blurb("TODO");

pub static Square: Elaborated<SquareImpl> = SquareImpl.new_trait_named("Square").blurb("TODO");
pub static AntiSquare: Elaborated<AntiSquareImpl> = AntiSquareImpl.new_trait_named("AntiSquare").blurb("TODO");

pub static BulkContraction: Elaborated<BulkContractionImpl> = BulkContractionImpl.new_trait_named("BulkContraction").blurb("TODO");

pub static WeightContraction: Elaborated<WeightContractionImpl> = WeightContractionImpl.new_trait_named("WeightContraction").blurb("TODO");

pub static BulkExpansion: Elaborated<BulkExpansionImpl> = BulkExpansionImpl.new_trait_named("BulkExpansion").blurb("TODO");

pub static WeightExpansion: Elaborated<WeightExpansionImpl> = WeightExpansionImpl.new_trait_named("WeightExpansion").blurb("TODO");

pub static Into: Elaborated<IntoImpl> = IntoImpl.new_trait_named("Into").blurb("TODO");

pub static TryInto: Elaborated<TryIntoImpl> = TryIntoImpl.new_trait_named("TryInto").blurb("TODO");

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
    use std::ops::MulAssign;
    use async_trait::async_trait;

    use crate::algebra2::basis::grades::{AntiGrades, Grades};
    use crate::algebra2::basis::{BasisElement, BasisSignature};
    use crate::algebra2::multivector::DynamicMultiVector;
    use crate::ast2::datatype::{Integer, MultiVector};
    use crate::ast2::expressions::{Expression, FloatExpr, IntExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
    use crate::ast2::traits::{HasNotReturned, TraitDef_1_Type_1_Arg, TraitDef_2_Types_2_Args, TraitImplBuilder, TraitImpl_10, TraitImpl_11, TraitImpl_21, TraitImpl_22, TraitDef_1_Type_2_Args_f32};
    use crate::ast2::Variable;
    use crate::build_scripts2::common_traits::{AntiInverse, AntiPowf, AntiReverse, AntiScalarProduct, AntiSquareRoot, AntiWedge, GeometricAntiProduct, GeometricProduct, Inverse, Powf, Reverse, RightAntiDual, RightDual, ScalarProduct, SquareRoot, Subtraction, Wedge};
    use crate::elements::scalar;

    #[macro_export]
    macro_rules! trait_impl_1_type_0_args {
        ($trait_impl:ident($builder:ident, $owner:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast2::traits::TraitImpl_10 for $trait_impl {
                type Output = $output;
                async fn general_implementation<const AntiScalar: $crate::algebra2::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast2::traits::TraitImplBuilder<AntiScalar, $crate::ast2::traits::HasNotReturned>,
                    $owner: $crate::ast2::datatype::MultiVector,
                ) -> Option<$crate::ast2::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
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
            impl $crate::ast2::traits::TraitImpl_11 for $trait_impl {
                type Output = $output;
                async fn general_implementation<const AntiScalar: $crate::algebra2::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast2::traits::TraitImplBuilder<AntiScalar, $crate::ast2::traits::HasNotReturned>,
                    $slf: $crate::ast2::Variable<$crate::ast2::datatype::MultiVector>,
                ) -> Option<$crate::ast2::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
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
            impl $crate::ast2::traits::TraitImpl_21 for $trait_impl {
                type Output = $output;
                async fn general_implementation<const AntiScalar: $crate::algebra2::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast2::traits::TraitImplBuilder<AntiScalar, $crate::ast2::traits::HasNotReturned>,
                    $slf: $crate::ast2::Variable<$crate::ast2::datatype::MultiVector>,
                    $other: $crate::ast2::datatype::MultiVector,
                ) -> Option<$crate::ast2::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
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
            impl $crate::ast2::traits::TraitImpl_22 for $trait_impl {
                type Output = $output;
                async fn general_implementation<const AntiScalar: $crate::algebra2::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast2::traits::TraitImplBuilder<AntiScalar, $crate::ast2::traits::HasNotReturned>,
                    $slf: $crate::ast2::Variable<$crate::ast2::datatype::MultiVector>,
                    $other: $crate::ast2::Variable<$crate::ast2::datatype::MultiVector>,
                ) -> Option<$crate::ast2::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
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
            impl $crate::ast2::traits::TraitImpl_12i for $trait_impl {
                type Output = $output;
                async fn general_implementation<const AntiScalar: $crate::algebra2::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast2::traits::TraitImplBuilder<AntiScalar, $crate::ast2::traits::HasNotReturned>,
                    $slf: $crate::ast2::Variable<$crate::ast2::datatype::MultiVector>,
                    $other: $crate::ast2::Variable<$crate::ast2::datatype::Integer>,
                ) -> Option<$crate::ast2::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
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
            impl $crate::ast2::traits::TraitImpl_12f for $trait_impl {
                type Output = $output;
                async fn general_implementation<const AntiScalar: $crate::algebra2::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast2::traits::TraitImplBuilder<AntiScalar, $crate::ast2::traits::HasNotReturned>,
                    $slf: $crate::ast2::Variable<$crate::ast2::datatype::MultiVector>,
                    $other: $crate::ast2::Variable<$crate::ast2::datatype::Float>,
                ) -> Option<$crate::ast2::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
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

    trait_impl_2_types_2_args!(ScalarProductImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_types_2_args!(AntiScalarProductImpl(builder, slf, other) -> MultiVector {
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
        let result = ScalarProduct.invoke(&mut builder, slf.clone(), slf).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiScalarNormSquaredImpl(builder, slf) -> MultiVector {
        let result = AntiScalarProduct.invoke(&mut builder, slf.clone(), slf).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ScalarNormImpl(builder, slf) -> MultiVector {
        let dot = ScalarProduct.invoke(&mut builder, slf.clone(), slf).await?;
        let result = SquareRoot.invoke(&mut builder, dot).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiScalarNormImpl(builder, slf) -> MultiVector {
        let dot = AntiScalarProduct.invoke(&mut builder, slf.clone(), slf).await?;
        let result = AntiSquareRoot.invoke(&mut builder, dot).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_2_arg_f32!(PowfImpl(builder, slf, exp) -> MultiVector {
        let exp: FloatExpr = exp.into();
        let mut dyn_mv = DynamicMultiVector::zero();
        let r = slf.clone();
        for (a, a_el) in slf.elements_flat() {
            for (b, b_el) in r.elements_flat() {
                let sop = builder.ga.product(a_el, b_el);
                for p in sop.sum {
                    let f = if a_el == b_el {
                        let a_exp = FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp.clone())), 1.0);
                        let b_exp = FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp.clone())), 1.0);
                        FloatExpr::Product(vec![(a_exp, 0.5), (b_exp, 0.5)], p.coefficient)
                    } else {
                        let mut f = exp.clone() * p.coefficient * 0.5;
                        let exp_minus_one = FloatExpr::Sum(vec![(exp.clone(), 1.0)], -1.0);
                        if a_el == scalar {
                            f.mul_assign(FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                        } else {
                            f.mul_assign(a.clone());
                        }
                        if b_el == scalar {
                            f.mul_assign(FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                        } else {
                            f.mul_assign(b.clone());
                        }
                        f
                    };
                    dyn_mv += (f, p.element);
                }
            }
        }
        let mut result = dyn_mv.construct(&builder)?;
        if result.mv_class != slf.expr_type {
            return None
        }
        builder.return_expr(result)
    });

    trait_impl_1_type_2_arg_i32!(PowiImpl(builder, slf, exp) -> MultiVector {
        let exp = FloatExpr::FromInt(exp.into());
        let result = Powf.inline(&builder, slf, exp).await?;
        builder.return_expr(result)
    });


    /*

Base:             AntiScalar( e12345(a12345) )
Accurate Square:  AntiScalar( e12345(((a12345 ^2))) )
Accurate Cube:    AntiScalar( e12345(((a12345 ^3))) )
Accurate 4th pow: AntiScalar( e12345(((a12345 ^4))) )
Accurate 5th pow: AntiScalar( e12345(((a12345 ^5))) )
Accurate 6th pow: AntiScalar( e12345(((a12345 ^6))) )

Base:             DualNum4( e4(a4), e12345(a12345) )
Accurate Square:  DualNum4( e4((a12345 * a4 * 2)), e12345(((a12345 ^2))) )
Accurate Cube:    DualNum4( e4(((a12345 ^2) * a4 * 3)), e12345(((a12345 ^3))) )
Accurate 4th pow: DualNum4( e4(((a12345 ^3) * a4 * 4)), e12345(((a12345 ^4))) )
Accurate 5th pow: DualNum4( e4(((a12345 ^4) * a4 * 5)), e12345(((a12345 ^5))) )
Accurate 6th pow: DualNum4( e4(((a12345 ^5) * a4 * 6)), e12345(((a12345 ^6))) )

Base:             DualNum5( e5(a5), e12345(a12345) )
Accurate Square:  DualNum5( e5((a12345 * a5 * 2)), e12345(((a12345 ^2))) )
Accurate Cube:    DualNum5( e5(((a12345 ^2) * a5 * 3)), e12345(((a12345 ^3))) )
Accurate 4th pow: DualNum5( e5(((a12345 ^3) * a5 * 4)), e12345(((a12345 ^4))) )
Accurate 5th pow: DualNum5( e5(((a12345 ^4) * a5 * 5)), e12345(((a12345 ^5))) )
Accurate 6th pow: DualNum5( e5(((a12345 ^5) * a5 * 6)), e12345(((a12345 ^6))) )

Base:             DualNum321( e321(a321), e12345(a12345) )
Accurate Square:  DualNum321(
    e321((a12345 * a321 * 2)),
    e12345((((a12345 ^2)) + ((a321 ^2)))) )
Accurate Cube:    DualNum321(
    e321((((a321 ^3)) + 3*((a12345 ^2) * a321))),
    e12345((((a12345 ^3)) + 3*(a12345 * (a321 ^2)))) )
Accurate 4th pow: DualNum321(
    e321((4*(a12345 * (a321 ^3)) + 4*((a12345 ^3) * a321))),
    e12345((((a12345 ^4)) + ((a321 ^4)) + 6*((a12345 ^2) * (a321 ^2)))) )
Accurate 5th pow: DualNum321(
    e321((((a321 ^5)) + 10*((a12345 ^2) * (a321 ^3)) + 5*((a12345 ^4) * a321))),
    e12345((((a12345 ^5)) + 5*(a12345 * (a321 ^4)) + 10*((a12345 ^3) * (a321 ^2)))) )
Accurate 6th pow: DualNum321(
    e321((6*(a12345 * (a321 ^5)) + 20*((a12345 ^3) * (a321 ^3)) + 6*((a12345 ^5) * a321))),
    e12345((((a12345 ^6)) + ((a321 ^6)) + 15*((a12345 ^2) * (a321 ^4)) + 15*((a12345 ^4) * (a321 ^2)))) )

Base:             TripleNum( e4(a4), e5(a5), e12345(a12345) )
Accurate Square:  TripleNum(
    e4((a12345 * a4 * 2)),
    e5((a12345 * a5 * 2)),
    e12345((((a12345 ^2)) + 2*(a4 * a5))) )
Accurate Cube:    TripleNum(
    e4((3*((a12345 ^2) * a4) + 2*((a4 ^2) * a5))),
    e5((2*(a4 * (a5 ^2)) + 3*((a12345 ^2) * a5))),
    e12345((((a12345 ^3)) + 6*(a12345 * a4 * a5))) )
Accurate 4th pow: TripleNum(
    e4((4*((a12345 ^3) * a4) + 8*(a12345 * (a4 ^2) * a5))),
    e5((4*((a12345 ^3) * a5) + 8*(a12345 * a4 * (a5 ^2)))),
    e12345((((a12345 ^4)) + 4*((a4 ^2) * (a5 ^2)) + 12*((a12345 ^2) * a4 * a5))) )
Accurate 5th pow: TripleNum(
    e4((4*((a4 ^3) * (a5 ^2)) + 5*((a12345 ^4) * a4) + 20*((a12345 ^2) * (a4 ^2) * a5))),
    e5((4*((a4 ^2) * (a5 ^3)) + 5*((a12345 ^4) * a5) + 20*((a12345 ^2) * a4 * (a5 ^2)))),
    e12345((((a12345 ^5)) + 20*(a12345 * (a4 ^2) * (a5 ^2)) + 20*((a12345 ^3) * a4 * a5))) )
Accurate 6th pow: TripleNum(
    e4((6*((a12345 ^5) * a4) + 24*(a12345 * (a4 ^3) * (a5 ^2)) + 40*((a12345 ^3) * (a4 ^2) * a5))),
    e5((6*((a12345 ^5) * a5) + 24*(a12345 * (a4 ^2) * (a5 ^3)) + 40*((a12345 ^3) * a4 * (a5 ^2)))),
    e12345((((a12345 ^6)) + 8*((a4 ^3) * (a5 ^3)) + 60*((a12345 ^2) * (a4 ^2) * (a5 ^2)) + 30*((a12345 ^4) * a4 * a5))) )

Base:             QuadNum( e4(a4), e5(a5), e321(a321), e12345(a12345) )
Accurate Square:  QuadNum(
    e4((a12345 * a4 * 2)),
    e5((a12345 * a5 * 2)),
    e321((a12345 * a321 * 2)),
    e12345((((a12345 ^2)) + ((a321 ^2)) + 2*(a4 * a5))) )
Accurate Cube:    QuadNum(
    e4((3*((a12345 ^2) * a4) + ((a321 ^2) * a4) + 2*((a4 ^2) * a5))),
    e5((2*(a4 * (a5 ^2)) + 3*((a12345 ^2) * a5) + ((a321 ^2) * a5))),
    e321((((a321 ^3)) + 3*((a12345 ^2) * a321) + 2*(a321 * a4 * a5))),
    e12345((((a12345 ^3)) + 3*(a12345 * (a321 ^2)) + 6*(a12345 * a4 * a5))) )
Accurate 4th pow: QuadNum(
    e4((4*((a12345 ^3) * a4) + 4*(a12345 * (a321 ^2) * a4) + 8*(a12345 * (a4 ^2) * a5))),
    e5((4*((a12345 ^3) * a5) + 8*(a12345 * a4 * (a5 ^2)) + 4*(a12345 * (a321 ^2) * a5))),
    e321((4*(a12345 * (a321 ^3)) + 4*((a12345 ^3) * a321) + 8*(a12345 * a321 * a4 * a5))),
    e12345((((a12345 ^4)) + ((a321 ^4)) + 6*((a12345 ^2) * (a321 ^2)) + 4*((a4 ^2) * (a5 ^2)) + 12*((a12345 ^2) * a4 * a5) + 4*((a321 ^2) * a4 * a5))) )
Accurate 5th pow: QuadNum(
    e4((4*((a4 ^3) * (a5 ^2)) + 5*((a12345 ^4) * a4) + ((a321 ^4) * a4) + 10*((a12345 ^2) * (a321 ^2) * a4) + 20*((a12345 ^2) * (a4 ^2) * a5) + 4*((a321 ^2) * (a4 ^2) * a5))),
    e5((4*((a4 ^2) * (a5 ^3)) + 5*((a12345 ^4) * a5) + ((a321 ^4) * a5) + 20*((a12345 ^2) * a4 * (a5 ^2)) + 10*((a12345 ^2) * (a321 ^2) * a5) + 4*((a321 ^2) * a4 * (a5 ^2)))),
    e321((((a321 ^5)) + 10*((a12345 ^2) * (a321 ^3)) + 5*((a12345 ^4) * a321) + 4*(a321 * (a4 ^2) * (a5 ^2)) + 4*((a321 ^3) * a4 * a5) + 20*((a12345 ^2) * a321 * a4 * a5))),
    e12345((((a12345 ^5)) + 5*(a12345 * (a321 ^4)) + 10*((a12345 ^3) * (a321 ^2)) + 20*(a12345 * (a4 ^2) * (a5 ^2)) + 20*((a12345 ^3) * a4 * a5) + 20*(a12345 * (a321 ^2) * a4 * a5))) )
Accurate 6th pow: QuadNum(
    e4((6*((a12345 ^5) * a4) + 24*(a12345 * (a4 ^3) * (a5 ^2)) + 6*(a12345 * (a321 ^4) * a4) + 20*((a12345 ^3) * (a321 ^2) * a4) + 40*((a12345 ^3) * (a4 ^2) * a5) + 24*(a12345 * (a321 ^2) * (a4 ^2) * a5))),
    e5((6*((a12345 ^5) * a5) + 24*(a12345 * (a4 ^2) * (a5 ^3)) + 6*(a12345 * (a321 ^4) * a5) + 40*((a12345 ^3) * a4 * (a5 ^2)) + 20*((a12345 ^3) * (a321 ^2) * a5) + 24*(a12345 * (a321 ^2) * a4 * (a5 ^2)))),
    e321((6*(a12345 * (a321 ^5)) + 20*((a12345 ^3) * (a321 ^3)) + 6*((a12345 ^5) * a321) + 24*(a12345 * a321 * (a4 ^2) * (a5 ^2)) + 24*(a12345 * (a321 ^3) * a4 * a5) + 40*((a12345 ^3) * a321 * a4 * a5))),
    e12345((((a12345 ^6)) + ((a321 ^6)) + 15*((a12345 ^2) * (a321 ^4)) + 8*((a4 ^3) * (a5 ^3)) + 15*((a12345 ^4) * (a321 ^2)) + 60*((a12345 ^2) * (a4 ^2) * (a5 ^2)) + 12*((a321 ^2) * (a4 ^2) * (a5 ^2)) + 30*((a12345 ^4) * a4 * a5) + 6*((a321 ^4) * a4 * a5) + 60*((a12345 ^2) * (a321 ^2) * a4 * a5))) )


    */

    trait_impl_1_type_2_arg_f32!(AntiPowfImpl(builder, slf, exp) -> MultiVector {
        // TODO after you finish fixing this, fix Powf, Powi, and AntiPowi also
        let exp: FloatExpr = exp.into();
        let mut dyn_mv = DynamicMultiVector::zero();

        let mut allowed_elements: Vec<BasisElement> = slf.elements_flat().map(|it| it.1).collect();
        let qty_elements = allowed_elements.len();
        if qty_elements > 8 {
            // We can't go too crazy.
            // For context, familiar 3D CGA using 5 dimensions has a VersorOdd and VersorEven each
            // with 16 elements, but the full MultiVector has 32 elements.
            // The significance of the number `8` or `16` or `32` here is that we have to get
            // every combination of coefficients. That is, while usually BasisElements don't
            // end up paired together because they cancel, the coefficients beside each BasisElement
            // do (in these Pow functions) because there are different ways to converge to an
            // (Anti)Scalar at different powers. So if a DualNum has 2 elements, then we might be
            // concerned about 2^2=4 combinations of FloatExpr. If a QuadNum has 4 elements, then we
            // might be concerned about 2^4=16 combinations of FloatExpr. A multivector type with
            // 16 different BasisElements will require 2^16 which is roughly 65,536 combinations of
            // FloatExpr. Anything higher is too crazy, with 2^24 in the millions and 2^32 in the
            // billions. Keep in mind all this is supposed to end up printed to a source file.
            // For a 16 element multivector, each BasisElement in the result of the Powf could
            // be the sum of 65k products. 65k * 16 elements = 1 million product terms in lots of
            // sums, where each product is several many bytes. You know what.... screw it. We are
            // limiting it to 8 element multivectors. 8 * 2^8 = 2048 product terms in a multivector
            // construction. Nobody asked for powf on anything beyond DualNum anyway (except me),
            // so don't lose sleep over it.
            return None;
        }
        // allowed_elements.sort();
        // let allowed_elements = allowed_elements;
        // if allowed_elements.binary_search(&builder.ga.anti_scalar()).is_err() {
        //     return None;
        // }
        //
        // let mut elements_and_stuff: Vec<_> = slf.elements_flat().collect();
        // elements_and_stuff.sort_by_key(|a| a.1)
        // let elements_and_stuff: Vec<_> = elements_and_stuff.into_iter().enumerate()
        //     .map(|(idx, (expr, el))| {
        //         let bits = 1u8 << idx;
        //         (el, bits, expr)
        //     }).collect();
        //
        // for product_term_idx in 1u8..(1u8 << qty_elements) {
        //     let qty_factors = product_term_idx.count_ones();
        //     let mut factors_here: Vec<(FloatExpr, f32)> = vec![];
        //     //
        // }


        let r = slf.clone();
        let anti_scalar = builder.ga.anti_scalar();
        for (a, a_el) in slf.elements_flat() {
            for (b, b_el) in r.elements_flat() {
                let sop = builder.ga.anti_product(a_el, b_el);
                for p in sop.sum {
                    if allowed_elements.binary_search(&p.element).is_err() {
                        return None;
                    }

                    let f = if a_el == b_el {
                        let a_exp = FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp.clone())), 1.0);
                        let b_exp = FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp.clone())), 1.0);
                        FloatExpr::Product(vec![(a_exp, 0.5), (b_exp, 0.5)], p.coefficient)
                    } else {
                        let mut f = exp.clone() * p.coefficient * 0.5;
                        let exp_minus_one = FloatExpr::Sum(vec![(exp.clone(), 1.0)], -1.0);
                        if a_el == anti_scalar {
                            f.mul_assign(FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                        } else {
                            f.mul_assign(a.clone());
                        }
                        if b_el == anti_scalar {
                            f.mul_assign(FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                        } else {
                            f.mul_assign(b.clone());
                        }
                        f
                    };
                    dyn_mv += (f, p.element);
                }
            }
        }
        let result = dyn_mv.construct(&builder)?;
        if result.mv_class != slf.expr_type {
            return None
        }
        builder.return_expr(result)
    });

    trait_impl_1_type_2_arg_i32!(AntiPowiImpl(builder, slf, exp) -> MultiVector {
        let exp = FloatExpr::FromInt(exp.into());
        let result = AntiPowf.inline(&builder, slf, exp).await?;
        builder.return_expr(result)
    });

    // TODO these violation traits should only be implemented where the constraint is possible to violate
    //  then also make a trait where it is always/only implemented where the constraint is impossible to violate
    trait_impl_1_type_1_arg!(ConstraintViolationImpl(builder, slf) -> MultiVector {
        let r = Reverse.inline(&builder, slf.clone()).await?;
        let p = GeometricProduct.inline(&builder, slf.clone(), r).await?;
        let d = ScalarProduct.inline(&builder, slf.clone(), slf.clone()).await?;
        let result = Subtraction.inline(&builder, p, d).await?;
        builder.return_expr(result)
    });
    trait_impl_1_type_1_arg!(AntiConstraintViolationImpl(builder, slf) -> MultiVector {
        let r = AntiReverse.inline(&builder, slf.clone()).await?;
        let p = GeometricAntiProduct.inline(&builder, slf.clone(), r).await?;
        let d = AntiScalarProduct.inline(&builder, slf.clone(), slf.clone()).await?;
        let result = Subtraction.inline(&builder, p, d).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ConstraintValidImpl(builder, slf) -> MultiVector {
        let r = Reverse.inline(&builder, slf.clone()).await?;
        let p = GeometricProduct.inline(&builder, slf.clone(), r).await;
        let d = ScalarProduct.inline(&builder, slf.clone(), slf.clone()).await;
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
        let d = AntiScalarProduct.inline(&builder, slf.clone(), slf.clone()).await;
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
        let dot = ScalarProduct.inline(&builder, slf.clone(), slf).await?;
        let raw_dot = FloatExpr::AccessMultiVecFlat(dot.into(), 0);
        let result = scalar_mv.construct_direct([(scalar, FloatExpr::Product(vec![(raw_dot, -1.0)], 1.0))]);
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiInverseImpl(builder, slf) -> MultiVector {
        let anti_scalar_mv = MultiVector::from(builder.mvs.anti_scalar());
        let anti_scalar = builder.ga.anti_scalar();
        let dot = AntiScalarProduct.inline(&builder, slf.clone(), slf).await?;
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

    // TODO this is just a placeholder to help me figure out powi and then powf and then sqrt
    trait_impl_1_type_1_arg!(SquareImpl(builder, slf) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        // let r = Reverse.inline(&builder, slf.clone()).await?;
        let r = slf.clone();
        for (a, a_el) in slf.elements_flat() {
            for (b, b_el) in r.elements_flat() {
                let sop = builder.ga.product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let result = dyn_mv.construct(&builder)?;
        builder.return_expr(result)
    });
    trait_impl_1_type_1_arg!(AntiSquareImpl(builder, slf) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        // let r = AntiReverse.inline(&builder, slf.clone()).await?;
        let r = slf.clone();
        for (a, a_el) in slf.elements_flat() {
            for (b, b_el) in r.elements_flat() {
                let sop = builder.ga.anti_product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let result = dyn_mv.construct(&builder)?;
        builder.return_expr(result)
    });

/*
impl AntiSquare for QuadNum {
    type Output = QuadNum;
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            (self[e4] * self[e12345] * 2.0),
            (self[e5] * self[e12345] * 2.0),
            (self[e321] * self[e12345] * 2.0),
            (f32::powi(self[e321], 2) + f32::powi(self[e12345], 2) + 2.0 * (self[e4] * self[e5])),
        ]));
    }
}
impl AntiSquare for DualNum5 {
    type Output = DualNum5;
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum5::from_groups(
            // e5, e12345
            (Simd32x2::from([
                (self[e5] * self[e12345]),
                f32::powi(self[e12345], 2)
            ]) * Simd32x2::from([2.0, 1.0])),
        );
    }
}
impl AntiSquare for DualNum4 {
    type Output = DualNum4;
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum4::from_groups(
            // e4, e12345
            (Simd32x2::from([(self[e4] * self[e12345]),
            f32::powi(self[e12345], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
    }
}
impl AntiSquare for DualNum321 {
    type Output = DualNum321;
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum321::from_groups(
            // e321, e12345
            Simd32x2::from([(self[e321] * self[e12345] * 2.0),
            (f32::powi(self[e321], 2) + f32::powi(self[e12345], 2))]),
        );
    }
}impl AntiSquare for TripleNum {
    type Output = TripleNum;
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return TripleNum::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            (self[e4] * self[e12345] * 2.0),
            (self[e5] * self[e12345] * 2.0),
            (f32::powi(self[e12345], 2) + 2.0 * (self[e4] * self[e5])),
        ]));
    }
}
*/

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
        // TODO inline again after getting Rust emission import fixed
        let dual = RightDual.invoke(&mut builder, other).await?;
        let wedge = Wedge.invoke(&mut builder, slf, dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(WeightExpansionImpl(builder, slf, other) -> MultiVector {
        // TODO inline again after getting Rust emission import fixed
        let anti_dual = RightAntiDual.invoke(&mut builder, other).await?;
        let wedge = Wedge.invoke(&mut builder, slf, anti_dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(BulkContractionImpl(builder, slf, other) -> MultiVector {
        // TODO inline again after getting Rust emission import fixed
        let dual = RightDual.invoke(&mut builder, other).await?;
        let anti_wedge = AntiWedge.invoke(&mut builder, slf, dual).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_types_2_args!(WeightContractionImpl(builder, slf, other) -> MultiVector {
        // TODO inline again after getting Rust emission import fixed
        let anti_dual = RightAntiDual.invoke(&mut builder, other).await?;
        let anti_wedge = AntiWedge.invoke(&mut builder, slf, anti_dual).await?;
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
