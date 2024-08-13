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

multi_vecs! { e0123;

    // Base might be either point or plane, depending on our interpretation
    Base as e0 | e1 | e2 | e3;
    BiVector as e01 | e02 | e03 | e23 | e31 | e12;
    TriVector as e021 | e013 | e032 | e123;
    Rotor as e01 | e02 | e03 | e23 | e31 | e12 | e0123;

    HalfRotorSandwich as e0 | e1 | e2 | e021 | e013 | e032 | e123;

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

    let base = MultiVector::from(&Base);
    let rotor = MultiVector::from(&Rotor);

    let a = base.construct(|el| {
        if el == e0 { float_var_expr("a0") } else if el == e1 { float_var_expr("a1") } else if el == e2 { float_var_expr("a2") } else if el == e3 { float_var_expr("a3") } else { FloatExpr::Literal(0.0) }
    });
    let b = base.construct(|el| {
        if el == e0 { float_var_expr("b0") } else if el == e1 { float_var_expr("b1") } else if el == e2 { float_var_expr("b2") } else if el == e3 { float_var_expr("b3") } else { FloatExpr::Literal(0.0) }
    });
    let rotor = rotor.construct(|el| {
        if el == e01 { FloatExpr::Literal(1.0) } else if el == e12 { FloatExpr::Literal(1.0) } else if el == e0123 { FloatExpr::Literal(1.0) } else { FloatExpr::Literal(0.0) }
    });

    println!("A = {a}");
    println!("B = {b}");
    println!("R = {rotor}");

    let builder = TraitImplBuilder::new_sandbox(rga3d.clone(), repo);
    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let a_dual = Dual.deep_inline(&builder, a.clone()).await?;
        println!("A* = {a_dual}");

        let b_dual = Dual.deep_inline(&builder, b.clone()).await?;
        println!("B* = {b_dual}");

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

        let r_wedge_dot_a_wedge_dot_r = Sandwich.deep_inline(&builder, rotor.clone(), a.clone()).await?;
        println!("R ⟑ A ⟑ ~R = {r_wedge_dot_a_wedge_dot_r}");

        let r_wedge_dot_b = GeometricProduct.deep_inline(&builder, rotor, b).await?;
        // println!("R ⟑ B = {r_wedge_dot_b}");
        let mut r_wedge_dot_b_wedge_dot_r = GeometricProduct.deep_inline(&builder, r_wedge_dot_b, r_reverse).await?;
        println!("R ⟑ B ⟑ ~R = {r_wedge_dot_b_wedge_dot_r}");

        Some(())
    });
    result.expect("Entire script must complete")
}
