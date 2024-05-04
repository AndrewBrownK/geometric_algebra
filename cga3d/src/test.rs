use crate::characteristics::{AntiSqrt, Attitude, Sqrt};
use crate::metrics::Distance;
use crate::norms::{BulkNorm, UnitizedNorm, WeightNorm};
use crate::products::dot::{AntiDot, Dot};
use crate::products::exterior::{Meet, Wedge};
use crate::{FlatPoint, Horizon, Infinity, Origin, RoundPoint, RoundPointOnOrigin};
use std::ops::Add;
use projective_ga::Unit;
use crate::products::geometric::WedgeDot;
use crate::unitize::Unitize;

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
    let a = RoundPoint::new(3.0, 0.0, 0.0, 1.0, 4.5);
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
        let diff = a - some_point;
        let distance = diff.dot(diff).sqrt();
        // println!("RoundPoint distance is {distance:?}");
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

    for mut some_point in vec![b, c, d, e, f, g, h, i, j, k] {

        // TODO.... apparently.... FlatPoints have an imaginary weight norm?
        //  FlatPoint.anti_dot(FlatPoint) is negative, which then we have to take square root
        //  to find the weight norm.... So any attempt to unitize (requiring division by weight norm)
        //  results in NaN...
        println!("some_point before: {some_point:?}");
        some_point = some_point.unitize();
        println!("some_point after: {some_point:?}");

        let round_a = some_point.meet(Horizon::unit());
        println!("Conversion of some_point: {round_a:?}");
        // let distance = a.distance(some_point).unitized_norm();
        // println!("FlatPoint distance is {distance}");
    }
}
