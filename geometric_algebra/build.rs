use ::codegen;
use codegen::algebra::rigid::RigidGeometricAlgebra;
use codegen::{CodeGenerator, read_multi_vector_from_str};
use codegen::algebra::dialect::Dialect;
use codegen::algebra::MultiVectorClassRegistry;
use codegen::emit::Emitter;

fn main() {

    let mv_iter =
        "\
        Scalar:1;\
        AntiScalar:e1234;\
        Magnitude:1,e1234;\
        \
        Point:e1,e2,e3,e4;\
        Origin:e4;\
        PointAtInfinity:e1,e2,e3;\
        \
        Line:e41,e42,e43|e23,e31,e12;\
        LineAtOrigin:e41,e42,e43;\
        LineAtInfinity:e23,e31,e12;\
        \
        Plane:e423,e431,e412,e321;\
        PlaneAtOrigin:e423,e431,e412;\
        Horizon:e321;\
        \
        Motor:e41,e42,e43,e1234|e23,e31,e12;\
        Rotor:e41,e42,e43,e1234;\
        Translator:e23,e31,e12,e1234;\
        \
        Flector:e1,e2,e3,e4|e423,e431,e412,e321;\
        \
        MultiVector:\
            1,e1234|\
            e1,e2,e3,e4|\
            e41,e42,e43|e23,e31,e12|\
            e423,e431,e412,e321\
        ".split(';');


    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().contractions().also_meet_and_join();


    let rga3d = RigidGeometricAlgebra {
        generator_squares: &[1, 1, 1, 0],
        name: "rga3d",
        dialect,
    };

    let mut registry = MultiVectorClassRegistry::default();
    for multi_vector_descriptor in mv_iter {
        let mv = read_multi_vector_from_str(multi_vector_descriptor, &rga3d);
        registry.register(mv);
    }
    let file_path = std::path::Path::new("./src/").join(std::path::Path::new(rga3d.name));
    let mut code_gen = CodeGenerator::new(rga3d);
    let mut emitter = Emitter::new(&file_path);
    // code_gen.lifetime_debug(&registry, &mut emitter);
    // code_gen.lifetime_debug2(&registry, &mut emitter);
    code_gen.preamble_and_universal_traits(&registry, &mut emitter).unwrap();
    code_gen.basic_norms(&registry, &mut emitter);
    // TODO fancy norms
    // code_gen.fancy_norms()
    code_gen.post_norm_universal_stuff(&registry, &mut emitter);
    code_gen.attitude_and_dependencies("Horizon", &registry, &mut emitter);
}