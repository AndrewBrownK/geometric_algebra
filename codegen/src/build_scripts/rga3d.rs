use std::collections::BTreeMap;

use crate::algebra::dialect::Dialect;
use crate::build_scripts::rga::rga_script;

const RGA3D: &str = "rga3d";
const RGA3D_CRATE_PREFIX: &str = "rga3d/";

pub fn script() -> std::io::Result<()> {
    script_custom(true, RGA3D_CRATE_PREFIX)
}

// noinspection DuplicatedCode
fn script_custom(actually_emit: bool, path_prefix: &str) -> std::io::Result<()> {
    let mv_iter = [
        "Scalar:1",
        "AntiScalar:e1234",
        "DualNum:1,e1234",
        "Point:e1,e2,e3,e4",
        "Origin:e4",
        "PointAtInfinity:e1,e2,e3",
        "Line:e41,e42,e43|e23,e31,e12",
        "LineAtOrigin:e41,e42,e43",
        "LineAtInfinity:e23,e31,e12",
        "Plane:e423,e431,e412,e321",
        "PlaneAtOrigin:e423,e431,e412",
        "Horizon:e321",
        "Motor:e41,e42,e43,e1234|e23,e31,e12",
        "Rotor:e41,e42,e43,e1234",
        "Translator:e23,e31,e12,e1234",
        "Flector:e1,e2,e3,e4|e423,e431,e412,e321",
        "Transflector:e1,e2,e3|e423,e431,e412,e321",
        "FlectorAtInfinity:e1,e2,e3,e321",
        "MultiVector:\
            1,e1234|\
            e1,e2,e3,e4|\
            e41,e42,e43|e23,e31,e12|\
            e423,e431,e412,e321",
        "MultiVectorAtOrigin:e4,e1234|e41,e42,e43|e423,e431,e412",
        "MultiVectorAtInfinity:1,e321|e1,e2,e3|e23,e31,e12",
    ];

    // On sandwich products, assume that the output
    // class is the same as the input class, unless it is
    // in the following guide
    // TODO add more of these if necessary
    let sandwich_outputs: BTreeMap<(&str, &str), &str> = [
        // Rotations of objects at origin are still objects at origin
        // But as you can see, the inputs and outputs are the same,
        // so we do not need to add special guidance here. The input
        // and output being the same is the default assumption.

        // ("Rotor", "Origin", "Origin"),
        // ("Rotor", "LineAtOrigin", "LineAtOrigin"),
        // ("Rotor", "PlaneAtOrigin", "PlaneAtOrigin"),

        // In contrast to rotations, translations of objects at origin
        // are not objects at origin. Therefore, we must add the special
        // guidance on output types here.
        ("Translator", "Origin", "Point"),
        ("Translator", "LineAtOrigin", "Line"),
        ("Translator", "PlaneAtOrigin", "Plane"),
        ("Translator", "Rotor", "Motor"),
        // And obviously motor outputs must be at least as general as translator outputs
        ("Motor", "Origin", "Point"),
        ("Motor", "LineAtOrigin", "Line"),
        ("Motor", "PlaneAtOrigin", "Plane"),
        ("Motor", "Rotor", "Motor"),
        // Flectors move stuff too
        ("Flector", "Origin", "Point"),
        ("Flector", "LineAtOrigin", "Line"),
        ("Flector", "PlaneAtOrigin", "Plane"),
        ("Flector", "Rotor", "Motor"),
        ("FlectorAtInfinity", "Origin", "Point"),
        ("FlectorAtInfinity", "LineAtOrigin", "Line"),
        ("FlectorAtInfinity", "PlaneAtOrigin", "Plane"),
        ("FlectorAtInfinity", "Rotor", "Motor"),
    ]
    .into_iter()
    .map(|it| ((it.0, it.1), it.2))
    .collect();

    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().also_meet_and_join();

    rga_script(path_prefix, RGA3D, dialect, 3, actually_emit, &mv_iter, sandwich_outputs)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::build_scripts::rga3d::{RGA3D, RGA3D_CRATE_PREFIX};
    use crate::validate::{validate_glsl, validate_wgsl};
    use std::path::Path;

    #[test]
    fn build_without_disk_writes() {
        // script_custom(false, RGA3D_CRATE_PREFIX).unwrap()
    }

    #[test]
    fn glsl_validation() {
        let file_path = Path::new("../").join(Path::new(RGA3D_CRATE_PREFIX)).join(Path::new("src/shaders/").join(RGA3D));
        validate_glsl(RGA3D, file_path);
    }

    #[test]
    fn wgsl_validation() {
        let file_path = Path::new("../").join(Path::new(RGA3D_CRATE_PREFIX)).join(Path::new("src/shaders/").join(RGA3D));
        validate_wgsl(RGA3D, file_path);
    }
}
