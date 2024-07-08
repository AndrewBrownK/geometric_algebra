#![allow(non_upper_case_globals)]

use crate::ast2::impls::Elaborated;
use crate::ast2::traits::{NameTrait, TraitImpl_10, TraitImpl_11};
use crate::build_scripts2::common_traits::impls::*;

pub static Zero: Elaborated<ZeroImpl> = ZeroImpl
    .name("Zero")
    .blurb("All elements set to zero.");
pub static One: Elaborated<OneImpl> = OneImpl
    .name("One")
    .blurb("The scalar element set to one, and all other elements set to zero.");
pub static Unit: Elaborated<UnitImpl> = UnitImpl
    .name("Unit")
    .blurb("All elements set to one.");
pub static Grade: Elaborated<GradeImpl> = GradeImpl
    .name("Grade")
    .blurb("A multivector class may have uniform grade, or mixed grade, depending on \
    the grades of its elements. This trait only characterizes uniform grade multivectors.");
pub static AntiGrade: Elaborated<AntiGradeImpl> = AntiGradeImpl
    .name("AntiGrade")
    .blurb("The AntiGrade can be described as the missing Grade with respect to an AntiScalar.");

mod impls {
    use async_trait::async_trait;

    use crate::algebra2::basis::{BasisElement, BasisSignature};
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

            // TODO I think I should pick up here


            todo!()
        }
    }

}