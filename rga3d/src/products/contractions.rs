//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

#![allow(clippy::assign_op_pattern)]
use crate::rga3d::aspect_duals::*;
use crate::rga3d::products::exterior::AntiWedge;
use crate::rga3d::*;

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
