//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::*;
use projective_ga::{simd::*, *};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Negates elements with `grade % 2 == 1`
///
/// Also called main involution
pub trait Automorphism {
    type Output;
    fn automorphism(self) -> Self::Output;
}

/// Negates elements with `(grade + 3) % 4 < 2`
pub trait Conjugation {
    type Output;
    fn conjugation(self) -> Self::Output;
}

/// Negates elements with `grade % 4 >= 2`
///
/// Also called transpose
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Reverses
pub trait Reversal {
    type Output;
    fn reversal(self) -> Self::Output;
}

/// Negates elements with `grade % 4 >= 2`
///
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Reverses
pub trait AntiReversal {
    type Output;
    fn anti_reversal(self) -> Self::Output;
}

/// Element order reversed
/// Also known as Right Complement
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}

/// AntiDuals are a special kind a Dual.
/// https://conformalgeometricalgebra.org/wiki/index.php?title=Duals
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait AntiDual {
    type Output;
    fn anti_dual(self) -> Self::Output;
}

/// Double Complement
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait DoubleComplement {
    type Output;
    fn double_complement(self) -> Self::Output;
}

/// Complement
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait Complement {
    type Output;
    fn complement(self) -> Self::Output;
}

/// Conformal Conjugates
/// See chapter 4.5.4 of the book (page 204).
pub trait ConformalConjugate {
    type Output;
    fn conformal_conjugate(self) -> Self::Output;
}

impl AntiDual for AntiScalar {
    type Output = Scalar;

    fn anti_dual(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for Circle {
    type Output = Dipole;

    fn anti_dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: Simd32x4::from([-self.group2()[0], -self.group2()[1], -self.group2()[2], self.group0()[3]]),
            },
        }
    }
}

impl AntiDual for Dipole {
    type Output = Circle;

    fn anti_dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], -self.group2()[3]]),
                g1: self.group1(),
                g2: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            },
        }
    }
}

impl AntiDual for DualNum {
    type Output = DualNum;

    fn anti_dual(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl AntiDual for FlatPoint {
    type Output = Circle;

    fn anti_dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, -self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl AntiDual for Flector {
    type Output = MultiVector;

    fn anti_dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g2: Simd32x2::from([0.0, -self.group1()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from([0.0, 0.0, 0.0, -self.group0()[3]]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiDual for Line {
    type Output = Dipole;

    fn anti_dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group0() * Simd32x3::from(-1.0),
                g2: Simd32x4::from([-self.group1()[0], -self.group1()[1], -self.group1()[2], 0.0]),
            },
        }
    }
}

impl AntiDual for Motor {
    type Output = MultiVector;

    fn anti_dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: Simd32x4::from([-self.group1()[0], -self.group1()[1], -self.group1()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiDual for MultiVector {
    type Output = MultiVector;

    fn anti_dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from([1.0, -1.0]),
                g1: self.group9(),
                g2: self.group10() * Simd32x2::from(-1.0),
                g3: Simd32x3::from([-self.group6()[0], self.group6()[1], self.group6()[2]]),
                g4: self.group7() * Simd32x3::from(-1.0),
                g5: Simd32x4::from([-self.group8()[0], -self.group8()[1], -self.group8()[2], self.group6()[3]]),
                g6: Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], -self.group5()[3]]),
                g7: self.group4(),
                g8: Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]),
                g9: self.group1() * Simd32x3::from(-1.0),
                g10: self.group2(),
            },
        }
    }
}

impl AntiDual for Plane {
    type Output = RoundPoint;

    fn anti_dual(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x2::from([0.0, -self.group0()[3]]),
            },
        }
    }
}

impl AntiDual for RoundPoint {
    type Output = Sphere;

    fn anti_dual(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl AntiDual for Scalar {
    type Output = AntiScalar;

    fn anti_dual(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: -self.group0() },
        }
    }
}

impl AntiDual for Sphere {
    type Output = RoundPoint;

    fn anti_dual(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl AntiReversal for AntiScalar {
    type Output = AntiScalar;

    fn anti_reversal(self) -> AntiScalar {
        self
    }
}

impl AntiReversal for Circle {
    type Output = Circle;

    fn anti_reversal(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for Dipole {
    type Output = Dipole;

    fn anti_reversal(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DualNum {
    type Output = DualNum;

    fn anti_reversal(self) -> DualNum {
        self
    }
}

impl AntiReversal for FlatPoint {
    type Output = FlatPoint;

    fn anti_reversal(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for Flector {
    type Output = Flector;

    fn anti_reversal(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl AntiReversal for Line {
    type Output = Line;

    fn anti_reversal(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for Motor {
    type Output = Motor;

    fn anti_reversal(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for MultiVector {
    type Output = MultiVector;

    fn anti_reversal(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x4::from(-1.0),
                g6: self.group6() * Simd32x4::from(-1.0),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x3::from(-1.0),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AntiReversal for Plane {
    type Output = Plane;

    fn anti_reversal(self) -> Plane {
        self
    }
}

impl AntiReversal for RoundPoint {
    type Output = RoundPoint;

    fn anti_reversal(self) -> RoundPoint {
        self
    }
}

impl AntiReversal for Scalar {
    type Output = Scalar;

    fn anti_reversal(self) -> Scalar {
        self
    }
}

impl AntiReversal for Sphere {
    type Output = Sphere;

    fn anti_reversal(self) -> Sphere {
        self
    }
}

impl Automorphism for AntiScalar {
    type Output = AntiScalar;

    fn automorphism(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: -self.group0() },
        }
    }
}

impl Automorphism for Circle {
    type Output = Circle;

    fn automorphism(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for Dipole {
    type Output = Dipole;

    fn automorphism(self) -> Dipole {
        self
    }
}

impl Automorphism for DualNum {
    type Output = DualNum;

    fn automorphism(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl Automorphism for FlatPoint {
    type Output = FlatPoint;

    fn automorphism(self) -> FlatPoint {
        self
    }
}

impl Automorphism for Flector {
    type Output = Flector;

    fn automorphism(self) -> Flector {
        self
    }
}

impl Automorphism for Line {
    type Output = Line;

    fn automorphism(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x2::from(-1.0),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6() * Simd32x4::from(-1.0),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x3::from(-1.0),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl Automorphism for Plane {
    type Output = Plane;

    fn automorphism(self) -> Plane {
        self
    }
}

impl Automorphism for RoundPoint {
    type Output = RoundPoint;

    fn automorphism(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Automorphism for Scalar {
    type Output = Scalar;

    fn automorphism(self) -> Scalar {
        self
    }
}

impl Automorphism for Sphere {
    type Output = Sphere;

    fn automorphism(self) -> Sphere {
        self
    }
}

impl Complement for AntiScalar {
    type Output = Scalar;

    fn complement(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl Complement for Circle {
    type Output = Dipole;

    fn complement(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group2() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Complement for Dipole {
    type Output = Circle;

    fn complement(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group2() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for DualNum {
    type Output = DualNum;

    fn complement(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: swizzle!(self.group0(), 1, 0),
            },
        }
    }
}

impl Complement for FlatPoint {
    type Output = Circle;

    fn complement(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl Complement for Flector {
    type Output = MultiVector;

    fn complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
                g2: Simd32x2::from([self.group1()[3], 0.0]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: self.group0() * Simd32x4::from(-1.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Complement for Line {
    type Output = Dipole;

    fn complement(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
                g2: Simd32x4::from(0.0),
            },
        }
    }
}

impl Complement for Motor {
    type Output = MultiVector;

    fn complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group1() * Simd32x3::from(-1.0),
                g4: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Complement for MultiVector {
    type Output = MultiVector;

    fn complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: swizzle!(self.group0(), 1, 0),
                g1: self.group9(),
                g2: swizzle!(self.group10(), 1, 0),
                g3: self.group8() * Simd32x3::from(-1.0),
                g4: self.group7() * Simd32x3::from(-1.0),
                g5: self.group6() * Simd32x4::from(-1.0),
                g6: self.group5() * Simd32x4::from(-1.0),
                g7: self.group4() * Simd32x3::from(-1.0),
                g8: self.group3() * Simd32x3::from(-1.0),
                g9: self.group1(),
                g10: swizzle!(self.group2(), 1, 0),
            },
        }
    }
}

impl Complement for Plane {
    type Output = RoundPoint;

    fn complement(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x2::from([self.group0()[3], 0.0]),
            },
        }
    }
}

impl Complement for RoundPoint {
    type Output = Sphere;

    fn complement(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: swizzle!(self.group1(), 1, 0),
            },
        }
    }
}

impl Complement for Scalar {
    type Output = AntiScalar;

    fn complement(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}

impl Complement for Sphere {
    type Output = RoundPoint;

    fn complement(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: swizzle!(self.group1(), 1, 0),
            },
        }
    }
}

impl ConformalConjugate for AntiScalar {
    type Output = AntiScalar;

    fn conformal_conjugate(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: -self.group0() },
        }
    }
}

impl ConformalConjugate for Circle {
    type Output = Circle;

    fn conformal_conjugate(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for Dipole {
    type Output = Dipole;

    fn conformal_conjugate(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for DualNum {
    type Output = DualNum;

    fn conformal_conjugate(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for FlatPoint {
    type Output = FlatPoint;

    fn conformal_conjugate(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for Flector {
    type Output = Flector;

    fn conformal_conjugate(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for Line {
    type Output = Line;

    fn conformal_conjugate(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for Motor {
    type Output = Motor;

    fn conformal_conjugate(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for MultiVector {
    type Output = MultiVector;

    fn conformal_conjugate(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
                g1: self.group1(),
                g2: self.group2() * Simd32x2::from([1.0, -1.0]),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5() * Simd32x4::from(-1.0),
                g6: self.group6(),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x3::from(-1.0),
                g9: self.group9() * Simd32x3::from(-1.0),
                g10: self.group10() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for Plane {
    type Output = Plane;

    fn conformal_conjugate(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for RoundPoint {
    type Output = RoundPoint;

    fn conformal_conjugate(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for Scalar {
    type Output = Scalar;

    fn conformal_conjugate(self) -> Scalar {
        self
    }
}

impl ConformalConjugate for Sphere {
    type Output = Sphere;

    fn conformal_conjugate(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl Conjugation for AntiScalar {
    type Output = AntiScalar;

    fn conjugation(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: -self.group0() },
        }
    }
}

impl Conjugation for Circle {
    type Output = Circle;

    fn conjugation(self) -> Circle {
        self
    }
}

impl Conjugation for Dipole {
    type Output = Dipole;

    fn conjugation(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for DualNum {
    type Output = DualNum;

    fn conjugation(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl Conjugation for FlatPoint {
    type Output = FlatPoint;

    fn conjugation(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for Flector {
    type Output = Flector;

    fn conjugation(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl Conjugation for Line {
    type Output = Line;

    fn conjugation(self) -> Line {
        self
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
                g1: self.group1(),
            },
        }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x2::from(-1.0),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x4::from(-1.0),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl Conjugation for Plane {
    type Output = Plane;

    fn conjugation(self) -> Plane {
        self
    }
}

impl Conjugation for RoundPoint {
    type Output = RoundPoint;

    fn conjugation(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Conjugation for Scalar {
    type Output = Scalar;

    fn conjugation(self) -> Scalar {
        self
    }
}

impl Conjugation for Sphere {
    type Output = Sphere;

    fn conjugation(self) -> Sphere {
        self
    }
}

impl DoubleComplement for AntiScalar {
    type Output = AntiScalar;

    fn double_complement(self) -> AntiScalar {
        self
    }
}

impl DoubleComplement for Circle {
    type Output = Circle;

    fn double_complement(self) -> Circle {
        self
    }
}

impl DoubleComplement for Dipole {
    type Output = Dipole;

    fn double_complement(self) -> Dipole {
        self
    }
}

impl DoubleComplement for DualNum {
    type Output = DualNum;

    fn double_complement(self) -> DualNum {
        self
    }
}

impl DoubleComplement for FlatPoint {
    type Output = FlatPoint;

    fn double_complement(self) -> FlatPoint {
        self
    }
}

impl DoubleComplement for Flector {
    type Output = Flector;

    fn double_complement(self) -> Flector {
        self
    }
}

impl DoubleComplement for Line {
    type Output = Line;

    fn double_complement(self) -> Line {
        self
    }
}

impl DoubleComplement for Motor {
    type Output = Motor;

    fn double_complement(self) -> Motor {
        self
    }
}

impl DoubleComplement for MultiVector {
    type Output = MultiVector;

    fn double_complement(self) -> MultiVector {
        self
    }
}

impl DoubleComplement for Plane {
    type Output = Plane;

    fn double_complement(self) -> Plane {
        self
    }
}

impl DoubleComplement for RoundPoint {
    type Output = RoundPoint;

    fn double_complement(self) -> RoundPoint {
        self
    }
}

impl DoubleComplement for Scalar {
    type Output = Scalar;

    fn double_complement(self) -> Scalar {
        self
    }
}

impl DoubleComplement for Sphere {
    type Output = Sphere;

    fn double_complement(self) -> Sphere {
        self
    }
}

impl Dual for AntiScalar {
    type Output = Scalar;

    fn dual(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: -self.group0() },
        }
    }
}

impl Dual for Circle {
    type Output = Dipole;

    fn dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: self.group1(),
                g2: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], -self.group0()[3]]),
            },
        }
    }
}

impl Dual for Dipole {
    type Output = Circle;

    fn dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], self.group2()[3]]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: Simd32x3::from([-self.group2()[0], self.group2()[1], self.group2()[2]]),
            },
        }
    }
}

impl Dual for DualNum {
    type Output = DualNum;

    fn dual(self) -> DualNum {
        DualNum {
            groups: DualNumGroups {
                g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]),
            },
        }
    }
}

impl Dual for FlatPoint {
    type Output = Circle;

    fn dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
            },
        }
    }
}

impl Dual for Flector {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from([-self.group1()[0], self.group1()[1], self.group1()[2]]),
                g2: Simd32x2::from([0.0, self.group1()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Dual for Line {
    type Output = Dipole;

    fn dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group0(),
                g2: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            },
        }
    }
}

impl Dual for Motor {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([-self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Dual for MultiVector {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]),
                g1: self.group9() * Simd32x3::from(-1.0),
                g2: self.group10(),
                g3: Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]),
                g4: self.group7(),
                g5: Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], -self.group6()[3]]),
                g6: Simd32x4::from([-self.group3()[0], -self.group3()[1], -self.group3()[2], self.group5()[3]]),
                g7: self.group4() * Simd32x3::from(-1.0),
                g8: Simd32x3::from([-self.group5()[0], self.group5()[1], self.group5()[2]]),
                g9: self.group1(),
                g10: self.group2() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Dual for Plane {
    type Output = RoundPoint;

    fn dual(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x2::from([0.0, self.group0()[3]]),
            },
        }
    }
}

impl Dual for RoundPoint {
    type Output = Sphere;

    fn dual(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Dual for Scalar {
    type Output = AntiScalar;

    fn dual(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}

impl Dual for Sphere {
    type Output = RoundPoint;

    fn dual(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl Reversal for AntiScalar {
    type Output = AntiScalar;

    fn reversal(self) -> AntiScalar {
        self
    }
}

impl Reversal for Circle {
    type Output = Circle;

    fn reversal(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for Dipole {
    type Output = Dipole;

    fn reversal(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for DualNum {
    type Output = DualNum;

    fn reversal(self) -> DualNum {
        self
    }
}

impl Reversal for FlatPoint {
    type Output = FlatPoint;

    fn reversal(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for Flector {
    type Output = Flector;

    fn reversal(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl Reversal for Line {
    type Output = Line;

    fn reversal(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for MultiVector {
    type Output = MultiVector;

    fn reversal(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x4::from(-1.0),
                g6: self.group6() * Simd32x4::from(-1.0),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x3::from(-1.0),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        self
    }
}

impl Reversal for RoundPoint {
    type Output = RoundPoint;

    fn reversal(self) -> RoundPoint {
        self
    }
}

impl Reversal for Scalar {
    type Output = Scalar;

    fn reversal(self) -> Scalar {
        self
    }
}

impl Reversal for Sphere {
    type Output = Sphere;

    fn reversal(self) -> Sphere {
        self
    }
}
