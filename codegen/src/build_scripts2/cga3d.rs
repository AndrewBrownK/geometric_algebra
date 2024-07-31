#![allow(non_upper_case_globals)]

use std::path::PathBuf;
use std::sync::Arc;

use crate::{ga, multi_vecs, operators, register_all, variants};
use crate::algebra2::basis::elements::e12345;
use crate::algebra2::basis::filter::{allow_all_signatures, SigFilter, signatures_containing};
use crate::algebra2::multivector::{DeclareMultiVecs, MultiVecRepository};
use crate::ast2::datatype::MultiVector;
use crate::ast2::impls::{Specialize_22, Specialized_22};
use crate::build_scripts2::common_traits::BulkExpansion;
use crate::emit2::FileOrganizing;
use crate::emit2::rust::Rust;

multi_vecs! { e12345;

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
    Motor      as e415, e425, e435, e12345 | e235, e315, e125, scalar;
    Flector    as e15, e25, e35, e45 | e4235, e4315, e4125, e3215;
}


/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
#[test]
pub fn cga3d_script() {
    let cga3d = ga! { e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    };
    let repo = generate_variants(register_multi_vecs(cga3d));
    let traits = register_all! { repo;
        Plane_BulkExpansion_Plane
        |
        Zero One AntiOne Unit
        Grade AntiGrade
        |
        Wedge AntiWedge GeometricProduct GeometricAntiProduct
    };
    operators! { traits;
        infix => Div;

        binary
        BitXor => Wedge,
        Mul => GeometricProduct;

        unary
        Not => Dual;
    }

    // TODO output files
    let mut file_path = PathBuf::from("../cga3d_new/src/");
    let rust = Rust { prefer_fancy_infix: false };
    let fo = FileOrganizing::recommended_for_rust("cga3d_new");
    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let file_path_2 = file_path.clone();
    rt.block_on(async move {
        if let Err(e) = fo.go_do_it(rust, file_path_2, repo, Arc::new(traits)).await {
            eprintln!("Errors: {e:?}")
        }
    });
}

fn generate_variants(mut declarations: DeclareMultiVecs<e12345>) -> Arc<MultiVecRepository<e12345>> {
    use crate::algebra2::basis::elements::*;

    let origin = signatures_containing(e4);
    let infinity = signatures_containing(e5);
    let flat_origin = origin & infinity;

    let all = allow_all_signatures();
    let is_flat = infinity.all_match();
    let is_not_flat = (!infinity).any_match();
    let tangent_null_cone = origin.any_match() ^ infinity.any_match();
    let intersects_null_cone = origin.any_match() & infinity.any_match();

    variants! { declarations;
        Null{type}AtOrigin => (origin & !infinity)  where is_not_flat => tangent_null_cone;
        {type}OnOrigin => (origin)                  where all => intersects_null_cone;
        {type}AtInfinity => (!origin)               where is_flat => tangent_null_cone;
        {type}AtOrigin => (origin ^ infinity)       where is_not_flat => intersects_null_cone;
        {type}AligningOrigin => (origin | infinity) where is_not_flat => intersects_null_cone;
        {type}OrthogonalOrigin => (!flat_origin)    where is_not_flat => intersects_null_cone;
        {type}AtInfinity => (!origin | flat_origin) where is_not_flat => intersects_null_cone;
    }

    declarations.generate_missing_duals();
    declarations.finished()
}



pub static Plane_BulkExpansion_Plane: Specialized_22<e12345, MultiVector>
    = BulkExpansion.specialize(&Plane, &Plane, &|mut b, slf, other| Box::pin(async move {
        // TODO actually implement
        b.return_expr(slf)
    }));