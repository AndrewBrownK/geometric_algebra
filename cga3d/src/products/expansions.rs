//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspect_duals::*;
use crate::products::exterior::Wedge;
use crate::*;

/// Bulk Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}

/// Weight Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
}

impl BulkExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Horizon) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for CircleBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for CircleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Horizon) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for CircleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Plane) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for CircleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Sphere) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Horizon) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Plane) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Sphere) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for CircleWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Horizon) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Line) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: LineAtInfinity) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Motor) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Plane) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Translator) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for DipoleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Circle) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for DipoleBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Horizon) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for DipoleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Line) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for DipoleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: LineAtInfinity) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for DipoleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Motor) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for DipoleBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Plane) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for DipoleBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Sphere) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for DipoleBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Translator) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Circle) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for DipoleCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Horizon) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Line) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: LineAtInfinity) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Motor) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for DipoleCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Plane) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for DipoleCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Sphere) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Translator) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for FlatPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for FlatPoint {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for FlatPoint {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for FlatPoint {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for FlatPoint {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for FlatPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for FlatPointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for FlatPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Horizon) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Plane) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Sphere) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for FlectorAtInfinity {
    type Output = Rotor;

    fn bulk_expansion(self, other: Horizon) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for FlectorAtInfinity {
    type Output = Rotor;

    fn bulk_expansion(self, other: Plane) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for FlectorAtInfinity {
    type Output = Rotor;

    fn bulk_expansion(self, other: Sphere) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Infinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Circle) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Infinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Dipole) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for Infinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlatPoint) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for Infinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Infinity {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Infinity> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Infinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Line) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Infinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Infinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Motor) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Infinity {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Plane) -> FlatPointAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPoint> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Infinity {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> FlatPointAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Infinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Translator) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlatPoint) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Horizon) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Infinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Infinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Line) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPoint) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Horizon) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Infinity> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: LineAtInfinity) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Motor) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Translator) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointAtInfinity {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointAtInfinity {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = Sphere;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointAtInfinity {
    type Output = Dipole;

    fn bulk_expansion(self, other: Horizon) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Infinity> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: LineAtInfinity) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: Motor) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for RoundPointAtInfinity {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointAtInfinity {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for RoundPointAtInfinity {
    type Output = Circle;

    fn bulk_expansion(self, other: Translator) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Circle) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Dipole) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlatPoint) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> FlatPointAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Infinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Line) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Motor) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Plane) -> FlatPointAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> FlatPointAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for RoundPointAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Translator) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for RoundPointBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Circle) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Dipole) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: FlatPoint) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointBulk {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointBulk {
    type Output = DipoleWeight;

    fn bulk_expansion(self, other: Horizon) -> DipoleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for RoundPointBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Line) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: LineAtInfinity) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for RoundPointBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Motor) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for RoundPointBulk {
    type Output = DipoleWeight;

    fn bulk_expansion(self, other: Plane) -> DipoleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointBulk {
    type Output = DipoleWeight;

    fn bulk_expansion(self, other: Sphere) -> DipoleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for RoundPointBulk {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Translator) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for RoundPointCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Circle) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for RoundPointCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: Dipole) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPoint> for RoundPointCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: FlatPoint) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = SphereWeight;

    fn bulk_expansion(self, other: FlatPointAtInfinity) -> SphereWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for RoundPointCarrierAspect {
    type Output = DipoleWeight;

    fn bulk_expansion(self, other: Horizon) -> DipoleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for RoundPointCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Line) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: LineAtInfinity) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for RoundPointCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Motor) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for RoundPointCarrierAspect {
    type Output = DipoleWeight;

    fn bulk_expansion(self, other: Plane) -> DipoleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for RoundPointCarrierAspect {
    type Output = DipoleWeight;

    fn bulk_expansion(self, other: Sphere) -> DipoleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for RoundPointCarrierAspect {
    type Output = CircleWeight;

    fn bulk_expansion(self, other: Translator) -> CircleWeight {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for SphereWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl WeightExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Circle {
    type Output = Circle;

    fn weight_expansion(self, other: Translator) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for CircleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for CircleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for CircleBulk {
    type Output = CircleBulk;

    fn weight_expansion(self, other: Translator) -> CircleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Plane) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: PlaneAtOrigin) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for CircleCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Sphere) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: Translator) -> CircleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for CircleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for CircleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for CircleWeight {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Plane) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for CircleWeight {
    type Output = SphereWeight;

    fn weight_expansion(self, other: PlaneAtOrigin) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for CircleWeight {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Sphere) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for CircleWeight {
    type Output = CircleWeight;

    fn weight_expansion(self, other: Translator) -> CircleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: Line) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: LineAtOrigin) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Plane) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Dipole {
    type Output = Dipole;

    fn weight_expansion(self, other: Translator) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for DipoleBulk {
    type Output = CircleBulk;

    fn weight_expansion(self, other: Plane) -> CircleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for DipoleBulk {
    type Output = CircleBulk;

    fn weight_expansion(self, other: PlaneAtOrigin) -> CircleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for DipoleBulk {
    type Output = CircleBulk;

    fn weight_expansion(self, other: Sphere) -> CircleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for DipoleBulk {
    type Output = DipoleBulk;

    fn weight_expansion(self, other: Translator) -> DipoleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Circle) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Line) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for DipoleCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: LineAtOrigin) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for DipoleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: Plane) -> CircleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for DipoleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: Sphere) -> CircleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_expansion(self, other: Translator) -> DipoleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for DipoleWeight {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Circle) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for DipoleWeight {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Line) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for DipoleWeight {
    type Output = SphereWeight;

    fn weight_expansion(self, other: LineAtOrigin) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for DipoleWeight {
    type Output = CircleWeight;

    fn weight_expansion(self, other: Plane) -> CircleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for DipoleWeight {
    type Output = CircleWeight;

    fn weight_expansion(self, other: PlaneAtOrigin) -> CircleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for DipoleWeight {
    type Output = CircleWeight;

    fn weight_expansion(self, other: Sphere) -> CircleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for DipoleWeight {
    type Output = DipoleWeight;

    fn weight_expansion(self, other: Translator) -> DipoleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for FlatPoint {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for FlatPoint {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for FlatPoint {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlatPoint {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for FlatPoint {
    type Output = Line;

    fn weight_expansion(self, other: Plane) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlatPoint {
    type Output = Line;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for FlatPoint {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for FlatPoint {
    type Output = FlatPoint;

    fn weight_expansion(self, other: Translator) -> FlatPoint {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for FlatPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Circle) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for FlatPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Line) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlatPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Sphere) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: Translator) -> FlatPointAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlatPointAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for FlatPointAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight_expansion(self, other: Translator) -> FlatPointAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPoint) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Plane) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Flector {
    type Output = Flector;

    fn weight_expansion(self, other: Translator) -> Flector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for FlectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Circle) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for FlectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Line) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Sphere) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn weight_expansion(self, other: Translator) -> FlectorAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Horizon {
    type Output = Horizon;

    fn weight_expansion(self, other: Translator) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Infinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Circle) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Infinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Dipole) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for Infinity {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPoint) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Infinity {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Infinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Line) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Infinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Infinity {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: Plane) -> FlatPointAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Infinity {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Infinity {
    type Output = FlatPointAtInfinity;

    fn weight_expansion(self, other: Sphere) -> FlatPointAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Infinity {
    type Output = Infinity;

    fn weight_expansion(self, other: Translator) -> Infinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Line {
    type Output = Line;

    fn weight_expansion(self, other: Translator) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Sphere) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Translator) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Translator) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Motor {
    type Output = Motor;

    fn weight_expansion(self, other: Translator) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Dipole) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlatPoint) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Line) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Origin {
    type Output = CircleWeight;

    fn weight_expansion(self, other: Circle) -> CircleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Origin {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Dipole) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for Origin {
    type Output = SphereWeight;

    fn weight_expansion(self, other: FlatPoint) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for Origin {
    type Output = SphereWeight;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Origin {
    type Output = CircleWeight;

    fn weight_expansion(self, other: Line) -> CircleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Origin {
    type Output = CircleWeight;

    fn weight_expansion(self, other: LineAtOrigin) -> CircleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Origin {
    type Output = DipoleWeight;

    fn weight_expansion(self, other: Plane) -> DipoleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Origin {
    type Output = DipoleWeight;

    fn weight_expansion(self, other: PlaneAtOrigin) -> DipoleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Origin {
    type Output = DipoleWeight;

    fn weight_expansion(self, other: Sphere) -> DipoleWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Origin {
    type Output = Origin;

    fn weight_expansion(self, other: Translator) -> Origin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Plane {
    type Output = Plane;

    fn weight_expansion(self, other: Translator) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Rotor {
    type Output = Rotor;

    fn weight_expansion(self, other: Translator) -> Rotor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for RoundPoint {
    type Output = RoundPoint;

    fn weight_expansion(self, other: Translator) -> RoundPoint {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Dipole) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPoint) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointAtInfinity {
    type Output = Circle;

    fn weight_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointAtInfinity {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = Dipole;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointAtInfinity {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_expansion(self, other: Translator) -> RoundPointAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPointAtOrigin {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPointAtOrigin {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPoint) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = Sphere;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for RoundPointAtOrigin {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointAtOrigin {
    type Output = Circle;

    fn weight_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointAtOrigin {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = Dipole;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointAtOrigin {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn weight_expansion(self, other: Translator) -> RoundPointAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointBulk {
    type Output = CircleBulk;

    fn weight_expansion(self, other: Circle) -> CircleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for RoundPointBulk {
    type Output = CircleBulk;

    fn weight_expansion(self, other: Line) -> CircleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointBulk {
    type Output = CircleBulk;

    fn weight_expansion(self, other: LineAtOrigin) -> CircleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointBulk {
    type Output = DipoleBulk;

    fn weight_expansion(self, other: Plane) -> DipoleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointBulk {
    type Output = DipoleBulk;

    fn weight_expansion(self, other: PlaneAtOrigin) -> DipoleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointBulk {
    type Output = DipoleBulk;

    fn weight_expansion(self, other: Sphere) -> DipoleBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn weight_expansion(self, other: Translator) -> RoundPointBulk {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: Circle) -> CircleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPointCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Dipole) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPoint> for RoundPointCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: FlatPoint) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = SphereWeight;

    fn weight_expansion(self, other: FlatPointAtOrigin) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: Line) -> CircleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn weight_expansion(self, other: LineAtOrigin) -> CircleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_expansion(self, other: Plane) -> DipoleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_expansion(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_expansion(self, other: Sphere) -> DipoleCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_expansion(self, other: Translator) -> RoundPointCarrierAspect {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Sphere {
    type Output = Sphere;

    fn weight_expansion(self, other: Translator) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for SphereWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for SphereWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for SphereWeight {
    type Output = SphereWeight;

    fn weight_expansion(self, other: Translator) -> SphereWeight {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: Sphere) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Translator {
    type Output = Translator;

    fn weight_expansion(self, other: Translator) -> Translator {
        self.wedge(other.right_weight_dual())
    }
}
