#![allow(non_upper_case_globals)]

use crate::algebra2::basis::grades::{AntiGrades, Grades};
use crate::ast2::impls::{Elaborated, InlineOnly};
use crate::ast2::traits::{NameTrait, TraitDef_1Class_1Param, TraitImpl_10, TraitImpl_11};
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
    .blurb("A multivector class may have uniform grade, or mixed grade, depending on \
    the grades of its elements. This trait only characterizes uniform grade multivectors.");
pub static AntiGrade: Elaborated<AntiGradeImpl> = AntiGradeImpl
    .new_trait_named("AntiGrade")
    .blurb("The AntiGrade can be described as the missing Grade with respect to an \
    AntiScalar. This trait only characterizes uniform anti-grade multivectors.");

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
    use async_trait::async_trait;

    use crate::algebra2::basis::{BasisElement, BasisSignature};
    use crate::algebra2::basis::grades::{AntiGrades, Grades};
    use crate::algebra2::multivector::DynamicMultiVector;
    use crate::ast2::datatype::{Integer, MultiVector};
    use crate::ast2::expressions::{FloatExpr, IntExpr};
    use crate::ast2::traits::{HasNotReturned, TraitImpl_10, TraitImpl_11, TraitImpl_22, TraitImplBuilder};
    use crate::ast2::Variable;

    #[derive(Clone, Copy)]
    pub struct ZeroImpl;
    #[async_trait]
    impl TraitImpl_10 for ZeroImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            owner: MultiVector
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
            owner: MultiVector
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
            owner: MultiVector
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
            owner: MultiVector
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
            owner: MultiVector
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
            owner: MultiVector
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ag = owner.anti_grade()?;
            b.return_expr(IntExpr::Literal(ag))
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
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let a = a.clone();
                    dyn_mv += (a * b, a_el.wedge(b_el));
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
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let a = a.clone();
                    dyn_mv += (a * b, a_el.anti_wedge(b_el, AntiScalar));
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
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let sop = ga.product(a_el, b_el);
                    for p in sop.sum {
                        let a = a.clone();
                        let b = b.clone();
                        let c = FloatExpr::Literal(p.coefficient);
                        dyn_mv += (a * b * c, p.element);
                    }
                }
            }
            let mv = dyn_mv.construct(&b)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct GeometricAntiProductImpl;
    #[async_trait]
    impl TraitImpl_22 for GeometricAntiProductImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let sop = ga.anti_product(a_el, b_el);
                    for p in sop.sum {
                        let a = a.clone();
                        let b = b.clone();
                        let c = FloatExpr::Literal(p.coefficient);
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
            slf: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
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
            slf: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
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
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO actual implementation
            b.return_expr(slf)
        }
    }

    #[derive(Clone, Copy)]
    pub struct WeightExpansionImpl;
    #[async_trait]
    impl TraitImpl_22 for WeightExpansionImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO actual implementation
            b.return_expr(slf)
        }
    }

    #[derive(Clone, Copy)]
    pub struct BulkContractionImpl;
    #[async_trait]
    impl TraitImpl_22 for BulkContractionImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO actual implementation
            b.return_expr(slf)
        }
    }

    #[derive(Clone, Copy)]
    pub struct WeightContractionImpl;
    #[async_trait]
    impl TraitImpl_22 for WeightContractionImpl {
        type Output = MultiVector;

        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            // TODO actual implementation
            b.return_expr(slf)
        }
    }

}