#![allow(non_upper_case_globals)]

use crate::algebra2::basis::grades::{AntiGrades, Grades};
use crate::ast2::impls::{Elaborated, InlineOnly};
use crate::ast2::traits::{NameTrait, TraitDef_1Class_1Param, TraitImpl_10, TraitImpl_11, TraitImpl_21};
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

pub static Dual: Elaborated<DualImpl> = DualImpl.new_trait_named("Dual").blurb("TODO");

pub static AntiDual: Elaborated<AntiDualImpl> = AntiDualImpl.new_trait_named("AntiDual").blurb("TODO");

pub static Reverse: Elaborated<ReverseImpl> = ReverseImpl.new_trait_named("Reverse").blurb("TODO");

pub static AntiReverse: Elaborated<AntiReverseImpl> = AntiReverseImpl.new_trait_named("AntiReverse").blurb("TODO");

pub static Wedge: Elaborated<WedgeImpl> = WedgeImpl.new_trait_named("Wedge").blurb("TODO");
pub static AntiWedge: Elaborated<AntiWedgeImpl> = AntiWedgeImpl.new_trait_named("AntiWedge").blurb("TODO");

pub static GeometricProduct: Elaborated<GeometricProductImpl> = GeometricProductImpl.new_trait_named("GeometricProduct").blurb("TODO");

pub static GeometricAntiProduct: Elaborated<GeometricAntiProductImpl> = GeometricAntiProductImpl.new_trait_named("GeometricAntiProduct").blurb("TODO");

pub static Sandwich: Elaborated<SandwichImpl> = SandwichImpl.new_trait_named("Sandwich").blurb("TODO");

pub static AntiSandwich: Elaborated<AntiSandwichImpl> = AntiSandwichImpl.new_trait_named("AntiSandwich").blurb("TODO");

pub static ScalarProduct: Elaborated<ScalarProductImpl> = ScalarProductImpl.new_trait_named("ScalarProduct").blurb("TODO");

pub static AntiScalarProduct: Elaborated<AntiScalarProductImpl> = AntiScalarProductImpl.new_trait_named("AntiScalarProduct").blurb("TODO");

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
    use crate::ast2::traits::{HasNotReturned, TraitDef_1Class_1Param, TraitDef_2Class_2Param, TraitImplBuilder, TraitImpl_10, TraitImpl_11, TraitImpl_21, TraitImpl_22};
    use crate::ast2::Variable;
    use crate::build_scripts2::common_traits::{AntiDual, AntiReverse, AntiWedge, Dual, GeometricAntiProduct, GeometricProduct, Reverse, Wedge};

    #[derive(Clone, Copy)]
    pub struct ZeroImpl;
    #[async_trait]
    impl TraitImpl_10 for ZeroImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            owner: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            b.return_expr(owner.construct(|_| FloatExpr::Literal(0.0)))
        }
    }

    #[derive(Clone, Copy)]
    pub struct OneImpl;
    #[async_trait]
    impl TraitImpl_10 for OneImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            owner: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            if !owner.elements().into_iter().any(|el| el.signature() == BasisSignature::scalar) {
                return None;
            }
            b.return_expr(owner.construct(|element| {
                if element.signature() == BasisSignature::scalar {
                    FloatExpr::Literal(1.0)
                } else {
                    FloatExpr::Literal(0.0)
                }
            }))
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiOneImpl;
    #[async_trait]
    impl TraitImpl_10 for AntiOneImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            owner: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            if !owner.elements().into_iter().any(|el| el.signature() == AntiScalar.signature()) {
                return None;
            }
            b.return_expr(owner.construct(|element| {
                if element.signature() == AntiScalar.signature() {
                    FloatExpr::Literal(1.0)
                } else {
                    FloatExpr::Literal(0.0)
                }
            }))
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitImpl;
    #[async_trait]
    impl TraitImpl_10 for UnitImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            owner: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            b.return_expr(owner.construct(|_| FloatExpr::Literal(1.0)))
        }
    }

    #[derive(Clone, Copy)]
    pub struct GradeImpl;
    #[async_trait]
    impl TraitImpl_10 for GradeImpl {
        type Output = Integer;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            owner: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let gr = owner.grade()?;
            b.return_expr(IntExpr::Literal(gr))
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiGradeImpl;
    #[async_trait]
    impl TraitImpl_10 for AntiGradeImpl {
        type Output = Integer;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            owner: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ag = owner.anti_grade()?;
            b.return_expr(IntExpr::Literal(ag))
        }
    }

    #[derive(Clone, Copy)]
    pub struct DualImpl;
    #[async_trait]
    impl TraitImpl_11 for DualImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut result = DynamicMultiVector::zero();
            for (fe, el) in slf.elements_by_groups() {
                let (f, el) = b.ga.dual(el);
                result += (fe * f, el);
            }
            let result = result.construct(&b)?;
            b.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiDualImpl;
    #[async_trait]
    impl TraitImpl_11 for AntiDualImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut result = DynamicMultiVector::zero();
            for (fe, el) in slf.elements_by_groups() {
                let (f, el) = b.ga.anti_dual(el);
                result += (fe * f, el);
            }
            let result = result.construct(&b)?;
            b.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ReverseImpl;
    #[async_trait]
    impl TraitImpl_11 for ReverseImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut result = DynamicMultiVector::zero();
            for (fe, el) in slf.elements_by_groups() {
                let (f, el) = b.ga.reverse(el);
                result += (fe * f, el);
            }
            let result = result.construct(&b)?;
            b.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiReverseImpl;
    #[async_trait]
    impl TraitImpl_11 for AntiReverseImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut result = DynamicMultiVector::zero();
            for (fe, el) in slf.elements_by_groups() {
                let (f, el) = b.ga.anti_reverse(el);
                result += (fe * f, el);
            }
            let result = result.construct(&b)?;
            b.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct WedgeImpl;
    #[async_trait]
    impl TraitImpl_22 for WedgeImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                for (b, b_el) in other.elements_by_groups() {
                    let a = a.clone();
                    let (f, c) = ga.wedge(a_el, b_el);
                    dyn_mv += (a * b * f, c);
                }
            }
            let mv = dyn_mv.construct(&b)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiWedgeImpl;
    #[async_trait]
    impl TraitImpl_22 for AntiWedgeImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                for (b, b_el) in other.elements_by_groups() {
                    let a = a.clone();
                    let (f, c) = ga.anti_wedge(a_el, b_el);
                    dyn_mv += (a * b * f, c);
                }
            }
            let mv = dyn_mv.construct(&b)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct GeometricProductImpl;
    #[async_trait]
    impl TraitImpl_22 for GeometricProductImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &builder.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                for (b, b_el) in other.elements_by_groups() {
                    let sop = ga.product(a_el, b_el);
                    for p in sop.sum {
                        let el = p.element;
                        let f = a.clone() * b.clone() * p.coefficient;
                        dyn_mv += (f, el);
                    }
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            builder.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct GeometricAntiProductImpl;
    #[async_trait]
    impl TraitImpl_22 for GeometricAntiProductImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &builder.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                for (b, b_el) in other.elements_by_groups() {
                    let sop = ga.anti_product(a_el, b_el);
                    for p in sop.sum {
                        let el = p.element;
                        let f = a.clone() * b.clone() * p.coefficient;
                        dyn_mv += (f, el);
                    }
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            builder.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct SandwichImpl;
    #[async_trait]
    impl TraitImpl_22 for SandwichImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO incorrect cycle detection if use all invoke
            let c = GeometricProduct.inline(&b, slf.clone(), other).await?;
            let r = Reverse.invoke(&mut b, slf).await?;
            let result = GeometricProduct.invoke(&mut b, c, r).await?;
            b.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiSandwichImpl;
    #[async_trait]
    impl TraitImpl_22 for AntiSandwichImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO incorrect cycle detection if use all invoke
            let c = GeometricAntiProduct.inline(&b, slf.clone(), other).await?;
            let r = AntiReverse.invoke(&mut b, slf).await?;
            let result = GeometricAntiProduct.invoke(&mut b, c, r).await?;
            b.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ScalarProductImpl;
    #[async_trait]
    impl TraitImpl_22 for ScalarProductImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                for (b, b_el) in other.elements_by_groups() {
                    let sop = ga.scalar_product(a_el, b_el);
                    for p in sop.sum {
                        let a = a.clone();
                        let b = b.clone();
                        let c = p.coefficient;
                        dyn_mv += (a * b * c, p.element);
                    }
                }
            }
            let mv = dyn_mv.construct(&b)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiScalarProductImpl;
    #[async_trait]
    impl TraitImpl_22 for AntiScalarProductImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                for (b, b_el) in other.elements_by_groups() {
                    let sop = ga.anti_scalar_product(a_el, b_el);
                    for p in sop.sum {
                        let a = a.clone();
                        let b = b.clone();
                        let c = p.coefficient;
                        dyn_mv += (a * b * c, p.element);
                    }
                }
            }
            let mv = dyn_mv.construct(&b)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct SelectGradesImpl(pub Grades);
    #[async_trait]
    impl TraitImpl_11 for SelectGradesImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                if self.0.contains(a_el.grades()) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&b)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct SelectAntiGradesImpl(pub AntiGrades);
    #[async_trait]
    impl TraitImpl_11 for SelectAntiGradesImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements_by_groups() {
                if self.0.contains(a_el.anti_grades(AntiScalar)) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&b)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct BulkExpansionImpl;
    #[async_trait]
    impl TraitImpl_22 for BulkExpansionImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO inline again after getting Rust emission import fixed
            let dual = Dual.invoke(&mut b, other).await?;
            let wedge = Wedge.invoke(&mut b, slf, dual).await?;
            b.return_expr(wedge)
        }
    }

    #[derive(Clone, Copy)]
    pub struct WeightExpansionImpl;
    #[async_trait]
    impl TraitImpl_22 for WeightExpansionImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO inline again after getting Rust emission import fixed
            let anti_dual = AntiDual.invoke(&mut b, other).await?;
            let wedge = Wedge.invoke(&mut b, slf, anti_dual).await?;
            b.return_expr(wedge)
        }
    }

    #[derive(Clone, Copy)]
    pub struct BulkContractionImpl;
    #[async_trait]
    impl TraitImpl_22 for BulkContractionImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO inline again after getting Rust emission import fixed
            let dual = Dual.invoke(&mut b, other).await?;
            let anti_wedge = AntiWedge.invoke(&mut b, slf, dual).await?;
            b.return_expr(anti_wedge)
        }
    }

    #[derive(Clone, Copy)]
    pub struct WeightContractionImpl;
    #[async_trait]
    impl TraitImpl_22 for WeightContractionImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO inline again after getting Rust emission import fixed
            let anti_dual = AntiDual.invoke(&mut b, other).await?;
            let anti_wedge = AntiWedge.invoke(&mut b, slf, anti_dual).await?;
            b.return_expr(anti_wedge)
        }
    }

    // Into is treated kind of special, because we actually want to implement From,
    // but the TraitImpl_21 pattern assumes the first argument is the owner.
    // So in the code generation we have a special exception to treat Into as From instead.
    #[derive(Clone, Copy)]
    pub struct IntoImpl;
    #[async_trait]
    impl TraitImpl_21 for IntoImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
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
            b.return_expr(result)
        }
    }

    // TryInto is treated kind of special, because we actually want to implement TryFrom,
    // but the TraitImpl_21 pattern assumes the first argument is the owner.
    // So in the code generation we have a special exception to treat TrInto as TryFrom instead.
    #[derive(Clone, Copy)]
    pub struct TryIntoImpl;
    #[async_trait]
    impl TraitImpl_21 for TryIntoImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: MultiVector,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
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
            b.return_expr(result)
        }
    }
}
