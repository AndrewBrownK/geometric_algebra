#![feature(try_blocks)]
#![feature(iter_intersperse)]
#![feature(exit_status_error)]

mod algebra;
mod ast;
mod compile;
mod emit;
mod impls;
mod validate;
mod shader_support;

mod build_scripts {
    pub mod cga;
    pub mod rga;
    pub mod cga3d;
    pub mod cga3d_min;
    pub mod rga3d;
}

fn main() {
    let result: std::io::Result<()> = try {
        // TODO create 2d and 4d variants

        build_scripts::rga3d::script()?;
        build_scripts::cga3d::script()?;
        build_scripts::cga3d_min::script()?;
    };
    result.expect("Must build successfully");
}
