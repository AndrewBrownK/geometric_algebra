//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspect_duals::*;
use crate::products::exterior::AntiWedge;
use crate::*;

/// Bulk Contraction (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait BulkContraction<T> {
    type Output;
    fn bulk_contraction(self, other: T) -> Self::Output;
}

/// Weight Contraction (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait WeightContraction<T> {
    type Output;
    fn weight_contraction(self, other: T) -> Self::Output;
}

impl BulkContraction<Circle> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Circle {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Circle {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Circle {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Circle {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Infinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Circle {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Circle {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Circle {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Dipole {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Infinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Dipole {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Dipole {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Dipole {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Infinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Infinity) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for FlatPointAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Infinity) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for FlatPointAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: RoundPoint) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Flector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Circle) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Flector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Line) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Flector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Flector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Motor) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Flector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Translator) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Horizon {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Circle) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Horizon {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: Dipole) -> DipoleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Horizon {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: FlatPoint) -> DipoleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Horizon {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Horizon {
    type Output = CircleBulk;

    fn bulk_contraction(self, other: Infinity) -> CircleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Horizon {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Line) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Horizon {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Horizon {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Motor) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Horizon {
    type Output = CircleBulk;

    fn bulk_contraction(self, other: RoundPoint) -> CircleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Horizon {
    type Output = CircleBulk;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> CircleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Horizon {
    type Output = CircleBulk;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Horizon {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Translator) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Line {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Line {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Line {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Line {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Infinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Line {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Line {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Line {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for LineAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Dipole) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for LineAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for LineAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for LineAtInfinity {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: Infinity) -> DipoleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for LineAtInfinity {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for LineAtInfinity {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for LineAtInfinity {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Dipole) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: FlatPoint) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for LineAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: Infinity) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for LineAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for LineAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for LineAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: Plane) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: Sphere) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Plane) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Plane {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Circle) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Plane {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Plane {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Plane {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Plane {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: Infinity) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Plane {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Line) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Plane {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Plane {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Motor) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Plane {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Plane {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Plane {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Plane {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Translator) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Circle) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: Dipole) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: FlatPoint) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn bulk_contraction(self, other: Infinity) -> CircleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Line) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: LineAtInfinity) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Motor) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn bulk_contraction(self, other: RoundPoint) -> CircleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> CircleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> CircleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Translator) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Rotor {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: Circle) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Rotor {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: Line) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Rotor {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: LineAtInfinity) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Rotor {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: Motor) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: Plane) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: Sphere) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Rotor {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: Translator) -> DipoleWeight {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Circle) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Sphere {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Sphere {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Sphere {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Sphere {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: Infinity) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Line) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Motor) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Sphere {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Sphere {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Sphere {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Translator) -> RoundPointCarrierAspect {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPoint> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: Plane) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: Sphere) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl WeightContraction<Circle> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Circle {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Dipole) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Circle {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Circle {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for FlatPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for FlatPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Flector {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Circle) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Flector {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Line) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Flector {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Line {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Dipole) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Line {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Line {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Dipole) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Motor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Plane) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Motor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Motor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Sphere) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Motor {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Plane) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for MultiVector {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Plane {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Circle) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Plane {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Dipole) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Plane {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPoint) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Plane {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Plane {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Line) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Plane {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Circle) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Dipole) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPoint) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Line) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Rotor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Plane) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Rotor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Rotor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Sphere) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Rotor {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Sphere {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Circle) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Sphere {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Dipole) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Sphere {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPoint) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Sphere {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Sphere {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Line) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Sphere {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Translator {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Circle) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Translator {
    type Output = CircleBulk;

    fn weight_contraction(self, other: Dipole) -> CircleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPoint> for Translator {
    type Output = CircleBulk;

    fn weight_contraction(self, other: FlatPoint) -> CircleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Translator {
    type Output = CircleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Translator {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Line) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Translator {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Translator {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Plane) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Translator {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Translator {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Sphere) -> RoundPointBulk {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Translator {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}
