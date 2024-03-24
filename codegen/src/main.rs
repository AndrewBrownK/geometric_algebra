#![feature(try_blocks)]
#![feature(iter_intersperse)]
#![feature(exit_status_error)]

mod algebra;
mod ast;
mod compile;
mod emit;

mod impls;
mod validate;

mod build_scripts {
    pub mod cga3d;
    pub mod rga3d;
}

fn main() {
    let result: std::io::Result<()> = try {
        build_scripts::rga3d::script()?;
        build_scripts::cga3d::script()?;
    };
    result.expect("Must build successfully");
}
