#![allow(non_upper_case_globals)]

use std::collections::BTreeSet;

use crate::algebra::basis::filter::{allow_all_signatures, signatures_containing};
use crate::algebra::multivector::DeclareMultiVecs;
use crate::ast::datatype::ExpressionType;
use crate::elements::{e12345, e4, e5};
use crate::Rust;

crate::multi_vecs! { e12345;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e12345;
    DualNum    as e5, e12345;
    // DualNum4   as e4, e12345;
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

#[test]
fn test_stuff() {
    let cga3d = crate::ga! { e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    };
    let mut decls = register_multi_vecs(cga3d);
    let repo = generate_variants(decls).finished();
    let traits = crate::register_all! { repo;
        Wedge
    };
    let traits = traits.finish();


    let mut rust = Rust::new(true).all_features();
    rust.sql = false;
    let mut infix_dummy = BTreeSet::new();

    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let impls = traits.get_impls().await;
        for i in impls {
            let ExpressionType::Class(owner) = i.owner.clone() else { continue };
            let Some(ExpressionType::Class(other)) = i.other_type_params.get(0).cloned() else { continue };

            if owner.name() == "AntiPlane" && other.name() == "AntiFlectorOnOrigin" {
                let r = &i.return_expr;
                println!("{r:?}");

                let mut buffer = Vec::new();
                rust.declare_trait_impl(&mut buffer, i, &mut infix_dummy).unwrap();
                let mut rust_output = String::from_utf8(buffer).unwrap();
                println!("{rust_output}");
            }
        }
        Some(())
    });
    result.expect("Entire script must complete")
}


fn generate_variants(mut declarations: DeclareMultiVecs<e12345>) -> DeclareMultiVecs<e12345> {
    use crate::algebra::basis::filter::{allow_all_signatures, SigFilter, signatures_containing};
    use crate::elements::*;

    let origin = signatures_containing(e4);
    let infinity = signatures_containing(e5);
    let flat_origin = origin & infinity;

    let all = allow_all_signatures();
    let is_flat = infinity.all_match();
    let is_not_flat = (!infinity).any_match();
    let tangent_null_cone = origin.any_match() ^ infinity.any_match();
    let intersects_null_cone = origin.any_match() & infinity.any_match();

    crate::variants! { declarations;

        #docs("This variant of {type} has a radius of zero and is centered on the Origin.")
        Null{type}AtOrigin => (origin & !infinity)  where is_not_flat => tangent_null_cone;

        #docs("This variant of {type} intersects the Origin.")
        {type}OnOrigin => (origin)                  where all => intersects_null_cone;

        #docs("This variant of {type} exists in the Horizon.")
        {type}AtInfinity => (!origin)               where is_flat => tangent_null_cone;

        #docs("This variant of {type} is centered on the Origin.")
        {type}AtOrigin => (origin ^ infinity)       where is_not_flat => intersects_null_cone;

        // TODO I think we want to rename this to AtInfinity
        //  Notably, both the carrier and cocarrier are completely in the horizon
        //  ACTUALLY.... Don't mix it up! MysteryDipole has e45! This makes the CoCarrier at infinity,
        //  but this element itself is actually the FlatOrigin!
        #docs("TODO this is currently a mystery I'm investigating")
        Mystery{type} => (!(origin ^ infinity))     where is_not_flat => intersects_null_cone;

        #docs("This variant of {type} has a Carrier that intersects the Origin.")
        {type}AligningOrigin => (origin | infinity) where is_not_flat => intersects_null_cone;

        #docs("This variant of {type} has a CoCarrier that intersects the Origin.")
        {type}OrthogonalOrigin => (!flat_origin)    where is_not_flat => intersects_null_cone;

        // TODO and this.... should change the name slightly.
        //  The carrier is at infinity, but the cocarrier is not
        //  and could there be yet another with finite carrier but infinite cocarrier?
        #docs("This variant of {type} exists at the Horizon.")
        {type}AtInfinity => (!origin | flat_origin) where is_not_flat => intersects_null_cone;
    }

    declarations.generate_missing_duals(Some(
        "This variant of {super} is the Dual to {type}. It is common for
        objects of this type to not intersect the null cone, which also prevents them from
        projecting onto the horosphere in the usual manner. When this happens, this
        object has behavioral and operative similarity to a {super},
        but an imaginary radius, and a spacial presence in the shape of a
        {type} with a real radius.",
    ));
    declarations
}
