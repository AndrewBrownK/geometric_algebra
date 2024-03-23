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

impl BulkExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
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

impl BulkExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
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

impl BulkExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Circle) -> AntiScalar {
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

impl WeightExpansion<Circle> for Circle {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
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

impl WeightExpansion<Sphere> for Circle {
    type Output = Sphere;

    fn weight_expansion(self, other: Sphere) -> Sphere {
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

impl WeightExpansion<Sphere> for Dipole {
    type Output = Circle;

    fn weight_expansion(self, other: Sphere) -> Circle {
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

impl WeightExpansion<Sphere> for Horizon {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
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

impl WeightExpansion<Sphere> for Infinity {
    type Output = Point;

    fn weight_expansion(self, other: Sphere) -> Point {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
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

impl WeightExpansion<Sphere> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for LineAtInfinity {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
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

impl WeightExpansion<Sphere> for LineAtInfinity {
    type Output = Plane;

    fn weight_expansion(self, other: Sphere) -> Plane {
        self.wedge(other.right_weight_dual())
    }
}

impl WeightExpansion<Circle> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Circle) -> AntiScalar {
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

impl WeightExpansion<Sphere> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Sphere) -> PlaneAtOrigin {
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

impl WeightExpansion<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Sphere) -> MultiVector {
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

impl WeightExpansion<Sphere> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Sphere) -> LineAtOrigin {
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

impl WeightExpansion<Sphere> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
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

impl WeightExpansion<Sphere> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
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

impl WeightExpansion<Sphere> for Point {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
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

impl WeightExpansion<Sphere> for PointAtInfinity {
    type Output = Line;

    fn weight_expansion(self, other: Sphere) -> Line {
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

impl WeightExpansion<Sphere> for RoundPoint {
    type Output = Dipole;

    fn weight_expansion(self, other: Sphere) -> Dipole {
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

impl WeightExpansion<Sphere> for Sphere {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Sphere) -> AntiScalar {
        self.wedge(other.right_weight_dual())
    }
}
