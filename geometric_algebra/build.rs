#![feature(try_blocks)]


fn main() {
    let result: std::io::Result<()> = try {
        codegen::build_scripts::rga3d::script()?;
    };
    result.expect("Must build successfully");
}
