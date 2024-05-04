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

impl Attitude for AntiScalar {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for DualNum {
    type Output = Horizon;

    fn attitude(self) -> Horizon {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Flector {
    type Output = MultiVectorAtInfinity;

    fn attitude(self) -> MultiVectorAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Line {
    type Output = PointAtInfinity;

    fn attitude(self) -> PointAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for LineAtOrigin {
    type Output = PointAtInfinity;

    fn attitude(self) -> PointAtInfinity {
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
    type Output = MultiVectorAtInfinity;

    fn attitude(self) -> MultiVectorAtInfinity {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn attitude(self) -> MultiVectorAtInfinity {
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

impl Attitude for Point {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Horizon::unit())
    }
}

impl Attitude for Rotor {
    type Output = FlectorAtInfinity;

    fn attitude(self) -> FlectorAtInfinity {
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

impl AntiInverse for AntiScalar {
    type Output = AntiScalar;

    fn anti_inverse(self) -> AntiScalar {
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

impl AntiInverse for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn anti_inverse(self) -> MultiVectorAtOrigin {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Origin {
    type Output = Origin;

    fn anti_inverse(self) -> Origin {
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

impl AntiInverse for Point {
    type Output = Point;

    fn anti_inverse(self) -> Point {
        self.geometric_anti_product(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Rotor {
    type Output = Rotor;

    fn anti_inverse(self) -> Rotor {
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

impl Inverse for Flector {
    type Output = Flector;

    fn inverse(self) -> Flector {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn inverse(self) -> FlectorAtInfinity {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Horizon {
    type Output = Horizon;

    fn inverse(self) -> Horizon {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for LineAtInfinity {
    type Output = LineAtInfinity;

    fn inverse(self) -> LineAtInfinity {
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

impl Inverse for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn inverse(self) -> MultiVectorAtInfinity {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Point {
    type Output = Point;

    fn inverse(self) -> Point {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for PointAtInfinity {
    type Output = PointAtInfinity;

    fn inverse(self) -> PointAtInfinity {
        self.geometric_product(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
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
