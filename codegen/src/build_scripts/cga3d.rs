use crate::algebra::conformal::ConformalGeometricAlgebra;
use crate::algebra::dialect::Dialect;
use crate::algebra::MultiVectorClassRegistry;
use crate::emit::Emitter;
use crate::{read_multi_vector_from_str, CodeGenerator};
use std::io::Write;

const CGA3D: &str = "cga3d";

pub fn script() -> std::io::Result<()> {
    script_custom(true, "")
}

//noinspection DuplicatedCode
fn script_custom(actually_emit: bool, path_prefix: &str) -> std::io::Result<()> {
    // TODO more precise rerun conditions
    //  https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script

    let mv_iter = [
        "Scalar:1",
        "AntiScalar:e12345",
        "Magnitude:1,e12345",
        "Point:e15,e25,e35,e45",
        "Point/Origin:e45",
        "Point/PointAtInfinity:e15,e25,e35",
        "Line:e415,e425,e435|e235,e315,e125",
        "Line/LineAtOrigin:e415,e425,e435",
        "Line/LineAtInfinity:e235,e315,e125",
        "Plane:e4235,e4315,e4125,e3215",
        "Plane/PlaneAtOrigin:e4235,e4315,e4125",
        "Plane/Horizon:e3215",
        // TODO not yet sure on what kinds of "superclassing" I want with round objects
        //  it partly depends on how the sandwich operators (are supposed to) turn out
        //  Maybe the flat objects should be subclassed to these round objects,
        //  or maybe there are yet other round variants of objects subclassed here
        "Radial:e1,e2,e3|e4,e5",
        "Dipole:e41,e42,e43|e23,e31,e12|e15,e25,e35,e45",
        "Circle:e423,e431,e412,e321|e415,e425,e435|e235,e315,e125",
        "Sphere:e4235,e4315,e4125|e1234,e3215",
        // TODO figure out these objects
        // "Motor:e41,e42,e43,e1234|e23,e31,e12",
        // "Motor/Rotor:e41,e42,e43,e1234",
        // "Motor/Translator:e23,e31,e12,e1234",
        //
        // "Flector:e1,e2,e3,e4|e423,e431,e412,e321",
        "MultiVector:\
            1,e12345|\
            e1,e2,e3|e4,e5|\
            e41,e42,e43|e23,e31,e12|e15,e25,e35,e45|\
            e423,e431,e412,e321|e415,e425,e435|e235,e315,e125|\
            e4235,e4315,e4125|e1234,e3215",
    ];

    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().also_meet_and_join();

    let cga3d = ConformalGeometricAlgebra::new(CGA3D, 3, dialect);

    let mut registry = MultiVectorClassRegistry::default();
    for multi_vector_descriptor in mv_iter {
        let (mv, sc) = read_multi_vector_from_str(multi_vector_descriptor, &cga3d);
        if let Some(sc) = sc {
            registry.register_with_superclass(mv, sc);
        } else {
            registry.register(mv);
        }
    }
    let mut code_gen = CodeGenerator::new(cga3d);
    code_gen.preamble_and_universal_traits(&registry).unwrap();
    code_gen.basic_norms(&registry);
    // TODO fancy norms
    // code_gen.fancy_norms()
    code_gen.post_norm_universal_stuff(&registry);
    code_gen.attitude_and_dependencies("Horizon", &registry);

    let mut file_path = std::path::Path::new("src/").join(std::path::Path::new(CGA3D));
    if !path_prefix.is_empty() {
        file_path = std::path::Path::new(path_prefix).join(file_path);
    }
    let file_path = file_path;

    let mut emitter = Emitter::new(actually_emit, &file_path);

    emitter.rust_collector.write_all(
        b"
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod aspects;
pub mod aspect_duals;
pub mod involutions;
pub mod unitize;
pub mod norms;
pub mod characteristics;
pub mod metrics;
pub mod products {
    pub mod geometric;
    pub mod exterior;
    pub mod contractions;
    pub mod expansions;
    pub mod projections;
    pub mod dot;
    pub mod isometries;
}

",
    )?;
    code_gen.emit_datatypes_and_external_traits(&registry, &mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("products/geometric")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;",
    )?;
    code_gen.emit_geometric_products(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("products/exterior")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;",
    )?;
    code_gen.emit_exterior_products(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("products/dot")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;",
    )?;
    code_gen.emit_dot_products(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("aspects")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::products::geometric::GeometricProduct;",
    )?;
    code_gen.emit_component_wise_aspects(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("involutions")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *, cga3d::*};
use std::ops::{Add, Div, Mul, Neg, Sub};",
    )?;
    code_gen.emit_involutions_and_duals(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("aspect_duals")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *, cga3d::*};
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::cga3d::aspects::{Bulk, Weight};
use crate::cga3d::involutions::*;",
    )?;
    code_gen.emit_aspect_duals(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("characteristics")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::products::exterior::AntiWedge;",
    )?;
    code_gen.emit_characteristic_features(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("norms")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::characteristics::Sqrt;
use crate::cga3d::products::dot::{AntiDot, Dot};",
    )?;
    code_gen.emit_norms(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("unitize")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::norms::WeightNorm;
use crate::cga3d::products::geometric::GeometricProduct;",
    )?;
    code_gen.emit_unitize(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("products/isometries")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::unitize::Unitize;
use crate::cga3d::involutions::AntiReversal;
use crate::cga3d::products::geometric::GeometricAntiProduct;",
    )?;
    code_gen.emit_isometries(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("products/contractions")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::aspect_duals::*;
use crate::cga3d::products::exterior::AntiWedge;",
    )?;
    code_gen.emit_contractions(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("products/expansions")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::aspect_duals::*;
use crate::cga3d::products::exterior::Wedge;",
    )?;
    code_gen.emit_expansions(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("products/projections")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::products::exterior::Wedge;
use crate::cga3d::products::exterior::AntiWedge;
use crate::cga3d::products::contractions::*;
use crate::cga3d::products::expansions::*;
use crate::cga3d::aspect_duals::*;",
    )?;
    code_gen.emit_projections_and_stuff(&mut emitter)?;

    emitter.with_new_rust_collector(
        &file_path.join(std::path::Path::new("metrics")),
        "
#![allow(clippy::assign_op_pattern)]
use crate::cga3d::*;
use crate::cga3d::unitize::Unitize;
use crate::cga3d::products::exterior::Wedge;
use crate::cga3d::characteristics::Attitude;
use crate::cga3d::products::projections::*;
use crate::cga3d::norms::*;
use crate::cga3d::products::contractions::WeightContraction;",
    )?;
    code_gen.emit_metric_operations(&mut emitter)?;

    // GLSL validation can stack overflow when ran in a build script (requires fix in Naga).
    // However, it is fine in a test (must be larger stack size).
    // So we will not validate here, and just use tests instead.
    // validate_glsl(CGA3D, file_path.clone());
    // validate_wgsl(CGA3D, file_path);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::build_scripts::cga3d::{script_custom, CGA3D};
    use crate::{validate_glsl, validate_wgsl};
    use std::path::Path;

    const OTHER_CRATE: &str = "../geometric_algebra/";

    // #[test]
    // fn build_with_disk_writes() {
    //     script_custom(true, OTHER_CRATE).unwrap()
    // }

    #[test]
    fn build_without_disk_writes() {
        script_custom(false, OTHER_CRATE).unwrap()
    }

    #[test]
    fn glsl_validation() {
        let file_path = Path::new(OTHER_CRATE).join(Path::new("src/").join(CGA3D));
        validate_glsl(CGA3D, file_path);
    }

    #[test]
    fn wgsl_validation() {
        let file_path = Path::new(OTHER_CRATE).join(Path::new("src/").join(CGA3D));
        validate_wgsl(CGA3D, file_path);
    }
}
