#![feature(try_blocks)]
#![feature(iter_intersperse)]
mod algebra;
mod ast;
mod compile;
pub mod emit;

mod old_lib;

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
