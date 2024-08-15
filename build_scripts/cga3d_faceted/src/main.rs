#![allow(non_upper_case_globals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(effects)]

use std::path::PathBuf;

use codegen::algebra2::multivector::DeclareMultiVecs;
use codegen::elements::e12345;

codegen::multi_vecs! { e12345;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e12345;
    Origin     as e4;
    Infinity   as e5;
    FlatOrigin as e45;
    Horizon    as e3215;
    DualNum    as e5, e12345;

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
    VersorOdd  as e41, e42, e43, scalar | e23, e31, e12, e45 | e15, e25, e35, e1234 | e4235, e4315, e4125, e3215;
    VersorEven as e423, e431, e412, e12345 | e415, e425, e435, e321 | e235, e315, e125, e5 | e1, e2, e3, e4;
}

#[test]
fn lazy_compile_button() {
    // hi
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
    let repo = generate_variants(base_documentation(register_multi_vecs(cga3d))).finished();
    let traits = codegen::register_all! { repo;
        // specialized::Plane_BulkExpansion_Plane
        // |
        Zero One AntiOne Unit
        Grade AntiGrade Into TryInto
        Dual AntiDual Reverse AntiReverse
        |
        Wedge AntiWedge GeometricProduct GeometricAntiProduct Sandwich AntiSandwich
        // TODO do CGA expansion/contraction, not naive flat ones
        // BulkExpansion BulkContraction WeightExpansion WeightContraction
    };
    codegen::operators! { traits;
        infix => Div;

        binary
        BitXor => Wedge,
        Mul => GeometricProduct;

        unary
        Not => Dual;
    }
    let traits = traits.finish();

    let file_path = PathBuf::from("libraries/cga3d_faceted/");
    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.write_crate(
        file_path.clone(),
        "cga3d_faceted",
        1, 0, 0, "",
        "Latest generated test case",
        "https://github.com/AndrewBrownK/projective_ga/",
        &[],
        repo, traits
    );
}

fn base_documentation(mut declarations: DeclareMultiVecs<e12345>) -> DeclareMultiVecs<e12345> {
    declarations.append_documentation(
        &Origin,
        "\
    The Origin is the RoundPoint where x, y, z, and radius are all zero.
    It is the base element e4.
    Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
    ",
    );
    // TODO more documentation
    declarations
}

fn generate_variants(mut declarations: DeclareMultiVecs<e12345>) -> DeclareMultiVecs<e12345> {
    use codegen::algebra2::basis::filter::{allow_all_signatures, SigFilter, signatures_containing};
    use codegen::elements::*;

    let origin = signatures_containing(e4);
    let infinity = signatures_containing(e5);
    let flat_origin = origin & infinity;

    let all = allow_all_signatures();
    let is_flat = infinity.all_match();
    let is_not_flat = (!infinity).any_match();
    let tangent_null_cone = origin.any_match() ^ infinity.any_match();
    let intersects_null_cone = origin.any_match() & infinity.any_match();

    codegen::variants! { declarations;

        #docs("This variant of {type} has a radius of zero and is centered on the Origin.")
        Null{type}AtOrigin => (origin & !infinity)  where is_not_flat => tangent_null_cone;

        #docs("This variant of {type} intersects the Origin.")
        {type}OnOrigin => (origin)                  where all => intersects_null_cone;

        #docs("This variant of {type} exists in the Horizon.")
        {type}AtInfinity => (!origin)               where is_flat => tangent_null_cone;

        #docs("This variant of {type} is centered on the Origin.")
        {type}AtOrigin => (origin ^ infinity)       where is_not_flat => intersects_null_cone;

        #docs("This variant of {type} has a Carrier that intersects the Origin.")
        {type}AligningOrigin => (origin | infinity) where is_not_flat => intersects_null_cone;

        #docs("This variant of {type} has a CoCarrier that intersects the Origin.")
        {type}OrthogonalOrigin => (!flat_origin)    where is_not_flat => intersects_null_cone;

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

pub mod specialized {
    use codegen::ast2::datatype::MultiVector;
    use codegen::ast2::impls::{Specialize_22, Specialized_22};
    use codegen::build_scripts2::common_traits::BulkExpansion;
    use codegen::elements::e12345;

    use crate::Plane;

    pub static Plane_BulkExpansion_Plane: Specialized_22<e12345, MultiVector> = BulkExpansion.specialize(&Plane, &Plane, &|mut b, slf, other| {
        Box::pin(async move {
            // TODO actually implement
            b.return_expr(slf)
        })
    });
}