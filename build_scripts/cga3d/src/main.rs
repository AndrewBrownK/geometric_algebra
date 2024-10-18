#![allow(non_upper_case_globals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(adt_const_params)]

use codegen::algebra2::multivector::DeclareMultiVecs;
use codegen::ast2::traits::{
    TraitDef_1_Type_1_Arg,
    TraitDef_1_Type_2_Args_f32,
    TraitDef_1_Type_2_Args_i32,
    TraitDef_2_Types_2_Args
    ,
};
use codegen::build_scripts2::common_traits::GeometricAntiProduct;
use codegen::elements::e12345;

codegen::multi_vecs! { e12345;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e12345;
    DualNum    as e4, e12345;
    // DualNum5   as e5, e12345;
    // DualNum321 as e321, e12345;
    // TripleNum    as e4, e5, e12345;
    // QuadNum    as e4, e5, e321, e12345;

    // Uniform Grade Flat Objects
    FlatPoint  as e15, e25, e35, e45;
    Line       as e415, e425, e435 | e235, e315, e125;
    Plane      as e4235, e4315, e4125, e3215;

    // Uniform Grade Round Objects
    RoundPoint as e1, e2, e3, e4 | e5;
    Dipole     as e41, e42, e43 | e23, e31, e12, e45 | e15, e25, e35;
    Circle     as e423, e431, e412 | e415, e425, e435, e321 | e235, e315, e125;
    Sphere     as e4235, e4315, e4125, e3215 | e1234;

    // Versors
    Motor      as e415, e425, e435, e12345 | e235, e315, e125, e5;
    Flector    as e15, e25, e35, e45 | e4235, e4315, e4125, e3215;
    // 2 reflections
    CircleRotor as e423, e431, e412 | e415, e425, e435, e321 | e235, e315, e125, e12345;
    // 3 reflections
    DipoleInversion as e41, e42, e43 | e23, e31, e12, e45 | e15, e25, e35, e1234 | e4235, e4315, e4125, e3215;
    // 4 reflections
    VersorEven as e423, e431, e412, e12345 | e415, e425, e435, e321 | e235, e315, e125, e5 | e1, e2, e3, e4;
    // 5 reflections
    VersorOdd  as e41, e42, e43, scalar | e23, e31, e12, e45 | e15, e25, e35, e1234 | e4235, e4315, e4125, e3215;
}


/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
fn main() {
    let cga3d = codegen::ga! { e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    };
    let repo = base_documentation(register_multi_vecs(cga3d)).finished();
    let traits = codegen::register_all! { repo;
        // specialized::Plane_BulkExpansion_Plane
        // |
        Zero One AntiOne Unit
        Grade AntiGrade Into TryInto
        RightDual RightAntiDual Reverse AntiReverse
        |
        Wedge AntiWedge GeometricProduct GeometricAntiProduct Sandwich AntiSandwich
        // TODO do CGA expansion/contraction, not naive flat ones
        // BulkExpansion BulkContraction WeightExpansion WeightContraction
        Fix AntiFix
        ConstraintViolation AntiConstraintViolation
        ConstraintValid AntiConstraintValid
    };
    codegen::operators! { repo, traits;
        fancy_infix => Div;

        binary
        Add => Addition,
        Sub => Subtraction,
        BitXor => Wedge,
        Mul => GeometricProduct;

        unary
        Neg => Negation,
        Not => RightDual;
    }
    let traits = traits.finish();

    let mut wgsl = codegen::Wgsl::new();
    wgsl.write_shader_file(
        "libraries/cga3d/src/",
        "cga3d",
        1,
        0,
        0,
        "",
        "Latest generation test case",
        "https://github.com/AndrewBrownK/projective_ga/",
        &[],
        repo.clone(),
        traits.clone(),
    );

    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.write_crate(
        "libraries/cga3d/",
        "cga3d",
        1,
        0,
        0,
        "",
        "Latest generated test case",
        "https://github.com/AndrewBrownK/projective_ga/",
        &[],
        repo,
        traits,
    );
}

fn base_documentation(mut declarations: DeclareMultiVecs<e12345>) -> DeclareMultiVecs<e12345> {
    // declarations.append_documentation(
    //     &Origin,
    //     "\
    // The Origin is the RoundPoint where x, y, z, and radius are all zero.
    // It is the base element e4.
    // Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
    // ",
    // );
    // TODO more documentation
    declarations.generate_missing_duals(None);
    declarations
}

// noinspection DuplicatedCode
pub mod custom_traits {
    use async_trait::async_trait;

    use codegen::algebra2::multivector::DynamicMultiVector;
    use codegen::ast2::datatype::MultiVector;
    use codegen::ast2::impls::Elaborated;
    use codegen::ast2::traits::NameTrait;
    use codegen::elements::e5;
    use codegen::trait_impl_1_type_1_arg;

    pub static ConformalConjugate: Elaborated<ConformalConjugateImpl> = ConformalConjugateImpl.new_trait_named("ConformalConjugate").blurb("TODO");

    trait_impl_1_type_1_arg!(ConformalConjugateImpl(builder, slf) -> MultiVector {
        let infinity_sig = e5.signature();
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements_by_groups() {
            if el.signature().contains(infinity_sig) {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });
}



#[test]
#[cfg(feature = "incorrect-wip-traits")]
fn test_powf() {
    let cga3d = codegen::ga! { e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    };
    fn float_var_expr(n: &str) -> FloatExpr {
        Variable::quick_var(n, Float).into()
    }
    let repo = base_documentation(register_multi_vecs(cga3d.clone())).finished();
    let builder = TraitImplBuilder::new_sandbox(cga3d.clone(), repo);
    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let mvs = [
            MultiVector::from(&AntiScalar),
            MultiVector::from(&DualNum4),
            MultiVector::from(&DualNum5),
            MultiVector::from(&DualNum321),
            MultiVector::from(&TripleNum),
            MultiVector::from(&QuadNum),
        ];

        for mv in mvs {
            let base_mv = mv.construct(|el| {
                let mut n = format!("{el}");
                n = n.replace("e", "a");
                float_var_expr(n.as_str())
            });

            let accurate_squared = GeometricAntiProduct.deep_inline(&builder, base_mv.clone(), base_mv.clone()).await?;
            let accurate_cubed = GeometricAntiProduct.deep_inline(&builder, accurate_squared.clone(), base_mv.clone()).await?;
            let accurate_4th_pow = GeometricAntiProduct.deep_inline(&builder, accurate_cubed.clone(), base_mv.clone()).await?;
            let accurate_5th_pow = GeometricAntiProduct.deep_inline(&builder, accurate_4th_pow.clone(), base_mv.clone()).await?;
            let accurate_6th_pow = GeometricAntiProduct.deep_inline(&builder, accurate_5th_pow.clone(), base_mv.clone()).await?;
            println!("\nBase:             {base_mv}");
            println!("Accurate Square:  {accurate_squared}");
            println!("Accurate Cube:    {accurate_cubed}");
            println!("Accurate 4th pow: {accurate_4th_pow}");
            println!("Accurate 5th pow: {accurate_5th_pow}");
            println!("Accurate 6th pow: {accurate_6th_pow}");

            // let trait_squared = AntiSquare.deep_inline(&builder, base_mv.clone()).await?;
            // if accurate_squared != trait_squared {
            //     println!("Wrong Trait Square: {trait_squared}");
            // }
            // let powf_squared = AntiPowf.deep_inline(&builder, base_mv.clone(), FloatExpr::Literal(2.0)).await?;
            // if accurate_squared != powf_squared {
            //     println!("Wrong Powf Square: {powf_squared}");
            // }
            // let powf_cubed = AntiPowf.deep_inline(&builder, base_mv.clone(), FloatExpr::Literal(3.0)).await?;
            // if accurate_cubed != powf_cubed {
            //     println!("Wrong Powf Cube: {powf_cubed}");
            // }
            //
            // let powf_root = AntiPowf.deep_inline(&builder, base_mv.clone(), FloatExpr::Literal(0.5)).await?;
            // let root_squared = GeometricAntiProduct.deep_inline(&builder, powf_root.clone(), powf_root.clone()).await?;
            // if base_mv != root_squared {
            //     println!("Wrong Root Squared: {root_squared}");
            //     println!("Wrong Powf Root:    {powf_root}");
            // }
        }
        Some(())
    });
    result.expect("Entire script must complete")
}