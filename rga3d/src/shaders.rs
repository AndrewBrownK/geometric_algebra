//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

/// Include the full wgsl source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
#[cfg(feature = "include-wgsl-src")]
pub const RGA3D_WGSL_SRC: &str = include_str!("shaders/rga3d.wgsl");

/// Include the full glsl source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
#[cfg(feature = "include-glsl-src")]
pub const RGA3D_GLSL_SRC: &str = include_str!("shaders/rga3d.glsl");

#[cfg(feature = "include-wgsl-src")]
pub fn wgsl_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {
    naga_oil::compose::ComposableModuleDescriptor {
        source: RGA3D_WGSL_SRC,
        file_path: "rga3d/src/shaders/rga3d.wgsl",
        language: naga_oil::compose::ShaderLanguage::Wgsl,
        ..Default::default()
    }
}

#[cfg(feature = "include-wgsl-src")]
pub fn wgsl_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {
    let mut composer = naga_oil::compose::Composer::default();
    // Long story for turning off validation:
    // - Composer.add_composable_module has a validation branch
    // - That validation branch will use naga to turn the Naga Internal Representation of the header
    //   into a string. The problem is that whenever naga does this, it renames things willy-nilly
    //   like it is totally harmless. The biggest offenses are in proc::namer.sanitize, where it
    //   adds underscores to names that end in digits and gets rid of double underscores in
    //   identifiers (which are actually EXTREMELY NECESSARY for readability in our case).
    // - So even though the Naga IR has correct identifiers, the wgsl string output of this IR
    //   does not. And so validation fails, because the identifiers are not the same before and
    //   after naga has mangled them
    // - So if we skip validation, the Naga IR stays correct, and we just hope everything is okay.
    // - Then if you really want to do validation, you can do it after composition instead of
    //   during composition.
    // TODO verify the post-composition validation actually does work.
    // composer.validate = false;
    composer.add_composable_module(wgsl_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;
    let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    for ep in naga_module.entry_points.iter() {
        pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    }
    naga_module = pruner.rewrite();
    return Ok(naga_module);
}

#[cfg(feature = "include-glsl-src")]
pub fn glsl_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {
    naga_oil::compose::ComposableModuleDescriptor {
        source: RGA3D_GLSL_SRC,
        file_path: "rga3d/src/shaders/rga3d.glsl",
        language: naga_oil::compose::ShaderLanguage::Glsl,
        ..Default::default()
    }
}

#[cfg(feature = "include-glsl-src")]
pub fn glsl_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {
    let mut composer = naga_oil::compose::Composer::default();
    composer.add_composable_module(glsl_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;
    let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    for ep in naga_module.entry_points.iter() {
        pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    }
    naga_module = pruner.rewrite();
    return Ok(naga_module);
}

// #[cfg(feature = "glsl-out")]
// #[cfg(feature = "spv-out")]
// #[cfg(feature = "wgsl-out")]
