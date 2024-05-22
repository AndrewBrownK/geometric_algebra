fn main() -> anyhow::Result<()> {
    let app_wgsl = "../hello_circle_wgsl/src/shader.wgsl";
    println!("cargo:rerun-if-changed={}", app_wgsl);
    cga3d_min::shaders::wgsl_compose_validate_and_spirv(
        app_wgsl,
        vec![
            ("res/shader.vs.spv", "vs_main", naga::ShaderStage::Vertex),
            ("res/shader.fs.spv", "fs_main", naga::ShaderStage::Fragment),
        ]
    )?;
    Ok(())
}
