//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::aspect_duals::*;
use crate::products::exterior::AntiWedge;
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

impl BulkContraction<Circle> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: Infinity) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Motor) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Circle {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: PointAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Circle {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Circle {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Infinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Dipole {
    type Output = Scalar;

    fn bulk_contraction(self, other: PointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
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

impl BulkContraction<Infinity> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Motor) -> RoundPoint {
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

impl BulkContraction<RoundPoint> for Flector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Flector {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Flector {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Translator) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Horizon {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Horizon {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
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

impl BulkContraction<Infinity> for Horizon {
    type Output = Circle;

    fn bulk_contraction(self, other: Infinity) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Horizon {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Horizon {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Horizon {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Motor) -> RoundPoint {
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
    type Output = Dipole;

    fn bulk_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Horizon {
    type Output = Dipole;

    fn bulk_contraction(self, other: PointAtInfinity) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Horizon {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Horizon {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Horizon {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Translator) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Infinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Infinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Line {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Line {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Line {
    type Output = Dipole;

    fn bulk_contraction(self, other: Infinity) -> Dipole {
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
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Line {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: PointAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Line {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Line {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for LineAtInfinity {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for LineAtInfinity {
    type Output = Dipole;

    fn bulk_contraction(self, other: Infinity) -> Dipole {
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
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for LineAtInfinity {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: PointAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for LineAtInfinity {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for LineAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for LineAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for LineAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: Infinity) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: LineAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Motor) -> Scalar {
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
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for LineAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: PointAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for LineAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: RoundPoint) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for LineAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Motor {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Horizon) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
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
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Motor {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Motor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
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

impl BulkContraction<Infinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
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

impl BulkContraction<RoundPoint> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Origin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Infinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Origin {
    type Output = Scalar;

    fn bulk_contraction(self, other: PointAtInfinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Origin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Plane {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
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

impl BulkContraction<Infinity> for Plane {
    type Output = Circle;

    fn bulk_contraction(self, other: Infinity) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Motor) -> RoundPoint {
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
    type Output = Dipole;

    fn bulk_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Plane {
    type Output = Dipole;

    fn bulk_contraction(self, other: PointAtInfinity) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Plane {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Plane {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Plane {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Translator) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for PlaneAtOrigin {
    type Output = Circle;

    fn bulk_contraction(self, other: Infinity) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Motor) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for PlaneAtOrigin {
    type Output = Dipole;

    fn bulk_contraction(self, other: PointAtInfinity) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for PlaneAtOrigin {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Translator) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Point {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Point {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Point {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Infinity) -> RoundPoint {
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

impl BulkContraction<RoundPoint> for Point {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for PointAtInfinity {
    type Output = Scalar;

    fn bulk_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for PointAtInfinity {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Infinity) -> RoundPoint {
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

impl BulkContraction<RoundPoint> for PointAtInfinity {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Rotor {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Horizon) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: LineAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Motor) -> MultiVector {
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
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Rotor {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Rotor {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: Infinity) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for RoundPoint {
    type Output = Scalar;

    fn bulk_contraction(self, other: RoundPoint) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Horizon) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: Infinity) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Line> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<LineAtInfinity> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: LineAtInfinity) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Motor> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Motor) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn bulk_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Plane> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Sphere {
    type Output = Dipole;

    fn bulk_contraction(self, other: PointAtInfinity) -> Dipole {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Sphere {
    type Output = Circle;

    fn bulk_contraction(self, other: RoundPoint) -> Circle {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Sphere {
    type Output = Scalar;

    fn bulk_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Sphere {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Translator) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Circle> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Dipole> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Flector> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Horizon> for Translator {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Horizon) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Infinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Infinity) -> MultiVector {
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
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Point> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<PointAtInfinity> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: PointAtInfinity) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<RoundPoint> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: RoundPoint) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Sphere> for Translator {
    type Output = RoundPoint;

    fn bulk_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl BulkContraction<Translator> for Translator {
    type Output = MultiVector;

    fn bulk_contraction(self, other: Translator) -> MultiVector {
        self.anti_wedge(other.right_bulk_dual())
    }
}

impl WeightContraction<Circle> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Circle {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Origin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Circle {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Circle {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Dipole {
    type Output = Scalar;

    fn weight_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Dipole {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Flector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Flector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Flector {
    type Output = RoundPoint;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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

impl WeightContraction<Rotor> for Flector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Flector {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Horizon {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Horizon {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Horizon {
    type Output = RoundPoint;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: Origin) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Horizon {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Horizon {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Horizon {
    type Output = Dipole;

    fn weight_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Horizon {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Horizon {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Infinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Line {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Line {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
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

impl WeightContraction<Motor> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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
    type Output = RoundPoint;

    fn weight_contraction(self, other: Origin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Line {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Line {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for LineAtInfinity {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Line) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for LineAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: LineAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for LineAtInfinity {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Origin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for LineAtInfinity {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for LineAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Circle) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for LineAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Dipole) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
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

impl WeightContraction<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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
    type Output = RoundPoint;

    fn weight_contraction(self, other: Origin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for LineAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Point) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for LineAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
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

impl WeightContraction<Motor> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Motor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Motor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Motor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Motor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Motor {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
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

impl WeightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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

impl WeightContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for MultiVector {
    type Output = MultiVector;

    fn weight_contraction(self, other: Sphere) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for MultiVector {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Origin {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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

impl WeightContraction<Rotor> for Origin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Plane {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Plane {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
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
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Plane {
    type Output = RoundPoint;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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
    type Output = Dipole;

    fn weight_contraction(self, other: Origin) -> Dipole {
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
    type Output = Dipole;

    fn weight_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Plane {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Plane {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for PlaneAtOrigin {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
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
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = RoundPoint;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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
    type Output = Dipole;

    fn weight_contraction(self, other: Origin) -> Dipole {
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
    type Output = Dipole;

    fn weight_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for PlaneAtOrigin {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for PlaneAtOrigin {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Point {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Point {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Point {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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

impl WeightContraction<Rotor> for Point {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for PointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Dipole) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for PointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Origin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for PointAtInfinity {
    type Output = Scalar;

    fn weight_contraction(self, other: Point) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for PointAtInfinity {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
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

impl WeightContraction<Motor> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Rotor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Rotor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Rotor {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Rotor {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Rotor {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for RoundPoint {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Sphere {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Circle) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: Dipole) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Sphere {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Line) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Sphere {
    type Output = RoundPoint;

    fn weight_contraction(self, other: LineAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<MultiVector> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Origin> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: Origin) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: Plane) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: PlaneAtOrigin) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Sphere {
    type Output = Dipole;

    fn weight_contraction(self, other: Point) -> Dipole {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Sphere {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Sphere {
    type Output = Scalar;

    fn weight_contraction(self, other: Sphere) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Circle> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Circle) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Dipole> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Dipole) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Flector> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Flector) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Line> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Line) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Motor> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Motor) -> MultiVector {
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
    type Output = MultiVector;

    fn weight_contraction(self, other: Origin) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Plane> for Translator {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Plane) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<PlaneAtOrigin> for Translator {
    type Output = RoundPoint;

    fn weight_contraction(self, other: PlaneAtOrigin) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Point> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Point) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Rotor> for Translator {
    type Output = MultiVector;

    fn weight_contraction(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Sphere> for Translator {
    type Output = RoundPoint;

    fn weight_contraction(self, other: Sphere) -> RoundPoint {
        self.anti_wedge(other.right_weight_dual())
    }
}

impl WeightContraction<Translator> for Translator {
    type Output = Scalar;

    fn weight_contraction(self, other: Translator) -> Scalar {
        self.anti_wedge(other.right_weight_dual())
    }
}