use std::collections::BTreeMap;
use std::path::Path;

use crate::algebra::conformal::ConformalGeometricAlgebra;
use crate::algebra::dialect::Dialect;
use crate::algebra::{read_multi_vector_from_str, MultiVectorClassRegistry};
use crate::compile::CodeGenerator;
use crate::emit::Emitter;
use crate::shader_support::emit_shader_support;

pub fn cga_script(
    path_prefix: &str,
    name: &'static str,
    dialect: Dialect,
    dimensions: usize,
    actually_emit: bool,
    multi_vector_classes: &[&str],
    sandwich_outputs: BTreeMap<(&str, &str), &str>,
) -> std::io::Result<()> {
    let cga_nd = ConformalGeometricAlgebra::new(name, dimensions, dialect);

    let mut registry = MultiVectorClassRegistry::default();
    for multi_vector_descriptor in multi_vector_classes {
        registry.register(read_multi_vector_from_str(multi_vector_descriptor, &cga_nd));
    }
    let mut code_gen = CodeGenerator::new(cga_nd);
    code_gen.preamble_and_universal_traits(&registry).unwrap();
    code_gen.dual_num_stuff(&registry).unwrap();
    code_gen.basic_norms(&registry);
    code_gen.post_norm_universal_stuff(&registry, &sandwich_outputs);
    code_gen.round_features(&registry);
    code_gen.fancy_norms(&registry);
    code_gen.attitude_and_dependencies(&registry);

    // TODO conformal conjugate (page 204-205)

    // TODO impose constraints on page 235
    //  The gist is that the flat half of the object must be contained in the carrier of the object,
    //  which is specified by the round half of the object. So this means you can't combine
    //  any arbitrary round half and flat half, they have to be compatible. There's a tangential
    //  question of "what even is the object if the constraint is violated?", but setting that
    //  aside from now, how can we impose constraints? Well this hearkens to another question,
    //  which is "what are the valid ways to construct an object?" The answer to that question is,
    //  it is preferred to construct objects using the join operation. If you only construct
    //  objects that way, then hypothetically they should stay valid and maintain the constraints.
    //  However let's suppose you construct an object with raw coefficients, or otherwise
    //  violate the constraints somehow anyway. What then? Well from here you basically have two
    //  choices. You can decide if the carrier or flat part is more correct, and which type of
    //  (anti)projection you want to use to bring the wrong half in line with the correct half.

    let mut file_path = Path::new("src/").to_path_buf();
    if !path_prefix.is_empty() {
        file_path = Path::new(path_prefix).join(file_path);
    }
    let file_path = file_path;

    let mut emitter = Emitter::new(actually_emit, &file_path, "lib", name);
    emitter.emit_shader_preamble(&code_gen.algebra.name)?;

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
pub mod shaders;
#[cfg(test)]
pub mod test;
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
use crate::products::geometric::*;",
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
use crate::aspects::{FlatBulk, FlatWeight, RoundBulk, RoundWeight};
use crate::involutions::*;",
    )?;
    code_gen.emit_aspect_duals(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("characteristics")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::*;
use crate::products::dot::*;
use crate::products::geometric::*;
use crate::involutions::*;",
    )?;
    code_gen.emit_characteristic_features(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("norms")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::characteristics::*;
use crate::aspects::*;
use crate::involutions::*;
use crate::products::exterior::*;
use crate::products::dot::*;",
    )?;
    code_gen.emit_norms(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("unitize")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::norms::RoundWeightNorm;
use crate::products::geometric::*;",
    )?;
    code_gen.emit_unitize(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/isometries")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::unitize::Unitize;
use crate::involutions::AntiReversal;
use crate::products::geometric::*;",
    )?;
    code_gen.emit_isometries(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/quotients")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::characteristics::Inverse;
use crate::characteristics::AntiInverse;
use crate::products::geometric::*;",
    )?;
    code_gen.emit_quotients(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/contractions")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::involutions::*;
use crate::products::exterior::*;",
    )?;
    code_gen.emit_contractions(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/expansions")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::involutions::*;
use crate::products::exterior::*;",
    )?;
    code_gen.emit_expansions(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/projections")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::involutions::*;",
    )?;
    code_gen.emit_projections_and_stuff(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/rejections")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::involutions::*;",
    )?;
    code_gen.emit_rejections_and_stuff(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("products/supports")));
    emitter.emit_rust_preamble(
        "
use crate::*;
use crate::products::exterior::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::involutions::*;",
    )?;
    code_gen.emit_supports(&mut emitter)?;

    emitter.new_rust_collector(&file_path.join(Path::new("metrics")));
    emitter.emit_rust_preamble(
        "
use crate::characteristics::{Attitude, Sqrt};
use crate::norms::*;
use crate::products::exterior::*;
use crate::products::projections::*;
use crate::unitize::Unitize;
use crate::*;
use crate::involutions::AntiDual;
use crate::products::geometric::*;",
    )?;
    code_gen.emit_metric_operations(&mut emitter)?;

    emit_shader_support(&mut emitter, &file_path, &code_gen.algebra.name)?;
    emitter.end_with_rust_fmt();

    // GLSL validation can stack overflow when ran in a build script (requires fix in Naga).
    // However, it is fine in a test (must be larger stack size).
    // So we will not validate here, and just use tests instead.
    // validate_glsl(CGA3D, file_path.clone());
    // validate_wgsl(CGA3D, file_path);

    Ok(())
}
