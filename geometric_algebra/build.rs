use::codegen;

fn main() {

    let algebras = [
        // TODO
        //  - The first thing you should take note of is that in "ppga3d:0,1,1,1" the projective
        //    dimensions is e0, while I am accustomed to the projective dimension being e4.
        //  - The second thing to notice then, is that this might actually be representing the
        //    duals of geometric objects, instead of the objects themselves. See exposition here:
        //    http://rigidgeometricalgebra.org/wiki/index.php?title=Duality
        //    And the formula for the dual listed here:
        //    https://conformalgeometricalgebra.org/wiki/index.php?title=Plane
        "epga1d:1,1;Scalar:1;ComplexNumber:1,e01",
        "ppga1d:0,1;Scalar:1;DualNumber:1,e01",
        "hpga1d:-1,1;Scalar:1;SplitComplexNumber:1,e01",
        "epga2d:1,1,1;Scalar:1;AntiScalar:e012;MultiVector:1,e12,e1,e2|e0,e012,e01,-e02;Rotor:1,e12;Point:e12,e01,-e02;IdealPoint:e01,-e02;Plane:e0,e2,e1;Translator:1,e01,-e02;Motor:1,e12,e01,-e02;MotorDual:e012,e0,e2,e1",
        "ppga2d:0,1,1;Scalar:1;AntiScalar:e012;MultiVector:1,e12,e1,e2|e0,e012,e01,-e02;Rotor:1,e12;Point:e12,e01,-e02;IdealPoint:e01,-e02;Plane:e0,e2,e1;Translator:1,e01,-e02;Motor:1,e12,e01,-e02;MotorDual:e012,e0,e2,e1",
        "hpga2d:-1,1,1;Scalar:1;AntiScalar:e012;MultiVector:1,e12,e1,e2|e0,e012,e01,-e02;Rotor:1,e12;Point:e12,e01,-e02;IdealPoint:e01,-e02;Plane:e0,e2,e1;Translator:1,e01,-e02;Motor:1,e12,e01,-e02;MotorDual:e012,e0,e2,e1",
        "epga3d:1,1,1,1;Scalar:1;AntiScalar:e0123;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3",
        "ppga3d:0,1,1,1;Scalar:1;AntiScalar:e0123;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3",
        "hpga3d:-1,1,1,1;Scalar:1;AntiScalar:e0123;MultiVector:1,e23,-e13,e12|e0,-e023,e013,-e012|e123,e1,e2,e3|e0123,e01,e02,e03;Rotor:1,e23,-e13,e12;Point:e123,-e023,e013,-e012;IdealPoint:e01,e02,e03;Plane:e0,e1,e2,e3;Line:e01,e02,e03|e23,-e13,e12;Translator:1,e01,e02,e03;Motor:1,e23,-e13,e12|e0123,e01,e02,e03;PointAndPlane:e123,-e023,e013,-e012|e0,e1,e2,e3",

        // TODO implementing conformal might not be straightforward, but here is an idea
        //  It might not work as raw [1,1,1,1,-1] since e+ and e- get treated weird to arrive at e4 and e5
        //  However maybe we can do [1,1,1,0,0] and hack in the proper behaviors where applicable
        //  Namely, the following traits (search "<MultiVector> for MultiVector"):
        //   - OuterProduct
        //   - SquaredMagnitude?
        //   - any projection-like products?
        //   - add some traits: Carrier, AntiCarrier, Center, Round/Flat Bulk/Weight, Container, etc

        // TODO I FIGURED OUT THE WEIRD ORDERING/GROUPING OF COMPONENTS!!!
        //  Look at epga3d MultiVectorGroups for example... and then look at the dot product table,
        //  and the implementation of InnerProduct<MultiVector> for MultiVector.
        //  I'll have to see if this is an optional optimization choice (and the algebra can
        //  still figure itself out even if you do not follow this pattern) or if this grouping
        //  pattern is really required.
        //  Okay actually on second thought... I'm not as immediately sure. The implementation
        //  of InnerProduct is more complex that I first realized. Which could be a good thing...
        //  in that it is actually doing what it is supposed to instead of making unclear
        //  assumptions about the grouping pattern. Let's see....
        "cga3d:1,1,1,0,0;\
            Scalar:1;\
            AntiScalar:e01234;\
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
                e3124,e3204,e3014,e2104"
    ];

    for algebra in algebras {
        let config = codegen::read_config_from_str(&algebra);
        codegen::generate_code(config, "./src/");
    }
}