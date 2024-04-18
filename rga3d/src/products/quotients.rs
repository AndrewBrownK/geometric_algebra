//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::characteristics::AntiInverse;
use crate::characteristics::Inverse;
use crate::products::geometric::GeometricAntiProduct;
use crate::products::geometric::GeometricProduct;
use crate::*;

/// The Geometric Quotient between `a` and `b` is the geometric product between `a` and `b^-1` (the inverse of `b`).
/// See also "Inverse".
pub trait GeometricQuotient<T> {
    type Output;
    fn geometric_quotient(self, other: T) -> Self::Output;
}

/// The Geometric AntiQuotient between `a` and `b` is the geometric anti-product between `a` and the anti-inverse of `b`.
/// See also "AntiInverse".
pub trait GeometricAntiQuotient<T> {
    type Output;
    fn geometric_anti_quotient(self, other: T) -> Self::Output;
}

impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: AntiScalar) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for AntiScalar {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for AntiScalar {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for AntiScalar {
    type Output = Line;

    fn geometric_anti_quotient(self, other: Line) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for AntiScalar {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Motor) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for AntiScalar {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for AntiScalar {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: Origin) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for AntiScalar {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: Plane) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for AntiScalar {
    type Output = Point;

    fn geometric_anti_quotient(self, other: Point) -> Point {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for AntiScalar {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for AntiScalar {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for AntiScalar {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Translator) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: AntiScalar) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for DualNum {
    type Output = DualNum;

    fn geometric_anti_quotient(self, other: DualNum) -> DualNum {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for DualNum {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for DualNum {
    type Output = Line;

    fn geometric_anti_quotient(self, other: Line) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for DualNum {
    type Output = Line;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for DualNum {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Origin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for DualNum {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Plane) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for DualNum {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for DualNum {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Point) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for DualNum {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for DualNum {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: DualNum) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Origin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Point) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Flector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Translator) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: DualNum) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Origin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Point) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Translator) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: DualNum) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Horizon {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Horizon {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Horizon {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Horizon {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: Origin) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Horizon {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: Point) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Horizon {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Horizon {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: Translator) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Line {
    type Output = Line;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Line {
    type Output = Line;

    fn geometric_anti_quotient(self, other: DualNum) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Line {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Origin) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Line {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Point) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Line {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Line {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: DualNum) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: Origin) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: Point) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Translator) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for LineAtOrigin {
    type Output = Line;

    fn geometric_anti_quotient(self, other: DualNum) -> Line {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: Origin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for LineAtOrigin {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Point) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Motor {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Origin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Point) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Motor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Origin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Point) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Origin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Point) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: Origin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Point) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Origin {
    type Output = Origin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Origin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Origin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: DualNum) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Origin {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Line) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Origin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Origin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Origin {
    type Output = AntiScalar;

    fn geometric_anti_quotient(self, other: Origin) -> AntiScalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Origin {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Point) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Origin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Origin {
    type Output = Point;

    fn geometric_anti_quotient(self, other: Translator) -> Point {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Plane {
    type Output = Plane;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Plane {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Plane {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: DualNum) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Origin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Plane {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Plane) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Plane {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Point) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Plane {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Plane {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_quotient(self, other: AntiScalar) -> PlaneAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for PlaneAtOrigin {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: DualNum) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_quotient(self, other: Origin) -> LineAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for PlaneAtOrigin {
    type Output = Motor;

    fn geometric_anti_quotient(self, other: Plane) -> Motor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Point) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for PlaneAtOrigin {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Point {
    type Output = Point;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Point {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Point {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: DualNum) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Point {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Point {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Line) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Point {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Point {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Point {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Point {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Origin) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Point {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Point {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Point {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Point) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Point {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Point {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Point {
    type Output = Point;

    fn geometric_anti_quotient(self, other: Translator) -> Point {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: AntiScalar) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: DualNum) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for PointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for PointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for PointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Origin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Point) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for PointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: Translator) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Origin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Plane) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Point) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_anti_quotient(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Rotor {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Transflector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Translator) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Scalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Scalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: DualNum) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Scalar {
    type Output = FlectorAtInfinity;

    fn geometric_anti_quotient(self, other: Flector) -> FlectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Scalar {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: Line) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Scalar {
    type Output = LineAtInfinity;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> LineAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Scalar {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Scalar {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Scalar {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Scalar {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: Origin) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Scalar {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: Plane) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Scalar {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Scalar {
    type Output = Horizon;

    fn geometric_anti_quotient(self, other: Point) -> Horizon {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Scalar {
    type Output = MultiVectorAtInfinity;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Scalar {
    type Output = PointAtInfinity;

    fn geometric_anti_quotient(self, other: Transflector) -> PointAtInfinity {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Scalar {
    type Output = Scalar;

    fn geometric_anti_quotient(self, other: Translator) -> Scalar {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: DualNum) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Flector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Line) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Motor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Origin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Plane) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Point) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Transflector {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Rotor) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Transflector {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Transflector {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Translator) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<AntiScalar> for Translator {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: AntiScalar) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<DualNum> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Flector> for Translator {
    type Output = Flector;

    fn geometric_anti_quotient(self, other: Flector) -> Flector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Line> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Line) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: LineAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Motor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<MultiVectorAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Origin> for Translator {
    type Output = Point;

    fn geometric_anti_quotient(self, other: Origin) -> Point {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Plane> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Plane) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<PlaneAtOrigin> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: PlaneAtOrigin) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Point> for Translator {
    type Output = Point;

    fn geometric_anti_quotient(self, other: Point) -> Point {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Rotor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Transflector> for Translator {
    type Output = Transflector;

    fn geometric_anti_quotient(self, other: Transflector) -> Transflector {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricAntiQuotient<Translator> for Translator {
    type Output = Translator;

    fn geometric_anti_quotient(self, other: Translator) -> Translator {
        self.geometric_anti_product(other.anti_inverse())
    }
}

impl GeometricQuotient<DualNum> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: DualNum) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for AntiScalar {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for AntiScalar {
    type Output = Flector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for AntiScalar {
    type Output = Origin;

    fn geometric_quotient(self, other: Horizon) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Line) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: LineAtInfinity) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Motor) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for AntiScalar {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for AntiScalar {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for AntiScalar {
    type Output = Origin;

    fn geometric_quotient(self, other: Plane) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Point) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: PointAtInfinity) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Scalar) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for AntiScalar {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Translator) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for DualNum {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for DualNum {
    type Output = Flector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for DualNum {
    type Output = Flector;

    fn geometric_quotient(self, other: Horizon) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for DualNum {
    type Output = Line;

    fn geometric_quotient(self, other: Line) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for DualNum {
    type Output = Line;

    fn geometric_quotient(self, other: LineAtInfinity) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for DualNum {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for DualNum {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for DualNum {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for DualNum {
    type Output = Flector;

    fn geometric_quotient(self, other: Point) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for DualNum {
    type Output = Transflector;

    fn geometric_quotient(self, other: PointAtInfinity) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for DualNum {
    type Output = DualNum;

    fn geometric_quotient(self, other: Scalar) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for DualNum {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for DualNum {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: DualNum) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Horizon) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Scalar) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Flector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Flector {
    type Output = Flector;

    fn geometric_quotient(self, other: Translator) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for FlectorAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: DualNum) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: Horizon) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for FlectorAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: LineAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for FlectorAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for FlectorAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Translator) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Horizon {
    type Output = Flector;

    fn geometric_quotient(self, other: DualNum) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Horizon {
    type Output = Scalar;

    fn geometric_quotient(self, other: Horizon) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Horizon {
    type Output = Transflector;

    fn geometric_quotient(self, other: Line) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Horizon {
    type Output = PointAtInfinity;

    fn geometric_quotient(self, other: LineAtInfinity) -> PointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Horizon {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Horizon {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Horizon {
    type Output = Translator;

    fn geometric_quotient(self, other: Point) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: PointAtInfinity) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Horizon {
    type Output = Horizon;

    fn geometric_quotient(self, other: Scalar) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Horizon {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Horizon {
    type Output = Point;

    fn geometric_quotient(self, other: Translator) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: DualNum) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Line {
    type Output = Transflector;

    fn geometric_quotient(self, other: Horizon) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: Point) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: PointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: Scalar) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Line {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for LineAtInfinity {
    type Output = Line;

    fn geometric_quotient(self, other: DualNum) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for LineAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_quotient(self, other: Horizon) -> PointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for LineAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for LineAtInfinity {
    type Output = Transflector;

    fn geometric_quotient(self, other: Point) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for LineAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: PointAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for LineAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: DualNum) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Horizon) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Line) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: LineAtInfinity) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Motor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Plane) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Point) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: PointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Translator) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: DualNum) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Horizon) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Point) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: PointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Scalar) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Motor {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Horizon) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: DualNum) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: Horizon) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: DualNum) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Flector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Horizon) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Line) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Motor) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Plane) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Point) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Transflector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: Translator) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Origin {
    type Output = Origin;

    fn geometric_quotient(self, other: DualNum) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Origin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Flector) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Origin {
    type Output = Rotor;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Origin {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Horizon) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Line) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Motor) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Origin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Origin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Origin {
    type Output = AntiScalar;

    fn geometric_quotient(self, other: Plane) -> AntiScalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Origin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Point) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Origin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: PointAtInfinity) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Origin {
    type Output = Origin;

    fn geometric_quotient(self, other: Scalar) -> Origin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Origin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Transflector) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Translator) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: DualNum) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Horizon) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: PointAtInfinity) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: Scalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Plane {
    type Output = Flector;

    fn geometric_quotient(self, other: Translator) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: DualNum) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Flector) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Horizon) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_quotient(self, other: Plane) -> LineAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Point) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: PointAtInfinity) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_quotient(self, other: Scalar) -> PlaneAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_quotient(self, other: Transflector) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_quotient(self, other: Translator) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Point {
    type Output = Flector;

    fn geometric_quotient(self, other: DualNum) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Point {
    type Output = Translator;

    fn geometric_quotient(self, other: Horizon) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Point {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Point {
    type Output = Transflector;

    fn geometric_quotient(self, other: LineAtInfinity) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Point {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Point {
    type Output = Point;

    fn geometric_quotient(self, other: Scalar) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Point {
    type Output = Transflector;

    fn geometric_quotient(self, other: Translator) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for PointAtInfinity {
    type Output = Transflector;

    fn geometric_quotient(self, other: DualNum) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: Horizon) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for PointAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for PointAtInfinity {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: LineAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for PointAtInfinity {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for PointAtInfinity {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_quotient(self, other: Scalar) -> PointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for PointAtInfinity {
    type Output = Transflector;

    fn geometric_quotient(self, other: Translator) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: DualNum) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: Horizon) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Line) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: LineAtInfinity) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Motor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: Point) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: PointAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Scalar) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Rotor {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Translator) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Scalar {
    type Output = DualNum;

    fn geometric_quotient(self, other: DualNum) -> DualNum {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Scalar {
    type Output = FlectorAtInfinity;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Scalar {
    type Output = Horizon;

    fn geometric_quotient(self, other: Horizon) -> Horizon {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Scalar {
    type Output = Line;

    fn geometric_quotient(self, other: Line) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Scalar {
    type Output = LineAtInfinity;

    fn geometric_quotient(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Scalar {
    type Output = MultiVectorAtInfinity;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;

    fn geometric_quotient(self, other: Plane) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Scalar {
    type Output = Point;

    fn geometric_quotient(self, other: Point) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Scalar {
    type Output = PointAtInfinity;

    fn geometric_quotient(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: Scalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Scalar {
    type Output = Transflector;

    fn geometric_quotient(self, other: Transflector) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Scalar {
    type Output = Translator;

    fn geometric_quotient(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: DualNum) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Flector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Horizon) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: Line) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: LineAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: Motor) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PointAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Transflector {
    type Output = Transflector;

    fn geometric_quotient(self, other: Scalar) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Transflector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Transflector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Transflector {
    type Output = Flector;

    fn geometric_quotient(self, other: Translator) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<DualNum> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: DualNum) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Flector> for Translator {
    type Output = Flector;

    fn geometric_quotient(self, other: Flector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<FlectorAtInfinity> for Translator {
    type Output = Flector;

    fn geometric_quotient(self, other: FlectorAtInfinity) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Horizon> for Translator {
    type Output = Point;

    fn geometric_quotient(self, other: Horizon) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Line> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: LineAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<MultiVectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Plane> for Translator {
    type Output = Flector;

    fn geometric_quotient(self, other: Plane) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for Translator {
    type Output = Transflector;

    fn geometric_quotient(self, other: Point) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<PointAtInfinity> for Translator {
    type Output = Transflector;

    fn geometric_quotient(self, other: PointAtInfinity) -> Transflector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Scalar) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Transflector> for Translator {
    type Output = Flector;

    fn geometric_quotient(self, other: Transflector) -> Flector {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}
