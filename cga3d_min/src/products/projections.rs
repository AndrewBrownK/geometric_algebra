//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::products::exterior::*;
use crate::*;

/// Orthogonal Projection
/// Typically involves bringing a lower dimensional object to a higher dimensional object
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectOrthogonallyOnto<T> {
    type Output;
    fn project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Orthogonal AntiProjection
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait AntiProjectOrthogonallyOnto<T> {
    type Output;
    fn anti_project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Central (to origin) Projection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectViaOriginOnto<T> {
    type Output;
    fn project_via_origin_onto(self, other: T) -> Self::Output;
}

/// Outward (to horizon) AntiProjection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
pub trait AntiProjectViaHorizonOnto<T> {
    type Output;
    fn anti_project_via_horizon_onto(self, other: T) -> Self::Output;
}

impl AntiProjectOrthogonallyOnto<Circle> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Circle {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Circle {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Circle {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Flector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Flector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Flector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Flector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Line {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;

    fn anti_project_orthogonally_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Line {
    type Output = Circle;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Line {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Motor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Motor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Motor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Plane) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Plane {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Plane {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Rotor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Rotor {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Sphere {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Transflector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Transflector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Transflector {
    type Output = Plane;

    fn anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Transflector {
    type Output = Sphere;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Circle> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Dipole> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<FlatPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Flector> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Line> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Motor> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Plane> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Sphere> for Translator {
    type Output = AntiScalar;

    fn anti_project_orthogonally_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectOrthogonallyOnto<Translator> for Translator {
    type Output = MultiVector;

    fn anti_project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.anti_dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Plane) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Transflector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Transflector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Rotor) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Transflector) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.anti_wedge(other.dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Circle {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: RoundPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Transflector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Transflector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Transflector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Transflector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Line> for Translator {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Translator {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.anti_dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Circle {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPoint) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Transflector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Dipole> for Transflector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Transflector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Transflector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Transflector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Circle> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Line> for Translator {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Motor> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Plane> for Translator {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Transflector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Transflector) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}

impl ProjectViaOriginOnto<Translator> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.wedge(other.dual()))
    }
}
