//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::AntiReversal;
use crate::products::geometric::GeometricAntiProduct;
use crate::unitize::Unitize;
use crate::*;

/// self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
/// Also called sandwich product
/// See article "Projective Geometric Algebra Done Right"
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projective_Geometric_Algebra_Done_Right
pub trait Sandwich<T> {
    type Output;
    fn sandwich(self, other: T) -> Self::Output;
}

/// Point Inversion
/// An improper isometry that performs an inversion through a point.
/// Points may pass as specialized as Flectors, so in other words, this is a specialized Flector sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
/// Be careful not to confuse with `Inverse`, which raises a number to the power of `-1.0`.
pub trait PointInversion<T> {
    type Output;
    fn point_inversion(self, other: T) -> Self::Output;
}

/// Reflection
/// An improper isometry that performs reflection across a plane.
/// Planes may pass as specialized Flectors, so in other words, this is a specialized Flector sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
pub trait Reflect<T> {
    type Output;
    fn reflect(self, other: T) -> Self::Output;
}

/// Transflection
/// An improper isometry that performs a reflection and translation.
/// Transflectors are specialized Flectors, so in other words, this is a specialized Flector sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Transflection
pub trait Transflect<T> {
    type Output;
    fn transflect(self, other: T) -> Self::Output;
}

/// Translate
/// A proper isometry that performs translation.
/// Translators are specialized Motors, so in other words, this is a specialized Motor sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Translation
pub trait Translate<T> {
    type Output;
    fn translate(self, other: T) -> Self::Output;
}

/// Rotate
/// A proper isometry that performs rotation.
/// Rotors are specialized Motors, so in other words, this is a specialized Motor sandwich.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Rotation
pub trait Rotate<T> {
    type Output;
    fn rotate(self, other: T) -> Self::Output;
}

impl Sandwich<Flector> for AntiScalar {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for AntiScalar {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for AntiScalar {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for AntiScalar {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtInfinity> for AntiScalar {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Motor> for AntiScalar {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for AntiScalar {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for AntiScalar {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for AntiScalar {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for AntiScalar {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Point> for AntiScalar {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<PointAtInfinity> for AntiScalar {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for AntiScalar {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<TransFlector> for AntiScalar {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Translator> for AntiScalar {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for Flector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Flector {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Flector {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Flector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Flector {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Flector {
    type Output = Line;

    fn sandwich(self, other: LineAtOrigin) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Flector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Flector {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Flector {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for Flector {
    type Output = Point;

    fn sandwich(self, other: Origin) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Flector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Flector {
    type Output = Plane;

    fn sandwich(self, other: PlaneAtOrigin) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Flector {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for Flector {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Flector {
    type Output = Motor;

    fn sandwich(self, other: Rotor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Flector {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Flector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Line {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Line {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Line {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Line {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Line {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Line {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Line {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Line {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Line {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for Line {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Line {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Line {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Line {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for Line {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Line {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Line {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Line {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for LineAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for LineAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for LineAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for LineAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for LineAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for LineAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for LineAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for LineAtOrigin {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<TransFlector> for LineAtOrigin {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for LineAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Magnitude {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Magnitude {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Magnitude {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for Magnitude {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Magnitude {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Magnitude {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Magnitude {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for Magnitude {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Magnitude {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Magnitude {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for Magnitude {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Magnitude {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Translator> for Magnitude {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Motor {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Motor {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Motor {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Motor {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Motor {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Motor {
    type Output = Line;

    fn sandwich(self, other: LineAtOrigin) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Motor {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Motor {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for Motor {
    type Output = Point;

    fn sandwich(self, other: Origin) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Motor {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn sandwich(self, other: PlaneAtOrigin) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Motor {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for Motor {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Motor {
    type Output = Motor;

    fn sandwich(self, other: Rotor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Motor {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Motor {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for MultiVector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for MultiVector {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for MultiVector {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for MultiVector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for MultiVector {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for MultiVector {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for MultiVector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for MultiVector {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for MultiVector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for MultiVector {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for MultiVector {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for MultiVector {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for MultiVector {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for MultiVector {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for MultiVector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for MultiVectorAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for MultiVectorAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for MultiVectorAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for MultiVectorAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for MultiVectorAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for MultiVectorAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for MultiVectorAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for MultiVectorAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for MultiVectorAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for MultiVectorAtOrigin {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for MultiVectorAtOrigin {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for MultiVectorAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for MultiVectorAtOrigin {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for MultiVectorAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Origin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Origin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Origin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for Origin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Origin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Motor> for Origin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Origin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Origin {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Origin {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Origin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Origin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Point> for Origin {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<PointAtInfinity> for Origin {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for Origin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Origin {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Origin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for Plane {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Plane {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Plane {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Plane {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Plane {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Plane {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Plane {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Plane {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for Plane {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Plane {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Plane {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for Plane {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Plane {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Plane {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Plane {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for PlaneAtOrigin {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for PlaneAtOrigin {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for PlaneAtOrigin {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for PlaneAtOrigin {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for PlaneAtOrigin {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for PlaneAtOrigin {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for PlaneAtOrigin {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for PlaneAtOrigin {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for PlaneAtOrigin {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Point {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for Point {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for Point {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for Point {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Point {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for Point {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Point {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Point {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Point {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Point {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for Point {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Point {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Point {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Point {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<PointAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for Point {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Point {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Point {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Flector> for Rotor {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Rotor {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Rotor {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Rotor {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Rotor {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Rotor {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Origin> for Rotor {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for Rotor {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Rotor {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for Rotor {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Rotor {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<TransFlector> for Rotor {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Rotor {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for TransFlector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<FlectorAtInfinity> for TransFlector {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Horizon> for TransFlector {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for TransFlector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for TransFlector {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtOrigin> for TransFlector {
    type Output = LineAtOrigin;

    fn sandwich(self, other: LineAtOrigin) -> LineAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for TransFlector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for TransFlector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for TransFlector {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for TransFlector {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for TransFlector {
    type Output = Origin;

    fn sandwich(self, other: Origin) -> Origin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Plane> for TransFlector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for TransFlector {
    type Output = PlaneAtOrigin;

    fn sandwich(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for TransFlector {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PointAtInfinity> for TransFlector {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for TransFlector {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for TransFlector {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for TransFlector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Translator {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<FlectorAtInfinity> for Translator {
    type Output = FlectorAtInfinity;

    fn sandwich(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Horizon> for Translator {
    type Output = Horizon;

    fn sandwich(self, other: Horizon) -> Horizon {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Line> for Translator {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn sandwich(self, other: LineAtInfinity) -> LineAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<LineAtOrigin> for Translator {
    type Output = Line;

    fn sandwich(self, other: LineAtOrigin) -> Line {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Translator {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Translator {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtInfinity> for Translator {
    type Output = MultiVectorAtInfinity;

    fn sandwich(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<MultiVectorAtOrigin> for Translator {
    type Output = MultiVectorAtOrigin;

    fn sandwich(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Origin> for Translator {
    type Output = Point;

    fn sandwich(self, other: Origin) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Translator {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<PlaneAtOrigin> for Translator {
    type Output = Plane;

    fn sandwich(self, other: PlaneAtOrigin) -> Plane {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<Point> for Translator {
    type Output = Point;

    fn sandwich(self, other: Point) -> Point {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn sandwich(self, other: PointAtInfinity) -> PointAtInfinity {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Rotor> for Translator {
    type Output = Motor;

    fn sandwich(self, other: Rotor) -> Motor {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal()).into()
    }
}

impl Sandwich<TransFlector> for Translator {
    type Output = TransFlector;

    fn sandwich(self, other: TransFlector) -> TransFlector {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl Sandwich<Translator> for Translator {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
    }
}

impl PointInversion<Flector> for Point {
    type Output = Flector;

    fn point_inversion(self, other: Flector) -> Flector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<FlectorAtInfinity> for Point {
    type Output = FlectorAtInfinity;

    fn point_inversion(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Horizon> for Point {
    type Output = Horizon;

    fn point_inversion(self, other: Horizon) -> Horizon {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Line> for Point {
    type Output = Line;

    fn point_inversion(self, other: Line) -> Line {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<LineAtInfinity> for Point {
    type Output = LineAtInfinity;

    fn point_inversion(self, other: LineAtInfinity) -> LineAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<LineAtOrigin> for Point {
    type Output = LineAtOrigin;

    fn point_inversion(self, other: LineAtOrigin) -> LineAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Motor> for Point {
    type Output = Motor;

    fn point_inversion(self, other: Motor) -> Motor {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<MultiVector> for Point {
    type Output = MultiVector;

    fn point_inversion(self, other: MultiVector) -> MultiVector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<MultiVectorAtInfinity> for Point {
    type Output = MultiVectorAtInfinity;

    fn point_inversion(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<MultiVectorAtOrigin> for Point {
    type Output = MultiVectorAtOrigin;

    fn point_inversion(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Origin> for Point {
    type Output = Origin;

    fn point_inversion(self, other: Origin) -> Origin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Plane> for Point {
    type Output = Plane;

    fn point_inversion(self, other: Plane) -> Plane {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<PlaneAtOrigin> for Point {
    type Output = PlaneAtOrigin;

    fn point_inversion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Point> for Point {
    type Output = Point;

    fn point_inversion(self, other: Point) -> Point {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<PointAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn point_inversion(self, other: PointAtInfinity) -> PointAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Rotor> for Point {
    type Output = Rotor;

    fn point_inversion(self, other: Rotor) -> Rotor {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<TransFlector> for Point {
    type Output = TransFlector;

    fn point_inversion(self, other: TransFlector) -> TransFlector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Translator> for Point {
    type Output = Translator;

    fn point_inversion(self, other: Translator) -> Translator {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Flector> for Plane {
    type Output = Flector;

    fn reflect(self, other: Flector) -> Flector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<FlectorAtInfinity> for Plane {
    type Output = FlectorAtInfinity;

    fn reflect(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Horizon> for Plane {
    type Output = Horizon;

    fn reflect(self, other: Horizon) -> Horizon {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Line> for Plane {
    type Output = Line;

    fn reflect(self, other: Line) -> Line {
        self.unitize().sandwich(other)
    }
}

impl Reflect<LineAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn reflect(self, other: LineAtInfinity) -> LineAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<LineAtOrigin> for Plane {
    type Output = LineAtOrigin;

    fn reflect(self, other: LineAtOrigin) -> LineAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Motor> for Plane {
    type Output = Motor;

    fn reflect(self, other: Motor) -> Motor {
        self.unitize().sandwich(other)
    }
}

impl Reflect<MultiVector> for Plane {
    type Output = MultiVector;

    fn reflect(self, other: MultiVector) -> MultiVector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<MultiVectorAtInfinity> for Plane {
    type Output = MultiVectorAtInfinity;

    fn reflect(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<MultiVectorAtOrigin> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reflect(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Origin> for Plane {
    type Output = Origin;

    fn reflect(self, other: Origin) -> Origin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Plane> for Plane {
    type Output = Plane;

    fn reflect(self, other: Plane) -> Plane {
        self.unitize().sandwich(other)
    }
}

impl Reflect<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn reflect(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Point> for Plane {
    type Output = Point;

    fn reflect(self, other: Point) -> Point {
        self.unitize().sandwich(other)
    }
}

impl Reflect<PointAtInfinity> for Plane {
    type Output = PointAtInfinity;

    fn reflect(self, other: PointAtInfinity) -> PointAtInfinity {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Rotor> for Plane {
    type Output = Rotor;

    fn reflect(self, other: Rotor) -> Rotor {
        self.unitize().sandwich(other)
    }
}

impl Reflect<TransFlector> for Plane {
    type Output = TransFlector;

    fn reflect(self, other: TransFlector) -> TransFlector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Translator> for Plane {
    type Output = Translator;

    fn reflect(self, other: Translator) -> Translator {
        self.unitize().sandwich(other)
    }
}

impl Rotate<Flector> for Rotor {
    type Output = Flector;

    fn rotate(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Rotate<FlectorAtInfinity> for Rotor {
    type Output = FlectorAtInfinity;

    fn rotate(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<Horizon> for Rotor {
    type Output = Horizon;

    fn rotate(self, other: Horizon) -> Horizon {
        self.sandwich(other)
    }
}

impl Rotate<Line> for Rotor {
    type Output = Line;

    fn rotate(self, other: Line) -> Line {
        self.sandwich(other)
    }
}

impl Rotate<LineAtInfinity> for Rotor {
    type Output = LineAtInfinity;

    fn rotate(self, other: LineAtInfinity) -> LineAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<LineAtOrigin> for Rotor {
    type Output = LineAtOrigin;

    fn rotate(self, other: LineAtOrigin) -> LineAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Motor> for Rotor {
    type Output = Motor;

    fn rotate(self, other: Motor) -> Motor {
        self.sandwich(other)
    }
}

impl Rotate<MultiVector> for Rotor {
    type Output = MultiVector;

    fn rotate(self, other: MultiVector) -> MultiVector {
        self.sandwich(other)
    }
}

impl Rotate<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn rotate(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<MultiVectorAtOrigin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn rotate(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Origin> for Rotor {
    type Output = Origin;

    fn rotate(self, other: Origin) -> Origin {
        self.sandwich(other)
    }
}

impl Rotate<Plane> for Rotor {
    type Output = Plane;

    fn rotate(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Rotate<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn rotate(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.sandwich(other)
    }
}

impl Rotate<Point> for Rotor {
    type Output = Point;

    fn rotate(self, other: Point) -> Point {
        self.sandwich(other)
    }
}

impl Rotate<PointAtInfinity> for Rotor {
    type Output = PointAtInfinity;

    fn rotate(self, other: PointAtInfinity) -> PointAtInfinity {
        self.sandwich(other)
    }
}

impl Rotate<Rotor> for Rotor {
    type Output = Rotor;

    fn rotate(self, other: Rotor) -> Rotor {
        self.sandwich(other)
    }
}

impl Rotate<TransFlector> for Rotor {
    type Output = TransFlector;

    fn rotate(self, other: TransFlector) -> TransFlector {
        self.sandwich(other)
    }
}

impl Rotate<Translator> for Rotor {
    type Output = Translator;

    fn rotate(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}

impl Transflect<Flector> for TransFlector {
    type Output = Flector;

    fn transflect(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Transflect<FlectorAtInfinity> for TransFlector {
    type Output = FlectorAtInfinity;

    fn transflect(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<Horizon> for TransFlector {
    type Output = Horizon;

    fn transflect(self, other: Horizon) -> Horizon {
        self.sandwich(other)
    }
}

impl Transflect<Line> for TransFlector {
    type Output = Line;

    fn transflect(self, other: Line) -> Line {
        self.sandwich(other)
    }
}

impl Transflect<LineAtInfinity> for TransFlector {
    type Output = LineAtInfinity;

    fn transflect(self, other: LineAtInfinity) -> LineAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<LineAtOrigin> for TransFlector {
    type Output = LineAtOrigin;

    fn transflect(self, other: LineAtOrigin) -> LineAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Motor> for TransFlector {
    type Output = Motor;

    fn transflect(self, other: Motor) -> Motor {
        self.sandwich(other)
    }
}

impl Transflect<MultiVector> for TransFlector {
    type Output = MultiVector;

    fn transflect(self, other: MultiVector) -> MultiVector {
        self.sandwich(other)
    }
}

impl Transflect<MultiVectorAtInfinity> for TransFlector {
    type Output = MultiVectorAtInfinity;

    fn transflect(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<MultiVectorAtOrigin> for TransFlector {
    type Output = MultiVectorAtOrigin;

    fn transflect(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Origin> for TransFlector {
    type Output = Origin;

    fn transflect(self, other: Origin) -> Origin {
        self.sandwich(other)
    }
}

impl Transflect<Plane> for TransFlector {
    type Output = Plane;

    fn transflect(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Transflect<PlaneAtOrigin> for TransFlector {
    type Output = PlaneAtOrigin;

    fn transflect(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.sandwich(other)
    }
}

impl Transflect<Point> for TransFlector {
    type Output = Point;

    fn transflect(self, other: Point) -> Point {
        self.sandwich(other)
    }
}

impl Transflect<PointAtInfinity> for TransFlector {
    type Output = PointAtInfinity;

    fn transflect(self, other: PointAtInfinity) -> PointAtInfinity {
        self.sandwich(other)
    }
}

impl Transflect<Rotor> for TransFlector {
    type Output = Rotor;

    fn transflect(self, other: Rotor) -> Rotor {
        self.sandwich(other)
    }
}

impl Transflect<TransFlector> for TransFlector {
    type Output = TransFlector;

    fn transflect(self, other: TransFlector) -> TransFlector {
        self.sandwich(other)
    }
}

impl Transflect<Translator> for TransFlector {
    type Output = Translator;

    fn transflect(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}

impl Translate<Flector> for Translator {
    type Output = Flector;

    fn translate(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Translate<FlectorAtInfinity> for Translator {
    type Output = FlectorAtInfinity;

    fn translate(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<Horizon> for Translator {
    type Output = Horizon;

    fn translate(self, other: Horizon) -> Horizon {
        self.sandwich(other)
    }
}

impl Translate<Line> for Translator {
    type Output = Line;

    fn translate(self, other: Line) -> Line {
        self.sandwich(other)
    }
}

impl Translate<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn translate(self, other: LineAtInfinity) -> LineAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<LineAtOrigin> for Translator {
    type Output = Line;

    fn translate(self, other: LineAtOrigin) -> Line {
        self.sandwich(other)
    }
}

impl Translate<Motor> for Translator {
    type Output = Motor;

    fn translate(self, other: Motor) -> Motor {
        self.sandwich(other)
    }
}

impl Translate<MultiVector> for Translator {
    type Output = MultiVector;

    fn translate(self, other: MultiVector) -> MultiVector {
        self.sandwich(other)
    }
}

impl Translate<MultiVectorAtInfinity> for Translator {
    type Output = MultiVectorAtInfinity;

    fn translate(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<MultiVectorAtOrigin> for Translator {
    type Output = MultiVectorAtOrigin;

    fn translate(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.sandwich(other)
    }
}

impl Translate<Origin> for Translator {
    type Output = Point;

    fn translate(self, other: Origin) -> Point {
        self.sandwich(other)
    }
}

impl Translate<Plane> for Translator {
    type Output = Plane;

    fn translate(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Translate<PlaneAtOrigin> for Translator {
    type Output = Plane;

    fn translate(self, other: PlaneAtOrigin) -> Plane {
        self.sandwich(other)
    }
}

impl Translate<Point> for Translator {
    type Output = Point;

    fn translate(self, other: Point) -> Point {
        self.sandwich(other)
    }
}

impl Translate<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn translate(self, other: PointAtInfinity) -> PointAtInfinity {
        self.sandwich(other)
    }
}

impl Translate<Rotor> for Translator {
    type Output = Motor;

    fn translate(self, other: Rotor) -> Motor {
        self.sandwich(other)
    }
}

impl Translate<TransFlector> for Translator {
    type Output = TransFlector;

    fn translate(self, other: TransFlector) -> TransFlector {
        self.sandwich(other)
    }
}

impl Translate<Translator> for Translator {
    type Output = Translator;

    fn translate(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}
