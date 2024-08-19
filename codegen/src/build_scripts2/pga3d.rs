#![allow(non_upper_case_globals)]

use crate::algebra2::basis::elements::e0123;
use crate::algebra2::multivector::DynamicMultiVector;
use crate::ast2::datatype::{Float, MultiVector};
use crate::ast2::expressions::FloatExpr;
use crate::ast2::traits::{TraitDef_1_Type_1_Arg, TraitDef_2_Types_2_Args, TraitImplBuilder};
use crate::ast2::Variable;
use crate::build_scripts2::common_traits::{AntiReverse, AntiScalarProduct, AntiWedge, Dual, GeometricProduct, Reverse, Sandwich, ScalarProduct, Wedge};
use crate::{ga, multi_vecs};

fn float_var(n: &str) -> Variable<Float> {
    Variable::quick_var(n, Float)
}
fn float_var_expr(n: &str) -> FloatExpr {
    Variable::quick_var(n, Float).into()
}

// R ⟑ C = MultiVector(
// scalar((c12 * -1)),
// e0(0), e1(0), e2(0), e3(0),
// e01((-c23 + c02)),
// e02((-c31 - c01 + c12)),
// e03((-c12 - c31)),
// e12(0),
// e31((c23 * -1)),
// e23(c31),
// e021(0), e013(0), e032(0), e123(0),
// e0123((c03 + c23))
// )
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
// noinspection DuplicatedCode
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
        if el == e01 {
            FloatExpr::Literal(1.0)
        } else if el == e12 {
            FloatExpr::Literal(1.0)
        } else if el == e0123 {
            FloatExpr::Literal(1.0)
        } else {
            FloatExpr::Literal(0.0)
        }
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

        // let r_wd_c = GeometricProduct.deep_inline(&builder, rotor.clone(), c.clone()).await?;
        // println!("R ⟑ C = {r_wd_c}");
        // println!("Alright so the above is perfect");

        // let h_wd_r = GeometricProduct.deep_inline(&builder, h.clone(), r_reverse.clone()).await?;
        // println!("H ⟑ ~R = {h_wd_r}");

        // let r_wd_awb_wd_r = GeometricProduct.deep_inline(&builder, rotor.clone(), a_wedge_b.clone()).await?;
        // println!("R ⟑ (A ∧ B) = {r_wd_awb_wd_r}");
        let r_wd_awb_wd_r = Sandwich.deep_inline(&builder, rotor.clone(), a_wedge_b.clone()).await?;
        println!("R ⟑ (A ∧ B) ⟑ ~R = {r_wd_awb_wd_r}");

        let rar_w_rbr = Wedge.deep_inline(&builder, r_wd_a_wd_r.clone(), r_wd_b_wd_r.clone()).await?;
        println!("(R ⟑ A ⟑ ~R) ∧ (R ⟑ B ⟑ ~R) = {rar_w_rbr}");

        /*
        TODO visually inspect the two outputs below and determine if they are equal, and also
         if there are more simplifications or stuff we should so

        TODO needed simplifications... it is tempting to think "lets not always distribute
         products into sums every time, because maybe it is less efficient to calculate that"
         however the end user can manage that with variable boundaries, but deep inlining
         absolutely needs those distributions to take place in order to reach maximum
         simplification. So I should indeed do those product into sum distributions, and if
         they wanted to not do that, they could/should have defined variables.

        TODO actually no.... I can't believe I'm saying this, but I can't get it out of my head now.
         I always thought I wouldn't resort to this, but I'm too tempted now. We can traverse
         the AST and try to find redundant expressions.... and then pull them out into variables.
         Then doing a deep inline and redundancy elimination will be the ultimate always optimal
         AST simplification. Then we can do this for every single trait implementation. It could
         be done with a similar tool as TrackOperations, where only non-trivial branches of the
         AST are collected (e.g. yes to arithmetic, no to variable invocations). We track both the
         TrackedOperations of the AST branch, and also how many duplicates we've found. We don't
         need to create a variable out of something unless it is used at least twice. If we do this,
         then it is always(?) a safe bet to compress nested sums and products into sum-of-products
         form. .....or is it? grr... I mean if all terms of a sum share a factor, obviously it makes
         sense to pull the factor out..... Well... maybe we need to go fully compacted
         sum-of-products anyway, just for term elimination, and all it means now is I need to
         figure out factorization too... If I'm going to extract redundant branches into variables,
         then pulling out common factors can't be so bad either... Except then you'd have different
         terms to try to eliminate.... ugh... Is it just the case that neither strategy is optimal
         in all cases, and there is always some imaginable case that is better to be factorized vs
         better to be variable extracted without factorizing? Well...
         1. deep inline
         2. flatten to sum of products for term elimination
         3. collect possible variable extractions, and the operative weights before and after
         4. collect possible factorizations, and the operative weights before and after
         5. Take a greedy approach of saving the most operations possible, whether variables or factorizing

        R ⟑ (A ∧ B) ⟑ ~R = BiVector(
            e01((2*(a1 * b3) - 2*(a3 * b1) + 2*(a1 * b2) - 2*(a2 * b1) - (a0 * b1) + (a1 * b0))),
            e02((2*(a2 * b3) - 2*(a3 * b2) - (a0 * b2) + (a2 * b0))),
            TODO lower three are good, above two not confirmed yet
            e03((2*(a2 * b3) - 2*(a3 * b2) + (a0 * b3) - (a3 * b0))),
            e23((-(a2 * b3) + (a3 * b2))), e31(((a1 * b3) - (a3 * b1))),
            e12(((a1 * b2) - (a2 * b1)))
        )
        (R ⟑ A ⟑ ~R) ∧ (R ⟑ B ⟑ ~R) = BiVector(
            e01((-((2*a3 + 2*a2 + a0) * b1) + ((2*b3 + 2*b2 + b0) * a1))),
            e02((-((2*a3 + 2*a2 + a0) * b2) + ((2*b3 + 2*b2 + b0) * a2))),
            TODO lower three are good, above two not confirmed yet
            e03((((2*a3 + 2*a2 + a0) * b3) - (a3 * (2*b3 + 2*b2 + b0)))),
            e23((-(b3 * a2) + (a3 * b2))), e31(((b3 * a1) - (a3 * b1))),
            e12(((a1 * b2) - (a2 * b1)))
        )

         */

        Some(())
    });
    result.expect("Entire script must complete")
}
