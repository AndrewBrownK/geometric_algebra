use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use crate::algebra::dialect::Dialect;
use crate::algebra::rigid::RigidGeometricAlgebra;
use crate::algebra::MultiVectorClassRegistry;
use crate::compile::CodeGenerator;
use crate::emit::Emitter;
use crate::algebra::read_multi_vector_from_str;

#[cfg(test)]
mod test;

const RGA3D: &str = "rga3d";
const RGA3D_CRATE_PREFIX: &str = "rga3d/";

pub fn script() -> std::io::Result<()> {
    script_custom(true, RGA3D_CRATE_PREFIX)
}

//noinspection DuplicatedCode
fn script_custom(actually_emit: bool, path_prefix: &str) -> std::io::Result<()> {

    let mv_iter = [
        "Scalar:1",
        "AntiScalar:e1234",
        "Magnitude:1,e1234",
        "Point:e1,e2,e3,e4",
        "Point/Origin:e4",
        "Point/PointAtInfinity:e1,e2,e3",
        "Line:e41,e42,e43|e23,e31,e12",
        "Line/LineAtOrigin:e41,e42,e43",
        "Line/LineAtInfinity:e23,e31,e12",
        "Plane:e423,e431,e412,e321",
        "Plane/PlaneAtOrigin:e423,e431,e412",
        "Plane/Horizon:e321",
        "Motor:e41,e42,e43,e1234|e23,e31,e12",
        "Motor/Rotor:e41,e42,e43,e1234",
        "Motor/Translator:e23,e31,e12,e1234",
        "Flector:e1,e2,e3,e4|e423,e431,e412,e321",
        "Flector/FlectorAtInfinity:e1,e2,e3,e321",
        "MultiVector:\
            1,e1234|\
            e1,e2,e3,e4|\
            e41,e42,e43|e23,e31,e12|\
            e423,e431,e412,e321",
        "MultiVectorAtOrigin:e4,e1234|e41,e42,e43|e423,e431,e412",
        "MultiVectorAtInfinity:1,e321|e1,e2,e3|e23,e31,e12"
    ];

    // On sandwich products, assume that the output
    // class is the same as the input class, unless it is
    // in the following guide
    let sandwich_outputs: BTreeMap<(&str, &str), &str> = [

        // Rotations of objects at origin are still objects at origin
        // But as you can see, the inputs and outputs are the same,
        // so we do not need to add special guidance here. The input
        // and output being the same is the default assumption.

        // (("Rotor", "Origin"), "Origin"),
        // (("Rotor", "LineAtOrigin"), "LineAtOrigin"),
        // (("Rotor", "PlaneAtOrigin"), "PlaneAtOrigin"),

        // In contrast to rotations, translations of objects at origin
        // are not objects at origin. Therefore, we must add the special
        // guidance on output types here.

        (("Translator", "Origin"), "Point"),
        (("Translator", "LineAtOrigin"), "Line"),
        (("Translator", "PlaneAtOrigin"), "Plane"),
        (("Translator", "Rotor"), "Motor"),

        // And obviously motor outputs must be at least as general as translator outputs

        (("Motor", "Origin"), "Point"),
        (("Motor", "LineAtOrigin"), "Line"),
        (("Motor", "PlaneAtOrigin"), "Plane"),
        (("Motor", "Rotor"), "Motor"),

    ].into_iter().collect();

    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().also_meet_and_join();

    let rga3d = RigidGeometricAlgebra {
        generator_squares: &[1, 1, 1, 0],
        name: RGA3D,
        dialect,
    };

    let mut registry = MultiVectorClassRegistry::default();
    for multi_vector_descriptor in mv_iter {
        let (mv, sc) = read_multi_vector_from_str(multi_vector_descriptor, &rga3d);
        if let Some(sc) = sc {
            registry.register_with_superclass(mv, sc);
        } else {
            registry.register(mv);
        }
    }

    let mut code_gen = CodeGenerator::new(rga3d);
    code_gen.preamble_and_universal_traits(&registry).unwrap();
    code_gen.basic_norms(&registry);
    code_gen.post_norm_universal_stuff(&registry);
    code_gen.attitude_and_dependencies("Horizon", &registry);

    let mut file_path = Path::new("src/").to_path_buf();
    if !path_prefix.is_empty() {
        file_path = Path::new(path_prefix).join(file_path);
    }
    let file_path = file_path;
    let mut emitter = Emitter::new(actually_emit, &file_path, "lib", RGA3D);
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
use crate::aspects::{Bulk, Weight};
use crate::involutions::*;",
    )?;
    code_gen.emit_aspect_duals(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("characteristics")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::AntiWedge;
use crate::products::dot::Dot;
use crate::products::dot::AntiDot;
use crate::products::geometric::GeometricProduct;
use crate::products::geometric::GeometricAntiProduct;",
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
    // validate_glsl(RGA3D, file_path.clone());
    // validate_wgsl(RGA3D, file_path);

    Ok(())
}
