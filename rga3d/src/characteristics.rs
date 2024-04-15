//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::products::dot::AntiDot;
use crate::products::dot::Dot;
use crate::products::exterior::AntiWedge;
use crate::products::geometric::GeometricAntiProduct;
use crate::products::geometric::GeometricProduct;
use crate::*;

/// Square Root
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
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

/// Inverse, as in `x^-1` (with respect to geometric product).
/// Useful to define the geometric quotient.
/// Not to be confused with the "Point Inversion" or "Sphere Inversion" operations.
pub trait Inverse {
    type Output;
    fn inverse(self) -> Self::Output;
}

/// Inverse, as in `x^-1` (with respect to geometric anti-product).
/// Useful to define the geometric anti-quotient.
/// Not to be confused with the "Point Inversion" or "Sphere Inversion" operations.
pub trait AntiInverse {
    type Output;
    fn anti_inverse(self) -> Self::Output;
}

impl AntiGrade for AntiScalar {
    type Output = isize;

    fn anti_grade() -> isize {
        0
    }
}

impl AntiGrade for Horizon {
    type Output = isize;

    fn anti_grade() -> isize {
        1
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

impl AntiGrade for Origin {
    type Output = isize;

    fn anti_grade() -> isize {
        3
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

impl AntiGrade for Point {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for PointAtInfinity {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for Scalar {
    type Output = isize;

    fn anti_grade() -> isize {
        4
    }
}

impl Grade for AntiScalar {
    type Output = isize;

    fn grade() -> isize {
        4
    }
}

impl Grade for Horizon {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for Line {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for LineAtInfinity {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for LineAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        2
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
        3
    }
}

impl Grade for PlaneAtOrigin {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for Point {
    type Output = isize;

    fn grade() -> isize {
        1
    }
}

impl Grade for PointAtInfinity {
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

impl Attitude for AntiScalar {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Flector {
    type Output = MultiVectorAtInfinity;

    fn attitude(self) -> MultiVectorAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Line {
    type Output = PointAtInfinity;

    fn attitude(self) -> PointAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for LineAtOrigin {
    type Output = PointAtInfinity;

    fn attitude(self) -> PointAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Magnitude {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Motor {
    type Output = FlectorAtInfinity;

    fn attitude(self) -> FlectorAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn attitude(self) -> MultiVectorAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn attitude(self) -> MultiVectorAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Origin {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Plane {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn attitude(self) -> LineAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Point {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Rotor {
    type Output = FlectorAtInfinity;

    fn attitude(self) -> FlectorAtInfinity {
        self.anti_wedge(Horizon::one())
    }
}

impl Attitude for Translator {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::one())
    }
}

impl Sqrt for AntiScalar {
    type Output = AntiScalar;

    fn sqrt(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0().sqrt() },
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

impl AntiInverse for AntiScalar {
    type Output = AntiScalar;

    fn anti_inverse(self) -> AntiScalar {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Flector {
    type Output = Flector;

    fn anti_inverse(self) -> Flector {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Line {
    type Output = Line;

    fn anti_inverse(self) -> Line {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_inverse(self) -> LineAtOrigin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Magnitude {
    type Output = Magnitude;

    fn anti_inverse(self) -> Magnitude {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Motor {
    type Output = Motor;

    fn anti_inverse(self) -> Motor {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for MultiVector {
    type Output = MultiVector;

    fn anti_inverse(self) -> MultiVector {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_inverse(self) -> MultiVectorAtOrigin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Origin {
    type Output = Origin;

    fn anti_inverse(self) -> Origin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Plane {
    type Output = Plane;

    fn anti_inverse(self) -> Plane {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_inverse(self) -> PlaneAtOrigin {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Point {
    type Output = Point;

    fn anti_inverse(self) -> Point {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Rotor {
    type Output = Rotor;

    fn anti_inverse(self) -> Rotor {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Translator {
    type Output = Translator;

    fn anti_inverse(self) -> Translator {
        self.geometric_anti_product(AntiScalar::one().div(self.anti_dot(self)))
    }
}

impl Inverse for Flector {
    type Output = Flector;

    fn inverse(self) -> Flector {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn inverse(self) -> FlectorAtInfinity {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Horizon {
    type Output = Horizon;

    fn inverse(self) -> Horizon {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for LineAtInfinity {
    type Output = LineAtInfinity;

    fn inverse(self) -> LineAtInfinity {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Magnitude {
    type Output = Magnitude;

    fn inverse(self) -> Magnitude {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn inverse(self) -> MultiVectorAtInfinity {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Point {
    type Output = Point;

    fn inverse(self) -> Point {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for PointAtInfinity {
    type Output = PointAtInfinity;

    fn inverse(self) -> PointAtInfinity {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}

impl Inverse for Translator {
    type Output = Translator;

    fn inverse(self) -> Translator {
        self.geometric_product(Scalar::one().div(self.dot(self)))
    }
}
