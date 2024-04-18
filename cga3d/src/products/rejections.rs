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

impl AntiRejectOrthogonallyFrom<Infinity> for Circle {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Circle {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Circle {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Circle {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Circle {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Circle {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Circle {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Circle {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Circle {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for CircleBulk {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for CircleBulk {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for CircleBulk {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for CircleBulk {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPointAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for CircleBulk {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for CircleBulk {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for CircleBulk {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for CircleBulk {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for CircleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for CircleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for CircleWeight {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for CircleWeight {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Flector) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Dipole {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Dipole {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for DipoleBulk {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for DipoleBulk {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> FlatPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for DipoleBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for DipoleBulk {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for DipoleBulk {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlatPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for DipoleBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for DipoleBulk {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for DipoleBulk {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for DipoleBulk {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for DipoleBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for DipoleBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for DipoleBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for DipoleBulk {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for DipoleBulk {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for DipoleBulk {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for DipoleBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Flector) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for DipoleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for DipoleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: Flector) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for DipoleWeight {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for DipoleWeight {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for FlatPoint {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for FlatPoint {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlatPointAtOrigin {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> DipoleBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for FlatPointAtOrigin {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> DipoleBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for FlatPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for FlatPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Infinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Circle) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for Infinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for Infinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Infinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Infinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Infinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Infinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Line {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Line {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Line {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Line {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Line {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Line {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Line {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Line {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for LineAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for LineAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Origin {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Circle) -> CircleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Origin {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for Origin {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for Origin {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for Origin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Origin {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: Line) -> CircleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for Origin {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> CircleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Origin {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: Motor) -> CircleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Origin {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Origin {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Origin {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Origin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for Origin {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: Translator) -> CircleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Plane {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Plane {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Rotor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Line) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for RoundPoint {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Motor) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for RoundPoint {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPoint {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Translator) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPointAtInfinity {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointAtInfinity {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> FlatPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> FlatPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointAtInfinity {
    type Output = Flector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> Flector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: Line) -> Line {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> Line {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for RoundPointAtInfinity {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: Motor) -> Line {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> Line {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for RoundPointAtInfinity {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointAtInfinity {
    type Output = Transflector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Transflector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Translator) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> RoundPointAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Line) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for RoundPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Motor) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPointAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for RoundPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Translator) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointBulk {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointBulk {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> CircleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPointBulk {
    type Output = CircleWeight;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> CircleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointBulk {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointBulk {
    type Output = DipoleBulk;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> DipoleBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointBulk {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPointBulk {
    type Output = DipoleWeight;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointBulk {
    type Output = FlatPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> FlatPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPointBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPointBulk {
    type Output = FlatPointAtOrigin;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointBulk {
    type Output = Flector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> Flector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPointBulk {
    type Output = Infinity;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> Infinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointBulk {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: Line) -> Line {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPointBulk {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPointBulk {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for RoundPointBulk {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointBulk {
    type Output = Line;

    fn anti_reject_orthogonally_from(self, other: Motor) -> Line {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPointBulk {
    type Output = Origin;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Origin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPointBulk {
    type Output = LineAtOrigin;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> LineAtOrigin {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for RoundPointBulk {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointBulk {
    type Output = Transflector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Transflector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointBulk {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Translator) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleBulk> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: CircleBulk) -> CircleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<CircleWeight> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: CircleWeight) -> CircleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Infinity> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Line) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Motor) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: Origin) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Translator) -> Circle {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Sphere {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Sphere {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleBulk> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Transflector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointBulk> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Translator {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for Circle {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Circle {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Circle {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Circle {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Circle {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Circle {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Circle {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Circle {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Circle {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for CircleBulk {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for CircleBulk {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for CircleBulk {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for CircleBulk {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPointAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for CircleBulk {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for CircleBulk {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for CircleBulk {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for CircleBulk {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for CircleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for CircleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for CircleWeight {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for CircleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for CircleWeight {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Dipole {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Dipole {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Dipole {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for DipoleBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for DipoleBulk {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for DipoleBulk {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for DipoleBulk {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> FlatPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for DipoleBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for DipoleBulk {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for DipoleBulk {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: Flector) -> FlatPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for DipoleBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for DipoleBulk {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for DipoleBulk {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for DipoleBulk {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for DipoleBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for DipoleBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for DipoleBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for DipoleBulk {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for DipoleBulk {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for DipoleBulk {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for DipoleBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for DipoleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for DipoleCarrierAspect {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for DipoleCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: Flector) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for DipoleWeight {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for DipoleWeight {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for DipoleWeight {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for FlatPoint {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for FlatPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for FlatPoint {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for FlatPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlatPointAtOrigin {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> DipoleBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for FlatPointAtOrigin {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> DipoleBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for FlatPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for FlatPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Infinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Circle) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for Infinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for Infinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Infinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Infinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Infinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Infinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Line {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Line {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Line {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Line {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Line {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Line {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Line {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Line {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for LineAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for LineAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for LineAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Circle) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: SphereWeight) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Origin {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Circle) -> CircleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Origin {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for Origin {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for Origin {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for Origin {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: Line) -> CircleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Origin {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> CircleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Origin {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: Motor) -> CircleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Origin {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Origin {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Origin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Origin {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: Translator) -> CircleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Plane {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Plane {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Line) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for RoundPoint {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for RoundPoint {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPoint {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Translator) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPointAtInfinity {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPointAtInfinity {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointAtInfinity {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> FlatPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> FlatPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPointAtInfinity {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Line) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for RoundPointAtInfinity {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPointAtInfinity {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for RoundPointAtInfinity {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointAtInfinity {
    type Output = Transflector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Transflector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> RoundPointAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Line) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for RoundPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPointAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for RoundPointAtOrigin {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointAtOrigin {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Translator) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointBulk {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointBulk {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> CircleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPointBulk {
    type Output = CircleWeight;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> CircleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointBulk {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointBulk {
    type Output = DipoleBulk;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> DipoleBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointBulk {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPointBulk {
    type Output = DipoleWeight;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleWeight {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointBulk {
    type Output = FlatPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> FlatPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPointBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPointBulk {
    type Output = FlatPointAtOrigin;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointBulk {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointBulk {
    type Output = FlatPointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> FlatPointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPointBulk {
    type Output = Infinity;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> Infinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointBulk {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Line) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPointBulk {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPointBulk {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for RoundPointBulk {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointBulk {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPointBulk {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPointBulk {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> LineAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointBulk {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointBulk {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for RoundPointBulk {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointBulk {
    type Output = Transflector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Transflector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointBulk {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleBulk> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: CircleBulk) -> CircleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<CircleWeight> for RoundPointCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: CircleWeight) -> CircleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for RoundPointCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> DipoleCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Infinity> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Infinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Line) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: Origin) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> RoundPoint {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> RoundPointCarrierAspect {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for RoundPointCarrierAspect {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for RoundPointCarrierAspect {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Translator) -> Circle {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Sphere {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Sphere {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleBulk> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: DipoleWeight) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointBulk> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointBulk) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Circle {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Circle {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Circle {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Circle {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Circle {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Circle {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Circle {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for CircleBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for CircleBulk {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: Circle) -> LineAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for CircleBulk {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for CircleBulk {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for CircleBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: Sphere) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for CircleBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for CircleWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: Circle) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for CircleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Horizon) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for CircleWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: Line) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for CircleWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for CircleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Plane) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for CircleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Sphere) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for CircleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Dipole {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Flector) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Dipole {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Dipole {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Transflector) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for DipoleBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: Flector) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for DipoleBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: Sphere) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for DipoleBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Flector) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Transflector) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Flector) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Horizon) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Plane) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Sphere) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Transflector) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for FlatPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for FlatPoint {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for FlatPoint {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for FlatPoint {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Horizon) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for FlatPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for FlatPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for FlatPoint {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for FlatPoint {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlatPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlatPoint {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for FlatPoint {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlatPoint {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for FlatPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlatPointAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Sphere) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for FlatPointAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Flector) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Plane) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlatPointAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for FlatPointAtOrigin {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Transflector) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Horizon {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Circle) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Horizon {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Horizon {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: CircleWeight) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Horizon {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: Dipole) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Horizon {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Horizon {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Horizon {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Sphere) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Line {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Horizon) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Line {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Line {
    type Output = Line;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Line {
    type Output = Line;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Line {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Line {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Line {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Circle) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: CircleWeight) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Sphere) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for LineAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for LineAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for LineAtOrigin {
    type Output = CircleBulk;

    fn reject_orthogonally_from(self, other: CircleBulk) -> CircleBulk {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for LineAtOrigin {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for LineAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for LineAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for LineAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for LineAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for LineAtOrigin {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for LineAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Infinity> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Plane {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Plane {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Plane {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Plane {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Plane {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Plane {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Plane {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Plane {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Plane {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Plane {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPoint) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Plane {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Plane {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Plane {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Plane {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Plane {
    type Output = Line;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Plane {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Plane {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Plane {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Plane {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Translator) -> Translator {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for PlaneAtOrigin {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for PlaneAtOrigin {
    type Output = CircleBulk;

    fn reject_orthogonally_from(self, other: CircleBulk) -> CircleBulk {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = CircleCarrierAspect;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> DipoleBulk {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = DipoleCarrierAspect;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> DipoleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPoint) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for PlaneAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for PlaneAtOrigin {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for PlaneAtOrigin {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for PlaneAtOrigin {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for PlaneAtOrigin {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for PlaneAtOrigin {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Translator) -> Translator {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Infinity> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for RoundPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for RoundPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPoint {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Sphere {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Sphere {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Sphere {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Sphere {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Sphere {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Sphere {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Sphere {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Sphere {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Sphere {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Sphere {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Sphere {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Sphere {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Sphere {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Sphere {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Sphere {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Sphere {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for SphereWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: Circle) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for SphereWeight {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: Dipole) -> DipoleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for SphereWeight {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: FlatPoint) -> DipoleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for SphereWeight {
    type Output = DipoleWeight;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> DipoleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for SphereWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Horizon) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for SphereWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: Line) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for SphereWeight {
    type Output = CircleWeight;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> CircleWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for SphereWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Plane) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for SphereWeight {
    type Output = SphereWeight;

    fn reject_orthogonally_from(self, other: Sphere) -> SphereWeight {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for SphereWeight {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Circle> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleBulk> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<CircleWeight> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Dipole> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlatPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Infinity> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointBulk> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Sphere> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<SphereWeight> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Circle {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Circle {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Circle {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Circle {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Circle {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Circle {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Circle {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for CircleBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for CircleBulk {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Circle) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for CircleBulk {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for CircleBulk {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for CircleBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Sphere) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for CircleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for CircleBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for CircleCarrierAspect {
    type Output = Circle;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for CircleCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for CircleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for CircleWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: Circle) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for CircleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Horizon) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for CircleWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: Line) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for CircleWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for CircleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Plane) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for CircleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Sphere) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for CircleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Dipole {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Flector) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Dipole {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Dipole {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Transflector) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for DipoleBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for DipoleBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Sphere) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for DipoleBulk {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for DipoleBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Flector) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for DipoleCarrierAspect {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Transflector) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Flector) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Horizon) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Plane) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Sphere) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for DipoleWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Transflector) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for FlatPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for FlatPoint {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for FlatPoint {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for FlatPoint {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Horizon) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for FlatPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for FlatPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for FlatPoint {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for FlatPoint {
    type Output = Plane;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for FlatPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for FlatPoint {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for FlatPoint {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for FlatPoint {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for FlatPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for FlatPointAtInfinity {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Sphere) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for FlatPointAtInfinity {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: SphereWeight) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Flector) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Plane) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for FlatPointAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for FlatPointAtOrigin {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for FlatPointAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Transflector) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for FlatPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Horizon {
    type Output = Line;

    fn reject_via_origin_from(self, other: Circle) -> Line {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Horizon {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Horizon {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: CircleWeight) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Horizon {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: Dipole) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Horizon {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Horizon {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: DipoleWeight) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Horizon {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Sphere) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Horizon {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: SphereWeight) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Infinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Line {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Line {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Line {
    type Output = Plane;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Line {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Horizon) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Line {
    type Output = Line;

    fn reject_via_origin_from(self, other: Line) -> Line {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Line {
    type Output = Line;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Line {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Line {
    type Output = Line;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Line {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Line {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Line {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Line {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Line {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Line {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: Circle) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: CircleWeight) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Sphere) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for LineAtInfinity {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: SphereWeight) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for LineAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for LineAtOrigin {
    type Output = CircleBulk;

    fn reject_via_origin_from(self, other: CircleBulk) -> CircleBulk {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for LineAtOrigin {
    type Output = CircleCarrierAspect;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for LineAtOrigin {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for LineAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for LineAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: Line) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for LineAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for LineAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for LineAtOrigin {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for LineAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Infinity> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Origin> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPoint> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointCarrierAspect> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Infinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointCarrierAspect> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Plane {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Plane {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Plane {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Plane {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Plane {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Plane {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Dipole) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Plane {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Plane {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Plane {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: DipoleWeight) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for Plane {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: FlatPoint) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Plane {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Plane {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Plane {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Plane {
    type Output = Line;

    fn reject_via_origin_from(self, other: Line) -> Line {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Plane {
    type Output = Line;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Line {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Plane {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Plane {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Plane {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Plane {
    type Output = Translator;

    fn reject_via_origin_from(self, other: Translator) -> Translator {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for PlaneAtOrigin {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for PlaneAtOrigin {
    type Output = CircleBulk;

    fn reject_via_origin_from(self, other: CircleBulk) -> CircleBulk {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = CircleCarrierAspect;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> CircleCarrierAspect {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for PlaneAtOrigin {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: CircleWeight) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Dipole) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for PlaneAtOrigin {
    type Output = DipoleBulk;

    fn reject_via_origin_from(self, other: DipoleBulk) -> DipoleBulk {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = DipoleCarrierAspect;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> DipoleCarrierAspect {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for PlaneAtOrigin {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: DipoleWeight) -> DipoleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: FlatPoint) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for PlaneAtOrigin {
    type Output = Horizon;

    fn reject_via_origin_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for PlaneAtOrigin {
    type Output = Line;

    fn reject_via_origin_from(self, other: Line) -> Line {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for PlaneAtOrigin {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for PlaneAtOrigin {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for PlaneAtOrigin {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: SphereWeight) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for PlaneAtOrigin {
    type Output = Translator;

    fn reject_via_origin_from(self, other: Translator) -> Translator {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Infinity> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Origin> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointCarrierAspect> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Rotor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for RoundPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for RoundPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for RoundPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for RoundPoint {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointBulk {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Sphere {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Sphere {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Sphere {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleBulk) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Sphere {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Sphere {
    type Output = Circle;

    fn reject_via_origin_from(self, other: CircleWeight) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Sphere {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Dipole) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Sphere {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: DipoleBulk) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Sphere {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Sphere {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: DipoleWeight) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for Sphere {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: FlatPoint) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Sphere {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Sphere {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> Dipole {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Horizon) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Sphere {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Sphere {
    type Output = Circle;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Sphere {
    type Output = Circle;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Circle {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Sphere {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: SphereWeight) -> Sphere {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for SphereWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: Circle) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for SphereWeight {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: Dipole) -> DipoleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for SphereWeight {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: FlatPoint) -> DipoleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for SphereWeight {
    type Output = DipoleWeight;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> DipoleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for SphereWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Horizon) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for SphereWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: Line) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for SphereWeight {
    type Output = CircleWeight;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> CircleWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for SphereWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Plane) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for SphereWeight {
    type Output = SphereWeight;

    fn reject_via_origin_from(self, other: Sphere) -> SphereWeight {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for SphereWeight {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Circle> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleBulk> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<CircleWeight> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: CircleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Dipole> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleBulk> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<DipoleWeight> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: DipoleWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPoint> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlatPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Infinity> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Origin> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPoint> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointBulk> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointBulk) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<RoundPointCarrierAspect> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPointCarrierAspect) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Sphere> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<SphereWeight> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: SphereWeight) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Translator {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}
