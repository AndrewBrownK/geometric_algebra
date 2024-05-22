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

impl Contraction<AntiCircleOnOrigin> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiLineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for AntiCircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: AntiPlane) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for AntiCircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for AntiCircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for AntiCircleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Infinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for AntiCircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: RoundPoint) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for AntiDipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: AntiCircleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for AntiDipoleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: AntiPlane) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for AntiDipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Dipole) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for AntiDipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleAtInfinity) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for AntiDipoleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for AntiDipoleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for AntiDipoleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: Infinity) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for AntiDipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for AntiDipoleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: RoundPoint) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: AntiCircleOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for AntiFlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: AntiPlane) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Dipole) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAtInfinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for AntiFlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for AntiFlatPointAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: RoundPoint) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for AntiLineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for AntiLineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiLineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for AntiLineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: AntiPlane) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for AntiLineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for AntiLineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for AntiLineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for AntiLineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for AntiLineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: RoundPoint) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for AntiPlane {
    type Output = Scalar;

    fn contraction(self, other: AntiPlane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for AntiPlane {
    type Output = Scalar;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for AntiPlane {
    type Output = Scalar;

    fn contraction(self, other: AntiSphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for AntiPlane {
    type Output = Scalar;

    fn contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for AntiPlane {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for AntiPlane {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for AntiPlane {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for AntiPlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiPlane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiSphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for AntiPlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for AntiSphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiPlane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for AntiSphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiSphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for AntiSphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for AntiSphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for AntiSphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: AntiCircleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: AntiLineAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Circle {
    type Output = Dipole;

    fn contraction(self, other: AntiPlane) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Circle {
    type Output = Dipole;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Circle {
    type Output = Dipole;

    fn contraction(self, other: AntiSphereOnOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Circle {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Circle {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Circle {
    type Output = AntiPlane;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Circle {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Circle {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: Infinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Circle {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Circle {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: LineAtOrigin) -> Scalar {
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

impl Contraction<NullCircleAtOrigin> for Circle {
    type Output = Scalar;

    fn contraction(self, other: NullCircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Circle {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Circle {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: Origin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Circle {
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Circle {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Circle {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiCircleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for CircleAligningOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for CircleAligningOrigin {
    type Output = Dipole;

    fn contraction(self, other: AntiPlane) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for CircleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> DipoleAligningOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for CircleAligningOrigin {
    type Output = Dipole;

    fn contraction(self, other: AntiSphereOnOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for CircleAligningOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for CircleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for CircleAligningOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for CircleAligningOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for CircleAligningOrigin {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: Infinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for CircleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: NullCircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for CircleAligningOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for CircleAligningOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: Origin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleAligningOrigin {
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for CircleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: AntiCircleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for CircleAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: AntiLineAtOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: AntiPlane) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: AntiPlaneAtOrigin) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for CircleAtInfinity {
    type Output = Dipole;

    fn contraction(self, other: AntiSphereOnOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for CircleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: DipoleAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for CircleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for CircleAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for CircleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for CircleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for CircleAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: FlatPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for CircleAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for CircleAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: Infinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: NullCircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for CircleAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for CircleAtInfinity {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: Origin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleAtInfinity {
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for CircleAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiCircleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for CircleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for CircleAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: AntiPlane) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> DipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for CircleAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for CircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for CircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for CircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for CircleAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for CircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for CircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for CircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: Infinity) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for CircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: NullCircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for CircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for CircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: Origin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: RoundPoint) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for CircleOnOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiCircleOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for CircleOnOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiLineAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for CircleOnOrigin {
    type Output = Dipole;

    fn contraction(self, other: AntiPlane) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for CircleOnOrigin {
    type Output = DipoleOnOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> DipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for CircleOnOrigin {
    type Output = DipoleOnOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> DipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for CircleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for CircleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for CircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for CircleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for CircleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for CircleOnOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for CircleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for CircleOnOrigin {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: Infinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for CircleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for CircleOnOrigin {
    type Output = Origin;

    fn contraction(self, other: NullDipoleAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for CircleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: Origin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleOnOrigin {
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiCircleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiLineAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for CircleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: AntiPlane) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for CircleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for CircleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for CircleOrthogonalOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: Infinity) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: NullCircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for CircleOrthogonalOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: Origin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: RoundPoint) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: AntiLineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: CircleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: CircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for Dilator {
    type Output = Infinity;

    fn contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: NullCircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Dilator {
    type Output = Origin;

    fn contraction(self, other: NullSphereAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Dilator {
    type Output = AntiPlane;

    fn contraction(self, other: Plane) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Dilator {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: PlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Dilator {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Dilator {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Dilator {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: SphereOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Dilator {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: AntiLineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: AntiPlane) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: AntiPlaneAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: AntiSphereOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Dipole {
    type Output = AntiPlane;

    fn contraction(self, other: Infinity) -> AntiPlane {
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

impl Contraction<NullDipoleAtOrigin> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: NullDipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Dipole {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Origin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Dipole {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for DipoleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiPlane) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for DipoleAligningOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for DipoleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiSphereOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for DipoleAligningOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: Infinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for DipoleAligningOrigin {
    type Output = Scalar;

    fn contraction(self, other: NullDipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for DipoleAligningOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Origin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleAligningOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: AntiLineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for DipoleAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: AntiPlane) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for DipoleAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: AntiPlaneAtOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for DipoleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: AntiSphereOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for DipoleAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: Infinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: NullDipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for DipoleAtInfinity {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Origin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for DipoleAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiPlane) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for DipoleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for DipoleAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiSphereOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for DipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Infinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for DipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: NullDipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for DipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Origin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for DipoleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiPlane) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for DipoleOnOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiSphereOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for DipoleOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for DipoleOnOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: Infinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for DipoleOnOrigin {
    type Output = Origin;

    fn contraction(self, other: Origin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiLineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for DipoleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiPlane) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiPlaneAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiSphereOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for DipoleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Infinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = Scalar;

    fn contraction(self, other: NullDipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for DipoleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Origin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleOrthogonalOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for FlatPoint {
    type Output = Infinity;

    fn contraction(self, other: AntiPlane) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for FlatPoint {
    type Output = Infinity;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn contraction(self, other: AntiSphereOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for FlatPoint {
    type Output = Infinity;

    fn contraction(self, other: Infinity) -> Infinity {
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

impl Contraction<NullDipoleAtOrigin> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: NullDipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for FlatPoint {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Origin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for FlatPoint {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: AntiCircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for FlatPointAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: AntiPlane) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for FlatPointAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for FlatPointAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: AntiSphereOnOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: NullDipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for FlatPointAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Origin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for FlatPointAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: RoundPoint) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for FlatPointAtOrigin {
    type Output = Infinity;

    fn contraction(self, other: AntiPlane) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for FlatPointAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiSphereOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for FlatPointAtOrigin {
    type Output = Infinity;

    fn contraction(self, other: Infinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for FlatPointAtOrigin {
    type Output = Origin;

    fn contraction(self, other: Origin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: RoundPoint) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: AntiDipoleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Flector {
    type Output = Infinity;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Flector {
    type Output = FlatPoint;

    fn contraction(self, other: AntiLineAtOrigin) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Flector {
    type Output = AntiPlane;

    fn contraction(self, other: CircleAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Flector {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Flector {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Flector {
    type Output = AntiPlane;

    fn contraction(self, other: Line) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Flector {
    type Output = Infinity;

    fn contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Flector {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
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

impl Contraction<NullCircleAtOrigin> for Flector {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Flector {
    type Output = Scalar;

    fn contraction(self, other: NullSphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Flector {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Flector {
    type Output = Scalar;

    fn contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Flector {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Flector {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Flector {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for FlectorAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: AntiDipoleOnOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for FlectorAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: AntiLineAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for FlectorAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: Circle) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for FlectorAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for FlectorAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for FlectorAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for FlectorAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: CircleOrthogonalOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for FlectorAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: Line) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for FlectorAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for FlectorAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: NullSphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for FlectorAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for FlectorAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for FlectorAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Horizon {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: AntiCircleOnOrigin) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Horizon {
    type Output = AntiPlane;

    fn contraction(self, other: AntiDipoleOnOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Horizon {
    type Output = Infinity;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Horizon {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: AntiLineAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Horizon {
    type Output = LineAtInfinity;

    fn contraction(self, other: AntiPlane) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn contraction(self, other: AntiPlaneAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Horizon {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Horizon {
    type Output = AntiPlane;

    fn contraction(self, other: Circle) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Horizon {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Horizon {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Horizon {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Horizon {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Horizon {
    type Output = AntiPlane;

    fn contraction(self, other: CircleOrthogonalOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Horizon {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: Dipole) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Horizon {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Horizon {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Horizon {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Horizon {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Horizon {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Horizon {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Horizon {
    type Output = Infinity;

    fn contraction(self, other: Line) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for Horizon {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Horizon {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Horizon {
    type Output = Scalar;

    fn contraction(self, other: NullSphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Horizon {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: Origin) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Horizon {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: RoundPoint) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Horizon {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Horizon {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Horizon {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Horizon {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Horizon {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Infinity {
    type Output = Scalar;

    fn contraction(self, other: AntiSphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Infinity {
    type Output = Scalar;

    fn contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Infinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Infinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Infinity {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: AntiCircleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Line {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Line {
    type Output = Infinity;

    fn contraction(self, other: AntiLineAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Line {
    type Output = FlatPoint;

    fn contraction(self, other: AntiPlane) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Line {
    type Output = FlatPoint;

    fn contraction(self, other: AntiPlaneAtOrigin) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Line {
    type Output = Dipole;

    fn contraction(self, other: AntiSphereOnOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Line {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Line {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Line {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Line {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Line {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Line {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Line {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Line {
    type Output = AntiPlane;

    fn contraction(self, other: DipoleAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Line {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Line {
    type Output = Infinity;

    fn contraction(self, other: FlatPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Line {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Line {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Line {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: Infinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Line {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Line {
    type Output = Scalar;

    fn contraction(self, other: LineAtOrigin) -> Scalar {
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

impl Contraction<NullCircleAtOrigin> for Line {
    type Output = Scalar;

    fn contraction(self, other: NullCircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Line {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Line {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: Origin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Line {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Line {
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Line {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Line {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Line {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for LineAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: AntiCircleOnOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for LineAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: AntiLineAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: AntiPlane) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: AntiPlaneAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for LineAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: AntiSphereOnOrigin) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for LineAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: Dipole) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for LineAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for LineAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: DipoleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for LineAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for LineAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for LineAtInfinity {
    type Output = AntiPlane;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for LineAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: NullCircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for LineAtInfinity {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for LineAtInfinity {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: Origin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for LineAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: RoundPoint) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for LineAtInfinity {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for LineAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiCircleOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for LineAtOrigin {
    type Output = FlatPoint;

    fn contraction(self, other: AntiPlane) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for LineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for LineAtOrigin {
    type Output = DipoleOnOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> DipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for LineAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for LineAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for LineAtOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: DipoleAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for LineAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for LineAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for LineAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for LineAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for LineAtOrigin {
    type Output = Infinity;

    fn contraction(self, other: FlatPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for LineAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for LineAtOrigin {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: Infinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for LineAtOrigin {
    type Output = Origin;

    fn contraction(self, other: NullDipoleAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for LineAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: Origin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for LineAtOrigin {
    type Output = DipoleAligningOrigin;

    fn contraction(self, other: RoundPoint) -> DipoleAligningOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for LineAtOrigin {
    type Output = DipoleAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Motor {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: AntiLineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Motor {
    type Output = Flector;

    fn contraction(self, other: AntiPlane) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Motor {
    type Output = Flector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: CircleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: CircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for Motor {
    type Output = Infinity;

    fn contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Motor {
    type Output = FlectorAtInfinity;

    fn contraction(self, other: Infinity) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Motor {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: LineAtOrigin) -> MultiVector {
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

impl Contraction<NullCircleAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: NullCircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Motor {
    type Output = Origin;

    fn contraction(self, other: NullSphereAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Motor {
    type Output = AntiPlane;

    fn contraction(self, other: Plane) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Motor {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: PlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Motor {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Motor {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Motor {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: SphereOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: AntiLineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: LineAtOrigin) -> MultiVector {
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

impl Contraction<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: NullCircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: NullSphereAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Plane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: SphereAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: SphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for NullCircleAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiCircleOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for NullCircleAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiLineAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for NullCircleAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: AntiPlane) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for NullCircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for NullCircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for NullCircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for NullCircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for NullCircleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Dipole) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for NullCircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for NullCircleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleAtInfinity) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for NullCircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for NullCircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for NullCircleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for NullCircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: Infinity) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for NullCircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for NullCircleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for NullCircleAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: RoundPoint) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for NullCircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for NullDipoleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: AntiPlane) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for NullDipoleAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiSphereOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for NullDipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for NullDipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAligningOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for NullDipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for NullDipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for NullDipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for NullDipoleAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for NullDipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: Infinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for NullDipoleAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: RoundPoint) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for NullDipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: AntiCircleOnOrigin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for NullSphereAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for NullSphereAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for NullSphereAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for NullSphereAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn contraction(self, other: AntiPlane) -> AntiDipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> NullCircleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for NullSphereAtOrigin {
    type Output = NullCircleAtOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> NullCircleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for NullSphereAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Circle) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for NullSphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for NullSphereAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleAtInfinity) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for NullSphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleOrthogonalOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for NullSphereAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: Dipole) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for NullSphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for NullSphereAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: DipoleAtInfinity) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for NullSphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for NullSphereAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for NullSphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for NullSphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for NullSphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for NullSphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: Infinity) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for NullSphereAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: Line) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for NullSphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtInfinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for NullSphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for NullSphereAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn contraction(self, other: RoundPoint) -> AntiDipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for NullSphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for NullSphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for NullSphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Origin {
    type Output = Scalar;

    fn contraction(self, other: AntiPlane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Origin {
    type Output = Scalar;

    fn contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Origin {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Origin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Origin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Plane {
    type Output = Dipole;

    fn contraction(self, other: AntiCircleOnOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: AntiDipoleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Plane {
    type Output = Infinity;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Plane {
    type Output = FlatPoint;

    fn contraction(self, other: AntiLineAtOrigin) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Plane {
    type Output = Line;

    fn contraction(self, other: AntiPlane) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Plane {
    type Output = Line;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Plane {
    type Output = Circle;

    fn contraction(self, other: AntiSphereOnOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Plane {
    type Output = AntiPlane;

    fn contraction(self, other: CircleAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Plane {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: CircleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Plane {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Plane {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Plane {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Plane {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Plane {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Plane {
    type Output = Dipole;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Plane {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: FlatPoint) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Plane {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Plane {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Plane {
    type Output = LineAtInfinity;

    fn contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Plane {
    type Output = AntiPlane;

    fn contraction(self, other: Line) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Plane {
    type Output = Infinity;

    fn contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Plane {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
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

impl Contraction<NullCircleAtOrigin> for Plane {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Plane {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Plane {
    type Output = Scalar;

    fn contraction(self, other: NullSphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Plane {
    type Output = AntiDipoleOnOrigin;

    fn contraction(self, other: Origin) -> AntiDipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Plane {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Plane {
    type Output = Scalar;

    fn contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Plane {
    type Output = Circle;

    fn contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Plane {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Plane {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Plane {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Plane {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Plane {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for PlaneAtOrigin {
    type Output = DipoleOnOrigin;

    fn contraction(self, other: AntiCircleOnOrigin) -> DipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for PlaneAtOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for PlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for PlaneAtOrigin {
    type Output = Line;

    fn contraction(self, other: AntiPlane) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for PlaneAtOrigin {
    type Output = CircleOnOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> CircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for PlaneAtOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: CircleAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for PlaneAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: CircleAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for PlaneAtOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: CircleOrthogonalOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for PlaneAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for PlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> DipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for PlaneAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for PlaneAtOrigin {
    type Output = DipoleAligningOrigin;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> DipoleAligningOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for PlaneAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: FlatPoint) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for PlaneAtOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: Line) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = Infinity;

    fn contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for PlaneAtOrigin {
    type Output = Origin;

    fn contraction(self, other: NullCircleAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for PlaneAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for PlaneAtOrigin {
    type Output = NullCircleAtOrigin;

    fn contraction(self, other: Origin) -> NullCircleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for PlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for PlaneAtOrigin {
    type Output = CircleAligningOrigin;

    fn contraction(self, other: RoundPoint) -> CircleAligningOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> CircleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for PlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Rotor {
    type Output = DipoleOnOrigin;

    fn contraction(self, other: AntiDipoleOnOrigin) -> DipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Rotor {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Rotor {
    type Output = Flector;

    fn contraction(self, other: AntiPlane) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: CircleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Rotor {
    type Output = DipoleAtOrigin;

    fn contraction(self, other: CircleAtOrigin) -> DipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: CircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Rotor {
    type Output = DipoleAligningOrigin;

    fn contraction(self, other: CircleOrthogonalOrigin) -> DipoleAligningOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for Rotor {
    type Output = Infinity;

    fn contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Rotor {
    type Output = FlectorAtInfinity;

    fn contraction(self, other: Infinity) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Rotor {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for Rotor {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Rotor {
    type Output = Origin;

    fn contraction(self, other: NullSphereAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Rotor {
    type Output = AntiPlane;

    fn contraction(self, other: Plane) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Rotor {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: PlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Rotor {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Rotor {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Rotor {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: SphereOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: AntiPlane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: AntiSphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: Infinity) -> Scalar {
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

impl Contraction<Origin> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for RoundPoint {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiPlane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: AntiSphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: AntiCircleOnOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: AntiDipoleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Sphere {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Sphere {
    type Output = DipoleAligningOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> DipoleAligningOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Sphere {
    type Output = Circle;

    fn contraction(self, other: AntiPlane) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Sphere {
    type Output = CircleAligningOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> CircleAligningOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Sphere {
    type Output = Circle;

    fn contraction(self, other: AntiSphereOnOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Sphere {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: CircleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Sphere {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: DipoleAtInfinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Sphere {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Sphere {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Sphere {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: FlatPoint) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Sphere {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Sphere {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Sphere {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: Infinity) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Sphere {
    type Output = AntiPlane;

    fn contraction(self, other: LineAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Sphere {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
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

impl Contraction<NullCircleAtOrigin> for Sphere {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Sphere {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: NullSphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Sphere {
    type Output = AntiDipoleOnOrigin;

    fn contraction(self, other: Origin) -> AntiDipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Sphere {
    type Output = Circle;

    fn contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Sphere {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Sphere {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for SphereAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: AntiCircleOnOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for SphereAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: AntiDipoleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for SphereAtOrigin {
    type Output = DipoleAtOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> DipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for SphereAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: AntiPlane) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for SphereAtOrigin {
    type Output = CircleAtOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> CircleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for SphereAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for SphereAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for SphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAligningOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for SphereAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for SphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for SphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for SphereAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for SphereAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: Dipole) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for SphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for SphereAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAtInfinity) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for SphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for SphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for SphereAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for SphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPoint) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for SphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPointAtInfinity) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for SphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for SphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: Infinity) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for SphereAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for SphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtInfinity) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for SphereAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for SphereAtOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for SphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: NullSphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for SphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: Origin) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for SphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for SphereAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: RoundPoint) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for SphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for SphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for SphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for SphereAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for SphereAtOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for SphereOnOrigin {
    type Output = DipoleOnOrigin;

    fn contraction(self, other: AntiCircleOnOrigin) -> DipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for SphereOnOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiDipoleOnOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for SphereOnOrigin {
    type Output = Origin;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for SphereOnOrigin {
    type Output = DipoleOnOrigin;

    fn contraction(self, other: AntiLineAtOrigin) -> DipoleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for SphereOnOrigin {
    type Output = Circle;

    fn contraction(self, other: AntiPlane) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for SphereOnOrigin {
    type Output = CircleOnOrigin;

    fn contraction(self, other: AntiPlaneAtOrigin) -> CircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for SphereOnOrigin {
    type Output = CircleOnOrigin;

    fn contraction(self, other: AntiSphereOnOrigin) -> CircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for SphereOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for SphereOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for SphereOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for SphereOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for SphereOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for SphereOnOrigin {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for SphereOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAligningOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for SphereOnOrigin {
    type Output = Dipole;

    fn contraction(self, other: DipoleAtInfinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for SphereOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: DipoleAtOrigin) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for SphereOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn contraction(self, other: DipoleOnOrigin) -> AntiCircleOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for SphereOnOrigin {
    type Output = Dipole;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for SphereOnOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn contraction(self, other: FlatPoint) -> DipoleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for SphereOnOrigin {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for SphereOnOrigin {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for SphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for SphereOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: Infinity) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for SphereOnOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for SphereOnOrigin {
    type Output = AntiPlane;

    fn contraction(self, other: LineAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for SphereOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for SphereOnOrigin {
    type Output = Origin;

    fn contraction(self, other: NullCircleAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for SphereOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn contraction(self, other: NullDipoleAtOrigin) -> NullDipoleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for SphereOnOrigin {
    type Output = NullCircleAtOrigin;

    fn contraction(self, other: Origin) -> NullCircleAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for SphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for SphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for SphereOnOrigin {
    type Output = Circle;

    fn contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for SphereOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> CircleOrthogonalOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for SphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for SphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for SphereOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for SphereOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Transflector {
    type Output = RoundPoint;

    fn contraction(self, other: AntiDipoleOnOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Transflector {
    type Output = Infinity;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Transflector {
    type Output = FlatPoint;

    fn contraction(self, other: AntiLineAtOrigin) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Transflector {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Transflector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAligningOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Transflector {
    type Output = AntiPlane;

    fn contraction(self, other: CircleAtInfinity) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Transflector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Transflector {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: CircleOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Transflector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleOrthogonalOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Transflector {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Transflector {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Transflector {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Transflector {
    type Output = LineAtInfinity;

    fn contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Transflector {
    type Output = AntiPlane;

    fn contraction(self, other: Line) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Transflector {
    type Output = Infinity;

    fn contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Transflector {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: LineAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for Transflector {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: NullCircleAtOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: NullSphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: SphereAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: SphereOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiCircleOnOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiDipoleOnOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiFlatPointAtOrigin> for Translator {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: AntiFlatPointAtOrigin) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiLineAtOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: AntiLineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlane> for Translator {
    type Output = Transflector;

    fn contraction(self, other: AntiPlane) -> Transflector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiPlaneAtOrigin> for Translator {
    type Output = Transflector;

    fn contraction(self, other: AntiPlaneAtOrigin) -> Transflector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<AntiSphereOnOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAligningOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: CircleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Translator {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: CircleAtInfinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOnOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: CircleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleOrthogonalOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dilator> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Dilator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAligningOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAligningOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOnOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleOrthogonalOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Translator {
    type Output = AntiFlatPointAtOrigin;

    fn contraction(self, other: FlatPointAtOrigin) -> AntiFlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for Translator {
    type Output = Infinity;

    fn contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for Translator {
    type Output = Horizon;

    fn contraction(self, other: Infinity) -> Horizon {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Translator {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: Line) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Translator {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Translator {
    type Output = AntiLineAtOrigin;

    fn contraction(self, other: LineAtOrigin) -> AntiLineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullCircleAtOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: NullCircleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullDipoleAtOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<NullSphereAtOrigin> for Translator {
    type Output = Origin;

    fn contraction(self, other: NullSphereAtOrigin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Translator {
    type Output = AntiPlane;

    fn contraction(self, other: Plane) -> AntiPlane {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Translator {
    type Output = AntiPlaneAtOrigin;

    fn contraction(self, other: PlaneAtOrigin) -> AntiPlaneAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Translator {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereAtOrigin> for Translator {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: SphereAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereOnOrigin> for Translator {
    type Output = AntiSphereOnOrigin;

    fn contraction(self, other: SphereOnOrigin) -> AntiSphereOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}
