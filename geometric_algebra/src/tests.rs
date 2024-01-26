use crate::{GeometricProduct, GeometricAntiProduct, Wedge, AntiWedge, AntiReversal};


// #[test]
// fn ppga3d_geometric_product_01() {
//     let a = ppga3d::Scalar::new(5.0);
//     let b = ppga3d::Scalar::new(7.0);
//     let c = a.geometric_product(b);
//     assert_eq!(c.group0(), 35.0);
// }

// No GeometricProduct between AntiScalar and AntiScalar
// #[test]
// fn ppga3d_geometric_product_02() {
//     let a = ppga3d::AntiScalar::new(5.0);
//     let b = ppga3d::AntiScalar::new(7.0);
//     let c = a.geometric_product(b);
//     assert_eq!(c.group0(), 0.0);
// }

// #[test]
// fn ppga3d_geometric_anti_product_01() {
//     let a = ppga3d::AntiScalar::new(5.0);
//     let b = ppga3d::AntiScalar::new(7.0);
//     let c = a.geometric_anti_product(b);
//     assert_eq!(c.group0(), 35.0);
// }

// No GeometricAntiProduct between Scalar and Scalar
// #[test]
// fn ppga3d_geometric_anti_product_02() {
//     let a = ppga3d::Scalar::new(5.0);
//     let b = ppga3d::Scalar::new(7.0);
//     let c = a.geometric_anti_product(b);
//     assert_eq!(c.group0(), 35.0);
// }


#[test]
fn ppga3d_use_a_motor() {

    // Alright so it is super complex to specify geometric objects directly,
    // but it might be easier to construct and use them one little bit at a time.

    // You apply a motor using sandwich product. In our case, that is:
    // anti_gp(anti_gp(motor, object), anti_reverse(motor))

    // You can construct a motor from the following:
    // - anti_gp(plane, anti_reverse(plane))
    // - anti_gp(line, anti_reverse(line))
    // - anti_gp(point, anti_reverse(point))

    // You can construct a plane from the following:
    // - line wedge point (contains point, contains line)
    // - point wedge line* (contains point, perpendicular to line)
    // - line wedge plane* (containing line, perpendicular to plane)

    // You can construct a line from the following:
    // - point wedge point
    // - plane anti_wedge plane
    // - point wedge plane* (contains point, perpendicular to plane)

    // You can construct a point from the following:
    // - Regular constructor is probably simple enough
    // - line anti_wedge plane

    // let point_a = ppga3d::Point::new(1.0, 1.0, 1.0, 1.0);
    // let point_b = ppga3d::Point::new(-1.0, 1.0, 1.0, 1.0);
    // let ipoint_a = ppga3d::IdealPoint::new(1.0, 1.0, 1.0);
    //
    // let line_ab = point_a.wedge(point_b);
}



// #[test]
// fn cga_basics() {
//     // https://conformalgeometricalgebra.com/wiki/index.php?title=Join_and_meet
//
//     // Joins
//
//     let radial_point_a = cga3d::RadialPoint::new(0.0, -1.0, -1.0, -1.0, 0.0);
//     let radial_point_b = cga3d::RadialPoint::new(0.0, 1.0, 1.0, 1.0, 0.0);
//     let dipole_ab = radial_point_a.wedge(radial_point_b);
//
//     let flat_point_b = cga3d::FlatPoint::new(1.0, 1.0, 1.0, 1.0);
//     let line_ab = radial_point_a.wedge(flat_point_b);
//     let line_ba = flat_point_b.wedge(radial_point_a);
//
//     let radial_point_c = cga3d::RadialPoint::new(1.0, 2.0, 3.0, 4.0, 5.0);
//     let circle_abc = dipole_ab.wedge(radial_point_c);
//     let circle_cab = radial_point_c.wedge(dipole_ab);
//
//     let flat_point_c = cga3d::FlatPoint::new(2.0, 3.0, 4.0, 5.0);
//     let plane_abc1 = radial_point_c.wedge(line_ab);
//     let plane_abc2 = dipole_ab.wedge(flat_point_c);
//     let plane_abc3 = line_ab.wedge(radial_point_c);
//     let plane_abc4 = flat_point_c.wedge(dipole_ab);
//
//     let radial_point_d = cga3d::RadialPoint::new(0.0, 0.0, 0.0, 0.0, 0.0);
//     let dipole_cd = radial_point_c.wedge(radial_point_d);
//     let sphere_abcd1 = circle_abc.wedge(radial_point_d);
//     let sphere_abcd2 = dipole_ab.wedge(dipole_cd);
//     let sphere_abcd3 = radial_point_d.wedge(circle_abc);
//     let sphere_abcd4 = dipole_cd.wedge(dipole_ab);
//
//     // Meets
//
//     let circle = sphere_abcd1.anti_wedge(sphere_abcd2);
//
//     // Motor?
//
//     let motor = line_ab.geometric_anti_product(line_ba.anti_reversal());
//
//
//     // Flector?
//
//     let flector = flat_point_b + plane_abc1;
//
//     // TODO dilation?
// }


#[test]
fn cga_ergonomics() {
    // TODO here I will invent some new constructors and methods to conveniently work with
    //  the CGA objects. For example, a sphere by specifying a center point and radius.
    //  I might also want to make a sandwich operator thing.
}



#[test]
fn cga_correctness() {
    // TODO correct types is one thing, ergonomics is another, and correct behavior is yet another.
    //  Here we make sure that CGA actually behaves as intended. Of particular concern is the fact
    //  that e+ and e- are kind of hidden in the algebra, and the code was generated using
    //  e0 and e4 instead.
}































//