#![allow(non_upper_case_globals)]

use paste::paste;
use crate::algebra2::basis::elements::e12345;
use crate::ast2::datatype::{AnyClasses, MultiVector};
use crate::ast2::impls::Elaborated;
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
    AntiScalar.");

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


macro_rules! select_grades {
    ($($gr:literal),+ $(,)?) => {
        paste::paste! {$(
            pub static [<SelectGrade $gr>]: Elaborated<[<SelectGrade $gr Impl>]> = [<SelectGrade $gr Impl>]
                .new_trait_named(concat!("SelectGrade", $gr))
                .blurb(concat!("Select only the portion of a multivector with grade ", $gr, "."));
        )+}
        paste::paste! {$(
            pub static [<SelectAntiGrade $gr>]: Elaborated<[<SelectAntiGrade $gr Impl>]> = [<SelectAntiGrade $gr Impl>]
                .new_trait_named(concat!("SelectAntiGrade", $gr))
                .blurb(concat!("Select only the portion of a multivector with anti_grade ", $gr, "."));
        )+}
    };
}
select_grades!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

// TODO I really want a SelectGradeN(u32) type
//  Or in other words, something like...
//  pub static SelectGrade: [Elaborated<SelectGradeN>; 17] = [...];
//  The key idea is that, in some other function, we will have runtime gr_a and gr_b, and
//  then do some runtime arithmetic like (gr_a + gr_b) or (gr_a - gr_b). Hell, we could
//  even do SelectMultipleGrades(Grades). Anyway, the reason we can't do something like
//  this currently is this...
//  - SelectGrade1, SelectGrade2, etc all have different types, so can't be put into the same
//    array or returned from the same function
//  - &'static dyn TraitDef_1Class_1Param is very unlikely to work, because it is very unlikely
//    that this trait is object-safe. WEELLLL... I certainly can't declare it statically for
//    all possible AntiScalars while the AntiScalar is trait-level. But if I switch it to be
//    method-level... yeah, same problem, needs to be Copy and Sized.
//  - We could put SelectGrade1, SelectGrade2, etc into a tuple, but again the problem is we
//    need the ability to choose on of them dynamically at runtime without having to handle
//    multiple types.
//  So ultimately, it looks like there is only one solution. And this solution might open other
//  possibilities too. We introduce self to general_implementation. This way runtime data can
//  get carried into the implementation. Despite this, in order for TraitDefs to get referenced
//  inside TraitImpls, they still have to be static items. So that means the TraitImpls
//  still need to be const evaluated, which in all likelihood makes them Copy as well. And besides,
//  TraitImpls are required to be Copy anyway. So yes, the side benefits of this is that it can pave
//  the way for more precise specification of the Owner and Other types, which up to this point we
//  did not have a grip on yet. We had no idea how to return runtime data for that yet.

// pub static THING: &'static dyn TraitDef_1Class_1Param<Owner=AnyClasses, Output=MultiVector> = &SelectGrade0;




mod impls {
    use async_trait::async_trait;

    use crate::algebra2::basis::{BasisElement, BasisSignature};
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
        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            owner: MultiVector
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            b.return_expr(owner.construct(|_| FloatExpr::Zero))
        }
    }

    #[derive(Clone, Copy)]
    pub struct OneImpl;
    #[async_trait]
    impl TraitImpl_10 for OneImpl {
        type Output = MultiVector;
        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            owner: MultiVector
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            b.return_expr(owner.construct(|element| {
                if element.signature() == BasisSignature::scalar {
                    FloatExpr::One
                } else {
                    FloatExpr::Zero
                }
            }))
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiOneImpl;
    #[async_trait]
    impl TraitImpl_10 for AntiOneImpl {
        type Output = MultiVector;
        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            owner: MultiVector
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            b.return_expr(owner.construct(|element| {
                if element.signature() == AntiScalar.signature() {
                    FloatExpr::One
                } else {
                    FloatExpr::Zero
                }
            }))
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitImpl;
    #[async_trait]
    impl TraitImpl_10 for UnitImpl {
        type Output = MultiVector;
        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            owner: MultiVector
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            b.return_expr(owner.construct(|_| FloatExpr::One))
        }
    }

    #[derive(Clone, Copy)]
    pub struct GradeImpl;
    #[async_trait]
    impl TraitImpl_10 for GradeImpl {
        type Output = Integer;
        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            owner: MultiVector
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            let gr = owner.grade()?;
            b.return_expr(IntExpr::Literal(gr))
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiGradeImpl;
    #[async_trait]
    impl TraitImpl_10 for AntiGradeImpl {
        type Output = Integer;
        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            owner: MultiVector
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            let ag = owner.anti_grade(AntiScalar)?;
            b.return_expr(IntExpr::Literal(ag))
        }
    }

    #[derive(Clone, Copy)]
    pub struct WedgeImpl;
    #[async_trait]
    impl TraitImpl_22 for WedgeImpl {
        type Output = MultiVector;
        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let a = a.clone();
                    dyn_mv += (a * b, a_el.wedge(b_el));
                }
            }
            let mv = dyn_mv.construct(&b.mvs)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiWedgeImpl;
    #[async_trait]
    impl TraitImpl_22 for AntiWedgeImpl {
        type Output = MultiVector;

        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let a = a.clone();
                    dyn_mv += (a * b, a_el.anti_wedge(b_el, AntiScalar));
                }
            }
            let mv = dyn_mv.construct(&b.mvs)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct GeometricProductImpl;
    #[async_trait]
    impl TraitImpl_22 for GeometricProductImpl {
        type Output = MultiVector;

        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let sop = ga.product(a_el, b_el);
                    for p in sop.sum {
                        let a = a.clone();
                        let b = b.clone();
                        let c = FloatExpr::Lit(p.coefficient);
                        dyn_mv += (a * b * c, p.element);
                    }
                }
            }
            let mv = dyn_mv.construct(&b.mvs)?;
            b.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct GeometricAntiProductImpl;
    #[async_trait]
    impl TraitImpl_22 for GeometricAntiProductImpl {
        type Output = MultiVector;

        async fn general_implementation<'impls, const AntiScalar: BasisElement>(
            b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
            other: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
            let ga = &b.ga;
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                for (b, b_el) in other.elements() {
                    let sop = ga.anti_product(a_el, b_el);
                    for p in sop.sum {
                        let a = a.clone();
                        let b = b.clone();
                        let c = FloatExpr::Lit(p.coefficient);
                        dyn_mv += (a * b * c, p.element);
                    }
                }
            }
            let mv = dyn_mv.construct(&b.mvs)?;
            b.return_expr(mv)
        }
    }

    macro_rules! select_grade_impls {
        ($($gr:expr),+ $(,)?) => {
            paste::paste! {
                $(
                #[derive(Clone, Copy)]
                pub struct [<SelectGrade $gr Impl>];
                #[async_trait]
                impl TraitImpl_11 for [<SelectGrade $gr Impl>] {
                    type Output = MultiVector;

                    async fn general_implementation<'impls, const AntiScalar: BasisElement>(
                        b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
                        slf: Variable<MultiVector>,
                    ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
                        let mut dyn_mv = DynamicMultiVector::zero();
                        for (a, a_el) in slf.elements() {
                            if a_el.grade() == $gr {
                                dyn_mv += (a, a_el);
                            }
                        }
                        let mv = dyn_mv.construct(&b.mvs)?;
                        b.return_expr(mv)
                    }
                }
                )+
            }
            paste::paste! {
                $(
                #[derive(Clone, Copy)]
                pub struct [<SelectAntiGrade $gr Impl>];
                #[async_trait]
                impl TraitImpl_11 for [<SelectAntiGrade $gr Impl>] {
                    type Output = MultiVector;

                    async fn general_implementation<'impls, const AntiScalar: BasisElement>(
                        b: TraitImplBuilder<'impls, AntiScalar, HasNotReturned>,
                        slf: Variable<MultiVector>,
                    ) -> Option<TraitImplBuilder<'impls, AntiScalar, Self::Output>> {
                        let mut dyn_mv = DynamicMultiVector::zero();
                        for (a, a_el) in slf.elements() {
                            if a_el.anti_grade(AntiScalar) == $gr {
                                dyn_mv += (a, a_el);
                            }
                        }
                        let mv = dyn_mv.construct(&b.mvs)?;
                        b.return_expr(mv)
                    }
                }
                )+
            }
        };
    }
    select_grade_impls!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

}