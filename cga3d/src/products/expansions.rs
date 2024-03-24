//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

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

impl BulkExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Horizon) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn bulk_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Circle {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Horizon) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Line) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: LineAtInfinity) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Motor) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Plane) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Dipole {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn bulk_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Dipole {
    type Output = Sphere;

    fn bulk_expansion(self, other: Translator) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Flector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: Horizon) -> Motor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: Motor) -> Plane {
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
    type Output = Motor;

    fn bulk_expansion(self, other: Plane) -> Motor {
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

impl BulkExpansion<Sphere> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: Sphere) -> Motor {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Flector {
    type Output = Plane;

    fn bulk_expansion(self, other: Translator) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Horizon {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
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

impl BulkExpansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: Circle) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Infinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Dipole) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Infinity {
    type Output = Point;

    fn bulk_expansion(self, other: Horizon) -> Point {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Infinity> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: Line) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: LineAtInfinity) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: Motor) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Infinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Infinity {
    type Output = Point;

    fn bulk_expansion(self, other: Plane) -> Point {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for Infinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Point) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Infinity {
    type Output = Plane;

    fn bulk_expansion(self, other: PointAtInfinity) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPoint> for Infinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Infinity {
    type Output = Point;

    fn bulk_expansion(self, other: Sphere) -> Point {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Infinity {
    type Output = Line;

    fn bulk_expansion(self, other: Translator) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Line {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: Horizon) -> Plane {
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
    type Output = Plane;

    fn bulk_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for LineAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Horizon) -> Plane {
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
    type Output = Plane;

    fn bulk_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Motor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: Horizon) -> Plane {
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
    type Output = Plane;

    fn bulk_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Dipole) -> MultiVector {
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

impl BulkExpansion<Infinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Infinity) -> MultiVector {
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

impl BulkExpansion<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: RoundPoint) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Origin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Origin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Origin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Origin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Origin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Origin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Origin {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> LineAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Origin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Plane {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
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

impl BulkExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Point {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for Point {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Point {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Point {
    type Output = Line;

    fn bulk_expansion(self, other: Horizon) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Point {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Point {
    type Output = Plane;

    fn bulk_expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Point {
    type Output = Plane;

    fn bulk_expansion(self, other: Motor) -> Plane {
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
    type Output = Line;

    fn bulk_expansion(self, other: Plane) -> Line {
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

impl BulkExpansion<Sphere> for Point {
    type Output = Line;

    fn bulk_expansion(self, other: Sphere) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Point {
    type Output = Plane;

    fn bulk_expansion(self, other: Translator) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for PointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for PointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for PointAtInfinity {
    type Output = Line;

    fn bulk_expansion(self, other: Horizon) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for PointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for PointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: LineAtInfinity) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for PointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Motor) -> Plane {
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
    type Output = Line;

    fn bulk_expansion(self, other: Plane) -> Line {
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

impl BulkExpansion<Sphere> for PointAtInfinity {
    type Output = Line;

    fn bulk_expansion(self, other: Sphere) -> Line {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for PointAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Translator) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Rotor {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Rotor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Horizon) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Infinity> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Infinity) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Line> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Line) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<LineAtInfinity> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: LineAtInfinity) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Motor> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Motor) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Point> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: Point) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<PointAtInfinity> for RoundPoint {
    type Output = Sphere;

    fn bulk_expansion(self, other: PointAtInfinity) -> Sphere {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: RoundPoint) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn bulk_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for RoundPoint {
    type Output = Circle;

    fn bulk_expansion(self, other: Translator) -> Circle {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Circle> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Flector> for Translator {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Horizon> for Translator {
    type Output = Plane;

    fn bulk_expansion(self, other: Horizon) -> Plane {
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
    type Output = Plane;

    fn bulk_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Sphere> for Translator {
    type Output = Plane;

    fn bulk_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_bulk_dual())
    }
}

impl BulkExpansion<Translator> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.right_bulk_dual())
    }
}

impl WeightExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Plane) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Circle {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Circle {
    type Output = Circle;

    fn weight_expansion(self, other: Translator) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: Circle) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: Line) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Dipole {
    type Output = Sphere;

    fn weight_expansion(self, other: LineAtOrigin) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Plane) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for Dipole {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Dipole {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Dipole {
    type Output = Dipole;

    fn weight_expansion(self, other: Translator) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
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

impl WeightExpansion<Motor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Sphere) -> Motor {
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
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Horizon {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Horizon {
    type Output = Horizon;

    fn weight_expansion(self, other: Translator) -> Horizon {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Infinity {
    type Output = Line;

    fn weight_expansion(self, other: Circle) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Infinity {
    type Output = Plane;

    fn weight_expansion(self, other: Dipole) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Infinity {
    type Output = Line;

    fn weight_expansion(self, other: Line) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Infinity {
    type Output = Line;

    fn weight_expansion(self, other: LineAtOrigin) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for Infinity {
    type Output = Plane;

    fn weight_expansion(self, other: Origin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Infinity {
    type Output = Point;

    fn weight_expansion(self, other: Plane) -> Point {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Infinity {
    type Output = Point;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Point {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for Infinity {
    type Output = Plane;

    fn weight_expansion(self, other: Point) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Infinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Infinity {
    type Output = Point;

    fn weight_expansion(self, other: Sphere) -> Point {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Infinity {
    type Output = Infinity;

    fn weight_expansion(self, other: Translator) -> Infinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
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

impl WeightExpansion<Motor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Line {
    type Output = Line;

    fn weight_expansion(self, other: Translator) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Translator) -> LineAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
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

impl WeightExpansion<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Translator) -> LineAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
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

impl WeightExpansion<Motor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Motor {
    type Output = Motor;

    fn weight_expansion(self, other: Translator) -> Motor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Circle) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Dipole) -> MultiVector {
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

impl WeightExpansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Sphere) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Translator) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Origin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Circle) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
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

impl WeightExpansion<Motor> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for Origin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Sphere) -> LineAtOrigin {
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
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
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
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Point {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for Point {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
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

impl WeightExpansion<Motor> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Point {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Point {
    type Output = Point;

    fn weight_expansion(self, other: Translator) -> Point {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for PointAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Circle) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for PointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Dipole) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for PointAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for PointAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for PointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for PointAtInfinity {
    type Output = Line;

    fn weight_expansion(self, other: Plane) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for PointAtInfinity {
    type Output = Line;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for PointAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for PointAtInfinity {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn weight_expansion(self, other: Translator) -> PointAtInfinity {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
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

impl WeightExpansion<Motor> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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

impl WeightExpansion<Rotor> for Rotor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Rotor {
    type Output = Rotor;

    fn weight_expansion(self, other: Translator) -> Rotor {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: Circle) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Dipole> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: Dipole) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: Line) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for RoundPoint {
    type Output = Circle;

    fn weight_expansion(self, other: LineAtOrigin) -> Circle {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Origin> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: Origin) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: Plane) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Point> for RoundPoint {
    type Output = Sphere;

    fn weight_expansion(self, other: Point) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for RoundPoint {
    type Output = RoundPoint;

    fn weight_expansion(self, other: Translator) -> RoundPoint {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Plane> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Sphere {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Sphere {
    type Output = Sphere;

    fn weight_expansion(self, other: Translator) -> Sphere {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Translator {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Flector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Line> for Translator {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Translator {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Motor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
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
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Translator {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Rotor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Sphere> for Translator {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Translator> for Translator {
    type Output = Translator;

    fn weight_expansion(self, other: Translator) -> Translator {
        self.wedge(other.right_weight_dual())
    }
}
