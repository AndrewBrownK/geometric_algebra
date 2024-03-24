

use crate::build_scripts::rga3d::{RGA3D, RGA3D_CRATE_PREFIX, script_custom};
use std::path::Path;
use crate::validate::{validate_glsl, validate_wgsl};

#[test]
fn build_without_disk_writes() {
    script_custom(false, RGA3D_CRATE_PREFIX).unwrap()
}

#[test]
fn glsl_validation() {
    let file_path = Path::new("../")
        .join(Path::new(RGA3D_CRATE_PREFIX))
        .join(Path::new("src/shaders/")
        .join(RGA3D));
    validate_glsl(RGA3D, file_path);
}

#[test]
fn wgsl_validation() {
    let file_path = Path::new("../")
        .join(Path::new(RGA3D_CRATE_PREFIX))
        .join(Path::new("src/shaders/")
        .join(RGA3D));
    validate_wgsl(RGA3D, file_path);
}
