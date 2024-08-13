// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::products::exterior::*;
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

impl AntiRejectOrthogonallyFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Circle {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Line) -> Dipole {
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

impl AntiRejectOrthogonallyFrom<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for Flector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Flector {
    type Output = Dipole;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Line {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Line {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for Motor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Motor {
    type Output = Circle;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVector {
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

impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Line) -> RoundPoint {
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

impl AntiRejectOrthogonallyFrom<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Plane) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Circle {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Line) -> Dipole {
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

impl AntiRejectViaHorizonFrom<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for Flector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Circle) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Flector {
    type Output = Dipole;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Dipole {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Line {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Line {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for Motor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Motor {
    type Output = Circle;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Circle {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Circle) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
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

impl AntiRejectViaHorizonFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Circle) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Dipole) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: FlatPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Line) -> RoundPoint {
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

impl AntiRejectViaHorizonFrom<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Plane) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn anti_reject_via_horizon_from(self, other: Sphere) -> RoundPoint {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_reject_via_horizon_from(self, other: RoundPoint) -> Sphere {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Circle {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
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

impl RejectOrthogonallyFrom<Sphere> for Circle {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Line) -> Dipole {
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

impl RejectOrthogonallyFrom<Sphere> for Dipole {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for FlatPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
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
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for FlatPoint {
    type Output = Dipole;

    fn reject_orthogonally_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Flector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Flector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Flector {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
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

impl RejectOrthogonallyFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Flector {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Line) -> Circle {
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

impl RejectOrthogonallyFrom<Sphere> for Line {
    type Output = Circle;

    fn reject_orthogonally_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
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

impl RejectOrthogonallyFrom<RoundPoint> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
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

impl RejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Line) -> Plane {
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

impl RejectOrthogonallyFrom<Plane> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Plane {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
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
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn reject_orthogonally_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Circle> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Dipole> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<FlatPoint> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Sphere {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Line) -> Sphere {
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

impl RejectOrthogonallyFrom<Plane> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Sphere> for Sphere {
    type Output = Sphere;

    fn reject_orthogonally_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectViaOriginFrom<Circle> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: FlatPoint) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Circle {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
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

impl RejectViaOriginFrom<Sphere> for Circle {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Dipole {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Line) -> Dipole {
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

impl RejectViaOriginFrom<Sphere> for Dipole {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for FlatPoint {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Circle) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for FlatPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
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
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Plane) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for FlatPoint {
    type Output = Dipole;

    fn reject_via_origin_from(self, other: Sphere) -> Dipole {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Flector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Flector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Flector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
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

impl RejectViaOriginFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Flector {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Circle) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Dipole) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Line {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Line) -> Circle {
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

impl RejectViaOriginFrom<Sphere> for Line {
    type Output = Circle;

    fn reject_via_origin_from(self, other: Sphere) -> Circle {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: FlatPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
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

impl RejectViaOriginFrom<RoundPoint> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: RoundPoint) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Motor {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Circle) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlatPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Line) -> MultiVector {
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

impl RejectViaOriginFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for MultiVector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Plane {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Line) -> Plane {
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

impl RejectViaOriginFrom<Plane> for Plane {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Plane {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for RoundPoint {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
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
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn reject_via_origin_from(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Circle> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Circle) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Dipole> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Dipole) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlatPoint> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: FlatPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Sphere {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Line) -> Sphere {
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

impl RejectViaOriginFrom<Plane> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Plane) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<RoundPoint> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: RoundPoint) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Sphere> for Sphere {
    type Output = Sphere;

    fn reject_via_origin_from(self, other: Sphere) -> Sphere {
        self.anti_wedge(other).wedge(other.dual())
    }
}
