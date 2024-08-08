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

mod algebra;
mod ast;
mod compile;
mod emit;
mod impls;
mod validate;
mod shader_support;
pub mod ast2;
pub mod algebra2;
mod utility;
pub mod emit2;
// TODO eventually migrate to Portable SIMD once it is stabilized https://github.com/rust-lang/rust/issues/86656
mod simd;
mod build_scripts2 {
    pub mod common_traits;
    pub mod cga3d;
    pub mod rga3d;
    pub mod uwu;
}

mod build_scripts {
    pub mod cga;
    pub mod rga;
    pub mod cga3d;
    pub mod cga3d_min;
    pub mod rga3d;
}

const SIMD_SRC: &'static str = include_str!("simd.rs");

// TODO documentation on this stuff
#[cfg(all(feature = "wedge-is-join", feature = "wedge-is-meet"))]
compile_error!(
    "You must use crate features in your cargo.toml to choose if you want the wedge product \
    to be a meet (wedge-is-meet) or join (wedge-is-join). See documentation for more \
    information on this decision."
);


fn main() {
    let result: std::io::Result<()> = try {
        // TODO create 2d and 4d variants

        build_scripts::rga3d::script()?;
        build_scripts::cga3d::script()?;
        build_scripts::cga3d_min::script()?;
    };
    result.expect("Must build successfully");
}
