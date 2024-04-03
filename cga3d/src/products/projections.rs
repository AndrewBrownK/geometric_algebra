//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspect_duals::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::products::exterior::AntiWedge;
use crate::products::exterior::Wedge;
use crate::*;

/// Orthogonal Projection
/// Typically involves bringing a lower dimensional object to a higher dimensional object
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectOrthogonallyOnto<T> {
    type Output;
    fn project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Orthogonal AntiProjection
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait AntiProjectOrthogonallyOnto<T> {
    type Output;
    fn anti_project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Central (to origin) Projection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectViaOriginOnto<T> {
    type Output;
    fn project_via_origin_onto(self, other: T) -> Self::Output;
}

/// Outward (to horizon) AntiProjection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
pub trait AntiProjectViaHorizonOnto<T> {
    type Output;
    fn anti_project_via_horizon_onto(self, other: T) -> Self::Output;
}

impl AntiProjectViaHorizonOnto<Circle> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Circle {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Circle {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Dipole {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for FlatPoint {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPoint {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> FlatPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> FlatPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for FlatPointAtOrigin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> FlatPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Flector {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Flector {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Infinity {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Infinity {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Line {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Line {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Line {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for LineAtInfinity {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtInfinity {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Plane) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Plane {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Line) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Motor) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Translator) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Circle) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Line) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Motor) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Translator) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPointAtInfinity {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Infinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> RoundPoint {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<FlatPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Plane) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPointAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Circle {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Circle {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Circle {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleBulk {
    type Output = CircleBulk;

    fn project_orthogonally_onto(self, other: Translator) -> CircleBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: Plane) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for CircleCarrierAspect {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> CircleWeight {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: Sphere) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: Plane) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for CircleWeight {
    type Output = CircleWeight;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> CircleWeight {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for CircleWeight {
    type Output = CircleCarrierAspect;

    fn project_orthogonally_onto(self, other: Sphere) -> CircleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for CircleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Dipole {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: Plane) -> DipoleBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: Sphere) -> DipoleBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_orthogonally_onto(self, other: Translator) -> DipoleBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Circle) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Line) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Plane) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Sphere) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Circle) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Line) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Plane) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for DipoleWeight {
    type Output = DipoleWeight;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> DipoleWeight {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for DipoleWeight {
    type Output = DipoleCarrierAspect;

    fn project_orthogonally_onto(self, other: Sphere) -> DipoleCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for DipoleWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPoint {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPoint {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Translator) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Line) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_orthogonally_onto(self, other: Translator) -> FlatPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for FlatPointAtOrigin {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for FlatPointAtOrigin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: Translator) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Flector {
    type Output = FlatPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> FlatPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Flector {
    type Output = FlatPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> FlatPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Horizon {
    type Output = Horizon;

    fn project_orthogonally_onto(self, other: Translator) -> Horizon {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Infinity {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Infinity {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Line) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Plane) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Infinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Infinity {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Translator) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Line {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Line {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Line {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Translator) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for LineAtOrigin {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Motor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Motor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Line) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Origin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Origin {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Translator) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Plane {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Plane {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Rotor {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Rotor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Translator) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointAtInfinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointAtInfinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Infinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_orthogonally_onto(self, other: Translator) -> RoundPointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_orthogonally_onto(self, other: Translator) -> RoundPointAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPointBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: Line) -> RoundPointBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPointBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPointBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPointBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointBulk {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPointBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_orthogonally_onto(self, other: Translator) -> RoundPointBulk {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPoint> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: FlatPoint) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<FlatPointAtOrigin> for RoundPointCarrierAspect {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: FlatPointAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Line) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Plane) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_orthogonally_onto(self, other: Translator) -> RoundPointCarrierAspect {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Sphere {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Sphere {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for SphereWeight {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Translator {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> LineAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Translator {
    type Output = Translator;

    fn project_orthogonally_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Horizon) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Circle {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Circle {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Circle {
    type Output = Motor;

    fn project_via_origin_onto(self, other: Motor) -> Motor {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Circle {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Plane) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Circle {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Circle {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for CircleBulk {
    type Output = CircleBulk;

    fn project_via_origin_onto(self, other: Horizon) -> CircleBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: Plane) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleBulk {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: Sphere) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for CircleCarrierAspect {
    type Output = CircleBulk;

    fn project_via_origin_onto(self, other: Horizon) -> CircleBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: Plane) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn project_via_origin_onto(self, other: Sphere) -> CircleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for CircleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for Dipole {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Horizon) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Line) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Plane) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Dipole {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Dipole {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Circle) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: Horizon) -> DipoleBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Line) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for DipoleBulk {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> DipoleBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Plane) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleBulk {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Sphere) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Circle) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: Horizon) -> DipoleBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Line) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for DipoleCarrierAspect {
    type Output = DipoleBulk;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> DipoleBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Plane) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn project_via_origin_onto(self, other: Sphere) -> DipoleCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for DipoleCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for DipoleWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for FlatPoint {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPoint {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Motor) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPoint {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPoint {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPoint {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for FlatPointAtInfinity {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Motor) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for FlatPointAtInfinity {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Plane) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for FlatPointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for FlatPointAtInfinity {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for FlatPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Flector {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for Flector {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Horizon) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Flector {
    type Output = FlatPoint;

    fn project_via_origin_onto(self, other: Line) -> FlatPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Flector {
    type Output = FlatPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> FlatPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Motor) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Plane) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Flector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Horizon {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Horizon {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Line) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Plane) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for Infinity {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for Infinity {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Infinity {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Infinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Line {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Line {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Line {
    type Output = Motor;

    fn project_via_origin_onto(self, other: Motor) -> Motor {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Line {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Line {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for LineAtInfinity {
    type Output = Motor;

    fn project_via_origin_onto(self, other: Motor) -> Motor {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for LineAtInfinity {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for LineAtInfinity {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for LineAtInfinity {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Motor {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Motor {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = Motor;

    fn project_via_origin_onto(self, other: Motor) -> Motor {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Motor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Motor {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Circle) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Dipole) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPoint) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Horizon) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Infinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Line) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Plane) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPoint) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Sphere) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Plane {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Plane {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPoint {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPoint {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPoint {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPointAtInfinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Line) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Plane) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointAtInfinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPoint) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Horizon) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Infinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Infinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Line) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointAtOrigin {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Plane) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPointAtOrigin {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = RoundPointAtInfinity;

    fn project_via_origin_onto(self, other: RoundPointAtInfinity) -> RoundPointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: RoundPointAtOrigin) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPointAtOrigin {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Circle) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: Horizon) -> RoundPointBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Line) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointBulk {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> RoundPointBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Plane) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointBulk {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointBulk {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Circle) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Dipole) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPoint> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: FlatPoint) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<FlatPointAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: FlatPointAtInfinity) -> RoundPointBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: Horizon) -> RoundPointBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Line) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for RoundPointCarrierAspect {
    type Output = RoundPointBulk;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> RoundPointBulk {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Plane) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for RoundPointCarrierAspect {
    type Output = RoundPointCarrierAspect;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPointCarrierAspect {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for RoundPointCarrierAspect {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Sphere {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Sphere {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Sphere {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Sphere {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for SphereWeight {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Translator {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Translator {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Translator {
    type Output = Motor;

    fn project_via_origin_onto(self, other: Motor) -> Motor {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Translator {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Translator {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Translator {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Translator {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.bulk_expansion(other))
    }
}
