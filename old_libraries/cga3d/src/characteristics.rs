// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::dot::*;
use crate::products::exterior::*;
use crate::products::geometric::*;
use crate::*;

/// Inverse, as in `x^-1` (with respect to geometric product).
/// Useful to define the geometric quotient.
/// Not to be confused with the "Point Inversion" or "Sphere Inversion" operations.
pub trait Inverse {
    type Output;
    fn inverse(self) -> Self::Output;
}

/// Anti Inverse, as in `x^-1` (with respect to geometric anti-product).
/// Useful to define the geometric anti-quotient.
/// Not to be confused with the "Point Inversion" or "Sphere Inversion" operations.
pub trait AntiInverse {
    type Output;
    fn anti_inverse(self) -> Self::Output;
}

/// Square Root (with respect to geometric product)
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

/// Anti Square Root (with respect to geometric anti-product)
pub trait AntiSqrt {
    type Output;
    fn anti_sqrt(self) -> Self::Output;
}

/// Inverse Square Root (with respect to geometric product)
pub trait InverseSqrt {
    type Output;
    fn inverse_sqrt(self) -> Self::Output;
}

/// Anti Inverse Square Root (with respect to geometric anti-product)
pub trait AntiInverseSqrt {
    type Output;
    fn anti_inverse_sqrt(self) -> Self::Output;
}

/// Self to the power of other (with respect to geometric product)
pub trait Pow<T> {
    type Output;
    fn pow(self, other: T) -> Self::Output;
}

/// Self to the power of other (with respect to geometric anti-product)
pub trait AntiPow<T> {
    type Output;
    fn anti_pow(self, other: T) -> Self::Output;
}

/// Natural Exponentiation (with respect to geometric product)
pub trait Exp {
    type Output;
    fn exp(self) -> Self::Output;
}

/// Anti Natural Exponentiation (with respect to geometric anti-product)
pub trait AntiExp {
    type Output;
    fn anti_exp(self) -> Self::Output;
}

/// Sine (with respect to geometric product)
pub trait Sin {
    type Output;
    fn sin(self) -> Self::Output;
}

/// Anti Sine (with respect to geometric anti-product)
/// Be careful not to confuse with "asin" aka "arcsin" aka "inverse sine".
pub trait AntiSin {
    type Output;
    fn anti_sin(self) -> Self::Output;
}

/// Cosine (with respect to geometric product)
pub trait Cos {
    type Output;
    fn cos(self) -> Self::Output;
}

/// Anti Cosine (with respect to geometric anti-product)
/// Be careful not to confuse with "acos" aka "arccos" aka "inverse cosine".
pub trait AntiCos {
    type Output;
    fn anti_cos(self) -> Self::Output;
}

/// Tangent (with respect to geometric product)
pub trait Tan {
    type Output;
    fn tan(self) -> Self::Output;
}

/// Anti Tangent (with respect to geometric anti-product)
/// Be careful not to confuse with "atan" aka "arctan" aka "inverse tangent".
pub trait AntiTan {
    type Output;
    fn anti_tan(self) -> Self::Output;
}

/// Hyperbolic Sine (with respect to geometric product)
pub trait Sinh {
    type Output;
    fn sinh(self) -> Self::Output;
}

/// Anti Hyperbolic Sine (with respect to geometric anti-product)
/// Be careful not to confuse with "asinh" aka "arcsinh" aka "inverse hyperbolic sine".
pub trait AntiSinh {
    type Output;
    fn anti_sinh(self) -> Self::Output;
}

/// Hyperbolic Cosine (with respect to geometric product)
pub trait Cosh {
    type Output;
    fn cosh(self) -> Self::Output;
}

/// Anti Hyperbolic Cosine (with respect to geometric anti-product)
/// Be careful not to confuse with "acosh" aka "arccosh" aka "inverse hyperbolic cosine".
pub trait AntiCosh {
    type Output;
    fn anti_cosh(self) -> Self::Output;
}

/// Hyperbolic Tangent (with respect to geometric product)
pub trait Tanh {
    type Output;
    fn tanh(self) -> Self::Output;
}

/// Anti Hyperbolic Tangent (with respect to geometric anti-product)
/// Be careful not to confuse with "atanh" aka "arctanh" aka "inverse hyperbolic tangent".
pub trait AntiTanh {
    type Output;
    fn anti_tanh(self) -> Self::Output;
}

/// Grade
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
pub trait Grade {
    type Output;
    fn grade() -> Self::Output;
}

/// Anti-Grade
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
pub trait AntiGrade {
    type Output;
    fn anti_grade() -> Self::Output;
}

/// Attitude
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Attitude
pub trait Attitude {
    type Output;
    fn attitude(self) -> Self::Output;
}

/// Carrier
/// The Carrier of a round object is the lowest dimensional flat object that contains it.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Carriers
pub trait Carrier {
    type Output;
    fn carrier(self) -> Self::Output;
}

/// CoCarrier
/// The CoCarrier of a round object is the Carrier of its antidual.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Carriers
pub trait CoCarrier {
    type Output;
    fn co_carrier(self) -> Self::Output;
}

/// Container
/// The Container of a round object is the smallest Sphere that contains it.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Containers
pub trait Container {
    type Output;
    fn container(self) -> Self::Output;
}

/// Center
/// The Center of a round object is the RoundPoint having the same center and radius.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Centers
pub trait Center {
    type Output;
    fn center(self) -> Self::Output;
}

/// Partner
/// The Partner of a round object is the round object having the same center, same carrier,
/// and same absolute size, but having a squared radius of the opposite sign.
/// The dot product between a round object and its partner is always zero. They are orthogonal.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Partners
pub trait Partner {
    type Output;
    fn partner(self) -> Self::Output;
}

impl AntiGrade for AntiCircleOnOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for AntiDipoleOnOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for AntiFlatPointAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for AntiLineAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for AntiPlane {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for AntiPlaneAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for AntiScalar {
    type Output = isize;

    fn anti_grade() -> isize {
        0
    }
}

impl AntiGrade for AntiSphereOnOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for Circle {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleAligningOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleOnOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for CircleOrthogonalOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for Dipole {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleAligningOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleOnOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for DipoleOrthogonalOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for FlatPoint {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for FlatPointAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for FlatPointAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for Horizon {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for Infinity {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for Line {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for LineAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for LineAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for NullCircleAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for NullDipoleAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for NullSphereAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for Origin {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for Plane {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for PlaneAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for RoundPoint {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for RoundPointAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl AntiGrade for Scalar {
    type Output = isize;

    fn anti_grade() -> isize {
        5
    }
}

impl AntiGrade for Sphere {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for SphereAtOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl AntiGrade for SphereOnOrigin {
    type Output = isize;

    fn anti_grade() -> isize {
        1
    }
}

impl Grade for AntiCircleOnOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for AntiDipoleOnOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for AntiFlatPointAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for AntiLineAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for AntiPlane {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for AntiPlaneAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for AntiScalar {
    type Output = isize;

    fn grade() -> isize {
        5
    }
}

impl Grade for AntiSphereOnOrigin {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for Circle {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleAligningOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleOnOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for CircleOrthogonalOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for Dipole {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleAligningOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleOnOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for DipoleOrthogonalOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for FlatPoint {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for FlatPointAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for FlatPointAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for Horizon {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for Infinity {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for Line {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for LineAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for LineAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for NullCircleAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for NullDipoleAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for NullSphereAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for Origin {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for Plane {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for PlaneAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for RoundPoint {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for RoundPointAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for Scalar {
    type Output = isize;

    fn grade() -> isize {
        0
    }
}

impl Grade for Sphere {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for SphereAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for SphereOnOrigin {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl AntiSqrt for AntiScalar {
    type Output = AntiScalar;

    fn anti_sqrt(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0().sqrt() },
        }
    }
}

impl AntiSqrt for DualNum {
    type Output = DualNum;

    fn anti_sqrt(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut sqrt_t: f32 = t.sqrt();
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([s / (2.0 * sqrt_t), sqrt_t]),
            },
        }
    }
}

impl Attitude for AntiCircleOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn attitude(self) -> AntiPlaneAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for AntiDipoleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn attitude(self) -> AntiLineAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for AntiScalar {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for AntiSphereOnOrigin {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Circle {
    type Output = DipoleAtInfinity;

    fn attitude(self) -> DipoleAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for CircleAligningOrigin {
    type Output = DipoleAtInfinity;

    fn attitude(self) -> DipoleAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for CircleAtInfinity {
    type Output = FlatPointAtInfinity;

    fn attitude(self) -> FlatPointAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for CircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn attitude(self) -> AntiLineAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for CircleOnOrigin {
    type Output = DipoleAtInfinity;

    fn attitude(self) -> DipoleAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for CircleOrthogonalOrigin {
    type Output = AntiLineAtOrigin;

    fn attitude(self) -> AntiLineAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Dilator {
    type Output = MultiVector;

    fn attitude(self) -> MultiVector {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Dipole {
    type Output = AntiPlane;

    fn attitude(self) -> AntiPlane {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for DipoleAligningOrigin {
    type Output = AntiPlane;

    fn attitude(self) -> AntiPlane {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for DipoleAtInfinity {
    type Output = Infinity;

    fn attitude(self) -> Infinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for DipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn attitude(self) -> AntiPlaneAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for DipoleOnOrigin {
    type Output = AntiPlane;

    fn attitude(self) -> AntiPlane {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for DipoleOrthogonalOrigin {
    type Output = AntiPlaneAtOrigin;

    fn attitude(self) -> AntiPlaneAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for DualNum {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for FlatPoint {
    type Output = Infinity;

    fn attitude(self) -> Infinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for FlatPointAtOrigin {
    type Output = Infinity;

    fn attitude(self) -> Infinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Flector {
    type Output = MultiVector;

    fn attitude(self) -> MultiVector {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Line {
    type Output = FlatPointAtInfinity;

    fn attitude(self) -> FlatPointAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for LineAtOrigin {
    type Output = FlatPointAtInfinity;

    fn attitude(self) -> FlatPointAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Motor {
    type Output = FlectorAtInfinity;

    fn attitude(self) -> FlectorAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for MultiVector {
    type Output = MultiVector;

    fn attitude(self) -> MultiVector {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for NullCircleAtOrigin {
    type Output = AntiLineAtOrigin;

    fn attitude(self) -> AntiLineAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for NullDipoleAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn attitude(self) -> AntiPlaneAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for NullSphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn attitude(self) -> AntiFlatPointAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Origin {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Plane {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Rotor {
    type Output = FlectorAtInfinity;

    fn attitude(self) -> FlectorAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for RoundPoint {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for RoundPointAtOrigin {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Sphere {
    type Output = CircleOrthogonalOrigin;

    fn attitude(self) -> CircleOrthogonalOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for SphereAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn attitude(self) -> AntiFlatPointAtOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for SphereOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn attitude(self) -> CircleOrthogonalOrigin {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Transflector {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Translator {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::unit())
    }
}

impl Carrier for AntiCircleOnOrigin {
    type Output = Line;

    fn carrier(self) -> Line {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for AntiDipoleOnOrigin {
    type Output = Plane;

    fn carrier(self) -> Plane {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for AntiFlatPointAtOrigin {
    type Output = Horizon;

    fn carrier(self) -> Horizon {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for AntiLineAtOrigin {
    type Output = LineAtInfinity;

    fn carrier(self) -> LineAtInfinity {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for AntiPlane {
    type Output = FlatPointAtInfinity;

    fn carrier(self) -> FlatPointAtInfinity {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for AntiPlaneAtOrigin {
    type Output = FlatPointAtInfinity;

    fn carrier(self) -> FlatPointAtInfinity {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for AntiSphereOnOrigin {
    type Output = FlatPoint;

    fn carrier(self) -> FlatPoint {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for Circle {
    type Output = Plane;

    fn carrier(self) -> Plane {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for CircleAligningOrigin {
    type Output = PlaneAtOrigin;

    fn carrier(self) -> PlaneAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for CircleAtInfinity {
    type Output = Horizon;

    fn carrier(self) -> Horizon {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for CircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn carrier(self) -> PlaneAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for CircleOnOrigin {
    type Output = PlaneAtOrigin;

    fn carrier(self) -> PlaneAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for CircleOrthogonalOrigin {
    type Output = Plane;

    fn carrier(self) -> Plane {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for Dilator {
    type Output = Plane;

    fn carrier(self) -> Plane {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for Dipole {
    type Output = Line;

    fn carrier(self) -> Line {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for DipoleAligningOrigin {
    type Output = LineAtOrigin;

    fn carrier(self) -> LineAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for DipoleAtInfinity {
    type Output = LineAtInfinity;

    fn carrier(self) -> LineAtInfinity {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for DipoleAtOrigin {
    type Output = LineAtOrigin;

    fn carrier(self) -> LineAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for DipoleOnOrigin {
    type Output = LineAtOrigin;

    fn carrier(self) -> LineAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for DipoleOrthogonalOrigin {
    type Output = Line;

    fn carrier(self) -> Line {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for DualNum {
    type Output = Infinity;

    fn carrier(self) -> Infinity {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for MultiVector {
    type Output = MultiVector;

    fn carrier(self) -> MultiVector {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for NullCircleAtOrigin {
    type Output = PlaneAtOrigin;

    fn carrier(self) -> PlaneAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for NullDipoleAtOrigin {
    type Output = LineAtOrigin;

    fn carrier(self) -> LineAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for NullSphereAtOrigin {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for Origin {
    type Output = FlatPointAtOrigin;

    fn carrier(self) -> FlatPointAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for RoundPoint {
    type Output = FlatPoint;

    fn carrier(self) -> FlatPoint {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for RoundPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn carrier(self) -> FlatPointAtOrigin {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for Scalar {
    type Output = Infinity;

    fn carrier(self) -> Infinity {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for Sphere {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for SphereAtOrigin {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(Infinity::unit())
    }
}

impl Carrier for SphereOnOrigin {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(Infinity::unit())
    }
}

impl CoCarrier for AntiCircleOnOrigin {
    type Output = PlaneAtOrigin;

    fn co_carrier(self) -> PlaneAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for AntiDipoleOnOrigin {
    type Output = LineAtOrigin;

    fn co_carrier(self) -> LineAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for AntiSphereOnOrigin {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for Circle {
    type Output = Line;

    fn co_carrier(self) -> Line {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for CircleAligningOrigin {
    type Output = Line;

    fn co_carrier(self) -> Line {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for CircleAtInfinity {
    type Output = LineAtInfinity;

    fn co_carrier(self) -> LineAtInfinity {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for CircleAtOrigin {
    type Output = LineAtOrigin;

    fn co_carrier(self) -> LineAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for CircleOnOrigin {
    type Output = Line;

    fn co_carrier(self) -> Line {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for CircleOrthogonalOrigin {
    type Output = LineAtOrigin;

    fn co_carrier(self) -> LineAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for Dilator {
    type Output = MultiVector;

    fn co_carrier(self) -> MultiVector {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for Dipole {
    type Output = Plane;

    fn co_carrier(self) -> Plane {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for DipoleAligningOrigin {
    type Output = Plane;

    fn co_carrier(self) -> Plane {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for DipoleAtInfinity {
    type Output = Horizon;

    fn co_carrier(self) -> Horizon {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for DipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn co_carrier(self) -> PlaneAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for DipoleOnOrigin {
    type Output = Plane;

    fn co_carrier(self) -> Plane {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for DipoleOrthogonalOrigin {
    type Output = PlaneAtOrigin;

    fn co_carrier(self) -> PlaneAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for DualNum {
    type Output = Infinity;

    fn co_carrier(self) -> Infinity {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for MultiVector {
    type Output = MultiVector;

    fn co_carrier(self) -> MultiVector {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for NullCircleAtOrigin {
    type Output = LineAtOrigin;

    fn co_carrier(self) -> LineAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for NullDipoleAtOrigin {
    type Output = PlaneAtOrigin;

    fn co_carrier(self) -> PlaneAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for NullSphereAtOrigin {
    type Output = FlatPointAtOrigin;

    fn co_carrier(self) -> FlatPointAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for Origin {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for RoundPoint {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for Sphere {
    type Output = FlatPoint;

    fn co_carrier(self) -> FlatPoint {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for SphereAtOrigin {
    type Output = FlatPointAtOrigin;

    fn co_carrier(self) -> FlatPointAtOrigin {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl CoCarrier for SphereOnOrigin {
    type Output = FlatPoint;

    fn co_carrier(self) -> FlatPoint {
        self.anti_dual().wedge(Infinity::unit())
    }
}

impl Sqrt for DualNum {
    type Output = DualNum;

    fn sqrt(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut sqrt_s: f32 = s.sqrt();
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([sqrt_s, t / (2.0 * sqrt_s)]),
            },
        }
    }
}

impl Sqrt for Scalar {
    type Output = Scalar;

    fn sqrt(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0().sqrt() },
        }
    }
}

impl Center for AntiCircleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn center(self) -> AntiSphereOnOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for AntiDipoleOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn center(self) -> AntiSphereOnOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn center(self) -> AntiSphereOnOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Circle {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for CircleAligningOrigin {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for CircleAtInfinity {
    type Output = Infinity;

    fn center(self) -> Infinity {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for CircleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn center(self) -> RoundPointAtOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for CircleOnOrigin {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for CircleOrthogonalOrigin {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Dilator {
    type Output = MultiVector;

    fn center(self) -> MultiVector {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Dipole {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DipoleAligningOrigin {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DipoleAtInfinity {
    type Output = Infinity;

    fn center(self) -> Infinity {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DipoleAtOrigin {
    type Output = RoundPointAtOrigin;

    fn center(self) -> RoundPointAtOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DipoleOnOrigin {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DipoleOrthogonalOrigin {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DualNum {
    type Output = Infinity;

    fn center(self) -> Infinity {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for MultiVector {
    type Output = MultiVector;

    fn center(self) -> MultiVector {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for NullCircleAtOrigin {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for NullDipoleAtOrigin {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for NullSphereAtOrigin {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Origin {
    type Output = Origin;

    fn center(self) -> Origin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for RoundPoint {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn center(self) -> RoundPointAtOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Sphere {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn center(self) -> RoundPointAtOrigin {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for SphereOnOrigin {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Container for AntiCircleOnOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for AntiDipoleOnOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for AntiFlatPointAtOrigin {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for AntiLineAtOrigin {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for AntiPlane {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for AntiPlaneAtOrigin {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for AntiSphereOnOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Circle {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleAligningOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleAtInfinity {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleAtOrigin {
    type Output = SphereAtOrigin;

    fn container(self) -> SphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleOnOrigin {
    type Output = SphereOnOrigin;

    fn container(self) -> SphereOnOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for CircleOrthogonalOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Dilator {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Dipole {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleAligningOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleAtInfinity {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleAtOrigin {
    type Output = SphereAtOrigin;

    fn container(self) -> SphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleOnOrigin {
    type Output = SphereOnOrigin;

    fn container(self) -> SphereOnOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DipoleOrthogonalOrigin {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for DualNum {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for MultiVector {
    type Output = MultiVector;

    fn container(self) -> MultiVector {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for NullCircleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn container(self) -> NullSphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for NullDipoleAtOrigin {
    type Output = NullSphereAtOrigin;

    fn container(self) -> NullSphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn container(self) -> NullSphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Origin {
    type Output = NullSphereAtOrigin;

    fn container(self) -> NullSphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPoint {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn container(self) -> SphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Scalar {
    type Output = Horizon;

    fn container(self) -> Horizon {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Sphere {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn container(self) -> SphereAtOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn container(self) -> SphereOnOrigin {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Partner for AntiCircleOnOrigin {
    type Output = Dipole;

    fn partner(self) -> Dipole {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for AntiDipoleOnOrigin {
    type Output = Circle;

    fn partner(self) -> Circle {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for AntiSphereOnOrigin {
    type Output = RoundPoint;

    fn partner(self) -> RoundPoint {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Circle {
    type Output = Circle;

    fn partner(self) -> Circle {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn partner(self) -> CircleAligningOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn partner(self) -> CircleAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for CircleOnOrigin {
    type Output = CircleAligningOrigin;

    fn partner(self) -> CircleAligningOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for CircleOrthogonalOrigin {
    type Output = Circle;

    fn partner(self) -> Circle {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Dilator {
    type Output = MultiVector;

    fn partner(self) -> MultiVector {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Dipole {
    type Output = Dipole;

    fn partner(self) -> Dipole {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn partner(self) -> DipoleAligningOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn partner(self) -> DipoleAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;

    fn partner(self) -> DipoleAligningOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for DipoleOrthogonalOrigin {
    type Output = Dipole;

    fn partner(self) -> Dipole {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for MultiVector {
    type Output = MultiVector;

    fn partner(self) -> MultiVector {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn partner(self) -> NullCircleAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn partner(self) -> NullDipoleAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn partner(self) -> NullSphereAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Origin {
    type Output = Origin;

    fn partner(self) -> Origin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for RoundPoint {
    type Output = RoundPoint;

    fn partner(self) -> RoundPoint {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn partner(self) -> RoundPointAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Sphere {
    type Output = Sphere;

    fn partner(self) -> Sphere {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn partner(self) -> SphereAtOrigin {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for SphereOnOrigin {
    type Output = Sphere;

    fn partner(self) -> Sphere {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl AntiInverse for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn anti_inverse(self) -> AntiCircleOnOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn anti_inverse(self) -> AntiDipoleOnOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn anti_inverse(self) -> AntiFlatPointAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn anti_inverse(self) -> AntiLineAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for AntiPlane {
    type Output = AntiPlane;

    fn anti_inverse(self) -> AntiPlane {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn anti_inverse(self) -> AntiPlaneAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for AntiScalar {
    type Output = AntiScalar;

    fn anti_inverse(self) -> AntiScalar {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn anti_inverse(self) -> AntiSphereOnOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Circle {
    type Output = Circle;

    fn anti_inverse(self) -> Circle {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn anti_inverse(self) -> CircleAligningOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn anti_inverse(self) -> CircleAtInfinity {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn anti_inverse(self) -> CircleAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn anti_inverse(self) -> CircleOnOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn anti_inverse(self) -> CircleOrthogonalOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Dilator {
    type Output = Dilator;

    fn anti_inverse(self) -> Dilator {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Dipole {
    type Output = Dipole;

    fn anti_inverse(self) -> Dipole {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn anti_inverse(self) -> DipoleAligningOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn anti_inverse(self) -> DipoleAtInfinity {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn anti_inverse(self) -> DipoleAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn anti_inverse(self) -> DipoleOnOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn anti_inverse(self) -> DipoleOrthogonalOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for DualNum {
    type Output = DualNum;

    fn anti_inverse(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([-1.0 * s / (t * t), 1.0 / t]),
            },
        }
    }
}

impl AntiInverse for FlatPoint {
    type Output = FlatPoint;

    fn anti_inverse(self) -> FlatPoint {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_inverse(self) -> FlatPointAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Flector {
    type Output = Flector;

    fn anti_inverse(self) -> Flector {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Line {
    type Output = Line;

    fn anti_inverse(self) -> Line {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_inverse(self) -> LineAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Motor {
    type Output = Motor;

    fn anti_inverse(self) -> Motor {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for MultiVector {
    type Output = MultiVector;

    fn anti_inverse(self) -> MultiVector {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Plane {
    type Output = Plane;

    fn anti_inverse(self) -> Plane {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_inverse(self) -> PlaneAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Rotor {
    type Output = Rotor;

    fn anti_inverse(self) -> Rotor {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPoint {
    type Output = RoundPoint;

    fn anti_inverse(self) -> RoundPoint {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_inverse(self) -> RoundPointAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Scalar {
    type Output = Scalar;

    fn anti_inverse(self) -> Scalar {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Sphere {
    type Output = Sphere;

    fn anti_inverse(self) -> Sphere {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn anti_inverse(self) -> SphereAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn anti_inverse(self) -> SphereOnOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Transflector {
    type Output = Transflector;

    fn anti_inverse(self) -> Transflector {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Translator {
    type Output = Translator;

    fn anti_inverse(self) -> Translator {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl Inverse for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn inverse(self) -> AntiCircleOnOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn inverse(self) -> AntiDipoleOnOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn inverse(self) -> AntiFlatPointAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn inverse(self) -> AntiLineAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for AntiPlane {
    type Output = AntiPlane;

    fn inverse(self) -> AntiPlane {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn inverse(self) -> AntiPlaneAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for AntiScalar {
    type Output = AntiScalar;

    fn inverse(self) -> AntiScalar {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn inverse(self) -> AntiSphereOnOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Circle {
    type Output = Circle;

    fn inverse(self) -> Circle {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn inverse(self) -> CircleAligningOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn inverse(self) -> CircleAtInfinity {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn inverse(self) -> CircleAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn inverse(self) -> CircleOnOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn inverse(self) -> CircleOrthogonalOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Dilator {
    type Output = Dilator;

    fn inverse(self) -> Dilator {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Dipole {
    type Output = Dipole;

    fn inverse(self) -> Dipole {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn inverse(self) -> DipoleAligningOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn inverse(self) -> DipoleAtInfinity {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn inverse(self) -> DipoleAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn inverse(self) -> DipoleOnOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn inverse(self) -> DipoleOrthogonalOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for DualNum {
    type Output = DualNum;

    fn inverse(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([1.0 / s, -1.0 * t / (s * s)]),
            },
        }
    }
}

impl Inverse for FlatPoint {
    type Output = FlatPoint;

    fn inverse(self) -> FlatPoint {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn inverse(self) -> FlatPointAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Flector {
    type Output = Flector;

    fn inverse(self) -> Flector {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for LineAtOrigin {
    type Output = LineAtOrigin;

    fn inverse(self) -> LineAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn inverse(self) -> PlaneAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Rotor {
    type Output = Rotor;

    fn inverse(self) -> Rotor {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for RoundPoint {
    type Output = RoundPoint;

    fn inverse(self) -> RoundPoint {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn inverse(self) -> RoundPointAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Sphere {
    type Output = Sphere;

    fn inverse(self) -> Sphere {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn inverse(self) -> SphereAtOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn inverse(self) -> SphereOnOrigin {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Transflector {
    type Output = Transflector;

    fn inverse(self) -> Transflector {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Translator {
    type Output = Translator;

    fn inverse(self) -> Translator {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl AntiCos for DualNum {
    type Output = DualNum;

    fn anti_cos(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([-1.0 * s * f32::sin(t), f32::cos(t)]),
            },
        }
    }
}

impl AntiCosh for AntiScalar {
    type Output = AntiScalar;

    fn anti_cosh(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: f32::cosh(self.group0()) },
        }
    }
}

impl AntiCosh for DualNum {
    type Output = DualNum;

    fn anti_cosh(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([s * f32::sinh(t), f32::cosh(t)]),
            },
        }
    }
}

impl AntiExp for AntiScalar {
    type Output = AntiScalar;

    fn anti_exp(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: f32::exp(self.group0()) },
        }
    }
}

impl AntiExp for DualNum {
    type Output = DualNum;

    fn anti_exp(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut exp_t: f32 = f32::exp(t);
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([s * exp_t, exp_t]),
            },
        }
    }
}

impl AntiInverseSqrt for AntiScalar {
    type Output = AntiScalar;

    fn anti_inverse_sqrt(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: 1.0 / self.group0().sqrt() },
        }
    }
}

impl AntiInverseSqrt for DualNum {
    type Output = DualNum;

    fn anti_inverse_sqrt(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut sqrt_t: f32 = t.sqrt();
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([-1.0 * s / (2.0 * t * sqrt_t), 1.0 / sqrt_t]),
            },
        }
    }
}

impl AntiPow<f32> for AntiScalar {
    type Output = AntiScalar;

    fn anti_pow(self, other: f32) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups {
                g0: f32::powf(self.group0(), other),
            },
        }
    }
}

impl AntiPow<f32> for DualNum {
    type Output = DualNum;

    fn anti_pow(self, other: f32) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([other * f32::powf(t, other - 1.0) * s, f32::powf(t, other)]),
            },
        }
    }
}

impl AntiSin for DualNum {
    type Output = DualNum;

    fn anti_sin(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([s * f32::cos(t), f32::sin(t)]),
            },
        }
    }
}

impl AntiSinh for AntiScalar {
    type Output = AntiScalar;

    fn anti_sinh(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: f32::sinh(self.group0()) },
        }
    }
}

impl AntiSinh for DualNum {
    type Output = DualNum;

    fn anti_sinh(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([s * f32::cosh(t), f32::sinh(t)]),
            },
        }
    }
}

impl AntiTan for DualNum {
    type Output = DualNum;

    fn anti_tan(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut tan_t: f32 = f32::tan(t);
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([s * (1.0 + tan_t * tan_t), tan_t]),
            },
        }
    }
}

impl AntiTanh for AntiScalar {
    type Output = AntiScalar;

    fn anti_tanh(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: f32::tanh(self.group0()) },
        }
    }
}

impl AntiTanh for DualNum {
    type Output = DualNum;

    fn anti_tanh(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut tanh_t: f32 = f32::tanh(t);
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([s * (1.0 - tanh_t * tanh_t), tanh_t]),
            },
        }
    }
}

impl Cos for DualNum {
    type Output = DualNum;

    fn cos(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([f32::cos(s), -1.0 * t * f32::sin(s)]),
            },
        }
    }
}

impl Cosh for DualNum {
    type Output = DualNum;

    fn cosh(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([f32::cosh(s), t * f32::sinh(s)]),
            },
        }
    }
}

impl Cosh for Scalar {
    type Output = Scalar;

    fn cosh(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: f32::cosh(self.group0()) },
        }
    }
}

impl Exp for DualNum {
    type Output = DualNum;

    fn exp(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut exp_s: f32 = f32::exp(s);
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([exp_s, t * exp_s]),
            },
        }
    }
}

impl Exp for Scalar {
    type Output = Scalar;

    fn exp(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: f32::exp(self.group0()) },
        }
    }
}

impl InverseSqrt for DualNum {
    type Output = DualNum;

    fn inverse_sqrt(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut sqrt_s: f32 = s.sqrt();
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([1.0 / sqrt_s, -1.0 * t / (2.0 * s * sqrt_s)]),
            },
        }
    }
}

impl InverseSqrt for Scalar {
    type Output = Scalar;

    fn inverse_sqrt(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: 1.0 / self.group0().sqrt() },
        }
    }
}

impl Pow<f32> for DualNum {
    type Output = DualNum;

    fn pow(self, other: f32) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([f32::powf(s, other), other * f32::powf(s, other - 1.0) * t]),
            },
        }
    }
}

impl Pow<f32> for Scalar {
    type Output = Scalar;

    fn pow(self, other: f32) -> Scalar {
        Scalar {
            groups: ScalarGroups {
                g0: f32::powf(self.group0(), other),
            },
        }
    }
}

impl Sin for DualNum {
    type Output = DualNum;

    fn sin(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([f32::sin(s), t * f32::cos(s)]),
            },
        }
    }
}

impl Sinh for DualNum {
    type Output = DualNum;

    fn sinh(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([f32::sinh(s), t * f32::cosh(s)]),
            },
        }
    }
}

impl Sinh for Scalar {
    type Output = Scalar;

    fn sinh(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: f32::sinh(self.group0()) },
        }
    }
}

impl Tan for DualNum {
    type Output = DualNum;

    fn tan(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut tan_s: f32 = f32::tan(s);
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([tan_s, t * (1.0 + tan_s * tan_s)]),
            },
        }
    }
}

impl Tanh for DualNum {
    type Output = DualNum;

    fn tanh(self) -> DualNum {
        let mut s: f32 = self.group0()[0];
        let mut t: f32 = self.group0()[1];
        let mut tanh_s: f32 = f32::tanh(s);
        DualNum {
            groups: DualNumGroups {
                g0: Simd32x2::from([tanh_s, t * (1.0 - tanh_s * tanh_s)]),
            },
        }
    }
}

impl Tanh for Scalar {
    type Output = Scalar;

    fn tanh(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: f32::tanh(self.group0()) },
        }
    }
}
