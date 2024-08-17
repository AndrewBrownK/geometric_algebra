#![allow(non_upper_case_globals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(adt_const_params)]

use codegen::algebra2::multivector::DeclareMultiVecs;
use codegen::elements::e12345;

codegen::multi_vecs! { e12345;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e12345;
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
        Dual AntiDual Reverse AntiReverse
        |
        Wedge AntiWedge GeometricProduct GeometricAntiProduct Sandwich AntiSandwich
        // TODO do CGA expansion/contraction, not naive flat ones
        // BulkExpansion BulkContraction WeightExpansion WeightContraction
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
        Not => Dual;
    }
    let traits = traits.finish();

    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.write_crate(
        "libraries/cga3d/",
        "cga3d",
        1, 0, 0, "",
        "Latest generated test case",
        "https://github.com/AndrewBrownK/projective_ga/",
        &[],
        repo, traits
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
    declarations
}

pub mod custom_traits {
    use async_trait::async_trait;

    use codegen::algebra2::basis::BasisElement;
    use codegen::algebra2::multivector::DynamicMultiVector;
    use codegen::ast2::datatype::MultiVector;
    use codegen::ast2::impls::Elaborated;
    use codegen::ast2::traits::{HasNotReturned, NameTrait, TraitImpl_11, TraitImplBuilder};
    use codegen::ast2::Variable;
    use codegen::elements::e5;

    pub static ConformalConjugate: Elaborated<ConformalConjugateImpl> = ConformalConjugateImpl
        .new_trait_named("ConformalConjugate")
        .blurb("TODO");
    #[derive(Clone, Copy)]
    struct ConformalConjugateImpl;
    #[async_trait]
    impl TraitImpl_11 for ConformalConjugateImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            b: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let infinity_sig = e5.signature();
            let mut result = DynamicMultiVector::zero();
            for (mut fe, el) in slf.elements_by_groups() {
                if el.signature().contains(infinity_sig) {
                    fe = fe * -1.0;
                }
                result += (fe, el);
            }
            let result = result.construct(&b)?;
            b.return_expr(result)
        }
    }


}