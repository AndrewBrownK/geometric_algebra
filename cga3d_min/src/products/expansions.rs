// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::exterior::*;
use crate::*;

/// Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait Expansion<T> {
    type Output;
    fn expansion(self, other: T) -> Self::Output;
}

impl Expansion<Circle> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Circle {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Circle {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Dipole {
    type Output = Circle;

    fn expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Dipole {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for FlatPoint {
    type Output = Line;

    fn expansion(self, other: Plane) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlatPoint {
    type Output = Line;

    fn expansion(self, other: Sphere) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Flector {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Flector {
    type Output = Plane;

    fn expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Flector {
    type Output = Motor;

    fn expansion(self, other: Plane) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Flector {
    type Output = Motor;

    fn expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Line {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Line {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Line {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Line {
    type Output = Plane;

    fn expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Line {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Motor {
    type Output = Plane;

    fn expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Motor {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: FlatPoint) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Line) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: RoundPoint) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}
