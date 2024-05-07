//
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

impl AntiGrade for AntiScalar {
    type Output = isize;

    fn anti_grade() -> isize {
        0
    }
}

impl AntiGrade for Circle {
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

impl AntiGrade for FlatPoint {
    type Output = isize;

    fn anti_grade() -> isize {
        3
    }
}

impl AntiGrade for Line {
    type Output = isize;

    fn anti_grade() -> isize {
        2
    }
}

impl AntiGrade for Plane {
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

impl Grade for AntiScalar {
    type Output = isize;

    fn grade() -> isize {
        5
    }
}

impl Grade for Circle {
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

impl Grade for FlatPoint {
    type Output = isize;

    fn grade() -> isize {
        2
    }
}

impl Grade for Line {
    type Output = isize;

    fn grade() -> isize {
        3
    }
}

impl Grade for Plane {
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
    type Output = Plane;

    fn attitude(self) -> Plane {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for Circle {
    type Output = Dipole;

    fn attitude(self) -> Dipole {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for Dipole {
    type Output = RoundPoint;

    fn attitude(self) -> RoundPoint {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for DualNum {
    type Output = Plane;

    fn attitude(self) -> Plane {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for FlatPoint {
    type Output = RoundPoint;

    fn attitude(self) -> RoundPoint {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for Flector {
    type Output = MultiVector;

    fn attitude(self) -> MultiVector {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for Line {
    type Output = FlatPoint;

    fn attitude(self) -> FlatPoint {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for Motor {
    type Output = Flector;

    fn attitude(self) -> Flector {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for MultiVector {
    type Output = MultiVector;

    fn attitude(self) -> MultiVector {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for Plane {
    type Output = Line;

    fn attitude(self) -> Line {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for RoundPoint {
    type Output = Scalar;

    fn attitude(self) -> Scalar {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Attitude for Sphere {
    type Output = Circle;

    fn attitude(self) -> Circle {
        self.anti_wedge(Plane {
            groups: PlaneGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            },
        })
    }
}

impl Carrier for Circle {
    type Output = Sphere;

    fn carrier(self) -> Sphere {
        self.wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl Carrier for Dipole {
    type Output = Circle;

    fn carrier(self) -> Circle {
        self.wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl Carrier for DualNum {
    type Output = RoundPoint;

    fn carrier(self) -> RoundPoint {
        self.wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl Carrier for MultiVector {
    type Output = MultiVector;

    fn carrier(self) -> MultiVector {
        self.wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl Carrier for RoundPoint {
    type Output = Dipole;

    fn carrier(self) -> Dipole {
        self.wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl Carrier for Scalar {
    type Output = RoundPoint;

    fn carrier(self) -> RoundPoint {
        self.wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl Carrier for Sphere {
    type Output = AntiScalar;

    fn carrier(self) -> AntiScalar {
        self.wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl CoCarrier for Circle {
    type Output = Circle;

    fn co_carrier(self) -> Circle {
        self.anti_dual().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl CoCarrier for Dipole {
    type Output = Sphere;

    fn co_carrier(self) -> Sphere {
        self.anti_dual().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl CoCarrier for DualNum {
    type Output = RoundPoint;

    fn co_carrier(self) -> RoundPoint {
        self.anti_dual().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl CoCarrier for MultiVector {
    type Output = MultiVector;

    fn co_carrier(self) -> MultiVector {
        self.anti_dual().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl CoCarrier for RoundPoint {
    type Output = AntiScalar;

    fn co_carrier(self) -> AntiScalar {
        self.anti_dual().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
    }
}

impl CoCarrier for Sphere {
    type Output = Dipole;

    fn co_carrier(self) -> Dipole {
        self.anti_dual().wedge(RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from(0.0),
                g1: Simd32x2::from([0.0, 1.0]),
            },
        })
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

impl Center for Circle {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Dipole {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for DualNum {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for MultiVector {
    type Output = MultiVector;

    fn center(self) -> MultiVector {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for RoundPoint {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Center for Sphere {
    type Output = RoundPoint;

    fn center(self) -> RoundPoint {
        self.co_carrier().anti_wedge(self)
    }
}

impl Container for Circle {
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

impl Container for DualNum {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for MultiVector {
    type Output = MultiVector;

    fn container(self) -> MultiVector {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for RoundPoint {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Scalar {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Container for Sphere {
    type Output = Sphere;

    fn container(self) -> Sphere {
        self.wedge(self.carrier().anti_dual())
    }
}

impl Partner for Circle {
    type Output = Circle;

    fn partner(self) -> Circle {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Dipole {
    type Output = Dipole;

    fn partner(self) -> Dipole {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for DualNum {
    type Output = Scalar;

    fn partner(self) -> Scalar {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for MultiVector {
    type Output = MultiVector;

    fn partner(self) -> MultiVector {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for RoundPoint {
    type Output = RoundPoint;

    fn partner(self) -> RoundPoint {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl Partner for Sphere {
    type Output = Sphere;

    fn partner(self) -> Sphere {
        self.dual().container().neg().anti_wedge(self.carrier())
    }
}

impl AntiInverse for AntiScalar {
    type Output = AntiScalar;

    fn anti_inverse(self) -> AntiScalar {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Circle {
    type Output = Circle;

    fn anti_inverse(self) -> Circle {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Dipole {
    type Output = Dipole;

    fn anti_inverse(self) -> Dipole {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
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
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Flector {
    type Output = Flector;

    fn anti_inverse(self) -> Flector {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Line {
    type Output = Line;

    fn anti_inverse(self) -> Line {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Motor {
    type Output = Motor;

    fn anti_inverse(self) -> Motor {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for MultiVector {
    type Output = MultiVector;

    fn anti_inverse(self) -> MultiVector {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Plane {
    type Output = Plane;

    fn anti_inverse(self) -> Plane {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for RoundPoint {
    type Output = RoundPoint;

    fn anti_inverse(self) -> RoundPoint {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Scalar {
    type Output = Scalar;

    fn anti_inverse(self) -> Scalar {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl AntiInverse for Sphere {
    type Output = Sphere;

    fn anti_inverse(self) -> Sphere {
        self.anti_wedge_dot(AntiScalar::unit().div(self.anti_dot(self)))
    }
}

impl Inverse for AntiScalar {
    type Output = AntiScalar;

    fn inverse(self) -> AntiScalar {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Circle {
    type Output = Circle;

    fn inverse(self) -> Circle {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Dipole {
    type Output = Dipole;

    fn inverse(self) -> Dipole {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
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
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Flector {
    type Output = Flector;

    fn inverse(self) -> Flector {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for RoundPoint {
    type Output = RoundPoint;

    fn inverse(self) -> RoundPoint {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
    }
}

impl Inverse for Sphere {
    type Output = Sphere;

    fn inverse(self) -> Sphere {
        self.wedge_dot(Scalar::unit().div(self.dot(self)))
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
