#![feature(try_blocks)]

fn main() {
    let result: std::io::Result<()> = try {
        // TODO figure out how to automatically run "cargo fmt" on the output files
        // codegen::build_scripts::rga3d::script()?;
        // codegen::build_scripts::cga3d::script()?;
    };
    result.expect("Must build successfully");
}
