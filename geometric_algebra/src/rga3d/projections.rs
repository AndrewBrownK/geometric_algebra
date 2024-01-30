
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::products::exterior::Wedge;
use crate::rga3d::products::exterior::AntiWedge;
use crate::rga3d::involutions::*;


/// Bulk Contraction
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait BulkContraction<T> {
    type Output;
    fn bulk_contraction(self, other: T) -> Self::Output;
}


/// Weight Contraction
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait WeightContraction<T> {
    type Output;
    fn weight_contraction(self, other: T) -> Self::Output;
}


/// Bulk Expansion
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}


/// Weight Expansion
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
}


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

impl AntiProjectViaHorizonOnto<Translator> for Flector {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Plane {
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

impl AntiProjectViaHorizonOnto<Point> for Horizon {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Point) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Horizon {
    type Output = Horizon;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Horizon {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Line {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Motor {
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

impl AntiProjectViaHorizonOnto<Translator> for Line {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtInfinity {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Motor {
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
    type Output = LineAtInfinity;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> LineAtInfinity {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for LineAtInfinity {
    type Output = Translator;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Translator {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for LineAtOrigin {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Motor {
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
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Point) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> LineAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Motor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Motor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> AntiScalar {
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
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Point) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Motor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Motor {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
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

impl AntiProjectViaHorizonOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Origin {
    type Output = Flector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Flector {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: MultiVector) -> MultiVector {
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

impl AntiProjectViaHorizonOnto<Translator> for Plane {
    type Output = Plane;

    fn anti_project_via_horizon_onto(self, other: Translator) -> Plane {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> MultiVector {
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

impl AntiProjectViaHorizonOnto<Point> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Point) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_project_via_horizon_onto(self, other: Translator) -> PlaneAtOrigin {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Point {
    type Output = Flector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Flector {
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

impl AntiProjectViaHorizonOnto<Flector> for PointAtInfinity {
    type Output = Flector;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Flector {
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

impl AntiProjectViaHorizonOnto<Flector> for Rotor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> AntiScalar {
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

impl AntiProjectViaHorizonOnto<Point> for Rotor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Point) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Rotor {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Rotor {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Translator) -> AntiScalar {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Flector> for Translator {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Flector) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Horizon> for Translator {
    type Output = AntiScalar;

    fn anti_project_via_horizon_onto(self, other: Horizon) -> AntiScalar {
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
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: Point) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<PointAtInfinity> for Translator {
    type Output = Motor;

    fn anti_project_via_horizon_onto(self, other: PointAtInfinity) -> Motor {
        other.wedge(self.bulk_contraction(other))
    }
}

impl AntiProjectViaHorizonOnto<Translator> for Translator {
    type Output = MultiVector;

    fn anti_project_via_horizon_onto(self, other: Translator) -> MultiVector {
        other.wedge(self.bulk_contraction(other))
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

impl BulkContraction<Line> for Flector {
    type Output = Point;

    fn bulk_contraction(self, other: Line) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Flector {
    type Output = Point;

    fn bulk_contraction(self, other: LineAtInfinity) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Flector {
    type Output = Point;

    fn bulk_contraction(self, other: Motor) -> Point {
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

impl BulkContraction<Point> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Flector {
    type Output = Point;

    fn bulk_contraction(self, other: Translator) -> Point {
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

impl BulkContraction<Line> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> PointAtInfinity {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Motor) -> PointAtInfinity {
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

impl BulkContraction<Point> for Horizon {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: Point) -> LineAtInfinity {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: PointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Translator) -> PointAtInfinity {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: Flector) -> Point {
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

impl BulkContraction<Point> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: Point) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: PointAtInfinity) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Flector) -> PointAtInfinity {
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

impl BulkContraction<Point> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Point) -> PointAtInfinity {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for LineAtOrigin {
    type Output = Point;

    fn bulk_contraction(self, other: Flector) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Point) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: PointAtInfinity) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: Flector) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
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

impl BulkContraction<Point> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: Point) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: PointAtInfinity) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
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

impl BulkContraction<Point> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
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
    type Output = Scalar;

    fn bulk_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
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

impl BulkContraction<Line> for Plane {
    type Output = Point;

    fn bulk_contraction(self, other: Line) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Plane {
    type Output = Point;

    fn bulk_contraction(self, other: LineAtInfinity) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Plane {
    type Output = Point;

    fn bulk_contraction(self, other: Motor) -> Point {
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

impl BulkContraction<Point> for Plane {
    type Output = Line;

    fn bulk_contraction(self, other: Point) -> Line {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Plane {
    type Output = Line;

    fn bulk_contraction(self, other: PointAtInfinity) -> Line {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Plane {
    type Output = Point;

    fn bulk_contraction(self, other: Translator) -> Point {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
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

impl BulkContraction<Point> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Point) -> LineAtOrigin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: PointAtInfinity) -> LineAtOrigin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Translator) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: PointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: PointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: Flector) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: LineAtInfinity) -> LineAtOrigin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Motor) -> LineAtOrigin {
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

impl BulkContraction<Point> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: Point) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: PointAtInfinity) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Translator) -> LineAtOrigin {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Translator {
    type Output = Flector;

    fn bulk_contraction(self, other: Flector) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
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

impl BulkContraction<Point> for Translator {
    type Output = Flector;

    fn bulk_contraction(self, other: Point) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Translator {
    type Output = Flector;

    fn bulk_contraction(self, other: PointAtInfinity) -> Flector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Horizon) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Plane) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for LineAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Horizon) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Line) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Point) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: PointAtInfinity) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Origin {
    type Output = Rotor;

    fn bulk_expansion(self, other: Flector) -> Rotor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Point {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Point {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Point {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Point {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for Point {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Point {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for PointAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for PointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for PointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Translator {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl ProjectOrthogonallyOnto<Flector> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Flector) -> Flector {
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

impl ProjectOrthogonallyOnto<Translator> for Flector {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Flector) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for Horizon {
    type Output = Horizon;

    fn project_orthogonally_onto(self, other: Translator) -> Horizon {
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

impl ProjectOrthogonallyOnto<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn project_orthogonally_onto(self, other: Translator) -> LineAtInfinity {
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

impl ProjectOrthogonallyOnto<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
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

impl ProjectOrthogonallyOnto<Translator> for Motor {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
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

impl ProjectOrthogonallyOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Origin {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Flector) -> Flector {
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

impl ProjectOrthogonallyOnto<Translator> for Origin {
    type Output = Origin;

    fn project_orthogonally_onto(self, other: Translator) -> Origin {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Plane {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Flector) -> Flector {
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

impl ProjectOrthogonallyOnto<Translator> for Plane {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for PlaneAtOrigin {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Flector) -> Flector {
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

impl ProjectOrthogonallyOnto<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for Point {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Flector) -> Flector {
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

impl ProjectOrthogonallyOnto<Translator> for Point {
    type Output = Point;

    fn project_orthogonally_onto(self, other: Translator) -> Point {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Flector> for PointAtInfinity {
    type Output = Flector;

    fn project_orthogonally_onto(self, other: Flector) -> Flector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Line> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_orthogonally_onto(self, other: Line) -> PointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<LineAtOrigin> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_orthogonally_onto(self, other: LineAtOrigin) -> PointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Plane> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_orthogonally_onto(self, other: Plane) -> PointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<PlaneAtOrigin> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_orthogonally_onto(self, other: PlaneAtOrigin) -> PointAtInfinity {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectOrthogonallyOnto<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn project_orthogonally_onto(self, other: Translator) -> PointAtInfinity {
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

impl ProjectOrthogonallyOnto<Translator> for Rotor {
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

impl ProjectOrthogonallyOnto<Translator> for Translator {
    type Output = Translator;

    fn project_orthogonally_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.weight_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Flector) -> Flector {
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

impl ProjectViaOriginOnto<Translator> for Flector {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Horizon {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Flector) -> Flector {
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

impl ProjectViaOriginOnto<Translator> for Line {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
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

impl ProjectViaOriginOnto<Translator> for Motor {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
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

impl ProjectViaOriginOnto<Translator> for MultiVector {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: Translator) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Origin {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Flector) -> Flector {
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
    type Output = Flector;

    fn project_via_origin_onto(self, other: Flector) -> Flector {
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

impl ProjectViaOriginOnto<Flector> for PlaneAtOrigin {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Flector) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn project_via_origin_onto(self, other: MultiVector) -> MultiVector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for Point {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Flector) -> Flector {
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

impl ProjectViaOriginOnto<Translator> for Point {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl ProjectViaOriginOnto<Flector> for PointAtInfinity {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Flector) -> Flector {
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

impl ProjectViaOriginOnto<Translator> for PointAtInfinity {
    type Output = Flector;

    fn project_via_origin_onto(self, other: Translator) -> Flector {
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

impl ProjectViaOriginOnto<Translator> for Translator {
    type Output = Translator;

    fn project_via_origin_onto(self, other: Translator) -> Translator {
        other.anti_wedge(self.bulk_expansion(other))
    }
}

impl WeightContraction<Flector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Flector {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Flector {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
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

impl WeightContraction<Point> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Line {
    type Output = Point;

    fn weight_contraction(self, other: Flector) -> Point {
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

impl WeightContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Line {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Origin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Line {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Point) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Flector) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for LineAtOrigin {
    type Output = Point;

    fn weight_contraction(self, other: Flector) -> Point {
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

impl WeightContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Origin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Point) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Motor {
    type Output = Flector;

    fn weight_contraction(self, other: Flector) -> Flector {
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

impl WeightContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Motor {
    type Output = Flector;

    fn weight_contraction(self, other: Origin) -> Flector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Motor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Motor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Motor {
    type Output = Flector;

    fn weight_contraction(self, other: Point) -> Flector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Motor {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
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

impl WeightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
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

impl WeightContraction<Point> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Point) -> MultiVector {
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
    type Output = Scalar;

    fn weight_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Point) -> Scalar {
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
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Plane {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Plane {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Origin) -> LineAtInfinity {
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

impl WeightContraction<Point> for Plane {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Point) -> LineAtInfinity {
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
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Origin) -> LineAtInfinity {
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

impl WeightContraction<Point> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Point) -> LineAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for PointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Rotor {
    type Output = Flector;

    fn weight_contraction(self, other: Flector) -> Flector {
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

impl WeightContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Rotor {
    type Output = Flector;

    fn weight_contraction(self, other: Origin) -> Flector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Rotor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Rotor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Rotor {
    type Output = Flector;

    fn weight_contraction(self, other: Point) -> Flector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Rotor {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Translator {
    type Output = Flector;

    fn weight_contraction(self, other: Flector) -> Flector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Translator {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Line) -> LineAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Translator {
    type Output = Horizon;

    fn weight_contraction(self, other: Origin) -> Horizon {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Translator {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Translator {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Translator {
    type Output = Horizon;

    fn weight_contraction(self, other: Point) -> Horizon {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Translator {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Flector) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Plane) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Flector {
    type Output = Flector;

    fn weight_expansion(self, other: Translator) -> Flector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Horizon {
    type Output = Horizon;

    fn weight_expansion(self, other: Translator) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Line {
    type Output = Line;

    fn weight_expansion(self, other: Translator) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for LineAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Translator) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Translator) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Motor {
    type Output = Motor;

    fn weight_expansion(self, other: Translator) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Line) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Origin) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Point) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Origin {
    type Output = Rotor;

    fn weight_expansion(self, other: Flector) -> Rotor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Origin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Origin {
    type Output = Origin;

    fn weight_expansion(self, other: Translator) -> Origin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Plane {
    type Output = Plane;

    fn weight_expansion(self, other: Translator) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Point {
    type Output = Motor;

    fn weight_expansion(self, other: Flector) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Point {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Point {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for Point {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Point {
    type Output = Line;

    fn weight_expansion(self, other: Plane) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Point {
    type Output = Line;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for Point {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Point {
    type Output = Point;

    fn weight_expansion(self, other: Translator) -> Point {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for PointAtInfinity {
    type Output = Motor;

    fn weight_expansion(self, other: Flector) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for PointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Line) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for PointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn weight_expansion(self, other: Translator) -> PointAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Rotor {
    type Output = Rotor;

    fn weight_expansion(self, other: Translator) -> Rotor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Translator {
    type Output = Plane;

    fn weight_expansion(self, other: Flector) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Translator {
    type Output = Translator;

    fn weight_expansion(self, other: Translator) -> Translator {
        self.wedge(other.right_weight_dual())
    }
}

