#![feature(try_blocks)]

use std::io::Write;
use ::codegen;
use codegen::algebra::rigid::RigidGeometricAlgebra;
use codegen::{CodeGenerator, read_multi_vector_from_str};
use codegen::algebra::dialect::Dialect;
use codegen::algebra::MultiVectorClassRegistry;
use codegen::emit::Emitter;

fn main() {

    let mv_iter =
        "\
        Scalar:1;\
        AntiScalar:e1234;\
        Magnitude:1,e1234;\
        \
        Point:e1,e2,e3,e4;\
        Origin:e4;\
        PointAtInfinity:e1,e2,e3;\
        \
        Line:e41,e42,e43|e23,e31,e12;\
        LineAtOrigin:e41,e42,e43;\
        LineAtInfinity:e23,e31,e12;\
        \
        Plane:e423,e431,e412,e321;\
        PlaneAtOrigin:e423,e431,e412;\
        Horizon:e321;\
        \
        Motor:e41,e42,e43,e1234|e23,e31,e12;\
        Rotor:e41,e42,e43,e1234;\
        Translator:e23,e31,e12,e1234;\
        \
        Flector:e1,e2,e3,e4|e423,e431,e412,e321;\
        \
        MultiVector:\
            1,e1234|\
            e1,e2,e3,e4|\
            e41,e42,e43|e23,e31,e12|\
            e423,e431,e412,e321\
        ".split(';');


    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().contractions().also_meet_and_join();


    let rga3d = RigidGeometricAlgebra {
        generator_squares: &[1, 1, 1, 0],
        name: "rga3d",
        dialect,
    };

    let mut registry = MultiVectorClassRegistry::default();
    for multi_vector_descriptor in mv_iter {
        let mv = read_multi_vector_from_str(multi_vector_descriptor, &rga3d);
        registry.register(mv);
    }

    let rga3d_name = rga3d.name.to_string();
    let mut code_gen = CodeGenerator::new(rga3d);
    // code_gen.lifetime_debug(&registry, &mut emitter);
    // code_gen.lifetime_debug2(&registry, &mut emitter);
    code_gen.preamble_and_universal_traits(&registry).unwrap();
    code_gen.basic_norms(&registry);
    // TODO fancy norms
    // code_gen.fancy_norms()
    code_gen.post_norm_universal_stuff(&registry);
    code_gen.attitude_and_dependencies("Horizon", &registry);


     let result: std::io::Result<()> = try {


         let file_path = std::path::Path::new("./src/").join(std::path::Path::new(rga3d_name.as_str()));
         let mut rust_emitter = Emitter::new_rust_only(&file_path);
         let mut glsl_emitter = Emitter::new_glsl_only(&file_path);
         let mut wgsl_emitter = Emitter::new_wgsl_only(&file_path);

         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Component-wise aspects to objects, and Unitization
pub mod aspects;

/// Products
pub mod products {
    pub mod geometric;
    pub mod exterior;
    pub mod remaining;
    pub mod dot;

    /// Isometries
    pub mod isometries;
}

/// Involutions and Duals
pub mod involutions;

/// Norms
pub mod norms;

/// Characteristic Features
pub mod characteristics;

/// Projections
pub mod projections;

/// Metric operations
pub mod metrics;

")?;


         code_gen.emit_datatypes_and_external_traits(&registry, &mut rust_emitter)?;
         code_gen.emit_datatypes_and_external_traits(&registry, &mut glsl_emitter)?;
         code_gen.emit_datatypes_and_external_traits(&registry, &mut wgsl_emitter)?;





         let module_path = file_path.join(std::path::Path::new("products/geometric"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;

")?;
         code_gen.emit_geometric_products(&mut rust_emitter)?;
         code_gen.emit_geometric_products(&mut glsl_emitter)?;
         code_gen.emit_geometric_products(&mut wgsl_emitter)?;





         let module_path = file_path.join(std::path::Path::new("products/exterior"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;

")?;
         code_gen.emit_exterior_products(&mut rust_emitter)?;
         code_gen.emit_exterior_products(&mut glsl_emitter)?;
         code_gen.emit_exterior_products(&mut wgsl_emitter)?;



         let module_path = file_path.join(std::path::Path::new("products/dot"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;

")?;
         code_gen.emit_dot_products(&mut rust_emitter)?;
         code_gen.emit_dot_products(&mut glsl_emitter)?;
         code_gen.emit_dot_products(&mut wgsl_emitter)?;




         // TODO replace or rename the remaining products, and give them a less weird module name

         let module_path = file_path.join(std::path::Path::new("products/remaining"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;

")?;
         code_gen.emit_remaining_products(&mut rust_emitter)?;
         code_gen.emit_remaining_products(&mut glsl_emitter)?;
         code_gen.emit_remaining_products(&mut wgsl_emitter)?;








         let module_path = file_path.join(std::path::Path::new("products/isometries"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::aspects::Unitize;
use crate::rga3d::involutions::AntiReversal;
use crate::rga3d::products::geometric::GeometricAntiProduct;

")?;

         code_gen.emit_isometries(&mut rust_emitter)?;
         code_gen.emit_isometries(&mut glsl_emitter)?;
         code_gen.emit_isometries(&mut wgsl_emitter)?;

         let module_path = file_path.join(std::path::Path::new("aspects"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::norms::WeightNorm;
use crate::rga3d::products::geometric::GeometricProduct;

")?;

         code_gen.emit_component_wise_aspects(&mut rust_emitter)?;
         code_gen.emit_component_wise_aspects(&mut glsl_emitter)?;
         code_gen.emit_component_wise_aspects(&mut wgsl_emitter)?;

         let module_path = file_path.join(std::path::Path::new("involutions"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *, rga3d::*};
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::rga3d::aspects::{Bulk, Weight};

")?;

         code_gen.emit_involutions_and_duals(&mut rust_emitter)?;
         code_gen.emit_involutions_and_duals(&mut glsl_emitter)?;
         code_gen.emit_involutions_and_duals(&mut wgsl_emitter)?;

         let module_path = file_path.join(std::path::Path::new("norms"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::characteristics::Sqrt;
use crate::rga3d::products::dot::{AntiDot, Dot};

")?;

         code_gen.emit_norms(&mut rust_emitter)?;
         code_gen.emit_norms(&mut glsl_emitter)?;
         code_gen.emit_norms(&mut wgsl_emitter)?;

         let module_path = file_path.join(std::path::Path::new("characteristics"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::products::exterior::AntiWedge;

")?;

         code_gen.emit_characteristic_features(&mut rust_emitter)?;
         code_gen.emit_characteristic_features(&mut glsl_emitter)?;
         code_gen.emit_characteristic_features(&mut wgsl_emitter)?;

         let module_path = file_path.join(std::path::Path::new("projections"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::products::exterior::Wedge;
use crate::rga3d::products::exterior::AntiWedge;
use crate::rga3d::involutions::*;

")?;

         code_gen.emit_projections_and_stuff(&mut rust_emitter)?;
         code_gen.emit_projections_and_stuff(&mut glsl_emitter)?;
         code_gen.emit_projections_and_stuff(&mut wgsl_emitter)?;

         let module_path = file_path.join(std::path::Path::new("metrics"));
         let mut rust_emitter = Emitter::new_rust_only(&module_path);
         rust_emitter.rust_collector.as_mut().unwrap().write_all(b"
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::aspects::Unitize;
use crate::rga3d::products::exterior::Wedge;
use crate::rga3d::characteristics::Attitude;
use crate::rga3d::projections::*;
use crate::rga3d::norms::*;

")?;

         code_gen.emit_metric_operations(&mut rust_emitter)?;
         code_gen.emit_metric_operations(&mut glsl_emitter)?;
         code_gen.emit_metric_operations(&mut wgsl_emitter)?;

    };
    result.unwrap();
}