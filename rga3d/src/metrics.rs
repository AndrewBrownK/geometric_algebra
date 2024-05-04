//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::characteristics::{Attitude, Sqrt};
use crate::involutions::AntiDual;
use crate::norms::*;
use crate::products::exterior::{AntiWedge, Wedge};
use crate::products::geometric::{GeometricAntiProduct, GeometricProduct};
use crate::products::projections::*;
use crate::unitize::Unitize;
use crate::*;

/// Euclidean distance between objects
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance
/// distance(a,b) = bulk_norm(attitude(a wedge b)) + weight_norm(a wedge attitude(b))
/// where attitude(c) = c anti_wedge complement(e4) where e4 is the projective dimension
pub trait Distance<T> {
    type Output;
    fn distance(self, other: T) -> Self::Output;
}

/// The cosine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait CosineAngle<T> {
    type Output;
    fn cosine_angle(self, other: T) -> Self::Output;
}

/// The sine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait SineAngle<T> {
    type Output;
    fn sine_angle(self, other: T) -> Self::Output;
}

impl CosineAngle<Line> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Origin> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Origin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Point> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Point) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Origin> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Origin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Point> for LineAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Point) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Origin> for Origin {
    type Output = DualNum;

    fn cosine_angle(self, other: Origin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Point> for Origin {
    type Output = DualNum;

    fn cosine_angle(self, other: Point) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Origin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Origin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<PlaneAtOrigin> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: PlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Point> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Point) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Line> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Origin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Origin) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: PlaneAtOrigin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Point> for PlaneAtOrigin {
    type Output = DualNum;

    fn cosine_angle(self, other: Point) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Origin> for Point {
    type Output = DualNum;

    fn cosine_angle(self, other: Origin) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl CosineAngle<Point> for Point {
    type Output = DualNum;

    fn cosine_angle(self, other: Point) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().geometric_anti_product(other.weight_norm()))
    }
}

impl Distance<Line> for Line {
    type Output = DualNum;

    fn distance(self, other: Line) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for Line {
    type Output = DualNum;

    fn distance(self, other: LineAtOrigin) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Line {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Line {
    type Output = DualNum;

    fn distance(self, other: Point) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for LineAtOrigin {
    type Output = DualNum;

    fn distance(self, other: Line) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for LineAtOrigin {
    type Output = DualNum;

    fn distance(self, other: Point) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Origin {
    type Output = DualNum;

    fn distance(self, other: Line) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Plane> for Origin {
    type Output = DualNum;

    fn distance(self, other: Plane) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Origin {
    type Output = DualNum;

    fn distance(self, other: Point) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Plane {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Plane {
    type Output = DualNum;

    fn distance(self, other: Point) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for PlaneAtOrigin {
    type Output = DualNum;

    fn distance(self, other: Point) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for Point {
    type Output = DualNum;

    fn distance(self, other: Line) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for Point {
    type Output = DualNum;

    fn distance(self, other: LineAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Point {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Plane> for Point {
    type Output = DualNum;

    fn distance(self, other: Plane) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<PlaneAtOrigin> for Point {
    type Output = DualNum;

    fn distance(self, other: PlaneAtOrigin) -> DualNum {
        self.anti_wedge(other).add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Point> for Point {
    type Output = DualNum;

    fn distance(self, other: Point) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl SineAngle<Line> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Origin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Point> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Point) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Origin> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Origin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Point> for LineAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Point) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Origin {
    type Output = DualNum;

    fn sine_angle(self, other: Origin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Point> for Origin {
    type Output = DualNum;

    fn sine_angle(self, other: Point) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Origin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: PlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Point> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Point) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: LineAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Origin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Origin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: PlaneAtOrigin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Point> for PlaneAtOrigin {
    type Output = DualNum;

    fn sine_angle(self, other: Point) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Origin> for Point {
    type Output = DualNum;

    fn sine_angle(self, other: Origin) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Point> for Point {
    type Output = DualNum;

    fn sine_angle(self, other: Point) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.geometric_product(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}
