#![allow(non_upper_case_globals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(effects)]

use codegen::algebra2::multivector::DeclareMultiVecs;
use codegen::elements::e1234;

codegen::multi_vecs! { e1234;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e1234;
    Origin     as e4;
    Horizon    as e321;
    DualNum    as scalar, e1234;

    // Uniform Grade Flat Objects
    Point      as e1, e2, e3, e4;
    Line       as e41, e42, e43 | e23, e31, e12;
    Plane      as e423, e431, e412, e321;

    // Versors
    Motor      as e41, e42, e43, e1234 | e23, e31, e12, scalar;
    Flector    as e1, e2, e3, e4 | e423, e431, e412, e321;
}

fn main() {
    let rga3d = codegen::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = base_documentation(register_multi_vecs(rga3d)).finished();
    let traits = codegen::register_all! { repo;
        Zero One AntiOne Unit
        Into TryInto
        Grade AntiGrade
        RightDual RightAntiDual
        Reverse AntiReverse
        Wedge AntiWedge
        GeometricProduct GeometricAntiProduct
        Sandwich AntiSandwich
        Fix AntiFix
        ConstraintViolation AntiConstraintViolation
        ConstraintValid AntiConstraintValid
        Inverse AntiInverse
        SquareRoot AntiSquareRoot
        GeometricQuotient GeometricAntiQuotient
    };
    codegen::operators! { repo, traits;
        fancy_infix => Div;

        binary
        Add => Addition,
        Sub => Subtraction,
        BitXor => Wedge,
        Mul => GeometricProduct;

        unary
        Neg => Negation,
        Not => RightDual;
    }
    let traits = traits.finish();

    let mut wgsl = codegen::Wgsl::new();
    wgsl.write_shader_file(
        "libraries/rga3d/src/",
        "rga3d",
        1,
        0,
        0,
        "",
        "Latest generation test case",
        "https://github.com/AndrewBrownK/projective_ga/",
        &[],
        repo.clone(),
        traits.clone(),
    );

    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.write_crate(
        "libraries/rga3d/",
        "rga3d",
        1,
        0,
        0,
        "",
        "Latest generated test case",
        "https://github.com/AndrewBrownK/projective_ga/",
        &[],
        repo,
        traits,
    );
}

fn base_documentation(mut declarations: DeclareMultiVecs<e1234>) -> DeclareMultiVecs<e1234> {
    declarations.append_documentation(
        &Origin,
        "\
    The Origin is the RoundPoint where x, y, z, and radius are all zero.
    It is the base element e4.
    Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
    ",
    );
    // TODO more documentation
    // declarations.generate_missing_duals(None);
    declarations
}

pub mod custom_traits {
    //
}
