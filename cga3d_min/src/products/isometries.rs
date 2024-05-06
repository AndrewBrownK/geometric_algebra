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

impl Sandwich<Rotor> for AntiScalar {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for AntiScalar {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Translator> for AntiScalar {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for Circle {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Circle {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Circle {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for Dipole {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Dipole {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Dipole {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for DualNum {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for DualNum {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for DualNum {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
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

impl Sandwich<Rotor> for FlatPoint {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for FlatPoint {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for FlatPoint {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
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

impl Sandwich<Rotor> for Flector {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Flector {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Flector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for Line {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Line {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Line {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for Motor {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Motor {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Motor {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for MultiVector {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for MultiVector {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for MultiVector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for Plane {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Plane {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Plane {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Rotor {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Rotor {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Rotor {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Rotor {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Line> for Rotor {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Rotor {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Rotor {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Rotor {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<RoundPoint> for Rotor {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Rotor {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Rotor {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Rotor {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for RoundPoint {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for RoundPoint {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for RoundPoint {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
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

impl Sandwich<Rotor> for Scalar {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Scalar {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Scalar {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
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

impl Sandwich<Rotor> for Sphere {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
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

impl Sandwich<Transflector> for Sphere {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Sphere {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Transflector {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Transflector {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Flector> for Transflector {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Line> for Transflector {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Transflector {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Transflector {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Transflector {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Transflector {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Transflector {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Transflector {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Transflector {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Translator> for Transflector {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Circle> for Translator {
    type Output = Circle;

    fn sandwich(self, other: Circle) -> Circle {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Dipole> for Translator {
    type Output = Dipole;

    fn sandwich(self, other: Dipole) -> Dipole {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<FlatPoint> for Translator {
    type Output = FlatPoint;

    fn sandwich(self, other: FlatPoint) -> FlatPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Flector> for Translator {
    type Output = Flector;

    fn sandwich(self, other: Flector) -> Flector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Line> for Translator {
    type Output = Line;

    fn sandwich(self, other: Line) -> Line {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Motor> for Translator {
    type Output = Motor;

    fn sandwich(self, other: Motor) -> Motor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<MultiVector> for Translator {
    type Output = MultiVector;

    fn sandwich(self, other: MultiVector) -> MultiVector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Plane> for Translator {
    type Output = Plane;

    fn sandwich(self, other: Plane) -> Plane {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Rotor> for Translator {
    type Output = Rotor;

    fn sandwich(self, other: Rotor) -> Rotor {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<RoundPoint> for Translator {
    type Output = RoundPoint;

    fn sandwich(self, other: RoundPoint) -> RoundPoint {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Sphere> for Translator {
    type Output = Sphere;

    fn sandwich(self, other: Sphere) -> Sphere {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal()).into()
    }
}

impl Sandwich<Transflector> for Translator {
    type Output = Transflector;

    fn sandwich(self, other: Transflector) -> Transflector {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
    }
}

impl Sandwich<Translator> for Translator {
    type Output = Translator;

    fn sandwich(self, other: Translator) -> Translator {
        self.anti_wedge_dot(other).anti_wedge_dot(self.anti_reversal())
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

impl PointInversion<Rotor> for FlatPoint {
    type Output = Rotor;

    fn point_inversion(self, other: Rotor) -> Rotor {
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

impl PointInversion<Transflector> for FlatPoint {
    type Output = Transflector;

    fn point_inversion(self, other: Transflector) -> Transflector {
        self.unitize().sandwich(other)
    }
}

impl PointInversion<Translator> for FlatPoint {
    type Output = Translator;

    fn point_inversion(self, other: Translator) -> Translator {
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

impl Reflect<Rotor> for Plane {
    type Output = Rotor;

    fn reflect(self, other: Rotor) -> Rotor {
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

impl Reflect<Transflector> for Plane {
    type Output = Transflector;

    fn reflect(self, other: Transflector) -> Transflector {
        self.unitize().sandwich(other)
    }
}

impl Reflect<Translator> for Plane {
    type Output = Translator;

    fn reflect(self, other: Translator) -> Translator {
        self.unitize().sandwich(other)
    }
}

impl Rotate<Circle> for Rotor {
    type Output = Circle;

    fn rotate(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Rotate<Dipole> for Rotor {
    type Output = Dipole;

    fn rotate(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Rotate<FlatPoint> for Rotor {
    type Output = FlatPoint;

    fn rotate(self, other: FlatPoint) -> FlatPoint {
        self.sandwich(other)
    }
}

impl Rotate<Flector> for Rotor {
    type Output = Flector;

    fn rotate(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Rotate<Line> for Rotor {
    type Output = Line;

    fn rotate(self, other: Line) -> Line {
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

impl Rotate<Plane> for Rotor {
    type Output = Plane;

    fn rotate(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Rotate<Rotor> for Rotor {
    type Output = Rotor;

    fn rotate(self, other: Rotor) -> Rotor {
        self.sandwich(other)
    }
}

impl Rotate<RoundPoint> for Rotor {
    type Output = RoundPoint;

    fn rotate(self, other: RoundPoint) -> RoundPoint {
        self.sandwich(other)
    }
}

impl Rotate<Sphere> for Rotor {
    type Output = Sphere;

    fn rotate(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Rotate<Transflector> for Rotor {
    type Output = Transflector;

    fn rotate(self, other: Transflector) -> Transflector {
        self.sandwich(other)
    }
}

impl Rotate<Translator> for Rotor {
    type Output = Translator;

    fn rotate(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}

impl Transflect<Circle> for Transflector {
    type Output = Circle;

    fn transflect(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Transflect<Dipole> for Transflector {
    type Output = Dipole;

    fn transflect(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Transflect<FlatPoint> for Transflector {
    type Output = FlatPoint;

    fn transflect(self, other: FlatPoint) -> FlatPoint {
        self.sandwich(other)
    }
}

impl Transflect<Flector> for Transflector {
    type Output = Flector;

    fn transflect(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Transflect<Line> for Transflector {
    type Output = Line;

    fn transflect(self, other: Line) -> Line {
        self.sandwich(other)
    }
}

impl Transflect<Motor> for Transflector {
    type Output = Motor;

    fn transflect(self, other: Motor) -> Motor {
        self.sandwich(other)
    }
}

impl Transflect<MultiVector> for Transflector {
    type Output = MultiVector;

    fn transflect(self, other: MultiVector) -> MultiVector {
        self.sandwich(other)
    }
}

impl Transflect<Plane> for Transflector {
    type Output = Plane;

    fn transflect(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Transflect<Rotor> for Transflector {
    type Output = Rotor;

    fn transflect(self, other: Rotor) -> Rotor {
        self.sandwich(other)
    }
}

impl Transflect<RoundPoint> for Transflector {
    type Output = RoundPoint;

    fn transflect(self, other: RoundPoint) -> RoundPoint {
        self.sandwich(other)
    }
}

impl Transflect<Sphere> for Transflector {
    type Output = Sphere;

    fn transflect(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Transflect<Transflector> for Transflector {
    type Output = Transflector;

    fn transflect(self, other: Transflector) -> Transflector {
        self.sandwich(other)
    }
}

impl Transflect<Translator> for Transflector {
    type Output = Translator;

    fn transflect(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}

impl Translate<Circle> for Translator {
    type Output = Circle;

    fn translate(self, other: Circle) -> Circle {
        self.sandwich(other)
    }
}

impl Translate<Dipole> for Translator {
    type Output = Dipole;

    fn translate(self, other: Dipole) -> Dipole {
        self.sandwich(other)
    }
}

impl Translate<FlatPoint> for Translator {
    type Output = FlatPoint;

    fn translate(self, other: FlatPoint) -> FlatPoint {
        self.sandwich(other)
    }
}

impl Translate<Flector> for Translator {
    type Output = Flector;

    fn translate(self, other: Flector) -> Flector {
        self.sandwich(other)
    }
}

impl Translate<Line> for Translator {
    type Output = Line;

    fn translate(self, other: Line) -> Line {
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

impl Translate<Plane> for Translator {
    type Output = Plane;

    fn translate(self, other: Plane) -> Plane {
        self.sandwich(other)
    }
}

impl Translate<Rotor> for Translator {
    type Output = Rotor;

    fn translate(self, other: Rotor) -> Rotor {
        self.sandwich(other)
    }
}

impl Translate<RoundPoint> for Translator {
    type Output = RoundPoint;

    fn translate(self, other: RoundPoint) -> RoundPoint {
        self.sandwich(other)
    }
}

impl Translate<Sphere> for Translator {
    type Output = Sphere;

    fn translate(self, other: Sphere) -> Sphere {
        self.sandwich(other)
    }
}

impl Translate<Transflector> for Translator {
    type Output = Transflector;

    fn translate(self, other: Transflector) -> Transflector {
        self.sandwich(other)
    }
}

impl Translate<Translator> for Translator {
    type Output = Translator;

    fn translate(self, other: Translator) -> Translator {
        self.sandwich(other)
    }
}
