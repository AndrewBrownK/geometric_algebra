//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
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

impl AntiProjectViaHorizonOnto<Point> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Point) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Circle {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Circle {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Circle {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
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
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Dipole {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Point> for Dipole {
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Point) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Dipole {
    type Output = PointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> PointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Dipole {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
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

impl AntiProjectViaHorizonOnto<Point> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Point) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Flector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
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
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Plane {
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
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Plane {
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

impl AntiProjectViaHorizonOnto<Point> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Point) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Horizon {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
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
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Plane {
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

impl AntiProjectViaHorizonOnto<Point> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Point) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Line {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Line {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
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

impl AntiProjectViaHorizonOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Line {
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

impl AntiProjectViaHorizonOnto<Point> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Point) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for LineAtInfinity {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtInfinity {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtInfinity {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Circle) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Circle {
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
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Line) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for LineAtOrigin {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Point> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: Point) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for LineAtOrigin {
    type Output = Line;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Line {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for LineAtOrigin {
    type Output = Circle;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Circle {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtOrigin {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
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

impl AntiProjectViaHorizonOnto<Point> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Point) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
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

impl AntiProjectViaHorizonOnto<Point> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Point) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
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

impl AntiProjectViaHorizonOnto<Dipole> for Origin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Origin {
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Point> for Origin {
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Point) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Origin {
    type Output = PointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> PointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Origin {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
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

impl AntiProjectViaHorizonOnto<Point> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Point) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Plane {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
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

impl AntiProjectViaHorizonOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for PlaneAtOrigin {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Line> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Line) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Motor) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Plane) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Point> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Point) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: Sphere) -> Sphere {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for PlaneAtOrigin {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Point {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Point {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for Point {
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Point {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Point> for Point {
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Point) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> PointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Point {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Infinity> for PointAtInfinity {
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Infinity) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Point> for PointAtInfinity {
    type Output = Point;

    fn anti_project_via_horizon_onto(self, other: Point) -> Point {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> PointAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for PointAtInfinity {
    type Output = Dipole;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Dipole {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Circle> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Circle) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Dipole> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Dipole) -> MultiVector {
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
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Line) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: LineAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Motor) -> MultiVector {
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

impl AntiProjectViaHorizonOnto<Point> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Point) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
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
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
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

impl AntiProjectViaHorizonOnto<Point> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Point) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Sphere {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Sphere {
    type Output = Sphere;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> Sphere {
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

impl AntiProjectViaHorizonOnto<Point> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Point) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<RoundPoint> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: RoundPoint) -> MultiVector {
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

impl ProjectOrthogonallyOnto<Origin> for Dipole {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
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

impl ProjectOrthogonallyOnto<Point> for Dipole {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Point) -> Point {
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

impl ProjectOrthogonallyOnto<Flector> for Flector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Flector {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Line) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Flector {
    type Output = Point;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Point {
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

impl ProjectOrthogonallyOnto<Origin> for Flector {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
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

impl ProjectOrthogonallyOnto<Point> for Flector {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Point) -> Point {
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

impl ProjectOrthogonallyOnto<Plane> for Horizon {
    type Output = Plane;

    fn project_orthogonally_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Horizon {
    type Output = PlaneAtOrigin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Horizon {
    type Output = Sphere;

    fn project_orthogonally_onto(self, other: Sphere) -> Sphere {
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
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Circle) -> RoundPoint {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Infinity {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Dipole) -> RoundPoint {
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

impl ProjectOrthogonallyOnto<Origin> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Origin) -> Infinity {
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

impl ProjectOrthogonallyOnto<Point> for Infinity {
    type Output = Infinity;

    fn project_orthogonally_onto(self, other: Point) -> Infinity {
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
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Sphere) -> RoundPoint {
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

impl ProjectOrthogonallyOnto<Circle> for LineAtInfinity {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for LineAtInfinity {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
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
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for LineAtInfinity {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
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

impl ProjectOrthogonallyOnto<Origin> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Origin) -> MultiVector {
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

impl ProjectOrthogonallyOnto<Point> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Point) -> MultiVector {
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
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Origin {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
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
    type Output = Point;

    fn project_orthogonally_onto(self, other: Line) -> Point {
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

impl ProjectOrthogonallyOnto<Origin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Origin {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Plane) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Point> for Origin {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Point) -> Point {
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
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
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

impl ProjectOrthogonallyOnto<Circle> for Point {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for Point {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Point {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Point {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Line) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Point {
    type Output = Point;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for Point {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Point {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Origin> for Point {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for Point {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Plane) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Point {
    type Output = Point;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Point> for Point {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Point) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for Point {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for Point {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Point {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Translator) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Circle> for PointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for PointAtInfinity {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Line) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for PointAtInfinity {
    type Output = Point;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Motor> for PointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Motor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Origin> for PointAtInfinity {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Origin) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for PointAtInfinity {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Plane) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for PointAtInfinity {
    type Output = Point;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Point> for PointAtInfinity {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Point) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Rotor> for PointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Rotor) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Sphere> for PointAtInfinity {
    type Output = Dipole;

    fn project_orthogonally_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_orthogonally_onto(self, other: Translator) -> PointAtInfinity {
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

impl ProjectOrthogonallyOnto<Origin> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Origin) -> RoundPoint {
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

impl ProjectOrthogonallyOnto<Point> for RoundPoint {
    type Output = RoundPoint;

    fn project_orthogonally_onto(self, other: Point) -> RoundPoint {
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

impl ProjectOrthogonallyOnto<Circle> for Translator {
    type Output = Circle;

    fn project_orthogonally_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Translator {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for Translator {
    type Output = Line;

    fn project_orthogonally_onto(self, other: Line) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for Translator {
    type Output = LineAtOrigin;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> LineAtOrigin {
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
    type Output = Line;

    fn project_orthogonally_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for Translator {
    type Output = Line;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> Line {
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

impl ProjectViaOriginOnto<Point> for Dipole {
    type Output = Point;

    fn project_via_origin_onto(self, other: Point) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for Dipole {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> PointAtInfinity {
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
    type Output = Point;

    fn project_via_origin_onto(self, other: Line) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Flector {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> PointAtInfinity {
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

impl ProjectViaOriginOnto<Point> for Flector {
    type Output = Point;

    fn project_via_origin_onto(self, other: Point) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for Flector {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> PointAtInfinity {
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

impl ProjectViaOriginOnto<Point> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: Point) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for Infinity {
    type Output = Infinity;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> Infinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Infinity {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Sphere) -> RoundPoint {
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

impl ProjectViaOriginOnto<Circle> for LineAtOrigin {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for LineAtOrigin {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for LineAtOrigin {
    type Output = Motor;

    fn project_via_origin_onto(self, other: Motor) -> Motor {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for LineAtOrigin {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for LineAtOrigin {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for LineAtOrigin {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
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

impl ProjectViaOriginOnto<Point> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Point) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: RoundPoint) -> MultiVector {
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

impl ProjectViaOriginOnto<Circle> for Origin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Origin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Origin {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Origin {
    type Output = Point;

    fn project_via_origin_onto(self, other: Line) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Origin {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Origin {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Motor) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Origin {
    type Output = Point;

    fn project_via_origin_onto(self, other: Plane) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Point> for Origin {
    type Output = Point;

    fn project_via_origin_onto(self, other: Point) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for Origin {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Origin {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Origin {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
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

impl ProjectViaOriginOnto<Horizon> for PlaneAtOrigin {
    type Output = Horizon;

    fn project_via_origin_onto(self, other: Horizon) -> Horizon {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn project_via_origin_onto(self, other: Plane) -> Plane {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for PlaneAtOrigin {
    type Output = Sphere;

    fn project_via_origin_onto(self, other: Sphere) -> Sphere {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Point {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for Point {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Point {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Point {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Point {
    type Output = Point;

    fn project_via_origin_onto(self, other: Line) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Point {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Motor) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Point {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Point {
    type Output = Point;

    fn project_via_origin_onto(self, other: Plane) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Point> for Point {
    type Output = Point;

    fn project_via_origin_onto(self, other: Point) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Point {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Point {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for PointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Circle) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Dipole> for PointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Dipole) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for PointAtInfinity {
    type Output = Point;

    fn project_via_origin_onto(self, other: Line) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for PointAtInfinity {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Motor) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for PointAtInfinity {
    type Output = Point;

    fn project_via_origin_onto(self, other: Plane) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Point> for PointAtInfinity {
    type Output = Point;

    fn project_via_origin_onto(self, other: Point) -> Point {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> PointAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for PointAtInfinity {
    type Output = Dipole;

    fn project_via_origin_onto(self, other: Sphere) -> Dipole {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for PointAtInfinity {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Circle> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Circle) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for Rotor {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: Horizon) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Line> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Line) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<LineAtInfinity> for Rotor {
    type Output = LineAtInfinity;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> LineAtInfinity {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Motor> for Rotor {
    type Output = Motor;

    fn project_via_origin_onto(self, other: Motor) -> Motor {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for Rotor {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Plane> for Rotor {
    type Output = Line;

    fn project_via_origin_onto(self, other: Plane) -> Line {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Sphere> for Rotor {
    type Output = Circle;

    fn project_via_origin_onto(self, other: Sphere) -> Circle {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Translator> for Rotor {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
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

impl ProjectViaOriginOnto<Flector> for RoundPoint {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Flector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Horizon> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Horizon) -> RoundPoint {
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
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: LineAtInfinity) -> RoundPoint {
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

impl ProjectViaOriginOnto<Point> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: Point) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<PointAtInfinity> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: PointAtInfinity) -> RoundPoint {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn project_via_origin_onto(self, other: RoundPoint) -> RoundPoint {
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
