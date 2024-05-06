//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::exterior::*;
use crate::*;

/// Contraction (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait Contraction<T> {
    type Output;
    fn contraction(self, other: T) -> Self::Output;
}

impl Contraction<Circle> for Circle {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Circle {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Circle {
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Flector {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Flector {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Line {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Line {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Line {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Line {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Line {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Line {
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Motor {
    type Output = RoundPoint;

    fn contraction(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Motor {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Plane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Plane {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Plane {
    type Output = Dipole;

    fn contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Plane {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Plane {
    type Output = Circle;

    fn contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Plane {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Sphere {
    type Output = Circle;

    fn contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}
