use crate::characteristics::{AntiSqrt, Attitude, Carrier, Center, CoCarrier, Container, Sqrt};
use crate::metrics::Distance;
use crate::products::dot::{AntiDot, Dot};
use crate::products::exterior::{Join, Meet, Wedge};
use crate::unitize::Unitize;
use crate::{Circle, Dipole, FlatPoint, Horizon, Infinity, Line, Origin, RoundPoint};
use projective_ga::Unit;
use std::ops::Add;
use crate::norms::{FlatWeightNormSquared, UnitizedRadiusNormSquared};
use crate::products::projections::ProjectOrthogonallyOnto;
use crate::products::rejections::RejectOrthogonallyFrom;

fn unitizing_equals<A, B>(a: A, b: B, epsilon: f32) -> bool {
    todo!()
}

#[test]
fn round_point_distances() {
    //TODO speculation on distance formula for CGA
    // Basically, if it is possible at all, then it looks something like this:
    // - take the meet of two objects
    // - interpret the radius of the meet according to some SphereAtOrigin that represents
    //   the curvature of your space
    // - the SphereAtOrigin will be a Horizon for flat space, or have a real radius for
    //   elliptic space, or imaginary radius for hyperbolic space

    // RoundPoint with real radius
    // 3, 0, 0 (radius of real 3)
    let a = RoundPoint::new(3.0, 0.0, 0.0, 1.0, 4.5);
    // If you do the above with a radius of imaginary 3, you get an imaginary distance for all results.

    // RoundPoints with zero radius
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

    // for mut some_point in vec![b, c, d, e, f, g, h, i, j, k] {
    //     println!("some_point before: {some_point:?}");
    //     some_point = some_point.unitize();
    //     println!("some_point after: {some_point:?}");
    //
    //     let round_a = some_point.meet(Horizon::unit());
    //     println!("Conversion of some_point: {round_a:?}");
    //     // let distance = a.distance(some_point).unitized_norm();
    //     // println!("FlatPoint distance is {distance}");
    // }
}

fn distance_speculation() {
    // assume we can calculate distance between two RoundPoints
    // assume we can calculate distance between two Spheres
    // Assume we can calculate distance between RoundPoint and Sphere

    // The remaining combinations necessary...
    // - Dipole and RoundPoint
    // - Dipole and Dipole
    // - Dipole and Circle
    // - Dipole and Sphere
    // - Circle and RoundPoint
    // - Circle and Circle
    // - Circle and Sphere

    // So the strategy is this... use geometric operations to create a RoundPoint of zero radius
    // on the object that is closest to the other object being compared. If we do this from
    // one side, then that solves the following:
    // - Dipole and RoundPoint
    // - Dipole and Sphere
    // - Circle and RoundPoint
    // - Circle and Sphere

    // The above has the slightly awkward caveat of imaginary radius objects. So watch out for that.

    // After that however, we still have these remaining cases that can get awkward if they are
    // parallel or skew.
    // - Dipole and Dipole
    // - Dipole and Circle
    // - Circle and Circle

    // Anyway... more on creating a RoundPoint on a Dipole|Circle closest to RoundPoint|Sphere...
    // Take the RoundPoint|Sphere and take its center (which is itself a RoundPoint).
    // Force the Center (RoundPoint) radius to zero.
    // Then take the carrier of the other Dipole|Circle (which will be a Line|Plane), and
    // orthogonally project the earlier mentioned Center (RoundPoint) onto it.
    // Then orthogonally project the Center (RoundPoint) again, but this time onto the Container of
    // the Dipole|Circle. In fact... you might not need the Container at all... assuming these
    // impls actually work and don't have anything funky about them:
    // - impl ProjectOrthogonallyOnto<Dipole> for RoundPoint
    // - impl ProjectOrthogonallyOnto<Circle> for RoundPoint
    // Even better if those projections work accurately for Dipole|Circle of imaginary radius.

    // So what about the potentially skew/parallel cases?
    // Assuming for starters that these round objects are really round, and not infinite radius flat.
    // You can still try the above technique and see what happens. Each dipole has a center, each
    // circle has a center. Consecutive orthogonal projections, and bam you've got closest points.
    // But then what about the parallel/skew case?
    // Let's start with parallel circles because there is fewer degrees of freedom for degenerate
    // cases like this. There's only one way for circles to be parallel, but lots of ways for
    // dipoles to be parallel or skew.

    // Well... huh... when I first started attempting to create this technique, I was imagining
    // somehow creating a RoundPoint by meeting the carriers of both sides... but it's actually
    // kind of hard to figure out how to create a RoundPoint that way, and so then I suggested
    // using the center operation... but meeting parallel carriers goes to infinity, which is the
    // awkward part, but orthogonally projecting a's center onto b's carrier is not awkward at all.
    // For parallel circles, it works quite easily in fact. Still though... it's not perfect...

    // Imagine you have two concentric circles. One has twice the radius of the other.
    // They are on the xy plane. now take the smaller concentric circle, and translate it along the
    // z axis. And then finally, translate it slightly on the x axis (so the circles are no longer
    // concentric). Viewed from z=infinity, one circle should look like it's inside the other circle.
    // So.... now we try to find the distance between them. Take the center of one of the circles,
    // and project it onto the other's carrier, and then project onto the object. This gives us
    // the closest point on the bigger circle, but the furthest point on the smaller circle.
    // So how do we fix that?
    // Well... I have a hunch... since we're taking the centers anyway... there might be some
    // insight or manipulating we can do with the relative positioning and relative radii of the
    // two centers.

    // I'm really aching to see some 3d visualizations. Maybe it's time to start
    // developing a "CGA Playground" demo. Even without all the fundamentals (like Distance)
    // figured out yet.

    // Another crazy example...
    // 2 circles on xy plane
    // a has radius 10
    // b has radius 9
    // translate b along z axis so its carrier is at z = 1
    // then rotate b so its carrier is the xz plane instead of the xy plane
    // hm
    // I think the above example, and also concentric circles, demonstrate something.
    // Sometimes the closest point between two objects is not a point after all.
    // Sometimes the closest "point" is actually a dipole, a circle itself, or even a sphere in
    // the case of concentric spheres. Even non-concentric spheres, if overlapping, meet as a
    // circle at the point of overlap ("point" of zero distance, aka the "closest points" of the
    // objects).
    // So what I think this means is... you have to solve this bottom-up. Higher dimensional objects
    // might be closest to other objects via ANY of the lower dimensional options. So start with
    // getting distances between RoundPoints... then RoundPoints and Dipoles... then Dipoles and
    // Dipoles.. then Circles and RoundPoints, Dipoles, and lastly Circles... so on.

    // Hmmmm... speaking of meets... you can very well take the meet operation on two objects
    // that aren't actually touching... you just get an imaginary result if they're not touching..
    // so.. maybe the distance has been encoded in the result of the meet this entire time?
}


const EPSILON: f32 = 0.0000001;
const EPSILON_SQUARED: f32 = EPSILON * EPSILON;

enum ClosestGeometryOnCircle {
    Circle(Circle),
    Dipole(Dipole),
    FlatPoint(FlatPoint)
}

fn closest_geometry_on_circle(
    line: Line,
    circle: Circle
) -> ClosestGeometryOnCircle {
    let circle_co_carrier = circle.co_carrier();
    if unitizing_equals(circle_co_carrier, line, EPSILON) {
        // Line passes through Center of Circle, orthogonal to Circle
        return ClosestGeometryOnCircle::Circle(circle)
    }

    let dipole = line.meet(circle.container());
    let dipole_center = dipole.center();
    let circle_center = circle.center();
    if unitizing_equals(dipole_center, circle_center, EPSILON) {
        // Line passes through Center of Circle, but at an angle
        let dipole = dipole.project_orthogonally_onto(circle);
        return ClosestGeometryOnCircle::Dipole(dipole)
    }

    let plane = circle.carrier();
    let line_parallel_carrier = line.project_orthogonally_onto(plane);
    let line_perpendicular_carrier = line.reject_orthogonally_from(plane);






    let round_point = line.meet(circle);
    let rp_rad_sq = round_point.unitized_radius_norm_squared();
    if rp_rad_sq.abs() < EPSILON_SQUARED {
        // Line intersects Circle
        let point = round_point.join(Infinity::unit());
        return ClosestGeometryOnCircle::FlatPoint(point)
    }
    if rp_rad_sq > EPSILON_SQUARED {
        // Line passes inside circle (off Center)

    }
    // Line passes outside circle





    //
    // let dipole = line.meet(circle.container()).project_orthogonally_onto(circle);
    // let dp_rad_sq = dipole.unitized_radius_norm_squared();
    // let thing = round_point.project_orthogonally_onto(dipole);
    //
    // // Line goes through Circle
    // if rp_rad_sq > 0.0 && rp_rad_sq >= dp_rad_sq {
    //     // both ends of Dipole are closest distance
    //     return ClosestGeometryOnCircle::Dipole(dipole)
    // }
    //
    // let flat_point = line.meet(circle.carrier());
    // if flat_point.flat_weight_norm_squared().group0() < EPSILON_SQUARED {
    //
    // }
}