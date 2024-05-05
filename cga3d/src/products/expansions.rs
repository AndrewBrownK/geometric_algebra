//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::exterior::Wedge;
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

impl Expansion<CircleAtInfinity> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Circle {
    type Output = AntiScalar;

    fn expansion(self, other: CircleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for Circle {
    type Output = Sphere;

    fn expansion(self, other: SpacialCurvature) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Circle {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Circle {
    type Output = Sphere;

    fn expansion(self, other: SphereWeight) -> Sphere {
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

impl Expansion<Circle> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for CircleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleWeight) -> AntiScalar {
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

impl Expansion<Plane> for CircleAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for CircleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: SpacialCurvature) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for CircleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: SphereWeight) -> Sphere {
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

impl Expansion<Circle> for CircleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for CircleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: CircleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for CircleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleBulk {
    type Output = Horizon;

    fn expansion(self, other: Horizon) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleBulk {
    type Output = Horizon;

    fn expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for CircleBulk {
    type Output = SpacialCurvature;

    fn expansion(self, other: SpacialCurvature) -> SpacialCurvature {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleBulk {
    type Output = SpacialCurvature;

    fn expansion(self, other: Sphere) -> SpacialCurvature {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for CircleBulk {
    type Output = SphereWeight;

    fn expansion(self, other: SphereWeight) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: CircleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleCarrierAspect {
    type Output = Plane;

    fn expansion(self, other: Horizon) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleCarrierAspect {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn expansion(self, other: PlaneAtOrigin) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for CircleCarrierAspect {
    type Output = Sphere;

    fn expansion(self, other: SpacialCurvature) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleCarrierAspect {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn expansion(self, other: SphereWeight) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for CircleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for CircleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: CircleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for CircleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for CircleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for CircleWeight {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for CircleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for CircleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for CircleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for CircleWeight {
    type Output = Sphere;

    fn expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for CircleWeight {
    type Output = SphereWeight;

    fn expansion(self, other: PlaneAtOrigin) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for CircleWeight {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for CircleWeight {
    type Output = Sphere;

    fn expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for CircleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Dipole {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for Dipole {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Dipole {
    type Output = Sphere;

    fn expansion(self, other: CircleWeight) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for Dipole {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for Dipole {
    type Output = Circle;

    fn expansion(self, other: SpacialCurvature) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Dipole {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Dipole {
    type Output = Circle;

    fn expansion(self, other: SphereWeight) -> Circle {
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

impl Expansion<Circle> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for DipoleAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for DipoleAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: CircleWeight) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for DipoleAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleWeight) -> AntiScalar {
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
    type Output = Horizon;

    fn expansion(self, other: Line) -> Horizon {
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
    type Output = Horizon;

    fn expansion(self, other: LineAtOrigin) -> Horizon {
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

impl Expansion<SpacialCurvature> for DipoleAtInfinity {
    type Output = Circle;

    fn expansion(self, other: SpacialCurvature) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleAtInfinity {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for DipoleAtInfinity {
    type Output = Circle;

    fn expansion(self, other: SphereWeight) -> Circle {
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

impl Expansion<Circle> for DipoleBulk {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleBulk {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for DipoleBulk {
    type Output = Sphere;

    fn expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for DipoleBulk {
    type Output = SphereWeight;

    fn expansion(self, other: CircleWeight) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for DipoleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for DipoleBulk {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleBulk {
    type Output = LineAtInfinity;

    fn expansion(self, other: Horizon) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleBulk {
    type Output = Horizon;

    fn expansion(self, other: Line) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleBulk {
    type Output = Horizon;

    fn expansion(self, other: LineAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleBulk {
    type Output = CircleAtInfinity;

    fn expansion(self, other: Plane) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleBulk {
    type Output = CircleBulk;

    fn expansion(self, other: PlaneAtOrigin) -> CircleBulk {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for DipoleBulk {
    type Output = Circle;

    fn expansion(self, other: SpacialCurvature) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleBulk {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for DipoleBulk {
    type Output = CircleWeight;

    fn expansion(self, other: SphereWeight) -> CircleWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for DipoleCarrierAspect {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleCarrierAspect {
    type Output = Plane;

    fn expansion(self, other: CircleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for DipoleCarrierAspect {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = Sphere;

    fn expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn expansion(self, other: CircleWeight) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleCarrierAspect {
    type Output = Line;

    fn expansion(self, other: Horizon) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleCarrierAspect {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleCarrierAspect {
    type Output = Plane;

    fn expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn expansion(self, other: LineAtOrigin) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleCarrierAspect {
    type Output = Circle;

    fn expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn expansion(self, other: PlaneAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for DipoleCarrierAspect {
    type Output = Circle;

    fn expansion(self, other: SpacialCurvature) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleCarrierAspect {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for DipoleCarrierAspect {
    type Output = CircleWeight;

    fn expansion(self, other: SphereWeight) -> CircleWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for DipoleWeight {
    type Output = Sphere;

    fn expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for DipoleWeight {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for DipoleWeight {
    type Output = Sphere;

    fn expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for DipoleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for DipoleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for DipoleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for DipoleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for DipoleWeight {
    type Output = AntiScalar;

    fn expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for DipoleWeight {
    type Output = LineAtOrigin;

    fn expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for DipoleWeight {
    type Output = Sphere;

    fn expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for DipoleWeight {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for DipoleWeight {
    type Output = SphereWeight;

    fn expansion(self, other: LineAtOrigin) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for DipoleWeight {
    type Output = Circle;

    fn expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for DipoleWeight {
    type Output = CircleWeight;

    fn expansion(self, other: PlaneAtOrigin) -> CircleWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for DipoleWeight {
    type Output = LineAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for DipoleWeight {
    type Output = Circle;

    fn expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for FlatPoint {
    type Output = Plane;

    fn expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleWeight) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for FlatPoint {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for FlatPoint {
    type Output = LineAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlatPoint {
    type Output = Line;

    fn expansion(self, other: Sphere) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for FlatPoint {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereWeight) -> LineAtOrigin {
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

impl Expansion<Circle> for FlatPointAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleWeight) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlatPointAtInfinity {
    type Output = Line;

    fn expansion(self, other: Sphere) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: SphereWeight) -> LineAtOrigin {
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

impl Expansion<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleCarrierAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
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

impl Expansion<Circle> for Flector {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Flector {
    type Output = Plane;

    fn expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Flector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleWeight) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for Flector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for Flector {
    type Output = Rotor;

    fn expansion(self, other: SpacialCurvature) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Flector {
    type Output = Motor;

    fn expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Flector {
    type Output = Rotor;

    fn expansion(self, other: SphereWeight) -> Rotor {
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

impl Expansion<Circle> for FlectorAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Plane;

    fn expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleWeight) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for FlectorAtInfinity {
    type Output = Rotor;

    fn expansion(self, other: SpacialCurvature) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for FlectorAtInfinity {
    type Output = Motor;

    fn expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for FlectorAtInfinity {
    type Output = Rotor;

    fn expansion(self, other: SphereWeight) -> Rotor {
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

impl Expansion<Rotor> for Horizon {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for Horizon {
    type Output = AntiScalar;

    fn expansion(self, other: SpacialCurvature) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Horizon {
    type Output = AntiScalar;

    fn expansion(self, other: SphereWeight) -> AntiScalar {
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

impl Expansion<Circle> for Infinity {
    type Output = Line;

    fn expansion(self, other: Circle) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Infinity {
    type Output = Line;

    fn expansion(self, other: CircleCarrierAspect) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Infinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleWeight) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Infinity {
    type Output = Plane;

    fn expansion(self, other: Dipole) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for Infinity {
    type Output = Plane;

    fn expansion(self, other: DipoleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for Infinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleWeight) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Infinity {
    type Output = Horizon;

    fn expansion(self, other: FlatPoint) -> Horizon {
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

impl Expansion<RoundPointOnOrigin> for Infinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for Infinity {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Infinity {
    type Output = FlatPoint;

    fn expansion(self, other: Sphere) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Infinity {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SphereWeight) -> FlatPointAtOrigin {
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

impl Expansion<Circle> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Line {
    type Output = AntiScalar;

    fn expansion(self, other: CircleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for Line {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Line {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Line {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereWeight) -> PlaneAtOrigin {
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

impl Expansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for LineAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: CircleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereWeight) -> PlaneAtOrigin {
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

impl Expansion<CircleCarrierAspect> for LineAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
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

impl Expansion<Circle> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Motor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for Motor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Motor {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Motor {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereWeight) -> PlaneAtOrigin {
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

impl Expansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleBulk) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleCarrierAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: CircleWeight) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: DipoleWeight) -> MultiVector {
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

impl Expansion<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: SpacialCurvature) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn expansion(self, other: SphereWeight) -> MultiVector {
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

impl Expansion<Circle> for Origin {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for Origin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleAtInfinity) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Origin {
    type Output = Circle;

    fn expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Origin {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for Origin {
    type Output = Sphere;

    fn expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for Origin {
    type Output = Sphere;

    fn expansion(self, other: FlatPoint) -> Sphere {
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
    type Output = SphereWeight;

    fn expansion(self, other: FlatPointAtOrigin) -> SphereWeight {
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
    type Output = Circle;

    fn expansion(self, other: Line) -> Circle {
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
    type Output = CircleWeight;

    fn expansion(self, other: LineAtOrigin) -> CircleWeight {
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
    type Output = Dipole;

    fn expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for Origin {
    type Output = DipoleWeight;

    fn expansion(self, other: PlaneAtOrigin) -> DipoleWeight {
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

impl Expansion<RoundPointAtInfinity> for Origin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for Origin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointOnOrigin> for Origin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for Origin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Origin {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
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

impl Expansion<SpacialCurvature> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: SpacialCurvature) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Plane {
    type Output = AntiScalar;

    fn expansion(self, other: SphereWeight) -> AntiScalar {
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

impl Expansion<CircleCarrierAspect> for Rotor {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
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

impl Expansion<Circle> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for RoundPoint {
    type Output = Line;

    fn expansion(self, other: CircleAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for RoundPoint {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleBulk) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for RoundPoint {
    type Output = Circle;

    fn expansion(self, other: CircleWeight) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for RoundPoint {
    type Output = Plane;

    fn expansion(self, other: DipoleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for RoundPoint {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for RoundPoint {
    type Output = Sphere;

    fn expansion(self, other: DipoleWeight) -> Sphere {
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
    type Output = SpacialCurvature;

    fn expansion(self, other: FlatPointAtOrigin) -> SpacialCurvature {
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
    type Output = Circle;

    fn expansion(self, other: LineAtOrigin) -> Circle {
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
    type Output = Dipole;

    fn expansion(self, other: PlaneAtOrigin) -> Dipole {
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

impl Expansion<RoundPointAtInfinity> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointBulk> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointOnOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: SpacialCurvature) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for RoundPoint {
    type Output = Dipole;

    fn expansion(self, other: SphereWeight) -> Dipole {
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

impl Expansion<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for RoundPointAtInfinity {
    type Output = Line;

    fn expansion(self, other: CircleAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for RoundPointAtInfinity {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleBulk) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = Circle;

    fn expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for RoundPointAtInfinity {
    type Output = Circle;

    fn expansion(self, other: CircleWeight) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPointAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for RoundPointAtInfinity {
    type Output = Plane;

    fn expansion(self, other: DipoleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for RoundPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for RoundPointAtInfinity {
    type Output = Sphere;

    fn expansion(self, other: DipoleWeight) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for RoundPointAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: FlatPoint) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Horizon;

    fn expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for RoundPointAtInfinity {
    type Output = CircleAtInfinity;

    fn expansion(self, other: Line) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for RoundPointAtInfinity {
    type Output = CircleAtInfinity;

    fn expansion(self, other: LineAtOrigin) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Origin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for RoundPointAtInfinity {
    type Output = DipoleAtInfinity;

    fn expansion(self, other: Plane) -> DipoleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = DipoleAtInfinity;

    fn expansion(self, other: PlaneAtOrigin) -> DipoleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointBulk> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for RoundPointAtInfinity {
    type Output = Dipole;

    fn expansion(self, other: SpacialCurvature) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPointAtInfinity {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for RoundPointAtInfinity {
    type Output = Dipole;

    fn expansion(self, other: SphereWeight) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleAtInfinity) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = Circle;

    fn expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleWeight) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPointAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleWeight) -> PlaneAtOrigin {
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
    type Output = SpacialCurvature;

    fn expansion(self, other: FlatPointAtOrigin) -> SpacialCurvature {
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
    type Output = Circle;

    fn expansion(self, other: Line) -> Circle {
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
    type Output = Circle;

    fn expansion(self, other: LineAtOrigin) -> Circle {
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

impl Expansion<Origin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for RoundPointAtOrigin {
    type Output = Dipole;

    fn expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = Dipole;

    fn expansion(self, other: PlaneAtOrigin) -> Dipole {
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

impl Expansion<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPointAtOrigin {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn expansion(self, other: SphereWeight) -> FlatPointAtOrigin {
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

impl Expansion<Circle> for RoundPointBulk {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for RoundPointBulk {
    type Output = Line;

    fn expansion(self, other: CircleAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for RoundPointBulk {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleBulk) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for RoundPointBulk {
    type Output = Circle;

    fn expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for RoundPointBulk {
    type Output = CircleWeight;

    fn expansion(self, other: CircleWeight) -> CircleWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPointBulk {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for RoundPointBulk {
    type Output = Plane;

    fn expansion(self, other: DipoleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for RoundPointBulk {
    type Output = Sphere;

    fn expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for RoundPointBulk {
    type Output = SphereWeight;

    fn expansion(self, other: DipoleWeight) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for RoundPointBulk {
    type Output = Horizon;

    fn expansion(self, other: FlatPoint) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for RoundPointBulk {
    type Output = Horizon;

    fn expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for RoundPointBulk {
    type Output = FlatPointAtInfinity;

    fn expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for RoundPointBulk {
    type Output = CircleAtInfinity;

    fn expansion(self, other: Line) -> CircleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for RoundPointBulk {
    type Output = LineAtInfinity;

    fn expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for RoundPointBulk {
    type Output = CircleBulk;

    fn expansion(self, other: LineAtOrigin) -> CircleBulk {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for RoundPointBulk {
    type Output = DipoleAtInfinity;

    fn expansion(self, other: Plane) -> DipoleAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for RoundPointBulk {
    type Output = DipoleBulk;

    fn expansion(self, other: PlaneAtOrigin) -> DipoleBulk {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for RoundPointBulk {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtInfinity> for RoundPointBulk {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointBulk> for RoundPointBulk {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointOnOrigin> for RoundPointBulk {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for RoundPointBulk {
    type Output = Dipole;

    fn expansion(self, other: SpacialCurvature) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPointBulk {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for RoundPointBulk {
    type Output = DipoleWeight;

    fn expansion(self, other: SphereWeight) -> DipoleWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for RoundPointOnOrigin {
    type Output = Circle;

    fn expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleAtInfinity> for RoundPointOnOrigin {
    type Output = Line;

    fn expansion(self, other: CircleAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleBulk> for RoundPointOnOrigin {
    type Output = LineAtOrigin;

    fn expansion(self, other: CircleBulk) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for RoundPointOnOrigin {
    type Output = Circle;

    fn expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for RoundPointOnOrigin {
    type Output = CircleWeight;

    fn expansion(self, other: CircleWeight) -> CircleWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for RoundPointOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleAtInfinity> for RoundPointOnOrigin {
    type Output = Plane;

    fn expansion(self, other: DipoleAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleBulk> for RoundPointOnOrigin {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for RoundPointOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for RoundPointOnOrigin {
    type Output = SphereWeight;

    fn expansion(self, other: DipoleWeight) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPoint> for RoundPointOnOrigin {
    type Output = Sphere;

    fn expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtInfinity> for RoundPointOnOrigin {
    type Output = Plane;

    fn expansion(self, other: FlatPointAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlatPointAtOrigin> for RoundPointOnOrigin {
    type Output = SphereWeight;

    fn expansion(self, other: FlatPointAtOrigin) -> SphereWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for RoundPointOnOrigin {
    type Output = FlatPoint;

    fn expansion(self, other: Horizon) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Infinity> for RoundPointOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Line> for RoundPointOnOrigin {
    type Output = Circle;

    fn expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtInfinity> for RoundPointOnOrigin {
    type Output = Line;

    fn expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<LineAtOrigin> for RoundPointOnOrigin {
    type Output = CircleCarrierAspect;

    fn expansion(self, other: LineAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for RoundPointOnOrigin {
    type Output = Dipole;

    fn expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<PlaneAtOrigin> for RoundPointOnOrigin {
    type Output = DipoleCarrierAspect;

    fn expansion(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPoint> for RoundPointOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointBulk> for RoundPointOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointBulk) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = AntiScalar;

    fn expansion(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for RoundPointOnOrigin {
    type Output = Dipole;

    fn expansion(self, other: SpacialCurvature) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for RoundPointOnOrigin {
    type Output = Dipole;

    fn expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for RoundPointOnOrigin {
    type Output = DipoleWeight;

    fn expansion(self, other: SphereWeight) -> DipoleWeight {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Flector> for SpacialCurvature {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for SpacialCurvature {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for SpacialCurvature {
    type Output = AntiScalar;

    fn expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for SpacialCurvature {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for SpacialCurvature {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for SpacialCurvature {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for SpacialCurvature {
    type Output = AntiScalar;

    fn expansion(self, other: SpacialCurvature) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for SpacialCurvature {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for SpacialCurvature {
    type Output = AntiScalar;

    fn expansion(self, other: SphereWeight) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for SpacialCurvature {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for SpacialCurvature {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
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

impl Expansion<SpacialCurvature> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: SpacialCurvature) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Sphere {
    type Output = AntiScalar;

    fn expansion(self, other: SphereWeight) -> AntiScalar {
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

impl Expansion<Flector> for SphereWeight {
    type Output = MultiVector;

    fn expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Horizon> for SphereWeight {
    type Output = AntiScalar;

    fn expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Motor> for SphereWeight {
    type Output = MultiVector;

    fn expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Plane> for SphereWeight {
    type Output = AntiScalar;

    fn expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SpacialCurvature> for SphereWeight {
    type Output = AntiScalar;

    fn expansion(self, other: SpacialCurvature) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for SphereWeight {
    type Output = AntiScalar;

    fn expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Translator> for SphereWeight {
    type Output = MultiVector;

    fn expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Circle> for Transflector {
    type Output = Plane;

    fn expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Transflector {
    type Output = Plane;

    fn expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Transflector {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: CircleWeight) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Dipole> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleCarrierAspect> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<DipoleWeight> for Transflector {
    type Output = AntiScalar;

    fn expansion(self, other: DipoleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for Transflector {
    type Output = Rotor;

    fn expansion(self, other: SpacialCurvature) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Transflector {
    type Output = Motor;

    fn expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Transflector {
    type Output = Rotor;

    fn expansion(self, other: SphereWeight) -> Rotor {
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

impl Expansion<Circle> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleCarrierAspect> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<CircleWeight> for Translator {
    type Output = AntiScalar;

    fn expansion(self, other: CircleWeight) -> AntiScalar {
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

impl Expansion<SpacialCurvature> for Translator {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SpacialCurvature) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<Sphere> for Translator {
    type Output = Plane;

    fn expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl Expansion<SphereWeight> for Translator {
    type Output = PlaneAtOrigin;

    fn expansion(self, other: SphereWeight) -> PlaneAtOrigin {
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
