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
    pub mod cga;
    pub mod rga;
    pub mod cga3d;
    pub mod rga3d;
}

fn main() {
    let result: std::io::Result<()> = try {
        // TODO create 2d and 4d variants
        // TODO create "min" variants (e.g. "cga3d_min") with as few named objects as possible
        //  since all the aspect slicing can seriously bloat the code and (even though there
        //  are some benefits) some people might find it intimidating, overwhelming, or unwieldly.

        build_scripts::rga3d::script()?;
        build_scripts::cga3d::script()?;
    };
    result.expect("Must build successfully");
}
