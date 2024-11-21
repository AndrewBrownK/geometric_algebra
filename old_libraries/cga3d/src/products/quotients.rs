// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::characteristics::AntiInverse;
use crate::characteristics::Inverse;
use crate::products::geometric::*;
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiFlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for AntiFlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiFlatPointAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: Translator) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiLineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiLineAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiLineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiLineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiLineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiLineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiLineAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiLineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for AntiLineAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiPlane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiPlane {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiPlane {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiPlane {
    type Output = AntiPlane;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiPlane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiPlane {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiPlane {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiPlane {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiPlane {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: Scalar) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiPlane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiPlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiPlaneAtOrigin {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiPlaneAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for AntiPlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiScalar {
    type Output = AntiCircleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiScalar {
    type Output = AntiDipoleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiScalar {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiScalar {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiScalar {
    type Output = AntiPlane;

    fn geometric_anti_quotient(self, other: AntiPlane) -> AntiPlane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiScalar {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiScalar {
    type Output = AntiSphereOnOrigin;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiScalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Circle) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiScalar {
    type Output = CircleAligningOrigin;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiScalar {
    type Output = CircleAtInfinity;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiScalar {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiScalar {
    type Output = CircleOnOrigin;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiScalar {
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiScalar {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Dilator) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiScalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Dipole) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiScalar {
    type Output = DipoleAligningOrigin;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiScalar {
    type Output = DipoleAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiScalar {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiScalar {
    type Output = DipoleOnOrigin;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiScalar {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiScalar {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
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

impl GeometricAntiQuotient<SphereAtOrigin> for AntiScalar {
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiScalar {
    type Output = SphereOnOrigin;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> SphereOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Circle {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleAligningOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleAligningOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Dilator {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Dilator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Dipole {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleAligningOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleAtInfinity {
    type Output = CircleAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleAligningOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: Scalar) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlatPoint {
    type Output = FlatPoint;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlatPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPoint {
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlane) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for FlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for FlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for FlatPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for FlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for FlatPointAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for FlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for FlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlane) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlane) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Horizon {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Sphere) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for Horizon {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Horizon {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> Dilator {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> FlatPointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlane) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Infinity {
    type Output = Infinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Infinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Infinity {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Infinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Infinity {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dilator {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Line {
    type Output = Line;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Line {
    type Output = DipoleAtInfinity;

    fn geometric_anti_quotient(self, other: Scalar) -> DipoleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for LineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for LineAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for LineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiLineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for LineAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Motor {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> NullSphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for NullSphereAtOrigin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for NullSphereAtOrigin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for NullSphereAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Plane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for NullSphereAtOrigin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: Scalar) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for NullSphereAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Sphere) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for NullSphereAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Origin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Origin {
    type Output = NullSphereAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> NullSphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = NullSphereAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> NullSphereAtOrigin {
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
    type Output = NullCircleAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> NullCircleAtOrigin {
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
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> NullDipoleAtOrigin {
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
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for Origin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Origin {
    type Output = NullSphereAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> NullSphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> NullDipoleAtOrigin {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Plane {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for Plane {
    type Output = AntiPlane;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiPlane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for Plane {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Sphere) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for Plane {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Plane {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> Dilator {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for PlaneAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> AntiLineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for PlaneAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Sphere) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for PlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for PlaneAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> Dilator {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for RoundPoint {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for RoundPoint {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPoint {
    type Output = RoundPoint;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPoint {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for RoundPoint {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPoint {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dilator {
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

impl GeometricAntiQuotient<SphereAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for RoundPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for RoundPointAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for RoundPointAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for RoundPointAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> SphereAtOrigin {
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
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> CircleAtOrigin {
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
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> DipoleAtOrigin {
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
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> SphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Scalar {
    type Output = CircleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> CircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Scalar {
    type Output = DipoleOnOrigin;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Scalar {
    type Output = FlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Scalar {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: AntiPlane) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Scalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Scalar {
    type Output = SphereOnOrigin;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> SphereOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Scalar {
    type Output = Dipole;

    fn geometric_anti_quotient(self, other: Circle) -> Dipole {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Scalar {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Scalar {
    type Output = DipoleAtInfinity;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> DipoleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Scalar {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Scalar {
    type Output = AntiCircleOnOrigin;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Scalar {
    type Output = DipoleAligningOrigin;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> DipoleAligningOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Scalar {
    type Output = Circle;

    fn geometric_anti_quotient(self, other: Dipole) -> Circle {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Scalar {
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Scalar {
    type Output = CircleAtInfinity;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> CircleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Scalar {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Scalar {
    type Output = AntiDipoleOnOrigin;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Scalar {
    type Output = CircleAligningOrigin;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> CircleAligningOrigin {
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
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: FlatPoint) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for Scalar {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> AntiFlatPointAtOrigin {
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
    type Output = DipoleAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> DipoleAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Scalar {
    type Output = AntiLineAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> AntiLineAtOrigin {
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
    type Output = AntiPlane;

    fn geometric_anti_quotient(self, other: Plane) -> AntiPlane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Scalar {
    type Output = AntiPlaneAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> AntiPlaneAtOrigin {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Scalar {
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> SphereAtOrigin {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Scalar {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Scalar {
    type Output = AntiSphereOnOrigin;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> AntiSphereOnOrigin {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Sphere {
    type Output = Sphere;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Sphere {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Plane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Sphere {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Dilator {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Sphere) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for Sphere {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Sphere {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> Dilator {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> SphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for SphereAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for SphereAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> SphereAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for SphereAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> DipoleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Plane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for SphereAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> CircleAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> RoundPointAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Sphere) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> SphereOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPoint> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<FlatPointAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Plane) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPoint> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<RoundPointAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Scalar> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn geometric_anti_quotient(self, other: Scalar) -> AntiSphereOnOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Sphere> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: Sphere) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereAtOrigin> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> Dilator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<AntiCircleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiDipoleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiFlatPointAtOrigin> for Translator {
    type Output = CircleOrthogonalOrigin;

    fn geometric_anti_quotient(self, other: AntiFlatPointAtOrigin) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiLineAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlane> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiPlaneAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Translator {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiSphereOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Circle> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Circle) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAligningOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<CircleOrthogonalOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dilator> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Dipole> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAligningOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DipoleOrthogonalOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricAntiQuotient<SphereAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<SphereOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> CircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn geometric_quotient(self, other: Scalar) -> AntiCircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn geometric_quotient(self, other: Scalar) -> AntiDipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> AntiLineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiFlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: CircleAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiFlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiFlatPointAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiFlatPointAtOrigin {
    type Output = Translator;

    fn geometric_quotient(self, other: FlatPoint) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiFlatPointAtOrigin {
    type Output = Transflector;

    fn geometric_quotient(self, other: Line) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiFlatPointAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiFlatPointAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> SphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> AntiFlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for AntiFlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiFlatPointAtOrigin {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: Translator) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiLineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiLineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiLineAtOrigin {
    type Output = Transflector;

    fn geometric_quotient(self, other: FlatPoint) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiLineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiLineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiLineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: LineAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiLineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiLineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiLineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiLineAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> AntiLineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for AntiLineAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiLineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiPlane {
    type Output = Plane;

    fn geometric_quotient(self, other: AntiScalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiPlane {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiPlane {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiPlane {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiPlane {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiPlane {
    type Output = Motor;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiPlane {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiPlane {
    type Output = AntiPlane;

    fn geometric_quotient(self, other: Scalar) -> AntiPlane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiPlane {
    type Output = Dilator;

    fn geometric_quotient(self, other: Sphere) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for AntiPlane {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiPlane {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiPlane {
    type Output = Transflector;

    fn geometric_quotient(self, other: Translator) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> AntiLineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiPlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiPlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiPlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiPlaneAtOrigin {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiPlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Rotor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> AntiPlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiPlaneAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: Sphere) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for AntiPlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiPlaneAtOrigin {
    type Output = Transflector;

    fn geometric_quotient(self, other: Translator) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for AntiScalar {
    type Output = CircleOnOrigin;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> CircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiScalar {
    type Output = DipoleOnOrigin;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiScalar {
    type Output = FlatPointAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> FlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiScalar {
    type Output = Plane;

    fn geometric_quotient(self, other: AntiPlane) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: AntiScalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiScalar {
    type Output = SphereOnOrigin;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> SphereOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiScalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Circle) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiScalar {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> DipoleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiScalar {
    type Output = DipoleAtInfinity;

    fn geometric_quotient(self, other: CircleAtInfinity) -> DipoleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiScalar {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: CircleAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiScalar {
    type Output = AntiCircleOnOrigin;

    fn geometric_quotient(self, other: CircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiScalar {
    type Output = DipoleAligningOrigin;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> DipoleAligningOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiScalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiScalar {
    type Output = Circle;

    fn geometric_quotient(self, other: Dipole) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiScalar {
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiScalar {
    type Output = CircleAtInfinity;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> CircleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiScalar {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiScalar {
    type Output = AntiDipoleOnOrigin;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiScalar {
    type Output = CircleAligningOrigin;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> CircleAligningOrigin {
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
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: FlatPoint) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiScalar {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> AntiFlatPointAtOrigin {
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
    type Output = DipoleAtInfinity;

    fn geometric_quotient(self, other: Line) -> DipoleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiScalar {
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> AntiLineAtOrigin {
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
    type Output = AntiPlane;

    fn geometric_quotient(self, other: Plane) -> AntiPlane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiScalar {
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> AntiPlaneAtOrigin {
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

impl GeometricQuotient<RoundPointAtOrigin> for AntiScalar {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> SphereAtOrigin {
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

impl GeometricQuotient<SphereAtOrigin> for AntiScalar {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiScalar {
    type Output = AntiSphereOnOrigin;

    fn geometric_quotient(self, other: SphereOnOrigin) -> AntiSphereOnOrigin {
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

impl GeometricQuotient<AntiCircleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> SphereOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: Plane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn geometric_quotient(self, other: Scalar) -> AntiSphereOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: Sphere) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for AntiSphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Circle {
    type Output = Dipole;

    fn geometric_quotient(self, other: AntiScalar) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Circle {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn geometric_quotient(self, other: Scalar) -> CircleAligningOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> CircleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> AntiCircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn geometric_quotient(self, other: Scalar) -> CircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleAligningOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: Scalar) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Dilator {
    type Output = Dilator;

    fn geometric_quotient(self, other: Scalar) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Dilator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Dipole {
    type Output = Circle;

    fn geometric_quotient(self, other: AntiScalar) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Dipole {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn geometric_quotient(self, other: Scalar) -> DipoleAligningOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleAtInfinity {
    type Output = CircleAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> CircleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> DipoleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for DipoleOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for DipoleOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> AntiDipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn geometric_quotient(self, other: Scalar) -> DipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> CircleAligningOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_quotient(self, other: Scalar) -> DipoleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: AntiScalar) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: Scalar) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for FlatPoint {
    type Output = Translator;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for FlatPoint {
    type Output = Transflector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlatPoint {
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for FlatPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: Translator) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for FlatPointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for FlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> AntiFlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for FlatPointAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for FlatPointAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for FlatPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
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
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> AntiLineAtOrigin {
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

impl GeometricQuotient<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
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

impl GeometricQuotient<SphereAtOrigin> for FlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: Translator) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Horizon {
    type Output = Infinity;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Horizon {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: AntiPlane) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Horizon {
    type Output = Infinity;

    fn geometric_quotient(self, other: AntiScalar) -> Infinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Horizon {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Horizon {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Horizon {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dilator {
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

impl GeometricQuotient<SphereAtOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Infinity {
    type Output = Horizon;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: AntiPlane) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Infinity {
    type Output = Horizon;

    fn geometric_quotient(self, other: AntiScalar) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Infinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: CircleAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Infinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: Sphere) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for Infinity {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Infinity {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereOnOrigin) -> Dilator {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Line {
    type Output = Transflector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Line {
    type Output = DipoleAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> DipoleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: AntiPlane) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn geometric_quotient(self, other: AntiScalar) -> FlatPointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: CircleAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for LineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> AntiLineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
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

impl GeometricQuotient<RoundPointAtOrigin> for LineAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> DipoleAtOrigin {
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

impl GeometricQuotient<SphereAtOrigin> for LineAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for NullSphereAtOrigin {
    type Output = Origin;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for NullSphereAtOrigin {
    type Output = Origin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for NullSphereAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for NullSphereAtOrigin {
    type Output = Origin;

    fn geometric_quotient(self, other: AntiScalar) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: CircleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> NullSphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> NullSphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for NullSphereAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for NullSphereAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> NullSphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: SphereOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Origin {
    type Output = NullSphereAtOrigin;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> NullSphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Origin {
    type Output = NullSphereAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> NullSphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> NullCircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Origin {
    type Output = NullSphereAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> NullSphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: CircleOnOrigin) -> NullDipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Origin {
    type Output = Origin;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = NullDipoleAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> NullDipoleAtOrigin {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: Plane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> NullCircleAtOrigin {
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

impl GeometricQuotient<RoundPointAtOrigin> for Origin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: Sphere) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for Origin {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn geometric_quotient(self, other: SphereOnOrigin) -> NullCircleAtOrigin {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: AntiPlane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Plane {
    type Output = AntiPlane;

    fn geometric_quotient(self, other: AntiScalar) -> AntiPlane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Plane {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Plane {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dilator {
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

impl GeometricQuotient<SphereAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for PlaneAtOrigin {
    type Output = Motor;

    fn geometric_quotient(self, other: AntiPlane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> AntiPlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for PlaneAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> AntiLineAtOrigin {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> CircleAtOrigin {
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

impl GeometricQuotient<SphereAtOrigin> for PlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPoint {
    type Output = Sphere;

    fn geometric_quotient(self, other: AntiScalar) -> Sphere {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: Plane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPoint {
    type Output = Dilator;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> Dilator {
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

impl GeometricQuotient<RoundPointAtOrigin> for RoundPoint {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: Sphere) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for RoundPoint {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for RoundPoint {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereOnOrigin) -> Dilator {
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

impl GeometricQuotient<AntiCircleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> SphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for RoundPointAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for RoundPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> SphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> DipoleAtOrigin {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: Plane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> CircleAtOrigin {
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

impl GeometricQuotient<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: Sphere) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for RoundPointAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for RoundPointAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: SphereOnOrigin) -> Dilator {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Scalar {
    type Output = AntiCircleOnOrigin;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> AntiCircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Scalar {
    type Output = AntiDipoleOnOrigin;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> AntiDipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Scalar {
    type Output = AntiFlatPointAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Scalar {
    type Output = AntiLineAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> AntiLineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Scalar {
    type Output = AntiPlane;

    fn geometric_quotient(self, other: AntiPlane) -> AntiPlane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Scalar {
    type Output = AntiPlaneAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: AntiScalar) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Scalar {
    type Output = AntiSphereOnOrigin;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Scalar {
    type Output = Circle;

    fn geometric_quotient(self, other: Circle) -> Circle {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Scalar {
    type Output = CircleAligningOrigin;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Scalar {
    type Output = CircleAtInfinity;

    fn geometric_quotient(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Scalar {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Scalar {
    type Output = CircleOnOrigin;

    fn geometric_quotient(self, other: CircleOnOrigin) -> CircleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Scalar {
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Scalar {
    type Output = Dilator;

    fn geometric_quotient(self, other: Dilator) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Scalar {
    type Output = Dipole;

    fn geometric_quotient(self, other: Dipole) -> Dipole {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Scalar {
    type Output = DipoleAligningOrigin;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> DipoleAligningOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Scalar {
    type Output = DipoleAtInfinity;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Scalar {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Scalar {
    type Output = DipoleOnOrigin;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> DipoleOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Scalar {
    type Output = DipoleOrthogonalOrigin;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
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

impl GeometricQuotient<RoundPointAtOrigin> for Scalar {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
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

impl GeometricQuotient<SphereAtOrigin> for Scalar {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Scalar {
    type Output = SphereOnOrigin;

    fn geometric_quotient(self, other: SphereOnOrigin) -> SphereOnOrigin {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Sphere {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Sphere {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Sphere {
    type Output = RoundPoint;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Sphere {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for Sphere {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dilator {
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

impl GeometricQuotient<SphereAtOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Sphere {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for SphereAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for SphereAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> RoundPointAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> SphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for SphereAtOrigin {
    type Output = CircleAtOrigin;

    fn geometric_quotient(self, other: LineAtOrigin) -> CircleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for SphereAtOrigin {
    type Output = DipoleAtOrigin;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> DipoleAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for SphereAtOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> SphereAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for SphereAtOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiPlane) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn geometric_quotient(self, other: AntiScalar) -> AntiSphereOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPoint> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPoint) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PlaneAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPoint> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPoint) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<RoundPointAtOrigin> for SphereOnOrigin {
    type Output = Dilator;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> Dilator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn geometric_quotient(self, other: Scalar) -> SphereOnOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Sphere> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Sphere) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereAtOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for SphereOnOrigin {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiCircleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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

impl GeometricQuotient<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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

impl GeometricQuotient<AntiCircleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiDipoleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiFlatPointAtOrigin> for Translator {
    type Output = FlatPoint;

    fn geometric_quotient(self, other: AntiFlatPointAtOrigin) -> FlatPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiLineAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiLineAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlane> for Translator {
    type Output = Transflector;

    fn geometric_quotient(self, other: AntiPlane) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiPlaneAtOrigin> for Translator {
    type Output = Transflector;

    fn geometric_quotient(self, other: AntiPlaneAtOrigin) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiScalar> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<AntiSphereOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Circle> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Circle) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAligningOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<CircleOrthogonalOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dilator> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dilator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Dipole> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Dipole) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAligningOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAligningOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOnOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DipoleOrthogonalOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DipoleOrthogonalOrigin) -> MultiVector {
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
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: FlatPoint) -> CircleOrthogonalOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlatPointAtOrigin> for Translator {
    type Output = CircleOrthogonalOrigin;

    fn geometric_quotient(self, other: FlatPointAtOrigin) -> CircleOrthogonalOrigin {
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

impl GeometricQuotient<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: RoundPointAtOrigin) -> MultiVector {
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

impl GeometricQuotient<SphereAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereAtOrigin) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<SphereOnOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: SphereOnOrigin) -> MultiVector {
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
