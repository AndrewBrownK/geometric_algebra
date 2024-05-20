use crate::norms::UnitizedRadiusNormSquared;
use crate::products::exterior::{AntiWedge, Wedge};
use crate::{FlatPoint, RoundPoint};

#[test]
fn circle_thing() {
    let left = RoundPoint::new(-1.0, 0.0, 0.0, 1.0, 0.5);
    let top = RoundPoint::new(0.0, 1.0, 0.0, 1.0, 0.5);
    let right = RoundPoint::new(1.0, 0.0, 0.0, 1.0, 0.5);
    let left_to_top_dipole = left.wedge(top);
    let circle = left_to_top_dipole.wedge(right);

    let grid = 10;
    for x in 0..grid {
        let x = (2.0 * (x as f32) / (grid as f32)) - 1.0;
        for y in 0..grid {
            let y = (2.0 * (y as f32) / (grid as f32)) - 1.0;
            let fp_e5 = (x * x + y * y + 0.0) / 2.0;
            let fragment_point = RoundPoint::new(x, y, 0.0, 1.0, fp_e5);
            let direction = FlatPoint::new(0.0, 0.0, 1.0, 0.0);
            let fragment_as_line = fragment_point.wedge(direction);
            let meet_fragment_and_circle = circle.anti_wedge(fragment_as_line);
            let radius_squared = meet_fragment_and_circle.unitized_radius_norm_squared();
            println!("({x:.2}, {y:.2}) -> {radius_squared:.2}");
        }
    }
}
