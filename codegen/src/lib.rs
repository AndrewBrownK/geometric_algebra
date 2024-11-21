#![feature(try_blocks)]
#![feature(iter_intersperse)]
#![feature(exit_status_error)]
#![feature(associated_type_defaults)]
#![feature(const_option)]
// TODO evaluate if I actually need/use these
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(const_trait_impl, effects)]
#![feature(marker_trait_attr)]
#![feature(const_mut_refs)]
#![feature(concat_idents)]
#![feature(const_swap)]
#![feature(box_patterns)]


// TODO rendering round objects in polygons (triangles)
//  https://www.youtube.com/watch?v=VEnglRKNHjU


// TODO calculus: https://en.wikipedia.org/wiki/Geometric_calculus
pub mod algebra2;
pub mod ast2;
pub mod emit2;
mod shader_support;
pub mod utility;
mod validate;
// TODO eventually migrate to Portable SIMD once it is stabilized
//  https://github.com/rust-lang/rust/issues/86656
//  That would also be the appropriate time to consider f64 support. Not eager until then.
mod simd;
pub mod build_scripts2 {
    pub mod common_traits;
    mod pga3d;
    mod rga3d;
    mod uwu;
}

const SIMD_SRC: &'static str = include_str!("simd.rs");

pub use emit2::rust::Rust;
pub use emit2::wgsl::Wgsl;
pub mod elements {
    pub use crate::algebra2::basis::elements::*;
}
