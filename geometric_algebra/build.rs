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
        AntiScalar:e0123;\
        Magnitude:1,e0123;\
        \
        Point:e0,e1,e2,e3;\
        Origin:e3;\
        PointAtInfinity:e0,e1,e2;\
        \
        Line:e30,e31,e32|e12,e20,e01;\
        LineAtOrigin:e30,e31,e32;\
        LineAtInfinity:e12,e20,e01;\
        \
        Plane:e312,e320,e301,e210;\
        PlaneAtOrigin:e312,e320,e301;\
        PlaneAtInfinity:e210;\
        \
        Motor:e30,e31,e32,e0123|e12,e20,e01;\
        Rotor:e30,e31,e32,e0123;\
        Translator:e12,e20,e01,e0123;\
        \
        Flector:e0,e1,e2,e3|e312,e320,e301,e210;\
        \
        MultiVector:\
            1,e0123|\
            e0,e1,e2,e3|\
            e30,e31,e32|e12,e20,e01|\
            e312,e320,e301,e210\
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
    code_gen.attitude_and_dependencies("Origin", &registry, &mut emitter);
}