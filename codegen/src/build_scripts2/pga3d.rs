#![allow(non_upper_case_globals)]

use crate::{ga, multi_vecs};
use crate::algebra2::basis::elements::e0123;
use crate::algebra2::multivector::DynamicMultiVector;
use crate::ast2::datatype::{Float, MultiVector};
use crate::ast2::expressions::FloatExpr;
use crate::ast2::traits::{TraitDef_1Class_1Param, TraitDef_2Class_2Param, TraitImplBuilder};
use crate::ast2::Variable;
use crate::build_scripts2::common_traits::{AntiReverse, AntiScalarProduct, AntiWedge, Dual, GeometricProduct, Reverse, Sandwich, ScalarProduct, Wedge};

fn float_var(n: &str) -> Variable<Float> {
    Variable::quick_var(n, Float)
}
fn float_var_expr(n: &str) -> FloatExpr {
    Variable::quick_var(n, Float).into()
}

/* R ⟑ C = MultiVector(
    scalar((c12 * -1)),
    e0(0), e1(0), e2(0), e3(0),
    e01((-c23 + c02)),
    e02((-c31 - c01 + c12)),
    e03((-c12 - c31)),
    e12(0),
    e31((c23 * -1)),
    e23(c31),
    e021(0), e013(0), e032(0), e123(0),
    e0123((c03 + c23))
) */
multi_vecs! { e0123;

    // Base might be either point or plane, depending on our interpretation
    Vector as e0 | e1 | e2 | e3;
    BiVector as e01 | e02 | e03 | e23 | e31 | e12;
    TriVector as e021 | e013 | e032 | e123;
    SimpleRotor as e01 | e12 | e0123;
    Rotor as e01 | e02 | e03 | e23 | e31 | e12 | e0123;

    HalfRotorSandwich1 as e0 | e1 | e2 | e021 | e013 | e032 | e123;
    HalfRotorSandwich2 as scalar | e01 | e02 | e03 | e12 | e31 | e23 | e0123;

    DualNum as scalar | e0123;
    MultiVector as scalar | e0 | e1 | e2 | e3 | e01 | e02 | e03 | e12 | e31 | e23 | e021 | e013 | e032 | e123 | e0123;
}

#[test]
//noinspection DuplicatedCode
fn anti_product_argument() {
    let rga3d = ga! { e0123;
        1 => e1, e2, e3;
        0 => e0;
    };
    use crate::algebra2::basis::elements::*;
    let repo = register_multi_vecs(rga3d.clone()).finished();

    let vector = MultiVector::from(&Vector);
    let rotor = MultiVector::from(&SimpleRotor);
    let bivector = MultiVector::from(&BiVector);
    let hrs2 = MultiVector::from(&HalfRotorSandwich2);

    let a = vector.construct(|el| {
        let mut n = format!("{el}");
        n = n.replace("e", "a");
        float_var_expr(n.as_str())
    });
    let b = vector.construct(|el| {
        let mut n = format!("{el}");
        n = n.replace("e", "b");
        float_var_expr(n.as_str())
    });
    let c = bivector.construct(|el| {
        let mut n = format!("{el}");
        n = n.replace("e", "c");
        float_var_expr(n.as_str())
    });
    let h = hrs2.construct(|el| {
        let mut n = format!("{el}");
        if n.as_str() == "scalar" {
            n = "h_scalar".to_string()
        } else {
            n = n.replace("e", "h");
        }
        float_var_expr(n.as_str())
    });
    let rotor = rotor.construct(|el| {
        if el == e01 { FloatExpr::Literal(1.0) } else if el == e12 { FloatExpr::Literal(1.0) } else if el == e0123 { FloatExpr::Literal(1.0) } else { FloatExpr::Literal(0.0) }
    });


    let builder = TraitImplBuilder::new_sandbox(rga3d.clone(), repo);
    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let a_dual = Dual.deep_inline(&builder, a.clone()).await?;
        println!("A = {a}");
        println!("A* = {a_dual}");

        let b_dual = Dual.deep_inline(&builder, b.clone()).await?;
        println!("B = {b}");
        println!("B* = {b_dual}");

        println!("C = {c}");
        println!("H = {h}");
        println!("R = {rotor}");

        let r_reverse = Reverse.deep_inline(&builder, rotor.clone()).await?;
        println!("~R = {r_reverse}");

        let r_anti_reverse = AntiReverse.deep_inline(&builder, rotor.clone()).await?;
        println!("R~ = {r_anti_reverse}");

        let a_wedge_b = Wedge.deep_inline(&builder, a.clone(), b.clone()).await?;
        println!("A ∧ B = {a_wedge_b}");

        let a_anti_wedge_b = AntiWedge.deep_inline(&builder, a.clone(), b.clone()).await;
        println!("A ∨ B = {a_anti_wedge_b:?}");

        let a_dot_b = ScalarProduct.deep_inline(&builder, a.clone(), b.clone()).await?;
        println!("A • B = {a_dot_b}");

        let a_anti_dot_b = AntiScalarProduct.deep_inline(&builder, a.clone(), b.clone()).await?;
        println!("A ∘ B = {a_anti_dot_b}");

        let r_wd_a_wd_r = Sandwich.deep_inline(&builder, rotor.clone(), a.clone()).await?;
        println!("R ⟑ A ⟑ ~R = {r_wd_a_wd_r}");

        let r_wd_b_wd_r = Sandwich.deep_inline(&builder, rotor.clone(), b.clone()).await?;
        println!("R ⟑ B ⟑ ~R = {r_wd_b_wd_r}");

        // Geometric tests

        let r_wd_c = GeometricProduct.deep_inline(&builder, rotor.clone(), c.clone()).await?;
        println!("R ⟑ C = {r_wd_c}");
        let h_wd_r = GeometricProduct.deep_inline(&builder, h.clone(), r_reverse.clone()).await?;
        println!("H ⟑ ~R = {h_wd_r}");

        let r_wd_awb_wd_r = GeometricProduct.deep_inline(&builder, rotor.clone(), a_wedge_b.clone()).await?;
        println!("R ⟑ (A ∧ B) = {r_wd_awb_wd_r}");
        let r_wd_awb_wd_r = Sandwich.deep_inline(&builder, rotor.clone(), a_wedge_b.clone()).await?;
        println!("R ⟑ (A ∧ B) ⟑ ~R = {r_wd_awb_wd_r}");

        let rar_w_rbr = Wedge.deep_inline(&builder, r_wd_a_wd_r.clone(), r_wd_b_wd_r.clone()).await?;
        println!("(R ⟑ A ⟑ ~R) ∧ (R ⟑ B ⟑ ~R) = {rar_w_rbr}");

        // TODO next issue:
        /* R ⟑ C = MultiVector(
            scalar((c12 * -1)),
            e0(0), e1(0), e2(0), e3(0),
            e01((-c23 + c02)),
            e02((-c31 - c01 + c12)),
            e03((-c12 - c31)),
            e12(0),
            e31((c23 * -1)),
            e23(c31),
            e021(0), e013(0), e032(0), e123(0),
            e0123((c03 + c23))
        ) */
        // TODO it looks like the above is all correct.
        //  Verified in expression evaluator at https://bivector.net/tools.html?p=3&q=0&r=1
        //  So I need to check the second half of the sandwich now.

        // TODO but is below an accurate translation of the above? Seems not....
        /* R ⟑ (A ∧ B) = MultiVector(
            scalar(((a1 * b2) - (a2 * b1))),
            e0(0), e1(0), e2(0), e3(0),
            e01(((a2 * b3) - (a3 * b2) + (a0 * b2) - (a2 * b0))),
            e02((-(a1 * b3) + (a3 * b1) + (a0 * b1) - (a1 * b0) + (a1 * b2) - (a2 * b1))),
            e03(((a1 * b2) - (a2 * b1) - (a1 * b3) + (a3 * b1))),  // <-- TODO actually is this the wrong sign? It is....
            e12(0),
            e31(((a2 * b3) - (a3 * b2))),
            e23((-(a1 * b3) + (a3 * b1))),
            e021(0), e013(0), e032(0), e123(0),
            e0123(((a0 * b3) - (a3 * b0) + (a2 * b3) - (a3 * b2)))
        ) */


        // TODO examining the second half of the sandwich, we have this that
        //  we need to verify term-for-term in the expression evaluator
        /* H ⟑ ~R = HalfRotorSandwich2(
            scalar(h12),                        // <-- TODO note this term is already canceling
            e01((-h23 - h_scalar + h02)),
            e02((-h31 - h01 + h12)),
            e03((h0123 - h12 - h31)),
            e12((h_scalar * -1)),
            e31((h23 * -1)),
            e23(h31),
            e0123((-h23 + h_scalar - h03))       // <-- TODO note this term is NOT canceling
                                                 // TODO circled back to e03 and noticed the wrong sign
        ) */
        /* R ⟑ (A ∧ B) ⟑ ~R = Rotor(
            e01((-2*(a1 * b2) + 2*(a2 * b1) - (a0 * b1) + (a1 * b0))),
            e02((-(a0 * b2) + (a2 * b0))),
            e03((-(a0 * b3) + (a3 * b0) - 2*(a2 * b3) + 2*(a3 * b2))),
            e23((-(a2 * b3) + (a3 * b2))),
            e31(((a1 * b3) - (a3 * b1))),
            e12((-(a1 * b2) + (a2 * b1))),
            e0123((2*(a1 * b3) - 2*(a3 * b1)))
        ) */








        /* (R ⟑ A ⟑ ~R) ∧ (R ⟑ B ⟑ ~R) = BiVector(
            e01((-((2*a3 + 2*a2 + a0) * b1) + ((2*b3 + 2*b2 + b0) * a1))),
            e02((-((2*a3 + 2*a2 + a0) * b2) + ((2*b3 + 2*b2 + b0) * a2))),
            e03((((2*a3 + 2*a2 + a0) * b3) - (a3 * (2*b3 + 2*b2 + b0)))),
            e23((-(b3 * a2) + (a3 * b2))),
            e31(((b3 * a1) - (a3 * b1))),
            e12(((a1 * b2) - (a2 * b1)))
        ) */


        Some(())
    });
    result.expect("Entire script must complete")
}
