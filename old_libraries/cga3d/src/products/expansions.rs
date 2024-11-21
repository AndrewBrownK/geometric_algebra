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

impl Expansion<AntiCircleOnOrigin> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for AntiCircleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for AntiCircleOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiLineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for AntiCircleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for AntiCircleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for AntiCircleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for AntiCircleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: CircleOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for AntiCircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for AntiCircleOnOrigin {
    type Output = Line;

    fn expansion(self, other: Horizon) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for AntiCircleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for AntiCircleOnOrigin {
    type Output = Plane;

    fn expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for AntiCircleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for AntiCircleOnOrigin {
    type Output = Circle;

    fn expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for AntiCircleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> AntiDipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for AntiCircleOnOrigin {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for AntiCircleOnOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: SphereAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for AntiCircleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn expansion(self, other: SphereOnOrigin) -> AntiDipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for AntiCircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for AntiDipoleOnOrigin {
    type Output = Plane;

    fn expansion(self, other: Horizon) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for AntiDipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for AntiDipoleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for AntiDipoleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for AntiDipoleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for AntiDipoleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for AntiDipoleOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: SphereAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for AntiDipoleOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for AntiDipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for AntiFlatPointAtOrigin {
    type Output = Horizon;

    fn expansion(self, other: Horizon) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for AntiFlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for AntiFlatPointAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for AntiFlatPointAtOrigin {
    type Output = Horizon;

    fn expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for AntiFlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: Sphere) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for AntiFlatPointAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for AntiFlatPointAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for AntiFlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for AntiLineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for AntiLineAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for AntiLineAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for AntiLineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiLineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for AntiLineAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for AntiLineAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: CircleAligningOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for AntiLineAtOrigin {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for AntiLineAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for AntiLineAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: CircleOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for AntiLineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for AntiLineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for AntiLineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for AntiLineAtOrigin {
    type Output = LineAtInfinity;

    fn expansion(self, other: Horizon) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for AntiLineAtOrigin {
    type Output = Plane;

    fn expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for AntiLineAtOrigin {
    type Output = Horizon;

    fn expansion(self, other: LineAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for AntiLineAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for AntiLineAtOrigin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for AntiLineAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn expansion(self, other: Plane) -> CircleOrthogonalOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for AntiLineAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> AntiFlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for AntiLineAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn expansion(self, other: Sphere) -> CircleOrthogonalOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for AntiLineAtOrigin {
    type Output = CircleAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> CircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for AntiLineAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn expansion(self, other: SphereOnOrigin) -> AntiDipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for AntiLineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for AntiPlane {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiCircleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for AntiPlane {
    type Output = CircleOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for AntiPlane {
    type Output = LineAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for AntiPlane {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiLineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlane> for AntiPlane {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlaneAtOrigin> for AntiPlane {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiSphereOnOrigin> for AntiPlane {
    type Output = AntiScalar;

    fn expansion(self, other: AntiSphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for AntiPlane {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for AntiPlane {
    type Output = Circle;

    fn expansion(self, other: CircleAligningOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for AntiPlane {
    type Output = CircleAtInfinity;

    fn expansion(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for AntiPlane {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for AntiPlane {
    type Output = Circle;

    fn expansion(self, other: CircleOnOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for AntiPlane {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for AntiPlane {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for AntiPlane {
    type Output = Sphere;

    fn expansion(self, other: DipoleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for AntiPlane {
    type Output = Plane;

    fn expansion(self, other: DipoleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for AntiPlane {
    type Output = Sphere;

    fn expansion(self, other: DipoleAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for AntiPlane {
    type Output = Sphere;

    fn expansion(self, other: DipoleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for AntiPlane {
    type Output = Sphere;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for AntiPlane {
    type Output = Sphere;

    fn expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for AntiPlane {
    type Output = Horizon;

    fn expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for AntiPlane {
    type Output = Horizon;

    fn expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for AntiPlane {
    type Output = FlatPointAtInfinity;

    fn expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for AntiPlane {
    type Output = CircleAtInfinity;

    fn expansion(self, other: Line) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for AntiPlane {
    type Output = LineAtInfinity;

    fn expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for AntiPlane {
    type Output = CircleOrthogonalOrigin;

    fn expansion(self, other: LineAtOrigin) -> CircleOrthogonalOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for AntiPlane {
    type Output = CircleOnOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for AntiPlane {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullDipoleAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for AntiPlane {
    type Output = DipoleOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> DipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Origin> for AntiPlane {
    type Output = AntiScalar;

    fn expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for AntiPlane {
    type Output = DipoleAtInfinity;

    fn expansion(self, other: Plane) -> DipoleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for AntiPlane {
    type Output = DipoleAtInfinity;

    fn expansion(self, other: PlaneAtOrigin) -> DipoleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for AntiPlane {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for AntiPlane {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for AntiPlane {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for AntiPlane {
    type Output = DipoleAligningOrigin;

    fn expansion(self, other: SphereAtOrigin) -> DipoleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for AntiPlane {
    type Output = Dipole;

    fn expansion(self, other: SphereOnOrigin) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for AntiPlane {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiCircleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for AntiPlaneAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiLineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlane> for AntiPlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiSphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiSphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for AntiPlaneAtOrigin {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = CircleOrthogonalOrigin;

    fn expansion(self, other: CircleAligningOrigin) -> CircleOrthogonalOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for AntiPlaneAtOrigin {
    type Output = CircleAtInfinity;

    fn expansion(self, other: CircleAtInfinity) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for AntiPlaneAtOrigin {
    type Output = CircleAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> CircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for AntiPlaneAtOrigin {
    type Output = AntiDipoleOnOrigin;

    fn expansion(self, other: CircleOnOrigin) -> AntiDipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for AntiPlaneAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for AntiPlaneAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: DipoleAligningOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for AntiPlaneAtOrigin {
    type Output = Plane;

    fn expansion(self, other: DipoleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for AntiPlaneAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: DipoleAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for AntiPlaneAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: DipoleOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for AntiPlaneAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for AntiPlaneAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: FlatPoint) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for AntiPlaneAtOrigin {
    type Output = Horizon;

    fn expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for AntiPlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for AntiPlaneAtOrigin {
    type Output = CircleAtInfinity;

    fn expansion(self, other: Line) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for AntiPlaneAtOrigin {
    type Output = LineAtInfinity;

    fn expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> AntiFlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for AntiPlaneAtOrigin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for AntiPlaneAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: NullDipoleAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for AntiPlaneAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> NullDipoleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for AntiPlaneAtOrigin {
    type Output = DipoleAtInfinity;

    fn expansion(self, other: Plane) -> DipoleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for AntiPlaneAtOrigin {
    type Output = AntiLineAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> AntiLineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for AntiPlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for AntiPlaneAtOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn expansion(self, other: Sphere) -> DipoleOrthogonalOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for AntiPlaneAtOrigin {
    type Output = DipoleAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> DipoleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for AntiPlaneAtOrigin {
    type Output = AntiCircleOnOrigin;

    fn expansion(self, other: SphereOnOrigin) -> AntiCircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for AntiPlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiCircleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for AntiSphereOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiLineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlane> for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiSphereOnOrigin> for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiSphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for AntiSphereOnOrigin {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for AntiSphereOnOrigin {
    type Output = Circle;

    fn expansion(self, other: CircleAligningOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for AntiSphereOnOrigin {
    type Output = Circle;

    fn expansion(self, other: CircleAtInfinity) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for AntiSphereOnOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for AntiSphereOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn expansion(self, other: CircleOnOrigin) -> AntiDipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for AntiSphereOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for AntiSphereOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for AntiSphereOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for AntiSphereOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for AntiSphereOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: DipoleOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for AntiSphereOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for AntiSphereOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for AntiSphereOnOrigin {
    type Output = Plane;

    fn expansion(self, other: FlatPointAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for AntiSphereOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: FlatPointAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for AntiSphereOnOrigin {
    type Output = FlatPoint;

    fn expansion(self, other: Horizon) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Infinity> for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for AntiSphereOnOrigin {
    type Output = Circle;

    fn expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for AntiSphereOnOrigin {
    type Output = Line;

    fn expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for AntiSphereOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn expansion(self, other: LineAtOrigin) -> AntiDipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for AntiSphereOnOrigin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for AntiSphereOnOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: NullDipoleAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for AntiSphereOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> NullDipoleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for AntiSphereOnOrigin {
    type Output = Dipole;

    fn expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for AntiSphereOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> AntiCircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for AntiSphereOnOrigin {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for AntiSphereOnOrigin {
    type Output = DipoleAligningOrigin;

    fn expansion(self, other: SphereAtOrigin) -> DipoleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for AntiSphereOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn expansion(self, other: SphereOnOrigin) -> AntiCircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for AntiSphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for Circle {
    type Output = Plane;

    fn expansion(self, other: Horizon) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
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

impl Expansion<NullCircleAtOrigin> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Circle {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Circle {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Circle {
    type Output = Sphere;

    fn expansion(self, other: PlaneAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Circle {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Circle {
    type Output = Sphere;

    fn expansion(self, other: SphereAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Circle {
    type Output = Sphere;

    fn expansion(self, other: SphereOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Circle {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for CircleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for CircleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: PlaneAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for CircleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for CircleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: SphereOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: Horizon) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for CircleAtInfinity {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleAtInfinity {
    type Output = Plane;

    fn expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for CircleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: SphereAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for CircleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: SphereOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for CircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for CircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for CircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for CircleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: SphereOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for CircleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Plane) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Sphere) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for CircleOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for CircleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: SphereOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleOrthogonalOrigin {
    type Output = Plane;

    fn expansion(self, other: Horizon) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for CircleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for CircleOrthogonalOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleOrthogonalOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for CircleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: SphereAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for CircleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: SphereOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for Dilator {
    type Output = Plane;

    fn expansion(self, other: Horizon) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for Dilator {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Dilator {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Dilator {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Dilator {
    type Output = Sphere;

    fn expansion(self, other: PlaneAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Dilator {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Dilator {
    type Output = Sphere;

    fn expansion(self, other: SphereAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Dilator {
    type Output = Sphere;

    fn expansion(self, other: SphereOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Dilator {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Dipole {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for Dipole {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: AntiLineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: CircleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: CircleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: CircleAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: CircleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: CircleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for Dipole {
    type Output = Line;

    fn expansion(self, other: Horizon) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for Dipole {
    type Output = Plane;

    fn expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: LineAtOrigin) -> Sphere {
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

impl Expansion<NullCircleAtOrigin> for Dipole {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Dipole {
    type Output = CircleOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Dipole {
    type Output = Circle;

    fn expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Dipole {
    type Output = Circle;

    fn expansion(self, other: PlaneAtOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Dipole {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Dipole {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: SphereAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Dipole {
    type Output = Circle;

    fn expansion(self, other: SphereOnOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Dipole {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for DipoleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for DipoleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for DipoleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for DipoleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for DipoleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleAligningOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for DipoleAligningOrigin {
    type Output = Sphere;

    fn expansion(self, other: LineAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for DipoleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for DipoleAligningOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for DipoleAligningOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: Plane) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: Sphere) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for DipoleAligningOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for DipoleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: SphereOnOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleAligningOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for DipoleAtInfinity {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for DipoleAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiLineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: CircleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: CircleAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: CircleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: CircleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleAtInfinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: Horizon) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: LineAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for DipoleAtInfinity {
    type Output = Plane;

    fn expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for DipoleAtInfinity {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for DipoleAtInfinity {
    type Output = CircleOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleAtInfinity {
    type Output = CircleAtInfinity;

    fn expansion(self, other: Plane) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleAtInfinity {
    type Output = CircleAtInfinity;

    fn expansion(self, other: PlaneAtOrigin) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleAtInfinity {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for DipoleAtInfinity {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: SphereAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for DipoleAtInfinity {
    type Output = Circle;

    fn expansion(self, other: SphereOnOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for DipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for DipoleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for DipoleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for DipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for DipoleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for DipoleAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for DipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for DipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for DipoleAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: Plane) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> CircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: Sphere) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for DipoleAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for DipoleAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: SphereOnOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Circle) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: CircleAligningOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: CircleAtInfinity) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for DipoleOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: CircleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleOnOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Line) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: LineAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleOnOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: Plane) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleOnOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleOnOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: Sphere) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for DipoleOnOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for DipoleOnOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: SphereOnOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for DipoleOrthogonalOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiLineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: CircleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleOrthogonalOrigin {
    type Output = Line;

    fn expansion(self, other: Horizon) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleOrthogonalOrigin {
    type Output = Plane;

    fn expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for DipoleOrthogonalOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for DipoleOrthogonalOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleOrthogonalOrigin {
    type Output = Circle;

    fn expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> CircleOrthogonalOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleOrthogonalOrigin {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: SphereAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for DipoleOrthogonalOrigin {
    type Output = Circle;

    fn expansion(self, other: SphereOnOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleOrthogonalOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: CircleAligningOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: CircleOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: LineAtOrigin) -> Plane {
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

impl Expansion<NullCircleAtOrigin> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for FlatPoint {
    type Output = LineAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for FlatPoint {
    type Output = Line;

    fn expansion(self, other: Plane) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for FlatPoint {
    type Output = Line;

    fn expansion(self, other: PlaneAtOrigin) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlatPoint {
    type Output = Line;

    fn expansion(self, other: Sphere) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for FlatPoint {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for FlatPoint {
    type Output = Line;

    fn expansion(self, other: SphereOnOrigin) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for FlatPoint {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for FlatPointAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for FlatPointAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleAligningOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for FlatPointAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: CircleAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for FlatPointAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for FlatPointAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: Line) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for FlatPointAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlatPointAtInfinity {
    type Output = Line;

    fn expansion(self, other: Sphere) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for FlatPointAtInfinity {
    type Output = Line;

    fn expansion(self, other: SphereOnOrigin) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAligningOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Flector {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Flector {
    type Output = Plane;

    fn expansion(self, other: CircleAligningOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Flector {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Flector {
    type Output = Plane;

    fn expansion(self, other: CircleOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Flector {
    type Output = Plane;

    fn expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Flector {
    type Output = Plane;

    fn expansion(self, other: LineAtOrigin) -> Plane {
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

impl Expansion<NullCircleAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Flector {
    type Output = Rotor;

    fn expansion(self, other: NullSphereAtOrigin) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Flector {
    type Output = Motor;

    fn expansion(self, other: Plane) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Flector {
    type Output = Motor;

    fn expansion(self, other: PlaneAtOrigin) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Flector {
    type Output = Motor;

    fn expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Flector {
    type Output = Rotor;

    fn expansion(self, other: SphereAtOrigin) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Flector {
    type Output = Motor;

    fn expansion(self, other: SphereOnOrigin) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Flector {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for FlectorAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for FlectorAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleAligningOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for FlectorAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: CircleAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for FlectorAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for FlectorAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: Line) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for FlectorAtInfinity {
    type Output = Rotor;

    fn expansion(self, other: NullSphereAtOrigin) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlectorAtInfinity {
    type Output = Motor;

    fn expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for FlectorAtInfinity {
    type Output = Rotor;

    fn expansion(self, other: SphereAtOrigin) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for FlectorAtInfinity {
    type Output = Motor;

    fn expansion(self, other: SphereOnOrigin) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Horizon {
    type Output = AntiScalar;

    fn expansion(self, other: NullSphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Horizon {
    type Output = AntiScalar;

    fn expansion(self, other: SphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Horizon {
    type Output = AntiScalar;

    fn expansion(self, other: SphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for Infinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiCircleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Infinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiSphereOnOrigin> for Infinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiSphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Infinity {
    type Output = Line;

    fn expansion(self, other: Circle) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Infinity {
    type Output = Line;

    fn expansion(self, other: CircleAligningOrigin) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Infinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: CircleAtInfinity) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Infinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Infinity {
    type Output = Line;

    fn expansion(self, other: CircleOnOrigin) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Infinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Infinity {
    type Output = Plane;

    fn expansion(self, other: Dipole) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for Infinity {
    type Output = Plane;

    fn expansion(self, other: DipoleAligningOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for Infinity {
    type Output = Horizon;

    fn expansion(self, other: DipoleAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for Infinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for Infinity {
    type Output = Plane;

    fn expansion(self, other: DipoleOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for Infinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Infinity {
    type Output = Plane;

    fn expansion(self, other: FlatPoint) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for Infinity {
    type Output = Horizon;

    fn expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Infinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: Line) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for Infinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for Infinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullDipoleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Infinity {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Origin> for Infinity {
    type Output = AntiScalar;

    fn expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Infinity {
    type Output = FlatPointAtInfinity;

    fn expansion(self, other: Plane) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn expansion(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for Infinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for Infinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Infinity {
    type Output = FlatPoint;

    fn expansion(self, other: Sphere) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Infinity {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Infinity {
    type Output = FlatPoint;

    fn expansion(self, other: SphereOnOrigin) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Infinity {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Line {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Line {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
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

impl Expansion<NullCircleAtOrigin> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Line {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Line {
    type Output = Plane;

    fn expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Line {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Line {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Line {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Line {
    type Output = Plane;

    fn expansion(self, other: SphereOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Line {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Line {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for LineAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for LineAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for LineAtInfinity {
    type Output = Plane;

    fn expansion(self, other: SphereOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for LineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
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

impl Expansion<NullCircleAtOrigin> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Motor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Motor {
    type Output = Plane;

    fn expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Motor {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Motor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Motor {
    type Output = Plane;

    fn expansion(self, other: SphereOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Motor {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: AntiCircleOnOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: AntiDipoleOnOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: AntiLineAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlane> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: AntiPlane) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: AntiPlaneAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: AntiSphereOnOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleAligningOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleOnOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleOrthogonalOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleAligningOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleOnOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: FlatPoint) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Horizon) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Infinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Infinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Line) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: LineAtOrigin) -> MultiVector {
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

impl Expansion<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: NullCircleAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: NullDipoleAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: NullSphereAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Origin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Origin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: RoundPoint) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: SphereAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: SphereOnOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for NullCircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for NullCircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for NullCircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for NullCircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for NullCircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for NullCircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for NullCircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for NullCircleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for NullCircleAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Plane) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for NullCircleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for NullCircleAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Sphere) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for NullCircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for NullCircleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for NullCircleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for NullDipoleAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Circle) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for NullDipoleAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: CircleAligningOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for NullDipoleAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: CircleAtInfinity) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for NullDipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for NullDipoleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: CircleOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for NullDipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for NullDipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for NullDipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for NullDipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for NullDipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for NullDipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for NullDipoleAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for NullDipoleAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for NullDipoleAtOrigin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Line) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for NullDipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for NullDipoleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for NullDipoleAtOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: Plane) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for NullDipoleAtOrigin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: Sphere) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for NullDipoleAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for NullDipoleAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for NullSphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for NullSphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for NullSphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for NullSphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: SphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for NullSphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlane> for Origin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Origin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: Circle) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Origin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: CircleAligningOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Origin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: CircleAtInfinity) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: CircleOnOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Origin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Origin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: Dipole) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for Origin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: DipoleAligningOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for Origin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: DipoleAtInfinity) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for Origin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: DipoleOnOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Origin {
    type Output = SphereOnOrigin;

    fn expansion(self, other: FlatPoint) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for Origin {
    type Output = NullSphereAtOrigin;

    fn expansion(self, other: FlatPointAtOrigin) -> NullSphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for Origin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Infinity> for Origin {
    type Output = AntiScalar;

    fn expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Origin {
    type Output = CircleOnOrigin;

    fn expansion(self, other: Line) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for Origin {
    type Output = LineAtOrigin;

    fn expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Origin {
    type Output = NullCircleAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> NullCircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Origin {
    type Output = DipoleOnOrigin;

    fn expansion(self, other: Plane) -> DipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> NullDipoleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for Origin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for Origin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Origin {
    type Output = DipoleOnOrigin;

    fn expansion(self, other: Sphere) -> DipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Origin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Origin {
    type Output = NullDipoleAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> NullDipoleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Origin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
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

impl Expansion<NullSphereAtOrigin> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: NullSphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: SphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: SphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Plane {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: SphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Rotor {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Rotor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Rotor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Rotor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Rotor {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Rotor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Rotor {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for RoundPoint {
    type Output = SphereOnOrigin;

    fn expansion(self, other: AntiCircleOnOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for RoundPoint {
    type Output = CircleOnOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiFlatPointAtOrigin> for RoundPoint {
    type Output = LineAtOrigin;

    fn expansion(self, other: AntiFlatPointAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiLineAtOrigin> for RoundPoint {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiLineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlane> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlaneAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiSphereOnOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: AntiSphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: CircleAligningOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: CircleAtInfinity) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for RoundPoint {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleAtOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: CircleOnOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for RoundPoint {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: DipoleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: DipoleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: DipoleAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: DipoleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for RoundPoint {
    type Output = Plane;

    fn expansion(self, other: FlatPointAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for RoundPoint {
    type Output = SphereAtOrigin;

    fn expansion(self, other: FlatPointAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for RoundPoint {
    type Output = FlatPoint;

    fn expansion(self, other: Horizon) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Infinity> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for RoundPoint {
    type Output = Line;

    fn expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for RoundPoint {
    type Output = CircleOrthogonalOrigin;

    fn expansion(self, other: LineAtOrigin) -> CircleOrthogonalOrigin {
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

impl Expansion<NullCircleAtOrigin> for RoundPoint {
    type Output = CircleOnOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> CircleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for RoundPoint {
    type Output = SphereOnOrigin;

    fn expansion(self, other: NullDipoleAtOrigin) -> SphereOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for RoundPoint {
    type Output = DipoleOnOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> DipoleOnOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Origin> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for RoundPoint {
    type Output = DipoleOrthogonalOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> DipoleOrthogonalOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for RoundPoint {
    type Output = DipoleAligningOrigin;

    fn expansion(self, other: SphereAtOrigin) -> DipoleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: SphereOnOrigin) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for RoundPoint {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiCircleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiPlane> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiPlane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiSphereOnOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: AntiSphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for RoundPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: Circle) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for RoundPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleAligningOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for RoundPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleAtInfinity) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for RoundPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: CircleOnOrigin) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPointAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for RoundPointAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleAligningOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for RoundPointAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleAtInfinity) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for RoundPointAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleOnOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for RoundPointAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn expansion(self, other: FlatPointAtOrigin) -> SphereAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Infinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for RoundPointAtOrigin {
    type Output = CircleAligningOrigin;

    fn expansion(self, other: Line) -> CircleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for RoundPointAtOrigin {
    type Output = CircleAtOrigin;

    fn expansion(self, other: LineAtOrigin) -> CircleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullDipoleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Origin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for RoundPointAtOrigin {
    type Output = DipoleAligningOrigin;

    fn expansion(self, other: Plane) -> DipoleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = DipoleAtOrigin;

    fn expansion(self, other: PlaneAtOrigin) -> DipoleAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPointAtOrigin {
    type Output = DipoleAligningOrigin;

    fn expansion(self, other: Sphere) -> DipoleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for RoundPointAtOrigin {
    type Output = DipoleAligningOrigin;

    fn expansion(self, other: SphereOnOrigin) -> DipoleAligningOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: Horizon) -> AntiScalar {
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

impl Expansion<NullSphereAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: NullSphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: SphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: SphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Sphere {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for SphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for SphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: NullSphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for SphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for SphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for SphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: SphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for SphereAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: SphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for SphereAtOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for SphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for SphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for SphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for SphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for SphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: SphereAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for SphereOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: SphereOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for SphereOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiCircleOnOrigin> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: AntiCircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: AntiDipoleOnOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Transflector {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Transflector {
    type Output = Plane;

    fn expansion(self, other: CircleAligningOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Transflector {
    type Output = Horizon;

    fn expansion(self, other: CircleAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Transflector {
    type Output = Plane;

    fn expansion(self, other: CircleOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleOrthogonalOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAligningOrigin> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtOrigin> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOnOrigin> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleOrthogonalOrigin> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for Transflector {
    type Output = Horizon;

    fn expansion(self, other: Line) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for Transflector {
    type Output = Horizon;

    fn expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for Transflector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullCircleAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullDipoleAtOrigin> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: NullDipoleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Transflector {
    type Output = Rotor;

    fn expansion(self, other: NullSphereAtOrigin) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Transflector {
    type Output = Translator;

    fn expansion(self, other: Plane) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Transflector {
    type Output = Translator;

    fn expansion(self, other: PlaneAtOrigin) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Transflector {
    type Output = Motor;

    fn expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Transflector {
    type Output = Rotor;

    fn expansion(self, other: SphereAtOrigin) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Transflector {
    type Output = Motor;

    fn expansion(self, other: SphereOnOrigin) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Transflector {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<AntiDipoleOnOrigin> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: AntiDipoleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAligningOrigin> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAligningOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtOrigin> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOnOrigin> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleOrthogonalOrigin> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleOrthogonalOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dilator> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: Dilator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullCircleAtOrigin> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: NullCircleAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<NullSphereAtOrigin> for Translator {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: NullSphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for Translator {
    type Output = Horizon;

    fn expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Translator {
    type Output = Horizon;

    fn expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Translator {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereAtOrigin> for Translator {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereOnOrigin> for Translator {
    type Output = Plane;

    fn expansion(self, other: SphereOnOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for Translator {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}
