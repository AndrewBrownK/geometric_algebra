use crate::{ga, multi_vec, multi_vecs};

// TODO ray tracing in non-euclidean geometry
//  https://websim.ai/c/UCoHPM1U5McVpoLNq

use crate::algebra2::basis::elements::*;
multi_vecs!(e12345;
    Scalar      as scalar;
    AntiScalar  as e12345;
    DualNum     as scalar, e12345;

    FlatPoint   as e15, e25, e35, e45;
    Line        as e415, e425, e435 | e235, e315, e125;
    Plane       as e4235, e4315, e4125, e3215;

    RoundPoint  as e1, e2, e3, | e4, e5;
    Dipole      as e41, e42, e43 | e23, e31, e12 | e15, e25, e35, e45;
    Circle      as e423, e431, e412, e321 | e415, e425, e435 | e235, e315, e125;
    Sphere      as e4235, e4315, e4125 | e1234, e3215;

    Motor       as e415, e425, e435, e12345 | e235, e315, e125;
    Flector     as e15, e25, e35, e45 | e4235, e4315, e4125, e3215;

    MultiVector as
        scalar, e12345 |
        e1,e2,e3 | e4, e5 |
        e41, e42, e43 | e23, e31, e12 | e15, e25, e35, e45 |
        e423, e431, e412, e321 | e415, e425, e435 | e235, e315, e125 |
        e4235, e4315, e4125 | e1234, e3215
);


/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
#[test]
pub fn cga3d_script() {

    let cga3d = ga!(
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    );

    // TODO I can see it already... instead of manually passing in MultiVecs to register
    //  them, the macro will generate a function that does the registration for you and
    //  simply provides the free MultiVecRegistry to you for invoking it.
    //  Hell... I wonder if I can't computer a const generic BasisElement anti_scalar on
    //  GeometricAlgebra...... it could be useful to specify an anti_scalar on the MultiVec
    //  generating macro, so it can do compile time validation of the const MultiVec
    //  declarations, but then it could output a MultiVecRegistry<anti_scalar> as long as you
    //  provide the generated function a GeometricAlgebra<anti_scalar> that matches.
    let mvs = ;





    // TODO get TraitImplRegistry, TraitDefRegistry
    // TODO MultiVectorRegistry?


}