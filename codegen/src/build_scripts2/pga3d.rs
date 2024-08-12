#![allow(non_upper_case_globals)]

use crate::{ga, multi_vecs};
use crate::algebra2::basis::elements::e0123;
use crate::algebra2::multivector::DynamicMultiVector;
use crate::ast2::datatype::{Float, MultiVector};
use crate::ast2::expressions::FloatExpr;
use crate::ast2::traits::{TraitDef_2Class_2Param, TraitImplBuilder};
use crate::ast2::Variable;
use crate::build_scripts2::common_traits::GeometricProduct;

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

    let mut builder = TraitImplBuilder::new_sandbox(rga3d.clone(), repo);
    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let mut a_dual = DynamicMultiVector::zero();
        for (fe, el) in a.elements_flat() {
            let (f, el) = rga3d.dual(el);
            a_dual += (fe * f, el);
        }
        let a_dual = a_dual.construct(&builder)?;
        println!("A* = {a_dual}");

        let mut b_dual = DynamicMultiVector::zero();
        for (fe, el) in a.elements_flat() {
            let (f, el) = rga3d.dual(el);
            b_dual += (fe * f, el);
        }
        let b_dual = b_dual.construct(&builder)?;
        println!("B* = {b_dual}");

        let mut r_reverse = DynamicMultiVector::zero();
        for (fe, el) in rotor.elements_flat() {
            let (f, el) = rga3d.reverse(el);
            r_reverse += (fe * f, el);
        }
        let r_reverse = r_reverse.construct(&builder)?;
        println!("~R = {r_reverse}");

        let mut r_anti_reverse = DynamicMultiVector::zero();
        for (fe, el) in rotor.elements_flat() {
            let (f, el) = rga3d.anti_reverse(el);
            r_anti_reverse += (fe * f, el);
        }
        let r_anti_reverse = r_anti_reverse.construct(&builder)?;
        println!("R~ = {r_anti_reverse}");

        let a_wedge_b = a.distributive_by_groups(&b, |a, b| a * b, |a, b| a.wedge(b));
        let a_wedge_b = a_wedge_b.construct(&builder)?;
        println!("A ∧ B = {a_wedge_b}");

        let a_anti_wedge_b = a.distributive_by_groups(&b, |a, b| a * b, |a, b| a.anti_wedge(b, e0123));
        let a_anti_wedge_b = a_anti_wedge_b.construct(&builder);
        println!("A ∨ B = {a_anti_wedge_b:?}");

        let mut a_dot_b = DynamicMultiVector::zero();
        for (a, a_el) in a.elements_flat() {
            for (b, b_el) in b.elements_flat() {
                let sop = rga3d.scalar_product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = FloatExpr::Literal(p.coefficient);
                    a_dot_b += (a * b * c, p.element);
                }
            }
        }
        let a_dot_b = a_dot_b.construct(&builder)?;
        println!("A • B = {a_dot_b}");

        let mut a_anti_dot_b = DynamicMultiVector::zero();
        for (a, a_el) in a.elements_flat() {
            for (b, b_el) in b.elements_flat() {
                let sop = rga3d.anti_scalar_product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = FloatExpr::Literal(p.coefficient);
                    a_anti_dot_b += (a * b * c, p.element);
                }
            }
        }
        let a_anti_dot_b = a_anti_dot_b.construct(&builder)?;
        println!("A ∘ B = {a_anti_dot_b}");

        let mut r_wedge_dot_a = DynamicMultiVector::zero();
        for (af, a_el) in rotor.elements_flat() {
            for (b, b_el) in a.elements_flat() {
                let a = &af;
                let sop = rga3d.product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = FloatExpr::Literal(p.coefficient);
                    r_wedge_dot_a += (a * b * c, p.element);
                }
            }
        }
        let r_wedge_dot_a = r_wedge_dot_a.construct(&builder)?;
        println!("R ⟑ A = {r_wedge_dot_a}");
        let mut r_wedge_dot_a_wedge_dot_r = DynamicMultiVector::zero();
        for (a, a_el) in r_wedge_dot_a.elements_flat() {
            for (b, b_el) in r_reverse.elements_flat() {
                let sop = rga3d.product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = FloatExpr::Literal(p.coefficient);
                    r_wedge_dot_a_wedge_dot_r += (a * b * c, p.element);
                }
            }
        }
        let r_wedge_dot_a_wedge_dot_r = r_wedge_dot_a_wedge_dot_r.construct(&builder)?;
        println!("R ⟑ A ⟑ ~R = {r_wedge_dot_a_wedge_dot_r}");

        let r_wedge_dot_b = GeometricProduct.deep_inline(&builder, rotor, b).await?;
        println!("R ⟑ B = {r_wedge_dot_b}");
        let mut r_wedge_dot_b_wedge_dot_r = GeometricProduct.deep_inline(&builder, r_wedge_dot_b, r_reverse).await?;
        println!("R ⟑ B ⟑ ~R = {r_wedge_dot_b_wedge_dot_r}");

        Some(())
    });
    result.expect("Entire script must complete")
}
