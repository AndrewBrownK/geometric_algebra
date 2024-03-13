
use std::io::Write;
use crate::algebra::rigid::RigidGeometricAlgebra;
use crate::{CodeGenerator, read_multi_vector_from_str, validate_glsl_and_wgsl};
use crate::algebra::dialect::Dialect;
use crate::algebra::MultiVectorClassRegistry;
use crate::emit::Emitter;

#[cfg(test)]
mod test {
    use crate::build_scripts::rga3d::script;

    #[test]
    fn test_script() {
        script().unwrap()
    }
}

pub fn script() -> std::io::Result<()> {

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

        "MultiVector:\
            1,e1234|\
            e1,e2,e3,e4|\
            e41,e42,e43|e23,e31,e12|\
            e423,e431,e412,e321"
        ];


    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().also_meet_and_join();


    let rga3d = RigidGeometricAlgebra {
        generator_squares: &[1, 1, 1, 0],
        name: "rga3d",
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

    let rga3d_name = rga3d.name.to_string();
    let mut code_gen = CodeGenerator::new(rga3d);
    code_gen.preamble_and_universal_traits(&registry).unwrap();
    code_gen.basic_norms(&registry);
    // TODO fancy norms
    // code_gen.fancy_norms()
    code_gen.post_norm_universal_stuff(&registry);
    code_gen.attitude_and_dependencies("Horizon", &registry);



    let file_path = std::path::Path::new("./src/").join(std::path::Path::new(rga3d_name.as_str()));
    let mut emitter = Emitter::new(&file_path);

    emitter.rust_collector.write_all(b"
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

")?;
    code_gen.emit_datatypes_and_external_traits(&registry, &mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("products/geometric")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;")?;
    code_gen.emit_geometric_products(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("products/exterior")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;")?;
    code_gen.emit_exterior_products(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("products/dot")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;")?;
    code_gen.emit_dot_products(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("aspects")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::products::geometric::GeometricProduct;")?;
    code_gen.emit_component_wise_aspects(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("involutions")), "
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *, rga3d::*};
use std::ops::{Add, Div, Mul, Neg, Sub};")?;
    code_gen.emit_involutions_and_duals(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("aspect_duals")), "
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *, rga3d::*};
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::rga3d::aspects::{Bulk, Weight};
use crate::rga3d::involutions::*;")?;
    code_gen.emit_aspect_duals(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("characteristics")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::products::exterior::AntiWedge;")?;
    code_gen.emit_characteristic_features(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("norms")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::characteristics::Sqrt;
use crate::rga3d::products::dot::{AntiDot, Dot};")?;
    code_gen.emit_norms(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("unitize")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::norms::WeightNorm;
use crate::rga3d::products::geometric::GeometricProduct;")?;
    code_gen.emit_unitize(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("products/isometries")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::unitize::Unitize;
use crate::rga3d::involutions::AntiReversal;
use crate::rga3d::products::geometric::GeometricAntiProduct;")?;
    code_gen.emit_isometries(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("products/contractions")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::aspect_duals::*;
use crate::rga3d::products::exterior::AntiWedge;")?;
    code_gen.emit_contractions(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("products/expansions")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::aspect_duals::*;
use crate::rga3d::products::exterior::Wedge;")?;
    code_gen.emit_expansions(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("products/projections")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::products::exterior::Wedge;
use crate::rga3d::products::exterior::AntiWedge;
use crate::rga3d::products::contractions::*;
use crate::rga3d::products::expansions::*;
use crate::rga3d::aspect_duals::*;")?;
    code_gen.emit_projections_and_stuff(&mut emitter)?;


    emitter.with_new_rust_collector(&file_path.join(std::path::Path::new("metrics")), "
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::unitize::Unitize;
use crate::rga3d::products::exterior::Wedge;
use crate::rga3d::characteristics::Attitude;
use crate::rga3d::products::projections::*;
use crate::rga3d::norms::*;
use crate::rga3d::products::contractions::WeightContraction;")?;
    code_gen.emit_metric_operations(&mut emitter)?;





    validate_glsl_and_wgsl("rga3d", file_path);
    Ok(())
}