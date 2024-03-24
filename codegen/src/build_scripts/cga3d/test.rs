

use crate::build_scripts::cga3d::{CGA3D, CGA3D_CRATE_PREFIX, script_custom};
use std::path::Path;
use crate::validate::{validate_glsl, validate_wgsl};

#[test]
fn build_without_disk_writes() {
    script_custom(false, CGA3D_CRATE_PREFIX).unwrap()
}

#[test]
fn glsl_validation() {
    let file_path = Path::new("../")
            .join(Path::new(CGA3D_CRATE_PREFIX))
            .join(Path::new("src/shaders/")
            .join(CGA3D));
    validate_glsl(CGA3D, file_path);
}

#[test]
fn wgsl_validation() {
    let file_path = Path::new("../")
            .join(Path::new(CGA3D_CRATE_PREFIX))
            .join(Path::new("src/shaders/")
            .join(CGA3D));
    validate_wgsl(CGA3D, file_path);
}

