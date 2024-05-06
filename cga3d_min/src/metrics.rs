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

impl CosineAngle<Circle> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Circle {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Dipole {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for FlatPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Line {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for Plane {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for RoundPoint {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Circle> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Circle) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Dipole> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Dipole) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<FlatPoint> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: FlatPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Line> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Line) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Plane> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Plane) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<RoundPoint> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: RoundPoint) -> DualNum {
        self.anti_wedge(other.anti_dual()).bulk_norm().add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl CosineAngle<Sphere> for Sphere {
    type Output = DualNum;

    fn cosine_angle(self, other: Sphere) -> DualNum {
        self.anti_wedge(other.anti_dual()).add(self.weight_norm().anti_wedge_dot(other.weight_norm()))
    }
}

impl SineAngle<Circle> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Circle {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Dipole {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for FlatPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Line {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for Plane {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPoint {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Circle) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Dipole) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: FlatPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Line> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Line) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Plane) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: RoundPoint) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for Sphere {
    type Output = DualNum;

    fn sine_angle(self, other: Sphere) -> DualNum {
        let mut cos: DualNum = self.cosine_angle(other);
        let mut cos_squared: DualNum = cos.wedge_dot(cos);
        let mut sub: DualNum = DualNum::unit().sub(cos_squared);
        sub.sqrt()
    }
}
