//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::products::exterior::AntiWedge;
use crate::products::exterior::Wedge;
use crate::*;

/// Orthogonal Rejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `ProjectOrthogonallyOnto`.
pub trait RejectOrthogonallyFrom<T> {
    type Output;
    fn reject_orthogonally_from(self, other: T) -> Self::Output;
}

/// Orthogonal AntiRejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `AntiProjectOrthogonallyOnto`.
pub trait AntiRejectOrthogonallyFrom<T> {
    type Output;
    fn anti_reject_orthogonally_from(self, other: T) -> Self::Output;
}

/// Central (from origin) Rejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `ProjectViaOriginOnto`.
pub trait RejectViaOriginFrom<T> {
    type Output;
    fn reject_via_origin_from(self, other: T) -> Self::Output;
}

/// Outward (from horizon) AntiRejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `AntiProjectViaHorizonOnto`.
pub trait AntiRejectViaHorizonFrom<T> {
    type Output;
    fn anti_reject_via_horizon_from(self, other: T) -> Self::Output;
}

impl AntiRejectOrthogonallyFrom<Dipole> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Circle {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Circle {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for Circle {
    type Output = CircleBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Circle {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Circle {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Origin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Circle {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for CircleBulk {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for CircleBulk {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for CircleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for CircleBulk {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for CircleBulk {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for CircleCarrierAspect {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for CircleWeight {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for CircleWeight {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for CircleWeight {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for CircleWeight {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for CircleWeight {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for CircleWeight {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for Dipole {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for Dipole {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Dipole {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for Dipole {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Dipole {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Dipole {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Origin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for FlatPoint {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> FlatPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for FlatPoint {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for FlatPoint {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> FlatPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Flector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for Flector {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for Flector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for Flector {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Flector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Flector {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> FlatPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Flector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Flector {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Flector {
    type Output = Flector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Flector {
    type Output = Flector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Flector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Horizon {
    type Output = SphereWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Horizon {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Horizon {
    type Output = SpacialCurvature;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Horizon {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for Infinity {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Infinity {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Infinity {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SpacialCurvature> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SphereWeight> for Infinity {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Line {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Line {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Line {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Line {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Line {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Line {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Line {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Line {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Line {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Line {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Line {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Motor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Motor {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Motor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Motor {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Motor {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Motor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Motor {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Motor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Motor {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Line {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Motor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Origin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for Origin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for Origin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Origin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Origin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Origin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for Origin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for Origin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Horizon> for Origin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for Origin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Origin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for Origin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for Origin {
    type Output = RoundPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Origin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Origin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Origin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SpacialCurvature> for Origin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for Origin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Plane {
    type Output = SphereWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Plane {
    type Output = Plane;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Plane {
    type Output = SpacialCurvature;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Plane {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Rotor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Rotor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Rotor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Rotor {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Rotor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPoint {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPoint {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Horizon> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPoint {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SpacialCurvature> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SphereWeight> for RoundPoint {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPointAtInfinity {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPointAtInfinity {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPointAtInfinity {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SpacialCurvature> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SphereWeight> for RoundPointAtInfinity {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SpacialCurvature> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SphereWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for RoundPointBulk {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPointOnOrigin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> Origin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Horizon> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for RoundPointOnOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<SpacialCurvature> for RoundPointOnOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for SpacialCurvature {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for SpacialCurvature {
    type Output = SphereWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for SpacialCurvature {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for SpacialCurvature {
    type Output = Plane;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for Sphere {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Sphere {
    type Output = SphereWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Sphere {
    type Output = SpacialCurvature;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Sphere {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for SphereWeight {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for SphereWeight {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for SphereWeight {
    type Output = Plane;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for SphereWeight {
    type Output = SpacialCurvature;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Transflector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for Transflector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for Transflector {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Transflector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Transflector {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Transflector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Transflector {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Transflector {
    type Output = Transflector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Transflector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Transflector {
    type Output = Transflector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> Transflector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Translator {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Translator {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Translator {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Translator {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Translator {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Translator {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointOnOrigin> for Translator {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Circle {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Circle {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for Circle {
    type Output = CircleBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Circle {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Circle {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Origin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Circle {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for CircleBulk {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for CircleBulk {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for CircleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for CircleBulk {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for CircleBulk {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for CircleCarrierAspect {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for CircleWeight {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for CircleWeight {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for CircleWeight {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for CircleWeight {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for CircleWeight {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for CircleWeight {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> CircleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for Dipole {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for Dipole {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Dipole {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for Dipole {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for Dipole {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Dipole {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Origin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Line) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for DipoleWeight {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> DipoleCarrierAspect {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for FlatPoint {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> FlatPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for FlatPoint {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for FlatPoint {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for FlatPoint {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> FlatPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> FlatPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Flector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for Flector {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for Flector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for Flector {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Flector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Flector {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> FlatPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Flector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Flector {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for FlectorAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Horizon {
    type Output = SphereWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Horizon {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Horizon {
    type Output = SpacialCurvature;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Horizon {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for Infinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Infinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Infinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SpacialCurvature> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for Infinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SphereWeight> for Infinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Line {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Line {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Line {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Line {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Line {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Line {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Line {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Line {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Line {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Line {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for LineAtInfinity {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for LineAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Motor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Motor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Motor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Motor {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Motor {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Motor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Motor {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Motor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Motor {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Motor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Circle) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: SpacialCurvature) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: SphereWeight) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Origin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for Origin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Origin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Origin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for Origin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for Origin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for Origin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for Origin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Origin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Origin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for Origin {
    type Output = RoundPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Origin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Origin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Origin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SpacialCurvature> for Origin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for Origin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Plane {
    type Output = SphereWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Plane {
    type Output = Plane;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Plane {
    type Output = SpacialCurvature;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Plane {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Rotor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Rotor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Rotor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Rotor {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Rotor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPoint {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPoint {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPoint {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPoint {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SpacialCurvature> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SphereWeight> for RoundPoint {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPointAtInfinity {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPointAtInfinity {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPointAtInfinity {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SpacialCurvature> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SphereWeight> for RoundPointAtInfinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SpacialCurvature> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SphereWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: SphereWeight) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointBulk {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for RoundPointBulk {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPointOnOrigin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Line) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPointOnOrigin {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Infinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for RoundPointOnOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Plane) -> RoundPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> RoundPointOnOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<SpacialCurvature> for RoundPointOnOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for SpacialCurvature {
    type Output = Horizon;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Horizon {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for SpacialCurvature {
    type Output = SphereWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for SpacialCurvature {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for SpacialCurvature {
    type Output = Plane;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for Sphere {
    type Output = Horizon;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Horizon {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Sphere {
    type Output = SphereWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> SphereWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Sphere {
    type Output = SpacialCurvature;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Sphere {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for SphereWeight {
    type Output = Horizon;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Horizon {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for SphereWeight {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for SphereWeight {
    type Output = Plane;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Plane {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for SphereWeight {
    type Output = SpacialCurvature;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Transflector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for Transflector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for Transflector {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Transflector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Transflector {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> FlatPointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Transflector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Transflector {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Transflector {
    type Output = Transflector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Transflector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Transflector {
    type Output = Transflector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> Transflector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Translator {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Translator {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Translator {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Translator {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: Origin) -> CircleWeight {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Translator {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Translator {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPointOnOrigin> for Translator {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPointOnOrigin) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Circle {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Circle {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Circle {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Circle {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Circle {
    type Output = CircleBulk;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for Circle {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Horizon) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Circle {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for CircleBulk {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for CircleBulk {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for CircleBulk {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for CircleBulk {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for CircleBulk {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for CircleBulk {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for CircleWeight {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for CircleWeight {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for CircleWeight {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for CircleWeight {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for CircleWeight {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for CircleWeight {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for CircleWeight {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for CircleWeight {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Dipole {
    type Output = FlatPointAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Dipole {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for Dipole {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Dipole {
    type Output = DipoleBulk;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Dipole {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for DipoleBulk {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for DipoleBulk {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for DipoleBulk {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for DipoleBulk {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for DipoleWeight {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for DipoleWeight {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for DipoleWeight {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for DipoleWeight {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for DipoleWeight {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for FlatPoint {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlatPoint {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for FlatPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlatPointAtOrigin {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Flector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Flector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Flector {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Flector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Flector {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Flector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Flector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Flector {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Flector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Flector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Flector {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Flector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for FlectorAtInfinity {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for FlectorAtInfinity {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Horizon {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Horizon {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Horizon {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Horizon {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Horizon {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Horizon {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Horizon {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Plane) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Horizon {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Horizon {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Horizon {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Horizon {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Horizon {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Horizon {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Infinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Infinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Infinity {
    type Output = RoundPointAtOrigin;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Infinity {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Infinity {
    type Output = Origin;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Infinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Line {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Line {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Line {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Line {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Line {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Line {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Line {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Plane) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Line {
    type Output = Line;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Line {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for LineAtInfinity {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for LineAtInfinity {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for LineAtInfinity {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for LineAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for LineAtOrigin {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for LineAtOrigin {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Plane) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for LineAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Motor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Plane) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Motor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for Origin {
    type Output = Infinity;

    fn reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Origin {
    type Output = RoundPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Origin {
    type Output = RoundPointAtOrigin;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Origin {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Plane {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Plane {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Plane {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Plane {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Plane {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Plane {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Rotor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Plane) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Rotor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for RoundPoint {
    type Output = Infinity;

    fn reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for RoundPoint {
    type Output = Origin;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for RoundPointAtInfinity {
    type Output = Origin;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for RoundPointBulk {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for RoundPointOnOrigin {
    type Output = Infinity;

    fn reject_orthogonally_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for RoundPointOnOrigin {
    type Output = RoundPointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for RoundPointOnOrigin {
    type Output = RoundPointAtOrigin;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for SpacialCurvature {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for SpacialCurvature {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Infinity> for SpacialCurvature {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Infinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Line) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for SpacialCurvature {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for SpacialCurvature {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for SpacialCurvature {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Sphere {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Sphere {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Sphere {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Sphere {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for Sphere {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Horizon) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Infinity> for Sphere {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Infinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Line) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Sphere {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Sphere {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Sphere {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Sphere {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Sphere {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for SphereWeight {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for SphereWeight {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for SphereWeight {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for SphereWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Horizon> for SphereWeight {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Infinity> for SphereWeight {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Infinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for SphereWeight {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Line) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for SphereWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for SphereWeight {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for SphereWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for SphereWeight {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for SphereWeight {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for SphereWeight {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for SphereWeight {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for SphereWeight {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Transflector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Transflector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Transflector {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Transflector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Transflector {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Transflector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Transflector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Transflector {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Transflector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Transflector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Transflector {
    type Output = SpacialCurvature;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Transflector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: CircleBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Translator {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Plane) -> Translator {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Translator {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Translator {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPointOnOrigin> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SpacialCurvature> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Translator> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectViaOriginFrom<Circle> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Circle {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Circle {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Circle {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Circle {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Circle {
    type Output = CircleBulk;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Circle {
    type Output = Line;

    fn reject_via_origin_from(self, other: Horizon) -> Line {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Circle {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for CircleBulk {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for CircleBulk {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for CircleBulk {
    type Output = Circle;

    fn reject_via_origin_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for CircleBulk {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for CircleBulk {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for CircleBulk {
    type Output = CircleBulk;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for CircleBulk {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> CircleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: Horizon) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for CircleCarrierAspect {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> CircleCarrierAspect {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for CircleWeight {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for CircleWeight {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for CircleWeight {
    type Output = Circle;

    fn reject_via_origin_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for CircleWeight {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: Horizon) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for CircleWeight {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for CircleWeight {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for CircleWeight {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Plane) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for CircleWeight {
    type Output = Circle;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for CircleWeight {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Dipole {
    type Output = FlatPointAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Dipole {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Dipole {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: Horizon) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Dipole {
    type Output = DipoleBulk;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Dipole {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for DipoleBulk {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for DipoleBulk {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for DipoleBulk {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for DipoleBulk {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: Horizon) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for DipoleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> DipoleBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for DipoleCarrierAspect {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for DipoleWeight {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: Horizon) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for DipoleWeight {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Line) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for DipoleWeight {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for DipoleWeight {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for DipoleWeight {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for DipoleWeight {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for FlatPoint {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for FlatPoint {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for FlatPoint {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for FlatPointAtInfinity {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> DipoleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for FlatPointAtOrigin {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for FlatPointAtOrigin {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> FlatPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> FlatPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for FlatPointAtOrigin {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Flector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Flector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Flector {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Flector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Flector {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Flector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Flector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for Flector {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Flector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Flector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Flector {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Flector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for FlectorAtInfinity {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for FlectorAtInfinity {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for FlectorAtInfinity {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for FlectorAtInfinity {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Horizon {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Horizon {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Horizon {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Horizon {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Horizon {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Horizon {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Horizon {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Horizon {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Horizon {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for Horizon {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Horizon {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Plane) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Horizon {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Horizon {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Horizon {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Horizon {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Horizon {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Horizon {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Horizon {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Infinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Infinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Infinity {
    type Output = RoundPointAtOrigin;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Infinity {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Infinity {
    type Output = Origin;

    fn reject_via_origin_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Infinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Line {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Line {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Line {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Line {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Line {
    type Output = Line;

    fn reject_via_origin_from(self, other: Line) -> Line {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Line {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Line {
    type Output = Line;

    fn reject_via_origin_from(self, other: Plane) -> Line {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Line {
    type Output = Line;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Line {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Line {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for LineAtInfinity {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for LineAtInfinity {
    type Output = Circle;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for LineAtInfinity {
    type Output = Line;

    fn reject_via_origin_from(self, other: Line) -> Line {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for LineAtInfinity {
    type Output = Circle;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for LineAtInfinity {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> CircleWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for LineAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for LineAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for LineAtOrigin {
    type Output = Line;

    fn reject_via_origin_from(self, other: Line) -> Line {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for LineAtOrigin {
    type Output = Line;

    fn reject_via_origin_from(self, other: Plane) -> Line {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for LineAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Motor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Plane) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Motor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Origin {
    type Output = Infinity;

    fn reject_via_origin_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Origin {
    type Output = RoundPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Origin {
    type Output = RoundPointAtOrigin;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Origin {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Plane {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Plane {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Plane {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for Plane {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Plane {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Plane {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Rotor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Plane) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Rotor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for RoundPoint {
    type Output = Infinity;

    fn reject_via_origin_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPointBulk;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for RoundPoint {
    type Output = Origin;

    fn reject_via_origin_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointBulk;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for RoundPointAtInfinity {
    type Output = Origin;

    fn reject_via_origin_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn reject_via_origin_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for RoundPointAtOrigin {
    type Output = Origin;

    fn reject_via_origin_from(self, other: SphereWeight) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for RoundPointBulk {
    type Output = RoundPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for RoundPointBulk {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for RoundPointOnOrigin {
    type Output = Infinity;

    fn reject_via_origin_from(self, other: Horizon) -> Infinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for RoundPointOnOrigin {
    type Output = RoundPointAtInfinity;

    fn reject_via_origin_from(self, other: Plane) -> RoundPointAtInfinity {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for RoundPointOnOrigin {
    type Output = RoundPointBulk;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> RoundPointBulk {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for RoundPointOnOrigin {
    type Output = RoundPointAtOrigin;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> RoundPointAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for RoundPointOnOrigin {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointOnOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for SpacialCurvature {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for SpacialCurvature {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Infinity> for SpacialCurvature {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Infinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Line) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for SpacialCurvature {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for SpacialCurvature {
    type Output = Plane;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for SpacialCurvature {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for SpacialCurvature {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for SpacialCurvature {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for SpacialCurvature {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Sphere {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Sphere {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Sphere {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Sphere {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Sphere {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Horizon) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Infinity> for Sphere {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Infinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Line) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Sphere {
    type Output = Plane;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for Sphere {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Sphere {
    type Output = Plane;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Sphere {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Sphere {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Sphere {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for SphereWeight {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for SphereWeight {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for SphereWeight {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for SphereWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for SphereWeight {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Infinity> for SphereWeight {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Infinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for SphereWeight {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Line) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for SphereWeight {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for SphereWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for SphereWeight {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for SphereWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for SphereWeight {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for SphereWeight {
    type Output = Plane;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for SphereWeight {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for SphereWeight {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for SphereWeight {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: CircleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Transflector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: DipoleBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Transflector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Transflector {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Transflector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlatPoint) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Transflector {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> Horizon {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Transflector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Transflector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Origin> for Transflector {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Origin) -> SphereWeight {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Transflector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Transflector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Transflector {
    type Output = SpacialCurvature;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> SpacialCurvature {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Transflector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleBulk> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: CircleBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<CircleWeight> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: DipoleBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Translator {
    type Output = Translator;

    fn reject_via_origin_from(self, other: Plane) -> Translator {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Translator {
    type Output = Translator;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Translator {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Rotor> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPointOnOrigin> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPointOnOrigin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SpacialCurvature> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SpacialCurvature) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<SphereWeight> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}
