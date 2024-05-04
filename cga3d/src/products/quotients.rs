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

impl GeometricAntiQuotient<CircleBulk> for AntiScalar {
    type Output = CircleBulk;

    fn geometric_anti_quotient(self, other: CircleBulk) -> CircleBulk {
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

impl GeometricAntiQuotient<DipoleBulk> for AntiScalar {
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> DipoleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for AntiScalar {
    type Output = DipoleCarrierAspect;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiScalar {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
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

impl GeometricAntiQuotient<RoundPointBulk> for AntiScalar {
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> RoundPointBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for AntiScalar {
    type Output = RoundPointOnOrigin;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: Scalar) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for AntiScalar {
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> SpacialCurvature {
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

impl GeometricAntiQuotient<CircleBulk> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Circle {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<AntiScalar> for CircleBulk {
    type Output = CircleBulk;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for CircleBulk {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: CircleBulk) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleBulk {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleBulk {
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> RoundPointBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleBulk {
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> DipoleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleBulk {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for CircleBulk {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleBulk {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for CircleBulk {
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> SpacialCurvature {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleBulk {
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

impl GeometricAntiQuotient<CircleBulk> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleCarrierAspect {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<AntiScalar> for CircleWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for CircleWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: CircleBulk) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for CircleWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleWeight {
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

impl GeometricAntiQuotient<CircleBulk> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Dipole {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<AntiScalar> for DipoleBulk {
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulk) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for DipoleBulk {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleBulk {
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for DipoleBulk {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleBulk {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for DipoleBulk {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleBulk {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for DipoleBulk {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleBulk {
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

impl GeometricAntiQuotient<CircleBulk> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleCarrierAspect {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<AntiScalar> for DipoleWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for DipoleWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: CircleBulk) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for DipoleWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: Scalar) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DualNum {
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

impl GeometricAntiQuotient<CircleBulk> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPoint {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Scalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: CircleBulk) -> FlatPointAtInfinity {
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

impl GeometricAntiQuotient<DipoleBulk> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for FlatPointAtOrigin {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: CircleBulk) -> Scalar {
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

impl GeometricAntiQuotient<DipoleBulk> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> RoundPointBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for FlatPointAtOrigin {
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> DipoleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPointAtOrigin {
    type Output = CircleBulk;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> RoundPointAtOrigin {
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

impl GeometricAntiQuotient<CircleBulk> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: CircleBulk) -> FlectorAtInfinity {
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

impl GeometricAntiQuotient<DipoleBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: CircleBulk) -> Horizon {
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

impl GeometricAntiQuotient<DipoleBulk> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Horizon {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: Scalar) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for Infinity {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: CircleBulk) -> Infinity {
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

impl GeometricAntiQuotient<DipoleBulk> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Infinity {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: Scalar) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Line {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Scalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: CircleBulk) -> LineAtInfinity {
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

impl GeometricAntiQuotient<DipoleBulk> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: CircleBulk) -> RoundPointBulk {
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

impl GeometricAntiQuotient<DipoleBulk> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for LineAtOrigin {
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for LineAtOrigin {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> Dipole {
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

impl GeometricAntiQuotient<CircleBulk> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: CircleBulk) -> Origin {
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

impl GeometricAntiQuotient<DipoleBulk> for Origin {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Origin {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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
    type Output = SphereWeight;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> SphereWeight {
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
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> CircleWeight {
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
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> DipoleWeight {
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

impl GeometricAntiQuotient<RoundPointBulk> for Origin {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Origin {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Origin {
    type Output = SphereWeight;

    fn geometric_anti_quotient(self, other: Scalar) -> SphereWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Plane {
    type Output = RoundPointAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: CircleBulk) -> DipoleBulk {
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

impl GeometricAntiQuotient<DipoleBulk> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPointBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for PlaneAtOrigin {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> Circle {
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

impl GeometricAntiQuotient<CircleBulk> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPoint {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Scalar) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for RoundPointAtInfinity {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for RoundPointAtInfinity {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointAtInfinity {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: Scalar) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulk) -> RoundPointAtOrigin {
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

impl GeometricAntiQuotient<DipoleBulk> for RoundPointAtOrigin {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> SpacialCurvature {
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

impl GeometricAntiQuotient<RoundPointBulk> for RoundPointAtOrigin {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointAtOrigin {
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: Scalar) -> SpacialCurvature {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<AntiScalar> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPointBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for RoundPointBulk {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulk) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for RoundPointBulk {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for RoundPointBulk {
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for RoundPointBulk {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPointBulk {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for RoundPointBulk {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for RoundPointBulk {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPointOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointOnOrigin {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: Scalar) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for RoundPointOnOrigin {
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

impl GeometricAntiQuotient<CircleBulk> for Scalar {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: CircleBulk) -> FlatPointAtOrigin {
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

impl GeometricAntiQuotient<DipoleBulk> for Scalar {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Scalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Scalar {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
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
    type Output = CircleBulk;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> CircleBulk {
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
    type Output = DipoleBulk;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> DipoleBulk {
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
    type Output = RoundPointBulk;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> RoundPointBulk {
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
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for Scalar {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Scalar {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Scalar {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> RoundPointAtOrigin {
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

impl GeometricAntiQuotient<AntiScalar> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: AntiScalar) -> SpacialCurvature {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn geometric_anti_quotient(self, other: CircleBulk) -> SpacialCurvature {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for SpacialCurvature {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for SpacialCurvature {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for SpacialCurvature {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for SpacialCurvature {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for SpacialCurvature {
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

impl GeometricAntiQuotient<CircleBulk> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Sphere {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<AntiScalar> for SphereWeight {
    type Output = SphereWeight;

    fn geometric_anti_quotient(self, other: AntiScalar) -> SphereWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleBulk> for SphereWeight {
    type Output = SphereWeight;

    fn geometric_anti_quotient(self, other: CircleBulk) -> SphereWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleCarrierAspect> for SphereWeight {
    type Output = SphereWeight;

    fn geometric_anti_quotient(self, other: CircleCarrierAspect) -> SphereWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleBulk> for SphereWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for SphereWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for SphereWeight {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for SphereWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for SphereWeight {
    type Output = CircleWeight;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> CircleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointBulk> for SphereWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for SphereWeight {
    type Output = DipoleWeight;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> DipoleWeight {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for SphereWeight {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: Scalar) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for SphereWeight {
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

impl GeometricAntiQuotient<CircleBulk> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricAntiQuotient<DipoleBulk> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricAntiQuotient<CircleBulk> for Translator {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: CircleBulk) -> Circle {
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

impl GeometricAntiQuotient<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointBulk> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SpacialCurvature> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for AntiScalar {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: CircleBulk) -> FlatPointAtOrigin {
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

impl GeometricQuotient<DipoleBulk> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: DipoleBulk) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for AntiScalar {
    type Output = Circle;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiScalar {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
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
    type Output = CircleBulk;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> CircleBulk {
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
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: LineAtOrigin) -> DipoleBulk {
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
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> RoundPointBulk {
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
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for AntiScalar {
    type Output = Sphere;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Scalar) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for AntiScalar {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: SpacialCurvature) -> RoundPointAtOrigin {
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

impl GeometricQuotient<CircleBulk> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Circle {
    type Output = Circle;

    fn geometric_quotient(self, other: Scalar) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<AntiScalar> for CircleBulk {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for CircleBulk {
    type Output = Scalar;

    fn geometric_quotient(self, other: CircleBulk) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for CircleBulk {
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: DipoleBulk) -> RoundPointBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleBulk {
    type Output = Translator;

    fn geometric_quotient(self, other: FlatPoint) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleBulk {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleBulk {
    type Output = Transflector;

    fn geometric_quotient(self, other: Line) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleBulk {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleBulk {
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for CircleBulk {
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: RoundPointBulk) -> DipoleBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleBulk {
    type Output = CircleBulk;

    fn geometric_quotient(self, other: Scalar) -> CircleBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for CircleBulk {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleBulk {
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

impl GeometricQuotient<CircleBulk> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn geometric_quotient(self, other: Scalar) -> CircleCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for CircleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<AntiScalar> for CircleWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for CircleWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: CircleBulk) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for CircleWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: Scalar) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleWeight {
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

impl GeometricQuotient<CircleBulk> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Dipole {
    type Output = Dipole;

    fn geometric_quotient(self, other: Scalar) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<AntiScalar> for DipoleBulk {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for DipoleBulk {
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: CircleBulk) -> RoundPointBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleBulk {
    type Output = Transflector;

    fn geometric_quotient(self, other: FlatPoint) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleBulk {
    type Output = Rotor;

    fn geometric_quotient(self, other: LineAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleBulk {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleBulk {
    type Output = Circle;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleBulk {
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: Scalar) -> DipoleBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for DipoleBulk {
    type Output = Dipole;

    fn geometric_quotient(self, other: SpacialCurvature) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleBulk {
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

impl GeometricQuotient<CircleBulk> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn geometric_quotient(self, other: Scalar) -> DipoleCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<AntiScalar> for DipoleWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: AntiScalar) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for DipoleWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: CircleBulk) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for DipoleWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: Scalar) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: AntiScalar) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: Scalar) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DualNum {
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

impl GeometricQuotient<CircleBulk> for FlatPoint {
    type Output = Translator;

    fn geometric_quotient(self, other: CircleBulk) -> Translator {
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

impl GeometricQuotient<DipoleBulk> for FlatPoint {
    type Output = Transflector;

    fn geometric_quotient(self, other: DipoleBulk) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: Scalar) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: CircleBulk) -> LineAtInfinity {
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

impl GeometricQuotient<DipoleBulk> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: DipoleBulk) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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
    type Output = CircleBulk;

    fn geometric_quotient(self, other: AntiScalar) -> CircleBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: CircleBulk) -> AntiScalar {
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

impl GeometricQuotient<DipoleBulk> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: LineAtOrigin) -> RoundPointBulk {
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
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> DipoleBulk {
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

impl GeometricQuotient<RoundPointBulk> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: RoundPointBulk) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for FlatPointAtOrigin {
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: SpacialCurvature) -> SpacialCurvature {
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

impl GeometricQuotient<CircleBulk> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Scalar) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: DipoleBulk) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for Horizon {
    type Output = Infinity;

    fn geometric_quotient(self, other: CircleBulk) -> Infinity {
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

impl GeometricQuotient<DipoleBulk> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: RoundPointBulk) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Horizon {
    type Output = Horizon;

    fn geometric_quotient(self, other: Scalar) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for Infinity {
    type Output = Horizon;

    fn geometric_quotient(self, other: CircleBulk) -> Horizon {
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

impl GeometricQuotient<DipoleBulk> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: DipoleBulk) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Infinity {
    type Output = Infinity;

    fn geometric_quotient(self, other: Scalar) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for Line {
    type Output = Transflector;

    fn geometric_quotient(self, other: CircleBulk) -> Transflector {
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

impl GeometricQuotient<DipoleBulk> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: Scalar) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: CircleBulk) -> FlatPointAtInfinity {
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

impl GeometricQuotient<DipoleBulk> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: RoundPointBulk) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: CircleBulk) -> PlaneAtOrigin {
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

impl GeometricQuotient<DipoleBulk> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: DipoleBulk) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> RoundPointBulk {
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

impl GeometricQuotient<RoundPointBulk> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for LineAtOrigin {
    type Output = Circle;

    fn geometric_quotient(self, other: SpacialCurvature) -> Circle {
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

impl GeometricQuotient<CircleBulk> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: CircleBulk) -> Flector {
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

impl GeometricQuotient<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Scalar) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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
    type Output = SphereWeight;

    fn geometric_quotient(self, other: AntiScalar) -> SphereWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for Origin {
    type Output = SphereWeight;

    fn geometric_quotient(self, other: CircleBulk) -> SphereWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for Origin {
    type Output = SphereWeight;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> SphereWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for Origin {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: DipoleBulk) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Origin {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: LineAtOrigin) -> DipoleWeight {
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
    type Output = CircleWeight;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> CircleWeight {
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

impl GeometricQuotient<RoundPointBulk> for Origin {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: RoundPointBulk) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Origin {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Origin {
    type Output = Origin;

    fn geometric_quotient(self, other: Scalar) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: RoundPointBulk) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: Scalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPointBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: CircleBulk) -> LineAtOrigin {
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

impl GeometricQuotient<DipoleBulk> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> DipoleBulk {
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

impl GeometricQuotient<RoundPointBulk> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: RoundPointBulk) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for PlaneAtOrigin {
    type Output = Dipole;

    fn geometric_quotient(self, other: SpacialCurvature) -> Dipole {
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

impl GeometricQuotient<CircleBulk> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: CircleBulk) -> Flector {
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

impl GeometricQuotient<DipoleBulk> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: DipoleBulk) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: RoundPointBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Scalar) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPoint {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: Scalar) -> RoundPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> RoundPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: AntiScalar) -> SpacialCurvature {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for RoundPointAtOrigin {
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: CircleBulk) -> SpacialCurvature {
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

impl GeometricQuotient<DipoleBulk> for RoundPointAtOrigin {
    type Output = Circle;

    fn geometric_quotient(self, other: DipoleBulk) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for RoundPointAtOrigin {
    type Output = Dipole;

    fn geometric_quotient(self, other: RoundPointBulk) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<AntiScalar> for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for RoundPointBulk {
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: CircleBulk) -> DipoleBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for RoundPointBulk {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPointBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for RoundPointBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPointBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPointBulk {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPointBulk {
    type Output = Rotor;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPointBulk {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for RoundPointBulk {
    type Output = Dipole;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: Scalar) -> RoundPointBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for RoundPointBulk {
    type Output = Circle;

    fn geometric_quotient(self, other: SpacialCurvature) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPointBulk {
    type Output = Transflector;

    fn geometric_quotient(self, other: Translator) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPointOnOrigin {
    type Output = Sphere;

    fn geometric_quotient(self, other: AntiScalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn geometric_quotient(self, other: Scalar) -> RoundPointOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for RoundPointOnOrigin {
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

impl GeometricQuotient<CircleBulk> for Scalar {
    type Output = CircleBulk;

    fn geometric_quotient(self, other: CircleBulk) -> CircleBulk {
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

impl GeometricQuotient<DipoleBulk> for Scalar {
    type Output = DipoleBulk;

    fn geometric_quotient(self, other: DipoleBulk) -> DipoleBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Scalar {
    type Output = DipoleCarrierAspect;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Scalar {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
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

impl GeometricQuotient<RoundPointBulk> for Scalar {
    type Output = RoundPointBulk;

    fn geometric_quotient(self, other: RoundPointBulk) -> RoundPointBulk {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Scalar {
    type Output = RoundPointOnOrigin;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: Scalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Scalar {
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: SpacialCurvature) -> SpacialCurvature {
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

impl GeometricQuotient<AntiScalar> for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for SpacialCurvature {
    type Output = Dipole;

    fn geometric_quotient(self, other: DipoleBulk) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> SpacialCurvature {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for SpacialCurvature {
    type Output = Circle;

    fn geometric_quotient(self, other: LineAtOrigin) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for SpacialCurvature {
    type Output = Dipole;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for SpacialCurvature {
    type Output = Circle;

    fn geometric_quotient(self, other: RoundPointBulk) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn geometric_quotient(self, other: Scalar) -> SpacialCurvature {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for SpacialCurvature {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Sphere {
    type Output = Sphere;

    fn geometric_quotient(self, other: Scalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<AntiScalar> for SphereWeight {
    type Output = Origin;

    fn geometric_quotient(self, other: AntiScalar) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleBulk> for SphereWeight {
    type Output = Origin;

    fn geometric_quotient(self, other: CircleBulk) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleCarrierAspect> for SphereWeight {
    type Output = Origin;

    fn geometric_quotient(self, other: CircleCarrierAspect) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleBulk> for SphereWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: DipoleBulk) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for SphereWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for SphereWeight {
    type Output = SphereWeight;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> SphereWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for SphereWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: LineAtOrigin) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for SphereWeight {
    type Output = DipoleWeight;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> DipoleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointBulk> for SphereWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: RoundPointBulk) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for SphereWeight {
    type Output = CircleWeight;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> CircleWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for SphereWeight {
    type Output = SphereWeight;

    fn geometric_quotient(self, other: Scalar) -> SphereWeight {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for SphereWeight {
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

impl GeometricQuotient<CircleBulk> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleBulk) -> MultiVector {
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

impl GeometricQuotient<DipoleBulk> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: DipoleBulk) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Transflector {
    type Output = Transflector;

    fn geometric_quotient(self, other: Scalar) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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

impl GeometricQuotient<CircleBulk> for Translator {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: CircleBulk) -> FlatPoint {
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

impl GeometricQuotient<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleBulk) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleCarrierAspect) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
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

impl GeometricQuotient<RoundPointBulk> for Translator {
    type Output = Transflector;

    fn geometric_quotient(self, other: RoundPointBulk) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Scalar) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SpacialCurvature> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SpacialCurvature) -> MultiVector {
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
