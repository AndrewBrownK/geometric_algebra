#![allow(non_upper_case_globals)]

use crate::algebra2::basis::grades::{AntiGrades, Grades};
use crate::ast2::impls::{Elaborated, InlineOnly};
use crate::ast2::traits::{NameTrait, TraitDef_1_Param_1_Arg, TraitImpl_10, TraitImpl_11, TraitImpl_21};
use crate::build_scripts2::common_traits::impls::*;

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
    .blurb(
    "A multivector class may have uniform grade, or mixed grade, depending on \
    the grades of its elements. This trait only characterizes uniform grade multivectors.",
);
pub static AntiGrade: Elaborated<AntiGradeImpl> = AntiGradeImpl
    .new_trait_named("AntiGrade")
    .blurb(
    "The AntiGrade can be described as the missing Grade with respect to an \
    AntiScalar. This trait only characterizes uniform anti-grade multivectors.",
);

pub static Dual: Elaborated<DualImpl> = DualImpl
    .new_trait_named("Dual")
    .blurb("TODO");

pub static AntiDual: Elaborated<AntiDualImpl> = AntiDualImpl
    .new_trait_named("AntiDual")
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

pub static ScalarProduct: Elaborated<ScalarProductImpl> = ScalarProductImpl
    .new_trait_named("ScalarProduct")
    .blurb("TODO");

pub static AntiScalarProduct: Elaborated<AntiScalarProductImpl> = AntiScalarProductImpl
    .new_trait_named("AntiScalarProduct")
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

// NOTE: If you find yourself wanting to generate grade selection traits, you are
// probably generating extremely wasteful implementations that perform a lot more
// floating point calculations than necessary. That is why these trait definitions
// are InlineOnly. They will not generate trait definitions, but you can still invoke
// them as if they were traits, and they will always be inlined instead. (With inlining,
// terms that are unused by the final result of each function will be cut out of the AST
// automatically. With grade selection declarations (not inlining strictly always), then
// you'd have the chance to use grade selection in application code which is very undesirable.)
// TODO actually cut out unused terms and variables, as claimed above

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

    use crate::algebra2::basis::grades::{AntiGrades, Grades};
    use crate::algebra2::basis::{BasisElement, BasisSignature};
    use crate::algebra2::multivector::DynamicMultiVector;
    use crate::ast2::datatype::{Integer, MultiVector};
    use crate::ast2::expressions::{Expression, FloatExpr, IntExpr};
    use crate::ast2::traits::{HasNotReturned, TraitDef_1_Param_1_Arg, TraitDef_2_Param_2_Arg, TraitImplBuilder, TraitImpl_10, TraitImpl_11, TraitImpl_21, TraitImpl_22};
    use crate::ast2::Variable;
    use crate::build_scripts2::common_traits::{AntiDual, AntiReverse, AntiWedge, Dual, GeometricAntiProduct, GeometricProduct, Reverse, Wedge};

    macro_rules! impl_trait {
        ($trait_name:ident, $fn_name:ident, $param:ident, $body:tt) => {
            impl $trait_name for MyStruct {
                fn $fn_name(&self, $param: u32) {
                    $body
                }
            }
        };
    }

    trait MyTrait {
        fn my_function(&self, param: u32);
    }

    struct MyStruct;
    impl_trait!(MyTrait, my_function, param, {
        println!("The parameter is: {}", param);
    });



    #[macro_export]
    macro_rules! trait_impl_1_param_0_arg {
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
    macro_rules! trait_impl_1_param_1_arg {
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
    macro_rules! trait_impl_2_param_1_arg {
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
    macro_rules! trait_impl_2_param_2_arg {
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

    trait_impl_1_param_0_arg!(ZeroImpl(builder, owner) -> MultiVector {
        builder.return_expr(owner.construct(|_| FloatExpr::Literal(0.0)))
    });

    trait_impl_1_param_0_arg!(OneImpl(builder, owner) -> MultiVector {
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

    trait_impl_1_param_0_arg!(AntiOneImpl(builder, owner) -> MultiVector {
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

    trait_impl_1_param_0_arg!(UnitImpl(builder, owner) -> MultiVector {
        builder.return_expr(owner.construct(|_| FloatExpr::Literal(1.0)))
    });

    trait_impl_1_param_0_arg!(GradeImpl(builder, owner) -> Integer {
        let gr = owner.grade()?;
        builder.return_expr(IntExpr::Literal(gr))
    });

    trait_impl_1_param_0_arg!(AntiGradeImpl(builder, owner) -> Integer {
        let ag = owner.anti_grade()?;
        builder.return_expr(IntExpr::Literal(ag))
    });

    trait_impl_1_param_1_arg!(DualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_param_1_arg!(AntiDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.anti_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_param_1_arg!(ReverseImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.reverse(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_param_1_arg!(AntiReverseImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.anti_reverse(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_param_1_arg!(AutoMorphismImpl(builder, slf) -> MultiVector {
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

    trait_impl_1_param_1_arg!(AntiAutoMorphismImpl(builder, slf) -> MultiVector {
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

    trait_impl_1_param_1_arg!(ConjugationImpl(builder, slf) -> MultiVector {
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

    trait_impl_1_param_1_arg!(AntiConjugationImpl(builder, slf) -> MultiVector {
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

    trait_impl_1_param_1_arg!(RightComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.fix_name_and_sign(el.right_complement(AntiScalar));
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_param_1_arg!(LeftComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.fix_name_and_sign(el.left_complement(AntiScalar));
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_param_1_arg!(DoubleComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            let el = el.right_complement(AntiScalar).right_complement(AntiScalar);
            let (f, el) = builder.ga.fix_name_and_sign(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_param_1_arg!(NegationImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements_by_groups() {
            result += (fe * -1.0, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_2_param_2_arg!(AdditionImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(SubtractionImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(WedgeImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(AntiWedgeImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(GeometricProductImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(GeometricAntiProductImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(SandwichImpl(builder, slf, other) -> MultiVector {
        // TODO incorrect cycle detection if use all invoke
        let c = GeometricProduct.inline(&builder, slf.clone(), other).await?;
        let r = Reverse.invoke(&mut builder, slf).await?;
        let result = GeometricProduct.invoke(&mut builder, c, r).await?;
        builder.return_expr(result)
    });

    trait_impl_2_param_2_arg!(AntiSandwichImpl(builder, slf, other) -> MultiVector {
        // TODO incorrect cycle detection if use all invoke
        let c = GeometricAntiProduct.inline(&builder, slf.clone(), other).await?;
        let r = AntiReverse.invoke(&mut builder, slf).await?;
        let result = GeometricAntiProduct.invoke(&mut builder, c, r).await?;
        builder.return_expr(result)
    });

    trait_impl_2_param_2_arg!(ScalarProductImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(AntiScalarProductImpl(builder, slf, other) -> MultiVector {
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

    trait_impl_2_param_2_arg!(BulkExpansionImpl(builder, slf, other) -> MultiVector {
        // TODO inline again after getting Rust emission import fixed
        let dual = Dual.invoke(&mut builder, other).await?;
        let wedge = Wedge.invoke(&mut builder, slf, dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_param_2_arg!(WeightExpansionImpl(builder, slf, other) -> MultiVector {
        // TODO inline again after getting Rust emission import fixed
        let anti_dual = AntiDual.invoke(&mut builder, other).await?;
        let wedge = Wedge.invoke(&mut builder, slf, anti_dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_param_2_arg!(BulkContractionImpl(builder, slf, other) -> MultiVector {
        // TODO inline again after getting Rust emission import fixed
        let dual = Dual.invoke(&mut builder, other).await?;
        let anti_wedge = AntiWedge.invoke(&mut builder, slf, dual).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_param_2_arg!(WeightContractionImpl(builder, slf, other) -> MultiVector {
        // TODO inline again after getting Rust emission import fixed
        let anti_dual = AntiDual.invoke(&mut builder, other).await?;
        let anti_wedge = AntiWedge.invoke(&mut builder, slf, anti_dual).await?;
        builder.return_expr(anti_wedge)
    });

    // Into is treated kind of special, because we actually want to implement From,
    // but the TraitImpl_21 pattern assumes the first argument is the owner.
    // So in the code generation we have a special exception to treat Into as From instead.
    trait_impl_2_param_1_arg!(IntoImpl(builder, slf, other) -> MultiVector {
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
    trait_impl_2_param_1_arg!(TryIntoImpl(builder, slf, other) -> MultiVector {
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
