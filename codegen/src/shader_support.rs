use std::io::Write;

pub fn emit_shader_support<W: Write>(w: &mut W, algebra_name: &str, shader_extension: &str) -> anyhow::Result<()> {
    let upper_extension = shader_extension.to_uppercase();
    let lower_extension = shader_extension.to_lowercase();
    let camel_extension = {
        let mut c= lower_extension.chars();
        c.next().expect("Provide an extension").to_uppercase().collect::<String>() + c.as_str()
    };

    /*
    TODO output the spv files in "OUT_DIR" if that makes more sense
     */

    let upper_snake_case_name = algebra_name.to_uppercase();
    let mut file = w;
    write!(&mut file, r#"
use std::fs;
use std::path::Path;

use naga::back::spv::{{Options, PipelineOptions, WriterFlags, ZeroInitializeWorkgroupMemoryMode}};
use naga::ShaderStage;
use naga::valid::{{Capabilities, ValidationFlags, Validator}};

/// Include the full {lower_extension} source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
pub const {upper_snake_case_name}_{upper_extension}_SRC: &str = include_str!("{algebra_name}.{lower_extension}");

pub fn {lower_extension}_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {{
    naga_oil::compose::ComposableModuleDescriptor {{
        source: {upper_snake_case_name}_{upper_extension}_SRC,
        file_path: "{algebra_name}/src/integrations/{algebra_name}.{lower_extension}",
        language: naga_oil::compose::ShaderLanguage::{camel_extension},
        ..Default::default()
    }}
}}

pub fn {lower_extension}_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {{
    let mut composer =  naga_oil::compose::Composer::default();
    composer.add_composable_module({lower_extension}_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;
    let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    for ep in naga_module.entry_points.iter() {{
        pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    }}
    naga_module = pruner.rewrite();
    return Ok(naga_module);
}}

/// Compose {lower_extension}, validate the module, and output a SPIR-V file for each entry point.
/// Half for utility, half for example, a pattern like this is useful in build.rs.
/// If you'd like to customize any of the options, you can copy and/or inline this function.
// TODO it is kind of a presumptuous/risky move to choose default options, even though the
//  succinctness can be nice. So it's probably best to atomize this function into
//  smaller itty-bitty parts that only add in assumptions layer by layer. Maybe the factory/builder
//  pattern can help keep things succinct.
pub fn {lower_extension}_compose_validate_and_spirv<P: AsRef<Path>, S: Into<String>>(
    {lower_extension}_file_path: &str,
    spirv_outputs: Vec<(P, S, ShaderStage)>
) -> anyhow::Result<()> {{
    let shader_src = fs::read_to_string({lower_extension}_file_path)?;
    let naga_module_descriptor = naga_oil::compose::NagaModuleDescriptor {{
        source: shader_src.as_str(),
        file_path: {lower_extension}_file_path,
        ..Default::default()
    }};
    let naga_module = {lower_extension}_compose_with_entrypoints(naga_module_descriptor)?;
    let validator_flags = ValidationFlags::default();
    let capabilities = Capabilities::default();
    let mut validator = Validator::new(validator_flags, capabilities);
    let naga_module_info = validator.validate(&naga_module)?;
    let options = Options {{
        lang_version: (1, 6),
        flags: WriterFlags::empty(),
        binding_map: Default::default(),
        capabilities: None,
        bounds_check_policies: Default::default(),
        zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryMode::Native,
        debug_info: None,
    }};
    for (spirv_path, entry_point, shader_stage) in spirv_outputs {{
        let pipeline_options = PipelineOptions {{
            shader_stage,
            entry_point: entry_point.into(),
        }};
        let spv = naga::back::spv::write_vec(
            &naga_module, &naga_module_info,
            &options, Some(&pipeline_options)
        )?;
        fs::write(spirv_path, bytemuck::cast_slice(spv.as_slice()))?;
    }}
    Ok(())
}}
"#)?;
    Ok(())
}
