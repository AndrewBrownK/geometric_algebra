use cga3d_min::{FlatPoint, RoundPoint};
use cga3d_min::norms::UnitizedRadiusNormSquared;
use cga3d_min::products::exterior::{AntiWedge, Wedge};

fn main() {
    // Start by constructing a Circle.
    // We'll create our circle by joining 3 RoundPoints.

    // Create 3 round points
    let left = RoundPoint::new(-1.0, 0.0, 0.0, 1.0, 0.5);
    let top = RoundPoint::new(0.0, 1.0, 0.0, 1.0, 0.5);
    let right = RoundPoint::new(1.0, 0.0, 0.0, 1.0, 0.5);

    // Join the first pair
    let left_to_top_dipole = left.wedge(top);

    // Join the remaining point
    let circle = left_to_top_dipole.wedge(right);

    // Sample across space to imagine an image of our circle
    let grid = 10;
    for x in 0..=grid {
        let x = (2.0 * (x as f32) / (grid as f32)) - 1.0;
        for y in 0..=grid {
            let y = (2.0 * (y as f32) / (grid as f32)) - 1.0;

            // Let's create a line representing this fragment
            let fp_e5 = (x * x + y * y + 0.0) / 2.0;
            let fragment_point = RoundPoint::new(x, y, 0.0, 1.0, fp_e5);
            let direction = FlatPoint::new(0.0, 0.0, 1.0, 0.0);

            // Join a round point and point at infinity to create a line
            let fragment_as_line = fragment_point.wedge(direction);

            // Now perform a meet to get a RoundPoint centered on the line and contained by the circle
            let meet_fragment_and_circle = circle.anti_wedge(fragment_as_line);

            // Now get the squared radius of the meet
            let radius_squared = meet_fragment_and_circle.unitized_radius_norm_squared();
            println!("({x:.2}, {y:.2}) -> {radius_squared:.2}");
        }
    }
}
