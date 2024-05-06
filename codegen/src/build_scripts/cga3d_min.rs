use std::collections::BTreeMap;
use std::io::Write;

use crate::algebra::dialect::Dialect;
use crate::algebra::GeometricAlgebraTrait;
use crate::build_scripts::cga::cga_script;

const CGA3D: &str = "cga3d_min";
const CGA3D_CRATE_PREFIX: &str = "cga3d_min/";

pub fn script() -> std::io::Result<()> {
    script_custom(true, CGA3D_CRATE_PREFIX)
}

//noinspection DuplicatedCode
fn script_custom(actually_emit: bool, path_prefix: &str) -> std::io::Result<()> {

    let mv_iter = [
        "Scalar:1",
        "AntiScalar:e12345",
        "DualNum:1,e12345",

        // Flat objects
        "FlatPoint:e15,e25,e35,e45",
        "Line:e415,e425,e435|e235,e315,e125",
        "Plane:e4235,e4315,e4125,e3215",

        // TODO since you no longer have separate Horizon and Origin definitions,
        //  you need to supplement some compilation logic to construct Planes
        //  and RoundPoints instead.

        // Round objects
        "RoundPoint:e1,e2,e3|e4,e5",
        "Dipole:e41,e42,e43|e23,e31,e12|e15,e25,e35,e45",
        "Circle:e423,e431,e412,e321|e415,e425,e435|e235,e315,e125",
        "Sphere:e4235,e4315,e4125|e1234,e3215",

        // Operator Objects
        "Motor:e415,e425,e435,e12345|e235,e315,e125",
        "Rotor:e415,e425,e435,e12345",
        "Translator:e235,e315,e125,e12345",
        "Flector:e15,e25,e35,e45|e4235,e4315,e4125,e3215",
        "Transflector:e15,e25,e35|e4235,e4315,e4125,e3215",

        "MultiVector:\
            1,e12345|\
            e1,e2,e3|e4,e5|\
            e41,e42,e43|e23,e31,e12|e15,e25,e35,e45|\
            e423,e431,e412,e321|e415,e425,e435|e235,e315,e125|\
            e4235,e4315,e4125|e1234,e3215",
    ];

    let sandwich_outputs: BTreeMap<(&str, &str), &str> = [

        ("Translator", "Origin", "Point"),
        // ("Translator", "LineAtOrigin", "Line"),
        // ("Translator", "PlaneAtOrigin", "Plane"),
        // ("Translator", "Rotor", "Motor"),

    ].into_iter().map(|it| ((it.0, it.1), it.2)).collect();

    // Minimal/succinct dialect
    let dialect = Dialect::default().wedge_dot().wedge().dot();

    cga_script(
        path_prefix,
        CGA3D,
        dialect,
        3,
        actually_emit,
        &mv_iter,
        sandwich_outputs,
    )?;


    Ok(())
}


#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::build_scripts::cga3d_min::{CGA3D, CGA3D_CRATE_PREFIX, script_custom};
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
}