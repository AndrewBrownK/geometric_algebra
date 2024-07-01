use crate::{ga, multi_vec, multi_vecs};
use crate::algebra2::basis::generators::GeneratorElement::e3;

/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
#[test]
pub fn cga3d_script() {

    let cga3d = ga!(
        1 => e1, e2, e3, e_plus;
        -1 => e_minus;
        where
        e4 => 0.5 * (e_minus - e_plus);
        e5 => e_plus + e_minus;
    );

    let mvs = multi_vecs!(D=5;
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





    // TODO get TraitImplRegistry, TraitDefRegistry
    // TODO MultiVectorRegistry?



    for thing in mvs.iter() {
        println!("{thing}");
    }

    let multi_vector = cga3d.full_multi_vector::<5>();
    println!("{multi_vector}");

    for thing in mvs.into_iter() {
        cga3d.internalize_names(thing)
    }

    let multi_vector = cga3d.full_multi_vector::<5>();
    println!("{multi_vector}");
}