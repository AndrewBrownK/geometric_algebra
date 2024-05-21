use std::fs;
use naga::back::spv::{Options, PipelineOptions, WriterFlags, ZeroInitializeWorkgroupMemoryMode};
use naga::ShaderStage;
use naga::valid::{Capabilities, ValidationFlags, Validator};
use naga_oil::compose::NagaModuleDescriptor;

fn main() {
    // TODO don't unwrap() stuff in build.rs, that is obnoxious


    let wgsl_entry_path = "../hello_circle_wgsl/src/shader.wgsl";
    println!("cargo:rerun-if-changed={}", wgsl_entry_path);

    eprintln!("About to read shader.wgsl");
    let wgsl_entry = fs::read_to_string(wgsl_entry_path).unwrap();
    eprintln!("Just read shader.wgsl");

    let naga_module_descriptor = NagaModuleDescriptor {
        source: wgsl_entry.as_str(),
        file_path: wgsl_entry_path,
        ..Default::default()
    };

    let naga_module = cga3d_min::shaders::wgsl_compose_with_entrypoints(naga_module_descriptor).unwrap();
    let validator_flags = ValidationFlags::default();
    let capabilities = Capabilities::default();
    let mut validator = Validator::new(validator_flags, capabilities);
    let module_info = validator.validate(&naga_module).unwrap();

    // TODO make spirv output a function built into the shader support files per GA
    //  ...but accept most of these options as parameters?

    let options = Options {
        lang_version: (1, 6),
        flags: WriterFlags::empty(),
        binding_map: Default::default(),
        capabilities: None,
        bounds_check_policies: Default::default(),
        zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryMode::Native,
        debug_info: None,
    };
    let pipeline_options = PipelineOptions {
        shader_stage: ShaderStage::Vertex,
        entry_point: "vs_main".to_string(),
    };

    let spv = naga::back::spv::write_vec(
        &naga_module, &module_info,
        &options, Some(&pipeline_options)
    ).unwrap();

    eprintln!("About to write shader.vs.spv");
    fs::write("res/shader.vs.spv", bytemuck::cast_slice(spv.as_slice())).unwrap();
    eprintln!("Just wrote shader.vs.spv");

    let pipeline_options = PipelineOptions {
        shader_stage: ShaderStage::Fragment,
        entry_point: "fs_main".to_string(),
    };
    let spv = naga::back::spv::write_vec(
        &naga_module, &module_info,
        &options, Some(&pipeline_options)
    ).unwrap();

    eprintln!("About to write shader.fs.spv");
    fs::write("res/shader.fs.spv", bytemuck::cast_slice(spv.as_slice())).unwrap();
    eprintln!("Just wrote shader.fs.spv");
}