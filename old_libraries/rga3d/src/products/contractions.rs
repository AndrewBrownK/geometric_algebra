// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::exterior::*;
use crate::*;

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
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Flector {
    type Output = Point;

    fn bulk_contraction(self, other: Line) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Flector {
    type Output = Point;

    fn bulk_contraction(self, other: LineAtInfinity) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Flector {
    type Output = Point;

    fn bulk_contraction(self, other: Motor) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
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
    type Output = Point;

    fn bulk_contraction(self, other: Translator) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for FlectorAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Motor) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for FlectorAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Point) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Transflector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Translator) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Motor) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Horizon {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: Point) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn bulk_contraction(self, other: PointAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Transflector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Horizon {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Translator) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: Flector) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: Point) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: PointAtInfinity) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Line {
    type Output = Point;

    fn bulk_contraction(self, other: Transflector) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Flector) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Point) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn bulk_contraction(self, other: Transflector) -> PointAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for LineAtOrigin {
    type Output = Point;

    fn bulk_contraction(self, other: Flector) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for LineAtOrigin {
    type Output = Point;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Point) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for LineAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: PointAtInfinity) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for LineAtOrigin {
    type Output = Point;

    fn bulk_contraction(self, other: Transflector) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: Flector) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
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
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVector {
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

impl BulkContraction<MultiVectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Motor {
    type Output = Origin;

    fn bulk_contraction(self, other: Plane) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: Point) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: PointAtInfinity) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Motor {
    type Output = Flector;

    fn bulk_contraction(self, other: Transflector) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
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

impl BulkContraction<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Plane) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
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

impl BulkContraction<Flector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for MultiVectorAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Line) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for MultiVectorAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Point) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Transflector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: Translator) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for MultiVectorAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for MultiVectorAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Plane) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: Point) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Origin {
    type Output = Origin;

    fn bulk_contraction(self, other: MultiVector) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Origin {
    type Output = Origin;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Transflector) -> Scalar {
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

impl BulkContraction<Horizon> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Plane {
    type Output = Point;

    fn bulk_contraction(self, other: Line) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Plane {
    type Output = Point;

    fn bulk_contraction(self, other: LineAtInfinity) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Plane {
    type Output = Point;

    fn bulk_contraction(self, other: Motor) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Plane {
    type Output = Line;

    fn bulk_contraction(self, other: Point) -> Line {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Plane {
    type Output = Line;

    fn bulk_contraction(self, other: PointAtInfinity) -> Line {
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
    type Output = Point;

    fn bulk_contraction(self, other: Translator) -> Point {
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

impl BulkContraction<Line> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Line) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: LineAtInfinity) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for PlaneAtOrigin {
    type Output = Origin;

    fn bulk_contraction(self, other: Motor) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Point) -> LineAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: PointAtInfinity) -> LineAtOrigin {
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
    type Output = Origin;

    fn bulk_contraction(self, other: Translator) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Point {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: PointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: Transflector) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: PointAtInfinity) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Transflector) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: Flector) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: LineAtInfinity) -> LineAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Motor) -> LineAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Rotor {
    type Output = Origin;

    fn bulk_contraction(self, other: Plane) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: Point) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: PointAtInfinity) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Rotor {
    type Output = Flector;

    fn bulk_contraction(self, other: Transflector) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Rotor {
    type Output = LineAtOrigin;

    fn bulk_contraction(self, other: Translator) -> LineAtOrigin {
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

impl BulkContraction<Horizon> for Transflector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Line> for Transflector {
    type Output = Point;

    fn bulk_contraction(self, other: Line) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<LineAtInfinity> for Transflector {
    type Output = Point;

    fn bulk_contraction(self, other: LineAtInfinity) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Motor> for Transflector {
    type Output = Point;

    fn bulk_contraction(self, other: Motor) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVector> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Transflector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
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
    type Output = Point;

    fn bulk_contraction(self, other: Translator) -> Point {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Flector> for Translator {
    type Output = Flector;

    fn bulk_contraction(self, other: Flector) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<FlectorAtInfinity> for Translator {
    type Output = Flector;

    fn bulk_contraction(self, other: FlectorAtInfinity) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Horizon> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: Horizon) -> Origin {
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
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVector {
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

impl BulkContraction<MultiVectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Plane> for Translator {
    type Output = Origin;

    fn bulk_contraction(self, other: Plane) -> Origin {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Point> for Translator {
    type Output = Transflector;

    fn bulk_contraction(self, other: Point) -> Transflector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<PointAtInfinity> for Translator {
    type Output = Transflector;

    fn bulk_contraction(self, other: PointAtInfinity) -> Transflector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Transflector> for Translator {
    type Output = Flector;

    fn bulk_contraction(self, other: Transflector) -> Flector {
        self.anti_wedge(other.dual())
    }
}

impl BulkContraction<Translator> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.dual())
    }
}

impl WeightContraction<Flector> for Flector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Flector {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Flector {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Flector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Flector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Flector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Flector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Origin) -> MultiVectorAtInfinity {
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

impl WeightContraction<Point> for Flector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Point) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Flector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: Transflector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Line {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Flector) -> PointAtInfinity {
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
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Line {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Line {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Line {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Origin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for Line {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Point) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Line {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Flector) -> PointAtInfinity {
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
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Origin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Point) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Motor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Motor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Line) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Motor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Motor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Motor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Motor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Motor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Origin) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Motor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Motor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for Motor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Point) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Motor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Motor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Transflector) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Line) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Origin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Plane) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Point) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Transflector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Line) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Origin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Plane) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Point) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Transflector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: MultiVector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Rotor) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Plane {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Plane {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Plane {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Plane {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Plane {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Plane {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Plane {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Origin) -> LineAtInfinity {
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

impl WeightContraction<Point> for Plane {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Point) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Plane {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: Transflector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Origin) -> LineAtInfinity {
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

impl WeightContraction<Point> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Point) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Transflector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Flector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: MultiVector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Rotor) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Rotor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Line) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Rotor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Origin) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Rotor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Rotor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for Rotor {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Point) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Rotor {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Transflector) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Transflector {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Line) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Transflector {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Transflector {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Origin) -> LineAtInfinity {
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

impl WeightContraction<Point> for Transflector {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Point) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Transflector {
    type Output = Scalar;

    fn weight_contraction(self, other: Transflector) -> Scalar {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Flector> for Translator {
    type Output = FlectorAtInfinity;

    fn weight_contraction(self, other: Flector) -> FlectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Line> for Translator {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: Line) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Translator {
    type Output = LineAtInfinity;

    fn weight_contraction(self, other: LineAtOrigin) -> LineAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Motor> for Translator {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Motor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVector> for Translator {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<MultiVectorAtOrigin> for Translator {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Origin> for Translator {
    type Output = Horizon;

    fn weight_contraction(self, other: Origin) -> Horizon {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Plane> for Translator {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Plane) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Translator {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Point> for Translator {
    type Output = Horizon;

    fn weight_contraction(self, other: Point) -> Horizon {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Rotor> for Translator {
    type Output = MultiVectorAtInfinity;

    fn weight_contraction(self, other: Rotor) -> MultiVectorAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}

impl WeightContraction<Transflector> for Translator {
    type Output = PointAtInfinity;

    fn weight_contraction(self, other: Transflector) -> PointAtInfinity {
        self.anti_wedge(other.anti_dual())
    }
}
