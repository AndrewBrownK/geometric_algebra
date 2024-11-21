use std::fs;

pub fn main() -> anyhow::Result<()> {
    hello_circle_wgsl_build::compile_shaders_in("examples/hello_circle_wgsl/hello_circle_wgsl")
}