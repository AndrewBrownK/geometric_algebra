//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::exterior::AntiWedge;
use crate::*;

///
/// Bulk Contraction (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
///
pub trait BulkContraction<T> {
    type Output;
    fn bulk_contraction(self, other: T) -> Self::Output;
}

///
/// Weight Contraction (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
///
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

impl BulkContraction<CircleBulkAspect> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleWeightAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Circle {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Dipole {
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

impl BulkContraction<Circle> for CircleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulkAspect> for CircleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for CircleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: Dipole) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for CircleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulkAspect> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> DipoleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> DipoleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for CircleBulkAspect {
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

impl BulkContraction<CircleBulkAspect> for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Scalar {
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

impl BulkContraction<DipoleBulkAspect> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> RoundPointCarrierAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulkAspect {
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
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: Infinity) -> DipoleBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> DipoleCarrierAspect {
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

impl BulkContraction<Circle> for CircleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for CircleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for CircleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for CircleWeightAspect {
    type Output = Origin;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for CircleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for CircleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for CircleWeightAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for CircleWeightAspect {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: Infinity) -> DipoleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for CircleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for CircleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulkAspect> for CircleWeightAspect {
    type Output = DipoleWeightAspect;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> DipoleWeightAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for CircleWeightAspect {
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

impl BulkContraction<DipoleBulkAspect> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> Scalar {
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

impl BulkContraction<RoundPointBulkAspect> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> RoundPoint {
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

impl BulkContraction<Dipole> for DipoleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for DipoleBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulkAspect> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for DipoleBulkAspect {
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

impl BulkContraction<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> Scalar {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: Infinity) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> RoundPointCarrierAspect {
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

impl BulkContraction<Dipole> for DipoleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for DipoleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for DipoleWeightAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: Infinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulkAspect> for DipoleWeightAspect {
    type Output = Origin;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for DipoleWeightAspect {
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

impl BulkContraction<DipoleWeightAspect> for FlatPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> Scalar {
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

impl BulkContraction<RoundPointBulkAspect> for FlatPoint {
    type Output = Infinity;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Infinity {
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

impl BulkContraction<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> Scalar {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: Origin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for FlatPointAtInfinity {
    type Output = Infinity;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Infinity {
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

impl BulkContraction<CircleBulkAspect> for Flector {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Flector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Flector {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl BulkContraction<SphereWeightAspect> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl BulkContraction<CircleBulkAspect> for FlectorAtInfinity {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for FlectorAtInfinity {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: CircleWeightAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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

impl BulkContraction<RoundPointBulkAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl BulkContraction<SphereWeightAspect> for FlectorAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl BulkContraction<CircleBulkAspect> for Horizon {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Horizon {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Horizon {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: CircleWeightAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Horizon {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Horizon {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> FlatPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Horizon {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Horizon {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> DipoleBulkAspect {
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
    type Output = CircleBulkAspect;

    fn bulk_contraction(self, other: Origin) -> CircleBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Horizon {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> LineAtInfinity {
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

impl BulkContraction<SphereWeightAspect> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl BulkContraction<CircleWeightAspect> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleWeightAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Line {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Line {
    type Output = Infinity;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Line {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Line {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Line {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> FlatPoint {
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

impl BulkContraction<CircleWeightAspect> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: CircleWeightAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: Dipole) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for LineAtInfinity {
    type Output = Infinity;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for LineAtInfinity {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> RoundPointBulkAspect {
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
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: Origin) -> DipoleBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> FlatPointAtInfinity {
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

impl BulkContraction<DipoleWeightAspect> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> Origin {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
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
    type Output = DipoleWeightAspect;

    fn bulk_contraction(self, other: Origin) -> DipoleWeightAspect {
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

impl BulkContraction<RoundPointBulkAspect> for LineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> FlatPointAtOrigin {
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

impl BulkContraction<CircleBulkAspect> for Motor {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleWeightAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Flector {
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

impl BulkContraction<SphereWeightAspect> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Origin {
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

impl BulkContraction<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleBulkAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleWeightAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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

impl BulkContraction<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl BulkContraction<SphereWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: SphereWeightAspect) -> MultiVector {
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

impl BulkContraction<CircleBulkAspect> for Plane {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Plane {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Plane {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Plane {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Plane {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Plane {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> DipoleCarrierAspect {
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
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Plane {
    type Output = Line;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Line {
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

impl BulkContraction<SphereWeightAspect> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl BulkContraction<CircleWeightAspect> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: CircleWeightAspect) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for PlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for PlaneAtOrigin {
    type Output = DipoleWeightAspect;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
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
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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
    type Output = CircleWeightAspect;

    fn bulk_contraction(self, other: Origin) -> CircleWeightAspect {
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

impl BulkContraction<RoundPointBulkAspect> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> LineAtOrigin {
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

impl BulkContraction<CircleBulkAspect> for Rotor {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Rotor {
    type Output = DipoleWeightAspect;

    fn bulk_contraction(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> LineAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Flector {
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

impl BulkContraction<SphereWeightAspect> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Origin {
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

impl BulkContraction<RoundPointBulkAspect> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Scalar {
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

impl BulkContraction<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Scalar {
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

impl BulkContraction<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPointBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Scalar {
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

impl BulkContraction<CircleBulkAspect> for Sphere {
    type Output = RoundPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulkAspect) -> RoundPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Sphere {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> DipoleCarrierAspect {
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
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Circle {
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

impl BulkContraction<SphereWeightAspect> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl BulkContraction<Circle> for SphereWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Circle) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleBulkAspect> for SphereWeightAspect {
    type Output = Origin;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for SphereWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for SphereWeightAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: Dipole) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> DipoleWeightAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for SphereWeightAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPoint> for SphereWeightAspect {
    type Output = DipoleCarrierAspect;

    fn bulk_contraction(self, other: FlatPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlatPointAtInfinity> for SphereWeightAspect {
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtInfinity) -> DipoleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for SphereWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Infinity> for SphereWeightAspect {
    type Output = CircleBulkAspect;

    fn bulk_contraction(self, other: Infinity) -> CircleBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for SphereWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: Line) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for SphereWeightAspect {
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for SphereWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPoint> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPoint) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtInfinity> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtInfinity) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointAtOrigin> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointBulkAspect> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> CircleWeightAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<RoundPointCarrierAspect> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn bulk_contraction(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Sphere> for SphereWeightAspect {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for SphereWeightAspect {
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

impl BulkContraction<CircleBulkAspect> for Transflector {
    type Output = Infinity;

    fn bulk_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Transflector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Transflector {
    type Output = RoundPointCarrierAspect;

    fn bulk_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Transflector {
    type Output = FlatPoint;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> FlatPoint {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl BulkContraction<SphereWeightAspect> for Transflector {
    type Output = Scalar;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl BulkContraction<CircleBulkAspect> for Translator {
    type Output = FlatPointAtOrigin;

    fn bulk_contraction(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<CircleWeightAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: CircleWeightAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Dipole> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleBulkAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleBulkAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<DipoleWeightAspect> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = CircleBulkAspect;

    fn bulk_contraction(self, other: FlatPointAtOrigin) -> CircleBulkAspect {
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
    type Output = DipoleBulkAspect;

    fn bulk_contraction(self, other: LineAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn bulk_contraction(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
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

impl BulkContraction<RoundPointBulkAspect> for Translator {
    type Output = Transflector;

    fn bulk_contraction(self, other: RoundPointBulkAspect) -> Transflector {
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

impl BulkContraction<SphereWeightAspect> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: SphereWeightAspect) -> Origin {
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

impl WeightContraction<CircleBulkAspect> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleBulkAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleWeightAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: DipoleBulkAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Circle {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Circle {
    type Output = Dipole;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Dipole {
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

impl WeightContraction<Circle> for CircleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulkAspect> for CircleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleBulkAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for CircleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: Dipole) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: DipoleBulkAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for CircleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for CircleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: RoundPoint) -> DipoleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> DipoleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> DipoleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulkAspect> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> DipoleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for CircleBulkAspect {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> DipoleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for CircleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for CircleBulkAspect {
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

impl WeightContraction<CircleBulkAspect> for CircleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleBulkAspect) -> Scalar {
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

impl WeightContraction<DipoleBulkAspect> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleBulkAspect) -> RoundPointCarrierAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulkAspect {
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
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: Infinity) -> DipoleBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> DipoleCarrierAspect {
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

impl WeightContraction<Circle> for CircleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for CircleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for CircleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Dipole) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for CircleWeightAspect {
    type Output = Origin;

    fn weight_contraction(self, other: DipoleBulkAspect) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for CircleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for CircleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: FlatPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for CircleWeightAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for CircleWeightAspect {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: Infinity) -> DipoleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for CircleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for CircleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulkAspect> for CircleWeightAspect {
    type Output = DipoleWeightAspect;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> DipoleWeightAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for CircleWeightAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for CircleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for CircleWeightAspect {
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

impl WeightContraction<DipoleBulkAspect> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleBulkAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleWeightAspect) -> Scalar {
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

impl WeightContraction<RoundPointBulkAspect> for Dipole {
    type Output = RoundPoint;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> RoundPoint {
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

impl WeightContraction<Dipole> for DipoleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for DipoleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleBulkAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for DipoleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for DipoleBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: RoundPoint) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulkAspect> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for DipoleBulkAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for DipoleBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for DipoleBulkAspect {
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

impl WeightContraction<DipoleBulkAspect> for DipoleCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleBulkAspect) -> Scalar {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: Infinity) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> RoundPointCarrierAspect {
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

impl WeightContraction<Dipole> for DipoleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for DipoleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for DipoleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for DipoleWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for DipoleWeightAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: Infinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulkAspect> for DipoleWeightAspect {
    type Output = Origin;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for DipoleWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for DipoleWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for DipoleWeightAspect {
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

impl WeightContraction<DipoleWeightAspect> for FlatPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleWeightAspect) -> Scalar {
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

impl WeightContraction<RoundPointBulkAspect> for FlatPoint {
    type Output = Infinity;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Infinity {
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

impl WeightContraction<DipoleWeightAspect> for FlatPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: DipoleWeightAspect) -> Scalar {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: Origin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for FlatPointAtInfinity {
    type Output = Infinity;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Infinity {
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

impl WeightContraction<CircleBulkAspect> for Flector {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Flector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Flector {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Flector {
    type Output = FlatPoint;

    fn weight_contraction(self, other: DipoleBulkAspect) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl WeightContraction<SphereWeightAspect> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl WeightContraction<CircleBulkAspect> for FlectorAtInfinity {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for FlectorAtInfinity {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: CircleWeightAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: DipoleBulkAspect) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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

impl WeightContraction<RoundPointBulkAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl WeightContraction<SphereWeightAspect> for FlectorAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl WeightContraction<CircleBulkAspect> for Horizon {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Horizon {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Horizon {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: CircleWeightAspect) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Horizon {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: DipoleBulkAspect) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Horizon {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: DipoleWeightAspect) -> DipoleBulkAspect {
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
    type Output = CircleBulkAspect;

    fn weight_contraction(self, other: Origin) -> CircleBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Horizon {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> LineAtInfinity {
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

impl WeightContraction<SphereWeightAspect> for Horizon {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl WeightContraction<CircleWeightAspect> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleWeightAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Line {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Line {
    type Output = Infinity;

    fn weight_contraction(self, other: DipoleBulkAspect) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Line {
    type Output = RoundPoint;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Line {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: DipoleWeightAspect) -> RoundPointCarrierAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Line {
    type Output = FlatPoint;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> FlatPoint {
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

impl WeightContraction<CircleWeightAspect> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: CircleWeightAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: Dipole) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for LineAtInfinity {
    type Output = Infinity;

    fn weight_contraction(self, other: DipoleBulkAspect) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for LineAtInfinity {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: DipoleWeightAspect) -> RoundPointBulkAspect {
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
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: Origin) -> DipoleBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> FlatPointAtInfinity {
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

impl WeightContraction<DipoleWeightAspect> for LineAtOrigin {
    type Output = Origin;

    fn weight_contraction(self, other: DipoleWeightAspect) -> Origin {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> RoundPointBulkAspect {
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
    type Output = DipoleWeightAspect;

    fn weight_contraction(self, other: Origin) -> DipoleWeightAspect {
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

impl WeightContraction<RoundPointBulkAspect> for LineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> FlatPointAtOrigin {
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

impl WeightContraction<CircleBulkAspect> for Motor {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleWeightAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleBulkAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Motor {
    type Output = Flector;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Flector {
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

impl WeightContraction<SphereWeightAspect> for Motor {
    type Output = Origin;

    fn weight_contraction(self, other: SphereWeightAspect) -> Origin {
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

impl WeightContraction<CircleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleBulkAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleWeightAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleBulkAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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

impl WeightContraction<RoundPointBulkAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl WeightContraction<SphereWeightAspect> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: SphereWeightAspect) -> MultiVector {
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

impl WeightContraction<CircleBulkAspect> for Plane {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Plane {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Plane {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Plane {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Plane {
    type Output = FlatPoint;

    fn weight_contraction(self, other: DipoleBulkAspect) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Plane {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Plane {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: DipoleWeightAspect) -> DipoleCarrierAspect {
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
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Plane {
    type Output = Line;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Line {
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

impl WeightContraction<SphereWeightAspect> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl WeightContraction<CircleWeightAspect> for PlaneAtOrigin {
    type Output = Origin;

    fn weight_contraction(self, other: CircleWeightAspect) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for PlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: DipoleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for PlaneAtOrigin {
    type Output = DipoleWeightAspect;

    fn weight_contraction(self, other: DipoleWeightAspect) -> DipoleWeightAspect {
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
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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
    type Output = CircleWeightAspect;

    fn weight_contraction(self, other: Origin) -> CircleWeightAspect {
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

impl WeightContraction<RoundPointBulkAspect> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> LineAtOrigin {
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

impl WeightContraction<CircleBulkAspect> for Rotor {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Rotor {
    type Output = DipoleWeightAspect;

    fn weight_contraction(self, other: CircleWeightAspect) -> DipoleWeightAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Rotor {
    type Output = LineAtOrigin;

    fn weight_contraction(self, other: DipoleBulkAspect) -> LineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Rotor {
    type Output = Flector;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Flector {
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

impl WeightContraction<SphereWeightAspect> for Rotor {
    type Output = Origin;

    fn weight_contraction(self, other: SphereWeightAspect) -> Origin {
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

impl WeightContraction<RoundPointBulkAspect> for RoundPoint {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Scalar {
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

impl WeightContraction<RoundPointBulkAspect> for RoundPointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Scalar {
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

impl WeightContraction<Flector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for RoundPointBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for RoundPointBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for RoundPointBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulkAspect> for RoundPointBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for RoundPointBulkAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for RoundPointBulkAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Scalar {
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

impl WeightContraction<CircleBulkAspect> for Sphere {
    type Output = RoundPointAtOrigin;

    fn weight_contraction(self, other: CircleBulkAspect) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Sphere {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Sphere {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleBulkAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Sphere {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: DipoleWeightAspect) -> DipoleCarrierAspect {
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
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Sphere {
    type Output = Circle;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Circle {
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

impl WeightContraction<SphereWeightAspect> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl WeightContraction<Circle> for SphereWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Circle) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleBulkAspect> for SphereWeightAspect {
    type Output = Origin;

    fn weight_contraction(self, other: CircleBulkAspect) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for SphereWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for SphereWeightAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: Dipole) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for SphereWeightAspect {
    type Output = DipoleWeightAspect;

    fn weight_contraction(self, other: DipoleBulkAspect) -> DipoleWeightAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for SphereWeightAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPoint> for SphereWeightAspect {
    type Output = DipoleCarrierAspect;

    fn weight_contraction(self, other: FlatPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlatPointAtInfinity> for SphereWeightAspect {
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: FlatPointAtInfinity) -> DipoleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<FlectorAtInfinity> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Horizon> for SphereWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Infinity> for SphereWeightAspect {
    type Output = CircleBulkAspect;

    fn weight_contraction(self, other: Infinity) -> CircleBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for SphereWeightAspect {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: Line) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtInfinity> for SphereWeightAspect {
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: LineAtInfinity) -> RoundPointBulkAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for SphereWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPoint> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPoint) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtInfinity> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtInfinity) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointAtOrigin> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPointAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointBulkAspect> for SphereWeightAspect {
    type Output = CircleWeightAspect;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> CircleWeightAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<RoundPointCarrierAspect> for SphereWeightAspect {
    type Output = CircleCarrierAspect;

    fn weight_contraction(self, other: RoundPointCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Sphere> for SphereWeightAspect {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for SphereWeightAspect {
    type Output = MultiVector;

    fn weight_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Translator> for SphereWeightAspect {
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

impl WeightContraction<CircleBulkAspect> for Transflector {
    type Output = Infinity;

    fn weight_contraction(self, other: CircleBulkAspect) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Transflector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Transflector {
    type Output = RoundPointCarrierAspect;

    fn weight_contraction(self, other: CircleWeightAspect) -> RoundPointCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Transflector {
    type Output = FlatPoint;

    fn weight_contraction(self, other: DipoleBulkAspect) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Transflector {
    type Output = MultiVector;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> MultiVector {
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

impl WeightContraction<SphereWeightAspect> for Transflector {
    type Output = Scalar;

    fn weight_contraction(self, other: SphereWeightAspect) -> Scalar {
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

impl WeightContraction<CircleBulkAspect> for Translator {
    type Output = FlatPointAtOrigin;

    fn weight_contraction(self, other: CircleBulkAspect) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<CircleWeightAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: CircleWeightAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Dipole> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleBulkAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleBulkAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<DipoleWeightAspect> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: DipoleWeightAspect) -> MultiVector {
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
    type Output = CircleBulkAspect;

    fn weight_contraction(self, other: FlatPointAtOrigin) -> CircleBulkAspect {
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
    type Output = DipoleBulkAspect;

    fn weight_contraction(self, other: LineAtOrigin) -> DipoleBulkAspect {
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
    type Output = RoundPointBulkAspect;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPointBulkAspect {
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

impl WeightContraction<RoundPointBulkAspect> for Translator {
    type Output = Transflector;

    fn weight_contraction(self, other: RoundPointBulkAspect) -> Transflector {
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

impl WeightContraction<SphereWeightAspect> for Translator {
    type Output = Origin;

    fn weight_contraction(self, other: SphereWeightAspect) -> Origin {
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
