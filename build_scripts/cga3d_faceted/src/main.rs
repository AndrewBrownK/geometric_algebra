#![allow(non_upper_case_globals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(adt_const_params)]

use crate::custom_traits::{Carrier, CoCarrier, ConformalConjugate};
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
    DualNum    as e4, e12345;
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
        RightDual RightAntiDual Reverse AntiReverse
        |
        Wedge AntiWedge GeometricProduct GeometricAntiProduct Sandwich AntiSandwich
        // TODO do CGA expansion/contraction, not naive flat ones
        // BulkExpansion BulkContraction WeightExpansion WeightContraction
        Carrier CoCarrier
        ConformalConjugate
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

    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.write_crate(
        "libraries/cga3d_faceted/",
        "cga3d_faceted",
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
    use codegen::algebra2::basis::filter::{allow_all_signatures, signatures_containing, SigFilter};
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

pub mod specialized_traits {
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

// noinspection DuplicatedCode
pub mod custom_traits {
    use async_trait::async_trait;

    use codegen::algebra2::multivector::DynamicMultiVector;
    use codegen::ast2::datatype::MultiVector;
    use codegen::ast2::impls::Elaborated;
    use codegen::ast2::traits::{NameTrait, TraitDef_1_Type_1_Arg};
    use codegen::build_scripts2::common_traits::RightAntiDual;
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

    pub static Carrier: Elaborated<CarrierImpl> = CarrierImpl.new_trait_named("Carrier").blurb("TODO");

    trait_impl_1_type_1_arg!(CarrierImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.wedge(el, e5);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    pub static CoCarrier: Elaborated<CoCarrierImpl> = CoCarrierImpl.new_trait_named("CoCarrier").blurb("TODO");

    trait_impl_1_type_1_arg!(CoCarrierImpl(builder, slf) -> MultiVector {
        let slf = RightAntiDual.invoke(&mut builder, slf).await?;
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements_by_groups() {
            let (f, el) = builder.ga.wedge(el, e5);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });
}
