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

impl AntiDual for CircleAtInfinity {
    type Output = FlatPoint;

    fn anti_dual(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: swizzle!(self.group0(), 1, 2, 3, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for CircleBulk {
    type Output = FlatPointAtOrigin;

    fn anti_dual(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for CircleCarrierAspect {
    type Output = Dipole;

    fn anti_dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            },
        }
    }
}

impl AntiDual for CircleWeight {
    type Output = DipoleWeight;

    fn anti_dual(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl AntiDual for DipoleAtInfinity {
    type Output = Line;

    fn anti_dual(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AntiDual for DipoleBulk {
    type Output = LineAtOrigin;

    fn anti_dual(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for DipoleCarrierAspect {
    type Output = Circle;

    fn anti_dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
                g1: self.group1(),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl AntiDual for DipoleWeight {
    type Output = CircleWeight;

    fn anti_dual(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups { g0: self.group0() },
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
    type Output = CircleAtInfinity;

    fn anti_dual(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: swizzle!(self.group0(), 3, 0, 1, 2) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_dual(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for FlatPointAtOrigin {
    type Output = CircleBulk;

    fn anti_dual(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: -self.group0() },
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

impl AntiDual for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from([0.0, -self.group0()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiDual for Horizon {
    type Output = Infinity;

    fn anti_dual(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: -self.group0() },
        }
    }
}

impl AntiDual for Infinity {
    type Output = Horizon;

    fn anti_dual(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for Line {
    type Output = DipoleAtInfinity;

    fn anti_dual(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_dual(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for LineAtOrigin {
    type Output = DipoleBulk;

    fn anti_dual(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl AntiDual for Origin {
    type Output = SphereWeight;

    fn anti_dual(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for Plane {
    type Output = RoundPointAtInfinity;

    fn anti_dual(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiDual for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn anti_dual(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for Rotor {
    type Output = MultiVector;

    fn anti_dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
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

impl AntiDual for RoundPointAtInfinity {
    type Output = Plane;

    fn anti_dual(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for RoundPointAtOrigin {
    type Output = SpacialCurvature;

    fn anti_dual(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn anti_dual(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for RoundPointOnOrigin {
    type Output = Sphere;

    fn anti_dual(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x2::from([self.group0()[3], 0.0]),
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

impl AntiDual for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn anti_dual(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
            },
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

impl AntiDual for SphereWeight {
    type Output = Origin;

    fn anti_dual(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl AntiDual for Transflector {
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
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0(),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiDual for Translator {
    type Output = MultiVector;

    fn anti_dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiReversal for AntiScalar {
    type Output = AntiScalar;

    fn anti_reversal(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
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

impl AntiReversal for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn anti_reversal(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for CircleBulk {
    type Output = CircleBulk;

    fn anti_reversal(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: -self.group0() },
        }
    }
}

impl AntiReversal for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn anti_reversal(self) -> CircleCarrierAspect {
        CircleCarrierAspect {
            groups: CircleCarrierAspectGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for CircleWeight {
    type Output = CircleWeight;

    fn anti_reversal(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl AntiReversal for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn anti_reversal(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DipoleBulk {
    type Output = DipoleBulk;

    fn anti_reversal(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn anti_reversal(self) -> DipoleCarrierAspect {
        DipoleCarrierAspect {
            groups: DipoleCarrierAspectGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DipoleWeight {
    type Output = DipoleWeight;

    fn anti_reversal(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DualNum {
    type Output = DualNum;

    fn anti_reversal(self) -> DualNum {
        DualNum {
            groups: DualNumGroups { g0: self.group0() },
        }
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

impl AntiReversal for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn anti_reversal(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_reversal(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
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

impl AntiReversal for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn anti_reversal(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiReversal for Horizon {
    type Output = Horizon;

    fn anti_reversal(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Infinity {
    type Output = Infinity;

    fn anti_reversal(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0() },
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

impl AntiReversal for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reversal(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_reversal(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl AntiReversal for Origin {
    type Output = Origin;

    fn anti_reversal(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Plane {
    type Output = Plane;

    fn anti_reversal(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_reversal(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Rotor {
    type Output = Rotor;

    fn anti_reversal(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiReversal for RoundPoint {
    type Output = RoundPoint;

    fn anti_reversal(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AntiReversal for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn anti_reversal(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reversal(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for RoundPointBulk {
    type Output = RoundPointBulk;

    fn anti_reversal(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn anti_reversal(self) -> RoundPointOnOrigin {
        RoundPointOnOrigin {
            groups: RoundPointOnOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Scalar {
    type Output = Scalar;

    fn anti_reversal(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for SpacialCurvature {
    type Output = SpacialCurvature;

    fn anti_reversal(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Sphere {
    type Output = Sphere;

    fn anti_reversal(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AntiReversal for SphereWeight {
    type Output = SphereWeight;

    fn anti_reversal(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Transflector {
    type Output = Transflector;

    fn anti_reversal(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl AntiReversal for Translator {
    type Output = Translator;

    fn anti_reversal(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
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

impl Automorphism for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn automorphism(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for CircleBulk {
    type Output = CircleBulk;

    fn automorphism(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: -self.group0() },
        }
    }
}

impl Automorphism for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn automorphism(self) -> CircleCarrierAspect {
        CircleCarrierAspect {
            groups: CircleCarrierAspectGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for CircleWeight {
    type Output = CircleWeight;

    fn automorphism(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for Dipole {
    type Output = Dipole;

    fn automorphism(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
            },
        }
    }
}

impl Automorphism for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn automorphism(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Automorphism for DipoleBulk {
    type Output = DipoleBulk;

    fn automorphism(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn automorphism(self) -> DipoleCarrierAspect {
        DipoleCarrierAspect {
            groups: DipoleCarrierAspectGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Automorphism for DipoleWeight {
    type Output = DipoleWeight;

    fn automorphism(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups { g0: self.group0() },
        }
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
        FlatPoint {
            groups: FlatPointGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn automorphism(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn automorphism(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Flector {
    type Output = Flector;

    fn automorphism(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Automorphism for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn automorphism(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Horizon {
    type Output = Horizon;

    fn automorphism(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Infinity {
    type Output = Infinity;

    fn automorphism(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: -self.group0() },
        }
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

impl Automorphism for LineAtInfinity {
    type Output = LineAtInfinity;

    fn automorphism(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for LineAtOrigin {
    type Output = LineAtOrigin;

    fn automorphism(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Automorphism for Origin {
    type Output = Origin;

    fn automorphism(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl Automorphism for Plane {
    type Output = Plane;

    fn automorphism(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn automorphism(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Rotor {
    type Output = Rotor;

    fn automorphism(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
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

impl Automorphism for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn automorphism(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn automorphism(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Automorphism for RoundPointBulk {
    type Output = RoundPointBulk;

    fn automorphism(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn automorphism(self) -> RoundPointOnOrigin {
        RoundPointOnOrigin {
            groups: RoundPointOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for Scalar {
    type Output = Scalar;

    fn automorphism(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for SpacialCurvature {
    type Output = SpacialCurvature;

    fn automorphism(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Sphere {
    type Output = Sphere;

    fn automorphism(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Automorphism for SphereWeight {
    type Output = SphereWeight;

    fn automorphism(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Transflector {
    type Output = Transflector;

    fn automorphism(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Automorphism for Translator {
    type Output = Translator;

    fn automorphism(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
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

impl Complement for CircleAtInfinity {
    type Output = Dipole;

    fn complement(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([-self.group0()[1], self.group0()[2], self.group0()[3]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from([0.0, 0.0, 0.0, -self.group0()[0]]),
            },
        }
    }
}

impl Complement for CircleBulk {
    type Output = FlatPointAtOrigin;

    fn complement(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Complement for CircleCarrierAspect {
    type Output = FlatPoint;

    fn complement(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Complement for CircleWeight {
    type Output = FlatPointAtInfinity;

    fn complement(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Complement for DipoleAtInfinity {
    type Output = Circle;

    fn complement(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([-self.group1()[0], -self.group1()[1], -self.group1()[2], 0.0]),
                g1: self.group0() * Simd32x3::from(-1.0),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl Complement for DipoleBulk {
    type Output = LineAtOrigin;

    fn complement(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for DipoleCarrierAspect {
    type Output = Line;

    fn complement(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for DipoleWeight {
    type Output = LineAtInfinity;

    fn complement(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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
    type Output = CircleCarrierAspect;

    fn complement(self) -> CircleCarrierAspect {
        CircleCarrierAspect {
            groups: CircleCarrierAspectGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Complement for FlatPointAtInfinity {
    type Output = CircleWeight;

    fn complement(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for FlatPointAtOrigin {
    type Output = CircleBulk;

    fn complement(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: -self.group0() },
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

impl Complement for FlectorAtInfinity {
    type Output = MultiVector;

    fn complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from([self.group0()[3], 0.0]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Complement for Horizon {
    type Output = Origin;

    fn complement(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
        }
    }
}

impl Complement for Infinity {
    type Output = SphereWeight;

    fn complement(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl Complement for Line {
    type Output = DipoleCarrierAspect;

    fn complement(self) -> DipoleCarrierAspect {
        DipoleCarrierAspect {
            groups: DipoleCarrierAspectGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for LineAtInfinity {
    type Output = DipoleWeight;

    fn complement(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for LineAtOrigin {
    type Output = DipoleBulk;

    fn complement(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Complement for Origin {
    type Output = Horizon;

    fn complement(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl Complement for Plane {
    type Output = RoundPointOnOrigin;

    fn complement(self) -> RoundPointOnOrigin {
        RoundPointOnOrigin {
            groups: RoundPointOnOriginGroups { g0: self.group0() },
        }
    }
}

impl Complement for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn complement(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups { g0: self.group0() },
        }
    }
}

impl Complement for Rotor {
    type Output = MultiVector;

    fn complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
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

impl Complement for RoundPointAtInfinity {
    type Output = Sphere;

    fn complement(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x2::from([self.group0()[3], 0.0]),
            },
        }
    }
}

impl Complement for RoundPointAtOrigin {
    type Output = SpacialCurvature;

    fn complement(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups {
                g0: swizzle!(self.group0(), 1, 0),
            },
        }
    }
}

impl Complement for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn complement(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Complement for RoundPointOnOrigin {
    type Output = Plane;

    fn complement(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
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

impl Complement for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn complement(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups {
                g0: swizzle!(self.group0(), 1, 0),
            },
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

impl Complement for SphereWeight {
    type Output = Infinity;

    fn complement(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0() },
        }
    }
}

impl Complement for Transflector {
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
                g6: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Complement for Translator {
    type Output = MultiVector;

    fn complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g4: Simd32x3::from(0.0),
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

impl ConformalConjugate for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn conformal_conjugate(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for CircleBulk {
    type Output = CircleBulk;

    fn conformal_conjugate(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: self.group0() },
        }
    }
}

impl ConformalConjugate for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn conformal_conjugate(self) -> CircleCarrierAspect {
        CircleCarrierAspect {
            groups: CircleCarrierAspectGroups { g0: self.group0() },
        }
    }
}

impl ConformalConjugate for CircleWeight {
    type Output = CircleWeight;

    fn conformal_conjugate(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups { g0: self.group0() },
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

impl ConformalConjugate for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn conformal_conjugate(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for DipoleBulk {
    type Output = DipoleBulk;

    fn conformal_conjugate(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups { g0: self.group0() },
        }
    }
}

impl ConformalConjugate for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn conformal_conjugate(self) -> DipoleCarrierAspect {
        DipoleCarrierAspect {
            groups: DipoleCarrierAspectGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl ConformalConjugate for DipoleWeight {
    type Output = DipoleWeight;

    fn conformal_conjugate(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups { g0: self.group0() },
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

impl ConformalConjugate for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn conformal_conjugate(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn conformal_conjugate(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
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

impl ConformalConjugate for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn conformal_conjugate(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for Horizon {
    type Output = Horizon;

    fn conformal_conjugate(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: -self.group0() },
        }
    }
}

impl ConformalConjugate for Infinity {
    type Output = Infinity;

    fn conformal_conjugate(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: -self.group0() },
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

impl ConformalConjugate for LineAtInfinity {
    type Output = LineAtInfinity;

    fn conformal_conjugate(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for LineAtOrigin {
    type Output = LineAtOrigin;

    fn conformal_conjugate(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl ConformalConjugate for Origin {
    type Output = Origin;

    fn conformal_conjugate(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
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

impl ConformalConjugate for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn conformal_conjugate(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for Rotor {
    type Output = Rotor;

    fn conformal_conjugate(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
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

impl ConformalConjugate for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn conformal_conjugate(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn conformal_conjugate(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for RoundPointBulk {
    type Output = RoundPointBulk;

    fn conformal_conjugate(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups { g0: self.group0() },
        }
    }
}

impl ConformalConjugate for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn conformal_conjugate(self) -> RoundPointOnOrigin {
        RoundPointOnOrigin {
            groups: RoundPointOnOriginGroups { g0: self.group0() },
        }
    }
}

impl ConformalConjugate for Scalar {
    type Output = Scalar;

    fn conformal_conjugate(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl ConformalConjugate for SpacialCurvature {
    type Output = SpacialCurvature;

    fn conformal_conjugate(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
            },
        }
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

impl ConformalConjugate for SphereWeight {
    type Output = SphereWeight;

    fn conformal_conjugate(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl ConformalConjugate for Transflector {
    type Output = Transflector;

    fn conformal_conjugate(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for Translator {
    type Output = Translator;

    fn conformal_conjugate(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
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
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
            },
        }
    }
}

impl Conjugation for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn conjugation(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for CircleBulk {
    type Output = CircleBulk;

    fn conjugation(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn conjugation(self) -> CircleCarrierAspect {
        CircleCarrierAspect {
            groups: CircleCarrierAspectGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for CircleWeight {
    type Output = CircleWeight;

    fn conjugation(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups { g0: self.group0() },
        }
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

impl Conjugation for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn conjugation(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for DipoleBulk {
    type Output = DipoleBulk;

    fn conjugation(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn conjugation(self) -> DipoleCarrierAspect {
        DipoleCarrierAspect {
            groups: DipoleCarrierAspectGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for DipoleWeight {
    type Output = DipoleWeight;

    fn conjugation(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Conjugation for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn conjugation(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn conjugation(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
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

impl Conjugation for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn conjugation(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Conjugation for Horizon {
    type Output = Horizon;

    fn conjugation(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for Infinity {
    type Output = Infinity;

    fn conjugation(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: -self.group0() },
        }
    }
}

impl Conjugation for Line {
    type Output = Line;

    fn conjugation(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Conjugation for LineAtInfinity {
    type Output = LineAtInfinity;

    fn conjugation(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for LineAtOrigin {
    type Output = LineAtOrigin;

    fn conjugation(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
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

impl Conjugation for Origin {
    type Output = Origin;

    fn conjugation(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl Conjugation for Plane {
    type Output = Plane;

    fn conjugation(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn conjugation(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for Rotor {
    type Output = Rotor;

    fn conjugation(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
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

impl Conjugation for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn conjugation(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn conjugation(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Conjugation for RoundPointBulk {
    type Output = RoundPointBulk;

    fn conjugation(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn conjugation(self) -> RoundPointOnOrigin {
        RoundPointOnOrigin {
            groups: RoundPointOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for Scalar {
    type Output = Scalar;

    fn conjugation(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for SpacialCurvature {
    type Output = SpacialCurvature;

    fn conjugation(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for Sphere {
    type Output = Sphere;

    fn conjugation(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Conjugation for SphereWeight {
    type Output = SphereWeight;

    fn conjugation(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for Transflector {
    type Output = Transflector;

    fn conjugation(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl Conjugation for Translator {
    type Output = Translator;

    fn conjugation(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl DoubleComplement for AntiScalar {
    type Output = AntiScalar;

    fn double_complement(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Circle {
    type Output = Circle;

    fn double_complement(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
            },
        }
    }
}

impl DoubleComplement for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn double_complement(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for CircleBulk {
    type Output = CircleBulk;

    fn double_complement(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn double_complement(self) -> CircleCarrierAspect {
        CircleCarrierAspect {
            groups: CircleCarrierAspectGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for CircleWeight {
    type Output = CircleWeight;

    fn double_complement(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Dipole {
    type Output = Dipole;

    fn double_complement(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
            },
        }
    }
}

impl DoubleComplement for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn double_complement(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for DipoleBulk {
    type Output = DipoleBulk;

    fn double_complement(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn double_complement(self) -> DipoleCarrierAspect {
        DipoleCarrierAspect {
            groups: DipoleCarrierAspectGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for DipoleWeight {
    type Output = DipoleWeight;

    fn double_complement(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for DualNum {
    type Output = DualNum;

    fn double_complement(self) -> DualNum {
        DualNum {
            groups: DualNumGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for FlatPoint {
    type Output = FlatPoint;

    fn double_complement(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn double_complement(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn double_complement(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Flector {
    type Output = Flector;

    fn double_complement(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn double_complement(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Horizon {
    type Output = Horizon;

    fn double_complement(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Infinity {
    type Output = Infinity;

    fn double_complement(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Line {
    type Output = Line;

    fn double_complement(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for LineAtInfinity {
    type Output = LineAtInfinity;

    fn double_complement(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for LineAtOrigin {
    type Output = LineAtOrigin;

    fn double_complement(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Motor {
    type Output = Motor;

    fn double_complement(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for MultiVector {
    type Output = MultiVector;

    fn double_complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4(),
                g5: self.group5(),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl DoubleComplement for Origin {
    type Output = Origin;

    fn double_complement(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Plane {
    type Output = Plane;

    fn double_complement(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn double_complement(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Rotor {
    type Output = Rotor;

    fn double_complement(self) -> Rotor {
        Rotor {
            groups: RotorGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for RoundPoint {
    type Output = RoundPoint;

    fn double_complement(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn double_complement(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn double_complement(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for RoundPointBulk {
    type Output = RoundPointBulk;

    fn double_complement(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn double_complement(self) -> RoundPointOnOrigin {
        RoundPointOnOrigin {
            groups: RoundPointOnOriginGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Scalar {
    type Output = Scalar;

    fn double_complement(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for SpacialCurvature {
    type Output = SpacialCurvature;

    fn double_complement(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Sphere {
    type Output = Sphere;

    fn double_complement(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for SphereWeight {
    type Output = SphereWeight;

    fn double_complement(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl DoubleComplement for Transflector {
    type Output = Transflector;

    fn double_complement(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl DoubleComplement for Translator {
    type Output = Translator;

    fn double_complement(self) -> Translator {
        Translator {
            groups: TranslatorGroups { g0: self.group0() },
        }
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

impl Dual for CircleAtInfinity {
    type Output = FlatPoint;

    fn dual(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: swizzle!(self.group0(), 1, 2, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for CircleBulk {
    type Output = FlatPointAtOrigin;

    fn dual(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Dual for CircleCarrierAspect {
    type Output = Dipole;

    fn dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x4::from([0.0, 0.0, 0.0, -self.group0()[3]]),
            },
        }
    }
}

impl Dual for CircleWeight {
    type Output = DipoleWeight;

    fn dual(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups { g0: self.group0() },
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

impl Dual for DipoleAtInfinity {
    type Output = Line;

    fn dual(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for DipoleBulk {
    type Output = LineAtOrigin;

    fn dual(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for DipoleCarrierAspect {
    type Output = Circle;

    fn dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: Simd32x3::from(0.0),
            },
        }
    }
}

impl Dual for DipoleWeight {
    type Output = CircleWeight;

    fn dual(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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
    type Output = CircleAtInfinity;

    fn dual(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: swizzle!(self.group0(), 3, 0, 1, 2) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            },
        }
    }
}

impl Dual for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn dual(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for FlatPointAtOrigin {
    type Output = CircleBulk;

    fn dual(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: self.group0() },
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

impl Dual for FlectorAtInfinity {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from(0.0),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from([0.0, self.group0()[3]]),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from(0.0),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Dual for Horizon {
    type Output = Infinity;

    fn dual(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0() },
        }
    }
}

impl Dual for Infinity {
    type Output = Horizon;

    fn dual(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: -self.group0() },
        }
    }
}

impl Dual for Line {
    type Output = DipoleAtInfinity;

    fn dual(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Dual for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn dual(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Dual for LineAtOrigin {
    type Output = DipoleBulk;

    fn dual(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups { g0: self.group0() },
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

impl Dual for Origin {
    type Output = SphereWeight;

    fn dual(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: -self.group0() },
        }
    }
}

impl Dual for Plane {
    type Output = RoundPointAtInfinity;

    fn dual(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Dual for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn dual(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for Rotor {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([-self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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

impl Dual for RoundPointAtInfinity {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for RoundPointAtOrigin {
    type Output = SpacialCurvature;

    fn dual(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Dual for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn dual(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Dual for RoundPointOnOrigin {
    type Output = Sphere;

    fn dual(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g1: Simd32x2::from([-self.group0()[3], 0.0]),
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

impl Dual for SpacialCurvature {
    type Output = RoundPointAtOrigin;

    fn dual(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups { g0: self.group0() },
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

impl Dual for SphereWeight {
    type Output = Origin;

    fn dual(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
        }
    }
}

impl Dual for Transflector {
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
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x3::from(-1.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Dual for Translator {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([-self.group0()[3], 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x3::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
                g6: Simd32x4::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x3::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Reversal for AntiScalar {
    type Output = AntiScalar;

    fn reversal(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
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

impl Reversal for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn reversal(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for CircleBulk {
    type Output = CircleBulk;

    fn reversal(self) -> CircleBulk {
        CircleBulk {
            groups: CircleBulkGroups { g0: -self.group0() },
        }
    }
}

impl Reversal for CircleCarrierAspect {
    type Output = CircleCarrierAspect;

    fn reversal(self) -> CircleCarrierAspect {
        CircleCarrierAspect {
            groups: CircleCarrierAspectGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for CircleWeight {
    type Output = CircleWeight;

    fn reversal(self) -> CircleWeight {
        CircleWeight {
            groups: CircleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Reversal for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn reversal(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for DipoleBulk {
    type Output = DipoleBulk;

    fn reversal(self) -> DipoleBulk {
        DipoleBulk {
            groups: DipoleBulkGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for DipoleCarrierAspect {
    type Output = DipoleCarrierAspect;

    fn reversal(self) -> DipoleCarrierAspect {
        DipoleCarrierAspect {
            groups: DipoleCarrierAspectGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for DipoleWeight {
    type Output = DipoleWeight;

    fn reversal(self) -> DipoleWeight {
        DipoleWeight {
            groups: DipoleWeightGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for DualNum {
    type Output = DualNum;

    fn reversal(self) -> DualNum {
        DualNum {
            groups: DualNumGroups { g0: self.group0() },
        }
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

impl Reversal for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn reversal(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn reversal(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
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

impl Reversal for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn reversal(self) -> FlectorAtInfinity {
        FlectorAtInfinity {
            groups: FlectorAtInfinityGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Reversal for Horizon {
    type Output = Horizon;

    fn reversal(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Infinity {
    type Output = Infinity;

    fn reversal(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0() },
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

impl Reversal for LineAtInfinity {
    type Output = LineAtInfinity;

    fn reversal(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for LineAtOrigin {
    type Output = LineAtOrigin;

    fn reversal(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Reversal for Origin {
    type Output = Origin;

    fn reversal(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl Reversal for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reversal(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Rotor {
    type Output = Rotor;

    fn reversal(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Reversal for RoundPoint {
    type Output = RoundPoint;

    fn reversal(self) -> RoundPoint {
        RoundPoint {
            groups: RoundPointGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Reversal for RoundPointAtInfinity {
    type Output = RoundPointAtInfinity;

    fn reversal(self) -> RoundPointAtInfinity {
        RoundPointAtInfinity {
            groups: RoundPointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Reversal for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn reversal(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Reversal for RoundPointBulk {
    type Output = RoundPointBulk;

    fn reversal(self) -> RoundPointBulk {
        RoundPointBulk {
            groups: RoundPointBulkGroups { g0: self.group0() },
        }
    }
}

impl Reversal for RoundPointOnOrigin {
    type Output = RoundPointOnOrigin;

    fn reversal(self) -> RoundPointOnOrigin {
        RoundPointOnOrigin {
            groups: RoundPointOnOriginGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Scalar {
    type Output = Scalar;

    fn reversal(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl Reversal for SpacialCurvature {
    type Output = SpacialCurvature;

    fn reversal(self) -> SpacialCurvature {
        SpacialCurvature {
            groups: SpacialCurvatureGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Sphere {
    type Output = Sphere;

    fn reversal(self) -> Sphere {
        Sphere {
            groups: SphereGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Reversal for SphereWeight {
    type Output = SphereWeight;

    fn reversal(self) -> SphereWeight {
        SphereWeight {
            groups: SphereWeightGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Transflector {
    type Output = Transflector;

    fn reversal(self) -> Transflector {
        Transflector {
            groups: TransflectorGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1(),
            },
        }
    }
}

impl Reversal for Translator {
    type Output = Translator;

    fn reversal(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}
