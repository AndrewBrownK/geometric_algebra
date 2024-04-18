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

impl AntiRejectOrthogonallyFrom<Flector> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Flector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Flector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for Flector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for Flector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Flector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Horizon {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Horizon {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Horizon {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Point) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Line {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Line {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Line {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Line {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Line {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Scalar {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for LineAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Scalar {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Motor {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Motor {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Motor {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Motor {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Motor {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Scalar {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for MultiVectorAtOrigin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Origin {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Origin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Origin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for Origin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Origin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Origin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Plane {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Plane {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Plane {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Origin) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Plane {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Point) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Plane {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for PlaneAtOrigin {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for PlaneAtOrigin {
    type Output = Horizon;

    fn anti_reject_orthogonally_from(self, other: Point) -> Horizon {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Point {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Point {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Point {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Point {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for PointAtInfinity {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Rotor {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Rotor {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Rotor {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> Scalar {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Transflector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Transflector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Transflector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> FlectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Translator {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Translator {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Translator {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Translator {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Translator {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.anti_dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for Flector {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Flector {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Line) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Flector {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Flector {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for Flector {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Point) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Flector {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Translator) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for FlectorAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for FlectorAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Line) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for FlectorAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for FlectorAtInfinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for FlectorAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Point) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for FlectorAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Horizon {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Horizon {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Point) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Line {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Line) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Line {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Line {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Line {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Point) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Line {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Line {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Translator) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Line) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for LineAtInfinity {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Point) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Line) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Point) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Translator) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Motor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Line) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Motor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Motor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Motor {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Point) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Motor {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Motor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Translator) -> LineAtOrigin {
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

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Point) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> MultiVector {
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

impl AntiRejectViaHorizonFrom<Flector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for MultiVectorAtInfinity {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Scalar {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Point) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for MultiVectorAtOrigin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for MultiVectorAtOrigin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Point) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Origin {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Origin {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Line) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Origin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Origin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Point) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Origin {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Origin {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Translator) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Plane {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Plane {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Point) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Plane {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Plane {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Point) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Point {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Point {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for Point {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Line) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Point {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Point {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for Point {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Point {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Translator) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for PointAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for PointAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Line) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for PointAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for PointAtInfinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for PointAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for PointAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Line) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Point) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Rotor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Rotor {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Translator) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Transflector {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Line) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Transflector {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Point {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Plane> for Transflector {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Point) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Transflector {
    type Output = Transflector;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Transflector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> PointAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Translator {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Line> for Translator {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Line) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Translator {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Motor) -> LineAtOrigin {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Point> for Translator {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Point) -> Line {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> LineAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Translator {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).anti_wedge(other.dual())
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Flector {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Origin) -> Horizon {
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

impl RejectOrthogonallyFrom<Point> for Flector {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Point) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for FlectorAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Line) -> Horizon {
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
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for FlectorAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Origin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Point> for FlectorAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Point) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Line) -> Horizon {
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
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Origin) -> Horizon {
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

impl RejectOrthogonallyFrom<Point> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Point) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Horizon {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Transflector) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Line {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Line {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
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

impl RejectOrthogonallyFrom<Transflector> for Line {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Transflector) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Flector) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
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
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
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
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Transflector) -> LineAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for LineAtOrigin {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
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
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for LineAtOrigin {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Transflector) -> Line {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Motor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Origin) -> AntiScalar {
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

impl RejectOrthogonallyFrom<Point> for Motor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Point) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Motor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Transflector) -> Motor {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
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

impl RejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Point) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Line) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for MultiVectorAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Origin) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Point> for MultiVectorAtInfinity {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Point) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for MultiVectorAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Origin) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Point> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Point) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Origin {
    type Output = FlectorAtInfinity;

    fn reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Origin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Origin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Origin {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Origin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Origin {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Plane {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Origin) -> Horizon {
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

impl RejectOrthogonallyFrom<Point> for Plane {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Point) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
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

impl RejectOrthogonallyFrom<Point> for PlaneAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Point) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Point {
    type Output = FlectorAtInfinity;

    fn reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for Point {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Point {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for Point {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Point {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Point {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Point {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for PointAtInfinity {
    type Output = FlectorAtInfinity;

    fn reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Motor> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Plane> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Rotor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Line> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Origin) -> AntiScalar {
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

impl RejectOrthogonallyFrom<Point> for Rotor {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Point) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Rotor {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Transflector) -> Motor {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Transflector {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Origin) -> Horizon {
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

impl RejectOrthogonallyFrom<Point> for Transflector {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Point) -> Horizon {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Flector> for Translator {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Flector) -> Motor {
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

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Origin> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Origin) -> AntiScalar {
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

impl RejectOrthogonallyFrom<Point> for Translator {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Point) -> AntiScalar {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Rotor> for Translator {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).wedge(other.anti_dual())
    }
}

impl RejectOrthogonallyFrom<Transflector> for Translator {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Transflector) -> Translator {
        self.anti_wedge(other).wedge(other.anti_dual())
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
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Flector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Flector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Flector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Flector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
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
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> PlaneAtOrigin {
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
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for FlectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
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
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> PlaneAtOrigin {
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
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Horizon {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Horizon {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Horizon {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for Horizon {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Horizon {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Line {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Line {
    type Output = Motor;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Line {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Line {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Line {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Line {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Line {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Transflector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for LineAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for LineAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Transflector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for LineAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for LineAtOrigin {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = Motor;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> LineAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for LineAtOrigin {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Transflector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Motor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Motor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Motor {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Horizon) -> Rotor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Motor {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Plane) -> Rotor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Point) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Motor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Transflector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
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
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
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
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for MultiVectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Origin {
    type Output = Flector;

    fn reject_via_origin_from(self, other: Flector) -> Flector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Origin {
    type Output = Flector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Flector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Origin {
    type Output = Origin;

    fn reject_via_origin_from(self, other: Horizon) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Origin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Origin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Origin {
    type Output = Origin;

    fn reject_via_origin_from(self, other: Plane) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Origin {
    type Output = Flector;

    fn reject_via_origin_from(self, other: Transflector) -> Flector {
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

impl RejectViaOriginFrom<Horizon> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
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
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> PlaneAtOrigin {
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

impl RejectViaOriginFrom<Horizon> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
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
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Point {
    type Output = Flector;

    fn reject_via_origin_from(self, other: Flector) -> Flector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Point {
    type Output = Flector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Flector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Point {
    type Output = Origin;

    fn reject_via_origin_from(self, other: Horizon) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Point {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Point {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Point {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Point {
    type Output = Origin;

    fn reject_via_origin_from(self, other: Plane) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Point {
    type Output = Flector;

    fn reject_via_origin_from(self, other: Transflector) -> Flector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Point {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for PointAtInfinity {
    type Output = Flector;

    fn reject_via_origin_from(self, other: Flector) -> Flector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for PointAtInfinity {
    type Output = Origin;

    fn reject_via_origin_from(self, other: Plane) -> Origin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for PointAtInfinity {
    type Output = Flector;

    fn reject_via_origin_from(self, other: Transflector) -> Flector {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Rotor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Rotor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Rotor {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Horizon) -> Rotor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Rotor {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Plane) -> Rotor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Point) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Rotor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Rotor {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Transflector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
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

impl RejectViaOriginFrom<Horizon> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Transflector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Transflector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Transflector {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
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
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> PlaneAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Flector> for Translator {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Flector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Translator {
    type Output = Motor;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Horizon> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Horizon) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Line> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Motor> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVector> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Plane> for Translator {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Plane) -> Rotor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Point> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Point) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Transflector> for Translator {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Transflector) -> Motor {
        self.anti_wedge(other).wedge(other.dual())
    }
}

impl RejectViaOriginFrom<Translator> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).wedge(other.dual())
    }
}
