//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::characteristics::AntiInverse;
use crate::characteristics::Inverse;
use crate::products::geometric::GeometricAntiProduct;
use crate::products::geometric::GeometricProduct;
use crate::*;

/// The Geometric Quotient between `a` and `b` is the geometric product between `a` and `b^-1` (the inverse of `b`).
/// See also "Inverse".
pub trait GeometricQuotient<T> {
    type Output;
    fn geometric_quotient(self, other: T) -> Self::Output;
}

/// The Geometric AntiQuotient between `a` and `b` is the geometric anti-product between `a` and the anti-inverse of `b`.
/// See also "AntiInverse".
pub trait GeometricAntiQuotient<T> {
    type Output;
    fn geometric_anti_quotient(self, other: T) -> Self::Output;
}

impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiScalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiScalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiScalar {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiScalar {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiScalar {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiScalar {
    type Output = Line;

    fn geometric_anti_quotient(self, other: Line) -> Line {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiScalar {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiScalar {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiScalar {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiScalar {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: Scalar) -> Scalar {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiScalar {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiScalar {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiScalar {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Circle {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Circle {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Circle {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Dipole {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Dipole {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Dipole {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DualNum {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: Scalar) -> DualNum {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlatPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for FlatPoint {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Translator {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for FlatPoint {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Line) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for FlatPoint {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for FlatPoint {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPoint {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: Translator) -> FlatPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Translator) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Line {
    type Output = Line;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Line {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Line {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Line {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Motor {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Motor {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Plane {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Plane {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Plane {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Plane) -> Motor {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Plane {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Plane {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Rotor {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPoint {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPoint {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Scalar) -> Sphere {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Scalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Scalar {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Scalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Circle) -> Dipole {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Scalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Dipole) -> Circle {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Scalar {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Scalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Circle {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Scalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Line) -> Dipole {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Scalar {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: Plane) -> RoundPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Scalar {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Sphere {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiScalar {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Scalar {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: Sphere) -> RoundPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Sphere {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Sphere {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Sphere {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Translator {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Translator {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Translator {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Translator {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Plane) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Translator {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other.anti_inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: AntiScalar) -> Scalar {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiScalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Circle) -> Dipole {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiScalar {
    type Output = Circle;

    fn geometric_quotient(self, other: Dipole) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiScalar {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiScalar {
    type Output = Circle;

    fn geometric_quotient(self, other: FlatPoint) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiScalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Line) -> Dipole {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiScalar {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: Plane) -> RoundPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiScalar {
    type Output = Sphere;

    fn geometric_quotient(self, other: RoundPoint) -> Sphere {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Scalar) -> AntiScalar {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiScalar {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: Sphere) -> RoundPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Circle {
    type Output = Dipole;

    fn geometric_quotient(self, other: AntiScalar) -> Dipole {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Circle {
    type Output = Circle;

    fn geometric_quotient(self, other: Scalar) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Dipole {
    type Output = Circle;

    fn geometric_quotient(self, other: AntiScalar) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Dipole {
    type Output = Dipole;

    fn geometric_quotient(self, other: Scalar) -> Dipole {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: AntiScalar) -> DualNum {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: Scalar) -> DualNum {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlatPoint {
    type Output = Circle;

    fn geometric_quotient(self, other: AntiScalar) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: Scalar) -> FlatPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for FlatPoint {
    type Output = Circle;

    fn geometric_quotient(self, other: Translator) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Scalar) -> Flector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Line {
    type Output = Dipole;

    fn geometric_quotient(self, other: AntiScalar) -> Dipole {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: Scalar) -> Line {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Scalar) -> Motor {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Scalar) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Plane {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: Scalar) -> Plane {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Scalar) -> Rotor {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPoint {
    type Output = Sphere;

    fn geometric_quotient(self, other: AntiScalar) -> Sphere {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPoint {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: Scalar) -> RoundPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: AntiScalar) -> AntiScalar {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Scalar {
    type Output = Circle;

    fn geometric_quotient(self, other: Circle) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Scalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Dipole) -> Dipole {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Scalar {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: FlatPoint) -> FlatPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Scalar {
    type Output = Line;

    fn geometric_quotient(self, other: Line) -> Line {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;

    fn geometric_quotient(self, other: Plane) -> Plane {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Scalar {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: RoundPoint) -> RoundPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: Scalar) -> Scalar {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Scalar {
    type Output = Sphere;

    fn geometric_quotient(self, other: Sphere) -> Sphere {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Scalar {
    type Output = Transflector;

    fn geometric_quotient(self, other: Transflector) -> Transflector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Scalar {
    type Output = Translator;

    fn geometric_quotient(self, other: Translator) -> Translator {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Sphere {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPoint {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Sphere {
    type Output = Sphere;

    fn geometric_quotient(self, other: Scalar) -> Sphere {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Transflector {
    type Output = Transflector;

    fn geometric_quotient(self, other: Scalar) -> Transflector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Translator {
    type Output = Circle;

    fn geometric_quotient(self, other: FlatPoint) -> Circle {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Line> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Scalar) -> Translator {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.wedge_dot(other.inverse())
    }
}
