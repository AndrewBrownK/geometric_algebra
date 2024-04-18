use std::collections::BTreeMap;
use crate::algebra::conformal::ConformalGeometricAlgebra;
use crate::algebra::dialect::Dialect;
use crate::algebra::{GeometricAlgebraTrait, MultiVectorClassRegistry};
use crate::emit::Emitter;
use crate::algebra::read_multi_vector_from_str;
use std::io::Write;
use std::path::Path;
use crate::compile::CodeGenerator;

#[cfg(test)]
mod test;

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
        "DualNum:1,e12345",

        "FlatPoint:e15,e25,e35,e45",
        "FlatPointAtOrigin:e45",
        "FlatPointAtInfinity:e15,e25,e35",

        "Line:e415,e425,e435|e235,e315,e125",
        "LineAtOrigin:e415,e425,e435",
        "LineAtInfinity:e235,e315,e125",

        "Plane:e4235,e4315,e4125,e3215",
        "PlaneAtOrigin:e4235,e4315,e4125",
        "Horizon:e3215",

        "RoundPoint:e1,e2,e3|e4,e5",
        "Dipole:e41,e42,e43|e23,e31,e12|e15,e25,e35,e45",
        "Circle:e423,e431,e412,e321|e415,e425,e435|e235,e315,e125",
        "Sphere:e4235,e4315,e4125|e1234,e3215",

        "Infinity:e5",
        "Origin:e4",
        "RoundPointAtOrigin:e4,e5",
        "RoundPointAtInfinity:e1,e2,e3,e5",

        // TODO can I get more interesting/intuitive names for these?
        "RoundPointBulk:e1,e2,e3",
        "RoundPointCarrierAspect:e1,e2,e3,e4",
        "DipoleBulk:e23,e31,e12",
        "DipoleWeight:e41,e42,e43",
        "DipoleCarrierAspect:e41,e42,e43|e23,e31,e12",
        "CircleBulk:e321",
        "CircleWeight:e423,e431,e412",
        "CircleCarrierAspect:e423,e431,e412,e321",
        "SphereWeight:e1234",


        // Operator Objects
        "Motor:e415,e425,e435,e12345|e235,e315,e125",
        "Rotor:e415,e425,e435,e12345",
        "Translator:e235,e315,e125,e12345",
        "Flector:e15,e25,e35,e45|e4235,e4315,e4125,e3215",
        "Transflector:e15,e25,e35|e4235,e4315,e4125,e3215",
        "FlectorAtInfinity:e15,e25,e35,e3215",

        "MultiVector:\
            1,e12345|\
            e1,e2,e3|e4,e5|\
            e41,e42,e43|e23,e31,e12|e15,e25,e35,e45|\
            e423,e431,e412,e321|e415,e425,e435|e235,e315,e125|\
            e4235,e4315,e4125|e1234,e3215",
    ];

    // TODO add more of these if/where applicable to CGA
    let sandwich_outputs: BTreeMap<(&str, &str), &str> = [

        ("Translator", "Origin", "Point"),
        ("Translator", "LineAtOrigin", "Line"),
        ("Translator", "PlaneAtOrigin", "Plane"),
        ("Translator", "Rotor", "Motor"),

        ("Motor", "Origin", "Point"),
        ("Motor", "LineAtOrigin", "Line"),
        ("Motor", "PlaneAtOrigin", "Plane"),
        ("Motor", "Rotor", "Motor"),

        ("Flector", "Origin", "Point"),
        ("Flector", "LineAtOrigin", "Line"),
        ("Flector", "PlaneAtOrigin", "Plane"),
        ("Flector", "Rotor", "Motor"),

        ("FlectorAtInfinity", "Origin", "Point"),
        ("FlectorAtInfinity", "LineAtOrigin", "Line"),
        ("FlectorAtInfinity", "PlaneAtOrigin", "Plane"),
        ("FlectorAtInfinity", "Rotor", "Motor"),

    ].into_iter().map(|it| ((it.0, it.1), it.2)).collect();

    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().also_meet_and_join();

    let cga3d = ConformalGeometricAlgebra::new(CGA3D, 3, dialect);

    let mut registry = MultiVectorClassRegistry::default();
    for multi_vector_descriptor in mv_iter {
        registry.register(read_multi_vector_from_str(multi_vector_descriptor, &cga3d));
    }
    let flat_basis = cga3d.parse("e5");
    let mut code_gen = CodeGenerator::new(cga3d);
    code_gen.preamble_and_universal_traits(&registry).unwrap();
    code_gen.basic_norms(&registry);
    code_gen.post_norm_universal_stuff(&registry, &sandwich_outputs);
    code_gen.round_features(flat_basis, &registry);
    code_gen.fancy_norms(&registry);
    code_gen.attitude_and_dependencies("Horizon", &registry);
    // TODO see loads of DualNum operations on page 126
    // TODO conjugates, page 204
    // TODO impose constraints on page 235

    let mut file_path = Path::new("src/").to_path_buf();
    if !path_prefix.is_empty() {
        file_path = Path::new(path_prefix).join(file_path);
    }
    let file_path = file_path;

    let mut emitter = Emitter::new(actually_emit, &file_path, "lib", CGA3D);
    emitter.emit_shader_preamble()?;

    emitter.emit_rust_preamble(
        "
use projective_ga::{simd::*, *};
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
    pub mod rejections;
    pub mod supports;
    pub mod dot;
    pub mod isometries;
    pub mod quotients;
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
use projective_ga::{simd::*, *};
use crate::*;
use std::ops::{Add, Div, Mul, Neg, Sub};",
    )?;
    code_gen.emit_involutions_and_duals(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("aspect_duals")));
    emitter.emit_rust_preamble(
        "
use projective_ga::{simd::*, *};
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
use crate::products::dot::Dot;
use crate::products::dot::AntiDot;
use crate::products::geometric::GeometricProduct;
use crate::products::geometric::GeometricAntiProduct;
use crate::involutions::*;",
    )?;
    code_gen.emit_characteristic_features(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("norms")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::characteristics::*;
use crate::aspects::*;
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

    emitter.new_rust_collector(&file_path.join(Path::new("products/quotients")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::characteristics::Inverse;
use crate::characteristics::AntiInverse;
use crate::products::geometric::GeometricAntiProduct;
use crate::products::geometric::GeometricProduct;",
    )?;
    code_gen.emit_quotients(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/contractions")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::involutions::*;
use crate::products::exterior::AntiWedge;",
    )?;
    code_gen.emit_contractions(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/expansions")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::involutions::*;
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
use crate::involutions::*;",
    )?;
    code_gen.emit_projections_and_stuff(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/rejections")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::Wedge;
use crate::products::exterior::AntiWedge;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::involutions::*;",
    )?;
    code_gen.emit_rejections_and_stuff(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/supports")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::Wedge;
use crate::products::exterior::AntiWedge;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::involutions::*;",
    )?;
    code_gen.emit_supports(&mut emitter)?;

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
