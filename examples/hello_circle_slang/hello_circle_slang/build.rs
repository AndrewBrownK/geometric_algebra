fn main() -> anyhow::Result<()> {
    // Depending on the algebra, compiling shaders can be extremely slow.
    // If your algebra is fast to compile, you might like to let build.rs handle it for you.
    // Otherwise, you might prefer to only trigger shader compilation manually.
    //
    // Putting the shader compilation into an adjacent crate with both lib.rs and main.rs
    // allows us to use either approach at will.
    //
    // In this project (since cga3d is slow to compile) we prefer manually building this example.
    // However, this is how it can work in this build.rs:

    // println!("cargo:rerun-if-changed={}", hello_circle_wgsl_build::APP_WGSL);
    // hello_circle_wgsl_build::compile_shaders()?;

    Ok(())
}