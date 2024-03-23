use crate::algebra::conformal::ConformalGeometricAlgebra;
use crate::algebra::dialect::Dialect;
use crate::algebra::{GeometricAlgebraTrait, MultiVectorClassRegistry};
use crate::emit::Emitter;
use crate::old_lib::{read_multi_vector_from_str, CodeGenerator};
use std::io::Write;
use std::path::Path;
use crate::algebra::basis_element::BasisElement;

const CGA3D: &str = "cga3d";
const CGA3D_CRATE_PREFIX: &str = "cga3d/";

pub fn script() -> std::io::Result<()> {
    script_custom(true, CGA3D_CRATE_PREFIX)
}

//noinspection DuplicatedCode
fn script_custom(actually_emit: bool, path_prefix: &str) -> std::io::Result<()> {

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
        "RoundPoint:e1,e2,e3|e4,e5",
        "Dipole:e41,e42,e43|e23,e31,e12|e15,e25,e35,e45",
        "Circle:e423,e431,e412,e321|e415,e425,e435|e235,e315,e125",
        "Sphere:e4235,e4315,e4125|e1234,e3215",

        // "RoundPoint/RoundOrigin:e4",
        "RoundPoint/Infinity:e5",

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
    let flat_basis = cga3d.parse("e5");
    let mut code_gen = CodeGenerator::new(cga3d);
    code_gen.preamble_and_universal_traits(&registry).unwrap();
    code_gen.basic_norms(&registry);

    // TODO fancy norms
    // code_gen.fancy_norms(&registry);

    code_gen.post_norm_universal_stuff(&registry);
    code_gen.round_features(flat_basis, &registry);
    code_gen.attitude_and_dependencies("Horizon", &registry);

    let mut file_path = Path::new("src/").to_path_buf();
    if !path_prefix.is_empty() {
        file_path = Path::new(path_prefix).join(file_path);
    }
    let file_path = file_path;

    let mut emitter = Emitter::new(actually_emit, &file_path, "lib", CGA3D);
    emitter.emit_shader_preamble()?;

    emitter.emit_rust_preamble(
        "
use geometric_algebra::{simd::*, *};
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
}",
    )?;
    code_gen.emit_datatypes_and_external_traits(&registry, &mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/geometric")));
    emitter.emit_rust_preamble(
        "
use crate::*;",
    )?;
    code_gen.emit_geometric_products(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/exterior")));
    emitter.emit_rust_preamble(
        "
use crate::*;",
    )?;
    code_gen.emit_exterior_products(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/dot")));
    emitter.emit_rust_preamble(
        "
use crate::*;",
    )?;
    code_gen.emit_dot_products(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("aspects")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::geometric::GeometricProduct;",
    )?;
    code_gen.emit_component_wise_aspects(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("involutions")));
    emitter.emit_rust_preamble(
        "
use geometric_algebra::{simd::*, *};
use crate::*;
use std::ops::{Add, Div, Mul, Neg, Sub};",
    )?;
    code_gen.emit_involutions_and_duals(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("aspect_duals")));
    emitter.emit_rust_preamble(
        "
use geometric_algebra::{simd::*, *};
use crate::*;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::aspects::{Bulk, Weight, RoundBulk, RoundWeight};
use crate::involutions::*;",
    )?;
    code_gen.emit_aspect_duals(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("characteristics")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::AntiWedge;
use crate::products::exterior::Wedge;
use crate::aspect_duals::*;",
    )?;
    code_gen.emit_characteristic_features(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("norms")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::characteristics::Sqrt;
use crate::products::dot::{AntiDot, Dot};",
    )?;
    code_gen.emit_norms(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("unitize")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::norms::WeightNorm;
use crate::products::geometric::GeometricProduct;",
    )?;
    code_gen.emit_unitize(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/isometries")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::unitize::Unitize;
use crate::involutions::AntiReversal;
use crate::products::geometric::GeometricAntiProduct;",
    )?;
    code_gen.emit_isometries(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/contractions")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::aspect_duals::*;
use crate::products::exterior::AntiWedge;",
    )?;
    code_gen.emit_contractions(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/expansions")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::aspect_duals::*;
use crate::products::exterior::Wedge;",
    )?;
    code_gen.emit_expansions(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/projections")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::Wedge;
use crate::products::exterior::AntiWedge;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::aspect_duals::*;",
    )?;
    code_gen.emit_projections_and_stuff(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("metrics")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::unitize::Unitize;
use crate::products::exterior::Wedge;
use crate::characteristics::Attitude;
use crate::products::projections::*;
use crate::norms::*;
use crate::products::contractions::WeightContraction;",
    )?;
    code_gen.emit_metric_operations(&mut emitter)?;

    emitter.end_with_rust_fmt();

    // GLSL validation can stack overflow when ran in a build script (requires fix in Naga).
    // However, it is fine in a test (must be larger stack size).
    // So we will not validate here, and just use tests instead.
    // validate_glsl(CGA3D, file_path.clone());
    // validate_wgsl(CGA3D, file_path);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::build_scripts::cga3d::{script_custom, CGA3D, CGA3D_CRATE_PREFIX};
    use crate::old_lib::{validate_glsl, validate_wgsl};
    use std::path::Path;

    #[test]
    fn build_without_disk_writes() {
        script_custom(false, CGA3D_CRATE_PREFIX).unwrap()
    }

    #[test]
    fn glsl_validation() {
        let file_path = Path::new(CGA3D_CRATE_PREFIX).join(Path::new("src/").join(CGA3D));
        validate_glsl(CGA3D, file_path);
    }

    #[test]
    fn wgsl_validation() {
        let file_path = Path::new(CGA3D_CRATE_PREFIX).join(Path::new("src/").join(CGA3D));
        validate_wgsl(CGA3D, file_path);
    }
}
