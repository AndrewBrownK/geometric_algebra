//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::exterior::Wedge;
use crate::*;

///
/// Bulk Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
///
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}

///
/// Weight Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
///
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
}

impl BulkExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Circle {
    type Output = Plane;

    fn bulk_expansion(self, other: Horizon) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: SphereWeightAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for CircleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for CircleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for CircleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for CircleBulkAspect {
    type Output = Horizon;

    fn bulk_expansion(self, other: Horizon) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for CircleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for CircleBulkAspect {
    type Output = Horizon;

    fn bulk_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for CircleBulkAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for CircleBulkAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for CircleCarrierAspect {
    type Output = Plane;

    fn bulk_expansion(self, other: Horizon) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for CircleCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for CircleCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for CircleCarrierAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for CircleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for CircleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for CircleWeightAspect {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for CircleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for CircleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for CircleWeightAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for CircleWeightAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for CircleWeightAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for Dipole {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: CircleWeightAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Dipole {
    type Output = Line;

    fn bulk_expansion(self, other: Horizon) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Line) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Dipole {
    type Output = Plane;

    fn bulk_expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: LineAtOrigin) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Plane) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: SphereWeightAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for DipoleBulkAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for DipoleBulkAspect {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for DipoleBulkAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for DipoleBulkAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: CircleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for DipoleBulkAspect {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: Horizon) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for DipoleBulkAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Line) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for DipoleBulkAspect {
    type Output = Horizon;

    fn bulk_expansion(self, other: LineAtInfinity) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for DipoleBulkAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Plane) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for DipoleBulkAspect {
    type Output = CircleBulkAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> CircleBulkAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for DipoleBulkAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for DipoleBulkAspect {
    type Output = CircleWeightAspect;

    fn bulk_expansion(self, other: SphereWeightAspect) -> CircleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for DipoleCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for DipoleCarrierAspect {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for DipoleCarrierAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: CircleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for DipoleCarrierAspect {
    type Output = Line;

    fn bulk_expansion(self, other: Horizon) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for DipoleCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Line) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for DipoleCarrierAspect {
    type Output = Plane;

    fn bulk_expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for DipoleCarrierAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: LineAtOrigin) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for DipoleCarrierAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Plane) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for DipoleCarrierAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for DipoleCarrierAspect {
    type Output = CircleWeightAspect;

    fn bulk_expansion(self, other: SphereWeightAspect) -> CircleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for DipoleWeightAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for DipoleWeightAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for DipoleWeightAspect {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for DipoleWeightAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Line) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for DipoleWeightAspect {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for DipoleWeightAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: LineAtOrigin) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for DipoleWeightAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Plane) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for DipoleWeightAspect {
    type Output = CircleWeightAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for DipoleWeightAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for FlatPoint {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for FlatPoint {
    type Output = Plane;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for FlatPoint {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for FlatPoint {
    type Output = Plane;

    fn bulk_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for FlatPoint {
    type Output = Line;

    fn bulk_expansion(self, other: Plane) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for FlatPoint {
    type Output = Line;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for FlatPoint {
    type Output = Line;

    fn bulk_expansion(self, other: Sphere) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for FlatPoint {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for FlatPointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for FlatPointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for FlatPointAtInfinity {
    type Output = Horizon;

    fn bulk_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for FlatPointAtInfinity {
    type Output = Line;

    fn bulk_expansion(self, other: Sphere) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: Plane) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: SphereWeightAspect) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for FlectorAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for FlectorAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn bulk_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for FlectorAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for FlectorAtInfinity {
    type Output = Rotor;

    fn bulk_expansion(self, other: SphereWeightAspect) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: SphereWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: Circle) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Infinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: CircleWeightAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for Infinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Dipole) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for Infinity {
    type Output = Plane;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for Infinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for Infinity {
    type Output = Plane;

    fn bulk_expansion(self, other: FlatPoint) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for Infinity {
    type Output = Horizon;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: Line) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Origin> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Infinity {
    type Output = FlatPointAtInfinity;

    fn bulk_expansion(self, other: Plane) -> FlatPointAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Infinity {
    type Output = FlatPoint;

    fn bulk_expansion(self, other: Sphere) -> FlatPoint {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Infinity {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> FlatPointAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: Plane) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for LineAtInfinity {
    type Output = Horizon;

    fn bulk_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for LineAtInfinity {
    type Output = Horizon;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: Plane) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: CircleBulkAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: CircleWeightAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlatPoint) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Horizon) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Infinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Infinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Line) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Origin> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Origin) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPoint) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPointBulkAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: SphereWeightAspect) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Origin {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Origin {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for Origin {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for Origin {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for Origin {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for Origin {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Origin {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Infinity> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Origin {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Origin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Origin {
    type Output = CircleWeightAspect;

    fn bulk_expansion(self, other: LineAtOrigin) -> CircleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Origin {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Origin {
    type Output = DipoleWeightAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> DipoleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Origin {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: SphereWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for RoundPoint {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleWeightAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for RoundPoint {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPoint {
    type Output = Plane;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for RoundPoint {
    type Output = FlatPoint;

    fn bulk_expansion(self, other: Horizon) -> FlatPoint {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Infinity> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPoint {
    type Output = Line;

    fn bulk_expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Origin> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointBulkAspect> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: SphereWeightAspect) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for RoundPointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleWeightAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointAtInfinity {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for RoundPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for RoundPointAtInfinity {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointAtInfinity {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = Horizon;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Horizon;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk_expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Origin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for RoundPointAtInfinity {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = Dipole;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointAtInfinity {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for RoundPointAtInfinity {
    type Output = Dipole;

    fn bulk_expansion(self, other: SphereWeightAspect) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: CircleWeightAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointAtOrigin {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointAtOrigin {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Infinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for RoundPointAtOrigin {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for RoundPointAtOrigin {
    type Output = Circle;

    fn bulk_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Origin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for RoundPointAtOrigin {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = Dipole;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointAtOrigin {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> FlatPointAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for RoundPointBulkAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for RoundPointBulkAspect {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for RoundPointBulkAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for RoundPointBulkAspect {
    type Output = CircleWeightAspect;

    fn bulk_expansion(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointBulkAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for RoundPointBulkAspect {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for RoundPointBulkAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for RoundPointBulkAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointBulkAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointBulkAspect {
    type Output = Horizon;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointBulkAspect {
    type Output = FlatPointAtInfinity;

    fn bulk_expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for RoundPointBulkAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointBulkAspect {
    type Output = LineAtInfinity;

    fn bulk_expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for RoundPointBulkAspect {
    type Output = CircleBulkAspect;

    fn bulk_expansion(self, other: LineAtOrigin) -> CircleBulkAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for RoundPointBulkAspect {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for RoundPointBulkAspect {
    type Output = DipoleBulkAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> DipoleBulkAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointBulkAspect {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for RoundPointBulkAspect {
    type Output = DipoleWeightAspect;

    fn bulk_expansion(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for RoundPointCarrierAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleBulkAspect> for RoundPointCarrierAspect {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for RoundPointCarrierAspect {
    type Output = CircleWeightAspect;

    fn bulk_expansion(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleBulkAspect> for RoundPointCarrierAspect {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for RoundPointCarrierAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Plane;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = SphereWeightAspect;

    fn bulk_expansion(self, other: FlatPointAtOrigin) -> SphereWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointCarrierAspect {
    type Output = FlatPoint;

    fn bulk_expansion(self, other: Horizon) -> FlatPoint {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Infinity> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for RoundPointCarrierAspect {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Line;

    fn bulk_expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn bulk_expansion(self, other: LineAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for RoundPointCarrierAspect {
    type Output = DipoleWeightAspect;

    fn bulk_expansion(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: SphereWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for SphereWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for SphereWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for SphereWeightAspect {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Transflector {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Transflector {
    type Output = Plane;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Transflector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Dipole> for Transflector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleCarrierAspect> for Transflector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<DipoleWeightAspect> for Transflector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlatPoint> for Transflector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Transflector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Transflector {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtOrigin> for Transflector {
    type Output = Horizon;

    fn bulk_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Transflector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Transflector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Transflector {
    type Output = Translator;

    fn bulk_expansion(self, other: Plane) -> Translator {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Transflector {
    type Output = Translator;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Translator {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Transflector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Transflector {
    type Output = Motor;

    fn bulk_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Transflector {
    type Output = Rotor;

    fn bulk_expansion(self, other: SphereWeightAspect) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Transflector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Transflector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Circle> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleCarrierAspect> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<CircleWeightAspect> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Translator {
    type Output = Horizon;

    fn bulk_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PlaneAtOrigin> for Translator {
    type Output = Horizon;

    fn bulk_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Rotor> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Sphere> for Translator {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<SphereWeightAspect> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl WeightExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for Circle {
    type Output = Plane;

    fn weight_expansion(self, other: Horizon) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: SphereWeightAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for CircleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for CircleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for CircleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for CircleBulkAspect {
    type Output = Horizon;

    fn weight_expansion(self, other: Horizon) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for CircleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for CircleBulkAspect {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for CircleBulkAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for CircleBulkAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for CircleCarrierAspect {
    type Output = Plane;

    fn weight_expansion(self, other: Horizon) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for CircleCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for CircleCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for CircleCarrierAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: SphereWeightAspect) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for CircleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for CircleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for CircleWeightAspect {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for CircleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for CircleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for CircleWeightAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for CircleWeightAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for CircleWeightAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for Dipole {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: CircleWeightAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for Dipole {
    type Output = Line;

    fn weight_expansion(self, other: Horizon) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for Dipole {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: LineAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: SphereWeightAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for DipoleBulkAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for DipoleBulkAspect {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for DipoleBulkAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for DipoleBulkAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: CircleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for DipoleBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for DipoleBulkAspect {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Horizon) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for DipoleBulkAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for DipoleBulkAspect {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for DipoleBulkAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for DipoleBulkAspect {
    type Output = CircleBulkAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> CircleBulkAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for DipoleBulkAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for DipoleBulkAspect {
    type Output = CircleWeightAspect;

    fn weight_expansion(self, other: SphereWeightAspect) -> CircleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for DipoleCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for DipoleCarrierAspect {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for DipoleCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for DipoleCarrierAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: CircleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for DipoleCarrierAspect {
    type Output = Line;

    fn weight_expansion(self, other: Horizon) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for DipoleCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for DipoleCarrierAspect {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for DipoleCarrierAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: LineAtOrigin) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for DipoleCarrierAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for DipoleCarrierAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for DipoleCarrierAspect {
    type Output = CircleWeightAspect;

    fn weight_expansion(self, other: SphereWeightAspect) -> CircleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for DipoleWeightAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for DipoleWeightAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for DipoleWeightAspect {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for DipoleWeightAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Line) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for DipoleWeightAspect {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for DipoleWeightAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: LineAtOrigin) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for DipoleWeightAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Plane) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for DipoleWeightAspect {
    type Output = CircleWeightAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> CircleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for DipoleWeightAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for FlatPoint {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for FlatPoint {
    type Output = Plane;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for FlatPoint {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlatPoint {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for FlatPoint {
    type Output = Line;

    fn weight_expansion(self, other: Plane) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlatPoint {
    type Output = Line;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for FlatPoint {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for FlatPoint {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for FlatPointAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for FlatPointAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlatPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for FlatPointAtInfinity {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleCarrierAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Flector {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Plane) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Flector {
    type Output = Rotor;

    fn weight_expansion(self, other: SphereWeightAspect) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for FlectorAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for FlectorAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for FlectorAtInfinity {
    type Output = Motor;

    fn weight_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for FlectorAtInfinity {
    type Output = Rotor;

    fn weight_expansion(self, other: SphereWeightAspect) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: SphereWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Infinity {
    type Output = Line;

    fn weight_expansion(self, other: Circle) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Infinity {
    type Output = Line;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Infinity {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: CircleWeightAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for Infinity {
    type Output = Plane;

    fn weight_expansion(self, other: Dipole) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for Infinity {
    type Output = Plane;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for Infinity {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: DipoleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for Infinity {
    type Output = Plane;

    fn weight_expansion(self, other: FlatPoint) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Infinity {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Infinity {
    type Output = Line;

    fn weight_expansion(self, other: Line) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for Infinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Infinity {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: Plane) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for Infinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for Infinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for Infinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Infinity {
    type Output = FlatPoint;

    fn weight_expansion(self, other: Sphere) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Infinity {
    type Output = FlatPointAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Line {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Motor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: CircleBulkAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: CircleCarrierAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: CircleWeightAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: DipoleBulkAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: DipoleWeightAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlatPoint) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Horizon) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Infinity> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Infinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Line) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Origin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: RoundPoint) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: RoundPointBulkAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: SphereWeightAspect) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Origin {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Origin {
    type Output = Circle;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for Origin {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for Origin {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for Origin {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Origin {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for Origin {
    type Output = FlatPointAtOrigin;

    fn weight_expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Infinity> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Origin {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Origin {
    type Output = CircleWeightAspect;

    fn weight_expansion(self, other: LineAtOrigin) -> CircleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Origin {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Origin {
    type Output = DipoleWeightAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> DipoleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtInfinity> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Origin {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: SphereWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for RoundPoint {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: CircleWeightAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for RoundPoint {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleWeightAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for RoundPoint {
    type Output = Plane;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for RoundPoint {
    type Output = FlatPoint;

    fn weight_expansion(self, other: Horizon) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Infinity> for RoundPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for RoundPoint {
    type Output = Line;

    fn weight_expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for RoundPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtInfinity> for RoundPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointBulkAspect> for RoundPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for RoundPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: SphereWeightAspect) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for RoundPointAtInfinity {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: CircleWeightAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPointAtInfinity {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for RoundPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for RoundPointAtInfinity {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleWeightAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPointAtInfinity {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointAtInfinity {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = Dipole;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointAtInfinity {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for RoundPointAtInfinity {
    type Output = Dipole;

    fn weight_expansion(self, other: SphereWeightAspect) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = Circle;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: CircleWeightAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPointAtOrigin {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: DipoleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPointAtOrigin {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight_expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Infinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for RoundPointAtOrigin {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointAtOrigin {
    type Output = Circle;

    fn weight_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointAtOrigin {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = Dipole;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointAtOrigin {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> FlatPointAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointBulkAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for RoundPointBulkAspect {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for RoundPointBulkAspect {
    type Output = Circle;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for RoundPointBulkAspect {
    type Output = CircleWeightAspect;

    fn weight_expansion(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPointBulkAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for RoundPointBulkAspect {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for RoundPointBulkAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for RoundPointBulkAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: DipoleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPointBulkAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for RoundPointBulkAspect {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for RoundPointBulkAspect {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: Horizon) -> FlatPointAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for RoundPointBulkAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for RoundPointBulkAspect {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointBulkAspect {
    type Output = CircleBulkAspect;

    fn weight_expansion(self, other: LineAtOrigin) -> CircleBulkAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointBulkAspect {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointBulkAspect {
    type Output = DipoleBulkAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> DipoleBulkAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointBulkAspect {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for RoundPointBulkAspect {
    type Output = DipoleWeightAspect;

    fn weight_expansion(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointCarrierAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleBulkAspect> for RoundPointCarrierAspect {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: CircleBulkAspect) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = Circle;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for RoundPointCarrierAspect {
    type Output = CircleWeightAspect;

    fn weight_expansion(self, other: CircleWeightAspect) -> CircleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleBulkAspect> for RoundPointCarrierAspect {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: DipoleBulkAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for RoundPointCarrierAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: DipoleWeightAspect) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPointCarrierAspect {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Plane;

    fn weight_expansion(self, other: FlatPointAtInfinity) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = SphereWeightAspect;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> SphereWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for RoundPointCarrierAspect {
    type Output = FlatPoint;

    fn weight_expansion(self, other: Horizon) -> FlatPoint {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Infinity> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for RoundPointCarrierAspect {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Line;

    fn weight_expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: LineAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPoint> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointBulkAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: RoundPointCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for RoundPointCarrierAspect {
    type Output = DipoleWeightAspect;

    fn weight_expansion(self, other: SphereWeightAspect) -> DipoleWeightAspect {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: SphereWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Horizon> for SphereWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for SphereWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for SphereWeightAspect {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Transflector {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Transflector {
    type Output = Plane;

    fn weight_expansion(self, other: CircleCarrierAspect) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Transflector {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: CircleWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Dipole> for Transflector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleCarrierAspect> for Transflector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<DipoleWeightAspect> for Transflector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: DipoleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlatPoint> for Transflector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Transflector {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Transflector {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Transflector {
    type Output = Translator;

    fn weight_expansion(self, other: Plane) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Transflector {
    type Output = Translator;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Transflector {
    type Output = Motor;

    fn weight_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Transflector {
    type Output = Rotor;

    fn weight_expansion(self, other: SphereWeightAspect) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Circle> for Translator {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleCarrierAspect> for Translator {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleCarrierAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<CircleWeightAspect> for Translator {
    type Output = AntiScalar;

    fn weight_expansion(self, other: CircleWeightAspect) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Translator {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Sphere> for Translator {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<SphereWeightAspect> for Translator {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: SphereWeightAspect) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Translator> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}
