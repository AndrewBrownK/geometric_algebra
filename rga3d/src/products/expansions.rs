//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

#![allow(clippy::assign_op_pattern)]
use crate::aspect_duals::*;
use crate::products::exterior::Wedge;
use crate::*;

/// Bulk Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}

/// Weight Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
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
