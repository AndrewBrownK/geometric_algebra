use crate::characteristics::{AntiSqrt, Attitude};
use crate::metrics::Distance;
use crate::norms::{BulkNorm, UnitizedNorm, WeightNorm};
use crate::products::dot::AntiDot;
use crate::products::exterior::Wedge;
use crate::{FlatPoint, Origin, RoundPoint, RoundPointOnOrigin};
use std::ops::Add;

#[test]
fn round_point_distances() {
    // TODO in order to better understand the "e1, e2, e3" object (tentatively called
    //  "RoundPointBulk"), I want to better understand the meaning of distances between
    //  RoundPoints, particularly RoundPoints with imaginary radius. If you consider
    //  a RoundPointOnOrigin, and then bring its e4 component to zero, you can see it
    //  approaches a point at infinity with an imaginary radius that stays at the origin.
    //  Thus it creates something like a PlaneAtOrigin. This also makes sense that a
    //  RoundPointBulk is commonly found as a WeightDual to various Planes. But if it's not
    //  a plane overtly.... then what is it? If we focus on the center of the point, it is like
    //  a "PlaneNormalVector" or something.. but as we know, the radial leash of a RoundPoint
    //  is one of it's most defining features. So it seems inappropriate to ignore. The e1,e2,e3
    //  elements certainly specify a direction, but if you want to be zero-distance from the
    //  object, you need to be near a PlaneAtOrigin, not near infinity. I guess.... this gets at
    //  the heart of a big and very interesting question. What is the fundamental authentic
    //  difference between a Sphere and RoundPoint? Because a plane is a sphere, and the sphere
    //  IS its surface, so a PlaneAtOrigin IS its surface. But a RoundPointBulk is zero distance
    //  away from some PlaneAtOrigin while NOT being its surface... it's weird.

    // TODO new problem.... RoundPoints don't have very many distance implementations.
    // TODO FlatPoints are same situation. Only have distance with Circles?

    // RoundPoint with real radius
    // 3, 0, 0 (radius of real 3)
    let a = RoundPoint::new(3.0, 0.0, 0.0, 1.0, 9.0);
    // If you do the above with a radius of imaginary 3, you get an imaginary distance for all results.

    // RoundPoints with zero radius

    // TODO none of the distances measured here care about the e5 parameter
    //  So the distance formula is measuring the distance between the centers of each
    //  RoundPoint, and not considering the radius.

    let b = RoundPoint::new(-3.0, 0.0, 0.0, 1.0, 4.5);
    let c = RoundPoint::new(-2.0, 0.0, 0.0, 1.0, 2.0);
    let d = RoundPoint::new(-1.0, 0.0, 0.0, 1.0, 0.5);
    let e = RoundPoint::new(0.0, 0.0, 0.0, 1.0, 0.0);
    let f = RoundPoint::new(1.0, 0.0, 0.0, 1.0, 0.5);
    let g = RoundPoint::new(2.0, 0.0, 0.0, 1.0, 2.0);
    let h = RoundPoint::new(3.0, 0.0, 0.0, 1.0, 4.5);
    let i = RoundPoint::new(4.0, 0.0, 0.0, 1.0, 8.0);
    let j = RoundPoint::new(5.0, 0.0, 0.0, 1.0, 12.5);
    let k = RoundPoint::new(6.0, 0.0, 0.0, 1.0, 18.0);

    for some_point in vec![b, c, d, e, f, g, h, i, j, k] {
        // println!("Self {a:?}");
        // println!("Other {some_point:?}");

        let bulk_wedge = a.wedge(some_point);
        let weight_attitude = some_point.attitude();
        // println!("Bulk Wedge {bulk_wedge:?}");
        // println!("Weight Attitude {weight_attitude:?}");

        let bulk_attitude = bulk_wedge.attitude();
        let weight_wedge = a.wedge(weight_attitude);
        // println!("Bulk Attitude {bulk_attitude:?}");

        // println!("{weight_wedge:?}");
        let weight_anti_dot = weight_wedge.anti_dot(weight_wedge);
        // println!("{weight_anti_dot:?}");
        let weight_part = weight_anti_dot.anti_sqrt();
        // println!("{weight_part:?}");

        // println!("Bulk Part {bulk_part:?}");
        let bulk_part = bulk_attitude.bulk_norm();
        let distance = bulk_part.add(weight_part);
        // let distance = distance.unitized_norm();

        // let distance = a.distance(some_point).unitized_norm();
        println!("RoundPoint distance is {distance:?}");
    }

    // 3, 0, 0
    let a = FlatPoint::new(3.0, 0.0, 0.0, 1.0);

    let b = FlatPoint::new(-3.0, 0.0, 0.0, 1.0);
    let c = FlatPoint::new(-2.0, 0.0, 0.0, 1.0);
    let d = FlatPoint::new(-1.0, 0.0, 0.0, 1.0);
    let e = FlatPoint::new(0.0, 0.0, 0.0, 1.0);
    let f = FlatPoint::new(1.0, 0.0, 0.0, 1.0);
    let g = FlatPoint::new(2.0, 0.0, 0.0, 1.0);
    let h = FlatPoint::new(3.0, 0.0, 0.0, 1.0);
    let i = FlatPoint::new(4.0, 0.0, 0.0, 1.0);
    let j = FlatPoint::new(5.0, 0.0, 0.0, 1.0);
    let k = FlatPoint::new(6.0, 0.0, 0.0, 1.0);

    for some_point in vec![b, c, d, e, f, g, h, i, j, k] {
        let distance = a.distance(some_point).unitized_norm();
        println!("FlatPoint distance is {distance}");
    }
}
