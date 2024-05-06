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

impl Contraction<Circle> for Circle {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Circle {
    type Output = Scalar;

    fn contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleBulk) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Circle {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Circle {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: DipoleWeight) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Circle {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: FlatPoint) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for Circle {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: FlatPointAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtOrigin> for Circle {
    type Output = RoundPointBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
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

impl Contraction<Origin> for Circle {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: Origin) -> DipoleCarrierAspect {
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

impl Contraction<RoundPointAtInfinity> for Circle {
    type Output = Dipole;

    fn contraction(self, other: RoundPointAtInfinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Circle {
    type Output = Dipole;

    fn contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Circle {
    type Output = Dipole;

    fn contraction(self, other: RoundPointBulk) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Circle {
    type Output = Dipole;

    fn contraction(self, other: RoundPointOnOrigin) -> Dipole {
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

impl Contraction<Circle> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for CircleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Dipole) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for CircleAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: DipoleBulk) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for CircleAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for CircleAtInfinity {
    type Output = RoundPointBulk;

    fn contraction(self, other: DipoleWeight) -> RoundPointBulk {
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

impl Contraction<Origin> for CircleAtInfinity {
    type Output = DipoleBulk;

    fn contraction(self, other: Origin) -> DipoleBulk {
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
    type Output = DipoleAtInfinity;

    fn contraction(self, other: RoundPoint) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: RoundPointAtInfinity) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleAtInfinity {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: RoundPointBulk) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: RoundPointOnOrigin) -> DipoleAtInfinity {
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

impl Contraction<Circle> for CircleBulk {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleBulk {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for CircleBulk {
    type Output = Scalar;

    fn contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for CircleBulk {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: Dipole) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for CircleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: DipoleBulk) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for CircleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleBulk {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleBulk {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPoint) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for CircleBulk {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for CircleBulk {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPointBulk) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for CircleBulk {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPointOnOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for CircleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for CircleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: CircleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: Dipole) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for CircleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: DipoleBulk) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for CircleCarrierAspect {
    type Output = RoundPointBulk;

    fn contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = RoundPointBulk;

    fn contraction(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for CircleCarrierAspect {
    type Output = DipoleBulk;

    fn contraction(self, other: Infinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for CircleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for CircleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: RoundPointBulk) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for CircleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for CircleWeight {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for CircleWeight {
    type Output = Scalar;

    fn contraction(self, other: CircleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for CircleWeight {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for CircleWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: Dipole) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for CircleWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: DipoleAtInfinity) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for CircleWeight {
    type Output = Origin;

    fn contraction(self, other: DipoleBulk) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for CircleWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for CircleWeight {
    type Output = RoundPointBulk;

    fn contraction(self, other: FlatPoint) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for CircleWeight {
    type Output = RoundPointBulk;

    fn contraction(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for CircleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for CircleWeight {
    type Output = DipoleBulk;

    fn contraction(self, other: Infinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for CircleWeight {
    type Output = Scalar;

    fn contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for CircleWeight {
    type Output = Scalar;

    fn contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for CircleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: RoundPoint) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: RoundPointAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for CircleWeight {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for CircleWeight {
    type Output = DipoleWeight;

    fn contraction(self, other: RoundPointBulk) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for CircleWeight {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for CircleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Dipole {
    type Output = Scalar;

    fn contraction(self, other: DipoleWeight) -> Scalar {
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
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Infinity) -> RoundPointAtInfinity {
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

impl Contraction<Origin> for Dipole {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: Origin) -> RoundPointOnOrigin {
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

impl Contraction<RoundPointAtInfinity> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointBulk) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Dipole {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPoint {
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

impl Contraction<Dipole> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for DipoleAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleWeight) -> Scalar {
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

impl Contraction<Origin> for DipoleAtInfinity {
    type Output = RoundPointBulk;

    fn contraction(self, other: Origin) -> RoundPointBulk {
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
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: RoundPoint) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for DipoleAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleAtInfinity {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for DipoleAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: RoundPointBulk) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for DipoleAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPointAtInfinity {
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

impl Contraction<Dipole> for DipoleBulk {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleBulk {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for DipoleBulk {
    type Output = Scalar;

    fn contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for DipoleBulk {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPoint) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for DipoleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for DipoleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPointBulk) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for DipoleBulk {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for DipoleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for DipoleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: DipoleBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for DipoleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for DipoleCarrierAspect {
    type Output = RoundPointBulk;

    fn contraction(self, other: Infinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: RoundPoint) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: RoundPointAtInfinity) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for DipoleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: RoundPointBulk) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for DipoleCarrierAspect {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for DipoleWeight {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for DipoleWeight {
    type Output = Scalar;

    fn contraction(self, other: DipoleAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for DipoleWeight {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for DipoleWeight {
    type Output = Scalar;

    fn contraction(self, other: FlatPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for DipoleWeight {
    type Output = Scalar;

    fn contraction(self, other: FlatPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for DipoleWeight {
    type Output = RoundPointBulk;

    fn contraction(self, other: Infinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for DipoleWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: RoundPoint) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for DipoleWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: RoundPointAtInfinity) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for DipoleWeight {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for DipoleWeight {
    type Output = Origin;

    fn contraction(self, other: RoundPointBulk) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for DipoleWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for FlatPoint {
    type Output = Scalar;

    fn contraction(self, other: DipoleWeight) -> Scalar {
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

impl Contraction<Origin> for FlatPoint {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: Origin) -> RoundPointOnOrigin {
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

impl Contraction<RoundPointAtInfinity> for FlatPoint {
    type Output = Infinity;

    fn contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for FlatPoint {
    type Output = Infinity;

    fn contraction(self, other: RoundPointBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPoint {
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

impl Contraction<Dipole> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for FlatPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: DipoleWeight) -> Scalar {
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

impl Contraction<Origin> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn contraction(self, other: Origin) -> RoundPointBulk {
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
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: RoundPoint) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = RoundPointBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for FlatPointAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: RoundPointBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for FlatPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPointAtInfinity {
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

impl Contraction<Dipole> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: DipoleCarrierAspect) -> Scalar {
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

impl Contraction<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = Infinity;

    fn contraction(self, other: RoundPointAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for FlatPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: RoundPointOnOrigin) -> RoundPointAtOrigin {
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

impl Contraction<Circle> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Flector {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Flector {
    type Output = Infinity;

    fn contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Flector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Flector {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Flector {
    type Output = FlatPoint;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Flector {
    type Output = FlatPoint;

    fn contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleWeight) -> MultiVector {
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
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Line) -> RoundPointAtInfinity {
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
    type Output = RoundPointBulk;

    fn contraction(self, other: LineAtOrigin) -> RoundPointBulk {
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

impl Contraction<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Flector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Flector {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Flector {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Flector {
    type Output = Scalar;

    fn contraction(self, other: SphereWeight) -> Scalar {
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

impl Contraction<Circle> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Circle) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for FlectorAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for FlectorAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for FlectorAtInfinity {
    type Output = RoundPointBulk;

    fn contraction(self, other: CircleWeight) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: DipoleWeight) -> MultiVector {
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

impl Contraction<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for FlectorAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for FlectorAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for FlectorAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: SphereWeight) -> Scalar {
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

impl Contraction<Circle> for Horizon {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Circle) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Horizon {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Horizon {
    type Output = Infinity;

    fn contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Horizon {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Horizon {
    type Output = RoundPointBulk;

    fn contraction(self, other: CircleWeight) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Horizon {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: Dipole) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Horizon {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Horizon {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Horizon {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: DipoleCarrierAspect) -> DipoleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Horizon {
    type Output = DipoleBulk;

    fn contraction(self, other: DipoleWeight) -> DipoleBulk {
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

impl Contraction<Origin> for Horizon {
    type Output = CircleBulk;

    fn contraction(self, other: Origin) -> CircleBulk {
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
    type Output = CircleAtInfinity;

    fn contraction(self, other: RoundPoint) -> CircleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn contraction(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Horizon {
    type Output = CircleBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Horizon {
    type Output = LineAtInfinity;

    fn contraction(self, other: RoundPointBulk) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Horizon {
    type Output = CircleAtInfinity;

    fn contraction(self, other: RoundPointOnOrigin) -> CircleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Horizon {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Horizon {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Horizon {
    type Output = Scalar;

    fn contraction(self, other: SphereWeight) -> Scalar {
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

impl Contraction<RoundPointOnOrigin> for Infinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPointOnOrigin) -> Scalar {
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

impl Contraction<Circle> for Line {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Line {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Line {
    type Output = Scalar;

    fn contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Line {
    type Output = Infinity;

    fn contraction(self, other: DipoleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Line {
    type Output = Infinity;

    fn contraction(self, other: DipoleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Line {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Line {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: DipoleWeight) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Line {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: FlatPoint) -> RoundPointAtInfinity {
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
    type Output = RoundPointBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
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

impl Contraction<Origin> for Line {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: Origin) -> DipoleCarrierAspect {
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

impl Contraction<RoundPointAtInfinity> for Line {
    type Output = FlatPoint;

    fn contraction(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Line {
    type Output = Dipole;

    fn contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Line {
    type Output = FlatPoint;

    fn contraction(self, other: RoundPointBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Line {
    type Output = Dipole;

    fn contraction(self, other: RoundPointOnOrigin) -> Dipole {
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

impl Contraction<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for LineAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: CircleWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Dipole) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for LineAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: DipoleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for LineAtInfinity {
    type Output = Infinity;

    fn contraction(self, other: DipoleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for LineAtInfinity {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for LineAtInfinity {
    type Output = RoundPointBulk;

    fn contraction(self, other: DipoleWeight) -> RoundPointBulk {
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

impl Contraction<Origin> for LineAtInfinity {
    type Output = DipoleBulk;

    fn contraction(self, other: Origin) -> DipoleBulk {
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

impl Contraction<RoundPointAtInfinity> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for LineAtInfinity {
    type Output = DipoleBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn contraction(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for LineAtInfinity {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: RoundPointOnOrigin) -> DipoleAtInfinity {
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

impl Contraction<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for LineAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: CircleCarrierAspect) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for LineAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for LineAtOrigin {
    type Output = Infinity;

    fn contraction(self, other: DipoleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for LineAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for LineAtOrigin {
    type Output = Origin;

    fn contraction(self, other: DipoleWeight) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for LineAtOrigin {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: FlatPoint) -> RoundPointAtInfinity {
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
    type Output = RoundPointBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> RoundPointBulk {
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

impl Contraction<Origin> for LineAtOrigin {
    type Output = DipoleWeight;

    fn contraction(self, other: Origin) -> DipoleWeight {
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
    type Output = Dipole;

    fn contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for LineAtOrigin {
    type Output = FlatPoint;

    fn contraction(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for LineAtOrigin {
    type Output = Dipole;

    fn contraction(self, other: RoundPointAtOrigin) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for LineAtOrigin {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: RoundPointBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for LineAtOrigin {
    type Output = Dipole;

    fn contraction(self, other: RoundPointOnOrigin) -> Dipole {
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

impl Contraction<Circle> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Motor {
    type Output = FlatPoint;

    fn contraction(self, other: CircleAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Motor {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleWeight) -> MultiVector {
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

impl Contraction<Origin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Motor {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Motor {
    type Output = RoundPointBulk;

    fn contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
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

impl Contraction<RoundPointAtInfinity> for Motor {
    type Output = Flector;

    fn contraction(self, other: RoundPointAtInfinity) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Motor {
    type Output = Flector;

    fn contraction(self, other: RoundPointBulk) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Motor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Motor {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Motor {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Motor {
    type Output = Origin;

    fn contraction(self, other: SphereWeight) -> Origin {
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

impl Contraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleWeight) -> MultiVector {
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

impl Contraction<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn contraction(self, other: SphereWeight) -> MultiVector {
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

impl Contraction<RoundPointAtInfinity> for Origin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Origin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Origin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointOnOrigin) -> Scalar {
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

impl Contraction<Circle> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Plane {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Plane {
    type Output = Infinity;

    fn contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Plane {
    type Output = RoundPoint;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Plane {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Plane {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Plane {
    type Output = FlatPoint;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Plane {
    type Output = FlatPoint;

    fn contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Plane {
    type Output = Dipole;

    fn contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Plane {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Plane {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: FlatPoint) -> DipoleAtInfinity {
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
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
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
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Line) -> RoundPointAtInfinity {
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
    type Output = RoundPointBulk;

    fn contraction(self, other: LineAtOrigin) -> RoundPointBulk {
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

impl Contraction<Origin> for Plane {
    type Output = CircleCarrierAspect;

    fn contraction(self, other: Origin) -> CircleCarrierAspect {
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

impl Contraction<RoundPointAtInfinity> for Plane {
    type Output = Line;

    fn contraction(self, other: RoundPointAtInfinity) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Plane {
    type Output = Circle;

    fn contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Plane {
    type Output = Line;

    fn contraction(self, other: RoundPointBulk) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Plane {
    type Output = Circle;

    fn contraction(self, other: RoundPointOnOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Plane {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Plane {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Plane {
    type Output = Scalar;

    fn contraction(self, other: SphereWeight) -> Scalar {
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

impl Contraction<Circle> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for PlaneAtOrigin {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for PlaneAtOrigin {
    type Output = Origin;

    fn contraction(self, other: CircleWeight) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for PlaneAtOrigin {
    type Output = FlatPoint;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for PlaneAtOrigin {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: DipoleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Dipole;

    fn contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn contraction(self, other: DipoleWeight) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for PlaneAtOrigin {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: FlatPoint) -> DipoleAtInfinity {
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
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
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
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Line) -> RoundPointAtInfinity {
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
    type Output = RoundPointBulk;

    fn contraction(self, other: LineAtOrigin) -> RoundPointBulk {
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

impl Contraction<Origin> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn contraction(self, other: Origin) -> CircleWeight {
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
    type Output = Circle;

    fn contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Line;

    fn contraction(self, other: RoundPointAtInfinity) -> Line {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = Circle;

    fn contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn contraction(self, other: RoundPointBulk) -> LineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = Circle;

    fn contraction(self, other: RoundPointOnOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
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

impl Contraction<Circle> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Rotor {
    type Output = FlatPoint;

    fn contraction(self, other: CircleAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Rotor {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Rotor {
    type Output = DipoleWeight;

    fn contraction(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Rotor {
    type Output = LineAtOrigin;

    fn contraction(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: DipoleWeight) -> MultiVector {
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

impl Contraction<Origin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Rotor {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Rotor {
    type Output = RoundPointBulk;

    fn contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
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

impl Contraction<RoundPointAtInfinity> for Rotor {
    type Output = Flector;

    fn contraction(self, other: RoundPointAtInfinity) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Rotor {
    type Output = Flector;

    fn contraction(self, other: RoundPointBulk) -> Flector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Rotor {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Rotor {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Rotor {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Rotor {
    type Output = Origin;

    fn contraction(self, other: SphereWeight) -> Origin {
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

impl Contraction<RoundPointAtInfinity> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for RoundPoint {
    type Output = Scalar;

    fn contraction(self, other: RoundPointOnOrigin) -> Scalar {
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

impl Contraction<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for RoundPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for RoundPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = Scalar;

    fn contraction(self, other: RoundPointOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
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

impl Contraction<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointOnOrigin) -> Scalar {
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

impl Contraction<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for RoundPointBulk {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for RoundPointBulk {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for RoundPointBulk {
    type Output = Scalar;

    fn contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for RoundPointBulk {
    type Output = Scalar;

    fn contraction(self, other: RoundPointOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for RoundPointOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for RoundPointOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtInfinity) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for RoundPointOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointBulk) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = Scalar;

    fn contraction(self, other: RoundPointOnOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for SpacialCurvature {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for SpacialCurvature {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for SpacialCurvature {
    type Output = RoundPoint;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for SpacialCurvature {
    type Output = RoundPointBulk;

    fn contraction(self, other: CircleWeight) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for SpacialCurvature {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for SpacialCurvature {
    type Output = Dipole;

    fn contraction(self, other: DipoleAtInfinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for SpacialCurvature {
    type Output = Dipole;

    fn contraction(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for SpacialCurvature {
    type Output = Dipole;

    fn contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for SpacialCurvature {
    type Output = DipoleBulk;

    fn contraction(self, other: DipoleWeight) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for SpacialCurvature {
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPoint) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for SpacialCurvature {
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for SpacialCurvature {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for SpacialCurvature {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for SpacialCurvature {
    type Output = Scalar;

    fn contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for SpacialCurvature {
    type Output = CircleBulk;

    fn contraction(self, other: Infinity) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for SpacialCurvature {
    type Output = RoundPointBulk;

    fn contraction(self, other: Line) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for SpacialCurvature {
    type Output = RoundPointBulk;

    fn contraction(self, other: LineAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for SpacialCurvature {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Origin> for SpacialCurvature {
    type Output = CircleBulk;

    fn contraction(self, other: Origin) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for SpacialCurvature {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for SpacialCurvature {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for SpacialCurvature {
    type Output = Circle;

    fn contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for SpacialCurvature {
    type Output = Circle;

    fn contraction(self, other: RoundPointAtInfinity) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for SpacialCurvature {
    type Output = CircleBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for SpacialCurvature {
    type Output = Circle;

    fn contraction(self, other: RoundPointBulk) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for SpacialCurvature {
    type Output = Circle;

    fn contraction(self, other: RoundPointOnOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for SpacialCurvature {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for SpacialCurvature {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for SpacialCurvature {
    type Output = Scalar;

    fn contraction(self, other: SphereWeight) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for SpacialCurvature {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for SpacialCurvature {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: CircleAtInfinity) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Sphere {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Sphere {
    type Output = RoundPoint;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Sphere {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: DipoleAtInfinity) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Sphere {
    type Output = Dipole;

    fn contraction(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Sphere {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Sphere {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: FlatPoint) -> DipoleAtInfinity {
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
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
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
    type Output = CircleAtInfinity;

    fn contraction(self, other: Infinity) -> CircleAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for Sphere {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Line) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for Sphere {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: LineAtInfinity) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtOrigin> for Sphere {
    type Output = RoundPointBulk;

    fn contraction(self, other: LineAtOrigin) -> RoundPointBulk {
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

impl Contraction<Origin> for Sphere {
    type Output = CircleCarrierAspect;

    fn contraction(self, other: Origin) -> CircleCarrierAspect {
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

impl Contraction<RoundPointAtInfinity> for Sphere {
    type Output = Circle;

    fn contraction(self, other: RoundPointAtInfinity) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Sphere {
    type Output = Circle;

    fn contraction(self, other: RoundPointAtOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Sphere {
    type Output = Circle;

    fn contraction(self, other: RoundPointBulk) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Sphere {
    type Output = Circle;

    fn contraction(self, other: RoundPointOnOrigin) -> Circle {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Sphere {
    type Output = Scalar;

    fn contraction(self, other: SphereWeight) -> Scalar {
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

impl Contraction<Circle> for SphereWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: Circle) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for SphereWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: CircleAtInfinity) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for SphereWeight {
    type Output = Origin;

    fn contraction(self, other: CircleBulk) -> Origin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for SphereWeight {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: Dipole) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: DipoleAtInfinity) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for SphereWeight {
    type Output = DipoleWeight;

    fn contraction(self, other: DipoleBulk) -> DipoleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for SphereWeight {
    type Output = DipoleCarrierAspect;

    fn contraction(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for SphereWeight {
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPoint) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPointAtInfinity> for SphereWeight {
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPointAtInfinity) -> DipoleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Flector> for SphereWeight {
    type Output = MultiVector;

    fn contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Horizon> for SphereWeight {
    type Output = Scalar;

    fn contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Infinity> for SphereWeight {
    type Output = CircleBulk;

    fn contraction(self, other: Infinity) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Line> for SphereWeight {
    type Output = RoundPointBulk;

    fn contraction(self, other: Line) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<LineAtInfinity> for SphereWeight {
    type Output = RoundPointBulk;

    fn contraction(self, other: LineAtInfinity) -> RoundPointBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Motor> for SphereWeight {
    type Output = MultiVector;

    fn contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for SphereWeight {
    type Output = Scalar;

    fn contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPoint> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn contraction(self, other: RoundPoint) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtInfinity> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn contraction(self, other: RoundPointAtInfinity) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for SphereWeight {
    type Output = CircleBulk;

    fn contraction(self, other: RoundPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for SphereWeight {
    type Output = CircleWeight;

    fn contraction(self, other: RoundPointBulk) -> CircleWeight {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for SphereWeight {
    type Output = CircleCarrierAspect;

    fn contraction(self, other: RoundPointOnOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for SphereWeight {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for SphereWeight {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Translator> for SphereWeight {
    type Output = MultiVector;

    fn contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Circle> for Transflector {
    type Output = RoundPoint;

    fn contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Transflector {
    type Output = Infinity;

    fn contraction(self, other: CircleAtInfinity) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Transflector {
    type Output = Infinity;

    fn contraction(self, other: CircleBulk) -> Infinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Transflector {
    type Output = RoundPoint;

    fn contraction(self, other: CircleCarrierAspect) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Transflector {
    type Output = RoundPointOnOrigin;

    fn contraction(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Transflector {
    type Output = FlatPoint;

    fn contraction(self, other: DipoleAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Transflector {
    type Output = FlatPoint;

    fn contraction(self, other: DipoleBulk) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Transflector {
    type Output = DipoleAtInfinity;

    fn contraction(self, other: FlatPoint) -> DipoleAtInfinity {
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
    type Output = DipoleBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> DipoleBulk {
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
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Line) -> RoundPointAtInfinity {
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
    type Output = RoundPointBulk;

    fn contraction(self, other: LineAtOrigin) -> RoundPointBulk {
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

impl Contraction<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Transflector {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: SpacialCurvature) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Transflector {
    type Output = Scalar;

    fn contraction(self, other: SphereWeight) -> Scalar {
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

impl Contraction<Circle> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleAtInfinity> for Translator {
    type Output = FlatPoint;

    fn contraction(self, other: CircleAtInfinity) -> FlatPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleBulk> for Translator {
    type Output = FlatPointAtOrigin;

    fn contraction(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<CircleWeight> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Dipole> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleAtInfinity> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleAtInfinity) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<FlatPoint> for Translator {
    type Output = CircleAtInfinity;

    fn contraction(self, other: FlatPoint) -> CircleAtInfinity {
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
    type Output = CircleBulk;

    fn contraction(self, other: FlatPointAtOrigin) -> CircleBulk {
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
    type Output = DipoleBulk;

    fn contraction(self, other: LineAtOrigin) -> DipoleBulk {
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

impl Contraction<Origin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Plane> for Translator {
    type Output = RoundPointAtInfinity;

    fn contraction(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<PlaneAtOrigin> for Translator {
    type Output = RoundPointBulk;

    fn contraction(self, other: PlaneAtOrigin) -> RoundPointBulk {
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

impl Contraction<RoundPointAtInfinity> for Translator {
    type Output = Transflector;

    fn contraction(self, other: RoundPointAtInfinity) -> Transflector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointBulk> for Translator {
    type Output = Transflector;

    fn contraction(self, other: RoundPointBulk) -> Transflector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<RoundPointOnOrigin> for Translator {
    type Output = MultiVector;

    fn contraction(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SpacialCurvature> for Translator {
    type Output = RoundPointAtOrigin;

    fn contraction(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<Sphere> for Translator {
    type Output = RoundPoint;

    fn contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.anti_dual())
    }
}

impl Contraction<SphereWeight> for Translator {
    type Output = Origin;

    fn contraction(self, other: SphereWeight) -> Origin {
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
