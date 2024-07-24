#![allow(non_upper_case_globals)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::{ga, multi_vecs, register_all};
use crate::algebra2::basis::BasisSignature;
use crate::algebra2::basis::elements::e12345;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{DeclareMultiVecs, MultiVecRepository};
use crate::ast2::datatype::MultiVector;
use crate::ast2::impls::{Specialize_22, Specialized_22};
use crate::ast2::traits::BinaryOps;
use crate::build_scripts2::common_traits::{AntiGrade, AntiOne, AntiWedge, BulkExpansion, GeometricAntiProduct, GeometricProduct, Grade, One, Unit, Wedge, Zero};

multi_vecs!(e12345;

    // Special Objects
    Origin     as e4;
    Infinity   as e5;
    FlatOrigin as e45;
    Horizon    as e3215;

    // Uniform Grade Flat Objects
    FlatPoint  as e15, e25, e35, e45;
    Line       as e415, e425, e435 | e235, e315, e125;
    Plane      as e4235, e4315, e4125, e3215;

    // Uniform Grade Round Objects
    RoundPoint as e1, e2, e3, | e4, e5;
    Dipole     as e41, e42, e43 | e23, e31, e12 | e15, e25, e35, e45;
    Circle     as e423, e431, e412, e321 | e415, e425, e435 | e235, e315, e125;
    Sphere     as e4235, e4315, e4125 | e1234, e3215;

    // Versors
    Motor      as e415, e425, e435, e12345 | e235, e315, e125;
    Flector    as e15, e25, e35, e45 | e4235, e4315, e4125, e3215;
);


/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
#[test]
pub fn cga3d_script() {
    let cga3d = ga!(e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    );
    let repo = generate_variants(register_multi_vecs(cga3d));
    let traits = register_all! { repo;
        Plane_BulkExpansion_Plane
        |
        Zero One AntiOne Unit
        Grade AntiGrade
        |
        Wedge AntiWedge GeometricProduct GeometricAntiProduct
    };
    traits.set_binary_operator(BinaryOps::BitXor, Wedge);
    traits.set_binary_operator(BinaryOps::Mul, GeometricProduct);
    traits.generate_infix_trick(BinaryOps::Div);

    // TODO output files
}

fn generate_variants(mut declarations: DeclareMultiVecs<e12345>) -> Arc<MultiVecRepository<e12345>> {
    use crate::algebra2::basis::elements::*;

    let origin = e4.signature();
    let infinity = e5.signature();
    let flat_origin = e45.signature();

    let all = |_: &Grades, _: &BTreeSet<BasisSignature>| true;
    let is_flat = |_: &Grades, sigs: &BTreeSet<BasisSignature>| sigs.iter().all(|sig| sig.contains(infinity));
    let is_not_flat = |_: &Grades, sigs: &BTreeSet<BasisSignature>| sigs.iter().any(|sig| !sig.contains(infinity));
    let tangent_null_cone = |_: &Grades, sigs: &BTreeSet<BasisSignature>| {
        let mut has_e4 = false;
        let mut has_e5 = false;
        for sig in sigs.iter() {
            has_e4 |= sig.contains(origin);
            has_e5 |= sig.contains(infinity);
            if has_e4 && has_e5 {
                return false
            }
        }
        return has_e4 || has_e5
    };
    let intersects_null_cone = |_: &Grades, sigs: &BTreeSet<BasisSignature>| {
        sigs.iter().any(|sig| sig.contains(origin)) && sigs.iter().any(|sig| sig.contains(infinity))
    };

    // TODO extra documentation for variants
    declarations.variants("Null", "AtOrigin", is_not_flat, |sig| sig.contains(origin) && !sig.contains(infinity), tangent_null_cone);
    declarations.variants("", "OnOrigin", all, |sig| sig.contains(origin), intersects_null_cone);
    declarations.variants("", "AtInfinity", is_flat, |sig| !sig.contains(origin), tangent_null_cone);
    declarations.variants("", "AtOrigin", is_not_flat, |sig| sig.contains(origin) ^ sig.contains(infinity), intersects_null_cone);
    declarations.variants("", "AligningOrigin", is_not_flat, |sig| sig.contains(origin) || sig.contains(infinity), intersects_null_cone);
    declarations.variants("", "OrthogonalOrigin", is_not_flat, |sig| !sig.contains(flat_origin), intersects_null_cone);
    declarations.variants("", "AtInfinity", is_not_flat, |sig| !sig.contains(origin) || sig.contains(flat_origin), intersects_null_cone);
    declarations.generate_missing_duals();
    declarations.finished()
}



pub static Plane_BulkExpansion_Plane: Specialized_22<e12345, MultiVector>
    = BulkExpansion.specialize(&Plane, &Plane, &|mut b, slf, other| Box::pin(async move {
        // TODO actually implement
        b.return_expr(slf)
    }));