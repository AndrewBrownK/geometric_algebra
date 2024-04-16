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
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiScalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Circle) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for AntiScalar {
    type Output = CircleBulkAspect;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for AntiScalar {
    type Output = CircleCarrierAspect;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiScalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for AntiScalar {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for AntiScalar {
    type Output = DipoleCarrierAspect;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiScalar {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiScalar {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiScalar {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiScalar {
    type Output = Line;

    fn geometric_anti_quotient(self, other: Line) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn geometric_anti_quotient(self, other: Magnitude) -> Magnitude {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiScalar {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Motor) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiScalar {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: Plane) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiScalar {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiScalar {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: RoundPoint) -> RoundPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for AntiScalar {
    type Output = RoundPointAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiScalar {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for AntiScalar {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for AntiScalar {
    type Output = RoundPointCarrierAspect;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: Scalar) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiScalar {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Sphere) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiScalar {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiScalar {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Translator) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Circle {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Circle {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for CircleBulkAspect {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for CircleBulkAspect {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleBulkAspect {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleBulkAspect {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for CircleBulkAspect {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleBulkAspect {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleBulkAspect {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Translator) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleCarrierAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleCarrierAspect {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Dipole {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Dipole {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for DipoleBulkAspect {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for DipoleBulkAspect {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleBulkAspect {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for DipoleBulkAspect {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleBulkAspect {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleCarrierAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleCarrierAspect {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlatPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for FlatPoint {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for FlatPoint {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for FlatPoint {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Line) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for FlatPoint {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for FlatPoint {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for FlatPoint {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPoint {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: Translator) -> FlatPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: FlatPoint) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: Translator) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for FlatPointAtOrigin {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for FlatPointAtOrigin {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for FlatPointAtOrigin {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for FlatPointAtOrigin {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Line) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for FlatPointAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for FlatPointAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for FlatPointAtOrigin {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPointAtOrigin {
    type Output = CircleBulkAspect;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: Translator) -> FlatPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Translator) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Translator) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Horizon {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Horizon {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Horizon {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Horizon {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Horizon {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: Scalar) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: Translator) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Infinity {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Infinity {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Infinity {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Infinity {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Infinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Infinity {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: Scalar) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Infinity {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: Translator) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Line {
    type Output = Line;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Line {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Line {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Line {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: FlatPoint) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Translator) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for LineAtOrigin {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for LineAtOrigin {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for LineAtOrigin {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Magnitude {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn geometric_anti_quotient(self, other: Magnitude) -> Magnitude {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Magnitude {
    type Output = Magnitude;

    fn geometric_anti_quotient(self, other: Scalar) -> Magnitude {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Motor {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Origin {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Origin {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Origin {
    type Output = SphereWeightAspect;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> SphereWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Origin {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Origin {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Origin {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Origin {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Origin {
    type Output = SphereWeightAspect;

    fn geometric_anti_quotient(self, other: Scalar) -> SphereWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Plane {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Plane {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Plane) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Plane {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Plane {
    type Output = RoundPointAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Plane {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for PlaneAtOrigin {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for PlaneAtOrigin {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Plane) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for PlaneAtOrigin {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for PlaneAtOrigin {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPoint {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPoint {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Scalar) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for RoundPointAtInfinity {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointAtInfinity {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: Scalar) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for RoundPointAtOrigin {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for RoundPointAtOrigin {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for RoundPointAtOrigin {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointAtOrigin {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Scalar) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for RoundPointBulkAspect {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for RoundPointBulkAspect {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for RoundPointBulkAspect {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointBulkAspect {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPointCarrierAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Scalar) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Scalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Scalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Circle) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Scalar {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Scalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Scalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Dipole) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Scalar {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Scalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Scalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: FlatPoint) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Scalar {
    type Output = CircleBulkAspect;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> CircleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Scalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Line) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Scalar {
    type Output = DipoleBulkAspect;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> DipoleBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Scalar {
    type Output = Magnitude;

    fn geometric_anti_quotient(self, other: Magnitude) -> Magnitude {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Scalar {
    type Output = RoundPointAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> RoundPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Scalar {
    type Output = RoundPointBulkAspect;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Scalar {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Scalar {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Scalar {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Scalar {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Scalar {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Scalar {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: Sphere) -> RoundPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Sphere {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Sphere {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn geometric_anti_quotient(self, other: AntiScalar) -> SphereWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> SphereWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for SphereWeightAspect {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> DipoleWeightAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for SphereWeightAspect {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: Scalar) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Translator {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulkAspect> for Translator {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: CircleBulkAspect) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulkAspect> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Translator {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: FlatPoint) -> FlatPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Translator {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> FlatPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Translator {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Magnitude> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Plane) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulkAspect> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Translator {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Translator) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: AntiScalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiScalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Circle) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for AntiScalar {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for AntiScalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiScalar {
    type Output = Circle;

    fn geometric_quotient(self, other: Dipole) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for AntiScalar {
    type Output = Circle;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiScalar {
    type Output = Circle;

    fn geometric_quotient(self, other: FlatPoint) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiScalar {
    type Output = CircleBulkAspect;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> CircleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiScalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Line) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiScalar {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: LineAtOrigin) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn geometric_quotient(self, other: Magnitude) -> Magnitude {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiScalar {
    type Output = RoundPointAtInfinity;

    fn geometric_quotient(self, other: Plane) -> RoundPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiScalar {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiScalar {
    type Output = Sphere;

    fn geometric_quotient(self, other: RoundPoint) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for AntiScalar {
    type Output = Plane;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiScalar {
    type Output = Sphere;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for AntiScalar {
    type Output = Sphere;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Scalar) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiScalar {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: Sphere) -> RoundPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Circle {
    type Output = Dipole;

    fn geometric_quotient(self, other: AntiScalar) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Circle {
    type Output = Circle;

    fn geometric_quotient(self, other: Scalar) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleBulkAspect {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for CircleBulkAspect {
    type Output = Scalar;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleBulkAspect {
    type Output = Translator;

    fn geometric_quotient(self, other: FlatPoint) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleBulkAspect {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleBulkAspect {
    type Output = Transflector;

    fn geometric_quotient(self, other: Line) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleBulkAspect {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleBulkAspect {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleBulkAspect {
    type Output = Sphere;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleBulkAspect {
    type Output = CircleBulkAspect;

    fn geometric_quotient(self, other: Scalar) -> CircleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleBulkAspect {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: Translator) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleCarrierAspect {
    type Output = Dipole;

    fn geometric_quotient(self, other: AntiScalar) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn geometric_quotient(self, other: Scalar) -> CircleCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for CircleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: CircleBulkAspect) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for CircleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: Scalar) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Dipole {
    type Output = Circle;

    fn geometric_quotient(self, other: AntiScalar) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Dipole {
    type Output = Dipole;

    fn geometric_quotient(self, other: Scalar) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleBulkAspect {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: CircleBulkAspect) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleBulkAspect {
    type Output = Transflector;

    fn geometric_quotient(self, other: FlatPoint) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleBulkAspect {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleBulkAspect {
    type Output = Rotor;

    fn geometric_quotient(self, other: LineAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleBulkAspect {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleBulkAspect {
    type Output = Circle;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleBulkAspect {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: Scalar) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleCarrierAspect {
    type Output = Circle;

    fn geometric_quotient(self, other: AntiScalar) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn geometric_quotient(self, other: Scalar) -> DipoleCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: AntiScalar) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for DipoleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: CircleBulkAspect) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for DipoleWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: Scalar) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlatPoint {
    type Output = Circle;

    fn geometric_quotient(self, other: AntiScalar) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for FlatPoint {
    type Output = Translator;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for FlatPoint {
    type Output = Transflector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: Scalar) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for FlatPoint {
    type Output = Circle;

    fn geometric_quotient(self, other: Translator) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: CircleBulkAspect) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: FlatPoint) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Flector) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Plane) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Transflector) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Translator) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlatPointAtOrigin {
    type Output = CircleBulkAspect;

    fn geometric_quotient(self, other: AntiScalar) -> CircleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: CircleBulkAspect) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: LineAtOrigin) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for FlatPointAtOrigin {
    type Output = Circle;

    fn geometric_quotient(self, other: Translator) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Scalar) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: FlatPoint) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Flector) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Plane) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Transflector) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Horizon {
    type Output = Infinity;

    fn geometric_quotient(self, other: AntiScalar) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Horizon {
    type Output = Infinity;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Horizon {
    type Output = Horizon;

    fn geometric_quotient(self, other: FlatPoint) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Horizon {
    type Output = Horizon;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Horizon {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Flector) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Line) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: LineAtOrigin) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: Plane) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Horizon {
    type Output = Horizon;

    fn geometric_quotient(self, other: Scalar) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: Transflector) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Horizon {
    type Output = Infinity;

    fn geometric_quotient(self, other: Translator) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Infinity {
    type Output = Horizon;

    fn geometric_quotient(self, other: AntiScalar) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Infinity {
    type Output = Horizon;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Infinity {
    type Output = Infinity;

    fn geometric_quotient(self, other: FlatPoint) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: Line) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Infinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Plane) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Infinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Infinity {
    type Output = Infinity;

    fn geometric_quotient(self, other: Scalar) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Transflector) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Infinity {
    type Output = Horizon;

    fn geometric_quotient(self, other: Translator) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Line {
    type Output = Dipole;

    fn geometric_quotient(self, other: AntiScalar) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Line {
    type Output = Transflector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: Scalar) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: CircleBulkAspect) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: FlatPoint) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Line) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: LineAtOrigin) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: Translator) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for LineAtOrigin {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for LineAtOrigin {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Dipole;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn geometric_quotient(self, other: AntiScalar) -> Magnitude {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn geometric_quotient(self, other: Magnitude) -> Magnitude {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Magnitude {
    type Output = Magnitude;

    fn geometric_quotient(self, other: Scalar) -> Magnitude {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Magnitude {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Scalar) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Origin {
    type Output = SphereWeightAspect;

    fn geometric_quotient(self, other: AntiScalar) -> SphereWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Origin {
    type Output = SphereWeightAspect;

    fn geometric_quotient(self, other: CircleBulkAspect) -> SphereWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Origin {
    type Output = SphereWeightAspect;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> SphereWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Origin {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Origin {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Origin {
    type Output = Origin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Origin {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: LineAtOrigin) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Origin {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Origin {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Origin {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Origin {
    type Output = Origin;

    fn geometric_quotient(self, other: Scalar) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Plane {
    type Output = RoundPointAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: Scalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for PlaneAtOrigin {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Motor;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Circle;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Scalar) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPoint {
    type Output = Sphere;

    fn geometric_quotient(self, other: AntiScalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPoint {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: Scalar) -> RoundPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPointAtInfinity {
    type Output = Plane;

    fn geometric_quotient(self, other: AntiScalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPointAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for RoundPointAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPointAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPointAtInfinity {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = Motor;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPointAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> RoundPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPointAtInfinity {
    type Output = Transflector;

    fn geometric_quotient(self, other: Translator) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPointAtOrigin {
    type Output = Sphere;

    fn geometric_quotient(self, other: AntiScalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for RoundPointAtOrigin {
    type Output = Sphere;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for RoundPointAtOrigin {
    type Output = Circle;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for RoundPointAtOrigin {
    type Output = Dipole;

    fn geometric_quotient(self, other: LineAtOrigin) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = Circle;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for RoundPointAtOrigin {
    type Output = Dipole;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPointBulkAspect {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for RoundPointBulkAspect {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: CircleBulkAspect) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for RoundPointBulkAspect {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPointBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for RoundPointBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPointBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPointBulkAspect {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPointBulkAspect {
    type Output = Rotor;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPointBulkAspect {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = Dipole;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointBulkAspect {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: Scalar) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPointBulkAspect {
    type Output = Transflector;

    fn geometric_quotient(self, other: Translator) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn geometric_quotient(self, other: AntiScalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn geometric_quotient(self, other: Scalar) -> RoundPointCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: AntiScalar) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Scalar {
    type Output = Circle;

    fn geometric_quotient(self, other: Circle) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Scalar {
    type Output = CircleBulkAspect;

    fn geometric_quotient(self, other: CircleBulkAspect) -> CircleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Scalar {
    type Output = CircleCarrierAspect;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Scalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Dipole) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Scalar {
    type Output = DipoleBulkAspect;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> DipoleBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Scalar {
    type Output = DipoleCarrierAspect;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: FlatPoint) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Scalar {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Scalar {
    type Output = Line;

    fn geometric_quotient(self, other: Line) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Scalar {
    type Output = Magnitude;

    fn geometric_quotient(self, other: Magnitude) -> Magnitude {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;

    fn geometric_quotient(self, other: Plane) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Scalar {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: RoundPoint) -> RoundPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Scalar {
    type Output = RoundPointAtInfinity;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Scalar {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Scalar {
    type Output = RoundPointBulkAspect;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Scalar {
    type Output = RoundPointCarrierAspect;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: Scalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Scalar {
    type Output = Sphere;

    fn geometric_quotient(self, other: Sphere) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Scalar {
    type Output = Transflector;

    fn geometric_quotient(self, other: Transflector) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Scalar {
    type Output = Translator;

    fn geometric_quotient(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Sphere {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Sphere {
    type Output = Sphere;

    fn geometric_quotient(self, other: Scalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for SphereWeightAspect {
    type Output = Origin;

    fn geometric_quotient(self, other: AntiScalar) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for SphereWeightAspect {
    type Output = Origin;

    fn geometric_quotient(self, other: CircleBulkAspect) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for SphereWeightAspect {
    type Output = Origin;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> SphereWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: LineAtOrigin) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> DipoleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> CircleWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for SphereWeightAspect {
    type Output = SphereWeightAspect;

    fn geometric_quotient(self, other: Scalar) -> SphereWeightAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for SphereWeightAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Transflector {
    type Output = Transflector;

    fn geometric_quotient(self, other: Scalar) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulkAspect> for Translator {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: CircleBulkAspect) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulkAspect> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulkAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Translator {
    type Output = Circle;

    fn geometric_quotient(self, other: FlatPoint) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Translator {
    type Output = Circle;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Magnitude> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Magnitude) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for Translator {
    type Output = Transflector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulkAspect> for Translator {
    type Output = Transflector;

    fn geometric_quotient(self, other: RoundPointBulkAspect) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Scalar) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}
