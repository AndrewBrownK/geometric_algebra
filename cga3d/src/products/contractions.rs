//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
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
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: DipoleBulk) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Circle {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Circle {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Circle {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: Infinity) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Circle {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Origin) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointBulk) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for CircleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for CircleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for CircleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for CircleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Dipole) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for CircleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: DipoleBulk) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for CircleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for CircleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for CircleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for CircleBulk {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for CircleBulk {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for CircleBulk {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for CircleBulk {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPointBulk) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for CircleBulk {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for CircleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleBulk) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for CircleCarrierAspect {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: Infinity) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointBulk) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for CircleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for CircleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for CircleWeight {
    type Output = Origin;

    fn bulk_contraction(self, other: DipoleBulk) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for CircleWeight {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for CircleWeight {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: Infinity) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for CircleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for CircleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for CircleWeight {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: RoundPointBulk) -> DipoleWeight {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for CircleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Dipole {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Dipole {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Origin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPointBulk) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for DipoleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for DipoleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for DipoleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for DipoleBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for DipoleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for DipoleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for DipoleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for DipoleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPointBulk) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for DipoleBulk {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for DipoleCarrierAspect {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Infinity) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for DipoleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for DipoleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for DipoleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for DipoleWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for DipoleWeight {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Infinity) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for DipoleWeight {
    type Output = Origin;

    fn bulk_contraction(self, other: RoundPointBulk) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for FlatPoint {
    type Output = Infinity;

    fn bulk_contraction(self, other: Infinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Origin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for FlatPoint {
    type Output = Infinity;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for FlatPoint {
    type Output = Infinity;

    fn bulk_contraction(self, other: RoundPointBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for FlatPoint {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for FlatPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: Origin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for FlatPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = Infinity;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for FlatPointAtInfinity {
    type Output = Infinity;

    fn bulk_contraction(self, other: RoundPointBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for FlatPointAtOrigin {
    type Output = Infinity;

    fn bulk_contraction(self, other: Infinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for FlatPointAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Origin) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = Infinity;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Flector {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Flector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Flector {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Flector {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Flector {
    type Output = Infinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Flector {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Circle) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for FlectorAtInfinity {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for FlectorAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: CircleWeight) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Line) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for FlectorAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for FlectorAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Horizon {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Circle) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Horizon {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Horizon {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Horizon {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: CircleWeight) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Horizon {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Horizon {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Horizon {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Horizon {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: DipoleWeight) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Horizon {
    type Output = Dipole;

    fn bulk_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Horizon {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Line) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Horizon {
    type Output = CircleBulk;

    fn bulk_contraction(self, other: Origin) -> CircleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Horizon {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Horizon {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Horizon {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: RoundPointBulk) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Horizon {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Horizon {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Line {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Line {
    type Output = Infinity;

    fn bulk_contraction(self, other: DipoleBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Line {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Line {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Line {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Line {
    type Output = Infinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Line {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Line {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Line {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Origin) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Line {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Line {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Line {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Line {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: RoundPointBulk) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Line {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Dipole) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for LineAtInfinity {
    type Output = Infinity;

    fn bulk_contraction(self, other: DipoleBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for LineAtInfinity {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: DipoleWeight) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for LineAtInfinity {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: Origin) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for LineAtInfinity {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for LineAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for LineAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: DipoleWeight) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for LineAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for LineAtOrigin {
    type Output = Infinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for LineAtOrigin {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for LineAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: Origin) -> DipoleWeight {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for LineAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for LineAtOrigin {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for LineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: RoundPointBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Motor {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Motor {
    type Output = Infinity;

    fn bulk_contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Motor {
    type Output = FlectorAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> FlectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Motor {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Motor {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for Motor {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: RoundPointBulk) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Motor {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Plane) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Plane {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Plane {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Plane {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Plane {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Plane {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Plane {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Plane {
    type Output = Dipole;

    fn bulk_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Plane {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Plane {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Plane {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Plane {
    type Output = Infinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Plane {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Plane {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: Origin) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Plane {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Plane {
    type Output = Line;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Line {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Plane {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Plane {
    type Output = Line;

    fn bulk_contraction(self, other: RoundPointBulk) -> Line {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Plane {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: CircleWeight) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for PlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: DipoleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: DipoleWeight) -> DipoleWeight {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = Infinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn bulk_contraction(self, other: Origin) -> CircleWeight {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for PlaneAtOrigin {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Line;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Line {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: RoundPointBulk) -> LineAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Rotor {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Rotor {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Rotor {
    type Output = Infinity;

    fn bulk_contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Rotor {
    type Output = FlectorAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> FlectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Rotor {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Rotor {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for Rotor {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: RoundPointBulk) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Rotor {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPointBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPointBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPointBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for RoundPointBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Sphere {
    type Output = RoundPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Sphere {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Sphere {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: Infinity) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Sphere {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Sphere {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Sphere {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: Origin) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointBulk) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for SphereWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Circle) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for SphereWeight {
    type Output = Origin;

    fn bulk_contraction(self, other: CircleBulk) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for SphereWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for SphereWeight {
    type Output = DipoleWeight;

    fn bulk_contraction(self, other: DipoleBulk) -> DipoleWeight {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for SphereWeight {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for SphereWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for SphereWeight {
    type Output = CircleBulk;

    fn bulk_contraction(self, other: Infinity) -> CircleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for SphereWeight {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Line) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for SphereWeight {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for SphereWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for SphereWeight {
    type Output = CircleWeight;

    fn bulk_contraction(self, other: RoundPointBulk) -> CircleWeight {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for SphereWeight {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for SphereWeight {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Transflector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Transflector {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Transflector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Transflector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Transflector {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Transflector {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Transflector {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Transflector {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Transflector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Transflector {
    type Output = Infinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Transflector {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Transflector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for Transflector {
    type Output = Scalar;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Transflector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Transflector {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Circle> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulk> for Translator {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeight> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtOrigin> for Translator {
    type Output = CircleBulk;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Translator {
    type Output = Infinity;

    fn bulk_contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for Translator {
    type Output = Horizon;

    fn bulk_contraction(self, other: Infinity) -> Horizon {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Translator {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtOrigin> for Translator {
    type Output = DipoleBulk;

    fn bulk_contraction(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Origin> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Translator {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PlaneAtOrigin> for Translator {
    type Output = RoundPointBulk;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for Translator {
    type Output = Transflector;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Transflector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulk> for Translator {
    type Output = Transflector;

    fn bulk_contraction(self, other: RoundPointBulk) -> Transflector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for Translator {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<SphereWeight> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl WeightContraction<Circle> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: DipoleBulk) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Circle {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Circle {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Circle {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Circle {
    type Output = Dipole;

    fn weight_contraction(self, other: Infinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Circle {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: Origin) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Circle {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Circle {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Circle {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Circle {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointBulk) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Circle {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for CircleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for CircleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for CircleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for CircleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Dipole) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for CircleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: DipoleBulk) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for CircleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for CircleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for CircleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for CircleBulk {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: RoundPoint) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for CircleBulk {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for CircleBulk {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for CircleBulk {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: RoundPointBulk) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for CircleBulk {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for CircleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for CircleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for CircleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleBulk) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for CircleCarrierAspect {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Infinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for CircleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for CircleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointBulk) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for CircleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for CircleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for CircleWeight {
    type Output = Origin;

    fn weight_contraction(self, other: DipoleBulk) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for CircleWeight {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for CircleWeight {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Infinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for CircleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for CircleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for CircleWeight {
    type Output = DipoleWeight;

    fn weight_contraction(self, other: RoundPointBulk) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for CircleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Dipole {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Infinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Dipole {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Origin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Dipole {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Dipole {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPointBulk) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Dipole {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for DipoleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for DipoleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for DipoleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for DipoleBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for DipoleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: RoundPoint) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for DipoleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for DipoleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for DipoleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: RoundPointBulk) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for DipoleBulk {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for DipoleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for DipoleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for DipoleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for DipoleCarrierAspect {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Infinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for DipoleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for DipoleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for DipoleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for DipoleWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for DipoleWeight {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Infinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for DipoleWeight {
    type Output = Origin;

    fn weight_contraction(self, other: RoundPointBulk) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for FlatPoint {
    type Output = Infinity;

    fn weight_contraction(self, other: Infinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for FlatPoint {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Origin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for FlatPoint {
    type Output = Infinity;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for FlatPoint {
    type Output = Infinity;

    fn weight_contraction(self, other: RoundPointBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for FlatPoint {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for FlatPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for FlatPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for FlatPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for FlatPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: Origin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for FlatPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: RoundPoint) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = Infinity;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for FlatPointAtInfinity {
    type Output = Infinity;

    fn weight_contraction(self, other: RoundPointBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for FlatPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for FlatPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for FlatPointAtOrigin {
    type Output = Infinity;

    fn weight_contraction(self, other: Infinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for FlatPointAtOrigin {
    type Output = Origin;

    fn weight_contraction(self, other: Origin) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn weight_contraction(self, other: RoundPoint) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = Infinity;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Flector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Flector {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Flector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Flector {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Flector {
    type Output = FlatPoint;

    fn weight_contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Flector {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Flector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Flector {
    type Output = Infinity;

    fn weight_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Flector {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Circle) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for FlectorAtInfinity {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for FlectorAtInfinity {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: CircleWeight) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Line) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for FlectorAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for FlectorAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Horizon {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Circle) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Horizon {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Horizon {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Horizon {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: CircleWeight) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Horizon {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Horizon {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: DipoleWeight) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Horizon {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Line) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Horizon {
    type Output = CircleBulk;

    fn weight_contraction(self, other: Origin) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Horizon {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Horizon {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Horizon {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: RoundPointBulk) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Horizon {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Horizon {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Horizon {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Infinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Infinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Infinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Infinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Line {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Line {
    type Output = Infinity;

    fn weight_contraction(self, other: DipoleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Line {
    type Output = RoundPoint;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Line {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Line {
    type Output = RoundPoint;

    fn weight_contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Line {
    type Output = Infinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Line {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Line {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: Infinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Line {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: Origin) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Line {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Line {
    type Output = FlatPoint;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Line {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Line {
    type Output = FlatPoint;

    fn weight_contraction(self, other: RoundPointBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Line {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Dipole) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for LineAtInfinity {
    type Output = Infinity;

    fn weight_contraction(self, other: DipoleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for LineAtInfinity {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: DipoleWeight) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for LineAtInfinity {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: Origin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for LineAtInfinity {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for LineAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for LineAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for LineAtOrigin {
    type Output = Origin;

    fn weight_contraction(self, other: DipoleWeight) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for LineAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: FlatPoint) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for LineAtOrigin {
    type Output = Infinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for LineAtOrigin {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: Infinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for LineAtOrigin {
    type Output = DipoleWeight;

    fn weight_contraction(self, other: Origin) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for LineAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for LineAtOrigin {
    type Output = FlatPoint;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for LineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: RoundPointBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Motor {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Horizon> for Motor {
    type Output = Infinity;

    fn weight_contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Motor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Infinity) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Motor {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Motor {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Motor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Motor {
    type Output = Flector;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Motor {
    type Output = Flector;

    fn weight_contraction(self, other: RoundPointBulk) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Motor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Motor {
    type Output = Origin;

    fn weight_contraction(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Horizon> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Plane) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Plane {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Plane {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Plane {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Plane {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Plane {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Plane {
    type Output = FlatPoint;

    fn weight_contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Plane {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Plane {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Plane {
    type Output = Dipole;

    fn weight_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Plane {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Plane {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Plane {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Plane {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Plane {
    type Output = Infinity;

    fn weight_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Plane {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Plane {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: Origin) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Plane {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Plane {
    type Output = Line;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Plane {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Plane {
    type Output = Line;

    fn weight_contraction(self, other: RoundPointBulk) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Plane {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for PlaneAtOrigin {
    type Output = Origin;

    fn weight_contraction(self, other: CircleWeight) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for PlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: DipoleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn weight_contraction(self, other: DipoleWeight) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for PlaneAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = Infinity;

    fn weight_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn weight_contraction(self, other: Origin) -> CircleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for PlaneAtOrigin {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Line;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn weight_contraction(self, other: RoundPointBulk) -> LineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Rotor {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Rotor {
    type Output = DipoleWeight;

    fn weight_contraction(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Rotor {
    type Output = LineAtOrigin;

    fn weight_contraction(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Horizon> for Rotor {
    type Output = Infinity;

    fn weight_contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Rotor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Infinity) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Rotor {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Rotor {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Rotor {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Rotor {
    type Output = Flector;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Rotor {
    type Output = Flector;

    fn weight_contraction(self, other: RoundPointBulk) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Rotor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Rotor {
    type Output = Origin;

    fn weight_contraction(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for RoundPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for RoundPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for RoundPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for RoundPointBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for RoundPointBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for RoundPointBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for RoundPointBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Sphere {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Sphere {
    type Output = RoundPointAtOrigin;

    fn weight_contraction(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Sphere {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Sphere {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Sphere {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Horizon> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Sphere {
    type Output = Circle;

    fn weight_contraction(self, other: Infinity) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Sphere {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Sphere {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: LineAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Sphere {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Sphere {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: Origin) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Sphere {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Sphere {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Sphere {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Sphere {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointBulk) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Sphere {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for SphereWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Circle) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for SphereWeight {
    type Output = Origin;

    fn weight_contraction(self, other: CircleBulk) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for SphereWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: Dipole) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for SphereWeight {
    type Output = DipoleWeight;

    fn weight_contraction(self, other: DipoleBulk) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: FlatPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for SphereWeight {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Horizon> for SphereWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for SphereWeight {
    type Output = CircleBulk;

    fn weight_contraction(self, other: Infinity) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for SphereWeight {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Line) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for SphereWeight {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for SphereWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for SphereWeight {
    type Output = CircleWeight;

    fn weight_contraction(self, other: RoundPointBulk) -> CircleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for SphereWeight {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for SphereWeight {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Transflector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Transflector {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Transflector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Transflector {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeight) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Transflector {
    type Output = FlatPoint;

    fn weight_contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Transflector {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Transflector {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Transflector {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Infinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Transflector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Transflector {
    type Output = Infinity;

    fn weight_contraction(self, other: LineAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Transflector {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Transflector {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Transflector {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Transflector {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Transflector {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Circle> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulk> for Translator {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeight> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtOrigin> for Translator {
    type Output = CircleBulk;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Horizon> for Translator {
    type Output = Infinity;

    fn weight_contraction(self, other: Horizon) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for Translator {
    type Output = Horizon;

    fn weight_contraction(self, other: Infinity) -> Horizon {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for Translator {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Translator {
    type Output = DipoleBulk;

    fn weight_contraction(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Translator {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Translator {
    type Output = RoundPointBulk;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for Translator {
    type Output = Transflector;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Transflector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulk> for Translator {
    type Output = Transflector;

    fn weight_contraction(self, other: RoundPointBulk) -> Transflector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for Translator {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<SphereWeight> for Translator {
    type Output = Origin;

    fn weight_contraction(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}
