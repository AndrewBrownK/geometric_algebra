use crate::{ga, multi_vec};


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

    // TODO get TraitImplRegistry, TraitDefRegistry
    // TODO MultiVectorRegistry?


    use crate::algebra2::basis::elements::*;

    let flat_point = multi_vec!(D=5; FlatPoint => [e15, e25, e35, e45]);
    let line = multi_vec!(D=5; Line => [e415, e425, e435], [e235, e315, e125]);
    let plane = multi_vec!(D=5; Plane => [e4235, e4315, e4125, e3215]);

    let rpoint = multi_vec!(D=5; RoundPoint => [e1, e2, e3, e4], [e5]);
    let dipole = multi_vec!(D=5; Dipole => [e41, e42, e43], [e23, e31, e12], [e15, e25, e35, e45]);
    let circle = multi_vec!(D=5; Circle => [e423, e431, e412, e321], [e415, e425, e435], [e235, e315, e125]);
    let sphere = multi_vec!(D=5; Sphere => [e1234], [e4235, e4315, e4125, e3215]);

    let multi_vector = cga3d.full_multi_vector::<5>();
    println!("{multi_vector}");

    cga3d.internalize_names(flat_point);
    cga3d.internalize_names(line);
    cga3d.internalize_names(plane);
    cga3d.internalize_names(rpoint);
    cga3d.internalize_names(dipole);
    cga3d.internalize_names(circle);
    cga3d.internalize_names(sphere);

    let multi_vector = cga3d.full_multi_vector::<5>();
    println!("{multi_vector}");
}