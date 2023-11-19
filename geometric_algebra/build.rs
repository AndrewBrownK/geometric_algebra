use::codegen;

fn main() {

    let algebras = [
        // "epga1d:1,1;Scalar:1;ComplexNumber:1,e01",
        // "ppga1d:0,1;Scalar:1;DualNumber:1,e01",
        // "hpga1d:-1,1;Scalar:1;SplitComplexNumber:1,e01",
        // "epga2d:1,1,1;Scalar:1;AntiScalar:e012;MultiVector:1,e12,e1,e2|e0,e012,e01,-e02;Rotor:1,e12;Point:e12,e01,-e02;IdealPoint:e01,-e02;Plane:e0,e2,e1;Translator:1,e01,-e02;Motor:1,e12,e01,-e02;MotorDual:e012,e0,e2,e1",
        // "ppga2d:0,1,1;Scalar:1;AntiScalar:e012;MultiVector:1,e12,e1,e2|e0,e012,e01,-e02;Rotor:1,e12;Point:e12,e01,-e02;IdealPoint:e01,-e02;Plane:e0,e2,e1;Translator:1,e01,-e02;Motor:1,e12,e01,-e02;MotorDual:e012,e0,e2,e1",
        // "hpga2d:-1,1,1;Scalar:1;AntiScalar:e012;MultiVector:1,e12,e1,e2|e0,e012,e01,-e02;Rotor:1,e12;Point:e12,e01,-e02;IdealPoint:e01,-e02;Plane:e0,e2,e1;Translator:1,e01,-e02;Motor:1,e12,e01,-e02;MotorDual:e012,e0,e2,e1",
        // "epga3d:1,1,1,1;Scalar:1;AntiScalar:e0123;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3",
        // "ppga3d:0,1,1,1;Scalar:1;AntiScalar:e0123;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3",
        // "hpga3d:-1,1,1,1;Scalar:1;AntiScalar:e0123;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3",

        // TODO implementing conformal might not be straightforward, but here is an idea
        //  It might not work as raw [1,1,1,1,-1] since e+ and e- get treated weird to arrive at e4 and e5
        //  However maybe we can do [1,1,1,0,0] and hack in the proper behaviors where applicable
        "cga3d:1,1,1,0,0;\
            Scalar:1;\
            AntiScalar:e01234;\
            HomogeneousMagnitude:1,e01234;\
            RadialPoint:e0,e1,e2|e3,e4;\
            FlatPoint:e04,e14,e24,e34;\
            Dipole:e30,e31,e32|e12,e20,e01|e04,e14,e24,e34;\
            Line:e304,e314,e324|e124,e204,e014;\
            Circle:e312,e320,e301,e210|e304,e314,e324|e124,e204,e014;\
            Plane:e3124,e3204,e3014,e2104;\
            Sphere:e0123,e2104|e3124,e3204,e3014;\
            Motor:e304,e314,e324,e01234|e124,e204,e014,e4;\
            Rotor:e304,e314,e324,e01234;\
            Translator:e124,e204,e014,e01234;\
            Flector:e04,e14,e24,e34|e3124,e3204,e3014,e2104;\
            Dilation:e230,e310,e120|e321,e01234;\
            MultiVector:\
                1,e0123,e01234|\
                e0,e1,e2|e3,e4|\
                e04,e14,e24,e34|\
                e30,e31,e32|e12,e20,e01|\
                e304,e314,e324|e124,e204,e014|\
                e312,e320,e301,e210|\
                e3124,e3204,e3014,e2104",

        // "raw_cga3d:1,1,1,1,-1;\
        //     Scalar:1;\
        //     AntiScalar:e01234;\
        //     HomogeneousMagnitude:1,e01234;\
        //     RadialPoint:e0,e1,e2|e3,e4;\
        //     FlatPoint:e04,e14,e24,e34|e03,e13,e23;\
        //     Dipole:e30,e31,e32|e12,e20,e01|e04,e14,e24,e34;\
        //     Line:e304,e314,e324|e124,e204,e014;\
        //     Circle:e312,e320,e301,e210|e304,e314,e324|e124,e204,e014;\
        //     Plane:e3124,e3204,e3014,e2104;\
        //     Sphere:e0123,e2104|e3124,e3204,e3014;\
        //     Motor:e304,e314,e324,e01234|e124,e204,e014,e4;\
        //     Rotor:e304,e314,e324,e01234;\
        //     Translator:e124,e204,e014,e01234;\
        //     Flector:e04,e14,e24,e34|e3124,e3204,e3014,e2104;\
        //     Dilation:e230,e310,e120|e321,e01234;\
        //     MultiVector:\
        //         1,e0123,e01234|\
        //         e0,e1,e2|e3,e4|\
        //         e04,e14,e24,e34|\
        //         e30,e31,e32|e12,e20,e01|\
        //         e304,e314,e324|e124,e204,e014|\
        //         e312,e320,e301,e210|\
        //         e3124,e3204,e3014,e2104",

        "rga3d:1,1,1,0;\
            Scalar:1;\
            AntiScalar:e0123;\
            HomogeneousMagnitude:1,e0123;\
            Point:e0,e1,e2,e3;\
            Line:e30,e31,e32|e12,e20,e01;\
            Plane:e312,e320,e301,e210;\
            Motor:e30,e31,e32,e0123|e12,e20,e01;\
            Rotor:e30,e31,e32,e0123;\
            Translator:e12,e20,e01,e0123;\
            Flector:e0,e1,e2,e3|e312,e320,e301,e210;\
            MultiVector:\
                1,e0123,e0123|\
                e0,e1,e2,e3|\
                e30,e31,e32|e12,e20,e01|\
                e30,e31,e32|e12,e20,e01|\
                e312,e320,e301,e210|\
                e312,e320,e301,e210"
    ];

    for algebra in algebras {
        let config = codegen::read_config_from_str(&algebra);
        codegen::generate_code(config, "./src/");
    }
}