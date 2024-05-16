use std::collections::BTreeMap;
use std::io::Write;

use crate::algebra::dialect::Dialect;
use crate::algebra::GeometricAlgebraTrait;
use crate::build_scripts::cga::cga_script;

const CGA3D: &str = "cga3d";
const CGA3D_CRATE_PREFIX: &str = "cga3d/";

pub fn script() -> std::io::Result<()> {
    script_custom(true, CGA3D_CRATE_PREFIX)
}

//noinspection DuplicatedCode
fn script_custom(actually_emit: bool, path_prefix: &str) -> std::io::Result<()> {

    //  Remember this heuristic, substituting dual objects as applicable:
    //  -
    //  "A dipole with imaginary radius behaves exactly like a circle with a real radius.
    //   It can truly be treated like a circle, but is only called an AntiCircle (of real
    //   radius) because of the grade mismatch. The grade mismatch is the only reason for
    //   invoking AntiSpace. Otherwise, it is just a Circle."

    let mv_iter = [
        "Scalar:1",
        "AntiScalar:e12345",
        "DualNum:1,e12345",

        // Grade 1 is Certified
        "RoundPoint:e1,e2,e3|e4,e5",
        "Origin:e4",
        "Infinity:e5", // Infinity = AntiHorizon = AntiPlaneAtInfinity
        "RoundPointAtOrigin:e4,e5",
        "AntiSphereOnOrigin:e1,e2,e3,e4",
        "AntiPlane:e1,e2,e3,e5", // AntiPlane = AntiSphereOnInfinity
        "AntiPlaneAtOrigin:e1,e2,e3",

        // Grade 2 is Certified
        "Dipole:e41,e42,e43|e23,e31,e12|e15,e25,e35,e45",
        "DipoleAtOrigin:e41,e42,e43|e15,e25,e35",
        "DipoleOnOrigin:e41,e42,e43,e45",
        "DipoleAligningOrigin:e41,e42,e43|e15,e25,e35,e45",
        "DipoleOrthogonalOrigin:e41,e42,e43|e23,e31,e12|e15,e25,e35",
        "DipoleAtInfinity:e23,e31,e12|e15,e25,e35,e45",
        "NullDipoleAtOrigin:e41,e42,e43",
        "AntiCircleOnOrigin:e41,e42,e43|e23,e31,e12",
        "AntiLineAtOrigin:e23,e31,e12",
        "FlatPoint:e15,e25,e35,e45",
        "FlatPointAtOrigin:e45",
        "FlatPointAtInfinity:e15,e25,e35",

        // Grade 3 is Certified
        "Circle:e423,e431,e412|e415,e425,e435|e235,e315,e125,e321",
        "CircleAtOrigin:e423,e431,e412|e235,e315,e125",
        "CircleOnOrigin:e423,e431,e412|e415,e425,e435",
        "CircleAligningOrigin:e423,e431,e412|e415,e425,e435|e235,e315,e125",
        "CircleOrthogonalOrigin:e423,e431,e412|e235,e315,e125,e321",
        "CircleAtInfinity:e415,e425,e435|e235,e315,e125,e321",
        "NullCircleAtOrigin:e423,e431,e412",
        "AntiDipoleOnOrigin:e423,e431,e412,e321",
        "AntiFlatPointAtOrigin:e321",
        "Line:e415,e425,e435|e235,e315,e125",
        "LineAtOrigin:e415,e425,e435",
        "LineAtInfinity:e235,e315,e125",

        // Grade 4 is Certified
        "Sphere:e4235,e4315,e4125|e1234,e3215",
        "NullSphereAtOrigin:e1234",
        "Horizon:e3215",
        "SphereAtOrigin:e1234,e3215",
        "SphereOnOrigin:e4235,e4315,e4125,e1234",
        "Plane:e4235,e4315,e4125,e3215",
        "PlaneAtOrigin:e4235,e4315,e4125",

        // Operator Objects
        "Motor:e415,e425,e435,e12345|e235,e315,e125",
        "Rotor:e415,e425,e435,e12345",
        "Translator:e235,e315,e125,e12345",
        "Flector:e15,e25,e35,e45|e4235,e4315,e4125,e3215",
        "Transflector:e15,e25,e35|e4235,e4315,e4125,e3215",
        "FlectorAtInfinity:e15,e25,e35,e3215",

        "MultiVector:\
            1,e12345|\
            e1,e2,e3|e4,e5|\
            e41,e42,e43,e45|e23,e31,e12|e15,e25,e35|\
            e423,e431,e412|e415,e425,e435|e235,e315,e125,e321|\
            e4235,e4315,e4125|e1234,e3215",
    ];

    // TODO add more of these if/where applicable to CGA
    let sandwich_outputs: BTreeMap<(&str, &str), &str> = [

        ("Translator", "Origin", "Point"),
        ("Translator", "LineAtOrigin", "Line"),
        ("Translator", "PlaneAtOrigin", "Plane"),
        ("Translator", "Rotor", "Motor"),

        ("Motor", "Origin", "Point"),
        ("Motor", "LineAtOrigin", "Line"),
        ("Motor", "PlaneAtOrigin", "Plane"),
        ("Motor", "Rotor", "Motor"),

        ("Flector", "Origin", "Point"),
        ("Flector", "LineAtOrigin", "Line"),
        ("Flector", "PlaneAtOrigin", "Plane"),
        ("Flector", "Rotor", "Motor"),

        ("FlectorAtInfinity", "Origin", "Point"),
        ("FlectorAtInfinity", "LineAtOrigin", "Line"),
        ("FlectorAtInfinity", "PlaneAtOrigin", "Plane"),
        ("FlectorAtInfinity", "Rotor", "Motor"),

    ].into_iter().map(|it| ((it.0, it.1), it.2)).collect();

    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().also_meet_and_join();

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
    use crate::build_scripts::cga3d::{CGA3D, CGA3D_CRATE_PREFIX, script_custom};
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