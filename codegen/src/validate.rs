use naga::front::glsl::Error;
use naga::valid::{Capabilities, ValidationFlags};
use naga::ShaderStage;
use std::io::Read;
use std::path::PathBuf;

// TODO put back to use, or clean up
#[allow(unused)]
pub fn validate_glsl(algebra_name: &str, file_path: PathBuf) {
    // Prepare some of naga's clutter
    let mut glsl_frontend = naga::front::glsl::Frontend::default();
    let mut validator = naga::valid::Validator::new(ValidationFlags::default(), Capabilities::default());
    let options = naga::front::glsl::Options {
        stage: ShaderStage::Compute,
        defines: Default::default(),
    };

    // Read the glsl
    let glsl_file_name = file_path.with_extension("glsl");
    let mut glsl_file = std::fs::File::open(glsl_file_name).unwrap();
    let mut glsl_contents = String::new();
    glsl_file.read_to_string(&mut glsl_contents).unwrap();

    // Trim the naga_oil directive off the front
    let naga_oil_directive = format!("#version 450\n#define_import_path {algebra_name}\n\n");
    glsl_contents.replace_range(0..naga_oil_directive.len(), "");

    // Parse and validate the naga module
    let module = match glsl_frontend.parse(&options, glsl_contents.as_str()) {
        Ok(m) => m,
        Err(err) => {
            let mut line = "??".to_string();
            let mut line_position = "??".to_string();
            if let Some(Error { meta, .. }) = err.first() {
                let location = meta.location(glsl_contents.as_str());
                line = location.line_number.to_string();
                line_position = location.line_position.to_string();
            }
            panic!("Error generating {algebra_name} glsl on line {line} at line position {line_position}: {err:?}")
        }
    };
    if let Err(err) = validator.validate(&module) {
        panic!("Error generating {algebra_name}: {err:?}")
    };
    // glsl success, woo hoo!
}

// TODO put back to use, or clean up
#[allow(unused)]
pub fn validate_wgsl(algebra_name: &str, file_path: PathBuf) {
    // Prepare some of naga's clutter
    let mut wgsl_frontend = naga::front::wgsl::Frontend::new();
    let mut validator = naga::valid::Validator::new(ValidationFlags::default(), Capabilities::default());

    // Read the wgsl
    let wgsl_file_name = file_path.with_extension("wgsl");
    let mut wgsl_file = std::fs::File::open(wgsl_file_name).unwrap();
    let mut wgsl_contents = String::new();
    wgsl_file.read_to_string(&mut wgsl_contents).unwrap();
    // Trim the naga_oil directive off the front
    let naga_oil_directive = format!("#define_import_path {algebra_name}\n\n");
    wgsl_contents.replace_range(0..naga_oil_directive.len(), "");

    // Parse and validate the naga module
    let module = match wgsl_frontend.parse(wgsl_contents.as_str()) {
        Ok(m) => m,
        Err(err) => {
            let mut line = "??".to_string();
            let mut line_position = "??".to_string();
            if let Some(loc) = err.location(wgsl_contents.as_str()) {
                line = loc.line_number.to_string();
                line_position = loc.line_position.to_string();
            }
            panic!("Error generating {algebra_name} wgsl on line {line} at line position {line_position}: {err:?}")
        }
    };
    if let Err(err) = validator.validate(&module) {
        panic!("Error generating {algebra_name}: {err:?}")
    };
    // wgsl success, woo hoo!
}
