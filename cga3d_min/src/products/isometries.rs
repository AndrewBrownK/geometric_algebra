//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::AntiReversal;
use crate::products::geometric::*;
use crate::unitize::Unitize;
use crate::*;

/// self.geometric_anti_product(other).geometric_anti_product(self.anti_reversal())
///
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
///
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

impl Sandwich<Circle> for AntiScalar {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for AntiScalar {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<FlatPoint> for AntiScalar {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Flector> for AntiScalar {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Line> for AntiScalar {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Motor> for AntiScalar {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for AntiScalar {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<RoundPoint> for AntiScalar {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Sphere> for AntiScalar {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Circle> for Circle {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Circle {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Circle {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Circle {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Circle {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Circle {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Circle {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Circle {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Circle {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Circle {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Dipole {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Dipole {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Dipole {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Dipole {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Dipole {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Dipole {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Dipole {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Dipole {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Dipole {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Dipole {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for DualNum {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for DualNum {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for DualNum {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for DualNum {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for DualNum {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for DualNum {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for DualNum {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for DualNum {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for DualNum {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for DualNum {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for FlatPoint {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for FlatPoint {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for FlatPoint {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for FlatPoint {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for FlatPoint {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for FlatPoint {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for FlatPoint {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Flector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Flector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Flector {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Flector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Flector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Flector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Flector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Flector {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Flector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Line {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Line {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Line {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Line {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Line> for Line {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Line {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Line {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Line {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Line {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Motor {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Motor {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Motor {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Motor {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Line> for Motor {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Motor {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Motor {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Motor {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Motor {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for MultiVector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for MultiVector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for MultiVector {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for MultiVector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for MultiVector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for MultiVector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for MultiVector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for MultiVector {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for MultiVector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Plane {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Plane {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Plane {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Plane {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Plane {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Plane {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Plane {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Plane {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Plane {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for RoundPoint {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for RoundPoint {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for RoundPoint {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for RoundPoint {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for RoundPoint {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for RoundPoint {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for RoundPoint {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for RoundPoint {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for RoundPoint {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for RoundPoint {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Scalar {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Dipole> for Scalar {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<FlatPoint> for Scalar {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Scalar {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Scalar {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Scalar {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Scalar {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Scalar {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Scalar {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Sphere> for Scalar {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Circle> for Sphere {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Sphere {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Sphere {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Sphere {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Sphere {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Sphere {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Sphere {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Sphere {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Sphere {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Sphere {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl PointInversion<Circle> for FlatPoint {
    type Output = Circle;

    fn point_inversion(self, other: Circle) -> Circle {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Dipole> for FlatPoint {
    type Output = Dipole;

    fn point_inversion(self, other: Dipole) -> Dipole {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<FlatPoint> for FlatPoint {
    type Output = FlatPoint;

    fn point_inversion(self, other: FlatPoint) -> FlatPoint {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Flector> for FlatPoint {
    type Output = Flector;

    fn point_inversion(self, other: Flector) -> Flector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Line> for FlatPoint {
    type Output = Line;

    fn point_inversion(self, other: Line) -> Line {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Motor> for FlatPoint {
    type Output = Motor;

    fn point_inversion(self, other: Motor) -> Motor {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<MultiVector> for FlatPoint {
    type Output = MultiVector;

    fn point_inversion(self, other: MultiVector) -> MultiVector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Plane> for FlatPoint {
    type Output = Plane;

    fn point_inversion(self, other: Plane) -> Plane {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<RoundPoint> for FlatPoint {
    type Output = RoundPoint;

    fn point_inversion(self, other: RoundPoint) -> RoundPoint {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Sphere> for FlatPoint {
    type Output = Sphere;

    fn point_inversion(self, other: Sphere) -> Sphere {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Circle> for Plane {
    type Output = Circle;

    fn reflect(self, other: Circle) -> Circle {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Dipole> for Plane {
    type Output = Dipole;

    fn reflect(self, other: Dipole) -> Dipole {
        self.unitize().sandwich(other)
    }
}

impl Reflect<FlatPoint> for Plane {
    type Output = FlatPoint;

    fn reflect(self, other: FlatPoint) -> FlatPoint {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Flector> for Plane {
    type Output = Flector;

    fn reflect(self, other: Flector) -> Flector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Line> for Plane {
    type Output = Line;

    fn reflect(self, other: Line) -> Line {
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

impl Reflect<Plane> for Plane {
    type Output = Plane;

    fn reflect(self, other: Plane) -> Plane {
        self.unitize().sandwich(other)
    }
}

impl Reflect<RoundPoint> for Plane {
    type Output = RoundPoint;

    fn reflect(self, other: RoundPoint) -> RoundPoint {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Sphere> for Plane {
    type Output = Sphere;

    fn reflect(self, other: Sphere) -> Sphere {
        self.unitize().sandwich(other)
    }
}
