use std::path::{Path, PathBuf};
use crate::emit::Emitter;

pub fn emit_shader_support(
    emitter: &mut Emitter<std::fs::File>,
    base_file_path: &PathBuf,
    algebra_name: &str
) -> std::io::Result<()> {

    let upper_snake_case_name = algebra_name.to_uppercase();


    emitter.new_rust_collector(&base_file_path.join(Path::new("shaders.rs")));
    let rust_source = format!("

/// Include the full wgsl source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
#[cfg(feature = \"include-wgsl-src\")]
pub const {upper_snake_case_name}_WGSL_SRC: &str = include_str!(\"shaders/{algebra_name}.wgsl\");

/// Include the full glsl source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
#[cfg(feature = \"include-glsl-src\")]
pub const {upper_snake_case_name}_GLSL_SRC: &str = include_str!(\"shaders/{algebra_name}.glsl\");

#[cfg(feature = \"include-wgsl-src\")]
pub fn wgsl_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {{
    naga_oil::compose::ComposableModuleDescriptor {{
        source: {upper_snake_case_name}_WGSL_SRC,
        file_path: \"{algebra_name}/src/shaders/{algebra_name}.wgsl\",
        ..Default::default()
    }}
}}

#[cfg(feature = \"include-wgsl-src\")]
pub fn wgsl_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {{
    let mut composer =  naga_oil::compose::Composer::default();
    composer.add_composable_module(wgsl_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;
    let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    for ep in naga_module.entry_points.iter() {{
        pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    }}
    naga_module = pruner.rewrite();
    return Ok(naga_module);
}}

#[cfg(feature = \"include-glsl-src\")]
pub fn glsl_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {{
    naga_oil::compose::ComposableModuleDescriptor {{
        source: {upper_snake_case_name}_GLSL_SRC,
        file_path: \"{algebra_name}/src/shaders/{algebra_name}.glsl\",
        ..Default::default()
    }}
}}

#[cfg(feature = \"include-glsl-src\")]
pub fn glsl_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {{
    let mut composer =  naga_oil::compose::Composer::default();
    composer.add_composable_module(glsl_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;
    let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    for ep in naga_module.entry_points.iter() {{
        pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    }}
    naga_module = pruner.rewrite();
    return Ok(naga_module);
}}


// #[cfg(feature = \"glsl-out\")]
// #[cfg(feature = \"spv-out\")]
// #[cfg(feature = \"wgsl-out\")]


");
    emitter.emit_rust_preamble(rust_source.as_str())?;

    Ok(())
}


